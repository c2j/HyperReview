# Gerrit Code Review Integration - API Contracts

This document defines comprehensive API contracts for the Gerrit Code Review integration feature, including Tauri commands, frontend services, data transfer objects, and API documentation.

## Table of Contents

1. [Tauri Command Contracts](#tauri-command-contracts)
2. [Frontend Service Contracts](#frontend-service-contracts)  
3. [Data Transfer Objects](#data-transfer-objects)
4. [API Documentation](#api-documentation)
5. [Error Handling](#error-handling)
6. [Validation Schemas](#validation-schemas)

---

## Tauri Command Contracts

### Instance Management Commands

#### `gerrit_get_instances`
**Purpose**: List all configured Gerrit instances

**Request Type**:
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct GetInstancesParams {
    pub include_inactive: bool,  // Include disconnected instances
}
```

**Response Type**:
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct GetInstancesResponse {
    pub success: bool,
    pub instances: Vec<GerritInstance>,
    pub total_count: u32,
    pub active_count: u32,
}
```

**Error Codes**:
- `GERRIT_INSTANCE_LIST_FAILED`: Unable to retrieve instances from database

---

#### `gerrit_create_instance`
**Purpose**: Configure a new Gerrit instance with encrypted credentials

**Request Type**:
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInstanceParams {
    pub name: String,                    // Display name (3-50 chars)
    pub url: String,                     // Base URL (must use HTTPS)
    pub username: String,                // Authentication username
    pub password: String,                // Password/token (will be encrypted)
    pub polling_interval: Option<u32>,   // Optional: seconds (default: 300)
    pub max_changes: Option<u32>,        // Optional: max imports (default: 100)
}
```

**Response Type**:
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInstanceResponse {
    pub success: bool,
    pub instance: GerritInstance,
    pub message: String,
    pub test_connection_result: Option<ConnectionTestResult>,
}
```

**Validation Rules**:
- `name`: 3-50 characters, alphanumeric + spaces + hyphens
- `url`: Valid HTTPS URL, must end with `/` or no trailing slash
- `username`: 1-100 characters, no whitespace
- `password`: 1-500 characters

**Error Codes**:
- `GERRIT_INVALID_INSTANCE_NAME`: Invalid instance name format
- `GERRIT_INVALID_URL`: Invalid or non-HTTPS URL
- `GERRIT_DUPLICATE_INSTANCE`: Instance with this name already exists
- `GERRIT_ENCRYPTION_FAILED`: Failed to encrypt credentials

---

#### `gerrit_test_connection`
**Purpose**: Validate connectivity and authentication to Gerrit instance

**Request Type**:
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct TestConnectionParams {
    pub instance_id: String,  // Instance UUID
}
```

**Response Type**:
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionTestResult {
    pub success: bool,
    pub status: ConnectionStatus,
    pub gerrit_version: String,
    pub server_time: String,          // ISO 8601 timestamp
    pub user_info: Option<GerritUser>, // Authenticated user details
    pub supported_features: Vec<String>,
    pub error_details: Option<String>,
}
```

**Error Codes**:
- `GERRIT_INSTANCE_NOT_FOUND`: Instance ID does not exist
- `GERRIT_CONNECTION_FAILED`: Network connection failed
- `GERRIT_AUTHENTICATION_FAILED`: Invalid credentials
- `GERRIT_VERSION_INCOMPATIBLE`: Gerrit version < 3.6

---

### Change Management Commands

#### `gerrit_get_change`
**Purpose**: Import a specific Gerrit change by ID with complete metadata

**Request Type**:
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct GetChangeParams {
    pub instance_id: String,      // Gerrit instance UUID
    pub change_id: String,        // Gerrit Change-ID (e.g., "12345" or "Iabc123...")
    pub include_comments: bool,   // Fetch existing comments
    pub include_files: bool,      // Fetch file list and diffs
    pub force_refresh: bool,      // Ignore cache and refresh from server
}
```

**Response Type**:
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct GetChangeResponse {
    pub success: bool,
    pub change: GerritChange,
    pub sync_status: SyncStatus,
    pub import_progress: ImportProgress,
    pub message: String,
    pub warnings: Vec<String>,
}
```

**Error Codes**:
- `GERRIT_CHANGE_NOT_FOUND`: Change ID does not exist
- `GERRIT_CHANGE_ACCESS_DENIED`: Insufficient permissions
- `GERRIT_CHANGE_ALREADY_IMPORTED`: Change already exists locally
- `GERRIT_IMPORT_FAILED`: General import failure

---

#### `gerrit_search_changes`
**Purpose**: Search and import multiple changes using Gerrit query syntax

**Request Type**:
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchChangesParams {
    pub instance_id: String,      // Gerrit instance UUID
    pub query: String,            // Gerrit query (e.g., "status:open project:payment")
    pub max_results: Option<u32>, // Limit results (default: 50, max: 200)
    pub import_mode: ImportMode,  // How to handle found changes
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ImportMode {
    PreviewOnly,      // Return results without importing
    ImportAll,        // Import all found changes
    ImportSelection,  // Import user-selected changes
}
```

**Response Type**:
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchChangesResponse {
    pub success: bool,
    pub query_id: String,                    // Search query ID for tracking
    pub results: Vec<SearchResult>,          // Found changes
    pub total_available: u32,                // Total matches on server
    pub imported_count: u32,                 // Successfully imported
    pub failed_count: u32,                   // Failed imports
    import_errors: Vec<ImportError>,         // Detailed error information
}
```

---

#### `gerrit_get_diff`
**Purpose**: Load file diff content with line-level granularity

**Request Type**:
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct GetDiffParams {
    pub change_id: String,        // Local change UUID
    pub file_path: String,        // File path within change
    pub patch_set_id: Option<String>, // Specific patch set (default: current)
    pub context_lines: Option<u32>,   // Lines of context (default: 3)
    pub ignore_whitespace: bool,      // Ignore whitespace changes
}
```

**Response Type**:
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct GetDiffResponse {
    pub success: bool,
    pub diff: GerritFileDiff,
    pub content: DiffContent,
    render_time_ms: u32,
    pub cache_hit: bool,
}
```

---

### Review Submission Commands

#### `gerrit_submit_comments`
**Purpose**: Submit comments to Gerrit with batch support

**Request Type**:
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitCommentsParams {
    pub change_id: String,           // Local change UUID
    pub comment_ids: Vec<String>,    // Local comment IDs to submit
    pub batch_mode: BatchMode,       // Submission strategy
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BatchMode {
    AllPending,      // Submit all pending comments
    SelectedOnly,    // Submit only selected comments
    Incremental,     // Submit and continue on partial failure
}
```

**Response Type**:
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitCommentsResponse {
    pub success: bool,
    pub submitted_count: u32,
    pub failed_count: u32,
    pub results: Vec<CommentSubmitResult>,
    pub conflicts: Vec<CommentConflict>,
    pub retry_suggested: bool,
}
```

---

#### `gerrit_submit_review`
**Purpose**: Submit complete review with scores and message

**Request Type**:
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitReviewParams {
    pub change_id: String,           // Local change UUID
    pub patch_set_id: String,        // Target patch set
    pub message: String,             // Review message
    pub labels: HashMap<String, i32>, // Label scores (e.g., {"Code-Review": 2})
    pub comment_ids: Vec<String>,    // Associated comments
    pub draft: bool,                 // Save as draft
    pub notify: NotifyHandling,      // Email notification settings
}
```

**Response Type**:
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitReviewResponse {
    pub success: bool,
    pub review_id: String,           // Gerrit review ID
    pub submitted_at: String,        // ISO 8601 timestamp
    pub message: String,
    pub label_updates: Vec<LabelUpdate>,
}
```

---

### Synchronization Commands

#### `gerrit_sync_changes`
**Purpose**: Synchronize changes with Gerrit server

**Request Type**:
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncChangesParams {
    pub instance_id: String,        // Gerrit instance UUID
    pub change_ids: Vec<String>,    // Specific changes to sync (empty = all)
    pub sync_type: SyncType,        // Type of synchronization
    pub force_full_sync: bool,      // Ignore incremental sync
}
```

**Response Type**:
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncChangesResponse {
    pub success: bool,
    pub sync_id: String,            // Sync operation ID
    pub changes_processed: u32,
    pub changes_updated: u32,
    pub conflicts_detected: u32,
    pub sync_duration_ms: u32,
    pub next_sync_at: String,       // ISO 8601 timestamp
}
```

---

#### `gerrit_get_sync_status`
**Purpose**: Get current synchronization status and pending operations

**Request Type**:
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct GetSyncStatusParams {
    pub instance_id: Option<String>, // Specific instance (null = all)
    pub change_id: Option<String>,   // Specific change (null = all)
}
```

**Response Type**:
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct GetSyncStatusResponse {
    pub success: bool,
    pub sync_operations: Vec<SyncOperation>,
    pub pending_operations: Vec<PendingOperation>,
    pub conflict_summary: ConflictSummary,
    pub last_sync_at: Option<String>,
}
```

---

## Frontend Service Contracts

### Instance Management Service

```typescript
interface GerritInstanceService {
  // Instance CRUD operations
  getInstances(includeInactive?: boolean): Promise<GerritInstance[]>;
  createInstance(params: CreateInstanceParams): Promise<CreateInstanceResult>;
  updateInstance(id: string, updates: Partial<GerritInstance>): Promise<GerritInstance>;
  deleteInstance(id: string): Promise<void>;
  
  // Connection management
  testConnection(instanceId: string): Promise<ConnectionTestResult>;
  setActiveInstance(instanceId: string): Promise<void>;
  getActiveInstance(): Promise<GerritInstance | null>;
  
  // Configuration validation
  validateInstanceConfig(config: Partial<GerritInstance>): ValidationResult[];
}
```

---

### Change Import Service

```typescript
interface GerritChangeService {
  // Change import operations
  importChange(params: ImportChangeParams): Promise<ImportChangeResult>;
  searchChanges(params: SearchChangesParams): Promise<SearchChangesResponse>;
  getChange(changeId: string): Promise<GerritChange>;
  
  // Change management
  listChanges(filters?: ChangeFilters): Promise<GerritChange[]>;
  updateChange(changeId: string, updates: Partial<GerritChange>): Promise<GerritChange>;
  deleteChange(changeId: string): Promise<void>;
  
  // File and diff operations
  getFileDiff(params: GetDiffParams): Promise<GetDiffResponse>;
  getChangeFiles(changeId: string): Promise<GerritFile[]>;
  markFileReviewed(changeId: string, filePath: string): Promise<void>;
}
```

---

### Comment Management Service

```typescript
interface GerritCommentService {
  // Local comment operations
  addComment(params: AddCommentParams): Promise<GerritComment>;
  updateComment(commentId: string, content: string): Promise<GerritComment>;
  deleteComment(commentId: string): Promise<void>;
  getComments(changeId: string, filePath?: string): Promise<GerritComment[]>;
  
  // Comment synchronization
  submitComments(params: SubmitCommentsParams): Promise<SubmitCommentsResponse>;
  syncComments(changeId: string): Promise<SyncCommentsResult>;
  
  // Comment utilities
  resolveComment(commentId: string): Promise<void>;
  createCommentThread(parentId: string, reply: string): Promise<GerritComment>;
  getCommentConflicts(changeId: string): Promise<CommentConflict[]>;
}
```

---

### Review Submission Service

```typescript
interface GerritReviewService {
  // Review submission
  submitReview(params: SubmitReviewParams): Promise<SubmitReviewResponse>;
  submitDraftReview(params: SubmitReviewParams): Promise<SubmitReviewResponse>;
  
  // Review management
  getReview(changeId: string, reviewId: string): Promise<GerritReview>;
  updateReview(reviewId: string, updates: Partial<GerritReview>): Promise<GerritReview>;
  deleteReview(reviewId: string): Promise<void>;
  
  // Label operations
  setLabel(changeId: string, label: string, score: number): Promise<void>;
  getLabelOptions(changeId: string): Promise<LabelOption[]>;
  
  // Batch operations
  batchSubmit(operations: BatchOperation[]): Promise<BatchSubmitResult>;
}
```

---

### Sync Management Service

```typescript
interface GerritSyncService {
  // Sync operations
  syncChanges(params: SyncChangesParams): Promise<SyncChangesResponse>;
  getSyncStatus(params: GetSyncStatusParams): Promise<GetSyncStatusResponse>;
  
  // Conflict resolution
  resolveConflict(conflictId: string, resolution: ConflictResolution): Promise<void>;
  getConflicts(changeId?: string): Promise<ConflictSummary>;
  
  // Auto-sync configuration
  enableAutoSync(instanceId: string, interval: number): Promise<void>;
  disableAutoSync(instanceId: string): Promise<void>;
  getAutoSyncStatus(): Promise<AutoSyncStatus>;
  
  // Operation queue
  getPendingOperations(changeId?: string): Promise<PendingOperation[]>;
  cancelOperation(operationId: string): Promise<void>;
  retryFailedOperations(changeId?: string): Promise<void>;
}
```

---

## Data Transfer Objects

### Core Entities

#### GerritInstance
```typescript
interface GerritInstance {
  id: string;                    // UUID v4
  name: string;                  // Display name
  url: string;                   // Base URL (HTTPS)
  username: string;              // Authentication username
  passwordEncrypted: boolean;    // Indicates encrypted storage
  version: string;               // Gerrit version
  isActive: boolean;             // Currently selected
  lastConnected: string | null;  // ISO 8601 timestamp
  connectionStatus: ConnectionStatus;
  pollingInterval: number;       // Seconds
  maxChanges: number;            // Max imports
  createdAt: string;             // ISO 8601 timestamp
  updatedAt: string;             // ISO 8601 timestamp
}

type ConnectionStatus = 
  | 'connected'
  | 'disconnected' 
  | 'authentication_failed'
  | 'version_incompatible'
  | 'network_error';
```

---

#### GerritChange
```typescript
interface GerritChange {
  id: string;                    // Local UUID
  changeId: string;              // Gerrit Change-ID
  instanceId: string;            // Instance UUID
  project: string;               // Project name
  branch: string;                // Target branch
  subject: string;               // Change title
  status: ChangeStatus;          // Current status
  owner: GerritUser;             // Change owner
  created: string;               // ISO 8601 timestamp
  updated: string;               // ISO 8601 timestamp
  insertions: number;            // Lines added
  deletions: number;             // Lines removed
  currentRevision: string;       // Current patch set revision
  currentPatchSetNum: number;    // Current patch set number
  patchSets: PatchSet[];         // All patch sets
  files: GerritFile[];           // Files in current patch set
  totalFiles: number;            // Total file count
  reviewedFiles: number;         // Locally reviewed files
  localComments: number;         // Local comments count
  remoteComments: number;        // Remote comments count
  importStatus: ImportStatus;    // Local import state
  lastSync: string | null;       // ISO 8601 timestamp
  conflictStatus: ConflictStatus;
  metadata: Record<string, string>;
}

type ChangeStatus = 'new' | 'draft' | 'merged' | 'abandoned';
type ImportStatus = 'pending' | 'importing' | 'imported' | 'failed' | 'outdated';
type ConflictStatus = 'none' | 'comments_pending' | 'patch_set_updated' | 'manual_resolution_required';
```

---

#### GerritComment
```typescript
interface GerritComment {
  id: string;                    // Local UUID
  gerritCommentId: string | null; // Remote comment ID
  changeId: string;              // Change UUID
  patchSetId: string;            // Patch set UUID
  filePath: string;              // File path
  side: CommentSide;             // Side of diff
  line: number;                  // Line number
  range: CommentRange | null;    // Character range
  message: string;               // Comment content
  author: GerritUser;            // Comment author
  created: string;               // ISO 8601 timestamp
  updated: string;               // ISO 8601 timestamp
  status: CommentSyncStatus;     // Sync state
  unresolved: boolean;           // Is unresolved?
  parent: string | null;         // Parent comment ID
  robotId: string | null;        // Automated comment ID
  properties: Record<string, string>;
}

type CommentSide = 'parent' | 'revision';
type CommentSyncStatus = 
  | 'local_only'
  | 'sync_pending' 
  | 'synced'
  | 'sync_failed'
  | 'conflict_detected'
  | 'modified_locally';

interface CommentRange {
  startLine: number;
  startCharacter: number;
  endLine: number;
  endCharacter: number;
}
```

---

### Request/Response Types

#### Connection Test Results
```typescript
interface ConnectionTestResult {
  success: boolean;
  status: ConnectionStatus;
  gerritVersion: string;
  serverTime: string;           // ISO 8601 timestamp
  userInfo: GerritUser | null;
  supportedFeatures: string[];
  errorDetails: string | null;
}

interface GerritUser {
  accountId: number;
  name: string;
  email: string;
  username: string | null;
  avatarUrl: string | null;
}
```

---

#### Import Operations
```typescript
interface ImportChangeParams {
  instanceId: string;
  changeId: string;             // Gerrit Change-ID
  includeComments: boolean;
  includeFiles: boolean;
  forceRefresh: boolean;
}

interface ImportChangeResult {
  success: boolean;
  change: GerritChange | null;
  syncStatus: SyncStatus;
  importProgress: ImportProgress;
  message: string;
  warnings: string[];
}

interface ImportProgress {
  stage: ImportStage;
  currentItem: number;
  totalItems: number;
  estimatedTimeRemaining: number; // seconds
}

type ImportStage = 
  | 'fetching_change'
  | 'fetching_files'
  | 'fetching_comments'
  | 'processing_diffs'
  | 'complete';
```

---

#### Search Operations
```typescript
interface SearchChangesParams {
  instanceId: string;
  query: string;                // Gerrit query syntax
  maxResults?: number;          // Default: 50, Max: 200
  importMode: ImportMode;
}

type ImportMode = 'preview_only' | 'import_all' | 'import_selection';

interface SearchChangesResponse {
  success: boolean;
  queryId: string;
  results: SearchResult[];
  totalAvailable: number;
  importedCount: number;
  failedCount: number;
  importErrors: ImportError[];
}

interface SearchResult {
  changeId: string;
  project: string;
  branch: string;
  subject: string;
  status: ChangeStatus;
  owner: GerritUser;
  created: string;
  updated: string;
  insertions: number;
  deletions: number;
  score: number;                // Relevance score 0-100
}
```

---

#### Diff Operations
```typescript
interface GetDiffParams {
  changeId: string;
  filePath: string;
  patchSetId?: string;
  contextLines?: number;        // Default: 3
  ignoreWhitespace: boolean;
}

interface GetDiffResponse {
  success: boolean;
  diff: GerritFileDiff;
  content: DiffContent;
  renderTimeMs: number;
  cacheHit: boolean;
}

interface GerritFileDiff {
  id: string;
  changeId: string;
  patchSetId: string;
  filePath: string;
  oldPath: string | null;
  changeType: FileChangeType;
  status: FileStatus;
  linesInserted: number;
  linesDeleted: number;
  sizeDelta: number;
  sizeNew: number;
  isBinary: boolean;
  contentType: string;
  diffContent: string | null;
  reviewProgress: ReviewProgress;
  lastReviewed: string | null;
}

type FileChangeType = 'added' | 'modified' | 'deleted' | 'renamed' | 'copied' | 'rewritten';
type FileStatus = 'unreviewed' | 'pending' | 'reviewed' | 'approved' | 'needs_work' | 'question';
```

---

#### Comment Submission
```typescript
interface SubmitCommentsParams {
  changeId: string;
  commentIds: string[];
  batchMode: BatchMode;
}

type BatchMode = 'all_pending' | 'selected_only' | 'incremental';

interface SubmitCommentsResponse {
  success: boolean;
  submittedCount: number;
  failedCount: number;
  results: CommentSubmitResult[];
  conflicts: CommentConflict[];
  retrySuggested: boolean;
}

interface CommentSubmitResult {
  commentId: string;
  success: boolean;
  gerritCommentId: string | null;
  error: string | null;
}

interface CommentConflict {
  commentId: string;
  conflictType: ConflictType;
  remoteComment: GerritComment | null;
  resolutionOptions: ConflictResolutionOption[];
}

type ConflictType = 'concurrent_edit' | 'line_modified' | 'comment_deleted';
```

---

#### Review Submission
```typescript
interface SubmitReviewParams {
  changeId: string;
  patchSetId: string;
  message: string;
  labels: Record<string, number>; // e.g., {"Code-Review": 2}
  commentIds: string[];
  draft: boolean;
  notify: NotifyHandling;
}

type NotifyHandling = 'none' | 'owner' | 'owner_reviewers' | 'all';

interface SubmitReviewResponse {
  success: boolean;
  reviewId: string;
  submittedAt: string;          // ISO 8601 timestamp
  message: string;
  labelUpdates: LabelUpdate[];
}

interface LabelUpdate {
  label: string;
  oldValue: number;
  newValue: number;
  applied: boolean;
}
```

---

#### Synchronization
```typescript
interface SyncChangesParams {
  instanceId: string;
  changeIds: string[];          // Empty = all changes
  syncType: SyncType;
  forceFullSync: boolean;
}

type SyncType = 'full' | 'incremental' | 'comments_only' | 'status_only' | 'push_local';

interface SyncChangesResponse {
  success: boolean;
  syncId: string;
  changesProcessed: number;
  changesUpdated: number;
  conflictsDetected: number;
  syncDurationMs: number;
  nextSyncAt: string;           // ISO 8601 timestamp
}

interface GetSyncStatusParams {
  instanceId?: string;          // Null = all instances
  changeId?: string;            // Null = all changes
}

interface GetSyncStatusResponse {
  success: boolean;
  syncOperations: SyncOperation[];
  pendingOperations: PendingOperation[];
  conflictSummary: ConflictSummary;
  lastSyncAt: string | null;
}
```

---

## API Documentation

### OpenAPI Specification

```yaml
openapi: 3.0.3
info:
  title: HyperReview Gerrit Integration API
  description: REST API for Gerrit Code Review integration
  version: 1.0.0
  contact:
    name: HyperReview Team
    email: support@hyperreview.dev

servers:
  - url: https://api.hyperreview.dev/v1
    description: Production server
  - url: http://localhost:3000/api/v1
    description: Development server

security:
  - bearerAuth: []

paths:
  /gerrit/instances:
    get:
      summary: List Gerrit instances
      operationId: getGerritInstances
      parameters:
        - name: include_inactive
          in: query
          schema:
            type: boolean
            default: false
      responses:
        '200':
          description: List of Gerrit instances
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/GerritInstancesResponse'
        '401':
          $ref: '#/components/responses/UnauthorizedError'
        '500':
          $ref: '#/components/responses/InternalServerError'

    post:
      summary: Create Gerrit instance
      operationId: createGerritInstance
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/CreateInstanceParams'
      responses:
        '201':
          description: Instance created successfully
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/CreateInstanceResponse'
        '400':
          $ref: '#/components/responses/ValidationError'
        '409':
          $ref: '#/components/responses/ConflictError'

  /gerrit/instances/{instanceId}/test:
    post:
      summary: Test Gerrit connection
      operationId: testGerritConnection
      parameters:
        - name: instanceId
          in: path
          required: true
          schema:
            type: string
      responses:
        '200':
          description: Connection test result
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ConnectionTestResult'
        '404':
          $ref: '#/components/responses/NotFoundError'

  /gerrit/changes:
    post:
      summary: Import Gerrit change
      operationId: importGerritChange
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/ImportChangeParams'
      responses:
        '201':
          description: Change imported successfully
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ImportChangeResponse'
        '400':
          $ref: '#/components/responses/ValidationError'
        '404':
          $ref: '#/components/responses/NotFoundError'

  /gerrit/changes/search:
    post:
      summary: Search Gerrit changes
      operationId: searchGerritChanges
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/SearchChangesParams'
      responses:
        '200':
          description: Search results
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/SearchChangesResponse'

components:
  securitySchemes:
    bearerAuth:
      type: http
      scheme: bearer
      bearerFormat: JWT

  schemas:
    GerritInstancesResponse:
      type: object
      properties:
        success:
          type: boolean
        instances:
          type: array
          items:
            $ref: '#/components/schemas/GerritInstance'
        total_count:
          type: integer
        active_count:
          type: integer

    GerritInstance:
      type: object
      required:
        - id
        - name
        - url
        - username
        - connection_status
      properties:
        id:
          type: string
          format: uuid
        name:
          type: string
          minLength: 3
          maxLength: 50
        url:
          type: string
          format: uri
          pattern: '^https://'
        username:
          type: string
          minLength: 1
          maxLength: 100
        version:
          type: string
        is_active:
          type: boolean
        last_connected:
          type: string
          format: date-time
        connection_status:
          type: string
          enum: [connected, disconnected, authentication_failed, version_incompatible, network_error]
        polling_interval:
          type: integer
          minimum: 60
          maximum: 3600
        max_changes:
          type: integer
          minimum: 1
          maximum: 500

    # Additional schemas would continue here...

  responses:
    UnauthorizedError:
      description: Authentication required
      content:
        application/json:
          schema:
            $ref: '#/components/schemas/ErrorResponse'
    
    ValidationError:
      description: Validation failed
      content:
        application/json:
          schema:
            $ref: '#/components/schemas/ValidationErrorResponse'
    
    NotFoundError:
      description: Resource not found
      content:
        application/json:
          schema:
            $ref: '#/components/schemas/ErrorResponse'
    
    ConflictError:
      description: Resource conflict
      content:
        application/json:
          schema:
            $ref: '#/components/schemas/ErrorResponse'
    
    InternalServerError:
      description: Internal server error
      content:
        application/json:
          schema:
            $ref: '#/components/schemas/ErrorResponse'

    ErrorResponse:
      type: object
      properties:
        success:
          type: boolean
          const: false
        error_code:
          type: string
        message:
          type: string
        details:
          type: object

    ValidationErrorResponse:
      allOf:
        - $ref: '#/components/schemas/ErrorResponse'
        - type: object
          properties:
            validation_errors:
              type: array
              items:
                type: object
                properties:
                  field:
                    type: string
                  message:
                    type: string
```

---

## Error Handling

### Error Response Structure
```typescript
interface ApiError {
  success: false;
  errorCode: string;
  message: string;
  details?: Record<string, any>;
  timestamp: string;           // ISO 8601 timestamp
  requestId?: string;          // For debugging
}
```

### Error Categories

#### Authentication Errors (4xx)
- `GERRIT_AUTH_FAILED`: Invalid credentials
- `GERRIT_TOKEN_EXPIRED`: Authentication token expired
- `GERRIT_INSUFFICIENT_PERMISSIONS`: User lacks required permissions

#### Validation Errors (4xx)
- `GERRIT_INVALID_INSTANCE_NAME`: Invalid instance name format
- `GERRIT_INVALID_URL`: Invalid or non-HTTPS URL
- `GERRIT_INVALID_CHANGE_ID`: Invalid change ID format
- `GERRIT_INVALID_QUERY`: Invalid Gerrit query syntax

#### Resource Errors (4xx)
- `GERRIT_INSTANCE_NOT_FOUND`: Instance does not exist
- `GERRIT_CHANGE_NOT_FOUND`: Change does not exist
- `GERRIT_COMMENT_NOT_FOUND`: Comment does not exist

#### Conflict Errors (409)
- `GERRIT_CONCURRENT_MODIFICATION`: Resource modified by another user
- `GERRIT_COMMENT_CONFLICT`: Comment conflict detected
- `GERRIT_CHANGE_ABANDONED`: Change has been abandoned

#### Server Errors (5xx)
- `GERRIT_CONNECTION_FAILED`: Unable to connect to Gerrit
- `GERRIT_API_ERROR`: Gerrit API returned error
- `GERRIT_SYNC_FAILED`: Synchronization failed
- `GERRIT_ENCRYPTION_FAILED`: Credential encryption failed

---

## Validation Schemas

### Instance Configuration Validation
```typescript
const instanceValidationSchema = {
  name: {
    required: true,
    type: 'string',
    minLength: 3,
    maxLength: 50,
    pattern: /^[a-zA-Z0-9\s\-]+$/,
    message: 'Instance name must be 3-50 characters, alphanumeric, spaces, and hyphens only'
  },
  url: {
    required: true,
    type: 'string',
    format: 'uri',
    pattern: /^https:\/\/.+/,
    message: 'URL must use HTTPS protocol'
  },
  username: {
    required: true,
    type: 'string',
    minLength: 1,
    maxLength: 100,
    pattern: /^\S+$/,
    message: 'Username cannot contain whitespace'
  },
  password: {
    required: true,
    type: 'string',
    minLength: 1,
    maxLength: 500,
    message: 'Password is required'
  }
};
```

### Change Import Validation
```typescript
const importValidationSchema = {
  instanceId: {
    required: true,
    type: 'string',
    format: 'uuid',
    message: 'Valid instance ID is required'
  },
  changeId: {
    required: true,
    type: 'string',
    minLength: 1,
    maxLength: 100,
    message: 'Change ID is required'
  },
  includeComments: {
    type: 'boolean',
    default: true
  },
  includeFiles: {
    type: 'boolean',
    default: true
  }
};
```

### Comment Validation
```typescript
const commentValidationSchema = {
  filePath: {
    required: true,
    type: 'string',
    minLength: 1,
    maxLength: 1000,
    message: 'File path is required'
  },
  line: {
    required: true,
    type: 'number',
    minimum: 1,
    message: 'Line number must be positive'
  },
  message: {
    required: true,
    type: 'string',
    minLength: 1,
    maxLength: 16384,  // Gerrit limit
    message: 'Comment message is required and must be under 16KB'
  },
  side: {
    required: true,
    type: 'string',
    enum: ['parent', 'revision'],
    message: 'Comment side must be parent or revision'
  }
};
```

### Review Submission Validation
```typescript
const reviewValidationSchema = {
  changeId: {
    required: true,
    type: 'string',
    format: 'uuid',
    message: 'Valid change ID is required'
  },
  patchSetId: {
    required: true,
    type: 'string',
    format: 'uuid',
    message: 'Valid patch set ID is required'
  },
  message: {
    required: true,
    type: 'string',
    minLength: 1,
    maxLength: 16384,
    message: 'Review message is required and must be under 16KB'
  },
  labels: {
    type: 'object',
    additionalProperties: {
      type: 'number',
      minimum: -2,
      maximum: 2
    },
    message: 'Label scores must be between -2 and +2'
  },
  draft: {
    type: 'boolean',
    default: false
  }
};
```

---

## Performance Specifications

### Response Time Targets
- **Instance connection test**: ≤ 2 seconds
- **Change import (127 files)**: ≤ 3 seconds  
- **File diff loading (5000 lines)**: ≤ 1 second
- **Comment batch submission (47 comments)**: ≤ 2 seconds
- **Search operation**: ≤ 5 seconds
- **Synchronization status**: ≤ 500ms

### Rate Limiting
- **Search queries**: 10 requests per minute per user
- **Change imports**: 5 concurrent imports per instance
- **Comment submissions**: 100 comments per batch maximum
- **API calls**: 100 requests per minute per instance

### Caching Strategy
- **Change metadata**: 5 minutes TTL
- **File diffs**: 15 minutes TTL
- **User information**: 60 minutes TTL
- **Connection status**: 30 seconds TTL
- **Search results**: 2 minutes TTL

---

This comprehensive API contract provides a complete specification for implementing the Gerrit Code Review integration with strong type safety, robust error handling, and performance optimization based on the functional requirements and success criteria defined in the specifications.