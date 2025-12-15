# Data Model: Frontend-Backend Integration

**Date**: 2025-12-14
**Feature**: 002-frontend-backend-integration

## Core Entities

### 1. Repository
**Purpose**: Represents a Git repository being reviewed

```typescript
interface Repository {
  path: string;              // Absolute path to repository
  current_branch: string;     // Active branch name
  last_opened: string;        // ISO 8601 timestamp
  head_commit: string;        // Current HEAD commit hash
  remote_url?: string;        // Remote repository URL
  is_active: boolean;         // Is this the currently loaded repo?
}
```

**Validation**:
- `path`: Must exist and be a valid git repository
- `current_branch`: Must exist in repository
- `last_opened`: ISO 8601 format
- `is_active`: Only one repository can be active at a time

**State Transitions**:
- `Inactive` → `Active`: When user loads repository
- `Active` → `Inactive`: When user switches to different repository

---

### 2. Branch
**Purpose**: Represents a git branch within a repository

```typescript
interface Branch {
  name: string;               // Branch name
  is_current: boolean;        // Is this the active branch?
  is_remote: boolean;         // Is this a remote branch?
  upstream?: string;          // Upstream branch name
  last_commit: string;        // Latest commit hash
  last_commit_message: string;
  last_commit_author: string;
  last_commit_date: string;   // ISO 8601 timestamp
}
```

**Validation**:
- `name`: Cannot be empty, follows git branch naming
- `is_current`: Only one branch can be current per repo
- `last_commit_date`: ISO 8601 format

---

### 3. DiffLine
**Purpose**: Represents a single line in a code diff

```typescript
interface DiffLine {
  old_line_number?: number;   // Line number in old version
  new_line_number?: number;   // Line number in new version
  content: string;            // Line content
  line_type: 'Added' | 'Removed' | 'Context' | 'Header';
  severity?: 'Error' | 'Warning' | 'Info' | 'Success';
  message?: string;           // Static analysis message
  hunk_header?: string;       // Hunk context (e.g., @@ -1,3 +1,3 @@)
}
```

**Validation**:
- `line_type`: Must be one of the four enumerated values
- `old_line_number` and `new_line_number`: Optional, but at least one must be present
- `content`: Cannot be null (empty string is valid)

**Derived Fields**:
- `is_added`: `line_type === 'Added'`
- `is_removed`: `line_type === 'Removed'`
- `is_context`: `line_type === 'Context'`

---

### 4. Comment
**Purpose**: Represents a code review comment

```typescript
interface Comment {
  id: string;                 // UUID v4
  file_path: string;          // Relative path within repository
  line_number: number;        // Line number in diff
  content: string;            // Comment text
  author: string;             // User identifier from local config
  created_at: string;         // ISO 8601 timestamp
  updated_at: string;         // ISO 8601 timestamp
  status: 'Draft' | 'Submitted' | 'Rejected';
  parent_id?: string;         // For threaded comments
  tags: string[];             // Associated tag IDs
}
```

**Validation**:
- `id`: Must be valid UUID v4
- `file_path`: Must exist in current repository
- `line_number`: Must correspond to a line in the diff
- `content`: 1-5000 characters
- `author`: Must match current user identifier
- `created_at` <= `updated_at`

**State Transitions**:
- `Draft` → `Submitted`: When user submits comment
- `Submitted` → `Rejected`: When comment is rejected
- `Draft` → `Draft`: When editing draft comment

**Relationships**:
- `parent_id`: Optional self-reference for comment threads
- `tags`: Many-to-many with Tag entities

---

### 5. Task
**Purpose**: Represents a review task or to-do item

```typescript
interface Task {
  id: string;                 // UUID v4
  title: string;              // Task description
  description?: string;       // Detailed description
  status: 'Active' | 'Pending' | 'Completed' | 'Blocked';
  priority: number;           // 1-5 (5 = highest)
  assignee?: string;          // User identifier
  created_at: string;         // ISO 8601 timestamp
  updated_at: string;         // ISO 8601 timestamp
  due_date?: string;          // ISO 8601 timestamp
  metadata: Record<string, string>;  // Flexible key-value store
}
```

**Validation**:
- `title`: 1-200 characters
- `description`: 0-2000 characters (optional)
- `status`: Must be one of the four enumerated values
- `priority`: Integer 1-5
- `assignee`: Optional, must match user identifier if present
- `due_date`: Optional, must be ISO 8601 format, cannot be in past

**State Transitions**:
- `Pending` → `Active`: When work begins
- `Active` → `Completed`: When task is finished
- `Active` → `Blocked`: When task is blocked
- `Blocked` → `Active`: When blockage is resolved
- `Completed`: Terminal state

---

### 6. ReviewStats
**Purpose**: Aggregated statistics for a review session

```typescript
interface ReviewStats {
  total_files: number;        // Total files in review
  reviewed_files: number;     // Files with comments/changes reviewed
  pending_files: number;      // Files not yet reviewed
  total_comments: number;     // Total comments added
  severe_issues: number;      // Comments marked as severe
  completion_percentage: number;  // 0-100
  estimated_time_remaining?: number;  // Minutes
  files_per_hour: number;     // Review velocity
}
```

**Validation**:
- All numeric fields must be >= 0
- `completion_percentage`: 0-100
- `reviewed_files` <= `total_files`
- `pending_files` = `total_files` - `reviewed_files`

**Derived Fields**:
- `is_complete`: `completion_percentage === 100`
- `is_overdue`: `due_date` exists and is in past

---

### 7. HeatmapItem
**Purpose**: File impact analysis for prioritization

```typescript
interface HeatmapItem {
  file_path: string;          // Relative path
  impact_score: number;       // 0-100 (higher = more important)
  churn_score: number;        // 0-100 (frequency of changes)
  complexity_score: number;   // 0-100 (code complexity)
  change_frequency: number;   // Number of recent changes
  lines_of_code: number;      // Total LOC
  category: 'High' | 'Medium' | 'Low';  // Impact category
}
```

**Validation**:
- All numeric fields must be >= 0
- `impact_score`: 0-100
- `category`: Determined by `impact_score` thresholds

**Computation**:
- `category = 'High'` if `impact_score >= 70`
- `category = 'Medium'` if `impact_score >= 40`
- `category = 'Low'` otherwise

---

### 8. ChecklistItem
**Purpose**: Smart checklist for code review

```typescript
interface ChecklistItem {
  id: string;                 // UUID v4
  description: string;        // Checklist item text
  category: 'Security' | 'Performance' | 'Style' | 'Architecture' | 'Testing' | 'Documentation';
  severity: 'Error' | 'Warning' | 'Info' | 'Success';
  applicable_file_types: string[];  // e.g., ['.rs', '.ts', '.js']
  applicable_patterns: string[];    // Regex patterns
  is_checked: boolean;        // User checked this item?
  is_auto_checkable: boolean; // Can be auto-verified?
  related_file?: string;      // Optional file association
}
```

**Validation**:
- `description`: 1-500 characters
- `applicable_file_types`: Array of file extensions with leading dot
- `applicable_patterns`: Valid regex patterns
- `is_checked`: Boolean flag

---

### 9. BlameLine
**Purpose**: Git blame information for a line

```typescript
interface BlameLine {
  line_number: number;        // Line number in file
  content: string;            // Line content
  commit_oid: string;         // Commit hash
  commit_message: string;
  author_name: string;
  author_email: string;
  committer_name: string;
  committer_email: string;
  commit_date: string;        // ISO 8601 timestamp
}
```

**Validation**:
- `line_number`: Must be >= 1
- `commit_oid`: Valid git commit hash (40 hex characters)
- `commit_date`: ISO 8601 format

---

### 10. SearchResult
**Purpose**: Results from repository search

```typescript
interface SearchResult {
  result_type: 'File' | 'Symbol' | 'Commit' | 'Command';
  file_path?: string;         // For file/symbol results
  line_number?: number;       // For symbol/line results
  content: string;            // Matched content or description
  highlight?: string;         // Highlighted match
  score: number;              // Relevance score 0-100
}
```

**Validation**:
- `result_type`: Must be one of the four enumerated values
- `content`: Cannot be empty
- `score`: 0-100 (higher = more relevant)

---

### 11. Tag
**Purpose**: User-defined tags for organizing items

```typescript
interface Tag {
  id: string;                 // UUID v4
  label: string;              // Tag name
  color: string;              // Hex color code (#RRGGBB)
  description?: string;       // Optional description
  usage_count: number;        // How many items use this tag
  created_at: string;         // ISO 8601 timestamp
  updated_at: string;         // ISO 8601 timestamp
}
```

**Validation**:
- `label`: 1-50 characters, unique per user
- `color`: Valid hex color code
- `usage_count`: Must be >= 0

---

## Entity Relationships

```
Repository (1) → (many) Branch
Repository (1) → (many) Comment
Repository (1) → (many) Task
Repository (1) → (many) Tag
Repository (1) → (many) HeatmapItem

Branch (1) → (many) DiffLine (via repository file)
Comment (many) → (1) Repository
Comment (many) → (1) DiffLine
Comment (many) → (many) Tag (via comment.tags)

Task (many) → (1) Repository
Task (many) → (1) User (assignee)

Tag (many) → (many) Comment (via comment.tags)
Tag (many) → (many) Repository
```

## Data Flow

```
User Action → React Component → Tauri IPC → Rust Command → SQLite/git2-rs
                                                    ↓
React State ← TypeScript Interface ← Serialized Result ← Database Response
```

## Validation Strategy

### Frontend Validation
- Input sanitization before IPC calls
- Type checking with TypeScript
- UI-level validation (required fields, formats)

### Backend Validation
- Path validation (prevent traversal attacks)
- Input sanitization (SQL injection prevention)
- Business rule validation (repository exists, branch valid)
- Data integrity checks

### Database Constraints
- Foreign key relationships
- Unique constraints (e.g., tag labels per user)
- Check constraints (e.g., priority range)

## Performance Considerations

### Caching Strategy
- **Backend**: LRU cache for diffs (T014)
- **Frontend**: Cache recently viewed files
- **Database**: Indexed queries for frequently accessed data

### Lazy Loading
- File tree: Load on-demand
- Commit history: Paginate
- Diff lines: Virtual scrolling for large files
- Search results: Limit and paginate

### Memory Management
- Diff cache: Evict old entries (LRU)
- Frontend: Virtualize large lists
- Database: Vacuum periodically
- Backend: Monitor memory usage (T092)

## Data Migration

### Schema Evolution
- Version field in database metadata
- Migration scripts for schema changes
- Backward compatibility for 1 version
- Data export/import for major version bumps

### Default Data
- Pre-populate review templates
- Create default tags
- Set up sample checklist items
- Initialize user profile (name, email)

## Security Model

### Data Protection
- Local SQLite encryption (T096)
- No sensitive data in logs
- Secure credential storage (T070)
- Path validation (T015)

### Access Control
- Single user per installation (simplified)
- File system permissions respected
- No network access required (offline-first)
- Input sanitization at all layers

## References

- Backend Models: `/Volumes/Raiden_C2J/Projects/Desktop_Projects/CR/HyperReview/src-tauri/src/models.rs`
- SQLite Schema: `/Volumes/Raiden_C2J/Projects/Desktop_Projects/CR/HyperReview/src-tauri/src/storage/sqlite.rs`
- API Documentation: `/Volumes/Raiden_C2J/Projects/Desktop_Projects/CR/HyperReview/src-tauri/docs/api.md`
