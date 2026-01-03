use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use log::{info, debug, warn};

use crate::models::gerrit::*;
use crate::errors::HyperReviewError;

/// Manages offline storage of Gerrit data including changes, comments, and files
pub struct OfflineCache {
    cache_dir: PathBuf,
    metadata: CacheMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetadata {
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub total_entries: u64,
    pub cache_size_bytes: u64,
    pub instance_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedChange {
    pub change: GerritChange,
    pub files: HashMap<String, CachedFile>,
    pub comments: Vec<GerritComment>,
    pub cached_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedFile {
    pub file: GerritFile,
    pub diff_content: Option<String>,
    pub cached_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedComment {
    pub comment: GerritComment,
    pub local_modifications: Option<LocalModifications>,
    pub cached_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalModifications {
    pub modified_at: DateTime<Utc>,
    pub original_message: String,
    pub modification_type: ModificationType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModificationType {
    Edit,
    Delete,
    Add,
}

impl OfflineCache {
    /// Create a new offline cache instance
    pub fn new(cache_dir: PathBuf) -> Result<Self, HyperReviewError> {
        // Ensure cache directory exists
        fs::create_dir_all(&cache_dir)
            .map_err(|e| HyperReviewError::Other { message: format!("Failed to create cache directory: {}", e) })?;
        
        let metadata = Self::load_or_create_metadata(&cache_dir)?;
        
        Ok(Self {
            cache_dir,
            metadata,
        })
    }
    
    /// Cache a complete change with all its data
    pub fn cache_change(
        &mut self,
        change: &GerritChange,
        files: &[CachedFile],
        comments: &[GerritComment],
        ttl_hours: u32,
    ) -> Result<(), HyperReviewError> {
        let cached_change = CachedChange {
            change: change.clone(),
            files: files.iter()
                .map(|f| (f.file.file_path.clone(), f.clone()))
                .collect(),
            comments: comments.to_vec(),
            cached_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::hours(ttl_hours as i64),
        };
        
        let file_path = self.get_change_cache_path(&change.id);
        let json = serde_json::to_string_pretty(&cached_change)
            .map_err(|e| HyperReviewError::Other { message: e.to_string() })?;
        
        fs::write(&file_path, json)
            .map_err(|e| HyperReviewError::Other { message: format!("Failed to write cache file: {}", e) })?;
        
        self.update_metadata();
        info!("Cached change {} with {} files and {} comments", change.id, files.len(), comments.len());
        
        Ok(())
    }
    
    /// Retrieve a cached change
    pub fn get_cached_change(&self,
        change_id: &str,
    ) -> Result<Option<CachedChange>, HyperReviewError> {
        let file_path = self.get_change_cache_path(change_id);
        
        if !file_path.exists() {
            return Ok(None);
        }
        
        let json = fs::read_to_string(&file_path)
            .map_err(|e| HyperReviewError::Other { message: format!("Failed to read cache file: {}", e) })?;
        
        let cached_change: CachedChange = serde_json::from_str(&json)
            .map_err(|e| HyperReviewError::Other { message: e.to_string() })?;
        
        // Check if cache has expired
        if cached_change.expires_at < Utc::now() {
            debug!("Cache expired for change {}", change_id);
            return Ok(None);
        }
        
        Ok(Some(cached_change))
    }
    
    /// Cache individual file with diff content
    pub fn cache_file(
        &mut self,
        file: &GerritFile,
        diff_content: Option<String>,
        ttl_hours: u32,
    ) -> Result<(), HyperReviewError> {
        let cached_file = CachedFile {
            file: file.clone(),
            diff_content,
            cached_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::hours(ttl_hours as i64),
        };
        
        let file_path = self.get_file_cache_path(&file.id);
        let json = serde_json::to_string_pretty(&cached_file)
            .map_err(|e| HyperReviewError::Other { message: e.to_string() })?;
        
        fs::write(&file_path, json)
            .map_err(|e| HyperReviewError::Other { message: format!("Failed to write file cache: {}", e) })?;
        
        self.update_metadata();
        debug!("Cached file {} with {} diff content", file.file_path, 
               if cached_file.diff_content.is_some() { "has" } else { "no" });
        
        Ok(())
    }
    
    /// Get cached file
    pub fn get_cached_file(&self,
        file_id: &str,
    ) -> Result<Option<CachedFile>, HyperReviewError> {
        let file_path = self.get_file_cache_path(file_id);
        
        if !file_path.exists() {
            return Ok(None);
        }
        
        let json = fs::read_to_string(&file_path)
            .map_err(|e| HyperReviewError::Other { message: format!("Failed to read file cache: {}", e) })?;
        
        let cached_file: CachedFile = serde_json::from_str(&json)
            .map_err(|e| HyperReviewError::Other { message: e.to_string() })?;
        
        if cached_file.expires_at < Utc::now() {
            return Ok(None);
        }
        
        Ok(Some(cached_file))
    }
    
    /// Store local modifications to comments
    pub fn store_local_modification(
        &mut self,
        comment_id: &str,
        original_message: &str,
        new_message: &str,
        modification_type: ModificationType,
    ) -> Result<(), HyperReviewError> {
        let cached_comment = CachedComment {
            comment: GerritComment {
                id: comment_id.to_string(),
                message: new_message.to_string(),
                ..Default::default()
            },
            local_modifications: Some(LocalModifications {
                modified_at: Utc::now(),
                original_message: original_message.to_string(),
                modification_type,
            }),
            cached_at: Utc::now(),
        };
        
        let file_path = self.get_comment_cache_path(comment_id);
        let json = serde_json::to_string_pretty(&cached_comment)
            .map_err(|e| HyperReviewError::Other { message: e.to_string() })?;
        
        fs::write(&file_path, json)
            .map_err(|e| HyperReviewError::Other { message: format!("Failed to write comment cache: {}", e) })?;
        
        self.update_metadata();
        info!("Stored local modification for comment {}", comment_id);
        
        Ok(())
    }
    
    /// Clear expired cache entries
    pub fn cleanup_expired_entries(&mut self,
    ) -> Result<u64, HyperReviewError> {
        let mut removed_count = 0;
        let now = Utc::now();
        
        // Clean up change caches
        let changes_dir = self.cache_dir.join("changes");
        if changes_dir.exists() {
            for entry in fs::read_dir(&changes_dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if let Ok(json) = fs::read_to_string(&path) {
                    if let Ok(cached_change) = serde_json::from_str::<CachedChange>(&json) {
                        if cached_change.expires_at < now {
                            fs::remove_file(&path)?;
                            removed_count += 1;
                            debug!("Removed expired change cache: {:?}", path);
                        }
                    }
                }
            }
        }
        
        // Clean up file caches
        let files_dir = self.cache_dir.join("files");
        if files_dir.exists() {
            for entry in fs::read_dir(&files_dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if let Ok(json) = fs::read_to_string(&path) {
                    if let Ok(cached_file) = serde_json::from_str::<CachedFile>(&json) {
                        if cached_file.expires_at < now {
                            fs::remove_file(&path)?;
                            removed_count += 1;
                            debug!("Removed expired file cache: {:?}", path);
                        }
                    }
                }
            }
        }
        
        self.update_metadata();
        info!("Cleaned up {} expired cache entries", removed_count);
        
        Ok(removed_count)
    }
    
    /// Get cache statistics
    pub fn get_cache_stats(&self,
    ) -> Result<CacheStats, HyperReviewError> {
        let mut total_entries = 0;
        let mut total_size = 0u64;
        
        // Count change caches
        let changes_dir = self.cache_dir.join("changes");
        if changes_dir.exists() {
            for entry in fs::read_dir(&changes_dir)? {
                let entry = entry?;
                let metadata = entry.metadata()?;
                total_entries += 1;
                total_size += metadata.len();
            }
        }
        
        // Count file caches
        let files_dir = self.cache_dir.join("files");
        if files_dir.exists() {
            for entry in fs::read_dir(&files_dir)? {
                let entry = entry?;
                let metadata = entry.metadata()?;
                total_entries += 1;
                total_size += metadata.len();
            }
        }
        
        Ok(CacheStats {
            total_entries,
            total_size_bytes: total_size,
            cache_hit_rate: 0.0, // Would need to track hits/misses
            oldest_entry: None, // Would need to track oldest
            newest_entry: None, // Would need to track newest
        })
    }
    
    // Helper methods
    
    fn get_change_cache_path(&self,
        change_id: &str,
    ) -> PathBuf {
        self.cache_dir.join("changes").join(format!("{}.json", change_id))
    }
    
    fn get_file_cache_path(
        &self,
        file_id: &str,
    ) -> PathBuf {
        self.cache_dir.join("files").join(format!("{}.json", file_id))
    }
    
    fn get_comment_cache_path(
        &self,
        comment_id: &str,
    ) -> PathBuf {
        self.cache_dir.join("comments").join(format!("{}.json", comment_id))
    }
    
    fn load_or_create_metadata(
        cache_dir: &Path,
    ) -> Result<CacheMetadata, HyperReviewError> {
        let metadata_path = cache_dir.join("metadata.json");
        
        if metadata_path.exists() {
            let json = fs::read_to_string(&metadata_path)
                .map_err(|e| HyperReviewError::Other { message: format!("Failed to read metadata: {}", e) })?;
            
            let metadata: CacheMetadata = serde_json::from_str(&json)
                .map_err(|e| HyperReviewError::Other { message: e.to_string() })?;
            
            Ok(metadata)
        } else {
            let metadata = CacheMetadata {
                version: "1.0.0".to_string(),
                created_at: Utc::now(),
                last_updated: Utc::now(),
                total_entries: 0,
                cache_size_bytes: 0,
                instance_ids: Vec::new(),
            };
            
            let json = serde_json::to_string_pretty(&metadata)
                .map_err(|e| HyperReviewError::Other { message: e.to_string() })?;
            
            fs::write(&metadata_path, json)
                .map_err(|e| HyperReviewError::Other { message: format!("Failed to write metadata: {}", e) })?;
            
            Ok(metadata)
        }
    }
    
    fn update_metadata(&mut self) {
        self.metadata.last_updated = Utc::now();
        self.metadata.total_entries = self.get_cache_stats().unwrap().total_entries;
        self.metadata.cache_size_bytes = self.get_cache_stats().unwrap().total_size_bytes;
        
        // Save updated metadata
        let metadata_path = self.cache_dir.join("metadata.json");
        if let Ok(json) = serde_json::to_string_pretty(&self.metadata) {
            let _ = fs::write(metadata_path, json);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub total_entries: u64,
    pub total_size_bytes: u64,
    pub cache_hit_rate: f64,
    pub oldest_entry: Option<DateTime<Utc>>,
    pub newest_entry: Option<DateTime<Utc>>,
}

// Default implementations for testing
impl Default for GerritChange {
    fn default() -> Self {
        GerritChange {
            id: String::new(),
            change_id: String::new(),
            instance_id: String::new(),
            project: String::new(),
            branch: String::new(),
            subject: String::new(),
            status: ChangeStatus::New,
            owner: GerritUser {
                account_id: 0,
                name: String::new(),
                email: String::new(),
                username: None,
                avatar_url: None,
            },
            created: Utc::now().to_rfc3339(),
            updated: Utc::now().to_rfc3339(),
            insertions: 0,
            deletions: 0,
            current_revision: String::new(),
            current_patch_set_num: 1,
            patch_sets: Vec::new(),
            files: Vec::new(),
            total_files: 0,
            reviewed_files: 0,
            local_comments: 0,
            remote_comments: 0,
            import_status: ImportStatus::Pending,
            last_sync: None,
            conflict_status: ConflictStatus::None,
            metadata: HashMap::new(),
        }
    }
}

impl Default for GerritComment {
    fn default() -> Self {
        GerritComment {
            id: String::new(),
            gerrit_comment_id: None,
            change_id: String::new(),
            patch_set_id: String::new(),
            file_path: String::new(),
            side: CommentSide::Revision,
            line: 1,
            range: None,
            message: String::new(),
            author: GerritUser {
                account_id: 0,
                name: String::new(),
                email: String::new(),
                username: None,
                avatar_url: None,
            },
            created: Utc::now().to_rfc3339(),
            updated: Utc::now().to_rfc3339(),
            status: CommentSyncStatus::LocalOnly,
            unresolved: true,
            parent: None,
            robot_id: None,
            properties: HashMap::new(),
        }
    }
}