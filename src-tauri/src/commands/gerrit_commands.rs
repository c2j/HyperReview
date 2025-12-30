// Gerrit Integration Commands
// Tauri command handlers for Gerrit Code Review integration

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::State;
use log::{info, warn, error};

use crate::models::gerrit::*;
use crate::models::HyperReviewError;
use crate::remote::gerrit_client::GerritClient;
use crate::storage::sqlite::Database;
use crate::AppState;

// ============================================================================
// Instance Management Commands
// ============================================================================

/// List all configured Gerrit instances
#[tauri::command]
pub async fn gerrit_get_instances(
    state: State<'_, AppState>,
    params: GetInstancesParams,
) -> Result<GetInstancesResponse, String> {
    info!("Fetching Gerrit instances, include_inactive: {}", params.include_inactive);
    
    let db = state.database.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    match db.get_gerrit_instances(params.include_inactive) {
        Ok(instances) => {
            let total_count = instances.len() as u32;
            let active_count = instances.iter().filter(|i| i.connection_status == ConnectionStatus::Connected).count() as u32;
            
            Ok(GetInstancesResponse {
                success: true,
                instances,
                total_count,
                active_count,
            })
        }
        Err(e) => {
            error!("Failed to get Gerrit instances: {}", e);
            Err(format!("Failed to retrieve instances: {}", e))
        }
    }
}

/// Create a new Gerrit instance configuration
#[tauri::command]
pub async fn gerrit_create_instance(
    state: State<'_, AppState>,
    params: CreateInstanceParams,
) -> Result<CreateInstanceResponse, String> {
    info!("Creating Gerrit instance: {}", params.name);
    
    // Validate parameters
    validate_instance_params(&params)?;
    
    let db = state.database.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    // Check for duplicate name
    if db.get_gerrit_instance_by_name(&params.name).is_ok() {
        return Err("Instance with this name already exists".to_string());
    }
    
    // Encrypt password
    let credential_store = state.credential_store.lock().map_err(|e| format!("Credential store lock error: {}", e))?;
    let encrypted_password = credential_store.encrypt_password(&params.password)
        .map_err(|e| format!("Failed to encrypt password: {}", e))?;
    
    // Create instance
    let mut instance = GerritInstance::new(
        params.name,
        params.url,
        params.username,
        encrypted_password,
    );
    
    if let Some(interval) = params.polling_interval {
        instance.polling_interval = interval;
    }
    
    if let Some(max_changes) = params.max_changes {
        instance.max_changes = max_changes;
    }
    
    // Save to database
    match db.create_gerrit_instance(&instance) {
        Ok(_) => {
            info!("Gerrit instance created successfully: {}", instance.id);
            
            // Test connection if requested
            let test_result = gerrit_test_connection_internal(&state, &instance.id).await.ok();
            
            Ok(CreateInstanceResponse {
                success: true,
                instance,
                message: "Instance created successfully".to_string(),
                test_connection_result: test_result,
            })
        }
        Err(e) => {
            error!("Failed to create Gerrit instance: {}", e);
            Err(format!("Failed to create instance: {}", e))
        }
    }
}

/// Test connection to Gerrit instance
#[tauri::command]
pub async fn gerrit_test_connection(
    state: State<'_, AppState>,
    params: TestConnectionParams,
) -> Result<ConnectionTestResult, String> {
    info!("Testing connection to Gerrit instance: {}", params.instance_id);
    gerrit_test_connection_internal(&state, &params.instance_id).await
}

// Internal connection test function
async fn gerrit_test_connection_internal(
    state: &State<AppState>,
    instance_id: &str,
) -> Result<ConnectionTestResult, String> {
    let db = state.database.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    // Get instance
    let instance = db.get_gerrit_instance(instance_id)
        .map_err(|e| format!("Instance not found: {}", e))?;
    
    let credential_store = state.credential_store.lock().map_err(|e| format!("Credential store lock error: {}", e))?;
    
    // Decrypt password
    let password = credential_store.decrypt_password(&instance.password_encrypted)
        .map_err(|e| format!("Failed to decrypt password: {}", e))?;
    
    // Create client and test connection
    let client = GerritClient::new(&instance.url)
        .with_auth(instance.username.clone(), password);
    
    match client.test_connection().await {
        Ok(result) => {
            // Update instance status
            let mut updated_instance = instance.clone();
            if result.success {
                updated_instance.mark_connected(result.gerrit_version.clone());
            } else {
                updated_instance.mark_disconnected(ConnectionStatus::AuthenticationFailed);
            }
            
            // Save updated status
            if let Err(e) = db.update_gerrit_instance(&updated_instance) {
                warn!("Failed to update instance status: {}", e);
            }
            
            Ok(result)
        }
        Err(e) => {
            error!("Connection test failed: {}", e);
            
            // Update instance status
            let mut updated_instance = instance.clone();
            updated_instance.mark_disconnected(ConnectionStatus::NetworkError);
            
            if let Err(e) = db.update_gerrit_instance(&updated_instance) {
                warn!("Failed to update instance status: {}", e);
            }
            
            Err(format!("Connection test failed: {}", e))
        }
    }
}

// ============================================================================
// Change Management Commands
// ============================================================================

/// Import a specific Gerrit change
#[tauri::command]
pub async fn gerrit_get_change(
    state: State<'_, AppState>,
    params: GetChangeParams,
) -> Result<GetChangeResponse, String> {
    info!("Importing Gerrit change: {} from instance: {}", params.change_id, params.instance_id);
    
    let db = state.database.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    // Get instance
    let instance = db.get_gerrit_instance(&params.instance_id)
        .map_err(|e| format!("Instance not found: {}", e))?;
    
    // Check if change already exists
    if let Ok(existing) = db.get_gerrit_change_by_change_id(&params.change_id) {
        if !params.force_refresh {
            return Ok(GetChangeResponse {
                success: true,
                change: existing,
                sync_status: get_sync_status(&db, &existing.id)?,
                import_progress: ImportProgress {
                    stage: ImportStage::Complete,
                    current_item: 0,
                    total_items: 0,
                    estimated_time_remaining: 0,
                },
                message: "Change already imported".to_string(),
                warnings: vec!["Use force_refresh to re-import".to_string()],
            });
        }
    }
    
    // Get credentials
    let credential_store = state.credential_store.lock().map_err(|e| format!("Credential store lock error: {}", e))?;
    let password = credential_store.decrypt_password(&instance.password_encrypted)
        .map_err(|e| format!("Failed to decrypt password: {}", e))?;
    
    // Create client
    let client = GerritClient::new(&instance.url)
        .with_auth(instance.username.clone(), password);
    
    // Import change
    match import_gerrit_change(&client, &params, &instance, &db).await {
        Ok(change) => {
            info!("Successfully imported change: {}", change.id);
            
            Ok(GetChangeResponse {
                success: true,
                change: change.clone(),
                sync_status: get_sync_status(&db, &change.id)?,
                import_progress: ImportProgress {
                    stage: ImportStage::Complete,
                    current_item: change.total_files,
                    total_items: change.total_files,
                    estimated_time_remaining: 0,
                },
                message: "Change imported successfully".to_string(),
                warnings: vec![],
            })
        }
        Err(e) => {
            error!("Failed to import change: {}", e);
            Err(format!("Import failed: {}", e))
        }
    }
}

/// Search for Gerrit changes
#[tauri::command]
pub async fn gerrit_search_changes(
    state: State<'_, AppState>,
    params: SearchChangesParams,
) -> Result<SearchChangesResponse, String> {
    info!("Searching Gerrit changes with query: '{}' on instance: {}", params.query, params.instance_id);
    
    let db = state.database.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    // Get instance
    let instance = db.get_gerrit_instance(&params.instance_id)
        .map_err(|e| format!("Instance not found: {}", e))?;
    
    // Get credentials
    let credential_store = state.credential_store.lock().map_err(|e| format!("Credential store lock error: {}", e))?;
    let password = credential_store.decrypt_password(&instance.password_encrypted)
        .map_err(|e| format!("Failed to decrypt password: {}", e))?;
    
    // Create client
    let client = GerritClient::new(&instance.url)
        .with_auth(instance.username.clone(), password);
    
    // Perform search
    match client.search_changes(&params.query, params.max_results.unwrap_or(50)).await {
        Ok(search_results) => {
            let mut results = vec![];
            let mut imported_count = 0;
            let mut failed_count = 0;
            let mut import_errors = vec![];
            
            // Process search results based on import mode
            match params.import_mode {
                ImportMode::PreviewOnly => {
                    // Just return search results without importing
                    for result in search_results {
                        results.push(result);
                    }
                }
                ImportMode::ImportAll => {
                    // Import all found changes
                    for result in search_results {
                        let import_params = GetChangeParams {
                            instance_id: params.instance_id.clone(),
                            change_id: result.change_id.clone(),
                            include_comments: true,
                            include_files: true,
                            force_refresh: false,
                        };
                        
                        match import_gerrit_change(&client, &import_params, &instance, &db).await {
                            Ok(_) => {
                                imported_count += 1;
                                results.push(result);
                            }
                            Err(e) => {
                                failed_count += 1;
                                import_errors.push(ImportError {
                                    change_id: result.change_id.clone(),
                                    error_code: "IMPORT_FAILED".to_string(),
                                    message: e,
                                    timestamp: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                                });
                            }
                        }
                    }
                }
                ImportMode::ImportSelection => {
                    // For selection mode, just return preview results
                    // Actual import would be handled by separate calls
                    for result in search_results {
                        results.push(result);
                    }
                }
            }
            
            let query_id = uuid::Uuid::new_v4().to_string();
            
            // Save search query for tracking
            let search_query = SearchQuery {
                id: query_id.clone(),
                instance_id: params.instance_id,
                query: params.query,
                query_type: QueryType::Custom,
                results: results.clone(),
                result_count: search_results.len() as u32,
                status: QueryStatus::Completed,
                created: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                expires: (chrono::Utc::now() + chrono::Duration::minutes(5)).format("%Y-%m-%d %H:%M:%S").to_string(),
            };
            
            if let Err(e) = db.save_search_query(&search_query) {
                warn!("Failed to save search query: {}", e);
            }
            
            Ok(SearchChangesResponse {
                success: true,
                query_id,
                results,
                total_available: search_results.len() as u32,
                imported_count,
                failed_count,
                import_errors,
            })
        }
        Err(e) => {
            error!("Search failed: {}", e);
            Err(format!("Search failed: {}", e))
        }
    }
}

/// Get file diff content
#[tauri::command]
pub async fn gerrit_get_diff(
    state: State<'_, AppState>,
    params: GetDiffParams,
) -> Result<GetDiffResponse, String> {
    info!("Fetching diff for file: {} in change: {}", params.file_path, params.change_id);
    
    let start_time = std::time::Instant::now();
    
    let db = state.database.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    // Get change
    let change = db.get_gerrit_change(&params.change_id)
        .map_err(|e| format!("Change not found: {}", e))?;
    
    // Get instance
    let instance = db.get_gerrit_instance(&change.instance_id)
        .map_err(|e| format!("Instance not found: {}", e))?;
    
    // Check cache first
    if let Ok(cached_diff) = db.get_cached_diff(&params.change_id, &params.file_path) {
        let render_time = start_time.elapsed().as_millis() as u32;
        return Ok(GetDiffResponse {
            success: true,
            diff: cached_diff.clone(),
            content: load_diff_content(&cached_diff),
            render_time_ms: render_time,
            cache_hit: true,
        });
    }
    
    // Get credentials
    let credential_store = state.credential_store.lock().map_err(|e| format!("Credential store lock error: {}", e))?;
    let password = credential_store.decrypt_password(&instance.password_encrypted)
        .map_err(|e| format!("Failed to decrypt password: {}", e))?;
    
    // Create client
    let client = GerritClient::new(&instance.url)
        .with_auth(instance.username.clone(), password);
    
    // Get diff from Gerrit
    let patch_set_id = params.patch_set_id.as_ref().unwrap_or(&change.current_revision);
    
    match client.get_diff(
        &change.change_id,
        patch_set_id,
        &params.file_path,
        params.context_lines.unwrap_or(3),
        params.ignore_whitespace,
    ).await {
        Ok(diff_content) => {
            // Find the file in the change
            let file = change.files.iter()
                .find(|f| f.file_path == params.file_path)
                .ok_or_else(|| "File not found in change".to_string())?;
            
            let mut updated_file = file.clone();
            updated_file.diff_content = Some(diff_content.clone());
            
            // Cache the diff
            if let Err(e) = db.cache_diff(&params.change_id, &params.file_path, &updated_file) {
                warn!("Failed to cache diff: {}", e);
            }
            
            let render_time = start_time.elapsed().as_millis() as u32;
            
            Ok(GetDiffResponse {
                success: true,
                diff: updated_file,
                content: parse_diff_content(&diff_content),
                render_time_ms: render_time,
                cache_hit: false,
            })
        }
        Err(e) => {
            error!("Failed to fetch diff: {}", e);
            Err(format!("Failed to fetch diff: {}", e))
        }
    }
}

// ============================================================================
// Review Submission Commands
// ============================================================================

/// Submit comments to Gerrit
#[tauri::command]
pub async fn gerrit_submit_comments(
    state: State<'_, AppState>,
    params: SubmitCommentsParams,
) -> Result<SubmitCommentsResponse, String> {
    info!("Submitting {} comments for change: {}", params.comment_ids.len(), params.change_id);
    
    let db = state.database.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    // Get change
    let change = db.get_gerrit_change(&params.change_id)
        .map_err(|e| format!("Change not found: {}", e))?;
    
    // Get instance
    let instance = db.get_gerrit_instance(&change.instance_id)
        .map_err(|e| format!("Instance not found: {}", e))?;
    
    // Get credentials
    let credential_store = state.credential_store.lock().map_err(|e| format!("Credential store lock error: {}", e))?;
    let password = credential_store.decrypt_password(&instance.password_encrypted)
        .map_err(|e| format!("Failed to decrypt password: {}", e))?;
    
    // Create client
    let client = GerritClient::new(&instance.url)
        .with_auth(instance.username.clone(), password);
    
    // Collect comments to submit
    let mut comments_to_submit = vec![];
    let mut local_comment_ids = vec![];
    
    for comment_id in &params.comment_ids {
        match db.get_gerrit_comment(comment_id) {
            Ok(comment) => {
                if comment.is_local() {
                    comments_to_submit.push(comment.clone());
                    local_comment_ids.push(comment_id.clone());
                }
            }
            Err(e) => {
                warn!("Comment not found: {} - {}", comment_id, e);
            }
        }
    }
    
    if comments_to_submit.is_empty() {
        return Ok(SubmitCommentsResponse {
            success: true,
            submitted_count: 0,
            failed_count: 0,
            results: vec![],
            conflicts: vec![],
            retry_suggested: false,
        });
    }
    
    // Submit comments to Gerrit
    match client.submit_comments(
        &change.change_id,
        &change.current_revision,
        comments_to_submit,
    ).await {
        Ok(submit_results) => {
            let mut submitted_count = 0;
            let mut failed_count = 0;
            let mut results = vec![];
            let mut conflicts = vec![];
            
            // Process results
            for (i, result) in submit_results.iter().enumerate() {
                let local_comment_id = &local_comment_ids[i];
                
                if result.success {
                    submitted_count += 1;
                    
                    // Update comment status
                    if let Ok(mut comment) = db.get_gerrit_comment(local_comment_id) {
                        comment.status = CommentSyncStatus::Synced;
                        comment.gerrit_comment_id = Some(result.gerrit_comment_id.clone());
                        
                        if let Err(e) = db.update_gerrit_comment(&comment) {
                            warn!("Failed to update comment status: {}", e);
                        }
                    }
                } else {
                    failed_count += 1;
                    
                    if result.is_conflict {
                        conflicts.push(CommentConflict {
                            comment_id: local_comment_id.clone(),
                            conflict_type: ConflictType::ConcurrentEdit,
                            remote_comment: result.remote_comment.clone(),
                            resolution_options: vec![],
                        });
                    }
                }
                
                results.push(CommentSubmitResult {
                    comment_id: local_comment_id.clone(),
                    success: result.success,
                    gerrit_comment_id: result.gerrit_comment_id.clone(),
                    error: result.error.clone(),
                });
            }
            
            info!("Comment submission completed: {} submitted, {} failed", submitted_count, failed_count);
            
            Ok(SubmitCommentsResponse {
                success: failed_count == 0,
                submitted_count,
                failed_count,
                results,
                conflicts,
                retry_suggested: failed_count > 0 && conflicts.is_empty(),
            })
        }
        Err(e) => {
            error!("Failed to submit comments: {}", e);
            Err(format!("Submission failed: {}", e))
        }
    }
}

/// Submit a complete review with labels and message
#[tauri::command]
pub async fn gerrit_submit_review(
    state: State<'_, AppState>,
    params: SubmitReviewParams,
) -> Result<SubmitReviewResponse, String> {
    info!("Submitting review for change: {} with {} comments", params.change_id, params.comment_ids.len());
    
    let db = state.database.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    // Get change
    let change = db.get_gerrit_change(&params.change_id)
        .map_err(|e| format!("Change not found: {}", e))?;
    
    // Get instance
    let instance = db.get_gerrit_instance(&change.instance_id)
        .map_err(|e| format!("Instance not found: {}", e))?;
    
    // Get credentials
    let credential_store = state.credential_store.lock().map_err(|e| format!("Credential store lock error: {}", e))?;
    let password = credential_store.decrypt_password(&instance.password_encrypted)
        .map_err(|e| format!("Failed to decrypt password: {}", e))?;
    
    // Create client
    let client = GerritClient::new(&instance.url)
        .with_auth(instance.username.clone(), password);
    
    // Create review
    let review = GerritReview {
        id: uuid::Uuid::new_v4().to_string(),
        gerrit_review_id: None,
        change_id: params.change_id.clone(),
        patch_set_id: params.patch_set_id.clone(),
        message: params.message.clone(),
        labels: params.labels.clone(),
        comments: params.comment_ids.clone(),
        author: get_current_user(), // This would come from session/auth
        created: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        submitted: None,
        status: ReviewStatus::PendingSubmission,
        draft: params.draft,
        notify: params.notify.clone(),
    };
    
    // Submit review to Gerrit
    match client.submit_review(
        &change.change_id,
        &params.patch_set_id,
        &review,
    ).await {
        Ok(submit_result) => {
            let mut updated_review = review.clone();
            updated_review.gerrit_review_id = Some(submit_result.review_id.clone());
            updated_review.submitted = Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
            updated_review.status = ReviewStatus::Submitted;
            
            // Save review to database
            if let Err(e) = db.create_gerrit_review(&updated_review) {
                warn!("Failed to save review: {}", e);
            }
            
            // Update comment statuses
            for comment_id in &params.comment_ids {
                if let Ok(mut comment) = db.get_gerrit_comment(comment_id) {
                    comment.status = CommentSyncStatus::Synced;
                    if let Err(e) = db.update_gerrit_comment(&comment) {
                        warn!("Failed to update comment status: {}", e);
                    }
                }
            }
            
            info!("Review submitted successfully: {}", submit_result.review_id);
            
            // Convert label updates
            let label_updates: Vec<LabelUpdate> = submit_result.labels.iter()
                .map(|(label, values)| LabelUpdate {
                    label: label.clone(),
                    old_value: values.old_value,
                    new_value: values.new_value,
                    applied: true,
                })
                .collect();
            
            Ok(SubmitReviewResponse {
                success: true,
                review_id: submit_result.review_id,
                submitted_at: updated_review.submitted.unwrap(),
                message: "Review submitted successfully".to_string(),
                label_updates,
            })
        }
        Err(e) => {
            error!("Failed to submit review: {}", e);
            Err(format!("Review submission failed: {}", e))
        }
    }
}

// ============================================================================
// Synchronization Commands
// ============================================================================

/// Synchronize changes with Gerrit
#[tauri::command]
pub async fn gerrit_sync_changes(
    state: State<'_, AppState>,
    params: SyncChangesParams,
) -> Result<SyncChangesResponse, String> {
    info!("Syncing changes for instance: {} with {} specific changes", params.instance_id, params.change_ids.len());
    
    let start_time = std::time::Instant::now();
    let sync_id = uuid::Uuid::new_v4().to_string();
    
    let db = state.database.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    // Get instance
    let instance = db.get_gerrit_instance(&params.instance_id)
        .map_err(|e| format!("Instance not found: {}", e))?;
    
    // Get credentials
    let credential_store = state.credential_store.lock().map_err(|e| format!("Credential store lock error: {}", e))?;
    let password = credential_store.decrypt_password(&instance.password_encrypted)
        .map_err(|e| format!("Failed to decrypt password: {}", e))?;
    
    // Create client
    let client = GerritClient::new(&instance.url)
        .with_auth(instance.username.clone(), password);
    
    // Get changes to sync
    let changes_to_sync = if params.change_ids.is_empty() {
        // Sync all imported changes
        db.get_gerrit_changes_by_instance(&params.instance_id)
            .map_err(|e| format!("Failed to get changes: {}", e))?
    } else {
        // Sync specific changes
        let mut changes = vec![];
        for change_id in &params.change_ids {
            if let Ok(change) = db.get_gerrit_change(change_id) {
                changes.push(change);
            }
        }
        changes
    };
    
    let total_changes = changes_to_sync.len() as u32;
    let mut changes_processed = 0;
    let mut changes_updated = 0;
    let mut conflicts_detected = 0;
    
    // Process each change
    for change in changes_to_sync {
        changes_processed += 1;
        
        match sync_gerrit_change(&client, &change, &params.sync_type, &db).await {
            Ok(sync_result) => {
                if sync_result.updated {
                    changes_updated += 1;
                }
                if sync_result.conflicts_detected > 0 {
                    conflicts_detected += sync_result.conflicts_detected;
                }
            }
            Err(e) => {
                warn!("Failed to sync change {}: {}", change.id, e);
            }
        }
    }
    
    let sync_duration = start_time.elapsed().as_millis() as u32;
    let next_sync_at = (chrono::Utc::now() + chrono::Duration::seconds(instance.polling_interval as i64))
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    
    info!("Sync completed: {} processed, {} updated, {} conflicts in {}ms", 
          changes_processed, changes_updated, conflicts_detected, sync_duration);
    
    Ok(SyncChangesResponse {
        success: true,
        sync_id,
        changes_processed,
        changes_updated,
        conflicts_detected,
        sync_duration_ms: sync_duration,
        next_sync_at,
    })
}

/// Get synchronization status
#[tauri::command]
pub async fn gerrit_get_sync_status(
    state: State<'_, AppState>,
    params: GetSyncStatusParams,
) -> Result<GetSyncStatusResponse, String> {
    info!("Getting sync status for instance: {:?}, change: {:?}", params.instance_id, params.change_id);
    
    let db = state.database.lock().map_err(|e| format!("Database lock error: {}", e))?;
    
    let sync_operations = match params.instance_id {
        Some(instance_id) => {
            db.get_sync_operations_by_instance(&instance_id)
                .map_err(|e| format!("Failed to get sync operations: {}", e))?
        }
        None => {
            db.get_all_sync_operations()
                .map_err(|e| format!("Failed to get sync operations: {}", e))?
        }
    };
    
    let pending_operations = match params.change_id {
        Some(change_id) => {
            db.get_pending_operations_by_change(&change_id)
                .map_err(|e| format!("Failed to get pending operations: {}", e))?
        }
        None => {
            db.get_all_pending_operations()
                .map_err(|e| format!("Failed to get pending operations: {}", e))?
        }
    };
    
    let conflict_summary = calculate_conflict_summary(&sync_operations, &pending_operations);
    
    let last_sync_at = sync_operations.iter()
        .filter(|op| op.status == SyncOperationStatus::Completed)
        .map(|op| op.last_sync.clone())
        .max();
    
    Ok(GetSyncStatusResponse {
        success: true,
        sync_operations,
        pending_operations,
        conflict_summary,
        last_sync_at,
    })
}

// ============================================================================
// Helper Functions
// ============================================================================

fn validate_instance_params(params: &CreateInstanceParams) -> Result<(), String> {
    // Validate name
    if params.name.len() < 3 || params.name.len() > 50 {
        return Err("Instance name must be 3-50 characters".to_string());
    }
    
    if !params.name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == ' ') {
        return Err("Instance name can only contain alphanumeric characters, hyphens, and spaces".to_string());
    }
    
    // Validate URL
    if !params.url.starts_with("https://") {
        return Err("URL must use HTTPS protocol".to_string());
    }
    
    // Basic URL validation
    if url::Url::parse(&params.url).is_err() {
        return Err("Invalid URL format".to_string());
    }
    
    // Validate username
    if params.username.is_empty() || params.username.len() > 100 {
        return Err("Username must be 1-100 characters".to_string());
    }
    
    if params.username.contains(' ') {
        return Err("Username cannot contain spaces".to_string());
    }
    
    // Validate password
    if params.password.is_empty() || params.password.len() > 500 {
        return Err("Password must be 1-500 characters".to_string());
    }
    
    Ok(())
}

fn get_sync_status(db: &Database, change_id: &str) -> Result<SyncStatus, String> {
    // Get latest sync operation for the change
    match db.get_latest_sync_status(change_id) {
        Ok(Some(sync)) => Ok(sync),
        Ok(None) => {
            // Create default sync status
            Ok(SyncStatus {
                id: uuid::Uuid::new_v4().to_string(),
                instance_id: "".to_string(), // Will be populated by caller
                change_id: Some(change_id.to_string()),
                last_sync: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                next_sync: None,
                sync_type: SyncType::Incremental,
                status: SyncOperationStatus::Pending,
                items_processed: 0,
                items_total: 0,
                conflicts_detected: 0,
                errors: vec![],
                metadata: HashMap::new(),
            })
        }
        Err(e) => Err(format!("Failed to get sync status: {}", e))
    }
}

fn get_current_user() -> GerritUser {
    // This would come from the current session/authentication
    // For now, return a default user
    GerritUser {
        account_id: 0,
        name: "Current User".to_string(),
        email: "user@example.com".to_string(),
        username: Some("currentuser".to_string()),
        avatar_url: None,
    }
}

fn calculate_conflict_summary(
    sync_operations: &[SyncStatus],
    pending_operations: &[OperationQueue],
) -> ConflictSummary {
    let total_conflicts = sync_operations.iter()
        .map(|op| op.conflicts_detected)
        .sum::<u32>();
    
    let comment_conflicts = sync_operations.iter()
        .filter(|op| op.sync_type == SyncType::CommentsOnly)
        .map(|op| op.conflicts_detected)
        .sum::<u32>();
    
    let pending_comment_ops = pending_operations.iter()
        .filter(|op| matches!(op.operation_type, OperationType::AddComment | OperationType::UpdateComment))
        .count() as u32;
    
    ConflictSummary {
        total_conflicts,
        comment_conflicts,
        status_conflicts: 0, // Would be calculated based on status conflicts
        file_conflicts: 0,   // Would be calculated based on file conflicts
        auto_resolvable: total_conflicts / 2, // Simplified heuristic
        manual_resolution_required: total_conflicts - (total_conflicts / 2),
    }
}

// Placeholder functions - these would be implemented in the actual GerritClient
async fn import_gerrit_change(
    client: &GerritClient,
    params: &GetChangeParams,
    instance: &GerritInstance,
    db: &Database,
) -> Result<GerritChange, String> {
    // This would be implemented in the actual GerritClient
    // For now, return a mock implementation
    todo!("Implement actual Gerrit change import")
}

async fn sync_gerrit_change(
    client: &GerritClient,
    change: &GerritChange,
    sync_type: &SyncType,
    db: &Database,
) -> Result<SyncResult, String> {
    // This would be implemented in the actual GerritClient
    // For now, return a mock implementation
    todo!("Implement actual Gerrit change sync")
}

fn load_diff_content(file: &GerritFile) -> DiffContent {
    // This would parse the diff content from the file
    // For now, return empty content
    DiffContent {
        lines: vec![],
        statistics: DiffStatistics {
            total_lines: 0,
            added_lines: 0,
            removed_lines: 0,
            modified_lines: 0,
            context_lines: 0,
        },
        metadata: HashMap::new(),
    }
}

fn parse_diff_content(content: &str) -> DiffContent {
    // This would parse the diff content string
    // For now, return empty content
    DiffContent {
        lines: vec![],
        statistics: DiffStatistics {
            total_lines: 0,
            added_lines: 0,
            removed_lines: 0,
            modified_lines: 0,
            context_lines: 0,
        },
        metadata: HashMap::new(),
    }
}

struct SyncResult {
    updated: bool,
    conflicts_detected: u32,
}

// ============================================================================
// Additional Request/Response Types
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct GetInstancesParams {
    pub include_inactive: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetInstancesResponse {
    pub success: bool,
    pub instances: Vec<GerritInstance>,
    pub total_count: u32,
    pub active_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInstanceParams {
    pub name: String,
    pub url: String,
    pub username: String,
    pub password: String,
    pub polling_interval: Option<u32>,
    pub max_changes: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInstanceResponse {
    pub success: bool,
    pub instance: GerritInstance,
    pub message: String,
    pub test_connection_result: Option<ConnectionTestResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestConnectionParams {
    pub instance_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionTestResult {
    pub success: bool,
    pub status: ConnectionStatus,
    pub gerrit_version: String,
    pub server_time: String,
    pub user_info: Option<GerritUser>,
    pub supported_features: Vec<String>,
    pub error_details: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetChangeParams {
    pub instance_id: String,
    pub change_id: String,
    pub include_comments: bool,
    pub include_files: bool,
    pub force_refresh: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetChangeResponse {
    pub success: bool,
    pub change: GerritChange,
    pub sync_status: SyncStatus,
    pub import_progress: ImportProgress,
    pub message: String,
    pub warnings: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportProgress {
    pub stage: ImportStage,
    pub current_item: u32,
    pub total_items: u32,
    pub estimated_time_remaining: u32, // seconds
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchChangesParams {
    pub instance_id: String,
    pub query: String,
    pub max_results: Option<u32>,
    pub import_mode: ImportMode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchChangesResponse {
    pub success: bool,
    pub query_id: String,
    pub results: Vec<SearchResult>,
    pub total_available: u32,
    pub imported_count: u32,
    pub failed_count: u32,
    pub import_errors: Vec<ImportError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportError {
    pub change_id: String,
    pub error_code: String,
    pub message: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDiffParams {
    pub change_id: String,
    pub file_path: String,
    pub patch_set_id: Option<String>,
    pub context_lines: Option<u32>,
    pub ignore_whitespace: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDiffResponse {
    pub success: bool,
    pub diff: GerritFile,
    pub content: DiffContent,
    pub render_time_ms: u32,
    pub cache_hit: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffContent {
    pub lines: Vec<DiffLine>,
    pub statistics: DiffStatistics,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffLine {
    pub line_number: u32,
    pub old_line_number: Option<u32>,
    pub new_line_number: Option<u32>,
    pub content: String,
    pub type_: DiffLineType,
    pub has_comments: bool,
    pub comment_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DiffLineType {
    Context,
    Added,
    Removed,
    Modified,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffStatistics {
    pub total_lines: u32,
    pub added_lines: u32,
    pub removed_lines: u32,
    pub modified_lines: u32,
    pub context_lines: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitCommentsParams {
    pub change_id: String,
    pub comment_ids: Vec<String>,
    pub batch_mode: BatchMode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitCommentsResponse {
    pub success: bool,
    pub submitted_count: u32,
    pub failed_count: u32,
    pub results: Vec<CommentSubmitResult>,
    pub conflicts: Vec<CommentConflict>,
    pub retry_suggested: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentSubmitResult {
    pub comment_id: String,
    pub success: bool,
    pub gerrit_comment_id: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentConflict {
    pub comment_id: String,
    pub conflict_type: ConflictType,
    pub remote_comment: Option<GerritComment>,
    pub resolution_options: Vec<ConflictResolutionOption>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConflictResolutionOption {
    pub id: String,
    pub description: String,
    pub action: String,
    pub auto_resolvable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitReviewParams {
    pub change_id: String,
    pub patch_set_id: String,
    pub message: String,
    pub labels: HashMap<String, i32>,
    pub comment_ids: Vec<String>,
    pub draft: bool,
    pub notify: NotifyHandling,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitReviewResponse {
    pub success: bool,
    pub review_id: String,
    pub submitted_at: String,
    pub message: String,
    pub label_updates: Vec<LabelUpdate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LabelUpdate {
    pub label: String,
    pub old_value: i32,
    pub new_value: i32,
    pub applied: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncChangesParams {
    pub instance_id: String,
    pub change_ids: Vec<String>,
    pub sync_type: SyncType,
    pub force_full_sync: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncChangesResponse {
    pub success: bool,
    pub sync_id: String,
    pub changes_processed: u32,
    pub changes_updated: u32,
    pub conflicts_detected: u32,
    pub sync_duration_ms: u32,
    pub next_sync_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSyncStatusParams {
    pub instance_id: Option<String>,
    pub change_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSyncStatusResponse {
    pub success: bool,
    pub sync_operations: Vec<SyncStatus>,
    pub pending_operations: Vec<OperationQueue>,
    pub conflict_summary: ConflictSummary,
    pub last_sync_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConflictSummary {
    pub total_conflicts: u32,
    pub comment_conflicts: u32,
    pub status_conflicts: u32,
    pub file_conflicts: u32,
    pub auto_resolvable: u32,
    pub manual_resolution_required: u32,
}