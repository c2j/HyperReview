//! HyperReview - Native, GPU-accelerated PR review application
//!
//! This application provides a unified inbox for GitHub PRs with high-performance
//! diff viewing, syntax highlighting, and keyboard-driven workflow.

use gpui::{
    prelude::*, Application, Bounds, Context, FontWeight, Window, WindowBounds, WindowOptions, div,
    px, rgb, size,
};
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod app;
mod config;
mod error;
mod models;
mod services;
mod views;
mod actions;

use app::{AppState, ViewState};

struct MainView {
    app_state: Arc<AppState>,
}

impl MainView {
    pub fn new(app_state: Arc<AppState>, _cx: &mut Context<Self>) -> Self {
        Self { app_state }
    }

    fn switch_to_auth(&mut self) {
        if let Some(app_state) = Arc::get_mut(&mut self.app_state) {
            app_state.current_view = ViewState::Auth;
        }
    }

    fn switch_to_inbox(&mut self) {
        if let Some(app_state) = Arc::get_mut(&mut self.app_state) {
            app_state.current_view = ViewState::Inbox;
        }
    }

    fn switch_to_review(&mut self, pr_id: String) {
        if let Some(app_state) = Arc::get_mut(&mut self.app_state) {
            app_state.current_view = ViewState::Review { pr_id };
        }
    }
}

impl Render for MainView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        match &self.app_state.current_view {
            ViewState::Auth => {
                let app_state = self.app_state.clone();
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
                            .child("HyperReview - Auth View"),
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
                            .child("Press '2' for Inbox, '3' for Review"),
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
                                    .child("Click to continue →")
                                    .on_mouse_down(gpui::MouseButton::Left, |_event, _window, _cx| {
                                        println!("Button clicked!");
                                    }),
                            ),
                    )
            }
            ViewState::Inbox => {
                let app_state = Arc::clone(&self.app_state);
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
                                    .child("Click PRs • Press '1' '3' to switch views"),
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
                                    .on_mouse_down(gpui::MouseButton::Left, move |_event, _window, _cx| {
                                        if let Some(state) = Arc::get_mut(&mut app_state.clone()) {
                                            state.current_view = ViewState::Review { pr_id: "123".to_string() };
                                        }
                                    })
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
            ViewState::Review { pr_id } => {
                let app_state = self.app_state.clone();
                div()
                    .flex()
                    .flex_col()
                    .size_full()
                    .gap_4()
                    .p_6()
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .items_center()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_3()
                                    .child(
                                        div()
                                            .text_2xl()
                                            .font_weight(FontWeight(700.0))
                                            .text_color(rgb(0x4a9eff))
                                            .child(format!("PR #{}", pr_id)),
                                    )
                                    .child(
                                        div()
                                            .text_color(rgb(0x888888))
                                            .text_sm()
                                            .child("Fix authentication bug"),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap_3()
                                    .child(
                                        div()
                                            .on_mouse_down(gpui::MouseButton::Left, move |_event, _window, _cx| {
                                                if let Some(state) = Arc::get_mut(&mut app_state.clone()) {
                                                    state.current_view = ViewState::Inbox;
                                                }
                                            })
                                            .bg(rgb(0x2a2a2a))
                                            .rounded_md()
                                            .p_2()
                                            .text_color(rgb(0x888888))
                                            .text_sm()
                                            .cursor(gpui::CursorStyle::PointingHand)
                                            .child("← Back to Inbox"),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_4()
                            .size_full()
                            .child(
                                div()
                                    .flex_1()
                                    .bg(rgb(0x2a2a2a))
                                    .rounded_md()
                                    .p_4()
                                    .border(px(1.0))
                                    .border_color(rgb(0x333333))
                                    .child(
                                        div()
                                            .text_color(rgb(0x888888))
                                            .child("DiffView - Code changes will appear here"),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_4()
                                    .w(px(400.0))
                                    .child(
                                        div()
                                            .bg(rgb(0x2a2a2a))
                                            .rounded_md()
                                            .p_4()
                                            .border(px(1.0))
                                            .border_color(rgb(0x333333))
                                            .child(
                                                div()
                                                    .text_color(rgb(0xcccccc))
                                                    .text_sm()
                                                    .child("Type your comment here..."),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .bg(rgb(0x2a2a2a))
                                            .rounded_md()
                                            .p_4()
                                            .border(px(1.0))
                                            .border_color(gpui::rgb(0x333333))
                                            .child(
                                                div()
                                                    .text_color(gpui::rgb(0xcccccc))
                                                    .child("Review Decision"),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .gap_3()
                                                    .mt_3()
                                                    .child(
                                                        div()
                                                            .bg(gpui::rgb(0x4a9eff))
                                                            .rounded_md()
                                                            .p_3()
                                                            .border(px(1.0))
                                                            .border_color(gpui::rgb(0x4a9eff))
                                                            .text_color(gpui::rgb(0xffffff))
                                                            .child("Approve"),
                                                    )
                                                    .child(
                                                        div()
                                                            .bg(gpui::rgb(0x2a2a2a))
                                                            .rounded_md()
                                                            .p_3()
                                                            .border(px(1.0))
                                                            .border_color(gpui::rgb(0x333333))
                                                            .text_color(gpui::rgb(0xcccccc))
                                                            .child("Request Changes"),
                                                    )
                                                    .child(
                                                        div()
                                                            .bg(gpui::rgb(0x2a2a2a))
                                                            .rounded_md()
                                                            .p_3()
                                                            .border(px(1.0))
                                                            .border_color(gpui::rgb(0x333333))
                                                            .text_color(gpui::rgb(0xcccccc))
                                                            .child("Comment"),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .bg(rgb(0x2a2a2a))
                                                    .rounded_md()
                                                    .p_4()
                                                    .border(px(1.0))
                                                    .border_color(gpui::rgb(0x333333))
                                                    .mt_4()
                                                    .child(
                                                        div()
                                                            .text_color(gpui::rgb(0x888888))
                                                            .child("Type your review summary here..."),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .justify_end()
                                                    .mt_4()
                                                    .child(
                                                        div()
                                                            .bg(gpui::rgb(0x4a9eff))
                                                            .text_color(gpui::rgb(0xffffff))
                                                            .rounded_md()
                                                            .p_3()
                                                            .cursor(gpui::CursorStyle::PointingHand)
                                                            .child("Submit Review"),
                                                    ),
                                            ),
                                    ),
                            ),
                    )
            }
        }
        .child(
            div()
                .absolute()
                .bottom_4()
                .left_4()
                .bg(rgb(0x2a2a2a))
                .rounded_md()
                .p_3()
                .text_color(rgb(0x888888))
                .text_sm()
                .child("Press '1' Auth • '2' Inbox • '3' Review"),
        )
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    tracing::info!("Starting HyperReview...");

    // Initialize application state
    let app_state = Arc::new(AppState::new(&()));

    // Launch GPUI application
    Application::new().run(move |cx: &mut gpui::App| {
        let bounds = Bounds::centered(None, size(px(1200.), px(800.)), cx);

        // Create window
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|cx| MainView::new(app_state.clone(), cx))
            },
        )
        .unwrap();
        cx.activate(true);
    });

    Ok(())
}
