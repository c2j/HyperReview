//! Command palette view for quick actions
//!
//! Provides a searchable command palette with common actions.

use gpui::*;

/// Command palette view state
pub struct CommandPaletteView {
    /// Search query
    query: String,

    /// List of available commands
    commands: Vec<Command>,

    /// Selected command index
    selected_index: usize,
}

/// Available commands in the palette
#[derive(Debug, Clone)]
pub struct Command {
    /// Command identifier
    id: &'static str,

    /// Display name
    name: &'static str,

    /// Description
    description: &'static str,

    /// Category
    category: &'static str,
}

impl CommandPaletteView {
    /// Create new command palette
    pub fn new() -> Self {
        let commands = vec![
            Command {
                id: "refresh",
                name: "Refresh",
                description: "Refresh current view",
                category: "General",
            },
            Command {
                id: "switch_inbox",
                name: "Switch to Inbox",
                description: "Navigate to PR inbox",
                category: "Navigation",
            },
            Command {
                id: "switch_review",
                name: "Switch to Review",
                description: "Navigate to PR review",
                category: "Navigation",
            },
            Command {
                id: "start_comment",
                name: "Start Comment",
                description: "Begin typing a comment",
                category: "Review",
            },
            Command {
                id: "submit_review",
                name: "Submit Review",
                description: "Submit the current review",
                category: "Review",
            },
            Command {
                id: "archive_pr",
                name: "Archive PR",
                description: "Archive selected pull request",
                category: "PR",
            },
        ];

        Self {
            query: String::new(),
            commands,
            selected_index: 0,
        }
    }

    /// Set search query
    pub fn set_query(&mut self, query: String) {
        self.query = query;
        self.selected_index = 0;
    }

    /// Get filtered commands
    pub fn filtered_commands(&self) -> &[Command] {
        if self.query.is_empty() {
            &self.commands
        } else {
            &self.commands
        }
    }

    /// Get selected command
    pub fn selected_command(&self) -> Option<&Command> {
        let filtered = self.filtered_commands();
        filtered.get(self.selected_index)
    }

    /// Move selection down
    pub fn move_down(&mut self) {
        let filtered = self.filtered_commands();
        if !filtered.is_empty() {
            self.selected_index = (self.selected_index + 1) % filtered.len();
        }
    }

    /// Move selection up
    pub fn move_up(&mut self) {
        let filtered = self.filtered_commands();
        if !filtered.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                filtered.len() - 1
            } else {
                self.selected_index - 1
            };
        }
    }
}

impl Default for CommandPaletteView {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for CommandPaletteView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .size_full()
            .bg(gpui::rgb(0x1a1a1a))
            .p_6()
            .child(
                div()
                    .text_3xl()
                    .font_weight(FontWeight(700.0))
                    .text_color(gpui::rgb(0x4a9eff))
                    .child("Command Palette"),
            )
            .child(
                div()
                    .bg(gpui::rgb(0x2a2a2a))
                    .rounded_md()
                    .border(px(1.0))
                    .border_color(gpui::rgb(0x333333))
                    .p_4()
                    .text_color(gpui::rgb(0x888888))
                    .child("Type to search commands..."),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_color(gpui::rgb(0xcccccc))
                            .child("6 commands"),
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
                            .child(
                                div()
                                    .flex()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_color(gpui::rgb(0xffffff))
                                            .font_weight(FontWeight(600.0))
                                            .child("Refresh"),
                                    )
                                    .child(
                                        div()
                                            .text_color(gpui::rgb(0x888888))
                                            .text_sm()
                                            .child("General"),
                                    ),
                            )
                            .child(
                                div()
                                    .text_color(gpui::rgb(0x888888))
                                    .text_sm()
                                    .child("Refresh current view"),
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
                                            .child("Switch to Inbox"),
                                    )
                                    .child(
                                        div()
                                            .text_color(gpui::rgb(0x888888))
                                            .text_sm()
                                            .child("Navigation"),
                                    ),
                            )
                            .child(
                                div()
                                    .text_color(gpui::rgb(0x888888))
                                    .text_sm()
                                    .child("Navigate to PR inbox"),
                            ),
                    ),
            )
            .child(
                div()
                    .text_color(gpui::rgb(0x666666))
                    .text_sm()
                    .child("↑↓ Navigate • Enter Select • Esc Close"),
            )
    }
}
