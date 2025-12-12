//! Navigation actions for keyboard shortcuts
//!
//! Defines all navigation-related actions used throughout the application.

/// Navigation actions
#[derive(Debug, Clone)]
pub enum NavigationAction {
    /// Move down in a list (j key)
    MoveDown,
    /// Move up in a list (k key)
    MoveUp,
    /// Open selected item (Enter key)
    OpenSelected,
    /// Move to next hunk/chunk (n key)
    NextHunk,
    /// Move to previous hunk/chunk (p key)
    PrevHunk,
    /// Open command palette (Cmd+K)
    OpenCommandPalette,
}
