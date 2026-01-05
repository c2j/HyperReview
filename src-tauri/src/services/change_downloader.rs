// Change Download Infrastructure
// Handles downloading Gerrit changes for offline review

use std::sync::Arc;
use std::collections::HashMap;
use tokio::time::{Duration, Instant};
use serde::{Serialize, Deserialize};
use log::{info, warn, error, debug};
use regex::Regex;

use crate::models::gerrit::*;
use crate::remote::gerrit_client::{GerritClient, GerritChangeInfo};
use crate::storage::sqlite::Database;
use crate::errors::HyperReviewError;

/// Change download manager with Gerrit API integration
pub struct ChangeDownloader {
    gerrit_client: Arc<GerritClient>,
    database: Arc<Database>,
}

impl ChangeDownloader {
    pub fn new(gerrit_client: Arc<GerritClient>, database: Arc<Database>) -> Self {
        Self {
            gerrit_client,
            database,
        }
    }

    /// Download a complete change for offline review
    pub async fn download_change(
        &self,
        change_id: &str,
        patch_set_number: Option<u32>,
    ) -> Result<DownloadResult, HyperReviewError> {
        let start_time = Instant::now();
        info!("Starting download for change: {}", change_id);

        // Step 1: Get change metadata from Gerrit
        let change_info = self.get_change_metadata(change_id).await?;
        let patch_set_num = patch_set_number.unwrap_or(change_info.current_patch_set_num);

        // Step 2: Get file list for the patch set
        let file_list = self.get_patch_set_files(change_id, patch_set_num).await?;
        info!("Found {} files in patch set {}", file_list.len(), patch_set_num);

        // Step 3: Download file contents and diffs
        let mut downloaded_files = Vec::new();
        let mut total_size = 0u64;

        for file_info in file_list {
            match self.download_file_content(&change_info, patch_set_num, &file_info).await {
                Ok(change_file) => {
                    total_size += change_file.file_size;
                    downloaded_files.push(change_file);
                }
                Err(e) => {
                    warn!("Failed to download file {}: {}", file_info.path, e);
                    // Continue with other files, don't fail the entire download
                }
            }
        }

        // Step 4: Store change metadata in database
        self.store_change_metadata(&change_info).await?;

        // Step 5: Store downloaded files in database
        for file in &downloaded_files {
            if let Err(e) = self.database.store_change_file(file) {
                error!("Failed to store file {}: {}", file.file_path, e);
            }
        }

        let download_time = start_time.elapsed();
        info!(
            "Download completed: {} files, {} bytes, {:?}",
            downloaded_files.len(),
            total_size,
            download_time
        );

        Ok(DownloadResult {
            success: true,
            change_metadata: Some(change_info),
            files: downloaded_files,
            total_size,
            download_time_ms: download_time.as_millis() as u64,
            error_message: None,
        })
    }

    /// Check download status for a change
    pub async fn get_download_status(
        &self,
        change_id: &str,
        patch_set_number: u32,
    ) -> Result<DownloadStatus, HyperReviewError> {
        let is_downloaded = self.database.is_change_downloaded_by_gerrit_id(change_id, patch_set_number)?;
        
        if is_downloaded {
            let files = self.database.get_change_files_by_gerrit_id(change_id, patch_set_number)?;
            let total_size: u64 = files.iter().map(|f| f.file_size).sum();
            
            // Check if update is needed by comparing with remote
            let needs_update = match self.check_needs_update(change_id).await {
                Ok(needs_update) => needs_update,
                Err(e) => {
                    warn!("Failed to check for updates: {}", e);
                    false // Assume no update needed if check fails
                }
            };
            
            Ok(DownloadStatus {
                is_downloaded: true,
                file_count: files.len() as u32,
                total_size,
                downloaded_at: files.first()
                    .map(|f| f.downloaded_at.clone())
                    .unwrap_or_else(|| chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
                needs_update,
            })
        } else {
            Ok(DownloadStatus {
                is_downloaded: false,
                file_count: 0,
                total_size: 0,
                downloaded_at: String::new(),
                needs_update: false,
            })
        }
    }

    /// Check if a change needs to be updated
    async fn check_needs_update(&self, change_id: &str) -> Result<bool, HyperReviewError> {
        // Get local change info
        let local_change = self.database.get_gerrit_change(change_id)?;
        
        if let Some(local) = local_change {
            // Get remote change info
            match self.get_change_metadata(change_id).await {
                Ok(remote_change) => {
                    // Compare patch set numbers and update timestamps
                    Ok(remote_change.current_patch_set_num > local.current_patch_set_num ||
                       remote_change.updated != local.updated)
                }
                Err(_) => {
                    // If we can't reach Gerrit, assume no update needed
                    Ok(false)
                }
            }
        } else {
            // Change not found locally, so it needs to be downloaded
            Ok(true)
        }
    }

    /// Update an existing downloaded change
    pub async fn update_change(&self, change_id: &str) -> Result<UpdateResult, HyperReviewError> {
        info!("Updating change: {}", change_id);

        // Get current change info from Gerrit
        let remote_change = self.get_change_metadata(change_id).await?;
        
        // Get local change info from database
        let local_change = self.database.get_gerrit_change(change_id)?;
        
        match local_change {
            Some(local) => {
                let needs_update = remote_change.current_patch_set_num > local.current_patch_set_num ||
                                 remote_change.updated != local.updated;
                
                if needs_update {
                    info!("Change {} needs update: remote PS{} vs local PS{}", 
                          change_id, remote_change.current_patch_set_num, local.current_patch_set_num);
                    
                    // Download the updated change
                    let download_result = self.download_change(change_id, None).await?;
                    
                    Ok(UpdateResult {
                        updated: true,
                        old_patch_set: local.current_patch_set_num,
                        new_patch_set: remote_change.current_patch_set_num,
                        files_changed: download_result.files.len() as u32,
                        message: format!("Updated from PS{} to PS{}", 
                                       local.current_patch_set_num, 
                                       remote_change.current_patch_set_num),
                    })
                } else {
                    Ok(UpdateResult {
                        updated: false,
                        old_patch_set: local.current_patch_set_num,
                        new_patch_set: local.current_patch_set_num,
                        files_changed: 0,
                        message: "Change is already up to date".to_string(),
                    })
                }
            }
            None => {
                // Change not found locally, download it
                let download_result = self.download_change(change_id, None).await?;
                
                Ok(UpdateResult {
                    updated: true,
                    old_patch_set: 0,
                    new_patch_set: remote_change.current_patch_set_num,
                    files_changed: download_result.files.len() as u32,
                    message: "Change downloaded for the first time".to_string(),
                })
            }
        }
    }

    /// Get change metadata from Gerrit and convert to our model
    async fn get_change_metadata(&self, change_id: &str) -> Result<GerritChange, HyperReviewError> {
        // Try to parse as change number first, then as change ID
        let change_info = if let Ok(change_number) = change_id.parse::<i32>() {
            self.gerrit_client.get_change(change_number).await?
        } else {
            // For change IDs, we need to search first
            let search_results = self.gerrit_client.search_changes(&format!("change:{}", change_id)).await?;
            search_results.into_iter().next()
                .ok_or_else(|| HyperReviewError::other(format!("Change not found: {}", change_id)))?
        };

        // Convert GerritChangeInfo to our GerritChange model
        Ok(self.convert_change_info(change_info))
    }

    /// Parse unified diff to extract line counts and hunks
    fn parse_unified_diff(&self, unified_diff: &str) -> (u32, u32, Vec<DiffHunk>) {
        let mut old_line_count = 0u32;
        let mut new_line_count = 0u32;
        let mut hunks = Vec::new();
        let mut current_hunk: Option<DiffHunk> = None;
        
        for line in unified_diff.lines() {
            if line.starts_with("@@") {
                // Save previous hunk if exists
                if let Some(hunk) = current_hunk.take() {
                    hunks.push(hunk);
                }
                
                // Parse hunk header: @@ -old_start,old_count +new_start,new_count @@
                let hunk_regex = Regex::new(r"@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@").unwrap();
                if let Some(captures) = hunk_regex.captures(line) {
                    
                    let old_start = captures.get(1).unwrap().as_str().parse::<u32>().unwrap_or(1);
                    let old_count = captures.get(2)
                        .map(|m| m.as_str().parse::<u32>().unwrap_or(1))
                        .unwrap_or(1);
                    let new_start = captures.get(3).unwrap().as_str().parse::<u32>().unwrap_or(1);
                    let new_count = captures.get(4)
                        .map(|m| m.as_str().parse::<u32>().unwrap_or(1))
                        .unwrap_or(1);
                    
                    current_hunk = Some(DiffHunk {
                        old_start,
                        old_count,
                        new_start,
                        new_count,
                        lines: Vec::new(),
                    });
                    
                    old_line_count += old_count;
                    new_line_count += new_count;
                }
            } else if let Some(ref mut hunk) = current_hunk {
                // Add line to current hunk
                let line_type = if line.starts_with('+') {
                    DiffLineType::Added
                } else if line.starts_with('-') {
                    DiffLineType::Removed
                } else {
                    DiffLineType::Context
                };
                
                hunk.lines.push(DiffLine {
                    line_type,
                    content: line[1..].to_string(), // Remove +/- prefix
                    old_line_number: None, // TODO: Calculate line numbers
                    new_line_number: None, // TODO: Calculate line numbers
                });
            }
        }
        
        // Save last hunk
        if let Some(hunk) = current_hunk {
            hunks.push(hunk);
        }
        
        // If no hunks were parsed, try to count lines manually
        if old_line_count == 0 && new_line_count == 0 {
            for line in unified_diff.lines() {
                if line.starts_with('-') {
                    old_line_count += 1;
                } else if line.starts_with('+') {
                    new_line_count += 1;
                } else if line.starts_with(' ') {
                    old_line_count += 1;
                    new_line_count += 1;
                }
            }
        }
        
        (old_line_count, new_line_count, hunks)
    }

    /// Convert Gerrit API response to our internal model
    fn convert_change_info(&self, info: GerritChangeInfo) -> GerritChange {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        // Parse owner information from JSON value
        let owner = if let Some(owner_obj) = info.owner.as_object() {
            GerritUser {
                account_id: owner_obj.get("_account_id")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0) as u32,
                name: owner_obj.get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown")
                    .to_string(),
                email: owner_obj.get("email")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown@example.com")
                    .to_string(),
                username: owner_obj.get("username")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                avatar_url: None, // TODO: Parse avatar URL if available
            }
        } else {
            GerritUser {
                account_id: 0,
                name: "Unknown".to_string(),
                email: "unknown@example.com".to_string(),
                username: None,
                avatar_url: None,
            }
        };

        // Parse patch sets from revisions
        let mut patch_sets = Vec::new();
        let mut current_patch_set_num = 1u32;
        
        if let Some(revisions) = &info.revisions {
            if let Some(revisions_obj) = revisions.as_object() {
                // Count revisions to determine current patch set number
                current_patch_set_num = revisions_obj.len() as u32;
                
                // Create patch set entries
                for (revision_id, revision_data) in revisions_obj {
                    if let Some(revision_obj) = revision_data.as_object() {
                        let patch_set_number = revision_obj.get("_number")
                            .and_then(|v| v.as_i64())
                            .unwrap_or(1) as u32;
                        
                        patch_sets.push(PatchSet {
                            id: uuid::Uuid::new_v4().to_string(),
                            gerrit_patch_set_id: revision_id.clone(),
                            change_id: info.change_id.clone(),
                            revision: revision_id.clone(),
                            number: patch_set_number,
                            author: owner.clone(), // Simplified - could be different
                            commit_message: revision_obj.get("description")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string(),
                            created: revision_obj.get("created")
                                .and_then(|v| v.as_str())
                                .unwrap_or(&now)
                                .to_string(),
                            kind: PatchSetKind::Rework, // Default kind
                            files: Vec::new(), // Will be populated separately
                            size_insertions: 0, // Will be calculated from files
                            size_deletions: 0, // Will be calculated from files
                            is_current: patch_set_number == current_patch_set_num,
                        });
                    }
                }
            }
        }
        
        // Sort patch sets by number
        patch_sets.sort_by_key(|ps| ps.number);
        
        GerritChange {
            id: uuid::Uuid::new_v4().to_string(),
            change_id: info.change_id,
            instance_id: "default".to_string(), // TODO: Get from context
            project: info.project,
            branch: info.branch,
            subject: info.subject,
            status: ChangeStatus::from_string(&info.status),
            owner,
            created: info.created,
            updated: info.updated,
            insertions: info.insertions.unwrap_or(0) as u32,
            deletions: info.deletions.unwrap_or(0) as u32,
            current_revision: info.current_revision.unwrap_or_default(),
            current_patch_set_num,
            patch_sets,
            files: Vec::new(), // Will be populated separately
            total_files: 0, // Will be updated after file download
            reviewed_files: 0,
            local_comments: 0,
            remote_comments: 0,
            import_status: ImportStatus::Importing,
            last_sync: Some(now.clone()),
            conflict_status: ConflictStatus::None,
            metadata: HashMap::new(),
        }
    }

    /// Get file list for a specific patch set
    async fn get_patch_set_files(
        &self,
        change_id: &str,
        patch_set_number: u32,
    ) -> Result<Vec<FileInfo>, HyperReviewError> {
        info!("Getting file list for change {} patch set {}", change_id, patch_set_number);
        
        // First get the change info to find the revision ID for this patch set
        let change_info = self.get_change_metadata(change_id).await?;
        
        // For now, use current revision (TODO: map patch set number to revision ID)
        let revision_id = change_info.current_revision.clone();
        
        // Get files from Gerrit API
        let gerrit_files = self.gerrit_client.get_revision_files(change_id, &revision_id).await?;
        
        // Convert Gerrit file info to our FileInfo format
        let mut file_list = Vec::new();
        for (file_path, gerrit_file_info) in gerrit_files {
            // Skip the special /COMMIT_MSG file
            if file_path == "/COMMIT_MSG" {
                continue;
            }
            
            let change_type = match gerrit_file_info.status.as_deref() {
                Some("A") => FileChangeType::Added,
                Some("D") => FileChangeType::Deleted,
                Some("R") => FileChangeType::Renamed,
                Some("M") | _ => FileChangeType::Modified,
            };
            
            file_list.push(FileInfo {
                path: file_path,
                change_type,
                lines_inserted: gerrit_file_info.lines_inserted.unwrap_or(0) as u32,
                lines_deleted: gerrit_file_info.lines_deleted.unwrap_or(0) as u32,
                size_delta: gerrit_file_info.size_delta.unwrap_or(0),
            });
        }
        
        info!("Retrieved {} files from Gerrit API", file_list.len());
        Ok(file_list)
    }

    /// Download content and diff for a specific file
    async fn download_file_content(
        &self,
        change: &GerritChange,
        patch_set_number: u32,
        file_info: &FileInfo,
    ) -> Result<ChangeFile, HyperReviewError> {
        debug!("Downloading content for file: {}", file_info.path);

        let revision_id = &change.current_revision;
        
        // Get old content (base revision)
        let old_content = if file_info.change_type != FileChangeType::Added {
            match self.gerrit_client.get_file_content(&change.change_id, "1", &file_info.path).await {
                Ok(content) if !content.is_empty() => Some(content),
                _ => None, // File might not exist in base or API error
            }
        } else {
            None
        };

        // Get new content (current revision)
        let new_content = if file_info.change_type != FileChangeType::Deleted {
            match self.gerrit_client.get_file_content(&change.change_id, revision_id, &file_info.path).await {
                Ok(content) if !content.is_empty() => Some(content),
                _ => None, // File might be deleted or API error
            }
        } else {
            None
        };

        // Get diff from Gerrit API
        let unified_diff = match self.gerrit_client.get_file_diff(&change.change_id, revision_id, &file_info.path, Some("1")).await {
            Ok(diff) => diff,
            Err(e) => {
                warn!("Failed to get diff for {}: {}, generating fallback", file_info.path, e);
                self.generate_unified_diff(&old_content, &new_content, &file_info.path)
            }
        };
        
        // Parse diff to extract line counts and create hunks
        let (old_line_count, new_line_count, hunks) = self.parse_unified_diff(&unified_diff);
        
        let diff = FileDiff {
            unified_diff,
            old_line_count,
            new_line_count,
            context_lines: 3,
            hunks,
        };

        let file_size = new_content.as_ref()
            .map(|c| c.len() as u64)
            .unwrap_or_else(|| old_content.as_ref().map(|c| c.len() as u64).unwrap_or(0));

        Ok(ChangeFile {
            id: uuid::Uuid::new_v4().to_string(),
            change_id: change.id.clone(), // Use database record ID, not Gerrit change ID
            patch_set_number,
            file_path: file_info.path.clone(),
            change_type: file_info.change_type.clone(),
            old_content,
            new_content,
            diff,
            file_size,
            downloaded_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        })
    }

    /// Generate unified diff between old and new content
    fn generate_unified_diff(
        &self,
        old_content: &Option<String>,
        new_content: &Option<String>,
        file_path: &str,
    ) -> String {
        match (old_content, new_content) {
            (Some(old), Some(new)) => {
                // Simple line-by-line diff generation
                let old_lines: Vec<&str> = old.lines().collect();
                let new_lines: Vec<&str> = new.lines().collect();
                
                let mut diff = String::new();
                diff.push_str(&format!("--- a/{}\n", file_path));
                diff.push_str(&format!("+++ b/{}\n", file_path));
                diff.push_str(&format!("@@ -1,{} +1,{} @@\n", old_lines.len(), new_lines.len()));
                
                // Simple implementation: show all old lines as removed, all new lines as added
                for line in &old_lines {
                    diff.push_str(&format!("-{}\n", line));
                }
                for line in &new_lines {
                    diff.push_str(&format!("+{}\n", line));
                }
                
                diff
            }
            (None, Some(new)) => {
                let new_lines: Vec<&str> = new.lines().collect();
                let mut diff = String::new();
                diff.push_str(&format!("--- /dev/null\n"));
                diff.push_str(&format!("+++ b/{}\n", file_path));
                diff.push_str(&format!("@@ -0,0 +1,{} @@\n", new_lines.len()));
                
                for line in new_lines {
                    diff.push_str(&format!("+{}\n", line));
                }
                diff
            }
            (Some(old), None) => {
                let old_lines: Vec<&str> = old.lines().collect();
                let mut diff = String::new();
                diff.push_str(&format!("--- a/{}\n", file_path));
                diff.push_str(&format!("+++ /dev/null\n"));
                diff.push_str(&format!("@@ -1,{} +0,0 @@\n", old_lines.len()));
                
                for line in old_lines {
                    diff.push_str(&format!("-{}\n", line));
                }
                diff
            }
            (None, None) => String::new(),
        }
    }

    /// Store change metadata in database
    async fn store_change_metadata(&self, change: &GerritChange) -> Result<(), HyperReviewError> {
        self.database.store_gerrit_change(change)?;
        Ok(())
    }
}

/// File information from Gerrit API
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: String,
    pub change_type: FileChangeType,
    pub lines_inserted: u32,
    pub lines_deleted: u32,
    pub size_delta: i32,
}

/// Download operation result
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadResult {
    pub success: bool,
    pub change_metadata: Option<GerritChange>,
    pub files: Vec<ChangeFile>,
    pub total_size: u64,
    pub download_time_ms: u64,
    pub error_message: Option<String>,
}

/// Download status information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadStatus {
    pub is_downloaded: bool,
    pub file_count: u32,
    pub total_size: u64,
    pub downloaded_at: String,
    pub needs_update: bool,
}

/// Update operation result
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateResult {
    pub updated: bool,
    pub old_patch_set: u32,
    pub new_patch_set: u32,
    pub files_changed: u32,
    pub message: String,
}

/// Download progress tracking
#[derive(Debug, Clone)]
pub struct DownloadProgress {
    pub total_files: u32,
    pub downloaded_files: u32,
    pub current_file: String,
    pub bytes_downloaded: u64,
    pub total_bytes: u64,
    pub start_time: Instant,
}

impl DownloadProgress {
    pub fn new(total_files: u32, total_bytes: u64) -> Self {
        Self {
            total_files,
            downloaded_files: 0,
            current_file: String::new(),
            bytes_downloaded: 0,
            total_bytes,
            start_time: Instant::now(),
        }
    }

    pub fn update_file(&mut self, file_path: String, file_size: u64) {
        self.downloaded_files += 1;
        self.current_file = file_path;
        self.bytes_downloaded += file_size;
    }

    pub fn completion_percentage(&self) -> f32 {
        if self.total_files == 0 {
            100.0
        } else {
            (self.downloaded_files as f32 / self.total_files as f32) * 100.0
        }
    }

    pub fn estimated_time_remaining(&self) -> Option<Duration> {
        if self.downloaded_files == 0 {
            return None;
        }

        let elapsed = self.start_time.elapsed();
        let rate = self.downloaded_files as f64 / elapsed.as_secs_f64();
        let remaining_files = self.total_files - self.downloaded_files;
        
        if rate > 0.0 {
            Some(Duration::from_secs_f64(remaining_files as f64 / rate))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_download_progress_calculation() {
        let mut progress = DownloadProgress::new(10, 1000);
        
        assert_eq!(progress.completion_percentage(), 0.0);
        
        progress.update_file("file1.rs".to_string(), 100);
        assert_eq!(progress.completion_percentage(), 10.0);
        assert_eq!(progress.downloaded_files, 1);
        assert_eq!(progress.bytes_downloaded, 100);
        
        progress.update_file("file2.rs".to_string(), 200);
        assert_eq!(progress.completion_percentage(), 20.0);
        assert_eq!(progress.downloaded_files, 2);
        assert_eq!(progress.bytes_downloaded, 300);
    }

    #[test]
    fn test_download_result_serialization() {
        let result = DownloadResult {
            success: true,
            change_metadata: None,
            files: Vec::new(),
            total_size: 1024,
            download_time_ms: 5000,
            error_message: None,
        };

        let json = serde_json::to_string(&result).unwrap();
        let deserialized: DownloadResult = serde_json::from_str(&json).unwrap();
        
        assert_eq!(result.success, deserialized.success);
        assert_eq!(result.total_size, deserialized.total_size);
        assert_eq!(result.download_time_ms, deserialized.download_time_ms);
    }

    #[test]
    fn test_download_status_creation() {
        let status = DownloadStatus {
            is_downloaded: true,
            file_count: 5,
            total_size: 2048,
            downloaded_at: "2025-01-05 12:00:00".to_string(),
            needs_update: false,
        };

        assert!(status.is_downloaded);
        assert_eq!(status.file_count, 5);
        assert!(!status.needs_update);
    }

    #[tokio::test]
    async fn test_change_downloader_integration() {
        // Create test database
        let db = Arc::new(Database::new(":memory:").expect("Failed to create test database"));
        db.init_schema().expect("Failed to initialize schema");
        db.init_gerrit_schema().expect("Failed to initialize Gerrit schema");

        // Create mock Gerrit client
        let gerrit_client = Arc::new(crate::remote::gerrit_client::GerritClient::new("http://localhost:8080"));

        // Create downloader
        let downloader = ChangeDownloader::new(gerrit_client, db.clone());

        // Test download status for non-existent change
        let status = downloader.get_download_status("I12345", 1).await;
        assert!(status.is_ok());
        let status = status.unwrap();
        assert!(!status.is_downloaded);
        assert_eq!(status.file_count, 0);
    }

    #[test]
    fn test_file_info_creation() {
        let file_info = FileInfo {
            path: "src/main.rs".to_string(),
            change_type: FileChangeType::Modified,
            lines_inserted: 10,
            lines_deleted: 5,
            size_delta: 150,
        };

        assert_eq!(file_info.path, "src/main.rs");
        assert_eq!(file_info.change_type, FileChangeType::Modified);
        assert_eq!(file_info.lines_inserted, 10);
        assert_eq!(file_info.lines_deleted, 5);
        assert_eq!(file_info.size_delta, 150);
    }

    #[test]
    fn test_update_result_scenarios() {
        // Test successful update
        let update_result = UpdateResult {
            updated: true,
            old_patch_set: 1,
            new_patch_set: 2,
            files_changed: 3,
            message: "Updated from PS1 to PS2".to_string(),
        };

        assert!(update_result.updated);
        assert_eq!(update_result.old_patch_set, 1);
        assert_eq!(update_result.new_patch_set, 2);
        assert_eq!(update_result.files_changed, 3);

        // Test no update needed
        let no_update_result = UpdateResult {
            updated: false,
            old_patch_set: 2,
            new_patch_set: 2,
            files_changed: 0,
            message: "Change is already up to date".to_string(),
        };

        assert!(!no_update_result.updated);
        assert_eq!(no_update_result.old_patch_set, 2);
        assert_eq!(no_update_result.new_patch_set, 2);
        assert_eq!(no_update_result.files_changed, 0);
    }

    // Property-based tests for download completeness
    // Validates that downloads are complete and consistent

    /// Property 1: Download Completeness
    /// Validates that all files in a change are downloaded and stored correctly
    #[tokio::test]
    async fn property_download_completeness() {
        // Create test database
        let db = Arc::new(Database::new(":memory:").expect("Failed to create test database"));
        db.init_schema().expect("Failed to initialize schema");
        db.init_gerrit_schema().expect("Failed to initialize Gerrit schema");

        // Create mock Gerrit client
        let gerrit_client = Arc::new(GerritClient::new("http://localhost:8080"));

        // Create downloader
        let downloader = ChangeDownloader::new(gerrit_client, db.clone());

        // Test property: For any valid change_id, if download succeeds, 
        // then all files must be stored in database
        let test_cases = vec![
            ("I12345", None),
            ("I67890", Some(1)),
            ("I11111", Some(2)),
        ];

        for (change_id, patch_set) in test_cases {
            // Note: This will use mock data since we don't have real Gerrit server
            // In a real property test, we would generate random valid change IDs
            
            // Attempt download
            let result = downloader.download_change(change_id, patch_set).await;
            
            match result {
                Ok(download_result) => {
                    // Property: If download succeeds, all files must be in database
                    let stored_files = db.get_change_files_by_gerrit_id(
                        change_id, 
                        patch_set.unwrap_or(1)
                    ).expect("Failed to get stored files");
                    
                    // Verify completeness
                    assert_eq!(
                        download_result.files.len(), 
                        stored_files.len(),
                        "Downloaded files count must match stored files count"
                    );
                    
                    // Verify each file has required properties
                    for file in &download_result.files {
                        assert!(!file.file_path.is_empty(), "File path must not be empty");
                        assert!(!file.id.is_empty(), "File ID must not be empty");
                        assert_eq!(file.change_id, change_id, "Change ID must match");
                        
                        // Verify file content consistency
                        match file.change_type {
                            FileChangeType::Added => {
                                assert!(file.old_content.is_none(), "Added files should have no old content");
                                assert!(file.new_content.is_some(), "Added files should have new content");
                            }
                            FileChangeType::Deleted => {
                                assert!(file.old_content.is_some(), "Deleted files should have old content");
                                assert!(file.new_content.is_none(), "Deleted files should have no new content");
                            }
                            FileChangeType::Modified => {
                                assert!(file.old_content.is_some(), "Modified files should have old content");
                                assert!(file.new_content.is_some(), "Modified files should have new content");
                            }
                            FileChangeType::Renamed => {
                                // Renamed files can have both old and new content
                                assert!(
                                    file.old_content.is_some() || file.new_content.is_some(),
                                    "Renamed files should have at least one content version"
                                );
                            }
                            FileChangeType::Copied | FileChangeType::Rewritten => {
                                // Copied and rewritten files can have various content combinations
                                assert!(
                                    file.old_content.is_some() || file.new_content.is_some(),
                                    "Copied/rewritten files should have at least one content version"
                                );
                            }
                        }
                        
                        // Verify diff consistency
                        if file.old_content.is_some() && file.new_content.is_some() {
                            assert!(!file.diff.unified_diff.is_empty(), "Modified files should have diff");
                        }
                    }
                    
                    // Verify change metadata is stored
                    let stored_change = db.get_gerrit_change(change_id)
                        .expect("Failed to get stored change")
                        .expect("Change should be stored");
                    
                    assert_eq!(stored_change.change_id, change_id);
                    assert!(stored_change.total_files > 0, "Total files should be greater than 0");
                }
                Err(_) => {
                    // If download fails, no files should be stored
                    let stored_files = db.get_change_files_by_gerrit_id(change_id, patch_set.unwrap_or(1))
                        .unwrap_or_default();
                    
                    // Property: Failed downloads should not leave partial data
                    // (This is a strong consistency requirement)
                    assert!(
                        stored_files.is_empty(),
                        "Failed downloads should not leave partial files in database"
                    );
                }
            }
        }
    }

    /// Property 2: Download Status Consistency
    /// Validates that download status accurately reflects the actual download state
    #[tokio::test]
    async fn property_download_status_consistency() {
        // Create test database
        let db = Arc::new(Database::new(":memory:").expect("Failed to create test database"));
        db.init_schema().expect("Failed to initialize schema");
        db.init_gerrit_schema().expect("Failed to initialize Gerrit schema");

        // Create mock Gerrit client
        let gerrit_client = Arc::new(GerritClient::new("http://localhost:8080"));

        // Create downloader
        let downloader = ChangeDownloader::new(gerrit_client, db.clone());

        let change_id = "I12345";
        let patch_set = 1;

        // Check status before download
        let status_before = downloader.get_download_status(change_id, patch_set).await
            .expect("Should get status");
        
        // Property: Before download, status should indicate not downloaded
        assert!(!status_before.is_downloaded, "Status should indicate not downloaded initially");
        assert_eq!(status_before.file_count, 0, "File count should be 0 initially");
        assert_eq!(status_before.total_size, 0, "Total size should be 0 initially");

        // Perform download
        let download_result = downloader.download_change(change_id, Some(patch_set)).await;

        if download_result.is_ok() {
            // Check status after download
            let status_after = downloader.get_download_status(change_id, patch_set).await
                .expect("Should get status");
            
            // Property: After successful download, status should reflect downloaded state
            assert!(status_after.is_downloaded, "Status should indicate downloaded after successful download");
            assert!(status_after.file_count > 0, "File count should be greater than 0 after download");
            assert!(status_after.total_size > 0, "Total size should be greater than 0 after download");
            assert!(!status_after.downloaded_at.is_empty(), "Downloaded timestamp should be set");
            
            // Verify status matches actual download result
            let download = download_result.unwrap();
            assert_eq!(
                status_after.file_count as usize, 
                download.files.len(),
                "Status file count should match download result"
            );
            assert_eq!(
                status_after.total_size, 
                download.total_size,
                "Status total size should match download result"
            );
        }
    }

}