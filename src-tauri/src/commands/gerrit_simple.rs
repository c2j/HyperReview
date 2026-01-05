// Simple Gerrit commands for basic operations
// Implements the core Gerrit integration commands called by the frontend

use tauri::State;
use serde::{Deserialize, Serialize};
use log::{info, warn, error};
use uuid::Uuid;
use chrono::Utc;

use crate::AppState;
use crate::errors::HyperReviewError;
use crate::models::gerrit::{GerritInstance, ConnectionStatus};
use crate::remote::gerrit_client::GerritClient;

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleGerritInstance {
    pub id: String,
    pub name: String,
    pub url: String,
    pub username: String,
    pub is_active: bool,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleCreateParams {
    pub name: String,
    pub url: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleChange {
    pub id: String,
    pub change_number: i32,
    pub subject: String,
    pub status: String,
    pub project: String,
    pub branch: String,
    pub topic: Option<String>,
    pub owner: String,
    pub updated: String,
    pub created: String,
    pub insertions: i32,
    pub deletions: i32,
    pub files: Vec<SimpleFileInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleFileInfo {
    pub path: String,
    pub change_type: String,
    pub insertions: i32,
    pub deletions: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetInstancesResponse {
    pub success: bool,
    pub instances: Vec<SimpleGerritInstance>,
}

/// Get all configured Gerrit instances
#[tauri::command]
pub async fn gerrit_get_instances_simple(
    state: State<'_, AppState>,
) -> Result<GetInstancesResponse, String> {
    info!("Getting Gerrit instances from database");
    
    let database = state.database.lock().unwrap();
    
    // Initialize Gerrit schema if needed
    if let Err(e) = database.init_gerrit_schema() {
        warn!("Failed to initialize Gerrit schema: {}", e);
    }
    
    // Try to get instances from database
    match database.get_all_gerrit_instances() {
        Ok(instances) => {
            let simple_instances: Vec<SimpleGerritInstance> = instances.into_iter().map(|instance| {
                SimpleGerritInstance {
                    id: instance.id,
                    name: instance.name,
                    url: instance.url,
                    username: instance.username,
                    is_active: instance.is_active,
                    status: instance.connection_status.to_string(),
                }
            }).collect();
            
            info!("Retrieved {} Gerrit instances from database", simple_instances.len());
            Ok(GetInstancesResponse {
                success: true,
                instances: simple_instances,
            })
        }
        Err(e) => {
            warn!("Failed to get instances from database: {}", e);
            
            // Return empty list instead of mock data
            // This ensures only real database instances are shown
            Ok(GetInstancesResponse {
                success: false, // Indicate there was an error
                instances: Vec::new(), // Return empty list
            })
        }
    }
}

/// Create a new Gerrit instance
#[tauri::command]
pub async fn gerrit_create_instance_simple(
    params: SimpleCreateParams,
    state: State<'_, AppState>,
) -> Result<SimpleGerritInstance, String> {
    info!("Creating Gerrit instance: {}", params.name);
    
    // Validate input
    if params.name.trim().is_empty() {
        return Err("Instance name cannot be empty".to_string());
    }
    if params.url.trim().is_empty() {
        return Err("Instance URL cannot be empty".to_string());
    }
    if params.username.trim().is_empty() {
        return Err("Username cannot be empty".to_string());
    }
    
    // Test connection to the Gerrit server
    let client = GerritClient::new(&params.url)
        .with_auth(params.username.clone(), params.password.clone());
    
    let connection_status = match client.test_connection().await {
        Ok(_) => {
            info!("Successfully connected to Gerrit server: {}", params.url);
            ConnectionStatus::Connected
        }
        Err(e) => {
            warn!("Failed to connect to Gerrit server {}: {}", params.url, e);
            ConnectionStatus::Disconnected
        }
    };
    
    // Store credentials securely
    let mut credential_store = state.credential_store.lock().unwrap();
    if let Err(e) = credential_store.store(
        "gerrit",
        &params.username,
        &params.password,
    ) {
        error!("Failed to store credentials: {}", e);
    }
    
    // Create the Gerrit instance
    let instance_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    let gerrit_instance = GerritInstance {
        id: instance_id.clone(),
        name: params.name.clone(),
        url: params.url.clone(),
        username: params.username.clone(),
        password_encrypted: params.password, // TODO: Encrypt this properly
        version: "".to_string(),
        is_active: true,
        last_connected: if connection_status == ConnectionStatus::Connected {
            Some(now.clone())
        } else {
            None
        },
        connection_status: connection_status.clone(),
        polling_interval: 300,
        max_changes: 100,
        created_at: now.clone(),
        updated_at: now,
    };
    
    // Store in database
    let database = state.database.lock().unwrap();
    
    // Initialize Gerrit schema if needed
    if let Err(e) = database.init_gerrit_schema() {
        warn!("Failed to initialize Gerrit schema: {}", e);
    }
    
    match database.store_gerrit_instance(&gerrit_instance) {
        Ok(_) => {
            info!("Successfully stored Gerrit instance in database: {}", instance_id);
            
            let simple_instance = SimpleGerritInstance {
                id: instance_id,
                name: params.name,
                url: params.url,
                username: params.username,
                is_active: true,
                status: connection_status.to_string(),
            };
            
            Ok(simple_instance)
        }
        Err(e) => {
            error!("Failed to store Gerrit instance in database: {}", e);
            Err(format!("Failed to store instance: {}", e))
        }
    }
}

/// Delete a Gerrit instance
#[tauri::command]
pub async fn gerrit_delete_instance_simple(
    instance_id: String,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    info!("Deleting Gerrit instance: {}", instance_id);
    
    let database = state.database.lock().unwrap();
    
    // Special handling for mock data IDs that might not be in database
    if instance_id == "test-instance-1" {
        warn!("Attempting to delete mock instance ID that doesn't exist in database");
        return Ok(false); // Return false to indicate it wasn't found/deleted
    }
    
    match database.delete_gerrit_instance(&instance_id) {
        Ok(deleted) => {
            if deleted {
                info!("Successfully deleted Gerrit instance: {}", instance_id);
                // TODO: Remove stored credentials
            } else {
                warn!("Gerrit instance not found: {}", instance_id);
            }
            Ok(deleted)
        }
        Err(e) => {
            error!("Failed to delete Gerrit instance: {}", e);
            Err(format!("Failed to delete instance: {}", e))
        }
    }
}

/// Import a change from Gerrit
#[tauri::command]
pub async fn gerrit_import_change_simple(
    change_id: String,
    state: State<'_, AppState>,
) -> Result<SimpleChange, String> {
    info!("Importing Gerrit change: {}", change_id);
    
    // Get active Gerrit instance (release lock before async operations)
    let (gerrit_url, username, password, instance_id) = {
        let database = state.database.lock().unwrap();
        
        // Initialize Gerrit schema if needed
        if let Err(e) = database.init_gerrit_schema() {
            warn!("Failed to initialize Gerrit schema: {}", e);
        }
        
        // Get active Gerrit instance
        let active_instance = match database.get_all_gerrit_instances() {
            Ok(instances) => instances.into_iter().find(|i| i.is_active),
            Err(e) => {
                warn!("Failed to get Gerrit instances: {}", e);
                None
            }
        };
        
        if let Some(instance) = active_instance {
            (instance.url.clone(), instance.username.clone(), instance.password_encrypted.clone(), instance.id.clone())
        } else {
            // Use test server as fallback
            (
                "http://edce7739774c:8080".to_string(),
                "admin".to_string(),
                "8EWK0RrulrdN8d7vTFVpbQTAjQFU2lFQpRhTZpISBw".to_string(),
                "test-instance-1".to_string(),
            )
        }
    }; // Database lock is released here
    
    // Try to connect to the Gerrit server
    let client = GerritClient::new(&gerrit_url)
        .with_auth(username, password);
    
    // Parse change ID (remove # if present)
    let clean_change_id = change_id.trim_start_matches('#');
    
    // Try to parse as number
    let change_number: i32 = clean_change_id.parse()
        .map_err(|_| format!("Invalid change ID format: {}", change_id))?;
    
    match client.get_change(change_number).await {
        Ok(gerrit_change) => {
            info!("Successfully fetched change from Gerrit: {}", gerrit_change.subject);
            
            // Convert to our internal GerritChange model
            let now = Utc::now().to_rfc3339();
            let internal_change = crate::models::gerrit::GerritChange {
                id: Uuid::new_v4().to_string(),
                change_id: gerrit_change.change_id.clone(),
                instance_id: instance_id.clone(),
                project: gerrit_change.project.clone(),
                branch: gerrit_change.branch.clone(),
                subject: gerrit_change.subject.clone(),
                status: crate::models::gerrit::ChangeStatus::from_string(&gerrit_change.status),
                owner: crate::models::gerrit::GerritUser {
                    account_id: 0,
                    name: "admin".to_string(), // TODO: Parse from owner object
                    email: "admin@example.com".to_string(),
                    username: Some("admin".to_string()),
                    avatar_url: None,
                },
                created: gerrit_change.created.clone(),
                updated: gerrit_change.updated.clone(),
                insertions: gerrit_change.insertions.unwrap_or(0) as u32,
                deletions: gerrit_change.deletions.unwrap_or(0) as u32,
                current_revision: gerrit_change.current_revision.unwrap_or_default(),
                current_patch_set_num: gerrit_change._number as u32,
                patch_sets: Vec::new(),
                files: Vec::new(),
                total_files: 0,
                reviewed_files: 0,
                local_comments: 0,
                remote_comments: 0,
                import_status: crate::models::gerrit::ImportStatus::Imported,
                last_sync: Some(now.clone()),
                conflict_status: crate::models::gerrit::ConflictStatus::None,
                metadata: std::collections::HashMap::new(),
            };
            
            // Store in database (acquire lock again)
            {
                let database = state.database.lock().unwrap();
                match database.store_gerrit_change(&internal_change) {
                    Ok(_) => {
                        info!("Successfully stored change in database: {}", internal_change.change_id);
                    }
                    Err(e) => {
                        warn!("Failed to store change in database: {}", e);
                        // Continue anyway, we can still return the change data
                    }
                }
            }
            
            // Convert to SimpleChange for frontend
            let simple_change = SimpleChange {
                id: internal_change.id,
                change_number: gerrit_change._number,
                subject: gerrit_change.subject,
                status: gerrit_change.status,
                project: gerrit_change.project,
                branch: gerrit_change.branch,
                topic: gerrit_change.topic,
                owner: format!("{:?}", gerrit_change.owner), // Simplified for now
                updated: gerrit_change.updated,
                created: gerrit_change.created,
                insertions: gerrit_change.insertions.unwrap_or(0),
                deletions: gerrit_change.deletions.unwrap_or(0),
                files: vec![], // TODO: Parse files from revisions
            };
            
            Ok(simple_change)
        }
        Err(e) => {
            warn!("Failed to import change from Gerrit: {}", e);
            
            // Try to get from database cache first
            let cached_result = {
                let database = state.database.lock().unwrap();
                database.get_gerrit_change(&change_id)
            };
            
            match cached_result {
                Ok(Some(cached_change)) => {
                    info!("Found cached change in database: {}", cached_change.subject);
                    let simple_change = SimpleChange {
                        id: cached_change.id,
                        change_number: cached_change.current_patch_set_num as i32,
                        subject: cached_change.subject,
                        status: cached_change.status.to_string(),
                        project: cached_change.project,
                        branch: cached_change.branch,
                        topic: None,
                        owner: cached_change.owner.name,
                        updated: cached_change.updated,
                        created: cached_change.created,
                        insertions: cached_change.insertions as i32,
                        deletions: cached_change.deletions as i32,
                        files: vec![],
                    };
                    return Ok(simple_change);
                }
                Ok(None) => {
                    info!("No cached change found in database for: {}", change_id);
                }
                Err(e) => {
                    warn!("Failed to query database for cached change: {}", e);
                }
            }
            
            // Return mock data as final fallback
            let mock_change = SimpleChange {
                id: Uuid::new_v4().to_string(),
                change_number: change_number,
                subject: format!("Mock change {} (Gerrit connection failed)", change_id),
                status: "NEW".to_string(),
                project: "test-project".to_string(),
                branch: "main".to_string(),
                topic: Some("feature/test".to_string()),
                owner: "admin".to_string(),
                updated: Utc::now().to_rfc3339(),
                created: Utc::now().to_rfc3339(),
                insertions: 42,
                deletions: 13,
                files: vec![
                    SimpleFileInfo {
                        path: "src/main.rs".to_string(),
                        change_type: "MODIFIED".to_string(),
                        insertions: 30,
                        deletions: 5,
                    },
                ],
            };
            
            Ok(mock_change)
        }
    }
}

/// Search for changes in Gerrit
#[tauri::command]
pub async fn gerrit_search_changes_simple(
    query: String,
    state: State<'_, AppState>,
) -> Result<Vec<SimpleChange>, String> {
    info!("Searching Gerrit changes with query: {}", query);
    
    // Validate query - if empty, provide a default query
    let search_query = if query.trim().is_empty() {
        "status:open".to_string() // Default to open changes
    } else {
        query.trim().to_string()
    };
    
    info!("Using search query: {}", search_query);
    
    // Get active Gerrit instance (release lock before async operations)
    let (gerrit_url, username, password) = {
        let database = state.database.lock().unwrap();
        
        // Initialize Gerrit schema if needed
        if let Err(e) = database.init_gerrit_schema() {
            warn!("Failed to initialize Gerrit schema: {}", e);
        }
        
        // Get active Gerrit instance
        let active_instance = match database.get_all_gerrit_instances() {
            Ok(instances) => instances.into_iter().find(|i| i.is_active),
            Err(e) => {
                warn!("Failed to get Gerrit instances: {}", e);
                None
            }
        };
        
        if let Some(instance) = active_instance {
            (instance.url, instance.username, instance.password_encrypted)
        } else {
            // Use test server as fallback
            (
                "http://edce7739774c:8080".to_string(),
                "admin".to_string(),
                "8EWK0RrulrdN8d7vTFVpbQTAjQFU2lFQpRhTZpISBw".to_string(),
            )
        }
    }; // Database lock is released here
    
    // Try to connect to the Gerrit server
    let client = GerritClient::new(&gerrit_url)
        .with_auth(username, password);
    
    match client.search_changes(&search_query).await {
        Ok(gerrit_changes) => {
            let mut simple_changes = Vec::new();
            
            for gerrit_change in gerrit_changes {
                // Convert to our internal GerritChange model and store in database
                let now = Utc::now().to_rfc3339();
                let internal_change = crate::models::gerrit::GerritChange {
                    id: Uuid::new_v4().to_string(),
                    change_id: gerrit_change.change_id.clone(),
                    instance_id: "test-instance-1".to_string(), // TODO: Get from active instance
                    project: gerrit_change.project.clone(),
                    branch: gerrit_change.branch.clone(),
                    subject: gerrit_change.subject.clone(),
                    status: crate::models::gerrit::ChangeStatus::from_string(&gerrit_change.status),
                    owner: crate::models::gerrit::GerritUser {
                        account_id: 0,
                        name: "admin".to_string(), // TODO: Parse from owner object
                        email: "admin@example.com".to_string(),
                        username: Some("admin".to_string()),
                        avatar_url: None,
                    },
                    created: gerrit_change.created.clone(),
                    updated: gerrit_change.updated.clone(),
                    insertions: gerrit_change.insertions.unwrap_or(0) as u32,
                    deletions: gerrit_change.deletions.unwrap_or(0) as u32,
                    current_revision: gerrit_change.current_revision.unwrap_or_default(),
                    current_patch_set_num: gerrit_change._number as u32,
                    patch_sets: Vec::new(),
                    files: Vec::new(),
                    total_files: 0,
                    reviewed_files: 0,
                    local_comments: 0,
                    remote_comments: 0,
                    import_status: crate::models::gerrit::ImportStatus::Imported,
                    last_sync: Some(now.clone()),
                    conflict_status: crate::models::gerrit::ConflictStatus::None,
                    metadata: std::collections::HashMap::new(),
                };
                
                // Store in database (acquire lock again)
                {
                    let database = state.database.lock().unwrap();
                    if let Err(e) = database.store_gerrit_change(&internal_change) {
                        warn!("Failed to store change in database: {}", e);
                    }
                }
                
                // Convert to SimpleChange for frontend
                let simple_change = SimpleChange {
                    id: internal_change.id,
                    change_number: gerrit_change._number,
                    subject: gerrit_change.subject,
                    status: gerrit_change.status,
                    project: gerrit_change.project,
                    branch: gerrit_change.branch,
                    topic: gerrit_change.topic,
                    owner: format!("{:?}", gerrit_change.owner), // Simplified for now
                    updated: gerrit_change.updated,
                    created: gerrit_change.created,
                    insertions: gerrit_change.insertions.unwrap_or(0),
                    deletions: gerrit_change.deletions.unwrap_or(0),
                    files: vec![], // TODO: Parse files from revisions
                };
                
                simple_changes.push(simple_change);
            }
            
            info!("Found and stored {} changes from Gerrit", simple_changes.len());
            Ok(simple_changes)
        }
        Err(e) => {
            warn!("Failed to search changes in Gerrit: {}", e);
            
            // Try to return cached results from database
            let cached_results = {
                let database = state.database.lock().unwrap();
                match database.get_all_gerrit_instances() {
                    Ok(instances) => {
                        if let Some(instance) = instances.first() {
                            database.get_gerrit_changes_for_instance(&instance.id)
                        } else {
                            Ok(Vec::new())
                        }
                    }
                    Err(e) => {
                        warn!("Failed to get instances for cached results: {}", e);
                        Ok(Vec::new())
                    }
                }
            };
            
            match cached_results {
                Ok(cached_changes) if !cached_changes.is_empty() => {
                    let simple_changes: Vec<SimpleChange> = cached_changes.into_iter().map(|change| {
                        SimpleChange {
                            id: change.id,
                            change_number: change.current_patch_set_num as i32,
                            subject: change.subject,
                            status: change.status.to_string(),
                            project: change.project,
                            branch: change.branch,
                            topic: None,
                            owner: change.owner.name,
                            updated: change.updated,
                            created: change.created,
                            insertions: change.insertions as i32,
                            deletions: change.deletions as i32,
                            files: vec![],
                        }
                    }).collect();
                    
                    info!("Returning {} cached changes from database", simple_changes.len());
                    return Ok(simple_changes);
                }
                Ok(_) => {
                    info!("No cached changes found in database");
                }
                Err(e) => {
                    warn!("Failed to get cached changes: {}", e);
                }
            }
            
            // Return mock search results as final fallback
            let mock_results = vec![
                SimpleChange {
                    id: Uuid::new_v4().to_string(),
                    change_number: 12346,
                    subject: format!("Mock search result for: {} (Gerrit connection failed)", search_query),
                    status: "NEW".to_string(),
                    project: "search-project".to_string(),
                    branch: "main".to_string(),
                    topic: None,
                    owner: "admin".to_string(),
                    updated: Utc::now().to_rfc3339(),
                    created: Utc::now().to_rfc3339(),
                    insertions: 25,
                    deletions: 7,
                    files: vec![],
                }
            ];
            
            Ok(mock_results)
        }
    }
}

/// Clear all Gerrit data (for debugging/testing purposes)
#[tauri::command]
pub async fn gerrit_clear_all_data_simple(
    state: State<'_, AppState>,
) -> Result<bool, String> {
    info!("Clearing all Gerrit data");
    
    let database = state.database.lock().unwrap();
    
    // This will clear all instances and their associated changes due to cascade delete
    match database.clear_all_gerrit_data() {
        Ok(rows_affected) => {
            info!("Cleared {} Gerrit instances and their associated data", rows_affected);
            Ok(true)
        }
        Err(e) => {
            error!("Failed to clear Gerrit data: {}", e);
            Err(format!("Failed to clear data: {}", e))
        }
    }
}

/// Set active Gerrit instance
#[tauri::command]
pub async fn gerrit_set_active_instance_simple(
    instance_id: String,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    info!("Setting active Gerrit instance: {}", instance_id);
    
    let database = state.database.lock().unwrap();
    
    // First check if the instance exists
    match database.get_gerrit_instance(&instance_id) {
        Ok(Some(_)) => {
            // Instance exists, proceed with setting it as active
            // First deactivate all instances
            match database.get_all_gerrit_instances() {
                Ok(instances) => {
                    for mut instance in instances {
                        instance.is_active = instance.id == instance_id;
                        if let Err(e) = database.store_gerrit_instance(&instance) {
                            error!("Failed to update instance {}: {}", instance.id, e);
                        }
                    }
                    info!("Successfully set instance {} as active", instance_id);
                    Ok(true)
                }
                Err(e) => {
                    error!("Failed to get instances: {}", e);
                    Err(format!("Failed to get instances: {}", e))
                }
            }
        }
        Ok(None) => {
            warn!("Instance not found: {}", instance_id);
            Err("Instance not found".to_string())
        }
        Err(e) => {
            error!("Database error: {}", e);
            Err(format!("Database error: {}", e))
        }
    }
}