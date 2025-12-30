# Gerrit Integration API Contracts

## Tauri Command Contracts (Rust Backend)

### 1. Instance Management Commands

#### `gerrit_get_instances`
**Description**: List all configured Gerrit instances with connection status.

**Request:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct GetInstancesRequest {
    pub include_inactive: bool,  // Include inactive instances
}
```

**Response:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct GerritInstanceInfo {
    pub id: String,
    pub name: String,
    pub url: String,
    pub version: Option<String>,
    pub is_active: bool,
    pub connection_status: ConnectionStatus,
    pub last_connected: Option<String>,  // ISO 8601
    pub created_at: String,  // ISO 8601
    pub updated_at: String,  // ISO 8601
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetInstancesResponse {
    pub instances: Vec<GerritInstanceInfo>,
    pub total_count: u32,
    pub active_instance: Option<String>,  // ID of active instance
}
```

**Error Codes:**
- `INSTANCE_LIST_FAILED`: Failed to retrieve instances
- `DATABASE_ERROR`: Database operation failed

---

#### `gerrit_create_instance`
**Description**: Create a new Gerrit instance configuration.

**Request:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInstanceRequest {
    pub name: String,  // Display name (1-100 chars)
    pub url: String,   // HTTPS URL only
    pub username: String,  // Gerrit username
    pub password: String,  // HTTP password token
    pub make_active: bool,  // Set as active instance immediately
}
```

**Response:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInstanceResponse {
    pub instance_id: String,
    pub connection_test: ConnectionTestResult,
    pub created_at: String,  // ISO 8601
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionTestResult {
    pub success: bool,
    pub gerrit_version: Option<String>,
    pub error_message: Option<String>,
    pub supported_features: Vec<String>,
}
```

**Error Codes:**
- `INVALID_INSTANCE_NAME`: Name validation failed
- `INVALID_URL`: URL format or protocol invalid
- `INVALID_CREDENTIALS`: Credentials validation failed
- `CONNECTION_TEST_FAILED`: Unable to connect to Gerrit
- `DUPLICATE_INSTANCE_NAME`: Name already exists
- `VERSION_INCOMPATIBLE`: Gerrit version too old (<3.6)

---

#### `gerrit_test_connection`
**Description**: Test connectivity and authentication for a Gerrit instance.

**Request:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct TestConnectionRequest {
    pub instance_id: String,
}
```

**Response:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct TestConnectionResponse {
    pub instance_id: String,
    pub success: bool,
    pub gerrit_version: String,
    pub supported_features: Vec<String>,
    pub connection_time_ms: u32,
    pub error_details: Option<ConnectionErrorDetails>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionErrorDetails {
    pub error_type: String,  // Authentication, Network, Version, etc.
    pub error_message: String,
    pub suggested_action: Option<String>,
}
```

**Error Codes:**
- `INSTANCE_NOT_FOUND`: Instance ID does not exist
- `CONNECTION_TIMEOUT`: Connection attempt timed out
- `AUTHENTICATION_FAILED`: Invalid credentials
- `NETWORK_ERROR`: Network connectivity issue
- `VERSION_CHECK_FAILED`: Unable to determine Gerrit version

---

### 2. Change Import Commands

#### `gerrit_get_change`
**Description**: Import a specific Gerrit change by ID with complete metadata.

**Request:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct GetChangeRequest {
    pub instance_id: String,
    pub change_id: String,  // Gerrit Change-ID (e.g., "12345")
    pub include_diffs: bool,  // Include file diffs (performance impact)
    pub include_comments: bool,  // Include existing comments
}
```

**Response:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct GetChangeResponse {
    pub change: GerritChangeDetail,
    pub import_status: ImportStatus,
    pub estimated_import_time_ms: u32,
    pub files_processed: u32,
    pub total_files: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GerritChangeDetail {
    pub id: String,  // Internal UUID
    pub change_id: String,  // Gerrit Change-ID
    pub project: String,
    pub branch: String,
    pub subject: String,
    pub status: String,
    pub owner: GerritUser,
    pub created: String,  // ISO 8601
    pub updated: String,  // ISO 8601
    pub current_revision: String,
    pub current_patch_set: PatchSetInfo,
    pub patch_sets: Vec<PatchSetInfo>,
    pub files: Vec<FileInfo>,
    pub total_files: u32,
    pub local_comments: u32,
    pub remote_comments: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatchSetInfo {
    pub id: String,
    pub number: u32,
    pub revision: String,
    pub author: GerritUser,
    pub commit_message: String,
    pub created: String,  // ISO 8601
    pub is_current: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub id: String,
    pub path: String,
    pub change_type: String,  // Added, Modified, Deleted, Renamed
    pub lines_inserted: u32,
    pub lines_deleted: u32,
    pub is_binary: bool,
    pub review_status: String,  // Unreviewed, Pending, Reviewed
}
```

**Error Codes:**
- `CHANGE_NOT_FOUND`: Change ID does not exist in Gerrit
- `INSTANCE_NOT_FOUND`: Instance ID does not exist
- `PERMISSION_DENIED`: User lacks permission to access change
- `IMPORT_FAILED`: Failed to import change data
- `FILE_PROCESSING_ERROR`: Error processing file diffs

---

#### `gerrit_search_changes`
**Description**: Search for Gerrit changes using query syntax and import results.

**Request:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchChangesRequest {
    pub instance_id: String,
    pub query: String,  // Gerrit query syntax (e.g., "status:open project:payment")
    pub limit: Option<u32>,  // Max results (default: 50, max: 500)
    pub include_diffs: bool,
    pub include_comments: bool,
    pub start: Option<u32>,  // Pagination offset
}
```

**Response:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchChangesResponse {
    pub changes: Vec<ChangeSummary>,
    pub total_count: u32,
    pub has_more: bool,
    pub next_offset: Option<u32>,
    pub search_time_ms: u32,
    pub import_status: Vec<ImportStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeSummary {
    pub change_id: String,
    pub project: String,
    pub branch: String,
    pub subject: String,
    pub status: String,
    pub owner: GerritUser,
    pub created: String,  // ISO 8601
    pub updated: String,  // ISO 8601
    pub current_patch_set: u32,
    pub total_files: u32,
    pub insertions: u32,
    pub deletions: u32,
    pub is_imported: bool,
    pub local_progress: Option<f64>,  // 0.0-1.0 if imported
}
```

**Error Codes:**
- `INVALID_QUERY`: Malformed Gerrit search query
- `SEARCH_FAILED`: Search execution failed
- `RESULT_LIMIT_EXCEEDED`: Too many results requested
- `INSTANCE_NOT_FOUND`: Instance ID does not exist

---

### 3. Diff and File Commands

#### `gerrit_get_diff`
**Description**: Get diff content for a specific file in a change.

**Request:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct GetDiffRequest {
    pub instance_id: String,
    pub change_id: String,
    pub file_path: String,
    pub patch_set_number: Option<u32>,  // Defaults to current
    // Pagination for large diffs
    pub start_line: Option<u32>,  // 1-based
    pub end_line: Option<u32>,   // 1-based
    pub context_lines: Option<u32>,  // Lines of context (default: 3)
}
```

**Response:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct GetDiffResponse {
    pub file_path: String,
    pub change_type: String,  // Added, Modified, Deleted, Renamed
    pub old_path: Option<String>,
    pub is_binary: bool,
    pub total_lines: u32,
    pub diff_chunks: Vec<DiffChunk>,
    pub has_more: bool,
    pub next_start_line: Option<u32>,
    pub load_time_ms: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffChunk {
    pub chunk_type: String,  // Context, Addition, Deletion
    pub old_start: u32,
    pub old_lines: u32,
    pub new_start: u32,
    pub new_lines: u32,
    pub lines: Vec<DiffLine>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffLine {
    pub line_type: String,  // Context, Addition, Deletion
    pub old_line_number: Option<u32>,
    pub new_line_number: Option<u32>,
    pub content: String,
    pub highlight_ranges: Vec<HighlightRange>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HighlightRange {
    pub start: u32,
    pub length: u32,
    pub highlight_type: String,  // Syntax, Search, Comment
}
```

**Error Codes:**
- `FILE_NOT_FOUND`: File path does not exist in change
- `DIFF_UNAVAILABLE`: Diff content not available
- `BINARY_FILE`: File is binary, no text diff
- `LARGE_FILE`: File too large for requested range
- `PATCH_SET_NOT_FOUND`: Specified patch set does not exist

---

### 4. Comment Management Commands

#### `gerrit_get_comments`
**Description**: Get all comments for a specific change or file.

**Request:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct GetCommentsRequest {
    pub change_id: String,
    pub file_path: Option<String>,  // Optional: filter by file
    pub patch_set_number: Option<u32>,  // Optional: filter by patch set
    pub include_local: bool,  // Include locally created comments
    pub include_remote: bool,  // Include comments from Gerrit
    pub status_filter: Option<Vec<String>>,  // Filter by sync status
}
```

**Response:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct GetCommentsResponse {
    pub comments: Vec<CommentInfo>,
    pub total_count: u32,
    pub local_count: u32,
    pub remote_count: u32,
    pub unresolved_count: u32,
    pub sync_status_summary: SyncStatusSummary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentInfo {
    pub id: String,  // Local UUID
    pub gerrit_comment_id: Option<String>,
    pub file_path: String,
    pub patch_set_number: u32,
    pub line: u32,
    pub range: Option<CommentRange>,
    pub message: String,
    pub author: GerritUser,
    pub created: String,  // ISO 8601
    pub updated: String,  // ISO 8601
    pub status: String,  // LocalOnly, SyncPending, Synced, etc.
    pub unresolved: bool,
    pub severity: Option<String>,  // Info, Warning, Error
    pub parent_id: Option<String>,  // For comment threads
    pub replies: Vec<CommentInfo>,  // Nested replies
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncStatusSummary {
    pub pending_sync: u32,
    pub synced: u32,
    pub failed: u32,
    pub conflicts: u32,
    pub last_sync_time: Option<String>,  // ISO 8601
}
```

**Error Codes:**
- `CHANGE_NOT_FOUND`: Change ID does not exist
- `FILE_NOT_FOUND`: Specified file path not found
- `COMMENTS_UNAVAILABLE`: Unable to retrieve comments

---

#### `gerrit_create_comment`
**Description**: Create a new local comment for review.

**Request:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCommentRequest {
    pub change_id: String,
    pub file_path: String,
    pub patch_set_number: u32,
    pub line: u32,
    pub range: Option<CommentRange>,  // Character range for inline comments
    pub message: String,  // 1-10000 characters
    pub severity: Option<String>,  // Info, Warning, Error
    pub parent_comment_id: Option<String>,  // For replies
}
```

**Response:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCommentResponse {
    pub comment_id: String,  // Local UUID
    pub status: String,  // LocalOnly
    pub created_at: String,  // ISO 8601
    pub estimated_sync_time: Option<String>,  // When sync expected
}
```

**Error Codes:**
- `INVALID_COMMENT_MESSAGE`: Message validation failed
- `FILE_NOT_FOUND`: File path does not exist
- `LINE_NUMBER_INVALID`: Line number out of range
- `PARENT_COMMENT_NOT_FOUND`: Reply-to comment not found
- `CHANGE_NOT_IMPORTED`: Change must be imported first

---

#### `gerrit_update_comment`
**Description**: Update an existing local comment.

**Request:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCommentRequest {
    pub comment_id: String,
    pub message: String,  // Updated message
    pub severity: Option<String>,  // Updated severity
    pub mark_resolved: Option<bool>,  // Update resolved status
}
```

**Response:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCommentResponse {
    pub comment_id: String,
    pub status: String,  // Updated status
    pub updated_at: String,  // ISO 8601
    pub sync_required: bool,  // Whether sync needed
}
```

**Error Codes:**
- `COMMENT_NOT_FOUND`: Comment ID does not exist
- `COMMENT_ALREADY_SYNCED`: Cannot edit synced comments
- `INVALID_COMMENT_UPDATE`: Update validation failed

---

#### `gerrit_delete_comment`
**Description**: Delete a local comment (before sync).

**Request:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteCommentRequest {
    pub comment_id: String,
    pub force: bool,  // Allow deletion of synced comments
}
```

**Response:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteCommentResponse {
    pub comment_id: String,
    pub deleted: bool,
    pub was_synced: bool,
}
```

**Error Codes:**
- `COMMENT_NOT_FOUND`: Comment ID does not exist
- `COMMENT_ALREADY_SYNCED`: Cannot delete synced comments without force
- `COMMENT_HAS_REPLIES`: Cannot delete comment with replies

---

### 5. Review Submission Commands

#### `gerrit_submit_review`
**Description**: Submit a complete review with scores and comments.

**Request:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitReviewRequest {
    pub change_id: String,
    pub patch_set_number: u32,
    pub message: String,  // Review message (optional)
    pub labels: HashMap<String, i32>,  // e.g., {"Code-Review": 2, "Verified": 1}
    pub comment_ids: Vec<String>,  // Comments to include in review
    pub notify: String,  // None, Owner, OwnerReviewers, All
    pub draft: bool,  // Submit as draft review
}
```

**Response:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitReviewResponse {
    pub review_id: String,  // Local review UUID
    pub gerrit_review_id: Option<String>,  // Gerrit review ID after sync
    pub status: String,  // PendingSubmission, Submitted, etc.
    pub submitted_comments: u32,
    pub submitted_labels: HashMap<String, i32>,
    pub estimated_completion_time: Option<String>,  // ISO 8601
}
```

**Error Codes:**
- `INVALID_REVIEW_MESSAGE`: Message validation failed
- `INVALID_LABEL_VALUES`: Label values out of valid range
- `NO_COMMENTS_TO_SUBMIT`: No comments selected for submission
- `CHANGE_NOT_READY`: Change not in valid state for review
- `PERMISSION_DENIED`: User lacks review permissions

---

### 6. Synchronization Commands

#### `gerrit_sync_changes`
**Description**: Synchronize local changes with Gerrit server.

**Request:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncChangesRequest {
    pub instance_id: String,
    pub change_ids: Vec<String>,  // Specific changes to sync (empty = all)
    pub sync_type: String,  // Full, Incremental, CommentsOnly, PushLocal
    pub force: bool,  // Force sync even if no changes detected
    pub conflict_resolution: String,  // Auto, Prompt, LocalWins, RemoteWins
}
```

**Response:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncChangesResponse {
    pub sync_id: String,  // Unique sync operation ID
    pub total_changes: u32,
    pub synced_changes: u32,
    pub failed_changes: u32,
    pub conflicts_detected: u32,
    pub conflicts_resolved: u32,
    pub sync_status: String,  // InProgress, Completed, Failed
    pub errors: Vec<SyncError>,
    pub start_time: String,  // ISO 8601
    pub end_time: Option<String>,  // ISO 8601
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncError {
    pub change_id: String,
    pub error_type: String,  // Conflict, Network, Permission, etc.
    pub error_message: String,
    pub suggested_action: Option<String>,
}
```

**Error Codes:**
- `SYNC_ALREADY_IN_PROGRESS`: Another sync operation running
- `INSTANCE_NOT_FOUND`: Instance ID does not exist
- `SYNC_FAILED`: General sync failure
- `CONFLICT_RESOLUTION_FAILED`: Unable to resolve conflicts
- `NETWORK_ERROR`: Connectivity issues during sync

---

#### `gerrit_get_sync_status`
**Description**: Get current synchronization status and queue information.

**Request:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct GetSyncStatusRequest {
    pub instance_id: String,
    pub change_id: Option<String>,  // Optional: filter by specific change
}
```

**Response:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct GetSyncStatusResponse {
    pub instance_status: InstanceSyncStatus,
    pub pending_operations: Vec<PendingOperation>,
    pub sync_statistics: SyncStatistics,
    pub last_sync_summary: Option<SyncSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceSyncStatus {
    pub instance_id: String,
    pub connection_status: String,
    pub last_successful_sync: Option<String>,  // ISO 8601
    pub next_scheduled_sync: Option<String>,   // ISO 8601
    pub is_syncing: bool,
    pub sync_progress: Option<f64>,  // 0.0-1.0
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PendingOperation {
    pub operation_id: String,
    pub operation_type: String,
    pub change_id: String,
    pub priority: String,
    pub status: String,
    pub retry_count: u32,
    pub created_at: String,  // ISO 8601
    pub estimated_completion: Option<String>,  // ISO 8601
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncStatistics {
    pub total_changes: u32,
    pub synced_changes: u32,
    pub pending_changes: u32,
    pub conflicted_changes: u32,
    pub failed_changes: u32,
    pub total_comments: u32,
    pub pending_comments: u32,
    pub total_reviews: u32,
    pub pending_reviews: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncSummary {
    pub sync_id: String,
    pub changes_processed: u32,
    pub comments_processed: u32,
    pub conflicts_resolved: u32,
    pub errors_encountered: u32,
    pub duration_ms: u32,
    pub completion_status: String,  // Success, Partial, Failed
}
```

**Error Codes:**
- `STATUS_UNAVAILABLE`: Unable to retrieve sync status
- `INSTANCE_NOT_FOUND`: Instance ID does not exist

## Frontend Service Contracts (TypeScript)

### 1. Instance Management Service

```typescript
interface GerritInstanceService {
  // Instance CRUD operations
  getInstances(includeInactive?: boolean): Promise<GerritInstanceInfo[]>;
  createInstance(config: CreateInstanceConfig): Promise<GerritInstanceInfo>;
  updateInstance(id: string, updates: Partial<InstanceUpdate>): Promise<GerritInstanceInfo>;
  deleteInstance(id: string): Promise<void>;
  setActiveInstance(id: string): Promise<void>;
  
  // Connection management
  testConnection(id: string): Promise<ConnectionTestResult>;
  getConnectionStatus(id: string): Promise<ConnectionStatus>;
  
  // Configuration
  getInstanceConfig(id: string): Promise<InstanceConfiguration>;
  validateInstanceConfig(config: InstanceConfiguration): Promise<ValidationResult>;
}

interface CreateInstanceConfig {
  name: string;  // 1-100 characters
  url: string;   // HTTPS URL
  username: string;
  password: string;  // Will be encrypted
  makeActive?: boolean;
}

interface GerritInstanceInfo {
  id: string;
  name: string;
  url: string;
  version?: string;
  isActive: boolean;
  connectionStatus: ConnectionStatus;
  lastConnected?: Date;
  createdAt: Date;
  updatedAt: Date;
}

type ConnectionStatus = 
  | 'connected'
  | 'disconnected' 
  | 'authentication_failed'
  | 'version_incompatible'
  | 'network_error';
```

---

### 2. Change Import Service

```typescript
interface GerritChangeService {
  // Change import operations
  importChange(changeId: string, options?: ImportOptions): Promise<GerritChange>;
  searchChanges(query: string, options?: SearchOptions): Promise<SearchResult>;
  getImportedChanges(filters?: ChangeFilters): Promise<GerritChange[]>;
  
  // Change management
  getChange(id: string): Promise<GerritChange>;
  removeChange(id: string): Promise<void>;
  refreshChange(id: string): Promise<GerritChange>;
  
  // Batch operations
  importMultiple(changeIds: string[], options?: ImportOptions): Promise<BatchImportResult>;
  removeMultiple(changeIds: string[]): Promise<void>;
}

interface ImportOptions {
  includeDiffs?: boolean;
  includeComments?: boolean;
  forceRefresh?: boolean;
}

interface SearchOptions {
  limit?: number;  // 1-500
  includeDiffs?: boolean;
  includeComments?: boolean;
  start?: number;  // Pagination offset
}

interface ChangeFilters {
  instanceId?: string;
  status?: ChangeStatus[];
  project?: string;
  branch?: string;
  importStatus?: ImportStatus;
  hasLocalComments?: boolean;
  searchText?: string;
}

interface GerritChange {
  id: string;
  changeId: string;  // Gerrit Change-ID
  instanceId: string;
  project: string;
  branch: string;
  subject: string;
  status: ChangeStatus;
  owner: GerritUser;
  created: Date;
  updated: Date;
  currentRevision: string;
  currentPatchSet: PatchSet;
  totalFiles: number;
  reviewedFiles: number;
  localComments: number;
  remoteComments: number;
  importStatus: ImportStatus;
  conflictStatus: ConflictStatus;
}
```

---

### 3. Comment Management Service

```typescript
interface GerritCommentService {
  // Comment CRUD operations
  getComments(changeId: string, options?: CommentOptions): Promise<CommentInfo[]>;
  createComment(comment: CreateCommentData): Promise<CommentInfo>;
  updateComment(id: string, updates: CommentUpdate): Promise<CommentInfo>;
  deleteComment(id: string): Promise<void>;
  
  // Comment threading
  replyToComment(parentId: string, reply: CreateCommentData): Promise<CommentInfo>;
  getCommentThread(commentId: string): Promise<CommentInfo[]>;
  
  // Batch operations
  createMultiple(comments: CreateCommentData[]): Promise<CommentInfo[]>;
  updateMultiple(updates: CommentUpdate[]): Promise<CommentInfo[]>;
  
  // Sync operations
  syncComments(changeId: string): Promise<SyncResult>;
  getSyncStatus(changeId: string): Promise<CommentSyncStatus>;
}

interface CreateCommentData {
  changeId: string;
  filePath: string;
  patchSetNumber: number;
  line: number;
  range?: CommentRange;
  message: string;
  severity?: CommentSeverity;
  parentCommentId?: string;
}

interface CommentUpdate {
  id: string;
  message?: string;
  severity?: CommentSeverity;
  markResolved?: boolean;
}

interface CommentInfo {
  id: string;
  gerritCommentId?: string;
  changeId: string;
  filePath: string;
  patchSetNumber: number;
  line: number;
  range?: CommentRange;
  message: string;
  author: GerritUser;
  created: Date;
  updated: Date;
  status: CommentSyncStatus;
  unresolved: boolean;
  severity?: CommentSeverity;
  parentId?: string;
  replies: CommentInfo[];
}
```

---

### 4. Review Submission Service

```typescript
interface GerritReviewService {
  // Review submission
  submitReview(review: SubmitReviewData): Promise<ReviewSubmissionResult>;
  
  // Review preparation
  prepareReview(changeId: string, options?: ReviewOptions): Promise<ReviewDraft>;
  saveReviewDraft(draft: ReviewDraft): Promise<ReviewDraft>;
  getReviewDraft(changeId: string): Promise<ReviewDraft | null>;
  
  // Batch operations
  submitMultiple(reviews: SubmitReviewData[]): Promise<BatchSubmissionResult>;
  
  // Review history
  getReviewHistory(changeId: string): Promise<ReviewHistory[]>;
}

interface SubmitReviewData {
  changeId: string;
  patchSetNumber: number;
  message?: string;
  labels: Record<string, number>;  // e.g., { 'Code-Review': 2 }
  commentIds: string[];
  notify?: NotifyHandling;
  draft?: boolean;
}

interface ReviewOptions {
  autoIncludeComments?: boolean;
  suggestedLabels?: Record<string, number>;
  template?: string;
}

interface ReviewDraft {
  id: string;
  changeId: string;
  patchSetNumber: number;
  message: string;
  labels: Record<string, number>;
  commentIds: string[];
  notify: NotifyHandling;
  draft: boolean;
  createdAt: Date;
  updatedAt: Date;
}

interface ReviewSubmissionResult {
  reviewId: string;
  gerritReviewId?: string;
  status: ReviewStatus;
  submittedComments: number;
  submittedLabels: Record<string, number>;
  estimatedCompletionTime?: Date;
}
```

---

### 5. Sync Management Service

```typescript
interface GerritSyncService {
  // Sync operations
  syncChanges(options: SyncOptions): Promise<SyncResult>;
  syncChange(changeId: string, options?: ChangeSyncOptions): Promise<ChangeSyncResult>;
  
  // Sync status
  getSyncStatus(instanceId?: string): Promise<SyncStatus>;
  getSyncHistory(options?: SyncHistoryOptions): Promise<SyncHistory[]>;
  
  // Sync configuration
  configureSync(config: SyncConfiguration): Promise<void>;
  getSyncConfiguration(): Promise<SyncConfiguration>;
  
  // Conflict resolution
  resolveConflict(conflictId: string, resolution: ConflictResolution): Promise<void>;
  getConflicts(changeId?: string): Promise<SyncConflict[]>;
}

interface SyncOptions {
  instanceId?: string;
  changeIds?: string[];
  syncType?: SyncType;
  force?: boolean;
  conflictResolution?: ConflictResolutionStrategy;
}

interface SyncResult {
  syncId: string;
  totalChanges: number;
  syncedChanges: number;
  failedChanges: number;
  conflictsDetected: number;
  conflictsResolved: number;
  status: SyncStatus;
  errors: SyncError[];
  startTime: Date;
  endTime?: Date;
}

interface SyncConfiguration {
  autoSync: boolean;
  syncInterval: number;  // seconds
  conflictResolution: ConflictResolutionStrategy;
  maxRetries: number;
  batchSize: number;
}
```

## Common Types and Enumerations

```typescript
// Core types
interface GerritUser {
  accountId: string;
  name: string;
  email: string;
  username?: string;
  avatarUrl?: string;
}

interface CommentRange {
  startLine: number;
  startCharacter: number;
  endLine: number;
  endCharacter: number;
}

// Enumerations
type ChangeStatus = 'new' | 'draft' | 'merged' | 'abandoned';
type ImportStatus = 'pending' | 'importing' | 'imported' | 'failed' | 'outdated';
type ConflictStatus = 'none' | 'commentsPending' | 'patchSetUpdated' | 'manualResolutionRequired';
type CommentSyncStatus = 'localOnly' | 'syncPending' | 'synced' | 'syncFailed' | 'conflictDetected' | 'modifiedLocally';
type CommentSeverity = 'info' | 'warning' | 'error';
type ReviewStatus = 'draft' | 'pendingSubmission' | 'submitted' | 'submissionFailed' | 'partiallySubmitted';
type NotifyHandling = 'none' | 'owner' | 'ownerReviewers' | 'all';
type SyncType = 'full' | 'incremental' | 'commentsOnly' | 'statusOnly' | 'pushLocal';
type ConflictResolutionStrategy = 'auto' | 'prompt' | 'localWins' | 'remoteWins';
type SyncStatus = 'pending' | 'inProgress' | 'completed' | 'failed' | 'cancelled';

// Error handling
interface ApiError {
  code: string;
  message: string;
  details?: any;
  suggestedAction?: string;
}

interface ValidationResult {
  isValid: boolean;
  errors: ValidationError[];
}

interface ValidationError {
  field: string;
  message: string;
  code: string;
}
```

## Error Handling Standards

### Error Response Format
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,           // Machine-readable error code
    pub message: String,        // Human-readable error message
    pub details: Option<Value>, // Additional error details
    pub suggested_action: Option<String>, // Suggested remediation
    pub retry_possible: bool,   // Whether operation can be retried
}
```

### Error Code Categories
- **AUTHENTICATION**: Authentication and authorization errors
- **VALIDATION**: Input validation errors  
- **NETWORK**: Network connectivity errors
- **PERMISSION**: Access permission errors
- **NOT_FOUND**: Resource not found errors
- **CONFLICT**: Data conflict errors
- **PROCESSING**: General processing errors
- **RATE_LIMIT**: Rate limiting errors

### Common Error Codes
```rust
// Authentication errors
const AUTH_INVALID_CREDENTIALS: &str = "AUTH_INVALID_CREDENTIALS";
const AUTH_TOKEN_EXPIRED: &str = "AUTH_TOKEN_EXPIRED";
const AUTH_PERMISSION_DENIED: &str = "AUTH_PERMISSION_DENIED";

// Validation errors
const VALIDATION_INVALID_INPUT: &str = "VALIDATION_INVALID_INPUT";
const VALIDATION_MISSING_REQUIRED: &str = "VALIDATION_MISSING_REQUIRED";
const VALIDATION_INVALID_FORMAT: &str = "VALIDATION_INVALID_FORMAT";

// Network errors
const NETWORK_CONNECTION_FAILED: &str = "NETWORK_CONNECTION_FAILED";
const NETWORK_TIMEOUT: &str = "NETWORK_TIMEOUT";
const NETWORK_RATE_LIMITED: &str = "NETWORK_RATE_LIMITED";

// Resource errors
const RESOURCE_NOT_FOUND: &str = "RESOURCE_NOT_FOUND";
const RESOURCE_ALREADY_EXISTS: &str = "RESOURCE_ALREADY_EXISTS";
const RESOURCE_CONFLICT: &str = "RESOURCE_CONFLICT";

// Processing errors
const PROCESSING_FAILED: &str = "PROCESSING_FAILED";
const PROCESSING_INCOMPLETE: &str = "PROCESSING_INCOMPLETE";
const PROCESSING_RETRY_EXHAUSTED: &str = "PROCESSING_RETRY_EXHAUSTED";
```

This comprehensive API contract specification provides type-safe interfaces for all Gerrit integration operations with robust error handling and validation based on the functional requirements.