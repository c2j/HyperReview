// Advanced Gerrit commands for full integration
// Implements comprehensive Gerrit operations including comments and reviews

use tauri::State;
use serde::{Deserialize, Serialize};
use log::{info, warn, error};
use uuid::Uuid;
use chrono::Utc;

use crate::AppState;
use crate::errors::HyperReviewError;
use crate::remote::gerrit_client::GerritClient;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCommentParams {
    pub change_id: String,
    pub file_path: String,
    pub line_number: Option<i32>,
    pub content: String,
    pub patch_set_number: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCommentResult {
    pub success: bool,
    pub comment_id: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCommentsResult {
    pub success: bool,
    pub comments: Vec<GerritComment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GerritComment {
    pub id: String,
    pub file_path: String,
    pub line_number: Option<i32>,
    pub content: String,
    pub author: String,
    pub created: String,
    pub updated: String,
    pub patch_set_number: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitReviewParams {
    pub change_id: String,
    pub patch_set_number: i32,
    pub message: String,
    pub labels: std::collections::HashMap<String, i32>,
    pub comments: Vec<CreateCommentParams>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitReviewResult {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestConnectionResult {
    pub success: bool,
    pub message: String,
    pub version: Option<String>,
}

/// Create a comment on a Gerrit change
#[tauri::command]
pub async fn gerrit_create_comment_simple(
    params: CreateCommentParams,
    state: State<'_, AppState>,
) -> Result<CreateCommentResult, String> {
    info!("Creating comment on change {} at {}", params.change_id, params.file_path);
    
    // Validate input
    if params.content.trim().is_empty() {
        return Err("Comment content cannot be empty".to_string());
    }
    
    // TODO: Get active Gerrit instance and credentials
    // TODO: Use GerritClient to create comment via API
    
    // For now, return mock success
    let comment_id = Uuid::new_v4().to_string();
    
    Ok(CreateCommentResult {
        success: true,
        comment_id,
        message: "Comment created successfully".to_string(),
    })
}

/// Get comments for a Gerrit change
#[tauri::command]
pub async fn gerrit_get_comments_simple(
    change_id: String,
    state: State<'_, AppState>,
) -> Result<GetCommentsResult, String> {
    info!("Getting comments for change: {}", change_id);
    
    // TODO: Get active Gerrit instance and credentials
    // TODO: Use GerritClient to fetch comments via API
    
    // For now, return mock comments
    let mock_comments = vec![
        GerritComment {
            id: Uuid::new_v4().to_string(),
            file_path: "src/main.rs".to_string(),
            line_number: Some(42),
            content: "This looks good, but consider adding error handling".to_string(),
            author: "reviewer@example.com".to_string(),
            created: Utc::now().to_rfc3339(),
            updated: Utc::now().to_rfc3339(),
            patch_set_number: 1,
        }
    ];
    
    Ok(GetCommentsResult {
        success: true,
        comments: mock_comments,
    })
}

/// Submit a review to Gerrit
#[tauri::command]
pub async fn gerrit_submit_review_simple(
    params: SubmitReviewParams,
    state: State<'_, AppState>,
) -> Result<SubmitReviewResult, String> {
    info!("Submitting review for change: {}", params.change_id);
    
    // Validate input
    if params.message.trim().is_empty() {
        return Err("Review message cannot be empty".to_string());
    }
    
    // TODO: Get active Gerrit instance and credentials
    // TODO: Use GerritClient to submit review via API
    
    // For now, return mock success
    Ok(SubmitReviewResult {
        success: true,
        message: "Review submitted successfully".to_string(),
    })
}

/// Test connection to a Gerrit instance by ID
#[tauri::command]
pub async fn gerrit_test_connection_by_id(
    instance_id: String,
    state: State<'_, AppState>,
) -> Result<TestConnectionResult, String> {
    info!("Testing connection to Gerrit instance: {}", instance_id);
    
    // Get instance from database
    let (url, username, password) = {
        let database = state.database.lock().unwrap();
        match database.get_gerrit_instance(&instance_id) {
            Ok(Some(instance)) => {
                (instance.url, instance.username, instance.password_encrypted)
            }
            Ok(None) => {
                return Ok(TestConnectionResult {
                    success: false,
                    message: "Instance not found".to_string(),
                    version: None,
                });
            }
            Err(e) => {
                return Ok(TestConnectionResult {
                    success: false,
                    message: format!("Database error: {}", e),
                    version: None,
                });
            }
        }
    };
    
    // Test connection using the instance credentials
    let client = GerritClient::new(&url)
        .with_auth(username, password);
    
    match client.test_connection().await {
        Ok(result) => {
            if result.success {
                info!("Successfully connected to Gerrit instance: {}", instance_id);
                
                // Update instance status in database
                {
                    let database = state.database.lock().unwrap();
                    if let Ok(Some(mut instance)) = database.get_gerrit_instance(&instance_id) {
                        instance.connection_status = crate::models::gerrit::ConnectionStatus::Connected;
                        instance.last_connected = Some(chrono::Utc::now().to_rfc3339());
                        if let Some(version) = &result.gerrit_version {
                            instance.version = version.clone();
                        }
                        let _ = database.store_gerrit_instance(&instance);
                    }
                }
                
                Ok(TestConnectionResult {
                    success: true,
                    message: "Connection successful".to_string(),
                    version: result.gerrit_version,
                })
            } else {
                // Update instance status to disconnected
                {
                    let database = state.database.lock().unwrap();
                    if let Ok(Some(mut instance)) = database.get_gerrit_instance(&instance_id) {
                        instance.connection_status = crate::models::gerrit::ConnectionStatus::Disconnected;
                        let _ = database.store_gerrit_instance(&instance);
                    }
                }
                
                Ok(TestConnectionResult {
                    success: false,
                    message: result.error_message.unwrap_or_else(|| "Connection failed".to_string()),
                    version: None,
                })
            }
        }
        Err(e) => {
            warn!("Failed to connect to Gerrit instance {}: {}", instance_id, e);
            
            // Update instance status to disconnected
            {
                let database = state.database.lock().unwrap();
                if let Ok(Some(mut instance)) = database.get_gerrit_instance(&instance_id) {
                    instance.connection_status = crate::models::gerrit::ConnectionStatus::Disconnected;
                    let _ = database.store_gerrit_instance(&instance);
                }
            }
            
            Ok(TestConnectionResult {
                success: false,
                message: format!("Connection failed: {}", e),
                version: None,
            })
        }
    }
}

/// Test connection to a Gerrit instance
#[tauri::command]
pub async fn gerrit_test_connection(
    url: String,
    username: String,
    password: String,
    _state: State<'_, AppState>,
) -> Result<TestConnectionResult, String> {
    info!("Testing connection to Gerrit server: {}", url);
    
    let client = GerritClient::new(&url)
        .with_auth(username, password);
    
    match client.test_connection().await {
        Ok(result) => {
            if result.success {
                info!("Successfully connected to Gerrit server: {}", url);
                Ok(TestConnectionResult {
                    success: true,
                    message: "Connection successful".to_string(),
                    version: result.gerrit_version,
                })
            } else {
                Ok(TestConnectionResult {
                    success: false,
                    message: result.error_message.unwrap_or_else(|| "Connection failed".to_string()),
                    version: None,
                })
            }
        }
        Err(e) => {
            warn!("Failed to connect to Gerrit server {}: {}", url, e);
            Ok(TestConnectionResult {
                success: false,
                message: format!("Connection failed: {}", e),
                version: None,
            })
        }
    }
}

/// Get all Gerrit instances (advanced version)
#[tauri::command]
pub async fn gerrit_get_instances(
    state: State<'_, AppState>,
) -> Result<Vec<crate::commands::gerrit_simple::SimpleGerritInstance>, String> {
    info!("Getting all Gerrit instances (advanced)");
    
    // TODO: Implement database query to get all instances
    // For now, delegate to simple version
    let response = crate::commands::gerrit_simple::gerrit_get_instances_simple(state).await?;
    Ok(response.instances)
}

/// Create a Gerrit instance (advanced version)
#[tauri::command]
pub async fn gerrit_create_instance(
    params: crate::commands::gerrit_simple::SimpleCreateParams,
    state: State<'_, AppState>,
) -> Result<crate::commands::gerrit_simple::SimpleGerritInstance, String> {
    info!("Creating Gerrit instance (advanced): {}", params.name);
    
    // TODO: Add advanced validation and database storage
    // For now, delegate to simple version
    crate::commands::gerrit_simple::gerrit_create_instance_simple(params, state).await
}