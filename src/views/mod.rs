//! Views module for HyperReview
//!
//! Contains all GPUI View components.

pub mod auth;
pub mod comment_input;
pub mod review_submit;
pub mod workspace;
pub mod diff;
pub mod inbox;
pub mod command_palette;

pub use auth::*;
pub use comment_input::*;
pub use review_submit::*;
pub use workspace::Workspace;
pub use diff::{DiffView, DiffViewEvent, create_diff_view};
pub use inbox::*;
pub use command_palette::*;
