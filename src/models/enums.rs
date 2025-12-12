//! Enum types for HyperReview domain models
//!
//! Following Constitution III requirements for strong typing.

use serde::{Deserialize, Serialize};

/// Git provider type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Provider {
    GitHub,
    GitLab,
}

impl Provider {
    /// Get provider name as string
    pub fn as_str(&self) -> &'static str {
        match self {
            Provider::GitHub => "github",
            Provider::GitLab => "gitlab",
        }
    }

    /// Parse from string
    pub fn from_str(s: &str) -> Result<Self, crate::error::Error> {
        match s {
            "github" => Ok(Provider::GitHub),
            "gitlab" => Ok(Provider::GitLab),
            _ => Err(crate::error::Error::InvalidId(format!(
                "Invalid provider: {}",
                s
            ))),
        }
    }
}

/// Pull request status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrStatus {
    Open,
    Closed,
    Merged,
}

impl PrStatus {
    /// Parse from GitHub API state string
    pub fn from_github(state: &str, merged: bool) -> Self {
        if merged {
            PrStatus::Merged
        } else if state == "closed" {
            PrStatus::Closed
        } else {
            PrStatus::Open
        }
    }

    /// Get status as string
    pub fn as_str(&self) -> &'static str {
        match self {
            PrStatus::Open => "open",
            PrStatus::Closed => "closed",
            PrStatus::Merged => "merged",
        }
    }
}

/// CI/Build status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CiStatus {
    Pending,
    Success,
    Failure,
}

impl CiStatus {
    /// Parse from GitHub API state string
    pub fn from_github(state: &str) -> Self {
        match state {
            "success" => CiStatus::Success,
            "failure" | "error" => CiStatus::Failure,
            _ => CiStatus::Pending,
        }
    }

    /// Get CI status as string
    pub fn as_str(&self) -> &'static str {
        match self {
            CiStatus::Pending => "pending",
            CiStatus::Success => "success",
            CiStatus::Failure => "failure",
        }
    }

    /// Parse from string
    pub fn from_str(s: &str) -> Result<Self, crate::error::Error> {
        match s {
            "pending" => Ok(CiStatus::Pending),
            "success" => Ok(CiStatus::Success),
            "failure" => Ok(CiStatus::Failure),
            _ => Err(crate::error::Error::InvalidId(format!(
                "Invalid CI status: {}",
                s
            ))),
        }
    }
}

/// Sync status for offline-first entities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncStatus {
    /// Created locally, not yet synced
    Pending,
    /// Successfully synced to provider
    Synced,
    /// Sync failed, manual intervention needed
    Failed,
}

impl SyncStatus {
    /// Get sync status as string
    pub fn as_str(&self) -> &'static str {
        match self {
            SyncStatus::Pending => "pending",
            SyncStatus::Synced => "synced",
            SyncStatus::Failed => "failed",
        }
    }
}

/// Review decision type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewDecision {
    Approve,
    RequestChanges,
    Comment,
}

impl ReviewDecision {
    /// Convert to GitHub API event string
    pub fn to_github_event(&self) -> &'static str {
        match self {
            ReviewDecision::Approve => "APPROVE",
            ReviewDecision::RequestChanges => "REQUEST_CHANGES",
            ReviewDecision::Comment => "COMMENT",
        }
    }
}

/// File status in diff
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileStatus {
    Added,
    Modified,
    Deleted,
    Renamed,
}

/// Line type in diff
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineType {
    /// Unchanged context line
    Context,
    /// Added line
    Addition,
    /// Deleted line
    Deletion,
}
