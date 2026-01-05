// Gerrit API client for review submission
// Fully async using tokio spawn_blocking for HTTP calls

use tokio::time::Duration;
use reqwest::ClientBuilder;
use reqwest::Client;
use serde_json::Value;
use serde::{Serialize, Deserialize};
use log::{info, error};
use rand::Rng;
use std::collections::HashMap;

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
        let username = self.username.clone();
        let password = self.http_password.clone();

        let result = tokio::task::spawn_blocking(move || {
            let client = reqwest::blocking::Client::new();
            let mut request = client.get(&url);
            
            // Add basic auth if credentials are available
            if let (Some(user), Some(pass)) = (username, password) {
                request = request.basic_auth(user, Some(pass));
            }

            let response = request.send()?;
            let status = response.status();
            let body = response.text()?;

            if status.is_success() {
                let cleaned = Self::clean_gerrit_json(&body)?;
                let json_value: Value = serde_json::from_str(&cleaned)?;
                let server_info = json_value.as_object().and_then(|v| v.get("gerrit_version"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());

                let mut features = vec![];
                if json_value.as_object().and_then(|v| v.get("auth")).is_some() {
                    features.push("Authentication".to_string());
                }
                if json_value.as_object().and_then(|v| v.get("default_theme")).is_some() {
                    features.push("REST API".to_string());
                }

                Ok(ConnectionTestResult {
                    success: true,
                    gerrit_version: server_info,
                    error_message: None,
                    supported_features: features,
                })
            } else {
                let error_msg = format!("Gerrit API error {}: {}", status, body);
                error!("{}", error_msg);
                Err(HyperReviewError::other(error_msg))
            }
        }).await.map_err(|e| HyperReviewError::other(format!("Task spawn failed: {}", e)))?;

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
                let change: GerritChangeInfo = serde_json::from_str(&cleaned)?;
                info!("Successfully parsed change: {}", change.subject);
                Ok(change)
            } else {
                let error_msg = format!("Gerrit API error {}: {}", status, body);
                error!("{}", error_msg);
                Err(HyperReviewError::other(error_msg))
            }
        }).await.map_err(|e| HyperReviewError::other(format!("Task spawn failed: {}", e)))?;

        result
    }

    fn clean_gerrit_json(json_text: &str) -> Result<String, HyperReviewError> {
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

impl GerritClient {
    /// Get comments for a change
    pub async fn get_comments(&self, change_id: &str) -> Result<Vec<Comment>, HyperReviewError> {
        info!("Getting comments for change: {}", change_id);
        
        let url = format!("{}/a/changes/{}/comments", self.base_url, change_id);
        
        let username = self.username.clone();
        let password = self.http_password.clone();
        
        let result = tokio::task::spawn_blocking(move || {
            let client = reqwest::blocking::Client::new();
            let mut request = client.get(&url);
            
            // Add basic auth if credentials are available
            if let (Some(user), Some(pass)) = (username, password) {
                request = request.basic_auth(user, Some(pass));
            }
            
            let response = request.send()?;
            let status = response.status();
            let body = response.text()?;
            
            if status.is_success() {
                let cleaned = Self::clean_gerrit_json(&body)?;
                // Parse Gerrit comments format and convert to our Comment model
                // For now, return empty vec until full parsing is implemented
                Ok(Vec::new())
            } else {
                let error_msg = format!("Failed to get comments: HTTP {}: {}", status, body);
                error!("{}", error_msg);
                Err(HyperReviewError::other(error_msg))
            }
        }).await.map_err(|e| HyperReviewError::other(format!("Task spawn failed: {}", e)))?;
        
        result
    }
    
    /// Search for changes
    pub async fn search_changes(&self, query: &str) -> Result<Vec<GerritChangeInfo>, HyperReviewError> {
        info!("Searching changes with query: {}", query);
        
        let encoded_query = urlencoding::encode(query);
        let url = format!("{}/a/changes/?q={}&o=CURRENT_REVISION", self.base_url, encoded_query);
        
        let username = self.username.clone();
        let password = self.http_password.clone();
        
        let result = tokio::task::spawn_blocking(move || {
            let client = reqwest::blocking::Client::new();
            let mut request = client.get(&url);
            
            // Add basic auth if credentials are available
            if let (Some(user), Some(pass)) = (username, password) {
                request = request.basic_auth(user, Some(pass));
            }
            
            let response = request.send()?;
            let status = response.status();
            let body = response.text()?;
            
            if status.is_success() {
                let cleaned = Self::clean_gerrit_json(&body)?;
                let changes: Vec<GerritChangeInfo> = serde_json::from_str(&cleaned)?;
                info!("Found {} changes", changes.len());
                Ok(changes)
            } else {
                let error_msg = format!("Failed to search changes: HTTP {}: {}", status, body);
                error!("{}", error_msg);
                Err(HyperReviewError::other(error_msg))
            }
        }).await.map_err(|e| HyperReviewError::other(format!("Task spawn failed: {}", e)))?;
        
        result
    }
    
    /// Submit a review
    pub async fn submit_review(&self, change_id: &str, review: &ReviewInput) -> Result<(), HyperReviewError> {
        info!("Submitting review for change: {}", change_id);
        
        let url = format!("{}/a/changes/{}/revisions/current/review", self.base_url, change_id);
        let review_json = serde_json::to_string(review)?;
        
        let username = self.username.clone();
        let password = self.http_password.clone();
        
        let result = tokio::task::spawn_blocking(move || {
            let client = reqwest::blocking::Client::new();
            let mut request = client.post(&url)
                .header("Content-Type", "application/json")
                .body(review_json);
            
            // Add basic auth if credentials are available
            if let (Some(user), Some(pass)) = (username, password) {
                request = request.basic_auth(user, Some(pass));
            }
            
            let response = request.send()?;
            let status = response.status();
            let body = response.text()?;
            
            if status.is_success() {
                info!("Review submitted successfully");
                Ok(())
            } else {
                let error_msg = format!("Failed to submit review: HTTP {}: {}", status, body);
                error!("{}", error_msg);
                Err(HyperReviewError::other(error_msg))
            }
        }).await.map_err(|e| HyperReviewError::other(format!("Task spawn failed: {}", e)))?;
        
        result
    }
}

#[derive(Debug, Serialize)]
pub struct ReviewInput {
    pub message: String,
    pub labels: std::collections::HashMap<String, i32>,
    pub comments: std::collections::HashMap<String, Vec<CommentInput>>,
}

#[derive(Debug, Serialize)]
pub struct CommentInput {
    pub line: Option<i32>,
    pub message: String,
}
