# HyperReview IPC API Documentation

This document describes all available IPC (Inter-Process Communication) commands exposed by the HyperReview backend to the frontend.

## Overview

HyperReview uses Tauri IPC for communication between the Rust backend and the React frontend. All commands are asynchronous and return Result<T, String> where T is the success type or String is an error message.

## Command Categories

### Repository Management

#### open_repo_dialog()
Opens a repository selection dialog.

**Parameters**: None

**Returns**: `Option<String>` - Selected repository path or None if cancelled

**Example**:
```javascript
const repoPath = await invoke('open_repo_dialog');
```

---

#### get_recent_repos()
Gets list of recently opened repositories.

**Parameters**: None

**Returns**: `Vec<Repo>` - Array of repository metadata

**Example**:
```javascript
const repos = await invoke('get_recent_repos');
console.log(repos); // [{ path, current_branch, last_opened, ... }]
```

---

#### get_branches()
Gets list of all branches (local and remote) for the current repository.

**Parameters**: None

**Returns**: `Vec<Branch>` - Array of branch information

**Example**:
```javascript
const branches = await invoke('get_branches');
console.log(branches); // [{ name, is_current, is_remote, last_commit, ... }]
```

---

#### load_repo(path: string)
Loads a repository into memory and stores metadata.

**Parameters**:
- `path` (string): Absolute path to the repository

**Returns**: `Repo` - Repository metadata

**Example**:
```javascript
const repo = await invoke('load_repo', { path: '/path/to/repo' });
console.log(repo.current_branch); // 'main'
```

---

### Code Review

#### get_file_diff(file_path: string, old_commit?: string, new_commit?: string)
Gets diff for a file between commits with static analysis.

**Parameters**:
- `file_path` (string): Path to the file
- `old_commit` (string, optional): Old commit hash or branch
- `new_commit` (string, optional): New commit hash or branch

**Returns**: `Vec<DiffLine>` - Array of diff lines with analysis

**Example**:
```javascript
const diff = await invoke('get_file_diff', {
  file_path: 'src/main.rs',
  old_commit: 'HEAD~1',
  new_commit: 'HEAD'
});
console.log(diff); // [{ old_line_number, new_line_number, content, line_type, ... }]
```

---

#### add_comment(file_path: string, line_number: number, content: string)
Adds a review comment to a file.

**Parameters**:
- `file_path` (string): Path to the file
- `line_number` (number): Line number to comment on
- `content` (string): Comment content

**Returns**: `Comment` - Created comment object

**Example**:
```javascript
const comment = await invoke('add_comment', {
  file_path: 'src/main.rs',
  line_number: 42,
  content: 'Consider using a more descriptive variable name here.'
});
console.log(comment.id); // UUID of the comment
```

---

### Task Management

#### get_tasks()
Gets all tasks for the current review.

**Parameters**: None

**Returns**: `Vec<Task>` - Array of task objects

**Example**:
```javascript
const tasks = await invoke('get_tasks');
console.log(tasks); // [{ id, title, status, priority, ... }]
```

---

#### get_review_stats()
Gets review statistics and progress metrics.

**Parameters**: None

**Returns**: `ReviewStats` - Review statistics object

**Example**:
```javascript
const stats = await invoke('get_review_stats');
console.log(stats.completion_percentage); // 75.5
console.log(stats.files_per_hour); // 12.3
```

---

#### get_quality_gates()
Gets quality gate status from CI/CD systems.

**Parameters**: None

**Returns**: `Vec<QualityGate>` - Array of quality gate objects

**Example**:
```javascript
const gates = await invoke('get_quality_gates');
console.log(gates); // [{ name, status, details, ... }]
```

---

#### get_review_templates()
Gets all review templates.

**Parameters**: None

**Returns**: `Vec<ReviewTemplate>` - Array of template objects

**Example**:
```javascript
const templates = await invoke('get_review_templates');
console.log(templates); // [{ id, name, content, placeholders, ... }]
```

---

#### create_template(name: string, content: string)
Creates a new review template.

**Parameters**:
- `name` (string): Template name
- `content` (string): Template content with placeholders like {{file}}

**Returns**: `ReviewTemplate` - Created template object

**Example**:
```javascript
const template = await invoke('create_template', {
  name: 'Security Review',
  content: 'Please review {{file}} for security issues.'
});
console.log(template.placeholders); // ['file']
```

---

### Analysis & Insights

#### get_heatmap()
Gets file impact heatmap based on change frequency and complexity.

**Parameters**: None

**Returns**: `Vec<HeatmapItem>` - Array of heatmap items

**Example**:
```javascript
const heatmap = await invoke('get_heatmap');
console.log(heatmap); // [{ file_path, impact_score, category, ... }]
```

---

#### get_checklist(file_path: string)
Gets smart checklist for a specific file based on file type and patterns.

**Parameters**:
- `file_path` (string): Path to the file

**Returns**: `Vec<ChecklistItem>` - Array of checklist items

**Example**:
```javascript
const checklist = await invoke('get_checklist', {
  file_path: 'src/main.rs'
});
console.log(checklist); // [{ description, category, severity, ... get_blame(file_path: string, commit?: string)
Gets git blame information for a file.

**Parameters**:
- `file_path` (string): }]
```

---

#### Path to the file
- `commit` (string, optional): Commit to blame from (default: HEAD)

**Returns**: `BlameInfo` - Blame information with line details

**Example**:
```javascript
const blame = await invoke('get_blame', {
  file_path: 'src/main.rs'
});
console.log(blame.lines); // [{ line_number, author_name, commit_message, ... }]
```

---

#### analyze_complexity(file_path: string)
Analyzes code complexity metrics for a file.

**Parameters**:
- `file_path` (string): Path to the file

**Returns**: `ComplexityMetrics` - Complexity metrics object

**Example**:
```javascript
const metrics = await invoke('analyze_complexity', {
  file_path: 'src/main.rs'
});
console.log(metrics.cyclomatic_complexity); // 15
console.log(metrics.cognitive_complexity); // 23
```

---

#### scan_security(file_path: string)
Scans a file for security issues using pattern matching.

**Parameters**:
- `file_path` (string): Path to the file

**Returns**: `Vec<SecurityIssue>` - Array of security issues

**Example**:
```javascript
const issues = await invoke('scan_security', {
  file_path: 'src/main.rs'
});
console.log(issues); // [{ severity, message, rule_id, ... }]
```

---

### External Integration

#### submit_review(system: string, review_data: object)
Submits review comments to an external system (GitLab, Gerrit, etc.).

**Parameters**:
- `system` (string): Target system ('gitlab', 'gerrit', etc.)
- `review_data` (object): Review submission data

**Returns**: `SubmitResult` - Submission result

**Example**:
```javascript
const result = await invoke('submit_review', {
  system: 'gitlab',
  review_data: {
    project_id: '123',
    merge_request_id: 456,
    comments: [...] // Array of comment objects
  }
});
console.log(result.success); // true
console.log(result.url); // Link to external system
```

---

#### sync_repo()
Syncs the current repository with remote.

**Parameters**: None

**Returns**: `SyncResult` - Synchronization result

**Example**:
```javascript
const result = await invoke('sync_repo');
console.log(result.commits_ahead); // 2
console.log(result.commits_behind); // 0
```

---

### Search & Navigation

#### search(query: string)
Searches repository for files, symbols, and commits.

**Parameters**:
- `query` (string): Search query

**Returns**: `Vec<SearchResult>` - Array of search results

**Example**:
```javascript
const results = await invoke('search', { query: 'function_name' });
console.log(results); // [{ result_type, file_path, content, score, ... }]
```

---

#### get_commands()
Gets list of available commands for the command palette.

**Parameters**: None

**Returns**: `Vec<CommandInfo>` - Array of command information

**Example**:
```javascript
const commands = await invoke('get_commands');
console.log(commands); // [{ id, name, description, category, ... }]
```

---

#### get_tags()
Gets all tags for the current repository.

**Parameters**: None

**Returns**: `Vec<Tag>` - Array of tag objects

**Example**:
```javascript
const tags = await invoke('get_tags');
console.log(tags); // [{ id, label, color, usage_count, ... }]
```

---

#### create_tag(label: string, color: string)
Creates a new tag.

**Parameters**:
- `label` (string): Tag label
- `color` (string): Tag color (hex or named)

**Returns**: `Tag` - Created tag object

**Example**:
```javascript
const tag = await invoke('create_tag', {
  label: 'security',
  color: '#ff0000'
});
console.log(tag.id); // UUID of the tag
```

---

## Data Models

### Repo
```typescript
interface Repo {
  path: string;
  current_branch: string;
  last_opened: string;
  head_commit: string;
  remote_url?: string;
  is_active: boolean;
}
```

### Branch
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
```

### DiffLine
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
```

### Comment
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
```

### Task
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
```

### ReviewStats
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
```

### QualityGate
```typescript
interface QualityGate {
  name: string;
  status: 'Passing' | 'Failing' | 'Pending' | 'Unknown';
  details?: string;
  last_checked: string;
  url?: string;
  metadata: Record<string, string>;
}
```

### ReviewTemplate
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
```

### HeatmapItem
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
```

### ChecklistItem
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
```

### BlameInfo
```typescript
interface BlameInfo {
  file_path: string;
  lines: BlameLine[];
}

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
```

### ComplexityMetrics
```typescript
interface ComplexityMetrics {
  file_path: string;
  cyclomatic_complexity: number;
  cognitive_complexity: number;
  lines_of_code: number;
  function_count: number;
  class_count: number;
}
```

### SecurityIssue
```typescript
interface SecurityIssue {
  severity: 'Error' | 'Warning' | 'Info' | 'Success';
  message: string;
  line_number?: number;
  file_path: string;
  rule_id: string;
}
```

### SearchResult
```typescript
interface SearchResult {
  result_type: 'File' | 'Symbol' | 'Commit' | 'Command';
  file_path?: string;
  line_number?: number;
  content: string;
  highlight?: string;
  score: number;
}
```

### Tag
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
```

### CommandInfo
```typescript
interface CommandInfo {
  id: string;
  name: string;
  description: string;
  category: string;
}
```

### SubmitResult
```typescript
interface SubmitResult {
  success: boolean;
  message: string;
  external_id?: string;
  url?: string;
}
```

### SyncResult
```typescript
interface SyncResult {
  success: boolean;
  message: string;
  commits_ahead?: number;
  commits_behind?: number;
}
```

## Error Handling

All commands return `Result<T, String>`. In case of an error, the String will contain a descriptive error message.

**Example Error Handling**:
```javascript
try {
  const diff = await invoke('get_file_diff', { file_path: 'src/main.rs' });
  console.log(diff);
} catch (error) {
  console.error('Failed to get diff:', error);
  // Handle error appropriately
}
```

## Performance Notes

- Commands should complete within 200ms for optimal user experience
- Large diffs are cached automatically
- Repository metadata is stored locally for fast retrieval
- Search results are limited to prevent overwhelming the UI

## Security Considerations

- All file paths are validated to prevent path traversal attacks
- User input is sanitized to prevent command injection
- Sensitive data should not be logged
- Review submissions require proper authentication to external systems

---

**Version**: 0.1.0
**Last Updated**: 2025-12-14
