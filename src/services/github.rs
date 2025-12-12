//! GitHub API integration service
//!
//! Handles OAuth2 authentication and GitHub REST API v3 calls.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::Result;
use crate::models::{Account, Provider, PullRequest, Repository};

// GitHub API response types
#[derive(Debug, Serialize, Deserialize)]
struct GitHubUser {
    login: String,
    id: u64,
    avatar_url: Option<String>,
    name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GitHubUserLite {
    login: String,
    avatar_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GitHubRef {
    sha: String,
    ref_field: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GitHubPullRequest {
    id: u64,
    number: u32,
    title: String,
    body: Option<String>,
    state: String,
    user: GitHubUserLite,
    head: GitHubRef,
    base: GitHubRef,
    html_url: String,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GitHubPr {
    number: u32,
    title: String,
    body: Option<String>,
    state: String,
    user: GitHubUserLite,
    head: GitHubRef,
    base: GitHubRef,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GitHubSearchResponse {
    total_count: u64,
    items: Vec<GitHubPullRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GitHubStatus {
    state: String, // "pending", "success", "failure", "error"
}

/// GitHub API client
pub struct GitHubService {
    /// Base URL for GitHub API
    base_url: String,

    /// OAuth client ID
    client_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeviceCodeRequest {
    client_id: String,
    scope: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: u32,
    interval: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenRequest {
    client_id: String,
    device_code: String,
    grant_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenResponse {
    access_token: Option<String>,
    token_type: Option<String>,
    scope: Option<String>,
    error: Option<String>,
}

/// Device flow initialization result
pub struct DeviceFlowResult {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u32,
    pub interval: u32,
}

impl GitHubService {
    /// Create a new GitHub service
    pub fn new(client_id: String) -> Self {
        Self {
            base_url: "https://api.github.com".to_string(),
            client_id,
        }
    }

    /// Start OAuth2 device flow
    pub async fn device_flow_start(&self) -> Result<DeviceFlowResult> {
        let client = reqwest::Client::new();

        let request = DeviceCodeRequest {
            client_id: self.client_id.clone(),
            scope: "repo read:user".to_string(),
        };

        let response: DeviceCodeResponse = client
            .post("https://github.com/login/device/code")
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        Ok(DeviceFlowResult {
            device_code: response.device_code,
            user_code: response.user_code,
            verification_uri: response.verification_uri,
            expires_in: response.expires_in,
            interval: response.interval,
        })
    }

    /// Poll for access token during device flow
    pub async fn device_flow_poll(&self, device_code: &str) -> Result<Option<String>> {
        let client = reqwest::Client::new();

        let request = TokenRequest {
            client_id: self.client_id.clone(),
            device_code: device_code.to_string(),
            grant_type: "urn:ietf:params:oauth:grant-type:device_code".to_string(),
        };

        let response: TokenResponse = client
            .post("https://github.com/login/oauth/access_token")
            .json(&request)
            .header("Accept", "application/json")
            .send()
            .await?
            .json()
            .await?;

        // Check for errors
        if let Some(error) = response.error {
            match error.as_str() {
                "authorization_pending" => return Ok(None),
                "slow_down" => {
                    // Add 5 seconds to interval
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    return Ok(None);
                }
                "expired_token" => {
                    return Err(crate::error::Error::OAuth("Device code expired".to_string()));
                }
                "access_denied" => {
                    return Err(crate::error::Error::OAuth("User denied access".to_string()));
                }
                _ => {
                    return Err(crate::error::Error::OAuth(format!(
                        "Device flow error: {}",
                        error
                    )));
                }
            }
        }

        // Success - return access token
        if let Some(token) = response.access_token {
            Ok(Some(token))
        } else {
            Err(crate::error::Error::OAuth(
                "No access token received".to_string(),
            ))
        }
    }

    /// Get authenticated user
    pub async fn get_authenticated_user(&self, access_token: &str) -> Result<Account> {
        let client = reqwest::Client::new();

        let response = client
            .get(&format!("{}/user", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Accept", "application/vnd.github+json")
            .send()
            .await?;

        // Handle HTTP status codes
        let status = response.status();
        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(crate::error::Error::TokenExpired);
        }
        if status == reqwest::StatusCode::FORBIDDEN {
            return Err(crate::error::Error::Auth(
                "Access forbidden. Check your permissions.".to_string(),
            ));
        }

        let user: GitHubUser = response.json().await?;

        let mut account = Account::new(Provider::GitHub, user.login, user.avatar_url);
        account.access_token = access_token.to_string();

        Ok(account)
    }

    /// Search for PRs where user is reviewer or mentioned
    pub async fn search_review_requests(
        &self,
        access_token: &str,
        username: &str,
    ) -> Result<Vec<PullRequest>> {
        let client = reqwest::Client::new();

        let query = format!(
            "is:pr is:open review-requested:{} OR mentions:{}",
            username, username
        );

        let response = client
            .get(&format!("{}/search/issues", self.base_url))
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Accept", "application/vnd.github+json")
            .query(&[("q", &query)])
            .send()
            .await?;

        // Handle HTTP status codes
        let status = response.status();
        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(crate::error::Error::TokenExpired);
        }
        if status == reqwest::StatusCode::FORBIDDEN {
            return Err(crate::error::Error::Auth(
                "Access forbidden. Check your permissions.".to_string(),
            ));
        }
        if status == reqwest::StatusCode::NOT_FOUND {
            return Err(crate::error::Error::NotFound(
                "GitHub API endpoint not found".to_string(),
            ));
        }

        let response: GitHubSearchResponse = response.json().await?;

        let mut prs = Vec::new();

        for item in response.items {
            // Parse repository info from HTML URL
            // Format: https://github.com/owner/repo/pull/123
            let url_parts: Vec<&str> = item.html_url.split("/").collect();
            if url_parts.len() < 5 {
                continue;
            }

            let owner = url_parts[4];
            let repo = url_parts[5];

            // Create repository
            let repo_id = crate::models::RepoId::new("github", owner, repo);

            // Create pull request
            let pr = PullRequest::new(
                repo_id,
                item.number,
                item.title,
                item.user.login,
                item.head.sha,
                item.base.sha,
            );

            prs.push(pr);
        }

        Ok(prs)
    }

    /// Get a specific pull request
    pub async fn get_pull_request(
        &self,
        access_token: &str,
        owner: &str,
        repo: &str,
        number: u32,
    ) -> Result<PullRequest> {
        let client = reqwest::Client::new();

        let response = client
            .get(&format!(
                "{}/repos/{}/{}/pulls/{}",
                self.base_url, owner, repo, number
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Accept", "application/vnd.github+json")
            .send()
            .await?;

        // Handle HTTP status codes
        let status = response.status();
        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(crate::error::Error::TokenExpired);
        }
        if status == reqwest::StatusCode::FORBIDDEN {
            return Err(crate::error::Error::Auth(
                "Access forbidden. Check your permissions.".to_string(),
            ));
        }
        if status == reqwest::StatusCode::NOT_FOUND {
            return Err(crate::error::Error::NotFound(format!(
                "Pull request #{}/{}#{} not found",
                owner, repo, number
            )));
        }

        let pr: GitHubPr = response.json().await?;

        let repo_id = crate::models::RepoId::new("github", owner, repo);

        let mut pull_request = PullRequest::new(
            repo_id,
            pr.number,
            pr.title,
            pr.user.login,
            pr.head.sha,
            pr.base.sha,
        );

        pull_request.set_body(pr.body.unwrap_or_default());

        Ok(pull_request)
    }

    /// Get commit status for a SHA
    pub async fn get_commit_status(
        &self,
        access_token: &str,
        owner: &str,
        repo: &str,
        sha: &str,
    ) -> Result<Option<crate::models::CiStatus>> {
        let client = reqwest::Client::new();

        let response = client
            .get(&format!(
                "{}/repos/{}/{}/commits/{}/status",
                self.base_url, owner, repo, sha
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Accept", "application/vnd.github+json")
            .send()
            .await?;

        // Handle HTTP status codes
        let status = response.status();
        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(crate::error::Error::TokenExpired);
        }
        if status == reqwest::StatusCode::FORBIDDEN {
            return Err(crate::error::Error::Auth(
                "Access forbidden. Check your permissions.".to_string(),
            ));
        }
        if status == reqwest::StatusCode::NOT_FOUND {
            // 404 for commit status is not critical, return None
            return Ok(None);
        }

        let response: Vec<GitHubStatus> = response.json().await?;

        if response.is_empty() {
            return Ok(None);
        }

        let state = &response[0].state;

        let ci_status = match state.as_str() {
            "pending" => crate::models::CiStatus::Pending,
            "success" => crate::models::CiStatus::Success,
            "failure" | "error" => crate::models::CiStatus::Failure,
            _ => return Ok(None),
        };

        Ok(Some(ci_status))
    }
}
