// Error handling infrastructure
// Custom error types using thiserror

use thiserror::Error;

/// Result type alias for HyperReview operations
pub type Result<T> = std::result::Result<T, HyperReviewError>;

/// Main error type for HyperReview
#[derive(Error, Debug)]
pub enum HyperReviewError {
    /// Git operation errors
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    /// Database operation errors
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    /// I/O operation errors
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Repository not found
    #[error("Repository not found: {path}")]
    RepositoryNotFound {
        path: String,
    },

    /// Invalid repository path
    #[error("Invalid repository path: {path}")]
    InvalidPath {
        path: String,
    },

    /// Permission denied
    #[error("Permission denied: {operation}")]
    PermissionDenied {
        operation: String,
    },

    /// Validation errors
    #[error("Validation error: {message}")]
    Validation {
        message: String,
        field: Option<String>,
    },

    /// Network errors
    #[error("Network error: {message}")]
    Network {
        message: String,
        status_code: Option<u16>,
    },

    /// Cache errors
    #[error("Cache error: {message}")]
    Cache {
        message: String,
    },

    /// Analysis errors
    #[error("Analysis error: {message}")]
    Analysis {
        message: String,
    },

    /// Configuration errors
    #[error("Configuration error: {message}")]
    Config {
        message: String,
    },

    /// Serialization/deserialization errors
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Other errors
    #[error("Error: {message}")]
    Other {
        message: String,
    },
}

impl HyperReviewError {
    /// Create a repository not found error
    pub fn repository_not_found(path: String) -> Self {
        Self::RepositoryNotFound { path }
    }

    /// Create an invalid path error
    pub fn invalid_path(path: String) -> Self {
        Self::InvalidPath { path }
    }

    /// Create a permission denied error
    pub fn permission_denied(operation: String) -> Self {
        Self::PermissionDenied { operation }
    }

    /// Create a validation error
    pub fn validation(message: String, field: Option<String>) -> Self {
        Self::Validation { message, field }
    }

    /// Create a network error
    pub fn network(message: String) -> Self {
        Self::Network {
            message,
            status_code: None,
        }
    }

    /// Create a network error with status code
    pub fn network_with_status(message: String, status_code: u16) -> Self {
        Self::Network {
            message,
            status_code: Some(status_code),
        }
    }

    /// Create a cache error
    pub fn cache(message: String) -> Self {
        Self::Cache { message }
    }

    /// Create an analysis error
    pub fn analysis(message: String) -> Self {
        Self::Analysis { message }
    }

    /// Create a configuration error
    pub fn config(message: String) -> Self {
        Self::Config { message }
    }

    /// Create an other error
    pub fn other(message: String) -> Self {
        Self::Other { message }
    }
}

/// Helper macros for error creation

/// Create a Git error
#[macro_export]
macro_rules! git_error {
    ($($arg:tt)*) => {
        HyperReviewError::git(format!($($arg)*), git2::Error::from_str("Git operation failed"))
    };
}

/// Create a database error
#[macro_export]
macro_rules! db_error {
    ($($arg:tt)*) => {
        HyperReviewError::database(format!($($arg)*), rusqlite::Error::InvalidQuery)
    };
}

/// Create a validation error
#[macro_export]
macro_rules! validation_error {
    ($($arg:tt)*) => {
        HyperReviewError::validation(format!($($arg)*), None)
    };
}

/// Create a validation error with field
#[macro_export]
macro_rules! validation_error_field {
    ($field:expr, $($arg:tt)*) => {
        HyperReviewError::validation(format!($($arg)*), Some($field.to_string()))
    };
}
