# Gerrit Code Review Integration: Data Model Design

## Entity Definitions

### 1. GerritInstance
Represents a configured Gerrit server instance with encrypted credentials and connection settings.

**Fields:**
- `id: String` - Unique identifier (UUID)
- `name: String` - Display name for the instance
- `url: String` - Gerrit server URL (validated format)
- `username_encrypted: Vec<u8>` - AES-256-GCM encrypted username
- `password_encrypted: Vec<u8>` - AES-256-GCM encrypted password
- `version: Option<String>` - Gerrit server version
- `last_connected: Option<DateTime>` - Last successful connection
- `is_active: bool` - Whether this instance is currently active
- `created_at: DateTime` - Instance creation timestamp
- `updated_at: DateTime` - Last modification timestamp

**Relationships:**
- One-to-many with GerritChange (cascade delete)

**Validation Rules:**
- URL must be valid HTTPS endpoint
- Name must be unique across all instances
- Credentials must be encryptable/decryptable

## Core Entities

### 1. GerritInstance

**Description**: Represents a configured Gerrit server instance with authentication and connection settings.

**Attributes**:
- `id`: UUID v4 (Primary Key)
  - Type: UUID v4
  - Generated automatically when instance is configured
  - Unique across all instances
  - Used for foreign key references

- `name`: String (Unique)
  - Type: UTF-8 String
  - Required field, unique constraint
  - Display name for the instance
  - Example: "Production Gerrit", "Dev Gerrit"
  - Max length: 100 characters

- `url`: String
  - Type: HTTPS URL
  - Required field
  - Base URL of Gerrit server
  - Example: "https://gerrit.example.com"
  - Must use HTTPS (TLS enforcement)
  - Validation: RFC 3986 compliant URL

- `username`: String
  - Type: UTF-8 String
  - Required field
  - Gerrit username for HTTP Basic Auth
  - Example: "alice.chen"
  - Max length: 255 characters

- `password_encrypted`: String
  - Type: AES-256-GCM encrypted string
  - Required field
  - HTTP password token encrypted with AES-256-GCM
  - Stored as base64-encoded ciphertext
  - Max length: 2048 characters

- `version`: String
  - Type: Version string
  - Auto-populated on connection
  - Gerrit server version (e.g., "3.6.2")
  - Used for feature compatibility

- `is_active`: Boolean
  - Type: Boolean
  - Default: false
  - Currently selected instance flag
  - Only one instance can be active at a time

- `connection_status`: ConnectionStatus
  - Type: Enum
  - Required field
  - Current connection state
  - Values: Connected, Disconnected, AuthenticationFailed, VersionIncompatible, NetworkError

- `polling_interval`: u32
  - Type: Unsigned 32-bit integer
  - Default: 300 (5 minutes)
  - Range: 60-3600 seconds
  - How often to poll for updates

- `max_changes`: u32
  - Type: Unsigned 32-bit integer
  - Default: 100
  - Range: 10-1000
  - Maximum changes to import in search results

- `created_at`: ISO 8601 String
  - Type: RFC 3339 timestamp
  - Auto-generated on creation
  - UTC timezone required

- `updated_at`: ISO 8601 String
  - Type: RFC 3339 timestamp
  - Auto-updated on modification
  - UTC timezone required

- `last_connected`: Option<ISO 8601>
  - Type: Optional RFC 3339 timestamp
  - Updated on successful connection
  - UTC timezone when present

**Validation Rules**:
- `name` must be unique across all instances
- `name` cannot be empty or whitespace-only
- `url` must be valid HTTPS URL
- `url` must not end with trailing slash
- `username` cannot be empty
- `password_encrypted` must be valid encrypted data
- Only one instance can have `is_active = true`
- `polling_interval` must be between 60-3600 seconds
- `max_changes` must be between 10-1000

**Business Rules**:
- Credentials are encrypted using AES-256-GCM with PBKDF2 key derivation
- Connection status is validated before operations
- Version compatibility check for Gerrit 3.6+
- Automatic reconnection with exponential backoff

### 2. GerritChange

**Description**: Represents a Gerrit Change imported for local review with complete metadata and sync tracking.

**Attributes**:
- `id`: UUID v4 (Primary Key)
  - Type: UUID v4
  - Generated automatically on import
  - Local identifier for the change

- `change_id`: String
  - Type: Gerrit Change-ID format
  - Required field
  - Original Gerrit Change-ID (e.g., "I1234567890abcdef")
  - Used for Gerrit API operations

- `instance_id`: UUID (Foreign Key)
  - Type: UUID v4
  - Required field
  - References GerritInstance.id
  - Establishes which Gerrit server hosts this change

- `project`: String
  - Type: Project path
  - Required field
  - Gerrit project name
  - Example: "platform/frameworks/base"
  - Max length: 500 characters

- `branch`: String
  - Type: Branch name
  - Required field
  - Target branch for the change
  - Example: "main", "develop"
  - Max length: 255 characters

- `subject`: String
  - Type: Change subject/title
  - Required field
  - Human-readable change description
  - Max length: 1000 characters

- `status`: ChangeStatus
  - Type: Enum
  - Required field
  - Current state in Gerrit workflow
  - Values: New, Draft, Merged, Abandoned

- `owner`: GerritUser (JSON)
  - Type: Serialized user object
  - Required field
  - Change creator information
  - Includes account_id, name, email, username, avatar_url

- `created`: ISO 8601 String
  - Type: RFC 3339 timestamp
  - Required field
  - When change was created in Gerrit
  - UTC timezone

- `updated`: ISO 8601 String
  - Type: RFC 8601 timestamp
  - Required field
  - Last update timestamp from Gerrit
  - UTC timezone

- `insertions`: u32
  - Type: Unsigned 32-bit integer
  - Required field
  - Lines of code added
  - Default: 0

- `deletions`: u32
  - Type: Unsigned 32-bit integer
  - Required field
  - Lines of code removed
  - Default: 0

- `current_revision`: String
  - Type: Git commit SHA
  - Required field
  - SHA-1 hash of current patch set
  - 40-character hex string

- `current_patch_set_num`: u32
  - Type: Unsigned 32-bit integer
  - Required field
  - Latest patch set number
  - Starts at 1, increments with each new patch set

- `patch_sets`: Vec<PatchSet>
  - Type: Vector of patch sets
  - Required field
  - All patch sets in the change
  - Ordered by patch set number

- `files`: Vec<GerritFile>
  - Type: Vector of files
  - Required field
  - Files in current patch set
  - Cached for offline access

- `total_files`: u32
  - Type: Unsigned 32-bit integer
  - Required field
  - Total number of files in change
  - Used for progress tracking

- `reviewed_files`: u32
  - Type: Unsigned 32-bit integer
  - Required field
  - Number of files reviewed locally
  - Default: 0, updated during review

- `local_comments`: u32
  - Type: Unsigned 32-bit integer
  - Required field
  - Comments created locally
  - Default: 0, updated when comments added

- `remote_comments`: u32
  - Type: Unsigned 32-bit integer
  - Required field
  - Comments from Gerrit
  - Default: 0, set during import

- `import_status`: ImportStatus
  - Type: Enum
  - Required field
  - Local import state
  - Values: Pending, Importing, Imported, Failed, Outdated

- `last_sync`: Option<ISO 8601>
  - Type: Optional RFC 3339 timestamp
  - When last sync with Gerrit occurred
  - UTC timezone when present

- `conflict_status`: ConflictStatus
  - Type: Enum
  - Required field
  - Conflict detection state
  - Values: None, CommentsPending, PatchSetUpdated, ManualResolutionRequired

- `metadata`: HashMap<String, String>
  - Type: JSON object
  - Optional additional metadata
  - Key-value pairs for extensibility

**Validation Rules**:
- `change_id` must be valid Gerrit Change-ID format
- `instance_id` must reference existing GerritInstance
- `project` cannot be empty
- `branch` cannot be empty
- `subject` cannot be empty
- `current_revision` must be valid 40-character SHA-1
- `current_patch_set_num` must be >= 1
- `total_files` must match length of `files` vector
- `reviewed_files` cannot exceed `total_files`
- Timestamps must be valid ISO 8601 format

**State Transitions**:
```
Import Status:
Pending → Importing → Imported
   ↓         ↓           ↓
Failed    Failed    Outdated (when newer patch set available)
```

**Business Rules**:
- Changes are immutable once imported (except review progress)
- Conflict detection compares local vs remote timestamps
- Outdated status when newer patch set exists in Gerrit
- Progress tracking updates incrementally during review

### 3. GerritComment

**Description**: Represents a review comment with support for both local creation and remote synchronization.

**Attributes**:
- `id`: UUID v4 (Primary Key)
  - Type: UUID v4
  - Generated automatically on creation
  - Local identifier for the comment

- `gerrit_comment_id`: Option<String>
  - Type: Optional Gerrit comment ID
  - Populated after successful sync
  - Used for updates and conflict detection
  - Format: Gerrit-specific comment identifier

- `change_id`: UUID (Foreign Key)
  - Type: UUID v4
  - Required field
  - References GerritChange.id
  - Establishes comment ownership

- `patch_set_id`: UUID (Foreign Key)
  - Type: UUID v4
  - Required field
  - References PatchSet.id
  - Associates comment with specific patch set

- `file_path`: String
  - Type: File path within change
  - Required field
  - Relative path from repository root
  - Example: "src/main/java/com/example/Main.java"

- `side`: CommentSide
  - Type: Enum
  - Required field
  - Which side of diff comment targets
  - Values: Parent (old version), Revision (new version)

- `line`: u32
  - Type: Unsigned 32-bit integer
  - Required field
  - 1-based line number in file
  - Range: 1 to file line count

- `range`: Option<CommentRange>
  - Type: Optional character range
  - For inline comments on specific characters
  - JSON format: {start_line, start_character, end_line, end_character}

- `message`: String
  - Type: Comment content
  - Required field
  - Markdown-supported text
  - Max length: 10000 characters

- `author`: GerritUser (JSON)
  - Type: Serialized user object
  - Required field
  - Comment creator information
  - For local comments, uses current user

- `created`: ISO 8601 String
  - Type: RFC 3339 timestamp
  - Required field
  - When comment was created
  - UTC timezone

- `updated`: ISO 8601 String
  - Type: RFC 3339 timestamp
  - Required field
  - Last modification timestamp
  - UTC timezone

- `status`: CommentSyncStatus
  - Type: Enum
  - Required field
  - Synchronization state
  - Values: LocalOnly, SyncPending, Synced, SyncFailed, ConflictDetected, ModifiedLocally

- `unresolved`: Boolean
  - Type: Boolean flag
  - Required field
  - Whether comment requires resolution
  - Default: true for new comments

- `parent`: Option<UUID>
  - Type: Optional foreign key
  - References GerritComment.id
  - For comment threads/replies
  - Null for top-level comments

- `robot_id`: Option<String>
  - Type: Optional robot identifier
  - For automated comments
  - Example: "checkstyle", "findbugs"

- `properties`: HashMap<String, String>
  - Type: JSON object
  - Additional comment properties
  - Extensibility for Gerrit-specific features

**CommentRange Struct**:
```rust
struct CommentRange {
    start_line: u32,      // 1-based line number
    start_character: u32, // 0-based character position
    end_line: u32,        // 1-based line number
    end_character: u32,   // 0-based character position
}
```

**Validation Rules**:
- `file_path` must exist in the change
- `line` must be valid for the file
- `message` cannot be empty
- `change_id` and `patch_set_id` must reference existing entities
- Character ranges must be valid for the file content
- `parent` must reference comment in same change

**Sync Status Transitions**:
```
LocalOnly → SyncPending → Synced
    ↓           ↓           ↓
   Failed    Failed    ModifiedLocally
    ↓           ↓           ↓
ConflictDetected ←─── SyncFailed
```

## SQLite Schema Design

### Database Schema

```sql
-- GerritInstance table
CREATE TABLE gerrit_instances (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    url TEXT NOT NULL,
    username TEXT NOT NULL,
    password_encrypted TEXT NOT NULL,
    version TEXT NOT NULL DEFAULT '',
    is_active INTEGER NOT NULL DEFAULT 0,
    last_connected TEXT,
    connection_status TEXT NOT NULL,
    polling_interval INTEGER NOT NULL DEFAULT 300,
    max_changes INTEGER NOT NULL DEFAULT 100,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    
    CONSTRAINT chk_url_https CHECK (url LIKE 'https://%'),
    CONSTRAINT chk_polling_interval CHECK (polling_interval BETWEEN 60 AND 3600),
    CONSTRAINT chk_max_changes CHECK (max_changes BETWEEN 10 AND 1000),
    CONSTRAINT chk_connection_status CHECK (connection_status IN ('Connected', 'Disconnected', 'AuthenticationFailed', 'VersionIncompatible', 'NetworkError'))
);

-- GerritChange table
CREATE TABLE gerrit_changes (
    id TEXT PRIMARY KEY,
    change_id TEXT NOT NULL,
    instance_id TEXT NOT NULL,
    project TEXT NOT NULL,
    branch TEXT NOT NULL,
    subject TEXT NOT NULL,
    status TEXT NOT NULL,
    owner TEXT NOT NULL, -- JSON
    created TEXT NOT NULL,
    updated TEXT NOT NULL,
    insertions INTEGER NOT NULL DEFAULT 0,
    deletions INTEGER NOT NULL DEFAULT 0,
    current_revision TEXT NOT NULL,
    current_patch_set_num INTEGER NOT NULL,
    total_files INTEGER NOT NULL DEFAULT 0,
    reviewed_files INTEGER NOT NULL DEFAULT 0,
    local_comments INTEGER NOT NULL DEFAULT 0,
    remote_comments INTEGER NOT NULL DEFAULT 0,
    import_status TEXT NOT NULL,
    last_sync TEXT,
    conflict_status TEXT NOT NULL DEFAULT 'None',
    metadata TEXT NOT NULL DEFAULT '{}', -- JSON
    
    FOREIGN KEY (instance_id) REFERENCES gerrit_instances(id) ON DELETE CASCADE,
    CONSTRAINT chk_change_status CHECK (status IN ('New', 'Draft', 'Merged', 'Abandoned')),
    CONSTRAINT chk_import_status CHECK (import_status IN ('Pending', 'Importing', 'Imported', 'Failed', 'Outdated')),
    CONSTRAINT chk_conflict_status CHECK (conflict_status IN ('None', 'CommentsPending', 'PatchSetUpdated', 'ManualResolutionRequired')),
    CONSTRAINT chk_revision_length CHECK (length(current_revision) = 40),
    CONSTRAINT chk_patch_set_num CHECK (current_patch_set_num >= 1),
    CONSTRAINT chk_file_counts CHECK (reviewed_files <= total_files),
    CONSTRAINT chk_comment_counts CHECK (local_comments >= 0 AND remote_comments >= 0),
    UNIQUE(instance_id, change_id)
);

-- PatchSet table
CREATE TABLE patch_sets (
    id TEXT PRIMARY KEY,
    gerrit_patch_set_id TEXT NOT NULL,
    change_id TEXT NOT NULL,
    revision TEXT NOT NULL,
    number INTEGER NOT NULL,
    author TEXT NOT NULL, -- JSON
    commit_message TEXT NOT NULL,
    created TEXT NOT NULL,
    kind TEXT NOT NULL,
    size_insertions INTEGER NOT NULL DEFAULT 0,
    size_deletions INTEGER NOT NULL DEFAULT 0,
    is_current INTEGER NOT NULL DEFAULT 0,
    
    FOREIGN KEY (change_id) REFERENCES gerrit_changes(id) ON DELETE CASCADE,
    CONSTRAINT chk_revision_length CHECK (length(revision) = 40),
    CONSTRAINT chk_patch_set_number CHECK (number >= 1),
    UNIQUE(change_id, number),
    UNIQUE(change_id, is_current) -- Only one current patch set per change
);

-- GerritFile table
CREATE TABLE gerrit_files (
    id TEXT PRIMARY KEY,
    change_id TEXT NOT NULL,
    patch_set_id TEXT NOT NULL,
    file_path TEXT NOT NULL,
    old_path TEXT,
    change_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Unreviewed',
    lines_inserted INTEGER NOT NULL DEFAULT 0,
    lines_deleted INTEGER NOT NULL DEFAULT 0,
    size_delta INTEGER NOT NULL DEFAULT 0,
    size_new INTEGER NOT NULL DEFAULT 0,
    is_binary INTEGER NOT NULL DEFAULT 0,
    content_type TEXT NOT NULL DEFAULT 'text/plain',
    diff_content TEXT,
    review_progress TEXT NOT NULL DEFAULT '{"total_lines":0,"reviewed_lines":0,"comment_count":0,"severity_score":0}', -- JSON
    last_reviewed TEXT,
    
    FOREIGN KEY (change_id) REFERENCES gerrit_changes(id) ON DELETE CASCADE,
    FOREIGN KEY (patch_set_id) REFERENCES patch_sets(id) ON DELETE CASCADE,
    CONSTRAINT chk_change_type CHECK (change_type IN ('Added', 'Modified', 'Deleted', 'Renamed', 'Copied', 'Rewritten')),
    CONSTRAINT chk_file_status CHECK (status IN ('Unreviewed', 'Pending', 'Reviewed', 'Approved', 'NeedsWork', 'Question')),
    CONSTRAINT chk_size_constraints CHECK (size_new >= 0 AND lines_inserted >= 0 AND lines_deleted >= 0),
    UNIQUE(change_id, patch_set_id, file_path)
);

-- GerritComment table
CREATE TABLE gerrit_comments (
    id TEXT PRIMARY KEY,
    gerrit_comment_id TEXT,
    change_id TEXT NOT NULL,
    patch_set_id TEXT NOT NULL,
    file_path TEXT NOT NULL,
    side TEXT NOT NULL,
    line INTEGER NOT NULL,
    range TEXT, -- JSON
    message TEXT NOT NULL,
    author TEXT NOT NULL, -- JSON
    created TEXT NOT NULL,
    updated TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'LocalOnly',
    unresolved INTEGER NOT NULL DEFAULT 1,
    parent TEXT,
    robot_id TEXT,
    properties TEXT NOT NULL DEFAULT '{}', -- JSON
    
    FOREIGN KEY (change_id) REFERENCES gerrit_changes(id) ON DELETE CASCADE,
    FOREIGN KEY (patch_set_id) REFERENCES patch_sets(id) ON DELETE CASCADE,
    FOREIGN KEY (parent) REFERENCES gerrit_comments(id) ON DELETE CASCADE,
    CONSTRAINT chk_comment_side CHECK (side IN ('Parent', 'Revision')),
    CONSTRAINT chk_comment_status CHECK (status IN ('LocalOnly', 'SyncPending', 'Synced', 'SyncFailed', 'ConflictDetected', 'ModifiedLocally')),
    CONSTRAINT chk_line_number CHECK (line >= 1),
    CONSTRAINT chk_message_length CHECK (length(message) <= 10000),
    UNIQUE(change_id, patch_set_id, file_path, line, side, message) -- Prevent duplicates
);

-- GerritReview table
CREATE TABLE gerrit_reviews (
    id TEXT PRIMARY KEY,
    gerrit_review_id TEXT,
    change_id TEXT NOT NULL,
    patch_set_id TEXT NOT NULL,
    message TEXT NOT NULL,
    labels TEXT NOT NULL DEFAULT '{}', -- JSON
    comments TEXT NOT NULL DEFAULT '[]', -- JSON array of UUIDs
    author TEXT NOT NULL, -- JSON
    created TEXT NOT NULL,
    submitted TEXT,
    status TEXT NOT NULL DEFAULT 'Draft',
    draft INTEGER NOT NULL DEFAULT 1,
    notify TEXT NOT NULL DEFAULT 'Owner',
    
    FOREIGN KEY (change_id) REFERENCES gerrit_changes(id) ON DELETE CASCADE,
    FOREIGN KEY (patch_set_id) REFERENCES patch_sets(id) ON DELETE CASCADE,
    CONSTRAINT chk_review_status CHECK (status IN ('Draft', 'PendingSubmission', 'Submitted', 'SubmissionFailed', 'PartiallySubmitted')),
    CONSTRAINT chk_notify CHECK (notify IN ('None', 'Owner', 'OwnerReviewers', 'All')),
    CONSTRAINT chk_message_length CHECK (length(message) <= 20000)
);

-- SyncStatus table
CREATE TABLE sync_status (
    id TEXT PRIMARY KEY,
    instance_id TEXT NOT NULL,
    change_id TEXT,
    last_sync TEXT NOT NULL,
    next_sync TEXT,
    sync_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Pending',
    items_processed INTEGER NOT NULL DEFAULT 0,
    items_total INTEGER NOT NULL DEFAULT 0,
    conflicts_detected INTEGER NOT NULL DEFAULT 0,
    errors TEXT NOT NULL DEFAULT '[]', -- JSON
    metadata TEXT NOT NULL DEFAULT '{}', -- JSON
    
    FOREIGN KEY (instance_id) REFERENCES gerrit_instances(id) ON DELETE CASCADE,
    FOREIGN KEY (change_id) REFERENCES gerrit_changes(id) ON DELETE CASCADE,
    CONSTRAINT chk_sync_type CHECK (sync_type IN ('Full', 'Incremental', 'CommentsOnly', 'StatusOnly', 'PushLocal')),
    CONSTRAINT chk_sync_operation_status CHECK (status IN ('Pending', 'InProgress', 'Completed', 'Failed', 'Cancelled')),
    CONSTRAINT chk_progress CHECK (items_processed <= items_total)
);

-- OperationQueue table
CREATE TABLE operation_queue (
    id TEXT PRIMARY KEY,
    instance_id TEXT NOT NULL,
    change_id TEXT NOT NULL,
    operation_type TEXT NOT NULL,
    payload TEXT NOT NULL,
    priority TEXT NOT NULL DEFAULT 'Normal',
    status TEXT NOT NULL DEFAULT 'Queued',
    retry_count INTEGER NOT NULL DEFAULT 0,
    max_retries INTEGER NOT NULL DEFAULT 3,
    created TEXT NOT NULL,
    last_attempt TEXT,
    next_retry TEXT,
    error_message TEXT,
    
    FOREIGN KEY (instance_id) REFERENCES gerrit_instances(id) ON DELETE CASCADE,
    FOREIGN KEY (change_id) REFERENCES gerrit_changes(id) ON DELETE CASCADE,
    CONSTRAINT chk_operation_type CHECK (operation_type IN ('AddComment', 'UpdateComment', 'DeleteComment', 'SubmitReview', 'UpdateLabels', 'PushPatchSet')),
    CONSTRAINT chk_priority CHECK (priority IN ('Low', 'Normal', 'High', 'Critical')),
    CONSTRAINT chk_operation_status CHECK (status IN ('Queued', 'Processing', 'Completed', 'Failed', 'Cancelled', 'WaitingForDependency')),
    CONSTRAINT chk_retry_count CHECK (retry_count <= max_retries),
    CONSTRAINT chk_max_retries CHECK (max_retries BETWEEN 0 AND 10)
);

-- Indexes for performance
CREATE INDEX idx_gerrit_changes_instance ON gerrit_changes(instance_id);
CREATE INDEX idx_gerrit_changes_status ON gerrit_changes(status);
CREATE INDEX idx_gerrit_changes_project ON gerrit_changes(project);
CREATE INDEX idx_gerrit_changes_updated ON gerrit_changes(updated);
CREATE INDEX idx_gerrit_changes_import_status ON gerrit_changes(import_status);

CREATE INDEX idx_patch_sets_change ON patch_sets(change_id);
CREATE INDEX idx_patch_sets_current ON patch_sets(is_current);

CREATE INDEX idx_gerrit_files_change ON gerrit_files(change_id);
CREATE INDEX idx_gerrit_files_patch_set ON gerrit_files(patch_set_id);
CREATE INDEX idx_gerrit_files_path ON gerrit_files(file_path);
CREATE INDEX idx_gerrit_files_status ON gerrit_files(status);

CREATE INDEX idx_gerrit_comments_change ON gerrit_comments(change_id);
CREATE INDEX idx_gerrit_comments_patch_set ON gerrit_comments(patch_set_id);
CREATE INDEX idx_gerrit_comments_file ON gerrit_comments(file_path);
CREATE INDEX idx_gerrit_comments_status ON gerrit_comments(status);
CREATE INDEX idx_gerrit_comments_line ON gerrit_comments(line);

CREATE INDEX idx_gerrit_reviews_change ON gerrit_reviews(change_id);
CREATE INDEX idx_gerrit_reviews_patch_set ON gerrit_reviews(patch_set_id);
CREATE INDEX idx_gerrit_reviews_status ON gerrit_reviews(status);

CREATE INDEX idx_sync_status_instance ON sync_status(instance_id);
CREATE INDEX idx_sync_status_change ON sync_status(change_id);
CREATE INDEX idx_sync_status_status ON sync_status(status);

CREATE INDEX idx_operation_queue_instance ON operation_queue(instance_id);
CREATE INDEX idx_operation_queue_change ON operation_queue(change_id);
CREATE INDEX idx_operation_queue_status ON operation_queue(status);
CREATE INDEX idx_operation_queue_priority ON operation_queue(priority);
CREATE INDEX idx_operation_queue_next_retry ON operation_queue(next_retry);
```

## Migration Scripts

### Initial Migration (V001)
```sql
-- Migration: V001__create_gerrit_tables.sql
-- Description: Initial schema creation for Gerrit integration

BEGIN TRANSACTION;

-- Create all tables and indexes as defined above

-- Insert initial data
INSERT INTO gerrit_instances (
    id, name, url, username, password_encrypted, version,
    is_active, connection_status, polling_interval, max_changes,
    created_at, updated_at
) VALUES (
    'default-local',
    'Local Development',
    'https://localhost:8080',
    'admin',
    'encrypted_default_password',
    '3.6.0',
    0,
    'Disconnected',
    300,
    100,
    datetime('now'),
    datetime('now')
);

COMMIT;
```

### Performance Optimization Migration (V002)
```sql
-- Migration: V002__add_performance_indexes.sql
-- Description: Add additional indexes for common query patterns

BEGIN TRANSACTION;

-- Composite indexes for complex queries
CREATE INDEX idx_changes_instance_status ON gerrit_changes(instance_id, status);
CREATE INDEX idx_changes_instance_updated ON gerrit_changes(instance_id, updated);
CREATE INDEX idx_comments_change_status ON gerrit_comments(change_id, status);
CREATE INDEX idx_files_change_status ON gerrit_files(change_id, status);
CREATE INDEX idx_reviews_change_status ON gerrit_reviews(change_id, status);

-- Full-text search indexes (if SQLite FTS is available)
CREATE VIRTUAL TABLE IF NOT EXISTS gerrit_changes_fts USING fts5(
    subject, project, 
    content='gerrit_changes',
    content_rowid='id'
);

CREATE VIRTUAL TABLE IF NOT EXISTS gerrit_comments_fts USING fts5(
    message,
    content='gerrit_comments',
    content_rowid='id'
);

COMMIT;
```

## Data Flow Patterns

### Import Operation Flow
```
1. User initiates import with Change ID
2. Validate GerritInstance connection
3. Fetch change details from Gerrit API
4. Create GerritChange record with ImportStatus=Importing
5. Fetch all patch sets and create PatchSet records
6. Fetch file list and create GerritFile records
7. Fetch existing comments and create GerritComment records
8. Update GerritChange with ImportStatus=Imported
9. Create initial SyncStatus record
```

### Comment Creation Flow
```
1. User adds comment in UI
2. Create GerritComment with status=LocalOnly
3. Associate with current patch set
4. Update GerritChange local_comments count
5. Create OperationQueue entry for sync
6. Update UI to show local comment
7. Schedule background sync if online
```

### Sync Operation Flow
```
1. Sync triggered (manual, scheduled, or background)
2. Create SyncStatus with status=InProgress
3. Fetch remote change status from Gerrit
4. Compare local vs remote timestamps
5. Detect conflicts (concurrent modifications)
6. Perform three-way merge for conflicts
7. Update local records with merged data
8. Process OperationQueue entries
9. Update SyncStatus with results
10. Notify UI of sync completion
```

### Batch Push Operation Flow
```
1. User initiates batch push (Shift+Enter)
2. Collect all pending local comments
3. Validate review completeness
4. Create GerritReview record
5. Update comment statuses to SyncPending
6. Create OperationQueue entries
7. Process operations sequentially
8. Handle conflicts with retry logic
9. Update statuses based on results
10. Provide user confirmation
```

## Conflict Resolution

### Conflict Detection
- Timestamp comparison between local and remote versions
- Hash-based content comparison for comments
- Three-way merge using common ancestor

### Conflict Types
1. **Comment Conflicts**: Same line has different local and remote comments
2. **Review State Conflicts**: Local and remote review scores differ
3. **Patch Set Conflicts**: Newer patch set exists in Gerrit
4. **Status Conflicts**: Change status differs (merged/abandoned locally vs remotely)

### Resolution Strategies
1. **Automatic Merge**: When changes don't overlap
2. **User Prompt**: For semantic conflicts requiring human judgment
3. **Local Wins**: Preserve local changes (user choice)
4. **Remote Wins**: Accept remote changes (user choice)
5. **Manual Resolution**: User edits merged result

## Performance Optimizations

### Database Optimizations
- **Indexes**: Strategic indexes on foreign keys and common query patterns
- **Composite Indexes**: For complex multi-column queries
- **Partial Indexes**: For status-based filtering
- **Covering Indexes**: Include frequently accessed columns

### Caching Strategy
- **Multi-tier Cache**: Memory (LRU) + SQLite persistent cache
- **TTL-based Invalidation**: Time-based cache expiration
- **Change-based Invalidation**: Invalidate on sync operations
- **Size Limits**: Prevent unbounded memory growth

### Query Optimization
- **Batch Operations**: Process multiple items in single queries
- **Lazy Loading**: Load data on-demand for large changes
- **Streaming**: Use streaming for large result sets
- **Pagination**: Implement cursor-based pagination

## Security Considerations

### Credential Security
- **AES-256-GCM Encryption**: Strong encryption for stored credentials
- **PBKDF2 Key Derivation**: High iteration count for key derivation
- **Secure Storage**: Use platform-specific secure storage APIs
- **Key Rotation**: Support for credential rotation

### Data Integrity
- **Foreign Key Constraints**: Enforce referential integrity
- **Check Constraints**: Validate data ranges and formats
- **Unique Constraints**: Prevent duplicate entries
- **Transaction Boundaries**: Ensure atomic operations

### Audit Trail
- **Operation Logging**: Log all Gerrit operations
- **Timestamp Tracking**: Track creation and modification times
- **User Attribution**: Track which user performed operations
- **Error Logging**: Comprehensive error tracking

## Summary

This comprehensive data model provides:

✅ **Complete Entity Coverage**: All 8 core entities with proper relationships
✅ **Normalization**: 3NF compliant schema preventing data duplication
✅ **Performance**: Strategic indexing and optimization for common queries
✅ **Integrity**: Constraints, foreign keys, and validation rules
✅ **Security**: AES-256-GCM encryption and secure credential storage
✅ **Conflict Resolution**: Three-way merge and user intervention strategies
✅ **Offline Support**: Operation queue for offline-to-online synchronization
✅ **Extensibility**: JSON metadata fields for future enhancements
✅ **Audit Trail**: Comprehensive logging and timestamp tracking
✅ **Migration Support**: Versioned schema evolution

The model supports all functional requirements while maintaining data integrity, performance, and security for enterprise-grade Gerrit integration.