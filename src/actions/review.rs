//! Review actions for PR review workflow
//!
//! Defines actions specific to reviewing pull requests.

/// Review-specific actions
#[derive(Debug, Clone)]
pub enum ReviewAction {
    /// Start a new comment (r key)
    StartComment,
    /// Submit comment (Cmd+Enter)
    SubmitComment,
}
