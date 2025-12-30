// Gerrit Integration Models
// Data structures for Gerrit Code Review integration

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Gerrit Instance Configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GerritInstance {
    pub id: String,                    // UUID v4
    pub name: String,                  // Display name
    pub url: String,                   // Base URL
    pub username: String,              // Authentication username
    pub password_encrypted: String,    // AES-encrypted password/token
    pub version: String,               // Gerrit version
    pub is_active: bool,               // Currently selected instance
    pub last_connected: Option<String>, // ISO 8601 timestamp
    pub connection_status: ConnectionStatus,
    pub polling_interval: u32,         // Seconds (default: 300)
    pub max_changes: u32,              // Max changes to import (default: 100)
    pub created_at: String,            // ISO 8601 timestamp
    pub updated_at: String,            // ISO 8601 timestamp
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
    AuthenticationFailed,
    VersionIncompatible,
    NetworkError,
}

/// Gerrit Change Entity
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GerritChange {
    pub id: String,                    // UUID v4 (local)
    pub change_id: String,             // Gerrit Change-ID
    pub instance_id: String,           // Foreign key to GerritInstance
    pub project: String,               // Project name
    pub branch: String,                // Target branch
    pub subject: String,               // Change title/subject
    pub status: ChangeStatus,          // Current status
    pub owner: GerritUser,             // Change owner information
    pub created: String,               // ISO 8601 timestamp
    pub updated: String,               // ISO 8601 timestamp
    pub insertions: u32,               // Lines added
    pub deletions: u32,                // Lines removed
    pub current_revision: String,      // Current patch set revision
    pub current_patch_set_num: u32,    // Current patch set number
    pub patch_sets: Vec<PatchSet>,     // All patch sets
    pub files: Vec<GerritFile>,        // Files in current patch set
    pub total_files: u32,              // Total file count
    pub reviewed_files: u32,           // Locally reviewed files
    pub local_comments: u32,           // Local comments count
    pub remote_comments: u32,          // Remote comments count
    pub import_status: ImportStatus,   // Local import state
    pub last_sync: Option<String>,     // ISO 8601 timestamp
    pub conflict_status: ConflictStatus,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChangeStatus {
    New,
    Draft,
    Merged,
    Abandoned,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ImportStatus {
    Pending,
    Importing,
    Imported,
    Failed,
    Outdated,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ConflictStatus {
    None,
    CommentsPending,
    PatchSetUpdated,
    ManualResolutionRequired,
}

/// Gerrit Comment Entity
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GerritComment {
    pub id: String,                    // UUID v4 (local)
    pub gerrit_comment_id: Option<String>, // Gerrit comment ID (remote only)
    pub change_id: String,             // Foreign key to GerritChange
    pub patch_set_id: String,          // Foreign key to PatchSet
    pub file_path: String,             // File path within change
    pub side: CommentSide,             // Side of diff (PARENT/REVISION)
    pub line: u32,                     // Line number
    pub range: Option<CommentRange>,   // Character range for inline comments
    pub message: String,               // Comment content
    pub author: GerritUser,            // Comment author
    pub created: String,               // ISO 8601 timestamp
    pub updated: String,               // ISO 8601 timestamp
    pub status: CommentSyncStatus,     // Sync state
    pub unresolved: bool,              // Is comment unresolved?
    pub parent: Option<String>,        // Parent comment ID (for threads)
    pub robot_id: Option<String>,      // Automated comment identifier
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CommentSide {
    Parent,      // Old version
    Revision,    // New version
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommentRange {
    pub start_line: u32,
    pub start_character: u32,
    pub end_line: u32,
    pub end_character: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CommentSyncStatus {
    LocalOnly,         // Created locally, not synced
    SyncPending,       // Queued for sync
    Synced,            // Successfully synced to Gerrit
    SyncFailed,        // Sync failed, needs retry
    ConflictDetected,  // Conflict with remote comment
    ModifiedLocally,   // Modified after sync
}

/// Gerrit Review Entity
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GerritReview {
    pub id: String,                    // UUID v4 (local)
    pub gerrit_review_id: Option<String>, // Gerrit review ID (remote)
    pub change_id: String,             // Foreign key to GerritChange
    pub patch_set_id: String,          // Target patch set
    pub message: String,               // Review message
    pub labels: HashMap<String, i32>,  // Label scores (Code-Review, Verified, etc.)
    pub comments: Vec<String>,         // Associated comment IDs
    pub author: GerritUser,            // Review author (local user)
    pub created: String,               // ISO 8601 timestamp
    pub submitted: Option<String>,     // ISO 8601 timestamp (when pushed to Gerrit)
    pub status: ReviewStatus,          // Local status
    pub draft: bool,                   // Is this a draft review?
    pub notify: NotifyHandling,        // Email notification settings
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ReviewStatus {
    Draft,
    PendingSubmission,
    Submitted,
    SubmissionFailed,
    PartiallySubmitted,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NotifyHandling {
    None,
    Owner,
    OwnerReviewers,
    All,
}

/// Gerrit File Entity
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GerritFile {
    pub id: String,                    // UUID v4 (local)
    pub change_id: String,             // Foreign key to GerritChange
    pub patch_set_id: String,          // Foreign key to PatchSet
    pub file_path: String,             // File path
    pub old_path: Option<String>,      // Previous path (for renames)
    pub change_type: FileChangeType,   // Type of change
    pub status: FileStatus,            // Review status
    pub lines_inserted: u32,           // Lines added
    pub lines_deleted: u32,            // Lines removed
    pub size_delta: i32,               // Size change in bytes
    pub size_new: u32,                 // New file size
    pub is_binary: bool,               // Is binary file?
    pub content_type: String,          // MIME type
    pub diff_content: Option<String>,  // Cached diff content
    pub review_progress: ReviewProgress,
    pub last_reviewed: Option<String>, // ISO 8601 timestamp
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FileChangeType {
    Added,
    Modified,
    Deleted,
    Renamed,
    Copied,
    Rewritten,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FileStatus {
    Unreviewed,
    Pending,
    Reviewed,
    Approved,
    NeedsWork,
    Question,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReviewProgress {
    pub total_lines: u32,
    pub reviewed_lines: u32,
    pub comment_count: u32,
    pub severity_score: u32,           // 0-100
}

/// Patch Set Entity
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PatchSet {
    pub id: String,                    // UUID v4 (local)
    pub gerrit_patch_set_id: String,   // Gerrit patch set ID
    pub change_id: String,             // Foreign key to GerritChange
    pub revision: String,              // Git commit SHA
    pub number: u32,                   // Patch set number
    pub author: GerritUser,            // Patch set author
    pub commit_message: String,        // Commit message
    pub created: String,               // ISO 8601 timestamp
    pub kind: PatchSetKind,            // Type of patch set
    pub files: Vec<String>,            // File IDs in this patch set
    pub size_insertions: u32,          // Total lines added
    pub size_deletions: u32,           // Total lines removed
    pub is_current: bool,              // Is this the current patch set?
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PatchSetKind {
    Rework,
    TrivialRebase,
    NoCodeChange,
    NoChange,
    MergeFirstParentUpdate,
    Merge,
    Rewritten,
}

/// Gerrit User Information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GerritUser {
    pub account_id: u32,               // Gerrit account ID
    pub name: String,                  // Display name
    pub email: String,                 // Email address
    pub username: Option<String>,      // Username
    pub avatar_url: Option<String>,    // Avatar image URL
}

/// Sync Status Tracking
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncStatus {
    pub id: String,                    // UUID v4
    pub instance_id: String,           // Foreign key to GerritInstance
    pub change_id: Option<String>,     // Optional: specific change
    pub last_sync: String,             // ISO 8601 timestamp
    pub next_sync: Option<String>,     // ISO 8601 timestamp
    pub sync_type: SyncType,           // Type of sync operation
    pub status: SyncOperationStatus,   // Current status
    pub items_processed: u32,          // Number of items processed
    pub items_total: u32,              // Total items to process
    pub conflicts_detected: u32,       // Number of conflicts
    pub errors: Vec<SyncError>,        // Error details
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SyncType {
    Full,
    Incremental,
    CommentsOnly,
    StatusOnly,
    PushLocal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SyncOperationStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncError {
    pub code: String,                  // Error code
    pub message: String,               // Error message
    pub context: Option<String>,       // Additional context
    pub timestamp: String,             // ISO 8601 timestamp
}

/// Operation Queue for Offline/Sync Operations
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OperationQueue {
    pub id: String,                    // UUID v4
    pub instance_id: String,           // Foreign key to GerritInstance
    pub change_id: String,             // Foreign key to GerritChange
    pub operation_type: OperationType, // Type of operation
    pub payload: String,               // JSON-encoded operation data
    pub priority: OperationPriority,   // Execution priority
    pub status: OperationStatus,       // Current status
    pub retry_count: u32,              // Number of retry attempts
    pub max_retries: u32,              // Maximum retry attempts
    pub created: String,               // ISO 8601 timestamp
    pub last_attempt: Option<String>,  // ISO 8601 timestamp
    pub next_retry: Option<String>,    // ISO 8601 timestamp
    pub error_message: Option<String>, // Last error
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OperationType {
    AddComment,
    UpdateComment,
    DeleteComment,
    SubmitReview,
    UpdateLabels,
    PushPatchSet,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OperationPriority {
    Low,
    Normal,
    High,
    Critical,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OperationStatus {
    Queued,
    Processing,
    Completed,
    Failed,
    Cancelled,
    WaitingForDependency,
}

/// Search Query and Results
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchQuery {
    pub id: String,                    // UUID v4
    pub instance_id: String,           // Foreign key to GerritInstance
    pub query: String,                 // Gerrit query string
    pub query_type: QueryType,         // Type of query
    pub results: Vec<SearchResult>,    // Cached results
    pub result_count: u32,             // Total results available
    pub status: QueryStatus,           // Query execution status
    pub created: String,               // ISO 8601 timestamp
    pub expires: String,               // ISO 8601 timestamp
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum QueryType {
    ChangeId,
    Status,
    Project,
    Owner,
    Search,
    Custom,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum QueryStatus {
    Pending,
    Executing,
    Completed,
    Failed,
    Expired,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResult {
    pub change_id: String,             // Gerrit Change-ID
    pub project: String,               // Project name
    pub branch: String,                // Target branch
    pub subject: String,               // Change subject
    pub status: ChangeStatus,          // Change status
    pub owner: GerritUser,             // Change owner
    pub created: String,               // ISO 8601 timestamp
    pub updated: String,               // ISO 8601 timestamp
    pub insertions: u32,               // Lines added
    pub deletions: u32,                // Lines removed
    pub score: f32,                    // Relevance score 0-100
}

/// API Request/Response Types

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportChangeParams {
    pub instance_id: String,
    pub change_id: String,  // Gerrit Change-ID or search query
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportChangeResult {
    pub success: bool,
    pub change: Option<GerritChange>,
    pub message: String,
    pub error_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddCommentParams {
    pub change_id: String,
    pub patch_set_id: String,
    pub file_path: String,
    pub line: u32,
    pub message: String,
    pub side: CommentSide,
    pub parent_id: Option<String>,
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
pub struct SyncParams {
    pub instance_id: String,
    pub change_id: Option<String>,
    pub sync_type: SyncType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchParams {
    pub instance_id: String,
    pub query: String,
    pub query_type: QueryType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub success: bool,
    pub query_id: String,
    pub results: Vec<SearchResult>,
    pub total_count: u32,
    pub message: String,
}

/// Helper Functions

impl GerritInstance {
    pub fn new(name: String, url: String, username: String, password_encrypted: String) -> Self {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            url,
            username,
            password_encrypted,
            version: String::new(),
            is_active: false,
            last_connected: None,
            connection_status: ConnectionStatus::Disconnected,
            polling_interval: 300,
            max_changes: 100,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    pub fn mark_connected(&mut self, version: String) {
        self.version = version;
        self.connection_status = ConnectionStatus::Connected;
        self.last_connected = Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
        self.updated_at = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    }

    pub fn mark_disconnected(&mut self, status: ConnectionStatus) {
        self.connection_status = status;
        self.updated_at = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    }
}

impl GerritChange {
    pub fn completion_percentage(&self) -> f32 {
        if self.total_files == 0 {
            0.0
        } else {
            (self.reviewed_files as f32 / self.total_files as f32) * 100.0
        }
    }

    pub fn has_local_changes(&self) -> bool {
        self.local_comments > 0 || self.import_status == ImportStatus::Outdated
    }

    pub fn needs_sync(&self) -> bool {
        self.conflict_status != ConflictStatus::None || self.has_local_changes()
    }
}

impl GerritComment {
    pub fn is_local(&self) -> bool {
        self.status == CommentSyncStatus::LocalOnly || 
        self.status == CommentSyncStatus::SyncPending ||
        self.status == CommentSyncStatus::ModifiedLocally
    }

    pub fn can_edit(&self, user_id: u32) -> bool {
        self.author.account_id == user_id && self.is_local()
    }
}

impl OperationQueue {
    pub fn should_retry(&self) -> bool {
        self.retry_count < self.max_retries && 
        matches!(self.status, OperationStatus::Failed | OperationStatus::Cancelled)
    }

    pub fn increment_retry(&mut self) {
        self.retry_count += 1;
        let now = chrono::Utc::now();
        self.last_attempt = Some(now.format("%Y-%m-%d %H:%M:%S").to_string());
        
        // Exponential backoff: 2^retry_count seconds
        let backoff_seconds = 2_u32.pow(self.retry_count);
        let next_retry_time = now + chrono::Duration::seconds(backoff_seconds as i64);
        self.next_retry = Some(next_retry_time.format("%Y-%m-%d %H:%M:%S").to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gerrit_change_completion() {
        let change = GerritChange {
            id: "test".to_string(),
            change_id: "I12345".to_string(),
            instance_id: "instance1".to_string(),
            project: "test-project".to_string(),
            branch: "main".to_string(),
            subject: "Test change".to_string(),
            status: ChangeStatus::New,
            owner: GerritUser {
                account_id: 123,
                name: "Test User".to_string(),
                email: "test@example.com".to_string(),
                username: Some("testuser".to_string()),
                avatar_url: None,
            },
            created: "2025-01-01 00:00:00".to_string(),
            updated: "2025-01-01 00:00:00".to_string(),
            insertions: 100,
            deletions: 50,
            current_revision: "abc123".to_string(),
            current_patch_set_num: 1,
            patch_sets: vec![],
            files: vec![],
            total_files: 10,
            reviewed_files: 7,
            local_comments: 5,
            remote_comments: 3,
            import_status: ImportStatus::Imported,
            last_sync: None,
            conflict_status: ConflictStatus::None,
            metadata: HashMap::new(),
        };

        assert_eq!(change.completion_percentage(), 70.0);
        assert!(change.has_local_changes());
        assert!(change.needs_sync());
    }

    #[test]
    fn test_operation_queue_retry() {
        let mut operation = OperationQueue {
            id: "test".to_string(),
            instance_id: "instance1".to_string(),
            change_id: "change1".to_string(),
            operation_type: OperationType::AddComment,
            payload: "{}".to_string(),
            priority: OperationPriority::Normal,
            status: OperationStatus::Failed,
            retry_count: 0,
            max_retries: 3,
            created: "2025-01-01 00:00:00".to_string(),
            last_attempt: None,
            next_retry: None,
            error_message: Some("Test error".to_string()),
        };

        assert!(operation.should_retry());
        
        operation.increment_retry();
        assert_eq!(operation.retry_count, 1);
        assert!(operation.last_attempt.is_some());
        assert!(operation.next_retry.is_some());
    }
}