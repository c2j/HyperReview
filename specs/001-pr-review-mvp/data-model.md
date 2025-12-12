# Data Model: HyperReview MVP

**Feature**: 001-pr-review-mvp
**Date**: 2025-11-23

## Domain Model Overview

The HyperReview data model follows a local-first architecture where all entities are stored in SQLite and synced with remote providers (GitHub/GitLab). Each entity has a sync status to support offline operation.

## Core Entities

### Account

Represents a connected provider account (GitHub/GitLab).

**Fields**:
- `id: String` - Composite key `{provider}:{username}` (e.g., "github:alice")
- `provider: Provider` - Enum: GitHub | GitLab
- `username: String` - Provider username
- `avatar_url: Option<String>` - User avatar URL
- `access_token: SecureString` - Encrypted OAuth access token
- `refresh_token: Option<SecureString>` - Encrypted refresh token
- `token_expires_at: Option<DateTime<Utc>>` - Token expiration timestamp

**Relationships**:
- One Account has many Repositories (implicit via provider queries)

**Validation Rules**:
- `id` MUST match pattern `{provider}:{username}`
- `access_token` MUST be encrypted at rest
- Token refresh MUST occur before `token_expires_at`

**State Transitions**:
```
Unauthenticated -> Authenticating (OAuth flow in progress)
Authenticating -> Authenticated (token received)
Authenticated -> TokenExpired (token_expires_at passed)
TokenExpired -> Authenticated (refresh successful)
```

---

### Repository

Represents a git repository from a provider.

**Fields**:
- `id: RepoId` - Newtype wrapping `String`, format: `{provider}:{owner}/{name}`
- `provider: Provider` - GitHub | GitLab
- `owner: String` - Repository owner (user or org)
- `name: String` - Repository name
- `local_clone_path: Option<PathBuf>` - Path to shallow clone if exists
- `last_synced_at: Option<DateTime<Utc>>` - Last successful sync timestamp

**Relationships**:
- One Repository has many PullRequests

**Validation Rules**:
- `id` MUST match pattern `{provider}:{owner}/{name}`
- `local_clone_path` MUST exist on filesystem if set
- `name` MUST NOT contain '/' or special chars

**Operations**:
- `ensure_cloned()` - Shallow clone if not present
- `fetch_updates()` - Fetch new commits for PRs
- `delete_clone()` - Remove local clone to free space

---

### PullRequest

Represents a code change proposal awaiting review.

**Fields**:
- `id: PrId` - Newtype wrapping `String`, format: `{provider}:{owner}/{repo}#{number}`
- `repo_id: RepoId` - Foreign key to Repository
- `number: u32` - PR number within repository
- `title: String` - PR title
- `author_username: String` - PR author
- `author_avatar_url: Option<String>` - Author avatar
- `status: PrStatus` - Enum: Open | Closed | Merged
- `head_sha: String` - Git SHA of head commit
- `base_sha: String` - Git SHA of base commit
- `ci_status: Option<CiStatus>` - Enum: Pending | Success | Failure
- `body_markdown: String` - PR description in markdown
- `is_read: bool` - User has viewed this PR
- `is_archived: bool` - User archived from inbox
- `updated_at: DateTime<Utc>` - Last updated on provider
- `created_at: DateTime<Utc>` - PR creation time
- `last_synced_at: Option<DateTime<Utc>>` - Last sync from provider

**Relationships**:
- Belongs to one Repository
- Has many Comments
- Has many Reviews

**Validation Rules**:
- `id` MUST match pattern `{provider}:{owner}/{repo}#{number}`
- `head_sha` and `base_sha` MUST be valid git SHAs (40 hex chars)
- `status` transitions: Open -> Closed, Open -> Merged (no reverse)

**Derived Fields**:
- `needs_review: bool` - Calculated from user assignment/mentions
- `file_count: u32` - Calculated from diff
- `additions: u32` - Calculated from diff
- `deletions: u32` - Calculated from diff

---

### Comment

Represents feedback on a specific code location or general PR comment.

**Fields**:
- `id: CommentId` - Newtype wrapping `String`, local UUID or remote ID
- `pr_id: PrId` - Foreign key to PullRequest
- `remote_id: Option<String>` - Provider comment ID (null if not synced)
- `file_path: Option<String>` - File path for inline comments (null for general)
- `line_number: Option<u32>` - Line number for inline comments
- `content: String` - Comment text (markdown)
- `author_username: String` - Comment author
- `sync_status: SyncStatus` - Enum: Pending | Synced | Failed
- `sync_error: Option<String>` - Error message if sync failed
- `created_at: DateTime<Utc>` - Local creation time
- `updated_at: DateTime<Utc>` - Last update time

**Relationships**:
- Belongs to one PullRequest
- May reply to another Comment (thread parent)

**Validation Rules**:
- Inline comments MUST have `file_path` AND `line_number`
- General comments MUST have `file_path = None` AND `line_number = None`
- `sync_status = Synced` REQUIRES `remote_id` to be set
- `content` MUST NOT be empty

**State Transitions**:
```
Pending (created offline) -> Syncing (upload in progress)
Syncing -> Synced (upload successful, remote_id set)
Syncing -> Failed (upload failed, sync_error set)
Failed -> Pending (retry requested)
```

---

### Review

Represents a formal review submission with decision.

**Fields**:
- `id: ReviewId` - Newtype wrapping `String`
- `pr_id: PrId` - Foreign key to PullRequest
- `remote_id: Option<String>` - Provider review ID
- `decision: ReviewDecision` - Enum: Approve | RequestChanges | Comment
- `body: Option<String>` - Optional summary text
- `comment_ids: Vec<CommentId>` - Associated comments submitted with review
- `sync_status: SyncStatus` - Pending | Synced | Failed
- `submitted_at: DateTime<Utc>` - Submission timestamp

**Relationships**:
- Belongs to one PullRequest
- References multiple Comments

**Validation Rules**:
- `decision = RequestChanges` MUST have at least one comment
- `decision = Approve` MAY have optional comments
- All `comment_ids` MUST reference same `pr_id`
- Review MUST be submitted atomically with all comments

**State Transitions**:
```
Draft (comments added) -> Submitting (API call in progress)
Submitting -> Synced (submission successful)
Submitting -> Failed (submission failed)
```

---

### Diff (Computed Entity - Not Persisted)

Represents the computed changes between base and head commits. Calculated on-demand from git2-rs.

**Fields**:
- `pr_id: PrId` - Associated PR
- `files: Vec<FileDiff>` - Changed files

**FileDiff**:
- `path: String` - File path
- `old_path: Option<String>` - Original path if renamed
- `status: FileStatus` - Added | Modified | Deleted | Renamed
- `hunks: Vec<Hunk>` - Changed regions
- `is_binary: bool` - Binary file indicator
- `additions: u32` - Lines added
- `deletions: u32` - Lines deleted

**Hunk**:
- `old_start: u32` - Starting line in old file
- `old_lines: u32` - Line count in old file
- `new_start: u32` - Starting line in new file
- `new_lines: u32` - Line count in new file
- `header: String` - Hunk header text
- `lines: Vec<DiffLine>` - Individual line changes

**DiffLine**:
- `line_type: LineType` - Context | Addition | Deletion
- `old_line_num: Option<u32>` - Line number in old file
- `new_line_num: Option<u32>` - Line number in new file
- `content: String` - Line content
- `highlight_ranges: Vec<Range<usize>>` - Syntax highlight ranges

---

## Newtype ID Wrappers

To satisfy Constitution III (strong typing), all entity IDs are wrapped in newtypes:

```rust
// In src/models/ids.rs
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RepoId(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrId(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommentId(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReviewId(String);

impl RepoId {
    pub fn new(provider: &str, owner: &str, name: &str) -> Self {
        Self(format!("{}:{}/{}", provider, owner, name))
    }

    pub fn parse(s: &str) -> Result<Self, ParseError> {
        // Validate pattern and return
    }
}

// Similar implementations for PrId, CommentId, ReviewId
```

---

## Enums

### Provider
```rust
pub enum Provider {
    GitHub,
    GitLab,
}
```

### PrStatus
```rust
pub enum PrStatus {
    Open,
    Closed,
    Merged,
}
```

### CiStatus
```rust
pub enum CiStatus {
    Pending,
    Success,
    Failure,
}
```

### SyncStatus
```rust
pub enum SyncStatus {
    Pending,   // Created locally, not yet synced
    Synced,    // Successfully synced to provider
    Failed,    // Sync failed, manual intervention needed
}
```

### ReviewDecision
```rust
pub enum ReviewDecision {
    Approve,
    RequestChanges,
    Comment,
}
```

### FileStatus
```rust
pub enum FileStatus {
    Added,
    Modified,
    Deleted,
    Renamed,
}
```

### LineType
```rust
pub enum LineType {
    Context,   // Unchanged line
    Addition,  // Added line
    Deletion,  // Deleted line
}
```

---

## Data Flow Diagrams

### Sync Flow (Offline -> Online)

```
User creates Comment (offline)
    ↓
Comment.sync_status = Pending
    ↓
SQLite INSERT
    ↓
Network available detected
    ↓
SyncService queries comments WHERE sync_status = Pending
    ↓
For each pending comment:
    ↓
    GitHub API POST /repos/{owner}/{repo}/pulls/{number}/comments
    ↓
    Success:
        - Set remote_id = response.id
        - Set sync_status = Synced
        - UPDATE SQLite
    ↓
    Failure:
        - Set sync_status = Failed
        - Set sync_error = error message
        - UPDATE SQLite
        - Log for retry
```

### PR Loading Flow

```
User selects PR in inbox (j/k navigation)
    ↓
InboxView emits SelectPR action with pr_id
    ↓
AppState receives action
    ↓
Background task spawned:
    ↓
    Repository.ensure_cloned() - shallow clone if needed
    ↓
    GitService.compute_diff(base_sha, head_sha)
        - Spawned on tokio::spawn_blocking (CPU-intensive)
        - Returns Diff entity
    ↓
    tree-sitter parse for syntax highlighting
        - Spawned on blocking thread pool
        - Returns highlight_ranges
    ↓
DiffView notified via GPUI update
    ↓
DiffView.render() with Editor + TextHighlights + BlockDecorations
```

---

## Indexing Strategy

For SQLite query performance:

```sql
-- Inbox queries (most common)
CREATE INDEX idx_prs_status_updated
ON pull_requests(status, updated_at DESC)
WHERE is_archived = 0;

-- Sync queries
CREATE INDEX idx_comments_sync
ON comments(sync_status)
WHERE sync_status != 'synced';

CREATE INDEX idx_reviews_sync
ON reviews(sync_status)
WHERE sync_status != 'synced';

-- PR lookup
CREATE UNIQUE INDEX idx_prs_repo_number
ON pull_requests(repo_id, number);

-- Comment threads
CREATE INDEX idx_comments_pr_line
ON comments(pr_id, file_path, line_number);
```

---

## Database Migrations

Migrations managed by sqlx-cli. Example initial migration:

```sql
-- migrations/001_initial_schema.sql
-- (Full schema from research.md RQ-4)
```

Future migrations follow naming: `{number}_{description}.sql`
