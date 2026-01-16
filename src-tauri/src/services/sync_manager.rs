use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use log::{info, warn, error, debug};
use serde::{Deserialize, Serialize};

use crate::models::gerrit::*;
use crate::models::HyperReviewError;
use crate::storage::sqlite::Database;
use crate::remote::gerrit_client::GerritClient;

/// Manages synchronization between local data and Gerrit servers
pub struct SyncManager {
    db: Arc<Mutex<Database>>,
    gerrit_clients: HashMap<String, GerritClient>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    pub success: bool,
    pub changes_processed: u32,
    pub comments_processed: u32,
    pub conflicts_detected: u32,
    pub conflicts_resolved: u32,
    pub errors: Vec<SyncError>,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncError {
    pub change_id: Option<String>,
    pub error_type: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub suggested_action: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictType {
    ConcurrentEdit,
    LineModified,
    CommentDeleted,
    StatusChanged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictInfo {
    pub conflict_type: ConflictType,
    pub local_version: serde_json::Value,
    pub remote_version: serde_json::Value,
    pub base_version: Option<serde_json::Value>,
    pub resolution_options: Vec<ConflictResolutionOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictResolutionOption {
    pub id: String,
    pub description: String,
    pub action: String,
    pub auto_resolvable: bool,
}

impl SyncManager {
    pub fn new(db: Arc<Mutex<Database>>) -> Self {
        Self {
            db,
            gerrit_clients: HashMap::new(),
        }
    }
    
    /// Synchronize a specific change with Gerrit
    pub async fn sync_change(
        &mut self,
        instance_id: &str,
        change_id: &str,
        sync_type: SyncType,
    ) -> Result<SyncResult, HyperReviewError> {
        let start_time = std::time::Instant::now();
        info!("Starting sync for change {} on instance {}", change_id, instance_id);
        
        let mut result = SyncResult {
            success: true,
            changes_processed: 0,
            comments_processed: 0,
            conflicts_detected: 0,
            conflicts_resolved: 0,
            errors: Vec::new(),
            duration_ms: 0,
        };
        
        // Get or create Gerrit client for this instance
        let client = self.get_or_create_client(instance_id).await?;
        
        // Fetch current change data from Gerrit
        let remote_change = match client.get_change(change_id).await {
            Ok(change) => change,
            Err(e) => {
                result.success = false;
                result.errors.push(SyncError {
                    change_id: Some(change_id.to_string()),
                    error_type: "FetchError".to_string(),
                    message: format!("Failed to fetch change from Gerrit: {}", e),
                    timestamp: Utc::now(),
                    suggested_action: Some("Verify change ID and permissions".to_string()),
                });
                return Ok(result);
            }
        };
        
        // Get local change data
        let db = self.db.lock().map_err(|e| HyperReviewError::DatabaseError(e.to_string()))?;
        let local_change = db.get_gerrit_change(change_id)?;
        drop(db);
        
        // Compare and detect conflicts
        let conflicts = self.detect_conflicts(&local_change, &remote_change, sync_type)?;
        result.conflicts_detected = conflicts.len() as u32;
        
        // Resolve conflicts automatically where possible
        let mut resolved_conflicts = 0;
        for conflict in &conflicts {
            if self.try_auto_resolve_conflict(conflict).await? {
                resolved_conflicts += 1;
            }
        }
        result.conflicts_resolved = resolved_conflicts;
        
        // Update local data with merged changes
        match self.merge_changes(&local_change, &remote_change, sync_type).await {
            Ok(_) => {
                result.changes_processed += 1;
                info!("Successfully synced change {}", change_id);
            }
            Err(e) => {
                result.success = false;
                result.errors.push(SyncError {
                    change_id: Some(change_id.to_string()),
                    error_type: "MergeError".to_string(),
                    message: format!("Failed to merge changes: {}", e),
                    timestamp: Utc::now(),
                    suggested_action: Some("Manual intervention may be required".to_string()),
                });
            }
        }
        
        // Sync comments if requested
        if sync_type == SyncType::Full || sync_type == SyncType::CommentsOnly {
            match self.sync_comments(instance_id, change_id).await {
                Ok(comment_count) => {
                    result.comments_processed = comment_count;
                }
                Err(e) => {
                    warn!("Failed to sync comments for change {}: {}", change_id, e);
                    result.errors.push(SyncError {
                        change_id: Some(change_id.to_string()),
                        error_type: "CommentSyncError".to_string(),
                        message: format!("Comment sync failed: {}", e),
                        timestamp: Utc::now(),
                        suggested_action: Some("Comments may be out of sync".to_string()),
                    });
                }
            }
        }
        
        result.duration_ms = start_time.elapsed().as_millis() as u64;
        info!("Sync completed for change {} in {}ms", change_id, result.duration_ms);
        
        Ok(result)
    }
    
    /// Detect conflicts between local and remote data
    fn detect_conflicts(
        &self,
        local: &GerritChange,
        remote: &GerritChange,
        sync_type: SyncType,
    ) -> Result<Vec<ConflictInfo>, HyperReviewError> {
        let mut conflicts = Vec::new();
        
        // Check for concurrent modifications based on timestamps
        if local.updated < remote.updated {
            conflicts.push(ConflictInfo {
                conflict_type: ConflictType::ConcurrentEdit,
                local_version: serde_json::to_value(local)?,
                remote_version: serde_json::to_value(remote)?,
                base_version: None,
                resolution_options: vec![
                    ConflictResolutionOption {
                        id: "use_remote".to_string(),
                        description: "Use remote version".to_string(),
                        action: "overwrite_local".to_string(),
                        auto_resolvable: true,
                    },
                    ConflictResolutionOption {
                        id: "use_local".to_string(),
                        description: "Keep local version".to_string(),
                        action: "keep_local".to_string(),
                        auto_resolvable: false,
                    },
                ],
            });
        }
        
        // Check for patch set updates
        if local.current_patch_set_num != remote.current_patch_set_num {
            conflicts.push(ConflictInfo {
                conflict_type: ConflictType::StatusChanged,
                local_version: serde_json::to_value(&local.current_patch_set_num)?,
                remote_version: serde_json::to_value(&remote.current_patch_set_num)?,
                base_version: None,
                resolution_options: vec![
                    ConflictResolutionOption {
                        id: "update_patch_set".to_string(),
                        description: "Update to newer patch set".to_string(),
                        action: "update_patch_set".to_string(),
                        auto_resolvable: true,
                    },
                ],
            });
        }
        
        Ok(conflicts)
    }
    
    /// Attempt to automatically resolve a conflict
    async fn try_auto_resolve_conflict(
        &self,
        conflict: &ConflictInfo,
    ) -> Result<bool, HyperReviewError> {
        // Auto-resolve simple conflicts based on rules
        match conflict.conflict_type {
            ConflictType::ConcurrentEdit => {
                // If remote is newer and no semantic conflicts, auto-resolve
                if let Some(option) = conflict.resolution_options.iter()
                    .find(|opt| opt.auto_resolvable) {
                    debug!("Auto-resolving conflict: {}", option.description);
                    return Ok(true);
                }
            }
            ConflictType::StatusChanged => {
                // Always auto-resolve patch set updates
                debug!("Auto-resolving patch set update conflict");
                return Ok(true);
            }
            _ => {}
        }
        
        Ok(false)
    }
    
    /// Merge local and remote changes
    async fn merge_changes(
        &self,
        local: &GerritChange,
        remote: &GerritChange,
        sync_type: SyncType,
    ) -> Result<(), HyperReviewError> {
        let db = self.db.lock().map_err(|e| HyperReviewError::DatabaseError(e.to_string()))?;
        
        // Update local change with remote data based on sync type
        match sync_type {
            SyncType::Full => {
                // Update all fields except local-only data
                db.update_gerrit_change(&local.id,
                    &remote.subject,
                    &remote.status,
                    &remote.current_revision,
                    remote.current_patch_set_num,
                    &remote.updated,
                )?;
            }
            SyncType::StatusOnly => {
                // Update only status-related fields
                db.update_gerrit_change_status(&local.id, &remote.status)?;
            }
            _ => {}
        }
        
        Ok(())
    }
    
    /// Sync comments for a change
    async fn sync_comments(
        &self,
        instance_id: &str,
        change_id: &str,
    ) -> Result<u32, HyperReviewError> {
        let client = self.get_or_create_client(instance_id).await?;
        
        // Fetch remote comments
        let remote_comments = client.get_comments(change_id).await?;
        
        let db = self.db.lock().map_err(|e| HyperReviewError::DatabaseError(e.to_string()))?;
        let mut synced_count = 0;
        
        for comment in remote_comments {
            // Check if comment already exists locally
            if !db.comment_exists(&comment.id)? {
                db.insert_comment(&comment)?;
                synced_count += 1;
            }
        }
        
        Ok(synced_count)
    }
    
    /// Get or create a Gerrit client for an instance
    async fn get_or_create_client(
        &mut self,
        instance_id: &str,
    ) -> Result<&mut GerritClient, HyperReviewError> {
        if !self.gerrit_clients.contains_key(instance_id) {
            let db = self.db.lock().map_err(|e| HyperReviewError::DatabaseError(e.to_string()))?;
            let instance = db.get_gerrit_instance(instance_id)?;
            drop(db);
            
            let client = GerritClient::new(&instance.url, &instance.username, &instance.password_encrypted)?;
            self.gerrit_clients.insert(instance_id.to_string(), client);
        }
        
        self.gerrit_clients.get_mut(instance_id)
            .ok_or_else(|| HyperReviewError::InternalError("Failed to get Gerrit client".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::sqlite::Database;
    
    #[test]
    fn test_conflict_detection() {
        // Test conflict detection logic
        let local = GerritChange {
            id: "test-change".to_string(),
            change_id: "12345".to_string(),
            updated: "2025-01-01T00:00:00Z".to_string(),
            current_patch_set_num: 1,
            ..Default::default()
        };
        
        let remote = GerritChange {
            id: "test-change".to_string(),
            change_id: "12345".to_string(),
            updated: "2025-01-02T00:00:00Z".to_string(),
            current_patch_set_num: 2,
            ..Default::default()
        };
        
        let sync_manager = SyncManager::new(Arc::new(Mutex::new(Database::new_in_memory().unwrap())));
        let conflicts = sync_manager.detect_conflicts(&local, &remote, SyncType::Full).unwrap();
        
        assert!(!conflicts.is_empty());
        assert!(conflicts.iter().any(|c| matches!(c.conflict_type, ConflictType::ConcurrentEdit)));
    }
}