# HyperReview Backend Data Model

**Date**: 2025-12-13
**Purpose**: Define data structures, relationships, and validation rules for HyperReview backend
**Source**: Feature specification entities and requirements

---

## 1. Repository Entity

### 1.1 Repository (Git Repository Metadata)

**Purpose**: Track repository state and metadata for quick access

**Fields**:
- `path`: String (absolute path to repository root)
  - Validation: Must exist and be a valid Git repository
  - Constraints: UTF-8 encoded, max 4096 characters
- `current_branch`: String (name of active branch)
  - Validation: Must exist in repository
  - Constraints: UTF-8 encoded, max 256 characters
- `last_opened`: DateTime (ISO 8601 timestamp)
  - Purpose: Sorting recent repositories
- `head_commit`: String (git OID of HEAD)
  - Format: 40-character hexadecimal string
- `remote_url`: Option<String> (remote origin URL)
  - Validation: Valid URL if present
- `is_active`: Boolean (currently loaded in memory)

**Relationships**:
- Repository has many Branches
- Repository has many Files
- Repository has many Comments (through Files)
- Repository has many Tasks

**State Transitions**:
- `inactive` → `loading` → `active` (when opened)
- `active` → `inactive` (when closed)
- `loading` → `error` (if open fails)

---

## 2. Version Control Entities

### 2.1 Branch

**Purpose**: Track Git branch information

**Fields**:
- `name`: String (branch name)
  - Validation: Must be valid Git reference
  - Constraints: UTF-8 encoded, max 256 characters
- `is_current`: Boolean (active branch flag)
- `is_remote`: Boolean (remote vs local branch)
- `upstream`: Option<String> (tracking branch name)
- `last_commit`: String (git OID)
- `last_commit_message`: String (truncated to 140 chars)
- `last_commit_author`: String (author name)
- `last_commit_date`: DateTime

**Validation Rules**:
- Branch names cannot contain `~`, `^`, `:`, `*`, `?`, `[`, `\`
- Cannot start with `/` or end with `/`
- Cannot contain consecutive slashes

### 2.2 Commit

**Purpose**: Represent Git commit metadata

**Fields**:
- `oid`: String (40-char git OID)
- `message`: String (commit message)
- `author_name`: String (author name)
- `author_email`: String (author email)
- `committer_name`: String (committer name)
- `committer_email`: String (committer email)
- `timestamp`: DateTime (commit time)
- `parents`: Vec<String> (parent commit OIDs)
- `tree_oid`: String (tree OID)

### 2.3 DiffLine

**Purpose**: Represent a single line in a file diff

**Fields**:
- `old_line_number`: Option<u32> (line in old version)
- `new_line_number`: Option<u32> (line in new version)
- `content`: String (line content without newline)
- `line_type`: DiffLineType (added/removed/context/header)
- `severity`: Option<Severity> (ERROR/WARNING/INFO/SUCCESS)
- `message`: Option<String> (analysis message for this line)
- `hunk_header`: Option<String> (for @@ -1,3 +1,4 @@ format)

**Validation Rules**:
- Either `old_line_number` or `new_line_number` must be present
- Content must be valid UTF-8
- Max line length: 10,000 characters (prevent DoS)

**Enums**:
```rust
enum DiffLineType {
    Added,
    Removed,
    Context,
    Header,
}

enum Severity {
    Error,
    Warning,
    Info,
    Success,
}
```

---

## 3. Review & Commenting Entities

### 3.1 Comment

**Purpose**: Store inline code review comments

**Fields**:
- `id`: String (UUID v4)
- `file_path`: String (relative to repository root)
  - Validation: Must exist in repository
  - Constraints: UTF-8, max 4096 chars
- `line_number`: u32 (1-based line number)
  - Validation: Must exist in file
- `content`: String (comment text)
  - Constraints: UTF-8, max 10,000 characters
- `author`: String (comment author name)
- `created_at`: DateTime (ISO 8601)
- `updated_at`: DateTime (ISO 8601)
- `status`: CommentStatus (draft/submitted/rejected)
- `parent_id`: Option<String> (UUID of parent comment for threads)
- `tags`: Vec<String> (tag IDs applied to comment)

**Validation Rules**:
- File path must be canonicalized and sanitized
- Line number must be > 0
- Content cannot be empty
- Timestamps must be valid and timezone-aware

**State Transitions**:
- `draft` → `submitted` (when pushed to remote)
- `submitted` → `rejected` (if remote rejects)
- `rejected` → `draft` (moved back to local)

**Enums**:
```rust
enum CommentStatus {
    Draft,
    Submitted,
    Rejected,
}
```

### 3.2 Tag

**Purpose**: Categorize and label comments for quick access

**Fields**:
- `id`: String (UUID v4)
- `label`: String (display name)
  - Constraints: UTF-8, max 64 characters
  - Unique: Case-insensitive within repository
- `color`: String (hex color code)
  - Format: #RRGGBB or #RRGGBBAA
- `description`: Option<String> (optional description)
  - Constraints: Max 500 characters
- `usage_count`: u32 (cached count for performance)
- `created_at`: DateTime
- `updated_at`: DateTime

**Validation Rules**:
- Label cannot be empty
- Color must be valid hex code
- Label must be unique per repository (case-insensitive)

---

## 4. Task & Progress Entities

### 4.1 Task

**Purpose**: Track review tasks and PRs awaiting review

**Fields**:
- `id`: String (UUID v4)
- `title`: String (task title)
  - Constraints: UTF-8, max 256 characters
- `description`: Option<String> (task description)
  - Constraints: Max 10,000 characters
- `status`: TaskStatus (active/pending/completed/blocked)
- `priority`: u32 (1-5, 1 = highest)
- `assignee`: Option<String> (reviewer name)
- `created_at`: DateTime
- `updated_at`: DateTime
- `due_date`: Option<DateTime> (optional deadline)
- `metadata`: HashMap<String, String> (extensible metadata)

**Validation Rules**:
- Title cannot be empty
- Priority must be 1-5
- Status must be valid enum value
- Metadata keys: max 64 chars, values: max 1024 chars

**State Transitions**:
- `pending` → `active` (when started)
- `active` → `completed` (when finished)
- `active` → `blocked` (when blocked)
- `blocked` → `active` (when unblocked)

**Enums**:
```rust
enum TaskStatus {
    Active,
    Pending,
    Completed,
    Blocked,
}
```

### 4.2 ReviewStats

**Purpose**: Aggregate statistics for current review session

**Fields**:
- `total_files`: u32 (total files in diff)
- `reviewed_files`: u32 (files with comments)
- `pending_files`: u32 (files without comments)
- `total_comments`: u32 (total inline comments)
- `severe_issues`: u32 (ERROR/WARNING severity comments)
- `completion_percentage`: f32 (0.0-100.0)
- `estimated_time_remaining`: Option<u32> (minutes)
- `files_per_hour`: f32 (calculated review velocity)

**Relationships**:
- Derived from Comment and Task entities
- Calculated in real-time

---

## 5. Insights & Analysis Entities

### 5.1 HeatmapItem

**Purpose**: Architectural impact visualization data

**Fields**:
- `file_path`: String (relative path)
  - Constraints: UTF-8, max 4096 characters
- `impact_score`: f32 (0.0-1.0, higher = more impactful)
- `churn_score`: f32 (0.0-1.0, recent change frequency)
- `complexity_score`: f32 (0.0-1.0, code complexity)
- `change_frequency`: u32 (commits in last 30 days)
- `lines_of_code`: u32 (approximate LOC)
- `category`: HeatmapCategory (high/medium/low impact)

**Relationships**:
- Calculated from git history and tree-sitter analysis

**Enums**:
```rust
enum HeatmapCategory {
    High,
    Medium,
    Low,
}
```

### 5.2 ChecklistItem

**Purpose**: Smart review checklist based on file types

**Fields**:
- `id`: String (UUID v4)
- `description`: String (check description)
  - Constraints: UTF-8, max 512 characters
- `category`: ChecklistCategory (security/performance/style/architecture)
- `severity`: Severity (importance level)
- `applicable_file_types`: Vec<String> (e.g., ["java", "xml"])
- `applicable_patterns`: Vec<String> (regex patterns that trigger this check)
- `is_checked`: Boolean (user checked off)
- `is_auto_checkable`: Boolean (can be automatically verified)
- `related_file`: Option<String> (file path this check applies to)

**Validation Rules**:
- Description cannot be empty
- Applicable patterns must be valid regex (if provided)
- File types must be lowercase extensions

**Enums**:
```rust
enum ChecklistCategory {
    Security,
    Performance,
    Style,
    Architecture,
    Testing,
    Documentation,
}
```

### 5.3 BlameInfo

**Purpose**: Git blame information for a file

**Fields**:
- `lines`: Vec<BlameLine> (line-by-line blame data)
- `file_path`: String (relative path)

**Relationships**:
- Computed from git blame operation

**Nested Structure**:
```rust
struct BlameLine {
    line_number: u32,
    content: String,
    commit_oid: String,
    commit_message: String,
    author_name: String,
    author_email: String,
    committer_name: String,
    committer_email: String,
    commit_date: DateTime,
}
```

---

## 6. Quality & CI/CD Entities

### 6.1 QualityGate

**Purpose**: Status of external quality checks

**Fields**:
- `name`: String (gate name, e.g., "CI Pipeline", "Test Coverage")
- `status`: QualityGateStatus (passing/failing/pending/unknown)
- `details`: Option<String> (detailed status message)
- `last_checked`: DateTime
- `url`: Option<String> (link to external system)
- `metadata`: HashMap<String, String> (additional gate data)

**Validation Rules**:
- Status must be valid enum value
- Name cannot be empty

**Enums**:
```rust
enum QualityGateStatus {
    Passing,
    Failing,
    Pending,
    Unknown,
}
```

---

## 7. Search & Configuration Entities

### 7.1 SearchResult

**Purpose**: Search results from repository

**Fields**:
- `type`: SearchResultType (file/symbol/commit/command)
- `file_path`: Option<String> (file path if applicable)
- `line_number`: Option<u32> (line number if applicable)
- `content`: String (matched content or description)
- `highlight`: Option<String> (highlighted snippet)
- `score`: f32 (relevance score 0.0-1.0)

**Enums**:
```rust
enum SearchResultType {
    File,
    Symbol,
    Commit,
    Command,
}
```

### 7.2 ReviewTemplate

**Purpose**: Canned responses for common review comments

**Fields**:
- `id`: String (UUID v4)
- `name`: String (template name)
  - Constraints: UTF-8, max 128 characters
- `content`: String (template text with placeholders)
  - Constraints: UTF-8, max 10,000 characters
- `placeholders`: Vec<String> (placeholder variables)
- `category`: Option<String> (e.g., "style", "architecture")
- `usage_count`: u32 (cached count)
- `created_at`: DateTime
- `updated_at`: DateTime

**Validation Rules**:
- Name cannot be empty
- Content cannot be empty
- Placeholders must be in format `{{placeholder_name}}`

---

## 8. Persistence Layer

### 8.1 Local Database Schema

**Database**: SQLite (hyper_review.db)

**Tables**:

1. **repos**
   - PRIMARY KEY: `path`
   - Columns: `current_branch`, `last_opened`, `head_commit`, `remote_url`, `is_active`

2. **branches**
   - PRIMARY KEY: `name`, `repo_path` (composite)
   - Columns: `is_current`, `is_remote`, `upstream`, `last_commit`, `last_commit_message`, `last_commit_author`, `last_commit_date`

3. **comments**
   - PRIMARY KEY: `id` (UUID)
   - Indexes: `file_path`, `created_at`, `author`
   - Columns: `file_path`, `line_number`, `content`, `author`, `created_at`, `updated_at`, `status`, `parent_id`, `tags`

4. **tags**
   - PRIMARY KEY: `id` (UUID), `repo_path` (composite)
   - Indexes: `label`
   - Columns: `label`, `color`, `description`, `usage_count`, `created_at`, `updated_at`

5. **tasks**
   - PRIMARY KEY: `id` (UUID)
   - Indexes: `status`, `assignee`, `due_date`
   - Columns: `title`, `description`, `status`, `priority`, `assignee`, `created_at`, `updated_at`, `due_date`, `metadata`

6. **checklist_templates**
   - PRIMARY KEY: `id` (UUID)
   - Indexes: `category`
   - Columns: `description`, `category`, `severity`, `applicable_file_types`, `applicable_patterns`, `is_auto_checkable`

7. **review_templates**
   - PRIMARY KEY: `id` (UUID)
   - Indexes: `category`
   - Columns: `name`, `content`, `placeholders`, `category`, `usage_count`, `created_at`, `updated_at`

8. **search_cache**
   - PRIMARY KEY: `id` (UUID)
   - Columns: `query`, `results`, `created_at`, `expires_at`

### 8.2 Caching Strategy

**In-Memory Caches** (LRU with size limits):

1. **Repository Cache**: 10 repositories max
   - Key: Repository path
   - Value: GitRepository struct

2. **Diff Cache**: 100 diffs max
   - Key: File path + base OID + head OID
   - Value: Vec<DiffLine>

3. **Blame Cache**: 50 files max
   - Key: File path + commit OID
   - Value: Vec<BlameLine>

4. **Analysis Cache**: 200 files max
   - Key: File path + commit OID
   - Value: Complexity metrics, AST data

**Cache Invalidation**:
- Repository cache: On repository close
- Diff/blame cache: On file modification
- Analysis cache: On code analysis update
- Time-based expiration: 1 hour default

---

## 9. Validation Rules Summary

### 9.1 Path Validation
- Must be UTF-8 encoded
- Max length: 4096 characters
- Must be canonicalized (no `..`, `.`, redundant slashes)
- Must start with repository root

### 9.2 Content Validation
- All text content: UTF-8 encoded
- Max line length: 10,000 characters (prevent DoS)
- Max comment length: 10,000 characters
- Max template length: 10,000 characters

### 9.3 Business Rule Validation
- Line numbers: 1-based, must exist in file
- Timestamps: ISO 8601 format, timezone-aware
- IDs: UUID v4 format (except Git OIDs which are SHA-1 hex)
- Email addresses: RFC 5322 compliant (for author/committer)
- File types: Lowercase, 1-10 characters (e.g., "java", "xml")

### 9.4 Security Constraints
- No HTML/script injection in comments/templates
- File paths must be within repository (sandboxing)
- Template placeholders must be validated
- No SQL injection (use parameterized queries)

---

## 10. Data Relationships Diagram

```
Repository (1) ←→ (N) Branch
Repository (1) ←→ (N) File
File (1) ←→ (N) Comment
File (1) ←→ (N) BlameInfo
File (1) ←→ (N) HeatmapItem

Comment (N) ←→ (1) Tag (through join table)
Task (1) ←→ (N) Comment (implicit)

Repository (1) ←→ (N) SearchResult
Repository (1) ←→ (N) QualityGate
Repository (1) ←→ (N) ReviewTemplate
Repository (1) ←→ (N) Tag

ChecklistItem (N) ←→ (N) File (through applicable_file_types)
```

---

## 11. Performance Considerations

### 11.1 Database Indexes

**Critical Indexes**:
- comments(file_path, line_number)
- comments(created_at)
- tags(label)
- tasks(status, priority)
- search_cache(query, expires_at)

**Query Optimization**:
- Use prepared statements for common queries
- Batch inserts/updates for multiple records
- Pagination for large result sets (LIMIT/OFFSET)

### 11.2 Serialization

**JSON Serialization**:
- Use `serde_json` with custom serialization
- Include version field for backward compatibility
- Compress large payloads (gzip)

**IPC Transfer**:
- Max payload size: 10MB per command
- Stream large datasets (diff with 10k+ lines)
- Use base64 for binary data (file contents)

---

**Data Model Version**: 1.0.0
**Last Updated**: 2025-12-13
**Compatibility**: Backward compatible for minor versions, major version bumps for breaking changes
