//! Workspace root view for HyperReview
//!
//! Following Constitution IV: All UI elements MUST be implemented as GPUI Views.

use std::sync::Arc;

use crate::app::AppState;

/// Root workspace view (simplified for Milestone 1)
pub struct Workspace {
    app_state: Arc<AppState>,
}

impl Workspace {
    /// Create new workspace view (simplified)
    pub fn new(app_state: Arc<AppState>, _cx: &()) -> Self {
        let mut workspace = Self { app_state };

        // For Milestone 1 demo, set up hardcoded configuration
        if let Some(ref mut app_state) = Arc::get_mut(&mut workspace.app_state) {
            // Set demo configuration from current directory
            app_state.config = app_state.config.clone().with_demo_repo(
                std::env::current_dir().unwrap_or_default(),
                "HEAD~1".to_string(),
                "HEAD".to_string(),
            );
        }

        workspace
    }

    /// Initialize diff view with loaded diff (simplified)
    pub fn init_diff_view(&mut self, _app_state: &(), _cx: &()) {
        // Placeholder for Milestone 1
    }
}
