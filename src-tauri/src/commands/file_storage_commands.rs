// File Storage Management Commands
// Tauri commands for managing local file storage and caching

use std::sync::Arc;
use tauri::State;
use log::{info, error};

use crate::AppState;
use crate::services::file_storage::{FileStorage, FileStorageConfig, CacheStats};
use crate::models::gerrit::ChangeFile;

/// Initialize file storage with configuration
#[tauri::command]
pub async fn file_storage_init(
    config: FileStorageConfig,
    _state: State<'_, AppState>,
) -> Result<CacheStats, String> {
    info!("Initializing file storage");

    tokio::task::spawn_blocking(move || {
        match FileStorage::new(config) {
            Ok(storage) => {
                let stats = storage.get_cache_stats();
                info!("File storage initialized successfully");
                Ok(stats)
            }
            Err(e) => {
                error!("Failed to initialize file storage: {}", e);
                Err(format!("Failed to initialize file storage: {}", e))
            }
        }
    }).await.map_err(|e| format!("Task join error: {}", e))?
}

/// Store a change file locally
#[tauri::command]
pub async fn file_storage_store_file(
    change_file: ChangeFile,
    change_id: String,
    patch_set_number: u32,
    _state: State<'_, AppState>,
) -> Result<bool, String> {
    info!("Storing file: {} for change {}", change_file.file_path, change_id);

    tokio::task::spawn_blocking(move || {
        let config = FileStorageConfig::default();
        let mut storage = FileStorage::new(config)
            .map_err(|e| format!("Failed to create storage: {}", e))?;

        match storage.store_file(&change_file, &change_id, patch_set_number) {
            Ok(result) => {
                info!("Successfully stored file: {}", change_file.file_path);
                Ok(result.success)
            }
            Err(e) => {
                error!("Failed to store file: {}", e);
                Err(format!("Failed to store file: {}", e))
            }
        }
    }).await.map_err(|e| format!("Task join error: {}", e))?
}

/// Retrieve a file from local storage
#[tauri::command]
pub async fn file_storage_get_file(
    change_id: String,
    patch_set_number: u32,
    file_path: String,
    _state: State<'_, AppState>,
) -> Result<Option<String>, String> {
    info!("Retrieving file: {} for change {}", file_path, change_id);

    tokio::task::spawn_blocking(move || {
        let config = FileStorageConfig::default();
        let mut storage = FileStorage::new(config)
            .map_err(|e| format!("Failed to create storage: {}", e))?;

        match storage.get_file(&change_id, patch_set_number, &file_path) {
            Ok(content) => {
                if content.is_some() {
                    info!("Successfully retrieved file: {}", file_path);
                } else {
                    info!("File not found in cache: {}", file_path);
                }
                Ok(content)
            }
            Err(e) => {
                error!("Failed to retrieve file: {}", e);
                Err(format!("Failed to retrieve file: {}", e))
            }
        }
    }).await.map_err(|e| format!("Task join error: {}", e))?
}

/// Check if a file is cached
#[tauri::command]
pub async fn file_storage_is_cached(
    change_id: String,
    patch_set_number: u32,
    file_path: String,
    _state: State<'_, AppState>,
) -> Result<bool, String> {
    tokio::task::spawn_blocking(move || {
        let config = FileStorageConfig::default();
        let storage = FileStorage::new(config)
            .map_err(|e| format!("Failed to create storage: {}", e))?;

        let is_cached = storage.is_file_cached(&change_id, patch_set_number, &file_path);
        Ok(is_cached)
    }).await.map_err(|e| format!("Task join error: {}", e))?
}

/// List cached files for a change
#[tauri::command]
pub async fn file_storage_list_files(
    change_id: String,
    patch_set_number: Option<u32>,
    _state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    info!("Listing cached files for change: {}", change_id);

    tokio::task::spawn_blocking(move || {
        let config = FileStorageConfig::default();
        let storage = FileStorage::new(config)
            .map_err(|e| format!("Failed to create storage: {}", e))?;

        let files = storage.list_cached_files(&change_id, patch_set_number);
        let file_paths: Vec<String> = files.iter()
            .map(|info| info.file_path.clone())
            .collect();

        info!("Found {} cached files for change: {}", file_paths.len(), change_id);
        Ok(file_paths)
    }).await.map_err(|e| format!("Task join error: {}", e))?
}

/// Remove cached files for a change
#[tauri::command]
pub async fn file_storage_remove_files(
    change_id: String,
    patch_set_number: Option<u32>,
    _state: State<'_, AppState>,
) -> Result<u32, String> {
    info!("Removing cached files for change: {}", change_id);

    tokio::task::spawn_blocking(move || {
        let config = FileStorageConfig::default();
        let mut storage = FileStorage::new(config)
            .map_err(|e| format!("Failed to create storage: {}", e))?;

        match storage.remove_change_files(&change_id, patch_set_number) {
            Ok(removed_count) => {
                info!("Successfully removed {} files for change: {}", removed_count, change_id);
                Ok(removed_count)
            }
            Err(e) => {
                error!("Failed to remove files: {}", e);
                Err(format!("Failed to remove files: {}", e))
            }
        }
    }).await.map_err(|e| format!("Task join error: {}", e))?
}

/// Clean up old cached files
#[tauri::command]
pub async fn file_storage_cleanup(
    days_threshold: u32,
    _state: State<'_, AppState>,
) -> Result<u32, String> {
    info!("Cleaning up files older than {} days", days_threshold);

    tokio::task::spawn_blocking(move || {
        let config = FileStorageConfig::default();
        let mut storage = FileStorage::new(config)
            .map_err(|e| format!("Failed to create storage: {}", e))?;

        match storage.cleanup_old_files(days_threshold) {
            Ok(removed_count) => {
                info!("Successfully cleaned up {} old files", removed_count);
                Ok(removed_count)
            }
            Err(e) => {
                error!("Failed to cleanup files: {}", e);
                Err(format!("Failed to cleanup files: {}", e))
            }
        }
    }).await.map_err(|e| format!("Task join error: {}", e))?
}

/// Get cache statistics
#[tauri::command]
pub async fn file_storage_get_stats(
    _state: State<'_, AppState>,
) -> Result<CacheStats, String> {
    tokio::task::spawn_blocking(move || {
        let config = FileStorageConfig::default();
        let storage = FileStorage::new(config)
            .map_err(|e| format!("Failed to create storage: {}", e))?;

        let stats = storage.get_cache_stats();
        Ok(stats)
    }).await.map_err(|e| format!("Task join error: {}", e))?
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::gerrit::FileChangeType;

    fn create_test_change_file() -> ChangeFile {
        use crate::models::gerrit::FileDiff;
        
        ChangeFile {
            id: "test-file-id".to_string(),
            change_id: "test-change".to_string(),
            patch_set_number: 1,
            file_path: "src/main.rs".to_string(),
            change_type: FileChangeType::Modified,
            old_content: Some("old content".to_string()),
            new_content: Some("fn main() {}".to_string()),
            diff: FileDiff::default(),
            file_size: 12,
            downloaded_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }

    #[tokio::test]
    async fn test_file_storage_config_creation() {
        let config = FileStorageConfig::default();
        assert_eq!(config.max_cache_size_mb, 1024);
        assert_eq!(config.cleanup_threshold_days, 30);
        assert!(!config.enable_compression);
    }

    #[tokio::test]
    async fn test_change_file_creation() {
        let change_file = create_test_change_file();
        assert_eq!(change_file.file_path, "src/main.rs");
        assert_eq!(change_file.change_type, FileChangeType::Modified);
        assert_eq!(change_file.file_size, 12);
    }
}