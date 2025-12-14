// HyperReview - Zero-latency code review desktop application
// Main entry point and application setup

// Enable logging
use env_logger::Env;
use std::sync::{Arc, Mutex};

// Import modules
pub mod commands;
pub mod models;
pub mod errors;

// Git operations module
pub mod git {
    pub mod service;
    pub mod diff;
}

// Static analysis module
pub mod analysis {
    pub mod engine;
    pub mod heatmap;
    pub mod checklist;
    pub mod stats;
    pub mod grammars;
}

// Storage module
pub mod storage {
    pub mod sqlite;
    pub mod cache;
    pub mod credentials;
}

// Search module
pub mod search {
    pub mod service;
    pub mod index;
}

// External system integration module
pub mod remote {
    pub mod client;
    pub mod gitlab_client;
    pub mod gerrit_client;
    pub mod codearts_client;
}

// Utility modules
pub mod utils {
    pub mod validation;
    pub mod metrics;
    pub mod memory;
}

/// Application state management
/// Thread-safe state with repository and cache management
pub struct AppState {
    /// Git repository service
    pub git_service: Arc<Mutex<git::service::GitService>>,
    /// Cache manager for diffs, blame, and analysis
    pub cache_manager: Arc<storage::cache::CacheManager>,
    /// SQLite database connection
    pub database: Arc<Mutex<storage::sqlite::Database>>,
    /// Background indexer for performance optimization
    pub background_indexer: Arc<Mutex<()>>, // TODO: Implement background indexer
}

impl AppState {
    /// Create new application state
    pub fn new() -> Result<Self, errors::HyperReviewError> {
        log::info!("Initializing application state");

        // Initialize database
        let db_path = "hyper_review.db";
        let database = storage::sqlite::Database::new(db_path)
            .map_err(errors::HyperReviewError::Database)?;

        // Initialize schema
        database.init_schema()
            .map_err(errors::HyperReviewError::Database)?;

        log::info!("Database initialized successfully");

        Ok(Self {
            git_service: Arc::new(Mutex::new(git::service::GitService::new())),
            cache_manager: Arc::new(storage::cache::CacheManager::new()),
            database: Arc::new(Mutex::new(database)),
            background_indexer: Arc::new(Mutex::new(())),
        })
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
            .expect("Failed to initialize AppState")
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    log::info!("Starting HyperReview application");

    // Initialize application state
    let app_state = match AppState::new() {
        Ok(state) => {
            log::info!("Application state initialized successfully");
            state
        }
        Err(e) => {
            log::error!("Failed to initialize application state: {}", e);
            panic!("Failed to initialize application state: {}", e);
        }
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            // Repository management commands
            commands::open_repo_dialog,
            commands::get_recent_repos,
            commands::get_branches,
            commands::load_repo,

            // Review workflow commands
            commands::get_file_diff,
            commands::add_comment,

            // Task management commands
            commands::get_tasks,
            commands::get_review_stats,
            commands::get_quality_gates,

            // Template management commands
            commands::get_review_templates,
            commands::create_template,

            // Insights and analysis commands
            commands::get_heatmap,
            commands::get_checklist,
            commands::get_blame,
            commands::analyze_complexity,
            commands::scan_security,

            // External integration commands
            commands::submit_review,
            commands::sync_repo,

            // Search and configuration commands
            commands::search,
            commands::get_commands,
            commands::get_tags,
            commands::create_tag,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    log::info!("HyperReview application stopped");
}
