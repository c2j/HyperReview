// Gerrit API client for review submission
// Fully async using tokio spawn_blocking for HTTP calls

use tokio::time::Duration;
use reqwest::ClientBuilder;
use reqwest::Client;
use serde_json::Value;
use serde::{Serialize, Deserialize};
use log::{info, error};
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
            .expect("Failed to build HTTP client");
        
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client,
            username: None,
            http_password: None,
            retry_config: RetryConfig::default(),
        }
    }
    
    pub fn with_timeout(mut self, timeout_secs: u64) -> Self {
        self.client = ClientBuilder::new()
            .timeout(Duration::from_secs(timeout_secs))
            .connect_timeout(Duration::from_secs(10))
            .user_agent("HyperReview/1.0 GerritClient")
            .build()
            .expect("Failed to build HTTP client with custom timeout");
        self
    }
    
    pub fn with_retry_config(mut self, retry_config: RetryConfig) -> Self {
        self.retry_config = retry_config;
        self
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn with_auth(mut self, username: String, http_password: String) -> Self {
        self.username = Some(username);
        self.http_password = Some(http_password);
        self
    }

    fn get_api_url(&self, path: &str) -> String {
        let prefix = if self.username.is_some() && self.http_password.is_some() {
            "/a"
        } else {
            ""
        };
        format!("{}{}{}", self.base_url, prefix, path)
    }

    pub async fn test_connection(&self) -> Result<ConnectionTestResult, HyperReviewError> {
        info!("Testing connection to Gerrit: {}", self.base_url);

        let url = format!("{}/config/server/info", self.base_url);
        
        let mut request = self.client.get(&url);
        
        // Add basic auth if credentials are available
        if let (Some(user), Some(pass)) = (&self.username, &self.http_password) {
            request = request.basic_auth(user, Some(pass));
        }

        let response = request.send().await?;
        let status = response.status();
        let body = response.text().await?;

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
    }

    pub async fn get_change(
        &self,
        change_number: i32,
    ) -> Result<GerritChangeInfo, HyperReviewError> {
        info!("Getting Gerrit change #{}", change_number);

        let url = self.get_api_url(&format!(
            "/changes/{}?o=CURRENT_REVISION&o=DETAILED_ACCOUNTS&o=DETAILED_LABELS",
            change_number
        ));

        let mut request = self.client.get(&url);

        // Add basic auth if credentials are available
        if let (Some(user), Some(pass)) = (&self.username, &self.http_password) {
            request = request.basic_auth(user, Some(pass));
        }

        info!("GET to Gerrit: {}", url);
        let response = request.send().await?;
        let status = response.status();
        let body = response.text().await?;

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
    pub async fn get_revision_files(
        &self,
        change_id: &str,
        revision_id: &str,
    ) -> Result<std::collections::HashMap<String, GerritFileInfo>, HyperReviewError> {
        info!("Getting files for change {} revision {}", change_id, revision_id);
        
        let url = self.get_api_url(&format!("/changes/{}/revisions/{}/files/", change_id, revision_id));
        
        let mut request = self.client.get(&url);
        
        // Add basic auth if credentials are available
        if let (Some(user), Some(pass)) = (&self.username, &self.http_password) {
            request = request.basic_auth(user, Some(pass));
        }
        
        let response = request.send().await?;
        let status = response.status();
        let body = response.text().await?;
        
        if status.is_success() {
            let cleaned = Self::clean_gerrit_json(&body)?;
            let files: std::collections::HashMap<String, GerritFileInfo> = serde_json::from_str(&cleaned)?;
            info!("Retrieved {} files", files.len());
            Ok(files)
        } else {
            let error_msg = format!("Failed to get files: HTTP {}: {}", status, body);
            error!("{}", error_msg);
            Err(HyperReviewError::other(error_msg))
        }
    }

    /// Get file content for a specific revision
    pub async fn get_file_content(
        &self,
        change_id: &str,
        revision_id: &str,
        file_path: &str,
    ) -> Result<String, HyperReviewError> {
        info!("Getting content for file {} in change {} revision {}", file_path, change_id, revision_id);
        
        let encoded_path = urlencoding::encode(file_path);
        let url = self.get_api_url(&format!("/changes/{}/revisions/{}/files/{}/content", 
            change_id, revision_id, encoded_path));
        
        let mut request = self.client.get(&url);
        
        // Add basic auth if credentials are available
        if let (Some(user), Some(pass)) = (&self.username, &self.http_password) {
            request = request.basic_auth(user, Some(pass));
        }
        
        let response = request.send().await?;
        let status = response.status();
        let body = response.text().await?;
            
            if status.is_success() {
                // Gerrit returns base64 encoded content
                let decoded = base64::decode(&body)
                    .map_err(|e| HyperReviewError::other(format!("Failed to decode base64: {}", e)))?;
                let content = String::from_utf8(decoded)
                    .map_err(|e| HyperReviewError::other(format!("Failed to convert to UTF-8: {}", e)))?;
                Ok(content)
            } else if status.as_u16() == 404 {
                // File doesn't exist (e.g., deleted file)
                Ok(String::new())
            } else {
                let error_msg = format!("Failed to get file content: HTTP {}: {}", status, body);
                error!("{}", error_msg);
                Err(HyperReviewError::other(error_msg))
            }
    }

    /// Get diff for a file between revisions
    pub async fn get_file_diff(
        &self,
        change_id: &str,
        revision_id: &str,
        file_path: &str,
        base_revision: Option<&str>,
    ) -> Result<String, HyperReviewError> {
        info!("Getting diff for file {} in change {} revision {}", file_path, change_id, revision_id);
        
        let encoded_path = urlencoding::encode(file_path);
        let mut path = format!("/changes/{}/revisions/{}/files/{}/diff", 
            change_id, revision_id, encoded_path);
        if let Some(base) = base_revision {
            path.push_str(&format!("?base={}", base));
        }
        let url = self.get_api_url(&path);
        
        let file_path = file_path.to_string();
        
        let mut request = self.client.get(&url);
        
        // Add basic auth if credentials are available
        if let (Some(user), Some(pass)) = (&self.username, &self.http_password) {
            request = request.basic_auth(user, Some(pass));
        }
        
        let response = request.send().await?;
        let status = response.status();
        let body = response.text().await?;
        
        if status.is_success() {
            let cleaned = Self::clean_gerrit_json(&body)?;
            let diff_info: GerritDiffInfo = serde_json::from_str(&cleaned)?;
            
            // Convert Gerrit diff format to unified diff
            let unified_diff = Self::convert_gerrit_diff_to_unified(&diff_info, &file_path);
            Ok(unified_diff)
        } else {
            let error_msg = format!("Failed to get file diff: HTTP {}: {}", status, body);
            error!("{}", error_msg);
            Err(HyperReviewError::other(error_msg))
        }
    }

    /// Convert Gerrit diff format to unified diff format
    fn convert_gerrit_diff_to_unified(diff_info: &GerritDiffInfo, file_path: &str) -> String {
        let mut unified_diff = String::new();
        
        // Add file headers
        unified_diff.push_str(&format!("--- a/{}\n", file_path));
        unified_diff.push_str(&format!("+++ b/{}\n", file_path));
        
        // Process content sections
        for content in &diff_info.content {
            if let Some(ab) = &content.ab {
                // Common lines
                for line in ab {
                    unified_diff.push_str(&format!(" {}\n", line));
                }
            }
            
            if let Some(a) = &content.a {
                // Deleted lines
                for line in a {
                    unified_diff.push_str(&format!("-{}\n", line));
                }
            }
            
            if let Some(b) = &content.b {
                // Added lines
                for line in b {
                    unified_diff.push_str(&format!("+{}\n", line));
                }
            }
        }
        
        unified_diff
    }

    /// Get comments for a change
    pub async fn get_comments(&self, change_id: &str) -> Result<Vec<Comment>, HyperReviewError> {
        info!("Getting comments for change: {}", change_id);
        
        let url = self.get_api_url(&format!("/changes/{}/comments", change_id));
        
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
    /// offset: Starting index for pagination (0-based, Gerrit API uses 'S' parameter)
    /// limit: Maximum number of results to return (Gerrit API uses 'n' parameter)
    pub async fn search_changes(&self, query: &str, offset: Option<u32>, limit: Option<u32>) -> Result<Vec<GerritChangeInfo>, HyperReviewError> {
        info!("Searching changes with query: {}, offset: {:?}, limit: {:?}", query, offset, limit);

        let encoded_query = urlencoding::encode(query);
        let mut url_path = format!("/changes/?q={}&o=CURRENT_REVISION", encoded_query);

        // Add pagination parameters if provided
        if let Some(start) = offset {
            url_path.push_str(&format!("&S={}", start));
        }
        if let Some(max_results) = limit {
            url_path.push_str(&format!("&n={}", max_results));
        }

        let url = self.get_api_url(&url_path);
        
        let mut request = self.client.get(&url);
        
        // Add basic auth if credentials are available
        if let (Some(user), Some(pass)) = (&self.username, &self.http_password) {
            request = request.basic_auth(user, Some(pass));
        }
        
        let response = request.send().await?;
        let status = response.status();
        let body = response.text().await?;
        
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
    }
    
    /// Submit a review
    pub async fn submit_review(&self, change_id: &str, review: &ReviewInput
    ) -> Result<(), HyperReviewError> {
        info!("Submitting review for change: {}", change_id);
        
        let url = self.get_api_url(&format!("/changes/{}/revisions/current/review", change_id));
        let review_json = serde_json::to_string(review)?;
        
        let mut request = self.client.post(&url)
            .header("Content-Type", "application/json")
            .body(review_json);
        
        // Add basic auth if credentials are available
        if let (Some(user), Some(pass)) = (&self.username, &self.http_password) {
            request = request.basic_auth(user, Some(pass));
        }
        
        let response = request.send().await?;
        let status = response.status();
        let body = response.text().await?;
        
        if status.is_success() {
            info!("Review submitted successfully");
            Ok(())
        } else {
            let error_msg = format!("Failed to submit review: HTTP {}: {}", status, body);
            error!("{}", error_msg);
            Err(HyperReviewError::other(error_msg))
        }
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

#[derive(Debug, Clone, serde::Deserialize)]
pub struct GerritFileInfo {
    pub status: Option<String>,
    pub lines_inserted: Option<i32>,
    pub lines_deleted: Option<i32>,
    pub size_delta: Option<i32>,
    pub size: Option<i32>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct GerritDiffInfo {
    pub meta_a: Option<GerritFileMeta>,
    pub meta_b: Option<GerritFileMeta>,
    pub change_type: Option<String>,
    pub intraline_status: Option<String>,
    pub content: Vec<GerritDiffContent>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct GerritFileMeta {
    pub name: String,
    pub content_type: String,
    pub lines: Option<i32>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct GerritDiffContent {
    pub ab: Option<Vec<String>>,  // Common lines
    pub a: Option<Vec<String>>,   // Deleted lines
    pub b: Option<Vec<String>>,   // Added lines
    pub edit_a: Option<Vec<Vec<i32>>>,  // Intraline edits in a
    pub edit_b: Option<Vec<Vec<i32>>>,  // Intraline edits in b
    pub due_to_rebase: Option<bool>,
    pub skip: Option<i32>,
}
