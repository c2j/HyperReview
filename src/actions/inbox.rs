//! Inbox actions for PR list management
//!
//! Defines actions specific to the inbox view.

/// Inbox-specific actions
#[derive(Debug, Clone)]
pub enum InboxAction {
    /// Toggle selection of an item (x key)
    ToggleSelect,
    /// Archive selected item (e key)
    Archive,
}
