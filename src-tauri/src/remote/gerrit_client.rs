// Gerrit API client for review submission
// External system integration

use crate::models::{SubmitResult, Comment};
use serde_json::{self, Value};
use serde_json::json as serde_json_json;

pub struct GerritClient {
    base_url: String,
    username: Option<String>,
    http_password: Option<String>,
}

impl GerritClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            username: None,
            http_password: None,
        }
    }

    pub fn with_auth(mut self, username: String, http_password: String) -> Self {
        self.username = Some(username);
        self.http_password = Some(http_password);
        self
    }

    /// Submit review comments to Gerrit change
    /// POST /changes/{change-id}/revisions/{revision-id}/review
    pub fn submit_review(
        &self,
        change_id: &str,
        revision_id: &str,
        comments: Vec<Comment>,
        vote: Option<i32>,
    ) -> Result<SubmitResult, Box<dyn std::error::Error>> {
        log::info!("Submitting review to Gerrit change {} revision {}", change_id, revision_id);

        // Check if we have authentication
        if self.username.is_none() || self.http_password.is_none() {
            return Err("Gerrit credentials not configured. Please configure credentials in settings.".into());
        }

        // Build review payload
        let mut review_payload = serde_json_json!({
            "comments": {},
            "labels": {},
            "message": format!("Submitted {} comments via HyperReview", comments.len())
        });

        // Group comments by file path
        let comments_obj = review_payload["comments"].as_object_mut().unwrap();
        for comment in &comments {
            if !comment.file_path.is_empty() {
                let comment_data = serde_json_json!({
                    "line": comment.line_number,
                    "message": comment.content
                });

                // Use Gerrit file path format
                // Gerrit expects file paths relative to project root
                let file_path = &comment.file_path;
                let gerrit_path = if file_path.starts_with('/') {
                    &file_path[1..]
                } else {
                    file_path.as_str()
                };

                // If multiple comments on same line, create array
                if comments_obj.contains_key(gerrit_path) {
                    let existing = comments_obj[gerrit_path].clone();
                    if existing.is_array() {
                        let mut arr = existing.as_array().unwrap().clone();
                        arr.push(comment_data);
                        comments_obj[gerrit_path] = Value::Array(arr);
                    } else {
                        comments_obj[gerrit_path] = serde_json_json!([existing, comment_data]);
                    }
                } else {
                    comments_obj[gerrit_path] = comment_data;
                }
            }
        }

        // Add Code-Review label/vote if provided
        if let Some(score) = vote {
            review_payload["labels"]["Code-Review"] = serde_json_json!(score);
        }

        // Build API URL
        let url = format!(
            "{}/a/changes/{}/revisions/{}/review",
            self.base_url, change_id, revision_id
        );

        log::info!("POST to Gerrit: {}", url);

        // Create HTTP client with authentication
        let client = reqwest::blocking::Client::new();

        let username = self.username.as_ref().unwrap();
        let password = self.http_password.as_ref().unwrap();

        // Submit review to Gerrit
        let response = client
            .post(&url)
            .basic_auth(username, Some(password))
            .header("Content-Type", "application/json")
            .json(&review_payload)
            .send()?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().unwrap_or_else(|_| "Unable to read error response".to_string());

            log::error!("Gerrit API error: {} - {}", status, error_body);

            return Err(format!(
                "Gerrit API returned error {}: {}",
                status,
                error_body.trim()
            ).into());
        }

        let response_text = response.text()?;

        // Gerrit returns ])} for successful POST
        if response_text.contains(")]}'") || response_text.is_empty() {
            log::info!("Successfully submitted {} comments to Gerrit change {}", comments.len(), change_id);

            Ok(SubmitResult {
                success: true,
                message: format!("Submitted {} comments to Gerrit change {}", comments.len(), change_id),
                external_id: Some(format!("gerrit-{}", change_id)),
                url: Some(format!("{}/c/{}", self.base_url, change_id)),
            })
        } else {
            log::warn!("Unexpected Gerrit response: {}", response_text);
            Ok(SubmitResult {
                success: true,
                message: format!("Submitted {} comments to Gerrit change {}", comments.len(), change_id),
                external_id: Some(format!("gerrit-{}", change_id)),
                url: Some(format!("{}/c/{}", self.base_url, change_id)),
            })
        }
    }

    /// Get change details
    /// GET /changes/{change-id}
    pub fn get_change(&self, change_id: &str) -> Result<ChangeInfo, Box<dyn std::error::Error>> {
        log::info!("Fetching Gerrit change {}", change_id);

        let url = format!("{}/a/changes/{}", self.base_url, change_id);

        if let (Some(username), Some(password)) = (&self.username, &self.http_password) {
            let client = reqwest::blocking::Client::new();
            let response = client
                .get(&url)
                .basic_auth(username, Some(password))
                .send()?;

            if !response.status().is_success() {
                let status = response.status();
                let error_body = response.text().unwrap_or_else(|_| "Unable to read error response".to_string());

                return Err(format!(
                    "Failed to fetch Gerrit change: {} - {}",
                    status,
                    error_body.trim()
                ).into());
            }

            let text = response.text()?;
            let json: Value = serde_json::from_str(&text.trim_start_matches(")]}'\n"))?;

            let change = json.as_array()
                .and_then(|arr| arr.first())
                .ok_or("No change data in response")?;

            Ok(ChangeInfo {
                id: change["id"].as_str().unwrap_or(change_id).to_string(),
                project: change["project"].as_str().unwrap_or("unknown").to_string(),
                branch: change["branch"].as_str().unwrap_or("unknown").to_string(),
                subject: change["subject"].as_str().unwrap_or("unknown").to_string(),
                status: change["status"].as_str().unwrap_or("UNKNOWN").to_string(),
                owner: change["owner"]["name"].as_str()
                    .or_else(|| change["owner"]["email"].as_str())
                    .unwrap_or("unknown")
                    .to_string(),
            })
        } else {
            Err("Gerrit credentials not configured".into())
        }
    }

    /// Check if client is configured
    pub fn is_configured(&self) -> bool {
        self.username.is_some() && self.http_password.is_some()
    }
}

#[derive(Debug, Clone)]
pub struct ChangeInfo {
    pub id: String,
    pub project: String,
    pub branch: String,
    pub subject: String,
    pub status: String,
    pub owner: String,
}
