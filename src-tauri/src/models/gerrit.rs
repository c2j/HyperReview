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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
    AuthenticationFailed,
    VersionIncompatible,
    NetworkError,
}

impl std::fmt::Display for ConnectionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionStatus::Connected => write!(f, "Connected"),
            ConnectionStatus::Disconnected => write!(f, "Disconnected"),
            ConnectionStatus::AuthenticationFailed => write!(f, "AuthenticationFailed"),
            ConnectionStatus::VersionIncompatible => write!(f, "VersionIncompatible"),
            ConnectionStatus::NetworkError => write!(f, "NetworkError"),
        }
    }
}

impl ConnectionStatus {
    pub fn from_string(s: &str) -> Self {
        match s {
            "Connected" => ConnectionStatus::Connected,
            "Disconnected" => ConnectionStatus::Disconnected,
            "AuthenticationFailed" => ConnectionStatus::AuthenticationFailed,
            "VersionIncompatible" => ConnectionStatus::VersionIncompatible,
            "NetworkError" => ConnectionStatus::NetworkError,
            _ => ConnectionStatus::Disconnected,
        }
    }
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ChangeStatus {
    New,
    Draft,
    Merged,
    Abandoned,
}

impl std::fmt::Display for ChangeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangeStatus::New => write!(f, "new"),
            ChangeStatus::Draft => write!(f, "draft"),
            ChangeStatus::Merged => write!(f, "merged"),
            ChangeStatus::Abandoned => write!(f, "abandoned"),
        }
    }
}

impl ChangeStatus {
    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "new" => ChangeStatus::New,
            "draft" => ChangeStatus::Draft,
            "merged" => ChangeStatus::Merged,
            "abandoned" => ChangeStatus::Abandoned,
            _ => ChangeStatus::New, // Default fallback
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ImportStatus {
    Pending,
    Importing,
    Imported,
    Failed,
    Outdated,
}

impl std::fmt::Display for ImportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportStatus::Pending => write!(f, "pending"),
            ImportStatus::Importing => write!(f, "importing"),
            ImportStatus::Imported => write!(f, "imported"),
            ImportStatus::Failed => write!(f, "failed"),
            ImportStatus::Outdated => write!(f, "outdated"),
        }
    }
}

impl ImportStatus {
    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "pending" => ImportStatus::Pending,
            "importing" => ImportStatus::Importing,
            "imported" => ImportStatus::Imported,
            "failed" => ImportStatus::Failed,
            "outdated" => ImportStatus::Outdated,
            _ => ImportStatus::Pending, // Default fallback
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ConflictStatus {
    None,
    CommentsPending,
    PatchSetUpdated,
    ManualResolutionRequired,
}

impl std::fmt::Display for ConflictStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConflictStatus::None => write!(f, "none"),
            ConflictStatus::CommentsPending => write!(f, "comments_pending"),
            ConflictStatus::PatchSetUpdated => write!(f, "patch_set_updated"),
            ConflictStatus::ManualResolutionRequired => write!(f, "manual_resolution_required"),
        }
    }
}

impl ConflictStatus {
    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "none" => ConflictStatus::None,
            "comments_pending" => ConflictStatus::CommentsPending,
            "patch_set_updated" => ConflictStatus::PatchSetUpdated,
            "manual_resolution_required" => ConflictStatus::ManualResolutionRequired,
            _ => ConflictStatus::None, // Default fallback
        }
    }
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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

// Note: ReviewStatus is defined later in the file for review sessions

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NotifyHandling {
    None,
    Owner,
    OwnerReviewers,
    All,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum FileChangeType {
    Added,
    Modified,
    Deleted,
    Renamed,
    Copied,
    Rewritten,
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

impl std::fmt::Display for FileChangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileChangeType::Added => write!(f, "added"),
            FileChangeType::Modified => write!(f, "modified"),
            FileChangeType::Deleted => write!(f, "deleted"),
            FileChangeType::Renamed => write!(f, "renamed"),
            FileChangeType::Copied => write!(f, "copied"),
            FileChangeType::Rewritten => write!(f, "rewritten"),
        }
    }
}

impl FileChangeType {
    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "added" => FileChangeType::Added,
            "modified" => FileChangeType::Modified,
            "deleted" => FileChangeType::Deleted,
            "renamed" => FileChangeType::Renamed,
            "copied" => FileChangeType::Copied,
            "rewritten" => FileChangeType::Rewritten,
            _ => FileChangeType::Modified,
        }
    }
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

// Note: ReviewProgress is defined later in the file for review sessions

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

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
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



/// Review Session for managing review workflows
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReviewSession {
    pub id: String,
    pub change_id: String,
    pub patch_set_number: u32,
    pub reviewer_id: String,
    pub mode: ReviewMode,
    pub status: ReviewStatus,
    pub progress: ReviewProgress,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ReviewMode {
    Online,
    Offline,
    Hybrid, // Can switch between online/offline
}

impl std::fmt::Display for ReviewMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReviewMode::Online => write!(f, "online"),
            ReviewMode::Offline => write!(f, "offline"),
            ReviewMode::Hybrid => write!(f, "hybrid"),
        }
    }
}

impl ReviewMode {
    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "online" => ReviewMode::Online,
            "offline" => ReviewMode::Offline,
            "hybrid" => ReviewMode::Hybrid,
            _ => ReviewMode::Online,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ReviewStatus {
    InProgress,
    ReadyForSubmission,
    Submitted,
    Abandoned,
}

impl std::fmt::Display for ReviewStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReviewStatus::InProgress => write!(f, "in_progress"),
            ReviewStatus::ReadyForSubmission => write!(f, "ready_for_submission"),
            ReviewStatus::Submitted => write!(f, "submitted"),
            ReviewStatus::Abandoned => write!(f, "abandoned"),
        }
    }
}

impl ReviewStatus {
    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "in_progress" => ReviewStatus::InProgress,
            "ready_for_submission" => ReviewStatus::ReadyForSubmission,
            "submitted" => ReviewStatus::Submitted,
            "abandoned" => ReviewStatus::Abandoned,
            _ => ReviewStatus::InProgress,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ReviewProgress {
    pub total_files: u32,
    pub reviewed_files: u32,
    pub files_with_comments: u32,
    pub pending_files: Vec<String>,
}

/// Change File for offline review
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChangeFile {
    pub id: String,
    pub change_id: String,
    pub patch_set_number: u32,
    pub file_path: String,
    pub change_type: FileChangeType,
    pub old_content: Option<String>,
    pub new_content: Option<String>,
    pub diff: FileDiff,
    pub file_size: u64,
    pub downloaded_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FileDiff {
    pub unified_diff: String,
    pub old_line_count: u32,
    pub new_line_count: u32,
    pub context_lines: u32,
    pub hunks: Vec<DiffHunk>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiffHunk {
    pub old_start: u32,
    pub old_count: u32,
    pub new_start: u32,
    pub new_count: u32,
    pub lines: Vec<DiffLine>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiffLine {
    pub line_type: DiffLineType,
    pub old_line_number: Option<u32>,
    pub new_line_number: Option<u32>,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum DiffLineType {
    Context,
    Added,
    Removed,
    NoNewlineAtEof,
}

/// File Review for tracking review progress per file
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileReview {
    pub id: String,
    pub session_id: String,
    pub file_path: String,
    pub review_status: FileReviewStatus,
    pub last_reviewed: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum FileReviewStatus {
    Pending,
    InProgress,
    Reviewed,
    HasComments,
    Approved,
    NeedsWork,
}

impl std::fmt::Display for FileReviewStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileReviewStatus::Pending => write!(f, "pending"),
            FileReviewStatus::InProgress => write!(f, "in_progress"),
            FileReviewStatus::Reviewed => write!(f, "reviewed"),
            FileReviewStatus::HasComments => write!(f, "has_comments"),
            FileReviewStatus::Approved => write!(f, "approved"),
            FileReviewStatus::NeedsWork => write!(f, "needs_work"),
        }
    }
}

impl FileReviewStatus {
    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "pending" => FileReviewStatus::Pending,
            "in_progress" => FileReviewStatus::InProgress,
            "reviewed" => FileReviewStatus::Reviewed,
            "has_comments" => FileReviewStatus::HasComments,
            "approved" => FileReviewStatus::Approved,
            "needs_work" => FileReviewStatus::NeedsWork,
            _ => FileReviewStatus::Pending,
        }
    }
}

/// Review Comment for inline and file-level comments
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReviewComment {
    pub id: String,
    pub session_id: String,
    pub file_path: String,
    pub line_number: Option<u32>,
    pub content: String,
    pub comment_type: CommentType,
    pub status: CommentStatus,
    pub parent_comment_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CommentType {
    Inline,
    FileLevel,
    General,
    Suggestion,
    Question,
    Issue,
}

impl std::fmt::Display for CommentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommentType::Inline => write!(f, "inline"),
            CommentType::FileLevel => write!(f, "file_level"),
            CommentType::General => write!(f, "general"),
            CommentType::Suggestion => write!(f, "suggestion"),
            CommentType::Question => write!(f, "question"),
            CommentType::Issue => write!(f, "issue"),
        }
    }
}

impl CommentType {
    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "inline" => CommentType::Inline,
            "file_level" => CommentType::FileLevel,
            "general" => CommentType::General,
            "suggestion" => CommentType::Suggestion,
            "question" => CommentType::Question,
            "issue" => CommentType::Issue,
            _ => CommentType::Inline,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CommentStatus {
    Draft,
    Published,
    Resolved,
    Acknowledged,
}

impl std::fmt::Display for CommentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommentStatus::Draft => write!(f, "draft"),
            CommentStatus::Published => write!(f, "published"),
            CommentStatus::Resolved => write!(f, "resolved"),
            CommentStatus::Acknowledged => write!(f, "acknowledged"),
        }
    }
}

impl CommentStatus {
    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "draft" => CommentStatus::Draft,
            "published" => CommentStatus::Published,
            "resolved" => CommentStatus::Resolved,
            "acknowledged" => CommentStatus::Acknowledged,
            _ => CommentStatus::Draft,
        }
    }
}

/// Review Template for reusable review patterns
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReviewTemplate {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub file_patterns: Vec<String>, // File patterns this template applies to
    pub template_content: String,
    pub category: Option<String>,
    pub usage_count: u32,
    pub created_at: String,
    pub updated_at: String,
}

/// Download Result for change download operations
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadResult {
    pub success: bool,
    pub change_metadata: Option<GerritChange>,
    pub files: Vec<ChangeFile>,
    pub total_size: u64,
    pub download_time_ms: u64,
    pub error_message: Option<String>,
}

/// Review Submission for submitting reviews to Gerrit
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReviewSubmission {
    pub change_id: String,
    pub patch_set_number: u32,
    pub overall_score: i32,
    pub verified_score: Option<i32>,
    pub message: String,
    pub comments: Vec<ReviewComment>,
    pub labels: HashMap<String, i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubmissionResult {
    pub success: bool,
    pub gerrit_response: Option<String>,
    pub error: Option<String>,
    pub submitted_comments: u32,
    pub failed_comments: Vec<String>,
}


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

    #[test]
    fn test_review_session_creation() {
        let session = ReviewSession {
            id: "test-session".to_string(),
            change_id: "I12345".to_string(),
            patch_set_number: 1,
            reviewer_id: "reviewer1".to_string(),
            mode: ReviewMode::Offline,
            status: ReviewStatus::InProgress,
            progress: ReviewProgress {
                total_files: 10,
                reviewed_files: 3,
                files_with_comments: 1,
                pending_files: vec!["file1.rs".to_string(), "file2.rs".to_string()],
            },
            created_at: "2025-01-05 12:00:00".to_string(),
            updated_at: "2025-01-05 12:00:00".to_string(),
        };

        assert_eq!(session.mode, ReviewMode::Offline);
        assert_eq!(session.status, ReviewStatus::InProgress);
        assert_eq!(session.progress.total_files, 10);
        assert_eq!(session.progress.reviewed_files, 3);
    }

    #[test]
    fn test_change_file_creation() {
        let file = ChangeFile {
            id: "file-1".to_string(),
            change_id: "I12345".to_string(),
            patch_set_number: 1,
            file_path: "src/main.rs".to_string(),
            change_type: FileChangeType::Modified,
            old_content: Some("old content".to_string()),
            new_content: Some("new content".to_string()),
            diff: FileDiff::default(),
            file_size: 1024,
            downloaded_at: "2025-01-05 12:00:00".to_string(),
        };

        assert_eq!(file.change_type, FileChangeType::Modified);
        assert_eq!(file.file_path, "src/main.rs");
        assert_eq!(file.file_size, 1024);
    }

    #[test]
    fn test_review_comment_creation() {
        let comment = ReviewComment {
            id: "comment-1".to_string(),
            session_id: "session-1".to_string(),
            file_path: "src/main.rs".to_string(),
            line_number: Some(42),
            content: "This needs improvement".to_string(),
            comment_type: CommentType::Inline,
            status: CommentStatus::Draft,
            parent_comment_id: None,
            created_at: "2025-01-05 12:00:00".to_string(),
            updated_at: "2025-01-05 12:00:00".to_string(),
        };

        assert_eq!(comment.comment_type, CommentType::Inline);
        assert_eq!(comment.status, CommentStatus::Draft);
        assert_eq!(comment.line_number, Some(42));
    }

    #[test]
    fn test_file_change_type_display() {
        assert_eq!(FileChangeType::Added.to_string(), "added");
        assert_eq!(FileChangeType::Modified.to_string(), "modified");
        assert_eq!(FileChangeType::Deleted.to_string(), "deleted");
        assert_eq!(FileChangeType::Renamed.to_string(), "renamed");
        assert_eq!(FileChangeType::Copied.to_string(), "copied");
        assert_eq!(FileChangeType::Rewritten.to_string(), "rewritten");
    }

    #[test]
    fn test_file_change_type_from_string() {
        assert_eq!(FileChangeType::from_string("added"), FileChangeType::Added);
        assert_eq!(FileChangeType::from_string("modified"), FileChangeType::Modified);
        assert_eq!(FileChangeType::from_string("deleted"), FileChangeType::Deleted);
        assert_eq!(FileChangeType::from_string("renamed"), FileChangeType::Renamed);
        assert_eq!(FileChangeType::from_string("copied"), FileChangeType::Copied);
        assert_eq!(FileChangeType::from_string("rewritten"), FileChangeType::Rewritten);
        assert_eq!(FileChangeType::from_string("unknown"), FileChangeType::Modified); // Default fallback
    }

    #[test]
    fn test_review_mode_conversion() {
        assert_eq!(ReviewMode::from_string("online"), ReviewMode::Online);
        assert_eq!(ReviewMode::from_string("offline"), ReviewMode::Offline);
        assert_eq!(ReviewMode::from_string("hybrid"), ReviewMode::Hybrid);
        assert_eq!(ReviewMode::from_string("unknown"), ReviewMode::Online); // Default fallback

        assert_eq!(ReviewMode::Online.to_string(), "online");
        assert_eq!(ReviewMode::Offline.to_string(), "offline");
        assert_eq!(ReviewMode::Hybrid.to_string(), "hybrid");
    }

    #[test]
    fn test_review_status_conversion() {
        assert_eq!(ReviewStatus::from_string("in_progress"), ReviewStatus::InProgress);
        assert_eq!(ReviewStatus::from_string("ready_for_submission"), ReviewStatus::ReadyForSubmission);
        assert_eq!(ReviewStatus::from_string("submitted"), ReviewStatus::Submitted);
        assert_eq!(ReviewStatus::from_string("abandoned"), ReviewStatus::Abandoned);
        assert_eq!(ReviewStatus::from_string("unknown"), ReviewStatus::InProgress); // Default fallback

        assert_eq!(ReviewStatus::InProgress.to_string(), "in_progress");
        assert_eq!(ReviewStatus::ReadyForSubmission.to_string(), "ready_for_submission");
        assert_eq!(ReviewStatus::Submitted.to_string(), "submitted");
        assert_eq!(ReviewStatus::Abandoned.to_string(), "abandoned");
    }
}