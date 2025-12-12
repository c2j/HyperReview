//! Review submit view for submitting PR reviews
//!
//! Provides decision selection (Approve/Request Changes/Comment) and final submission.

use gpui::*;
use std::rc::Rc;

/// Review decision type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReviewDecision {
    Approve,
    RequestChanges,
    Comment,
}

impl ReviewDecision {
    /// Get display name
    pub fn as_str(&self) -> &'static str {
        match self {
            ReviewDecision::Approve => "Approve",
            ReviewDecision::RequestChanges => "Request Changes",
            ReviewDecision::Comment => "Comment",
        }
    }
}

/// Review submit event
#[derive(Debug, Clone)]
pub enum ReviewSubmitEvent {
    DecisionChanged(ReviewDecision),
    SummaryChanged(String),
    SubmitReview,
}

/// Review submit view state
pub struct ReviewSubmitView {
    /// Selected review decision
    decision: Option<ReviewDecision>,

    /// Summary comment
    summary: String,

    /// Number of pending inline comments
    pending_comments_count: usize,

    /// Event handler callback
    on_event: Option<Rc<dyn Fn(ReviewSubmitEvent)>>,
}

impl ReviewSubmitView {
    /// Create new review submit view
    pub fn new(pending_comments_count: usize) -> Self {
        Self {
            decision: None,
            summary: String::new(),
            pending_comments_count,
            on_event: None,
        }
    }

    /// Set event handler
    pub fn on_event(mut self, handler: Rc<dyn Fn(ReviewSubmitEvent)>) -> Self {
        self.on_event = Some(handler);
        self
    }

    /// Set review decision
    pub fn set_decision(&mut self, decision: ReviewDecision) {
        self.decision = Some(decision);
    }

    /// Get selected decision
    pub fn decision(&self) -> Option<ReviewDecision> {
        self.decision
    }

    /// Get pending comments count
    pub fn pending_comments_count(&self) -> usize {
        self.pending_comments_count
    }
}

impl Render for ReviewSubmitView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                    .child("Submit Review"),
            )
            .child(
                div()
                    .text_color(gpui::rgb(0xffffff))
                    .child(format!(
                        "{} pending comment{}",
                        self.pending_comments_count,
                        if self.pending_comments_count != 1 { "s" } else { "" }
                    )),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(
                        div()
                            .text_color(gpui::rgb(0xcccccc))
                            .child("Review Decision"),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_3()
                            .child(self.render_decision_button(ReviewDecision::Approve, None))
                            .child(self.render_decision_button(ReviewDecision::RequestChanges, None))
                            .child(self.render_decision_button(ReviewDecision::Comment, None)),
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(
                        div()
                            .text_color(gpui::rgb(0xcccccc))
                            .child("Summary (optional)"),
                    )
                    .child(
                        div()
                            .bg(gpui::rgb(0x2a2a2a))
                            .rounded_md()
                            .border(px(1.0))
                            .border_color(gpui::rgb(0x333333))
                            .min_h(px(100.0))
                            .p_4()
                            .text_color(gpui::rgb(0x888888))
                            .child("Type your review summary here..."),
                    )
            )
            .child(
                div()
                    .flex()
                    .justify_end()
                    .child(
                        div()
                            .bg(gpui::rgb(0x4a9eff))
                            .text_color(gpui::rgb(0xffffff))
                            .rounded_md()
                            .p_3()
                            .cursor(gpui::CursorStyle::PointingHand)
                            .child("Submit Review"),
                    )
            )
    }
}

impl ReviewSubmitView {
    fn render_decision_button(&self, decision_type: ReviewDecision, selected: Option<ReviewDecision>) -> impl IntoElement {
        let is_selected = selected == Some(decision_type);

        div()
            .flex()
            .items_center()
            .gap_2()
            .bg(if is_selected {
                gpui::rgb(0x4a9eff)
            } else {
                gpui::rgb(0x2a2a2a)
            })
            .rounded_md()
            .p_3()
            .border(px(1.0))
            .border_color(if is_selected {
                gpui::rgb(0x4a9eff)
            } else {
                gpui::rgb(0x333333)
            })
            .text_color(if is_selected {
                gpui::rgb(0xffffff)
            } else {
                gpui::rgb(0xcccccc)
            })
            .child(decision_type.as_str())
    }
}
