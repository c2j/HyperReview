//! Database service for HyperReview
//!
//! SQLite operations following Constitution II (Offline-First) requirements.

use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::path::Path;

use crate::error::Result;

/// Database service for SQLite operations
pub struct DbService {
    pool: SqlitePool,
}

impl DbService {
    /// Create new database service and run migrations
    pub async fn new(path: &Path) -> Result<Self> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let database_url = format!("sqlite:{}?mode=rwc", path.display());

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .map_err(|e| crate::error::Error::Internal(format!("Migration failed: {}", e)))?;

        Ok(Self { pool })
    }

    /// Create in-memory database for testing
    #[cfg(test)]
    pub async fn new_in_memory() -> Result<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await?;

        Ok(Self { pool })
    }

    /// Get a reference to the connection pool
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}
