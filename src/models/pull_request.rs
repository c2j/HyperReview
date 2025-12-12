//! PullRequest model for code change proposals
//!
//! Represents a PR from a provider with metadata for review.

use serde::{Deserialize, Serialize};

use crate::models::{CiStatus, PrId, PrStatus, RepoId, SyncStatus};

/// Represents a pull request awaiting review
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullRequest {
    /// Pull Request ID: {provider}:{owner}/{repo}#{number}
    pub id: PrId,

    /// Associated repository ID
    pub repo_id: RepoId,

    /// PR number within repository
    pub number: u32,

    /// PR title
    pub title: String,

    /// PR author username
    pub author_username: String,

    /// Author avatar URL
    pub author_avatar_url: Option<String>,

    /// PR status (Open/Closed/Merged)
    pub status: PrStatus,

    /// SHA of head commit
    pub head_sha: String,

    /// SHA of base commit
    pub base_sha: String,

    /// CI status
    pub ci_status: Option<CiStatus>,

    /// PR description in markdown
    pub body_markdown: String,

    /// User has viewed this PR
    pub is_read: bool,

    /// User archived this PR
    pub is_archived: bool,

    /// Last updated timestamp on provider
    pub updated_at: chrono::DateTime<chrono::Utc>,

    /// PR creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// Last sync timestamp
    pub last_synced_at: Option<chrono::DateTime<chrono::Utc>>,

    /// PR sync status
    pub sync_status: SyncStatus,
}

impl PullRequest {
    /// Create a new pull request
    pub fn new(
        repo_id: RepoId,
        number: u32,
        title: String,
        author_username: String,
        head_sha: String,
        base_sha: String,
    ) -> Self {
        let id = PrId::new_from_parts(&repo_id, number);
        Self {
            id,
            repo_id,
            number,
            title,
            author_username,
            author_avatar_url: None,
            status: PrStatus::Open,
            head_sha,
            base_sha,
            ci_status: None,
            body_markdown: String::new(),
            is_read: false,
            is_archived: false,
            updated_at: chrono::Utc::now(),
            created_at: chrono::Utc::now(),
            last_synced_at: None,
            sync_status: SyncStatus::Pending,
        }
    }

    /// Mark PR as read
    pub fn mark_read(&mut self) {
        self.is_read = true;
    }

    /// Mark PR as archived
    pub fn mark_archived(&mut self) {
        self.is_archived = true;
    }

    /// Mark PR as successfully synced
    pub fn mark_synced(&mut self) {
        self.sync_status = SyncStatus::Synced;
        self.last_synced_at = Some(chrono::Utc::now());
    }

    /// Mark PR sync as failed
    pub fn mark_sync_failed(&mut self) {
        self.sync_status = SyncStatus::Failed;
    }

    /// Set CI status
    pub fn set_ci_status(&mut self, ci_status: CiStatus) {
        self.ci_status = Some(ci_status);
    }

    /// Set body markdown
    pub fn set_body(&mut self, body: String) {
        self.body_markdown = body;
    }

    /// Check if PR is open and not archived
    pub fn is_active(&self) -> bool {
        self.status == PrStatus::Open && !self.is_archived
    }

    /// Get display title with PR number
    pub fn display_title(&self) -> String {
        format!("#{}: {}", self.number, self.title)
    }
}
