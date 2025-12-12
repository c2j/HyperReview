//! Authentication view for OAuth2 device flow
//!
//! Displays device code and verification URL for GitHub authentication.

use gpui::*;
use std::sync::Arc;
use crate::app::AppState;

/// Authentication view
pub struct AuthView {
    /// Current authentication state
    auth_state: AuthState,

    /// Reference to app state
    app_state: Option<Arc<AppState>>,
}

#[derive(Debug, Clone)]
pub enum AuthState {
    Initial,
    WaitingForUser,
    Polling,
    Success,
    Failed(String),
}

impl AuthView {
    /// Create new authentication view
    pub fn new() -> Self {
        Self {
            auth_state: AuthState::Initial,
            app_state: None,
        }
    }

    /// Set app state reference
    pub fn with_app_state(mut self, app_state: Arc<AppState>) -> Self {
        self.app_state = Some(app_state);
        self
    }

    /// Check if authenticated
    pub fn is_authenticated(&self) -> bool {
        matches!(self.auth_state, AuthState::Success)
    }

    /// Navigate to Inbox view
    pub fn navigate_to_inbox(&mut self) {
        if let Some(app_state) = &self.app_state {
            // This would need to be called from a context with mutable access
        }
    }
}

impl Default for AuthView {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for AuthView {
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
                    .text_3xl()
                    .font_weight(FontWeight(700.0))
                    .text_color(rgb(0x4a9eff))
                    .child("HyperReview"),
            )
            .child(
                div()
                    .text_lg()
                    .text_color(rgb(0xffffff))
                    .child("Connect to GitHub to view PRs"),
            )
            .child(
                div()
                    .text_color(rgb(0x888888))
                    .mt_4()
                    .child("Demo Mode: Click the button below to explore the UI"),
            )
            .child(
                div()
                    .flex()
                    .gap_3()
                    .mt_6()
                    .child(
                        div()
                            .bg(rgb(0x4a9eff))
                            .text_color(rgb(0xffffff))
                            .rounded_md()
                            .p_3()
                            .cursor(gpui::CursorStyle::PointingHand)
                            .child("Continue to Inbox (Demo)"),
                    ),
            )
    }
}
