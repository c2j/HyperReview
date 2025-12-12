//! Account model for connected provider accounts (GitHub/GitLab)
//!
//! Represents a user's authentication and profile information from a provider.

use serde::{Deserialize, Serialize};

use crate::models::{Provider, SyncStatus};

/// Represents a connected provider account
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Account {
    /// Composite ID: {provider}:{username}
    pub id: String,

    /// Provider (GitHub/GitLab)
    pub provider: Provider,

    /// Provider username
    pub username: String,

    /// User avatar URL
    pub avatar_url: Option<String>,

    /// OAuth access token (encrypted at rest via keyring)
    pub access_token: String,

    /// OAuth refresh token (encrypted at rest via keyring)
    pub refresh_token: Option<String>,

    /// Token expiration timestamp
    pub token_expires_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Account sync status
    pub sync_status: SyncStatus,

    /// Last successful sync timestamp
    pub last_synced_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Account {
    /// Create a new account
    pub fn new(
        provider: Provider,
        username: String,
        avatar_url: Option<String>,
    ) -> Self {
        let id = format!("{}:{}", provider.as_str(), username);
        Self {
            id,
            provider,
            username,
            avatar_url,
            access_token: String::new(),
            refresh_token: None,
            token_expires_at: None,
            sync_status: SyncStatus::Pending,
            last_synced_at: None,
        }
    }

    /// Check if token is expired
    pub fn is_token_expired(&self) -> bool {
        if let Some(expires_at) = self.token_expires_at {
            chrono::Utc::now() > expires_at
        } else {
            false
        }
    }

    /// Mark account as successfully synced
    pub fn mark_synced(&mut self) {
        self.sync_status = SyncStatus::Synced;
        self.last_synced_at = Some(chrono::Utc::now());
    }

    /// Mark account sync as failed
    pub fn mark_sync_failed(&mut self) {
        self.sync_status = SyncStatus::Failed;
    }
}
