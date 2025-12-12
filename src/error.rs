//! Error types for HyperReview
//!
//! Provides domain-specific error types following Constitution III requirements.

use thiserror::Error;

/// Result type alias using HyperReview Error
pub type Result<T> = std::result::Result<T, Error>;

/// Domain error types for HyperReview
#[derive(Error, Debug)]
pub enum Error {
    /// Git operation errors
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    /// Database errors
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// HTTP/API errors
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization errors
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Authentication errors
    #[error("Authentication failed: {0}")]
    Auth(String),

    /// OAuth2 device flow errors
    #[error("OAuth error: {0}")]
    OAuth(String),

    /// Token expired
    #[error("Token expired, please re-authenticate")]
    TokenExpired,

    /// Rate limit exceeded
    #[error("Rate limit exceeded, retry after {0} seconds")]
    RateLimited(u64),

    /// Resource not found
    #[error("Not found: {0}")]
    NotFound(String),

    /// Invalid ID format
    #[error("Invalid ID format: {0}")]
    InvalidId(String),

    /// Sync error
    #[error("Sync failed: {0}")]
    SyncFailed(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Generic internal error
    #[error("Internal error: {0}")]
    Internal(String),
}

impl Error {
    /// Check if error is recoverable (user can retry)
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Error::Http(_) | Error::RateLimited(_) | Error::SyncFailed(_)
        )
    }

    /// Get user-friendly error message
    pub fn user_message(&self) -> String {
        match self {
            Error::Auth(msg) => format!("Authentication failed. {}", msg),
            Error::TokenExpired => "Your session has expired. Please sign in again.".into(),
            Error::RateLimited(secs) => {
                format!("GitHub rate limit reached. Please wait {} seconds.", secs)
            }
            Error::NotFound(what) => format!("{} not found. It may have been deleted.", what),
            Error::SyncFailed(_) => "Failed to sync with GitHub. Will retry automatically.".into(),
            _ => "An unexpected error occurred. Please try again.".into(),
        }
    }
}
