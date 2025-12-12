//! Application state management for HyperReview
//!
//! Provides centralized AppState following Constitution IV requirements.

use gpui::*;
use std::sync::Arc;

use crate::config::Config;
use crate::error::Result;
use crate::services::{DbService, GitService};
use crate::models::Diff;

/// Central application state
///
/// Following Constitution IV: Application state MUST be centralized in AppState struct
pub struct AppState {
    /// Application configuration
    pub config: Config,

    /// Database service (initialized lazily)
    pub db: Option<Arc<DbService>>,

    /// Git service
    pub git: Arc<GitService>,

    /// Current view state
    pub current_view: ViewState,

    /// Network availability
    pub is_online: bool,

    /// Demo diff for Milestone 1
    pub demo_diff: Option<Diff>,
}

/// Current view state
#[derive(Debug, Clone, PartialEq)]
pub enum ViewState {
    /// Authentication view
    Auth,
    /// PR inbox view
    Inbox,
    /// PR review/diff view
    Review { pr_id: String },
}

impl AppState {
    /// Create new application state (simplified for Milestone 1)
    pub fn new(_cx: &()) -> Self {
        let config = Config::default();
        let git = Arc::new(GitService::new());

        Self {
            config,
            db: None,
            git,
            current_view: ViewState::Inbox,
            is_online: true,
            demo_diff: None,
        }
    }

    /// Navigate to a different view
    pub fn navigate_to(&mut self, view: ViewState) {
        self.current_view = view;
    }

    /// Navigate to Inbox view
    pub fn navigate_to_inbox(&mut self) {
        self.current_view = ViewState::Inbox;
    }

    /// Navigate to Review view for a specific PR
    pub fn navigate_to_review(&mut self, pr_id: String) {
        self.current_view = ViewState::Review { pr_id };
    }

    /// Navigate to Auth view
    pub fn navigate_to_auth(&mut self) {
        self.current_view = ViewState::Auth;
    }

    /// Initialize database connection
    pub async fn init_db(&mut self) -> Result<()> {
        let db = DbService::new(&self.config.database_path).await?;
        self.db = Some(Arc::new(db));
        Ok(())
    }

    /// Load demo diff for Milestone 1
    pub async fn load_demo_diff(&mut self) -> Result<()> {
        if let (Some(repo_path), Some(base_sha), Some(head_sha)) = (
            &self.config.demo_repo_path,
            &self.config.demo_base_sha,
            &self.config.demo_head_sha,
        ) {
            let repo = self.git.open_repository(repo_path).await?;
            let diff = self.git.compute_diff_to_domain(&repo, base_sha, head_sha, None).await?;
            self.demo_diff = Some(diff);
            Ok(())
        } else {
            Err(crate::error::Error::Config(
                "Demo configuration not set".to_string()
            ))
        }
    }
}
