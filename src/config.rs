//! Configuration module for HyperReview
//!
//! Provides application configuration including hardcoded demo paths.

use std::path::PathBuf;

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// Path to local git repository for demo (Phase 1)
    pub demo_repo_path: Option<PathBuf>,

    /// Base SHA for diff comparison
    pub demo_base_sha: Option<String>,

    /// Head SHA for diff comparison
    pub demo_head_sha: Option<String>,

    /// GitHub OAuth client ID
    pub github_client_id: Option<String>,

    /// Database path
    pub database_path: PathBuf,

    /// Cache directory for repository clones
    pub cache_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let data_dir = home.join(".hyperreview");

        Self {
            demo_repo_path: None,
            demo_base_sha: None,
            demo_head_sha: None,
            github_client_id: std::env::var("HYPERREVIEW_GITHUB_CLIENT_ID").ok(),
            database_path: data_dir.join("hyperreview.db"),
            cache_dir: data_dir.join("repos"),
        }
    }
}

impl Config {
    /// Create a new configuration with demo repository path
    pub fn with_demo_repo(mut self, path: PathBuf, base_sha: String, head_sha: String) -> Self {
        self.demo_repo_path = Some(path);
        self.demo_base_sha = Some(base_sha);
        self.demo_head_sha = Some(head_sha);
        self
    }
}
