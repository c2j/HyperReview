/**
 * Gerrit Integration TypeScript Type Definitions
 * 
 * This file contains all TypeScript interfaces and types for the Gerrit Code Review
 * integration feature, providing complete type safety for frontend development.
 */

// ============================================================================
// Core Enumerations
// ============================================================================

/**
 * Gerrit connection status states
 */
export enum ConnectionStatus {
  CONNECTED = 'connected',
  DISCONNECTED = 'disconnected',
  AUTHENTICATION_FAILED = 'authentication_failed',
  VERSION_INCOMPATIBLE = 'version_incompatible',
  NETWORK_ERROR = 'network_error'
}

/**
 * Gerrit change status values
 */
export enum ChangeStatus {
  NEW = 'new',
  DRAFT = 'draft',
  MERGED = 'merged',
  ABANDONED = 'abandoned'
}

/**
 * Import status for local changes
 */
export enum ImportStatus {
  PENDING = 'pending',
  IMPORTING = 'importing',
  IMPORTED = 'imported',
  FAILED = 'failed',
  OUTDATED = 'outdated'
}

/**
 * Conflict status for changes
 */
export enum ConflictStatus {
  NONE = 'none',
  COMMENTS_PENDING = 'comments_pending',
  PATCH_SET_UPDATED = 'patch_set_updated',
  MANUAL_RESOLUTION_REQUIRED = 'manual_resolution_required'
}

/**
 * Comment side (parent vs revision)
 */
export enum CommentSide {
  PARENT = 'parent',
  REVISION = 'revision'
}

/**
 * Comment synchronization status
 */
export enum CommentSyncStatus {
  LOCAL_ONLY = 'local_only',
  SYNC_PENDING = 'sync_pending',
  SYNCED = 'synced',
  SYNC_FAILED = 'sync_failed',
  CONFLICT_DETECTED = 'conflict_detected',
  MODIFIED_LOCALLY = 'modified_locally'
}

/**
 * Review submission status
 */
export enum ReviewStatus {
  DRAFT = 'draft',
  PENDING_SUBMISSION = 'pending_submission',
  SUBMITTED = 'submitted',
  SUBMISSION_FAILED = 'submission_failed',
  PARTIALLY_SUBMITTED = 'partially_submitted'
}

/**
 * Notification handling options
 */
export enum NotifyHandling {
  NONE = 'none',
  OWNER = 'owner',
  OWNER_REVIEWERS = 'owner_reviewers',
  ALL = 'all'
}

/**
 * File change types
 */
export enum FileChangeType {
  ADDED = 'added',
  MODIFIED = 'modified',
  DELETED = 'deleted',
  RENAMED = 'renamed',
  COPIED = 'copied',
  REWRITTEN = 'rewritten'
}

/**
 * File review status
 */
export enum FileStatus {
  UNREVIEWED = 'unreviewed',
  PENDING = 'pending',
  REVIEWED = 'reviewed',
  APPROVED = 'approved',
  NEEDS_WORK = 'needs_work',
  QUESTION = 'question'
}

/**
 * Patch set kinds
 */
export enum PatchSetKind {
  REWORK = 'rework',
  TRIVIAL_REBASE = 'trivial_rebase',
  NO_CODE_CHANGE = 'no_code_change',
  NO_CHANGE = 'no_change',
  MERGE_FIRST_PARENT_UPDATE = 'merge_first_parent_update',
  MERGE = 'merge',
  REWRITTEN = 'rewritten'
}

/**
 * Sync operation types
 */
export enum SyncType {
  FULL = 'full',
  INCREMENTAL = 'incremental',
  COMMENTS_ONLY = 'comments_only',
  STATUS_ONLY = 'status_only',
  PUSH_LOCAL = 'push_local'
}

/**
 * Sync operation status
 */
export enum SyncOperationStatus {
  PENDING = 'pending',
  IN_PROGRESS = 'in_progress',
  COMPLETED = 'completed',
  FAILED = 'failed',
  CANCELLED = 'cancelled'
}

/**
 * Operation types for queue
 */
export enum OperationType {
  ADD_COMMENT = 'add_comment',
  UPDATE_COMMENT = 'update_comment',
  DELETE_COMMENT = 'delete_comment',
  SUBMIT_REVIEW = 'submit_review',
  UPDATE_LABELS = 'update_labels',
  PUSH_PATCH_SET = 'push_patch_set'
}

/**
 * Operation priority levels
 */
export enum OperationPriority {
  LOW = 'low',
  NORMAL = 'normal',
  HIGH = 'high',
  CRITICAL = 'critical'
}

/**
 * Operation status
 */
export enum OperationStatus {
  QUEUED = 'queued',
  PROCESSING = 'processing',
  COMPLETED = 'completed',
  FAILED = 'failed',
  CANCELLED = 'cancelled',
  WAITING_FOR_DEPENDENCY = 'waiting_for_dependency'
}

/**
 * Query types for search
 */
export enum QueryType {
  CHANGE_ID = 'change_id',
  STATUS = 'status',
  PROJECT = 'project',
  OWNER = 'owner',
  SEARCH = 'search',
  CUSTOM = 'custom'
}

/**
 * Query execution status
 */
export enum QueryStatus {
  PENDING = 'pending',
  EXECUTING = 'executing',
  COMPLETED = 'completed',
  FAILED = 'failed',
  EXPIRED = 'expired'
}

/**
 * Import modes for search results
 */
export enum ImportMode {
  PREVIEW_ONLY = 'preview_only',
  IMPORT_ALL = 'import_all',
  IMPORT_SELECTION = 'import_selection'
}

/**
 * Batch modes for comment submission
 */
export enum BatchMode {
  ALL_PENDING = 'all_pending',
  SELECTED_ONLY = 'selected_only',
  INCREMENTAL = 'incremental'
}

/**
 * Conflict types
 */
export enum ConflictType {
  CONCURRENT_EDIT = 'concurrent_edit',
  LINE_MODIFIED = 'line_modified',
  COMMENT_DELETED = 'comment_deleted'
}

/**
 * Import stages for progress tracking
 */
export enum ImportStage {
  FETCHING_CHANGE = 'fetching_change',
  FETCHING_FILES = 'fetching_files',
  FETCHING_COMMENTS = 'fetching_comments',
  PROCESSING_DIFFS = 'processing_diffs',
  COMPLETE = 'complete'
}

// ============================================================================
// Core Entity Interfaces
// ============================================================================

/**
 * Gerrit instance configuration
 */
export interface GerritInstance {
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

/**
 * Gerrit user information
 */
export interface GerritUser {
  accountId: number;
  name: string;
  email: string;
  username: string | null;
  avatarUrl: string | null;
}

/**
 * Gerrit change entity
 */
export interface GerritChange {
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

/**
 * Patch set information
 */
export interface PatchSet {
  id: string;                    // Local UUID
  gerritPatchSetId: string;      // Gerrit patch set ID
  changeId: string;              // Foreign key to GerritChange
  revision: string;              // Git commit SHA
  number: number;                // Patch set number
  author: GerritUser;            // Patch set author
  commitMessage: string;         // Commit message
  created: string;               // ISO 8601 timestamp
  kind: PatchSetKind;            // Type of patch set
  files: string[];               // File IDs in this patch set
  sizeInsertions: number;        // Total lines added
  sizeDeletions: number;         // Total lines removed
  isCurrent: boolean;            // Is this the current patch set?
}

/**
 * Gerrit comment entity
 */
export interface GerritComment {
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

/**
 * Comment range for inline comments
 */
export interface CommentRange {
  startLine: number;
  startCharacter: number;
  endLine: number;
  endCharacter: number;
}

/**
 * Gerrit review entity
 */
export interface GerritReview {
  id: string;                    // Local UUID
  gerritReviewId: string | null; // Gerrit review ID
  changeId: string;              // Change UUID
  patchSetId: string;            // Target patch set
  message: string;               // Review message
  labels: Record<string, number>; // Label scores
  comments: string[];            // Associated comment IDs
  author: GerritUser;            // Review author
  created: string;               // ISO 8601 timestamp
  submitted: string | null;      // When pushed to Gerrit
  status: ReviewStatus;          // Local status
  draft: boolean;                // Is draft review?
  notify: NotifyHandling;        // Email notification settings
}

/**
 * Gerrit file entity
 */
export interface GerritFile {
  id: string;                    // Local UUID
  changeId: string;              // Foreign key to GerritChange
  patchSetId: string;            // Foreign key to PatchSet
  filePath: string;              // File path
  oldPath: string | null;        // Previous path (for renames)
  changeType: FileChangeType;    // Type of change
  status: FileStatus;            // Review status
  linesInserted: number;         // Lines added
  linesDeleted: number;          // Lines removed
  sizeDelta: number;             // Size change in bytes
  sizeNew: number;               // New file size
  isBinary: boolean;             // Is binary file?
  contentType: string;           // MIME type
  diffContent: string | null;    // Cached diff content
  reviewProgress: ReviewProgress;
  lastReviewed: string | null;   // ISO 8601 timestamp
}

/**
 * Review progress tracking
 */
export interface ReviewProgress {
  totalLines: number;
  reviewedLines: number;
  commentCount: number;
  severityScore: number;         // 0-100
}

/**
 * Sync status tracking
 */
export interface SyncStatus {
  id: string;                    // UUID v4
  instanceId: string;            // Foreign key to GerritInstance
  changeId: string | null;       // Optional: specific change
  lastSync: string;              // ISO 8601 timestamp
  nextSync: string | null;       // ISO 8601 timestamp
  syncType: SyncType;            // Type of sync operation
  status: SyncOperationStatus;   // Current status
  itemsProcessed: number;        // Number of items processed
  itemsTotal: number;            // Total items to process
  conflictsDetected: number;     // Number of conflicts
  errors: SyncError[];           // Error details
  metadata: Record<string, string>;
}

/**
 * Sync error details
 */
export interface SyncError {
  code: string;                  // Error code
  message: string;               // Error message
  context: string | null;        // Additional context
  timestamp: string;             // ISO 8601 timestamp
}

/**
 * Operation queue entry
 */
export interface OperationQueue {
  id: string;                    // UUID v4
  instanceId: string;            // Foreign key to GerritInstance
  changeId: string;              // Foreign key to GerritChange
  operationType: OperationType;  // Type of operation
  payload: string;               // JSON-encoded operation data
  priority: OperationPriority;   // Execution priority
  status: OperationStatus;       // Current status
  retryCount: number;            // Number of retry attempts
  maxRetries: number;            // Maximum retry attempts
  created: string;               // ISO 8601 timestamp
  lastAttempt: string | null;    // ISO 8601 timestamp
  nextRetry: string | null;      // ISO 8601 timestamp
  errorMessage: string | null;   // Last error
}

/**
 * Search query and results
 */
export interface SearchQuery {
  id: string;                    // UUID v4
  instanceId: string;            // Foreign key to GerritInstance
  query: string;                 // Gerrit query string
  queryType: QueryType;          // Type of query
  results: SearchResult[];       // Cached results
  resultCount: number;           // Total results available
  status: QueryStatus;           // Query execution status
  created: string;               // ISO 8601 timestamp
  expires: string;               // ISO 8601 timestamp
}

/**
 * Search result item
 */
export interface SearchResult {
  changeId: string;              // Gerrit Change-ID
  project: string;               // Project name
  branch: string;                // Target branch
  subject: string;               // Change subject
  status: ChangeStatus;          // Change status
  owner: GerritUser;             // Change owner
  created: string;               // ISO 8601 timestamp
  updated: string;               // ISO 8601 timestamp
  insertions: number;            // Lines added
  deletions: number;             // Lines removed
  score: number;                 // Relevance score 0-100
}

// ============================================================================
// Request/Response Interfaces
// ============================================================================

/**
 * Connection test parameters
 */
export interface TestConnectionParams {
  instanceId: string;
}

/**
 * Connection test result
 */
export interface ConnectionTestResult {
  success: boolean;
  status: ConnectionStatus;
  gerritVersion: string;
  serverTime: string;           // ISO 8601 timestamp
  userInfo: GerritUser | null;
  supportedFeatures: string[];
  errorDetails: string | null;
}

/**
 * Create instance parameters
 */
export interface CreateInstanceParams {
  name: string;
  url: string;
  username: string;
  password: string;
  pollingInterval?: number;
  maxChanges?: number;
}

/**
 * Create instance result
 */
export interface CreateInstanceResult {
  success: boolean;
  instance: GerritInstance;
  message: string;
  testConnectionResult?: ConnectionTestResult;
}

/**
 * Import change parameters
 */
export interface ImportChangeParams {
  instanceId: string;
  changeId: string;
  includeComments: boolean;
  includeFiles: boolean;
  forceRefresh: boolean;
}

/**
 * Import change result
 */
export interface ImportChangeResult {
  success: boolean;
  change: GerritChange | null;
  syncStatus: SyncStatus;
  importProgress: ImportProgress;
  message: string;
  warnings: string[];
}

/**
 * Import progress tracking
 */
export interface ImportProgress {
  stage: ImportStage;
  currentItem: number;
  totalItems: number;
  estimatedTimeRemaining: number; // seconds
}

/**
 * Search changes parameters
 */
export interface SearchChangesParams {
  instanceId: string;
  query: string;
  maxResults?: number;
  importMode: ImportMode;
}

/**
 * Search changes response
 */
export interface SearchChangesResponse {
  success: boolean;
  queryId: string;
  results: SearchResult[];
  totalAvailable: number;
  importedCount: number;
  failedCount: number;
  importErrors: ImportError[];
}

/**
 * Import error details
 */
export interface ImportError {
  changeId: string;
  errorCode: string;
  message: string;
  timestamp: string;
}

/**
 * Get diff parameters
 */
export interface GetDiffParams {
  changeId: string;
  filePath: string;
  patchSetId?: string;
  contextLines?: number;
  ignoreWhitespace: boolean;
}

/**
 * Get diff response
 */
export interface GetDiffResponse {
  success: boolean;
  diff: GerritFile;
  content: DiffContent;
  renderTimeMs: number;
  cacheHit: boolean;
}

/**
 * Diff content structure
 */
export interface DiffContent {
  lines: DiffLine[];
  statistics: DiffStatistics;
  metadata: Record<string, string>;
}

/**
 * Individual diff line
 */
export interface DiffLine {
  lineNumber: number;
  oldLineNumber: number | null;
  newLineNumber: number | null;
  content: string;
  type: DiffLineType;
  hasComments: boolean;
  commentIds: string[];
}

/**
 * Diff line types
 */
export enum DiffLineType {
  CONTEXT = 'context',
  ADDED = 'added',
  REMOVED = 'removed',
  MODIFIED = 'modified'
}

/**
 * Diff statistics
 */
export interface DiffStatistics {
  totalLines: number;
  addedLines: number;
  removedLines: number;
  modifiedLines: number;
  contextLines: number;
}

/**
 * Add comment parameters
 */
export interface AddCommentParams {
  changeId: string;
  patchSetId: string;
  filePath: string;
  line: number;
  message: string;
  side: CommentSide;
  parentId?: string;
  range?: CommentRange;
}

/**
 * Submit comments parameters
 */
export interface SubmitCommentsParams {
  changeId: string;
  commentIds: string[];
  batchMode: BatchMode;
}

/**
 * Submit comments response
 */
export interface SubmitCommentsResponse {
  success: boolean;
  submittedCount: number;
  failedCount: number;
  results: CommentSubmitResult[];
  conflicts: CommentConflict[];
  retrySuggested: boolean;
}

/**
 * Comment submission result
 */
export interface CommentSubmitResult {
  commentId: string;
  success: boolean;
  gerritCommentId: string | null;
  error: string | null;
}

/**
 * Comment conflict information
 */
export interface CommentConflict {
  commentId: string;
  conflictType: ConflictType;
  remoteComment: GerritComment | null;
  resolutionOptions: ConflictResolutionOption[];
}

/**
 * Conflict resolution option
 */
export interface ConflictResolutionOption {
  id: string;
  description: string;
  action: string;
  autoResolvable: boolean;
}

/**
 * Submit review parameters
 */
export interface SubmitReviewParams {
  changeId: string;
  patchSetId: string;
  message: string;
  labels: Record<string, number>;
  commentIds: string[];
  draft: boolean;
  notify: NotifyHandling;
}

/**
 * Submit review response
 */
export interface SubmitReviewResponse {
  success: boolean;
  reviewId: string;
  submittedAt: string;
  message: string;
  labelUpdates: LabelUpdate[];
}

/**
 * Label update information
 */
export interface LabelUpdate {
  label: string;
  oldValue: number;
  newValue: number;
  applied: boolean;
}

/**
 * Sync changes parameters
 */
export interface SyncChangesParams {
  instanceId: string;
  changeIds: string[];
  syncType: SyncType;
  forceFullSync: boolean;
}

/**
 * Sync changes response
 */
export interface SyncChangesResponse {
  success: boolean;
  syncId: string;
  changesProcessed: number;
  changesUpdated: number;
  conflictsDetected: number;
  syncDurationMs: number;
  nextSyncAt: string;
}

/**
 * Get sync status parameters
 */
export interface GetSyncStatusParams {
  instanceId?: string;
  changeId?: string;
}

/**
 * Get sync status response
 */
export interface GetSyncStatusResponse {
  success: boolean;
  syncOperations: SyncOperation[];
  pendingOperations: PendingOperation[];
  conflictSummary: ConflictSummary;
  lastSyncAt: string | null;
}

/**
 * Sync operation details
 */
export interface SyncOperation {
  id: string;
  instanceId: string;
  changeId: string | null;
  syncType: SyncType;
  status: SyncOperationStatus;
  startedAt: string;
  completedAt: string | null;
  itemsProcessed: number;
  itemsTotal: number;
  errors: SyncError[];
}

/**
 * Pending operation
 */
export interface PendingOperation {
  id: string;
  operationType: OperationType;
  changeId: string;
  priority: OperationPriority;
  status: OperationStatus;
  retryCount: number;
  maxRetries: number;
  createdAt: string;
  nextRetryAt: string | null;
}

/**
 * Conflict summary
 */
export interface ConflictSummary {
  totalConflicts: number;
  commentConflicts: number;
  statusConflicts: number;
  fileConflicts: number;
  autoResolvable: number;
  manualResolutionRequired: number;
}

// ============================================================================
// Error Handling Interfaces
// ============================================================================

/**
 * API error response
 */
export interface ApiError {
  success: false;
  errorCode: string;
  message: string;
  details?: Record<string, any>;
  timestamp: string;           // ISO 8601 timestamp
  requestId?: string;          // For debugging
}

/**
 * Validation error response
 */
export interface ValidationErrorResponse extends ApiError {
  validationErrors: ValidationError[];
}

/**
 * Individual validation error
 */
export interface ValidationError {
  field: string;
  message: string;
  value?: any;
}

/**
 * Conflict resolution
 */
export interface ConflictResolution {
  conflictId: string;
  resolutionType: 'merge' | 'overwrite' | 'discard' | 'manual';
  resolutionData?: Record<string, any>;
}

// ============================================================================
// Service Interfaces
// ============================================================================

/**
 * Instance management service interface
 */
export interface GerritInstanceService {
  getInstances(includeInactive?: boolean): Promise<GerritInstance[]>;
  createInstance(params: CreateInstanceParams): Promise<CreateInstanceResult>;
  updateInstance(id: string, updates: Partial<GerritInstance>): Promise<GerritInstance>;
  deleteInstance(id: string): Promise<void>;
  testConnection(instanceId: string): Promise<ConnectionTestResult>;
  setActiveInstance(instanceId: string): Promise<void>;
  getActiveInstance(): Promise<GerritInstance | null>;
  validateInstanceConfig(config: Partial<GerritInstance>): ValidationError[];
}

/**
 * Change management service interface
 */
export interface GerritChangeService {
  importChange(params: ImportChangeParams): Promise<ImportChangeResult>;
  searchChanges(params: SearchChangesParams): Promise<SearchChangesResponse>;
  getChange(changeId: string): Promise<GerritChange>;
  listChanges(filters?: ChangeFilters): Promise<GerritChange[]>;
  updateChange(changeId: string, updates: Partial<GerritChange>): Promise<GerritChange>;
  deleteChange(changeId: string): Promise<void>;
  getFileDiff(params: GetDiffParams): Promise<GetDiffResponse>;
  getChangeFiles(changeId: string): Promise<GerritFile[]>;
  markFileReviewed(changeId: string, filePath: string): Promise<void>;
}

/**
 * Comment management service interface
 */
export interface GerritCommentService {
  addComment(params: AddCommentParams): Promise<GerritComment>;
  updateComment(commentId: string, content: string): Promise<GerritComment>;
  deleteComment(commentId: string): Promise<void>;
  getComments(changeId: string, filePath?: string): Promise<GerritComment[]>;
  submitComments(params: SubmitCommentsParams): Promise<SubmitCommentsResponse>;
  syncComments(changeId: string): Promise<SyncCommentsResult>;
  resolveComment(commentId: string): Promise<void>;
  createCommentThread(parentId: string, reply: string): Promise<GerritComment>;
  getCommentConflicts(changeId: string): Promise<CommentConflict[]>;
}

/**
 * Review submission service interface
 */
export interface GerritReviewService {
  submitReview(params: SubmitReviewParams): Promise<SubmitReviewResponse>;
  submitDraftReview(params: SubmitReviewParams): Promise<SubmitReviewResponse>;
  getReview(changeId: string, reviewId: string): Promise<GerritReview>;
  updateReview(reviewId: string, updates: Partial<GerritReview>): Promise<GerritReview>;
  deleteReview(reviewId: string): Promise<void>;
  setLabel(changeId: string, label: string, score: number): Promise<void>;
  getLabelOptions(changeId: string): Promise<LabelOption[]>;
  batchSubmit(operations: BatchOperation[]): Promise<BatchSubmitResult>;
}

/**
 * Sync management service interface
 */
export interface GerritSyncService {
  syncChanges(params: SyncChangesParams): Promise<SyncChangesResponse>;
  getSyncStatus(params: GetSyncStatusParams): Promise<GetSyncStatusResponse>;
  resolveConflict(conflictId: string, resolution: ConflictResolution): Promise<void>;
  getConflicts(changeId?: string): Promise<ConflictSummary>;
  enableAutoSync(instanceId: string, interval: number): Promise<void>;
  disableAutoSync(instanceId: string): Promise<void>;
  getAutoSyncStatus(): Promise<AutoSyncStatus>;
  getPendingOperations(changeId?: string): Promise<PendingOperation[]>;
  cancelOperation(operationId: string): Promise<void>;
  retryFailedOperations(changeId?: string): Promise<void>;
}

/**
 * Additional helper interfaces
 */
export interface ChangeFilters {
  instanceId?: string;
  status?: ChangeStatus[];
  project?: string[];
  branch?: string[];
  owner?: string[];
  hasLocalChanges?: boolean;
  needsSync?: boolean;
  searchQuery?: string;
}

export interface SyncCommentsResult {
  success: boolean;
  syncedCount: number;
  conflictCount: number;
  errors: SyncError[];
}

export interface LabelOption {
  name: string;
  description: string;
  values: LabelValue[];
  defaultValue: number;
}

export interface LabelValue {
  value: number;
  description: string;
}

export interface BatchOperation {
  type: 'comment' | 'review' | 'label';
  params: any;
  priority: OperationPriority;
}

export interface BatchSubmitResult {
  success: boolean;
  operations: BatchOperationResult[];
  totalCount: number;
  successCount: number;
  failureCount: number;
}

export interface BatchOperationResult {
  operationId: string;
  success: boolean;
  error?: string;
  result?: any;
}

export interface AutoSyncStatus {
  enabled: boolean;
  instanceId?: string;
  interval: number;
  lastSync?: string;
  nextSync?: string;
  status: SyncOperationStatus;
}

// ============================================================================
// Error Code Constants
// ============================================================================

/**
 * Gerrit integration error codes
 */
export enum GerritErrorCode {
  // Authentication errors (4xx)
  AUTH_FAILED = 'GERRIT_AUTH_FAILED',
  TOKEN_EXPIRED = 'GERRIT_TOKEN_EXPIRED',
  INSUFFICIENT_PERMISSIONS = 'GERRIT_INSUFFICIENT_PERMISSIONS',
  
  // Validation errors (4xx)
  INVALID_INSTANCE_NAME = 'GERRIT_INVALID_INSTANCE_NAME',
  INVALID_URL = 'GERRIT_INVALID_URL',
  INVALID_CHANGE_ID = 'GERRIT_INVALID_CHANGE_ID',
  INVALID_QUERY = 'GERRIT_INVALID_QUERY',
  
  // Resource errors (4xx)
  INSTANCE_NOT_FOUND = 'GERRIT_INSTANCE_NOT_FOUND',
  CHANGE_NOT_FOUND = 'GERRIT_CHANGE_NOT_FOUND',
  COMMENT_NOT_FOUND = 'GERRIT_COMMENT_NOT_FOUND',
  
  // Conflict errors (409)
  CONCURRENT_MODIFICATION = 'GERRIT_CONCURRENT_MODIFICATION',
  COMMENT_CONFLICT = 'GERRIT_COMMENT_CONFLICT',
  CHANGE_ABANDONED = 'GERRIT_CHANGE_ABANDONED',
  
  // Server errors (5xx)
  CONNECTION_FAILED = 'GERRIT_CONNECTION_FAILED',
  API_ERROR = 'GERRIT_API_ERROR',
  SYNC_FAILED = 'GERRIT_SYNC_FAILED',
  ENCRYPTION_FAILED = 'GERRIT_ENCRYPTION_FAILED',
  
  // General errors
  UNKNOWN_ERROR = 'GERRIT_UNKNOWN_ERROR',
  NOT_IMPLEMENTED = 'GERRIT_NOT_IMPLEMENTED',
  RATE_LIMIT_EXCEEDED = 'GERRIT_RATE_LIMIT_EXCEEDED'
}

// ============================================================================
// Utility Types
// ============================================================================

/**
 * Type guards
 */
export const isGerritInstance = (obj: any): obj is GerritInstance => {
  return obj && typeof obj.id === 'string' && typeof obj.name === 'string' && typeof obj.url === 'string';
};

export const isGerritChange = (obj: any): obj is GerritChange => {
  return obj && typeof obj.id === 'string' && typeof obj.changeId === 'string' && typeof obj.instanceId === 'string';
};

export const isGerritComment = (obj: any): obj is GerritComment => {
  return obj && typeof obj.id === 'string' && typeof obj.changeId === 'string' && typeof obj.message === 'string';
};

export const isApiError = (obj: any): obj is ApiError => {
  return obj && obj.success === false && typeof obj.errorCode === 'string' && typeof obj.message === 'string';
};

/**
 * Helper type for partial updates
 */
export type PartialGerritInstance = Partial<GerritInstance>;
export type PartialGerritChange = Partial<GerritChange>;
export type PartialGerritComment = Partial<GerritComment>;
export type PartialGerritReview = Partial<GerritReview>;

/**
 * Collection types
 */
export type GerritInstanceCollection = Record<string, GerritInstance>;
export type GerritChangeCollection = Record<string, GerritChange>;
export type GerritCommentCollection = Record<string, GerritComment>;
export type GerritReviewCollection = Record<string, GerritReview>;

/**
 * Status update callbacks
 */
export type StatusUpdateCallback = (status: SyncOperationStatus, progress?: number) => void;
export type ErrorCallback = (error: ApiError) => void;
export type ProgressCallback = (stage: ImportStage, current: number, total: number) => void;

// Default export for convenience
export default {
  ConnectionStatus,
  ChangeStatus,
  ImportStatus,
  ConflictStatus,
  CommentSide,
  CommentSyncStatus,
  ReviewStatus,
  NotifyHandling,
  FileChangeType,
  FileStatus,
  PatchSetKind,
  SyncType,
  SyncOperationStatus,
  OperationType,
  OperationPriority,
  OperationStatus,
  QueryType,
  QueryStatus,
  ImportMode,
  BatchMode,
  ConflictType,
  ImportStage,
  DiffLineType,
  GerritErrorCode
};