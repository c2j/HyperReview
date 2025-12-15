# IPC Commands Contract

**Date**: 2025-12-14
**Feature**: 002-frontend-backend-integration

## Overview

This document defines the IPC (Inter-Process Communication) contract between the React frontend and Rust backend via Tauri. All commands follow the pattern `invoke('command_name', params)` and return `Result<T, String>`.

## Repository Management Commands

### open_repo_dialog

**Purpose**: Opens a native file dialog for repository selection

```typescript
type OpenRepoDialogResult = string | null;  // Selected path or null if cancelled

invoke('open_repo_dialog'): Promise<OpenRepoDialogResult>
```

**Parameters**: None

**Returns**: Selected repository path, or null if user cancelled

**Error Cases**:
- User cancelled dialog: returns `null`
- Dialog system error: returns error string

---

### get_recent_repos

**Purpose**: Retrieves list of recently opened repositories

```typescript
interface Repo {
  path: string;
  current_branch: string;
  last_opened: string;
  head_commit: string;
  remote_url?: string;
  is_active: boolean;
}

invoke('get_recent_repos'): Promise<Repo[]>
```

**Parameters**: None

**Returns**: Array of repository metadata

**Error Cases**:
- Database error: returns error string

---

### get_branches

**Purpose**: Gets all branches (local and remote) for current repository

```typescript
interface Branch {
  name: string;
  is_current: boolean;
  is_remote: boolean;
  upstream?: string;
  last_commit: string;
  last_commit_message: string;
  last_commit_author: string;
  last_commit_date: string;
}

invoke('get_branches'): Promise<Branch[]>
```

**Parameters**: None

**Returns**: Array of branch information

**Error Cases**:
- No repository loaded: returns error string
- Git operation error: returns error string

---

### load_repo

**Purpose**: Loads a repository into memory and stores metadata

```typescript
invoke('load_repo', { path: string }): Promise<Repo>
```

**Parameters**:
- `path` (string): Absolute path to repository

**Returns**: Repository metadata

**Error Cases**:
- Invalid path: returns error string
- Not a git repository: returns error string
- Permission denied: returns error string

---

## Code Review Commands

### get_file_diff

**Purpose**: Gets diff for a file between commits

```typescript
interface DiffLine {
  old_line_number?: number;
  new_line_number?: number;
  content: string;
  line_type: 'Added' | 'Removed' | 'Context' | 'Header';
  severity?: 'Error' | 'Warning' | 'Info' | 'Success';
  message?: string;
  hunk_header?: string;
}

invoke('get_file_diff', {
  file_path: string,
  old_commit?: string,
  new_commit?: string
}): Promise<DiffLine[]>
```

**Parameters**:
- `file_path` (string): Path to file (relative to repository)
- `old_commit` (string, optional): Old commit hash or branch
- `new_commit` (string, optional): New commit hash or branch (default: HEAD)

**Returns**: Array of diff lines

**Error Cases**:
- File not found: returns error string
- Not a text file (binary): returns error string
- Invalid commit: returns error string

---

### add_comment

**Purpose**: Adds a review comment to a file

```typescript
interface Comment {
  id: string;
  file_path: string;
  line_number: number;
  content: string;
  author: string;
  created_at: string;
  updated_at: string;
  status: 'Draft' | 'Submitted' | 'Rejected';
  parent_id?: string;
  tags: string[];
}

invoke('add_comment', {
  file_path: string,
  line_number: number,
  content: string
}): Promise<Comment>
```

**Parameters**:
- `file_path` (string): Path to file
- `line_number` (number): Line number to comment on
- `content` (string): Comment content (1-5000 chars)

**Returns**: Created comment object

**Error Cases**:
- Invalid file path: returns error string
- Invalid line number: returns error string
- Empty content: returns error string
- Database error: returns error string

---

## Task Management Commands

### get_tasks

**Purpose**: Gets all tasks for current review

```typescript
interface Task {
  id: string;
  title: string;
  description?: string;
  status: 'Active' | 'Pending' | 'Completed' | 'Blocked';
  priority: number;
  assignee?: string;
  created_at: string;
  updated_at: string;
  due_date?: string;
  metadata: Record<string, string>;
}

invoke('get_tasks'): Promise<Task[]>
```

**Parameters**: None

**Returns**: Array of task objects

**Error Cases**:
- Database error: returns error string

---

### get_review_stats

**Purpose**: Gets review statistics

```typescript
interface ReviewStats {
  total_files: number;
  reviewed_files: number;
  pending_files: number;
  total_comments: number;
  severe_issues: number;
  completion_percentage: number;
  estimated_time_remaining?: number;
  files_per_hour: number;
}

invoke('get_review_stats'): Promise<ReviewStats>
```

**Parameters**: None

**Returns**: Review statistics object

**Error Cases**:
- Database error: returns error string

---

### get_quality_gates

**Purpose**: Gets quality gate status from CI/CD

```typescript
interface QualityGate {
  name: string;
  status: 'Passing' | 'Failing' | 'Pending' | 'Unknown';
  details?: string;
  last_checked: string;
  url?: string;
  metadata: Record<string, string>;
}

invoke('get_quality_gates'): Promise<QualityGate[]>
```

**Parameters**: None

**Returns**: Array of quality gate objects

**Error Cases**:
- Network error: returns error string
- CI/CD system error: returns error string

---

### get_review_templates

**Purpose**: Gets all review templates

```typescript
interface ReviewTemplate {
  id: string;
  name: string;
  content: string;
  placeholders: string[];
  category?: string;
  usage_count: number;
  created_at: string;
  updated_at: string;
}

invoke('get_review_templates'): Promise<ReviewTemplate[]>
```

**Parameters**: None

**Returns**: Array of template objects

**Error Cases**:
- Database error: returns error string

---

### create_template

**Purpose**: Creates a new review template

```typescript
invoke('create_template', {
  name: string,
  content: string
}): Promise<ReviewTemplate>
```

**Parameters**:
- `name` (string): Template name (1-100 chars)
- `content` (string): Template content with placeholders

**Returns**: Created template object

**Error Cases**:
- Empty name: returns error string
- Database error: returns error string

---

## Analysis & Insights Commands

### get_heatmap

**Purpose**: Gets file impact heatmap

```typescript
interface HeatmapItem {
  file_path: string;
  impact_score: number;
  churn_score: number;
  complexity_score: number;
  change_frequency: number;
  lines_of_code: number;
  category: 'High' | 'Medium' | 'Low';
}

invoke('get_heatmap'): Promise<HeatmapItem[]>
```

**Parameters**: None

**Returns**: Array of heatmap items

**Error Cases**:
- Database error: returns error string

---

### get_checklist

**Purpose**: Gets smart checklist for a file

```typescript
interface ChecklistItem {
  id: string;
  description: string;
  category: 'Security' | 'Performance' | 'Style' | 'Architecture' | 'Testing' | 'Documentation';
  severity: 'Error' | 'Warning' | 'Info' | 'Success';
  applicable_file_types: string[];
  applicable_patterns: string[];
  is_checked: boolean;
  is_auto_checkable: boolean;
  related_file?: string;
}

invoke('get_checklist', {
  file_path: string
}): Promise<ChecklistItem[]>
```

**Parameters**:
- `file_path` (string): Path to file

**Returns**: Array of checklist items

**Error Cases**:
- File not found: returns error string

---

### get_blame

**Purpose**: Gets git blame information

```typescript
interface BlameLine {
  line_number: number;
  content: string;
  commit_oid: string;
  commit_message: string;
  author_name: string;
  author_email: string;
  committer_name: string;
  committer_email: string;
  commit_date: string;
}

interface BlameInfo {
  file_path: string;
  lines: BlameLine[];
}

invoke('get_blame', {
  file_path: string,
  commit?: string
}): Promise<BlameInfo>
```

**Parameters**:
- `file_path` (string): Path to file
- `commit` (string, optional): Commit to blame from (default: HEAD)

**Returns**: Blame information

**Error Cases**:
- File not found: returns error string
- Git operation error: returns error string

---

### analyze_complexity

**Purpose**: Analyzes code complexity metrics

```typescript
interface ComplexityMetrics {
  file_path: string;
  cyclomatic_complexity: number;
  cognitive_complexity: number;
  lines_of_code: number;
  function_count: number;
  class_count: number;
}

invoke('analyze_complexity', {
  file_path: string
}): Promise<ComplexityMetrics>
```

**Parameters**:
- `file_path` (string): Path to file

**Returns**: Complexity metrics

**Error Cases**:
- File not found: returns error string
- Unsupported file type: returns error string

---

### scan_security

**Purpose**: Scans file for security issues

```typescript
interface SecurityIssue {
  severity: 'Error' | 'Warning' | 'Info' | 'Success';
  message: string;
  line_number?: number;
  file_path: string;
  rule_id: string;
}

invoke('scan_security', {
  file_path: string
}): Promise<SecurityIssue[]>
```

**Parameters**:
- `file_path` (string): Path to file

**Returns**: Array of security issues

**Error Cases**:
- File not found: returns error string

---

## External Integration Commands

### submit_review

**Purpose**: Submits review to external system

```typescript
interface SubmitResult {
  success: boolean;
  message: string;
  external_id?: string;
  url?: string;
}

invoke('submit_review', {
  system: string,
  review_data: object
}): Promise<SubmitResult>
```

**Parameters**:
- `system` (string): Target system ('gitlab', 'gerrit', etc.)
- `review_data` (object): Review submission data

**Returns**: Submission result

**Error Cases**:
- Network error: returns error string
- Authentication error: returns error string
- Invalid system: returns error string

---

### sync_repo

**Purpose**: Syncs repository with remote

```typescript
interface SyncResult {
  success: boolean;
  message: string;
  commits_ahead?: number;
  commits_behind?: number;
}

invoke('sync_repo'): Promise<SyncResult>
```

**Parameters**: None

**Returns**: Synchronization result

**Error Cases**:
- No remote configured: returns error string
- Network error: returns error string
- Git operation error: returns error string

---

## Search & Configuration Commands

### search

**Purpose**: Searches repository

```typescript
interface SearchResult {
  result_type: 'File' | 'Symbol' | 'Commit' | 'Command';
  file_path?: string;
  line_number?: number;
  content: string;
  highlight?: string;
  score: number;
}

invoke('search', {
  query: string
}): Promise<SearchResult[]>
```

**Parameters**:
- `query` (string): Search query

**Returns**: Array of search results

**Error Cases**:
- Empty query: returns error string

---

### get_commands

**Purpose**: Gets available commands for command palette

```typescript
interface CommandInfo {
  id: string;
  name: string;
  description: string;
  category: string;
}

invoke('get_commands'): Promise<CommandInfo[]>
```

**Parameters**: None

**Returns**: Array of command information

**Error Cases**: None

---

### get_tags

**Purpose**: Gets all tags

```typescript
interface Tag {
  id: string;
  label: string;
  color: string;
  description?: string;
  usage_count: number;
  created_at: string;
  updated_at: string;
}

invoke('get_tags'): Promise<Tag[]>
```

**Parameters**: None

**Returns**: Array of tags

**Error Cases**:
- Database error: returns error string

---

### create_tag

**Purpose**: Creates a new tag

```typescript
invoke('create_tag', {
  label: string,
  color: string
}): Promise<Tag>
```

**Parameters**:
- `label` (string): Tag label (1-50 chars, unique)
- `color` (string): Hex color code (#RRGGBB)

**Returns**: Created tag object

**Error Cases**:
- Empty label: returns error string
- Invalid color: returns error string
- Duplicate label: returns error string
- Database error: returns error string

---

## Error Handling Pattern

All commands follow this error pattern:

```typescript
// Success case
await invoke('command_name', params)  // Returns T

// Error case
await invoke('command_name', params)  // Returns string (error message)

// Frontend handling
try {
  const result = await invoke('command_name', params);
  // Handle success
} catch (error) {
  // Handle error - display user-friendly message
  console.error('Command failed:', error);
  // Show toast/notification
}
```

## Performance Expectations

- **Repository operations**: < 2 seconds
- **Diff viewing**: < 1 second for files < 1000 lines
- **Comment operations**: < 200ms perceived latency
- **Search**: < 500ms for typical queries
- **Statistics**: < 1 second for calculation

All operations are tracked by backend performance monitoring (T090).

## Security Considerations

- All user input sanitized in backend
- Path validation prevents traversal attacks
- No direct file system access from frontend
- IPC allowlist in tauri.conf.json restricts commands
- Security audit framework in place (T097)

## Data Flow

```
Frontend Component
  ↓
TypeScript Interface Validation
  ↓
Tauri invoke()
  ↓
Rust Command Handler
  ↓
Business Logic + Validation
  ↓
SQLite/git2-rs
  ↓
Result Serialization
  ↓
Frontend State Update
```

## References

- Backend Implementation: `/Volumes/Raiden_C2J/Projects/Desktop_Projects/CR/HyperReview/src-tauri/src/commands.rs`
- API Documentation: `/Volumes/Raiden_C2J/Projects/Desktop_Projects/CR/HyperReview/src-tauri/docs/api.md`
- Data Model: [data-model.md](./data-model.md)
