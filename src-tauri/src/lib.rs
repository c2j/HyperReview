// HyperReview - Zero-latency code review desktop application
// Main entry point and application setup

// Enable logging
use env_logger::Env;
use std::sync::{Arc, Mutex};

// Import modules
pub mod commands;

pub mod models;
pub mod git;
pub mod errors;

pub mod analysis {
    pub mod engine;
    pub mod heatmap;
    pub mod checklist;
    pub mod stats;
    pub mod grammars;
    pub mod diff_analysis;
}

pub mod storage {
    pub mod sqlite;
    pub mod cache;
    pub mod credentials;
    pub mod task_store;
    pub mod settings;
    pub mod metadata;
    pub mod migrations;
    pub mod offline_cache;
    pub mod operation_queue;
}

pub mod search {
    pub mod service;
    pub mod index;
}

pub mod services {
    pub mod encryption;
    pub mod credential_store;
    pub mod change_downloader;
    pub mod review_session;
    pub mod file_storage;
    pub mod diff_engine;
    pub mod file_tree;
}

pub mod remote {
    pub mod client;
    pub mod gitlab_client;
    pub mod gerrit_client;
    pub mod codearts_client;
    pub mod custom_client;
}

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
    pub background_indexer: Arc<Mutex<()>>,
    /// Credential store for external systems
    pub credential_store: Arc<Mutex<storage::credentials::CredentialStore>>,
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
        
        // Initialize Gerrit schema
        database.init_gerrit_schema()
            .map_err(|e| {
                log::warn!("Failed to initialize Gerrit schema: {}", e);
                e
            })?;
        
        log::info!("Database and Gerrit schema initialized successfully");
        
        Ok(Self {
            git_service: Arc::new(Mutex::new(git::service::GitService::new())),
            cache_manager: Arc::new(storage::cache::CacheManager::new()),
            database: Arc::new(Mutex::new(database)),
            background_indexer: Arc::new(Mutex::new(())),
            credential_store: Arc::new(Mutex::new(storage::credentials::CredentialStore::new())),
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
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            // Repository management commands
            commands::general::open_repo_dialog,
            commands::general::open_repo_dialog_frontend,
            commands::general::get_recent_repos,
            commands::general::get_branches,
            commands::general::load_repo,

            // Review workflow commands
            commands::general::get_file_diff,
            commands::general::get_complete_file_diff,
            commands::general::add_comment,
            commands::general::update_comment,
            commands::general::delete_comment,
            commands::general::get_comments,

            // Template management commands
            commands::general::get_review_templates,
            commands::general::create_template,

            // Insights and analysis commands
            commands::general::get_heatmap,
            commands::general::get_file_tree,
            commands::general::get_checklist,
            commands::general::get_blame,
            commands::general::read_file_content,

            commands::general::analyze_complexity,
            commands::general::scan_security,
            commands::general::get_review_guide,

            // Local task commands
            commands::task_commands::parse_task_text,
            commands::task_commands::create_task,
            commands::task_commands::list_tasks,
            commands::task_commands::get_task,
            commands::task_commands::update_task_progress,
            commands::task_commands::read_task_item_from_ref,
            commands::task_commands::delete_task,
            commands::task_commands::archive_task,
            commands::task_commands::reimport_task_text,
            commands::task_commands::update_task,
            commands::task_commands::export_task,
            commands::task_commands::export_all_tasks,
            commands::task_commands::submit_task_to_gerrit,
            commands::task_commands::submit_task_to_codearts,
            commands::task_commands::submit_task_to_custom_api,

            commands::general::get_tags,
            commands::general::create_tag,

            // Credential management commands
            commands::general::store_gerrit_credentials,
            commands::general::get_gerrit_credentials,
            commands::general::delete_gerrit_credentials,
            commands::general::has_gerrit_credentials,

            // Gerrit instance management commands
            commands::gerrit_test::gerrit_test_connectivity,
            commands::gerrit_simple::gerrit_get_instances_simple,
            commands::gerrit_simple::gerrit_create_instance_simple,
            commands::gerrit_simple::gerrit_delete_instance_simple,
            commands::gerrit_simple::gerrit_import_change_simple,
            commands::gerrit_simple::gerrit_search_changes_simple,
            commands::gerrit_simple::gerrit_clear_all_data_simple,
            commands::gerrit_simple::gerrit_set_active_instance_simple,
            commands::gerrit_commands::gerrit_get_instances,
            commands::gerrit_commands::gerrit_create_instance,
            commands::gerrit_commands::gerrit_test_connection,
            commands::gerrit_commands::gerrit_test_connection_by_id,
            commands::gerrit_commands::gerrit_create_comment_simple,
            commands::gerrit_commands::gerrit_get_comments_simple,
            commands::gerrit_commands::gerrit_submit_review_simple,

            // Change download commands
            commands::change_download_commands::gerrit_download_change,
            commands::change_download_commands::gerrit_get_download_status,
            commands::change_download_commands::gerrit_update_change,
            commands::change_download_commands::gerrit_get_downloaded_files,
            commands::change_download_commands::gerrit_is_change_downloaded,
            commands::change_download_commands::gerrit_delete_downloaded_change,

            // Review session commands
            commands::review_session_commands::gerrit_create_review_session,
            commands::review_session_commands::gerrit_get_review_session,
            commands::review_session_commands::gerrit_update_review_session,
            commands::review_session_commands::gerrit_switch_review_mode,
            commands::review_session_commands::gerrit_update_review_progress,
            commands::review_session_commands::gerrit_get_sessions_for_reviewer,
            commands::review_session_commands::gerrit_get_active_sessions,
            commands::review_session_commands::gerrit_abandon_session,
            commands::review_session_commands::gerrit_mark_ready_for_submission,
            commands::review_session_commands::gerrit_recover_session,

            // File storage commands
            commands::file_storage_commands::file_storage_init,
            commands::file_storage_commands::file_storage_store_file,
            commands::file_storage_commands::file_storage_get_file,
            commands::file_storage_commands::file_storage_is_cached,
            commands::file_storage_commands::file_storage_list_files,
            commands::file_storage_commands::file_storage_remove_files,
            commands::file_storage_commands::file_storage_cleanup,
            commands::file_storage_commands::file_storage_get_stats,

            // Diff engine commands
            commands::diff_engine_commands::diff_generate_unified,
            commands::diff_engine_commands::diff_generate_side_by_side,
            commands::diff_engine_commands::diff_create_line_mapping,
            commands::diff_engine_commands::diff_navigate_to_line,
            commands::diff_engine_commands::diff_get_context_around_line,
            commands::diff_engine_commands::diff_get_config,
            commands::diff_engine_commands::diff_update_config,

            // File tree commands
            commands::file_tree_commands::file_tree_build,
            commands::file_tree_commands::file_tree_search,
            commands::file_tree_commands::file_tree_filter,
            commands::file_tree_commands::file_tree_get_stats,
            commands::file_tree_commands::file_tree_toggle_node,
            commands::file_tree_commands::file_tree_get_visible_nodes,
            commands::file_tree_commands::file_tree_get_default_config,
            commands::file_tree_commands::file_tree_update_config,
            commands::file_tree_commands::file_tree_create_search_criteria,

            // Search and configuration commands
            commands::general::search,
            commands::general::get_commands,

            // Persistence commands
            commands::persistence_commands::get_user_settings,
            commands::persistence_commands::save_user_setting,
            commands::persistence_commands::get_repo_selection,
            commands::persistence_commands::save_repo_selection,
            commands::persistence_commands::clear_repo_selection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    log::info!("HyperReview application stopped");
}
