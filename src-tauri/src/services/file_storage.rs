// File Storage Service
// Handles local file storage, organization, and caching for downloaded change files

use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Write};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use log::{info, warn, error, debug};
use sha2::{Sha256, Digest};

use crate::errors::HyperReviewError;
use crate::models::gerrit::{ChangeFile, FileChangeType};

/// File storage service for managing downloaded change files
pub struct FileStorage {
    base_path: PathBuf,
    cache: HashMap<String, CachedFileInfo>,
}

/// Information about a cached file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedFileInfo {
    pub file_path: String,
    pub change_id: String,
    pub patch_set_number: u32,
    pub local_path: PathBuf,
    pub checksum: String,
    pub size: u64,
    pub cached_at: String,
    pub last_accessed: String,
}

/// File storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStorageConfig {
    pub base_directory: PathBuf,
    pub max_cache_size_mb: u64,
    pub cleanup_threshold_days: u32,
    pub enable_compression: bool,
}

impl Default for FileStorageConfig {
    fn default() -> Self {
        Self {
            base_directory: PathBuf::from("hyperreview_cache"),
            max_cache_size_mb: 1024, // 1GB default
            cleanup_threshold_days: 30,
            enable_compression: false,
        }
    }
}

/// File storage operations result
#[derive(Debug)]
pub struct StorageResult {
    pub success: bool,
    pub local_path: Option<PathBuf>,
    pub size: u64,
    pub checksum: String,
}

impl FileStorage {
    /// Create a new file storage instance
    pub fn new(config: FileStorageConfig) -> Result<Self, HyperReviewError> {
        info!("Initializing file storage at: {:?}", config.base_directory);

        // Create base directory if it doesn't exist
        if !config.base_directory.exists() {
            fs::create_dir_all(&config.base_directory)
                .map_err(|e| HyperReviewError::other(format!("Failed to create storage directory: {}", e)))?;
        }

        let mut storage = Self {
            base_path: config.base_directory,
            cache: HashMap::new(),
        };

        // Load existing cache metadata
        storage.load_cache_metadata()?;

        info!("File storage initialized with {} cached files", storage.cache.len());
        Ok(storage)
    }

    /// Store a change file locally
    pub fn store_file(
        &mut self,
        change_file: &ChangeFile,
        change_id: &str,
        patch_set_number: u32,
    ) -> Result<StorageResult, HyperReviewError> {
        debug!("Storing file: {} for change {}", change_file.file_path, change_id);

        let local_path = self.get_local_path(change_id, patch_set_number, &change_file.file_path);
        
        // Create directory structure
        if let Some(parent) = local_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| HyperReviewError::other(format!("Failed to create directory: {}", e)))?;
        }

        // Write file content
        let binding = String::new();
        let content = change_file.new_content.as_ref()
            .unwrap_or(change_file.old_content.as_ref().unwrap_or(&binding));
        
        let mut file = fs::File::create(&local_path)
            .map_err(|e| HyperReviewError::other(format!("Failed to create file: {}", e)))?;
        
        file.write_all(content.as_bytes())
            .map_err(|e| HyperReviewError::other(format!("Failed to write file: {}", e)))?;

        // Calculate checksum
        let checksum = self.calculate_checksum(content.as_bytes());
        let size = content.len() as u64;

        // Store metadata
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let cache_info = CachedFileInfo {
            file_path: change_file.file_path.clone(),
            change_id: change_id.to_string(),
            patch_set_number,
            local_path: local_path.clone(),
            checksum: checksum.clone(),
            size,
            cached_at: now.clone(),
            last_accessed: now,
        };

        let cache_key = self.get_cache_key(change_id, patch_set_number, &change_file.file_path);
        self.cache.insert(cache_key, cache_info);

        // Save cache metadata
        self.save_cache_metadata()?;

        info!("Successfully stored file: {} ({} bytes)", change_file.file_path, size);

        Ok(StorageResult {
            success: true,
            local_path: Some(local_path),
            size,
            checksum,
        })
    }

    /// Retrieve a file from local storage
    pub fn get_file(
        &mut self,
        change_id: &str,
        patch_set_number: u32,
        file_path: &str,
    ) -> Result<Option<String>, HyperReviewError> {
        debug!("Retrieving file: {} for change {}", file_path, change_id);

        let cache_key = self.get_cache_key(change_id, patch_set_number, file_path);
        
        // Check if file exists in cache
        let should_remove = if let Some(cache_info) = self.cache.get(&cache_key) {
            // Update last accessed time
            let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

            // Check if file exists
            if cache_info.local_path.exists() {
                match fs::read_to_string(&cache_info.local_path) {
                    Ok(content) => {
                        // Verify checksum
                        let current_checksum = self.calculate_checksum(content.as_bytes());
                        if current_checksum == cache_info.checksum {
                            debug!("File retrieved from cache: {}", file_path);
                            // Update last accessed time after successful read
                            if let Some(cache_info_mut) = self.cache.get_mut(&cache_key) {
                                cache_info_mut.last_accessed = now;
                            }
                            return Ok(Some(content));
                        } else {
                            warn!("Checksum mismatch for file: {}, removing from cache", file_path);
                            true // Mark for removal
                        }
                    }
                    Err(e) => {
                        warn!("Failed to read cached file {}: {}", file_path, e);
                        true // Mark for removal
                    }
                }
            } else {
                warn!("Cached file not found on disk: {}", file_path);
                true // Mark for removal
            }
        } else {
            false // Not in cache
        };
        
        // Remove the file from cache if there were issues
        if should_remove {
            self.remove_file_from_cache(&cache_key)?;
        }

        debug!("File not found in cache: {}", file_path);
        Ok(None)
    }

    /// Check if a file is cached
    pub fn is_file_cached(
        &self,
        change_id: &str,
        patch_set_number: u32,
        file_path: &str,
    ) -> bool {
        let cache_key = self.get_cache_key(change_id, patch_set_number, file_path);
        
        if let Some(cache_info) = self.cache.get(&cache_key) {
            cache_info.local_path.exists()
        } else {
            false
        }
    }

    /// Get file metadata from cache
    pub fn get_file_metadata(
        &self,
        change_id: &str,
        patch_set_number: u32,
        file_path: &str,
    ) -> Option<&CachedFileInfo> {
        let cache_key = self.get_cache_key(change_id, patch_set_number, file_path);
        self.cache.get(&cache_key)
    }

    /// List all cached files for a change
    pub fn list_cached_files(
        &self,
        change_id: &str,
        patch_set_number: Option<u32>,
    ) -> Vec<&CachedFileInfo> {
        self.cache.values()
            .filter(|info| {
                info.change_id == change_id && 
                (patch_set_number.is_none() || Some(info.patch_set_number) == patch_set_number)
            })
            .collect()
    }

    /// Remove files for a specific change from cache
    pub fn remove_change_files(
        &mut self,
        change_id: &str,
        patch_set_number: Option<u32>,
    ) -> Result<u32, HyperReviewError> {
        info!("Removing cached files for change: {}", change_id);

        let keys_to_remove: Vec<String> = self.cache.iter()
            .filter(|(_, info)| {
                info.change_id == change_id && 
                (patch_set_number.is_none() || Some(info.patch_set_number) == patch_set_number)
            })
            .map(|(key, _)| key.clone())
            .collect();

        let mut removed_count = 0;
        for key in keys_to_remove {
            if self.remove_file_from_cache(&key).is_ok() {
                removed_count += 1;
            }
        }

        self.save_cache_metadata()?;
        info!("Removed {} files for change: {}", removed_count, change_id);
        Ok(removed_count)
    }

    /// Clean up old cached files
    pub fn cleanup_old_files(&mut self, days_threshold: u32) -> Result<u32, HyperReviewError> {
        info!("Cleaning up files older than {} days", days_threshold);

        let threshold_date = chrono::Utc::now() - chrono::Duration::days(days_threshold as i64);
        let threshold_str = threshold_date.format("%Y-%m-%d %H:%M:%S").to_string();

        let keys_to_remove: Vec<String> = self.cache.iter()
            .filter(|(_, info)| info.last_accessed < threshold_str)
            .map(|(key, _)| key.clone())
            .collect();

        let mut removed_count = 0;
        for key in keys_to_remove {
            if self.remove_file_from_cache(&key).is_ok() {
                removed_count += 1;
            }
        }

        self.save_cache_metadata()?;
        info!("Cleaned up {} old files", removed_count);
        Ok(removed_count)
    }

    /// Get cache statistics
    pub fn get_cache_stats(&self) -> CacheStats {
        let total_files = self.cache.len();
        let total_size = self.cache.values().map(|info| info.size).sum();
        let changes: std::collections::HashSet<_> = self.cache.values()
            .map(|info| &info.change_id)
            .collect();

        CacheStats {
            total_files: total_files as u32,
            total_size_bytes: total_size,
            total_changes: changes.len() as u32,
            cache_directory: self.base_path.clone(),
        }
    }

    // Private helper methods

    fn get_local_path(&self, change_id: &str, patch_set_number: u32, file_path: &str) -> PathBuf {
        let sanitized_path = file_path.replace(['/', '\\'], "_");
        self.base_path
            .join(change_id)
            .join(format!("ps{}", patch_set_number))
            .join(sanitized_path)
    }

    fn get_cache_key(&self, change_id: &str, patch_set_number: u32, file_path: &str) -> String {
        format!("{}:{}:{}", change_id, patch_set_number, file_path)
    }

    fn calculate_checksum(&self, content: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content);
        format!("{:x}", hasher.finalize())
    }

    fn remove_file_from_cache(&mut self, cache_key: &str) -> Result<(), HyperReviewError> {
        if let Some(cache_info) = self.cache.remove(cache_key) {
            if cache_info.local_path.exists() {
                fs::remove_file(&cache_info.local_path)
                    .map_err(|e| HyperReviewError::other(format!("Failed to remove file: {}", e)))?;
            }
        }
        Ok(())
    }

    fn load_cache_metadata(&mut self) -> Result<(), HyperReviewError> {
        let metadata_path = self.base_path.join("cache_metadata.json");
        
        if metadata_path.exists() {
            match fs::read_to_string(&metadata_path) {
                Ok(content) => {
                    match serde_json::from_str::<HashMap<String, CachedFileInfo>>(&content) {
                        Ok(cache) => {
                            self.cache = cache;
                            debug!("Loaded cache metadata with {} entries", self.cache.len());
                        }
                        Err(e) => {
                            warn!("Failed to parse cache metadata: {}", e);
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to read cache metadata: {}", e);
                }
            }
        }

        Ok(())
    }

    fn save_cache_metadata(&self) -> Result<(), HyperReviewError> {
        let metadata_path = self.base_path.join("cache_metadata.json");
        
        let content = serde_json::to_string_pretty(&self.cache)
            .map_err(|e| HyperReviewError::other(format!("Failed to serialize cache metadata: {}", e)))?;

        fs::write(&metadata_path, content)
            .map_err(|e| HyperReviewError::other(format!("Failed to write cache metadata: {}", e)))?;

        Ok(())
    }
}

/// Cache statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct CacheStats {
    pub total_files: u32,
    pub total_size_bytes: u64,
    pub total_changes: u32,
    pub cache_directory: PathBuf,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_storage() -> (FileStorage, TempDir) {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let config = FileStorageConfig {
            base_directory: temp_dir.path().to_path_buf(),
            max_cache_size_mb: 100,
            cleanup_threshold_days: 7,
            enable_compression: false,
        };
        let storage = FileStorage::new(config).expect("Failed to create storage");
        (storage, temp_dir)
    }

    fn create_test_change_file(path: &str, content: &str) -> ChangeFile {
        use crate::models::gerrit::FileDiff;
        
        ChangeFile {
            id: "test-file-id".to_string(),
            change_id: "test-change".to_string(),
            patch_set_number: 1,
            file_path: path.to_string(),
            change_type: FileChangeType::Modified,
            old_content: Some("old content".to_string()),
            new_content: Some(content.to_string()),
            diff: FileDiff::default(),
            file_size: content.len() as u64,
            downloaded_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }

    #[test]
    fn test_store_and_retrieve_file() {
        let (mut storage, _temp_dir) = create_test_storage();
        
        let change_file = create_test_change_file("src/main.rs", "fn main() {}");
        
        // Store file
        let result = storage.store_file(&change_file, "test-change", 1).unwrap();
        assert!(result.success);
        assert!(result.local_path.is_some());
        assert_eq!(result.size, 12); // "fn main() {}" length

        // Retrieve file
        let content = storage.get_file("test-change", 1, "src/main.rs").unwrap();
        assert_eq!(content, Some("fn main() {}".to_string()));
    }

    #[test]
    fn test_file_caching() {
        let (mut storage, _temp_dir) = create_test_storage();
        
        let change_file = create_test_change_file("test.txt", "test content");
        
        // Initially not cached
        assert!(!storage.is_file_cached("test-change", 1, "test.txt"));

        // Store file
        storage.store_file(&change_file, "test-change", 1).unwrap();

        // Now should be cached
        assert!(storage.is_file_cached("test-change", 1, "test.txt"));

        // Should have metadata
        let metadata = storage.get_file_metadata("test-change", 1, "test.txt");
        assert!(metadata.is_some());
        assert_eq!(metadata.unwrap().file_path, "test.txt");
    }

    #[test]
    fn test_list_cached_files() {
        let (mut storage, _temp_dir) = create_test_storage();
        
        // Store multiple files
        let file1 = create_test_change_file("file1.txt", "content1");
        let file2 = create_test_change_file("file2.txt", "content2");
        
        storage.store_file(&file1, "change1", 1).unwrap();
        storage.store_file(&file2, "change1", 1).unwrap();

        // List files for change
        let files = storage.list_cached_files("change1", Some(1));
        assert_eq!(files.len(), 2);

        // List all files for change (any patch set)
        let all_files = storage.list_cached_files("change1", None);
        assert_eq!(all_files.len(), 2);
    }

    #[test]
    fn test_remove_change_files() {
        let (mut storage, _temp_dir) = create_test_storage();
        
        let change_file = create_test_change_file("test.txt", "test content");
        storage.store_file(&change_file, "test-change", 1).unwrap();

        // Verify file is cached
        assert!(storage.is_file_cached("test-change", 1, "test.txt"));

        // Remove files for change
        let removed_count = storage.remove_change_files("test-change", None).unwrap();
        assert_eq!(removed_count, 1);

        // Verify file is no longer cached
        assert!(!storage.is_file_cached("test-change", 1, "test.txt"));
    }

    #[test]
    fn test_cache_stats() {
        let (mut storage, _temp_dir) = create_test_storage();
        
        let file1 = create_test_change_file("file1.txt", "content1");
        let file2 = create_test_change_file("file2.txt", "content2");
        
        storage.store_file(&file1, "change1", 1).unwrap();
        storage.store_file(&file2, "change2", 1).unwrap();

        let stats = storage.get_cache_stats();
        assert_eq!(stats.total_files, 2);
        assert_eq!(stats.total_changes, 2);
        assert!(stats.total_size_bytes > 0);
    }
}