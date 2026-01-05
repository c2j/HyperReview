// Tauri commands for change download functionality

use std::sync::Arc;
use tauri::State;
use log::{info, error};

use crate::services::change_downloader::{DownloadResult, DownloadStatus, UpdateResult};
use crate::remote::gerrit_client::GerritClient;
use crate::storage::sqlite::Database;
use crate::AppState;

/// Download a Gerrit change for offline review
#[tauri::command]
pub async fn gerrit_download_change(
    instance_id: String,
    change_id: String,
    patch_set_number: Option<u32>,
) -> Result<DownloadResult, String> {
    info!("Download change command: {} from instance {}", change_id, instance_id);

    tokio::task::spawn_blocking(move || {
        // Create database connection
        let database = Database::new("hyper_review.db")
            .map_err(|e| format!("Database error: {}", e))?;

        // Get Gerrit instance configuration
        let instance = database.get_gerrit_instance(&instance_id)
            .map_err(|e| format!("Failed to get instance: {}", e))?
            .ok_or_else(|| "Gerrit instance not found".to_string())?;

        // Create Gerrit client
        let gerrit_client = Arc::new(
            GerritClient::new(&instance.url)
                .with_auth(instance.username.clone(), instance.password_encrypted.clone())
        );

        // Create database reference
        let db_ref = Arc::new(database);

        // Create downloader
        let downloader = crate::services::change_downloader::ChangeDownloader::new(gerrit_client, db_ref);

        // Use tokio runtime for async operations within blocking context
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| format!("Failed to create runtime: {}", e))?;

        // Perform download
        rt.block_on(async {
            match downloader.download_change(&change_id, patch_set_number).await {
                Ok(result) => {
                    info!("Download completed successfully: {} files", result.files.len());
                    Ok(result)
                }
                Err(e) => {
                    error!("Download failed: {}", e);
                    Err(format!("Download failed: {}", e))
                }
            }
        })
    }).await.map_err(|e| format!("Task join error: {}", e))?
}

/// Get download status for a change
#[tauri::command]
pub async fn gerrit_get_download_status(
    change_id: String,
    patch_set_number: u32,
) -> Result<DownloadStatus, String> {
    info!("Get download status: {} PS{}", change_id, patch_set_number);

    tokio::task::spawn_blocking(move || {
        // Create database connection
        let database = Database::new("hyper_review.db")
            .map_err(|e| format!("Database error: {}", e))?;

        // Create database reference
        let db_ref = Arc::new(database);

        // Create a minimal downloader (we only need database access for status)
        let gerrit_client = Arc::new(GerritClient::new("http://localhost"));
        let downloader = crate::services::change_downloader::ChangeDownloader::new(gerrit_client, db_ref);

        // Use tokio runtime for async operations within blocking context
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| format!("Failed to create runtime: {}", e))?;

        rt.block_on(async {
            match downloader.get_download_status(&change_id, patch_set_number).await {
                Ok(status) => {
                    info!("Download status retrieved: downloaded={}", status.is_downloaded);
                    Ok(status)
                }
                Err(e) => {
                    error!("Failed to get download status: {}", e);
                    Err(format!("Failed to get download status: {}", e))
                }
            }
        })
    }).await.map_err(|e| format!("Task join error: {}", e))?
}

/// Update an existing downloaded change
#[tauri::command]
pub async fn gerrit_update_change(
    instance_id: String,
    change_id: String,
) -> Result<UpdateResult, String> {
    info!("Update change command: {} from instance {}", change_id, instance_id);

    tokio::task::spawn_blocking(move || {
        // Create database connection
        let database = Database::new("hyper_review.db")
            .map_err(|e| format!("Database error: {}", e))?;

        // Get Gerrit instance configuration
        let instance = database.get_gerrit_instance(&instance_id)
            .map_err(|e| format!("Failed to get instance: {}", e))?
            .ok_or_else(|| "Gerrit instance not found".to_string())?;

        // Create Gerrit client
        let gerrit_client = Arc::new(
            GerritClient::new(&instance.url)
                .with_auth(instance.username.clone(), instance.password_encrypted.clone())
        );

        // Create database reference
        let db_ref = Arc::new(database);

        // Create downloader
        let downloader = crate::services::change_downloader::ChangeDownloader::new(gerrit_client, db_ref);

        // Use tokio runtime for async operations within blocking context
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| format!("Failed to create runtime: {}", e))?;

        rt.block_on(async {
            match downloader.update_change(&change_id).await {
                Ok(result) => {
                    info!("Update completed: updated={}, files_changed={}", result.updated, result.files_changed);
                    Ok(result)
                }
                Err(e) => {
                    error!("Update failed: {}", e);
                    Err(format!("Update failed: {}", e))
                }
            }
        })
    }).await.map_err(|e| format!("Task join error: {}", e))?
}

/// Get downloaded files for a change
#[tauri::command]
pub async fn gerrit_get_downloaded_files(
    state: State<'_, AppState>,
    change_id: String,
    patch_set_number: u32,
) -> Result<Vec<crate::models::gerrit::ChangeFile>, String> {
    info!("Get downloaded files: {} PS{}", change_id, patch_set_number);

    let files = {
        let database = state.database.lock().unwrap();
        database.get_change_files_by_gerrit_id(&change_id, patch_set_number)
            .map_err(|e| format!("Failed to get downloaded files: {}", e))?
    };
    
    info!("Retrieved {} downloaded files", files.len());
    Ok(files)
}

/// Check if a change is downloaded
#[tauri::command]
pub async fn gerrit_is_change_downloaded(
    state: State<'_, AppState>,
    change_id: String,
    patch_set_number: u32,
) -> Result<bool, String> {
    let is_downloaded = {
        let database = state.database.lock().unwrap();
        database.is_change_downloaded_by_gerrit_id(&change_id, patch_set_number)
            .map_err(|e| format!("Failed to check download status: {}", e))?
    };
    
    info!("Change {} PS{} downloaded: {}", change_id, patch_set_number, is_downloaded);
    Ok(is_downloaded)
}

/// Delete downloaded change data
#[tauri::command]
pub async fn gerrit_delete_downloaded_change(
    state: State<'_, AppState>,
    change_id: String,
) -> Result<bool, String> {
    info!("Delete downloaded change: {}", change_id);

    let deleted = {
        let database = state.database.lock().unwrap();
        database.delete_gerrit_change(&change_id)
            .map_err(|e| format!("Failed to delete change: {}", e))?
    };
    
    if deleted {
        info!("Successfully deleted change: {}", change_id);
    } else {
        info!("Change not found for deletion: {}", change_id);
    }
    Ok(deleted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_download_status_serialization() {
        let status = DownloadStatus {
            is_downloaded: true,
            file_count: 5,
            total_size: 1024,
            downloaded_at: "2025-01-05 12:00:00".to_string(),
            needs_update: false,
        };

        let json = serde_json::to_string(&status).unwrap();
        let deserialized: DownloadStatus = serde_json::from_str(&json).unwrap();
        
        assert_eq!(status.is_downloaded, deserialized.is_downloaded);
        assert_eq!(status.file_count, deserialized.file_count);
    }

    #[tokio::test]
    async fn test_update_result_creation() {
        let result = UpdateResult {
            updated: true,
            old_patch_set: 1,
            new_patch_set: 2,
            files_changed: 3,
            message: "Updated successfully".to_string(),
        };

        assert!(result.updated);
        assert_eq!(result.old_patch_set, 1);
        assert_eq!(result.new_patch_set, 2);
        assert_eq!(result.files_changed, 3);
    }
}