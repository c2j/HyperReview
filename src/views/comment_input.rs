//! Comment input view for inline code review comments
//!
//! Provides text input with markdown support and sync status tracking.

use gpui::*;

/// Comment input view state
pub struct CommentInputView {
    /// Comment text content
    content: String,

    /// Sync status of the comment
    sync_status: CommentSyncStatus,
}

/// Sync status for comments
#[derive(Debug, Clone)]
pub enum CommentSyncStatus {
    /// Local draft, not yet synced
    Draft,
    /// Pending sync to GitHub
    Pending,
    /// Successfully synced
    Synced,
    /// Sync failed
    Failed(String),
}

impl CommentInputView {
    /// Create new comment input view
    pub fn new() -> Self {
        Self {
            content: String::new(),
            sync_status: CommentSyncStatus::Draft,
        }
    }

    /// Update comment content
    pub fn set_content(&mut self, content: String) {
        self.content = content;
        if !self.content.is_empty() && !matches!(self.sync_status, CommentSyncStatus::Pending) {
            self.sync_status = CommentSyncStatus::Draft;
        }
    }

    /// Mark comment as pending sync
    pub fn mark_pending(&mut self) {
        self.sync_status = CommentSyncStatus::Pending;
    }

    /// Mark comment as synced
    pub fn mark_synced(&mut self) {
        self.sync_status = CommentSyncStatus::Synced;
    }

    /// Mark comment as failed
    pub fn mark_failed(&mut self, error: String) {
        self.sync_status = CommentSyncStatus::Failed(error);
    }

    /// Get current content
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Check if comment is empty
    pub fn is_empty(&self) -> bool {
        self.content.trim().is_empty()
    }

    /// Get sync status
    pub fn sync_status(&self) -> &CommentSyncStatus {
        &self.sync_status
    }
}

impl Default for CommentInputView {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for CommentInputView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0x2a2a2a))
            .rounded_md()
            .p_4()
            .border(px(1.0))
            .border_color(match &self.sync_status {
                CommentSyncStatus::Draft => gpui::rgb(0x333333),
                CommentSyncStatus::Pending => gpui::rgb(0xffaa00),
                CommentSyncStatus::Synced => gpui::rgb(0x00ff00),
                CommentSyncStatus::Failed(_) => gpui::rgb(0xff0000),
            })
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x888888))
                            .child("Add a comment"),
                    )
                    .child(self.render_sync_status_badge()),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .text_color(gpui::rgb(0xcccccc))
                            .child("Tip: Use **bold**, *italic*, and `code` for formatting"),
                    )
                    .child(
                        div()
                            .text_color(gpui::rgb(0x888888))
                            .min_h(px(80.0))
                            .child("Type your comment here..."),
                    ),
            )
    }
}

impl CommentInputView {
    fn render_sync_status_badge(&self) -> impl IntoElement {
        match &self.sync_status {
            CommentSyncStatus::Draft => div()
                .text_sm()
                .text_color(gpui::rgb(0x888888))
                .child("Draft"),
            CommentSyncStatus::Pending => div()
                .text_sm()
                .text_color(gpui::rgb(0xffaa00))
                .child("⟳ Syncing..."),
            CommentSyncStatus::Synced => div()
                .text_sm()
                .text_color(gpui::rgb(0x00ff00))
                .child("✓ Synced"),
            CommentSyncStatus::Failed(error) => div()
                .text_sm()
                .text_color(gpui::rgb(0xff0000))
                .child(format!("✗ Failed: {}", error)),
        }
    }
}
