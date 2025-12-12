//! Inbox view for displaying pull requests
//!
//! Shows a list of PRs that need review.

use gpui::*;
use crate::actions::{NavigationAction, InboxAction};

/// Inbox view state
pub struct InboxView {
    /// Currently selected PR index
    selected_index: usize,

    /// List of PR IDs (placeholder for now)
    pr_ids: Vec<String>,

    /// Set of selected PR indices for multi-select
    selected_prs: std::collections::HashSet<usize>,

    /// Event handler callback
    on_navigate: Option<Box<dyn Fn()>>,
}

impl InboxView {
    /// Create new inbox view
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            pr_ids: vec![],
            selected_prs: std::collections::HashSet::new(),
            on_navigate: None,
        }
    }

    /// Set navigation callback
    pub fn on_navigate(mut self, callback: Box<dyn Fn()>) -> Self {
        self.on_navigate = Some(callback);
        self
    }

    /// Move selection down
    pub fn move_down(&mut self) {
        if !self.pr_ids.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.pr_ids.len();
        }
    }

    /// Move selection up
    pub fn move_up(&mut self) {
        if !self.pr_ids.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                self.pr_ids.len() - 1
            } else {
                self.selected_index - 1
            };
        }
    }

    /// Open selected PR
    pub fn open_selected(&mut self, _cx: &mut Context<Self>) {
        if let Some(callback) = &self.on_navigate {
            callback();
        }
    }

    /// Toggle selection of current PR
    pub fn toggle_select(&mut self) {
        if !self.pr_ids.is_empty() {
            if self.selected_prs.contains(&self.selected_index) {
                self.selected_prs.remove(&self.selected_index);
            } else {
                self.selected_prs.insert(self.selected_index);
            }
        }
    }

    /// Archive selected PRs
    pub fn archive(&mut self) {
        if !self.selected_prs.is_empty() {
            let indices: Vec<usize> = self.selected_prs.iter().copied().collect();
            for &index in &indices {
                if index < self.pr_ids.len() {
                    self.pr_ids.remove(index);
                }
            }
            self.selected_prs.clear();
            if self.selected_index >= self.pr_ids.len() && !self.pr_ids.is_empty() {
                self.selected_index = self.pr_ids.len() - 1;
            }
        }
    }
}

impl Default for InboxView {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for InboxView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .size_full()
            .bg(rgb(0x1a1a1a))
            .p_6()
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(
                        div()
                            .text_3xl()
                            .font_weight(FontWeight(700.0))
                            .text_color(rgb(0x4a9eff))
                            .child("Review Inbox"),
                    )
                    .child(
                        div()
                            .bg(rgb(0x2a2a2a))
                            .rounded_md()
                            .p_2()
                            .text_color(rgb(0x888888))
                            .text_sm()
                            .child("↑↓ j/k • Enter • x • e"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_color(rgb(0x888888))
                            .text_sm()
                            .child("2 PRs to review"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .bg(gpui::rgb(0x2a2a2a))
                            .rounded_md()
                            .border(px(1.0))
                            .border_color(gpui::rgb(0x4a9eff))
                            .p_3()
                            .cursor(gpui::CursorStyle::PointingHand)
                            .child(
                                div()
                                    .flex()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_color(gpui::rgb(0xffffff))
                                            .font_weight(FontWeight(600.0))
                                            .child("#123 Fix authentication bug"),
                                    )
                                    .child(
                                        div()
                                            .text_color(gpui::rgb(0x888888))
                                            .text_sm()
                                            .child("by alice"),
                                    ),
                            )
                            .child(
                                div()
                                    .text_color(gpui::rgb(0x888888))
                                    .text_sm()
                                    .child("Add login validation and session management"),
                            )
                            .child(
                                div()
                                    .mt_2()
                                    .text_color(gpui::rgb(0x4a9eff))
                                    .text_sm()
                                    .child("Click to review →"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .bg(gpui::rgb(0x1a1a1a))
                            .rounded_md()
                            .border(px(1.0))
                            .border_color(gpui::rgb(0x333333))
                            .p_3()
                            .child(
                                div()
                                    .flex()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_color(gpui::rgb(0xcccccc))
                                            .font_weight(FontWeight(600.0))
                                            .child("#124 Update documentation"),
                                    )
                                    .child(
                                        div()
                                            .text_color(gpui::rgb(0x888888))
                                            .text_sm()
                                            .child("by bob"),
                                    ),
                            )
                            .child(
                                div()
                                    .text_color(gpui::rgb(0x888888))
                                    .text_sm()
                                    .child("Update API docs with new endpoints"),
                            ),
                    ),
            )
    }
}
