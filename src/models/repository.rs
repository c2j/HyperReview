//! Repository model for git repositories from a provider
//!
//! Represents local and remote repository metadata.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::models::{Provider, RepoId, SyncStatus};

/// Represents a git repository from a provider
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Repository {
    /// Repository ID: {provider}:{owner}/{name}
    pub id: RepoId,

    /// Provider (GitHub/GitLab)
    pub provider: Provider,

    /// Repository owner (user or organization)
    pub owner: String,

    /// Repository name
    pub name: String,

    /// Path to local shallow clone (if exists)
    pub local_clone_path: Option<PathBuf>,

    /// Last successful sync timestamp
    pub last_synced_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Repository sync status
    pub sync_status: SyncStatus,
}

impl Repository {
    /// Create a new repository
    pub fn new(
        provider: Provider,
        owner: String,
        name: String,
    ) -> Self {
        let id = RepoId::new(&provider.as_str(), &owner, &name);
        Self {
            id,
            provider,
            owner,
            name,
            local_clone_path: None,
            last_synced_at: None,
            sync_status: SyncStatus::Pending,
        }
    }

    /// Set the local clone path
    pub fn set_clone_path(&mut self, path: PathBuf) {
        self.local_clone_path = Some(path);
    }

    /// Check if repository has a local clone
    pub fn is_cloned(&self) -> bool {
        self.local_clone_path.is_some()
    }

    /// Mark repository as successfully synced
    pub fn mark_synced(&mut self) {
        self.sync_status = SyncStatus::Synced;
        self.last_synced_at = Some(chrono::Utc::now());
    }

    /// Mark repository sync as failed
    pub fn mark_sync_failed(&mut self) {
        self.sync_status = SyncStatus::Failed;
    }

    /// Get the display name (owner/name)
    pub fn display_name(&self) -> String {
        format!("{}/{}", self.owner, self.name)
    }
}
