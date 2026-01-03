// Gerrit API client for review submission
// Fully async using tokio spawn_blocking for HTTP calls

use tokio::time::Duration;
use reqwest::ClientBuilder;
use serde_json::Value;
use log::{info, error};
use rand::Rng;

use crate::models::{SubmitResult, Comment};
use crate::errors::HyperReviewError;

#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub base_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_multiplier: f64,
    pub jitter_factor: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay_ms: 1000,
            max_delay_ms: 30000,
            backoff_multiplier: 2.0,
            jitter_factor: 0.1,
        }
    }
}

pub struct GerritClient {
    base_url: String,
    client: Client,
    username: Option<String>,
    http_password: Option<String>,
    retry_config: RetryConfig,
}

impl GerritClient {
    pub fn new(base_url: &str) -> Self {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .user_agent("HyperReview/1.0 GerritClient")
            .build()
            .unwrap_or_else(|_| Client::new());

        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client,
            username: None,
            http_password: None,
            retry_config: RetryConfig::default(),
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn with_auth(mut self, username: String, http_password: String) -> Self {
        self.username = Some(username);
        self.http_password = Some(http_password);
        self
    }

    fn calculate_backoff_delay(&self, attempt: u32) -> Duration {
        let base_delay = self.retry_config.base_delay_ms;
        let exponential_delay = (base_delay as f64) * self.retry_config.backoff_multiplier.powi(attempt as i32);

        let max_delay = self.retry_config.max_delay_ms;
        let capped_delay = (exponential_delay as u64).min(max_delay);

        let jitter_range = (capped_delay as f64) * self.retry_config.jitter_factor;
        let jitter = rand::thread_rng().gen_range(-jitter_range..=jitter_range);

        let final_delay = (capped_delay as f64) + jitter;
        let final_delay_ms = final_delay.max(0.0) as u64;

        Duration::from_millis(final_delay_ms)
    }

    pub async fn test_connection(&self) -> Result<ConnectionTestResult, HyperReviewError> {
        info!("Testing connection to Gerrit: {}", self.base_url);

        let url = format!("{}/config/server/info", self.base_url);

        let result = tokio::task::spawn_blocking(move || {
            let client = reqwest::blocking::Client::new();

            let mut request = client
                .get(&url)
                .header("Accept", "application/json");

            let response = request.send()?;

            let status = response.status();
            let body = response.text()?;

            if status.is_success() {
                let cleaned = Self::clean_gerrit_json(&body)?;
                let server_info = cleaned.as_object().and_then(|v| v.get("gerrit_version"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());

                let mut features = vec![];
                if cleaned.as_object().and_then(|v| v.get("auth")).is_some() {
                    features.push("Authentication".to_string());
                }
                if cleaned.as_object().and_then(|v| v.get("default_theme")).is_some() {
                    features.push("REST API".to_string());
                }

                Ok(ConnectionTestResult {
                    success: true,
                    gerrit_version: server_info,
                    error_message: None,
                    supported_features: features,
                })
            } else {
                let error_msg = format!("HTTP {}: {}", status, body);
                error!("{}", error_msg);
                Err(error_msg.into())
            }
        }).await.map_err(|e| format!("Task spawn failed: {}", e))?;

        result
    }

    pub async fn submit_review(
        &self,
        change_id: &str,
        revision_id: &str,
        comments: Vec<Comment>,
        vote: Option<i32>,
    ) -> Result<SubmitResult, HyperReviewError> {
        info!("Submitting review to Gerrit change {} revision {}", change_id, revision_id);

        if self.username.is_none() || self.http_password.is_none() {
            return Err("Gerrit credentials not configured. Please configure credentials in settings.".into());
        }

        let mut review_payload = serde_json::json!({
            "comments": {},
            "labels": {},
            "message": format!("Submitted {} comments via HyperReview", comments.len())
        });

        let comments_obj = review_payload["comments"].as_object_mut().unwrap();
        for comment in &comments {
            if !comment.file_path.is_empty() {
                let comment_data = serde_json::json!({
                    "line": comment.line_number,
                    "message": comment.content
                });

                let file_path = &comment.file_path;
                let gerrit_path = if file_path.starts_with('/') {
                    &file_path[1..]
                } else {
                    file_path.as_str()
                };

                if comments_obj.contains_key(gerrit_path) {
                    let existing = comments_obj[gerrit_path].clone();
                    if existing.is_array() {
                        let mut arr = existing.as_array().unwrap().clone();
                        arr.push(comment_data);
                        comments_obj[gerrit_path] = Value::Array(arr);
                    } else {
                        comments_obj[gerrit_path] = serde_json::json!([existing, comment_data]);
                    }
                } else {
                    comments_obj[gerrit_path] = comment_data;
                }
            }
        }

        if let Some(score) = vote {
            review_payload["labels"]["Code-Review"] = serde_json::json!(score);
        }

        let url = format!(
            "{}/a/changes/{}/revisions/{}/review",
            self.base_url, change_id, revision_id
        );

        info!("POST to Gerrit: {}", url);

        let username = self.username.clone().unwrap();
        let password = self.http_password.clone().unwrap();
        let base_url_clone = self.base_url.clone();

        let result = tokio::task::spawn_blocking(move || {
            let client = reqwest::blocking::Client::new();
            let response = client
                .post(&url)
                .basic_auth(&username, password)
                .json(&review_payload)
                .send()?;

            let status = response.status();
            let response_text = response.text()?;

            info!("Gerrit response status: {}, body length: {}", status, response_text.len());

            if status.is_success() {
                info!("Review submitted successfully to Gerrit change {}", change_id);
                Ok(SubmitResult {
                    success: true,
                    message: format!("Successfully submitted {} comments to Gerrit", comments.len()),
                    external_id: Some(change_id.to_string()),
                    url: Some(format!("{}/c/{}/{}", base_url_clone, change_id, revision_id)),
                })
            } else {
                let error_msg = format!("Gerrit API error {}: {}", status, response_text);
                error!("{}", error_msg);
                Err(error_msg.into())
            }
        }).await.map_err(|e| format!("Task spawn failed: {}", e))?;

        result
    }

    pub async fn query_changes(
        &self,
        query: &str,
    ) -> Result<Vec<GerritChangeInfo>, HyperReviewError> {
        info!("Querying Gerrit changes with query: {}", query);

        let url = if query.is_empty() {
            format!("{}/a/changes/?o=CURRENT_REVISION&o=DETAILED_ACCOUNTS", self.base_url)
        } else {
            format!(
                "{}/a/changes/?q={}&o=CURRENT_REVISION&o=DETAILED_ACCOUNTS",
                self.base_url,
                urlencoding::encode(query)
            )
        };

        info!("GET to Gerrit: {}", url);

        let result = tokio::task::spawn_blocking(move || {
            let client = reqwest::blocking::Client::new();
            let mut request = client.get(&url);

            let response = request.send()?;
            let status = response.status();
            let body = response.text()?;

            if status.is_success() {
                let cleaned = Self::clean_gerrit_json(&body)?;

                if cleaned.trim().is_empty() {
                    Ok(vec![])
                } else {
                    let changes = serde_json::from_str(&cleaned)?;
                    Ok(changes)
                }
            } else {
                let error_msg = format!("Gerrit API error {}: {}", status, body);
                error!("{}", error_msg);
                Err(error_msg.into())
            }
        }).await.map_err(|e| format!("Task spawn failed: {}", e))?;

        result
    }

    pub async fn get_change(
        &self,
        change_number: i32,
    ) -> Result<GerritChangeInfo, HyperReviewError> {
        info!("Getting Gerrit change #{}", change_number);

        let url = format!(
            "{}/a/changes/{}?o=CURRENT_REVISION&o=DETAILED_ACCOUNTS&o=DETAILED_LABELS",
            self.base_url, change_number
        );

        info!("GET to Gerrit: {}", url);

        let result = tokio::task::spawn_blocking(move || {
            let client = reqwest::blocking::Client::new();
            let mut request = client.get(&url);

            let response = request.send()?;
            let status = response.status();
            let body = response.text()?;

            info!("Response status: {}, body length: {}", status, body.len());

            if status.is_success() {
                let cleaned = Self::clean_gerrit_json(&body)?;
                let change = serde_json::from_str(&cleaned)?;
                info!("Successfully parsed change: {}", change.subject);
                Ok(change)
            } else {
                let error_msg = format!("Gerrit API error {}: {}", status, body);
                error!("{}", error_msg);
                Err(error_msg.into())
            }
        }).await.map_err(|e| format!("Task spawn failed: {}", e))?;

        result
    }

    fn clean_gerrit_json(&self, json_text: &str) -> Result<String, HyperReviewError> {
        let cleaned = json_text.trim();
        let prefixes = vec![")]}'", ")]}'\n", ")]}'\r\n", "while(1);", "while(1);\n", "for(;;);"];

        for prefix in prefixes {
            if cleaned.starts_with(prefix) {
                return Ok(cleaned[prefix.len()..].trim().to_string());
            }
        }

        Ok(cleaned.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct ConnectionTestResult {
    pub success: bool,
    pub gerrit_version: Option<String>,
    pub error_message: Option<String>,
    pub supported_features: Vec<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct GerritChangeInfo {
    pub id: String,
    pub change_id: String,
    #[serde(default)]
    pub _number: i32,
    pub subject: String,
    pub status: String,
    pub project: String,
    pub branch: String,
    pub topic: Option<String>,
    pub owner: serde_json::Value,
    pub updated: String,
    pub created: String,
    pub insertions: Option<i32>,
    pub deletions: Option<i32>,
    pub current_revision: Option<String>,
    pub revisions: Option<serde_json::Value>,
}
