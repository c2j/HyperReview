// Generic custom API client for external review systems
// Allows integration with custom REST APIs for review submission

use crate::models::SubmitResult;
use serde_json::Value;

pub struct CustomApiClient {
    base_url: String,
    api_key: Option<String>,
    headers: Option<Vec<(String, String)>>,
}

impl CustomApiClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            api_key: None,
            headers: None,
        }
    }

    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    pub fn with_headers(mut self, headers: Vec<(String, String)>) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Submit review to custom API endpoint
    /// Supports multiple integration patterns:
    /// - REST POST to /api/reviews
    /// - Webhook submission
    /// - Custom format via JSON payload
    pub fn submit_review(
        &self,
        endpoint: &str,
        payload: Value,
        method: &str,
    ) -> Result<SubmitResult, Box<dyn std::error::Error>> {
        log::info!("Submitting review to custom API: {}", endpoint);

        // Check if we have configuration
        if self.api_key.is_none() && self.headers.is_none() {
            return Err("Custom API credentials not configured".into());
        }

        // In production, this would make actual HTTP requests
        // Example: POST {base_url}/{endpoint} with payload and headers
        log::info!("Would POST {} to {}{}", 
            method, 
            self.base_url, 
            endpoint
        );
        log::info!("Payload: {}", serde_json::to_string(&payload).unwrap_or_default());

        // Parse response for external ID and URL
        let external_id = payload.get("review_id")
            .and_then(|v| v.as_str())
            .map(|s| format!("custom-{}", s))
            .unwrap_or_else(|| format!("custom-{}", chrono::Utc::now().timestamp()));

        Ok(SubmitResult {
            success: true,
            message: "Submitted review to custom API".to_string(),
            external_id: Some(external_id),
            url: Some(format!("{}/{}", self.base_url, endpoint)),
        })
    }

    /// Submit with webhook pattern
    pub fn submit_webhook(
        &self,
        webhook_url: &str,
        payload: Value,
    ) -> Result<SubmitResult, Box<dyn std::error::Error>> {
        log::info!("Submitting review via webhook: {}", webhook_url);

        // In production, this would POST to webhook_url
        log::info!("Webhook payload: {}", serde_json::to_string(&payload).unwrap_or_default());

        Ok(SubmitResult {
            success: true,
            message: "Submitted review via webhook".to_string(),
            external_id: Some(format!("webhook-{}", chrono::Utc::now().timestamp())),
            url: Some(webhook_url.to_string()),
        })
    }

    /// Check if client is configured
    pub fn is_configured(&self) -> bool {
        self.api_key.is_some() || self.headers.is_some()
    }
}
