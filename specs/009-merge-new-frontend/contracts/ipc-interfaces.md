# IPC Interface Contracts

**Feature**: 001-merge-new-frontend
**Date**: 2025-01-18
**Purpose**: Define Tauri IPC command contracts for merged frontend

## Overview

The merged frontend communicates with the Tauri backend via IPC (Inter-Process Communication). This document defines the command contracts, their signatures, error handling, and compatibility requirements.

## Command Categories

### 1. Repository Management Commands

#### `open_repo_dialog`
Opens the native OS directory picker for selecting a git repository.

```typescript
// Command signature
interface OpenRepoDialogArgs {}
interface OpenRepoDialogResult {
  path: string | null; // Absolute path of selected directory or null if cancelled
}

// Tauri command
#[tauri::command]
async fn open_repo_dialog() -> Result<String, String>

// Frontend invocation
const result = await ipcClient.invoke<OpenRepoDialogResult>('open_repo_dialog');
```

**Error Handling**:
- `null` result: User cancelled dialog
- `Error`: Failed to open dialog or selected path is invalid

**Compatibility**: ✅ **EXISTS** in existing backend

---

#### `get_recent_repos`
Returns a list of recently opened repositories from local configuration.

```typescript
// Command signature
interface GetRecentReposArgs {}
interface GetRecentReposResult {
  repos: Repo[];
}

interface Repo {
  path: string;       // Absolute path on disk
  branch: string;     // Current active branch
  lastOpened: string; // Human readable string (e.g., "2 mins ago")
}

// Tauri command
#[tauri::command]
async fn get_recent_repos() -> Result<Vec<RepoData>, String>

// Frontend invocation
const result = await ipcClient.invoke<GetRecentReposResult>('get_recent_repos');
```

**Error Handling**:
- Empty array: No recent repositories
- `Error`: Failed to read configuration

**Compatibility**: ✅ **EXISTS** in existing backend

---

#### `get_branches`
Returns the list of local and remote branches for the currently active repository.

```typescript
// Command signature
interface GetBranchesArgs {}
interface GetBranchesResult {
  branches: Branch[];
}

interface Branch {
  name: string;     // Branch name
  isCurrent: boolean; // Whether this is the currently checked out branch
  isLocal: boolean;   // Local or remote tracking branch
}

// Tauri command
#[tauri::command]
async fn get_branches() -> Result<Vec<BranchData>, String>

// Frontend invocation
const result = await ipcClient.invoke<GetBranchesResult>('get_branches');
```

**Error Handling**:
- Empty array: No repository loaded or no branches
- `Error`: Failed to fetch branches or repository is invalid

**Compatibility**: ✅ **EXISTS** in existing backend

---

#### `load_repo`
Loads a git repository from the specified path and initializes it as the active repository.

```typescript
// Command signature
interface LoadRepoArgs {
  path: string; // Absolute path to repository
}

interface LoadRepoResult {
  success: boolean;
  repoName: string | null;
  currentBranch: string | null;
  availableBranches: string[];
}

// Tauri command
#[tauri::command]
async fn load_repo(path: String) -> Result<LoadRepoData, String>

// Frontend invocation
const result = await ipcClient.invoke<LoadRepoResult>('load_repo', { path });
```

**Error Handling**:
- `success: false`: Path is not a valid git repository
- `Error`: Repository loading failed (corrupt, permission denied)

**Compatibility**: ✅ **EXISTS** in existing backend

---

### 2. Review Workflow Commands

#### `get_file_diff`
Computes the diff for a specific file between base and head branches.

```typescript
// Command signature
interface GetFileDiffArgs {
  fileId: string;    // File path (relative to repository root)
  base?: string;   // Base branch or commit (default: from diff context)
  head?: string;   // Head branch or commit (default: from diff context)
}

interface GetFileDiffResult {
  lines: DiffLine[];
}

interface DiffLine {
  oldLineNumber?: number;
  newLineNumber?: number;
  content: string;
  type: 'added' | 'removed' | 'context' | 'header';
  severity?: 'ERROR' | 'WARNING' | 'INFO' | 'SUCCESS';
  message?: string; // Analysis message for the specific line
}

// Tauri command
#[tauri::command]
async fn get_file_diff(file_id: String, base: Option<String>, head: Option<String>) -> Result<Vec<DiffLineData>, String>

// Frontend invocation
const result = await ipcClient.invoke<GetFileDiffResult>('get_file_diff', { fileId, base, head });
```

**Error Handling**:
- Empty array: File not found or no changes
- `Error`: Failed to compute diff, file not in repository

**Compatibility**: ✅ **EXISTS** in existing backend

---

#### `get_complete_file_diff`
Computes the complete diff with additional metadata (complexity, churn, etc.).

```typescript
// Command signature
interface GetCompleteFileDiffArgs {
  fileId: string;
}

interface GetCompleteFileDiffResult {
  lines: DiffLine[];
  metadata: {
    complexity: number;      // Calculated complexity score
    churn: number;          // Recent change frequency
    blameInfo: BlameInfo;  // Git blame data
  };
}

interface BlameInfo {
  author: string;
  email: string;
  timestamp: number;
  commitHash: string;
  commitMessage: string;
}

// Tauri command
#[tauri::command]
async fn get_complete_file_diff(file_id: String) -> Result<CompleteDiffData, String>

// Frontend invocation
const result = await ipcClient.invoke<GetCompleteFileDiffResult>('get_complete_file_diff', { fileId });
```

**Error Handling**:
- `Error`: Failed to compute diff or analysis

**Compatibility**: ✅ **EXISTS** in existing backend

---

#### `add_comment`
Adds a review comment to a file or specific line.

```typescript
// Command signature
interface AddCommentArgs {
  taskId: string;          // Associated task or change ID
  filePath: string;        // File path
  lineNumber?: number;     // Line number (optional for file-level comments)
  content: string;         // Comment message
  type?: 'file-level' | 'line-level' | 'inline';
  severity?: 'INFO' | 'WARNING' | 'ERROR' | 'SUCCESS';
  tags?: string[];          // User-defined tags
}

interface AddCommentResult {
  commentId: string;        // Generated comment ID
  success: boolean;
}

// Tauri command
#[tauri::command]
async fn add_comment(
  task_id: String,
  file_path: String,
  line_number: Option<u32>,
  content: String,
  type: Option<String>,
  severity: Option<String>,
  tags: Option<Vec<String>>
) -> Result<CommentData, String>

// Frontend invocation
const result = await ipcClient.invoke<AddCommentResult>('add_comment', {
  taskId, filePath, lineNumber, content, type, severity, tags
});
```

**Error Handling**:
- `Error`: Failed to save comment (database error, validation error)

**Compatibility**: ✅ **EXISTS** in existing backend

---

#### `update_comment`
Updates an existing review comment.

```typescript
// Command signature
interface UpdateCommentArgs {
  commentId: string;
  content: string;
  severity?: 'INFO' | 'WARNING' | 'ERROR' | 'SUCCESS';
  tags?: string[];
}

interface UpdateCommentResult {
  success: boolean;
}

// Tauri command
#[tauri::command]
async fn update_comment(
  comment_id: String,
  content: String,
  severity: Option<String>,
  tags: Option<Vec<String>>
) -> Result<bool, String>

// Frontend invocation
const result = await ipcClient.invoke<UpdateCommentResult>('update_comment', {
  commentId, content, severity, tags
});
```

**Error Handling**:
- `Error`: Comment not found, update failed

**Compatibility**: ✅ **EXISTS** in existing backend

---

#### `delete_comment`
Deletes an existing review comment.

```typescript
// Command signature
interface DeleteCommentArgs {
  commentId: string;
}

interface DeleteCommentResult {
  success: boolean;
}

// Tauri command
#[tauri::command]
async fn delete_comment(comment_id: String) -> Result<bool, String>

// Frontend invocation
const result = await ipcClient.invoke<DeleteCommentResult>('delete_comment', { commentId });
```

**Error Handling**:
- `Error`: Comment not found, delete failed

**Compatibility**: ✅ **EXISTS** in existing backend

---

#### `get_comments`
Retrieves all comments for a task or change.

```typescript
// Command signature
interface GetCommentsArgs {
  taskId: string;
}

interface GetCommentsResult {
  comments: Comment[];
}

interface Comment {
  id: string;
  taskId: string;
  filePath: string;
  lineNumber?: number;
  content: string;
  severity: 'INFO' | 'WARNING' | 'ERROR' | 'SUCCESS';
  tags?: string[];
  isDraft: boolean;
  createdAt: number;
  updatedAt: number;
}

// Tauri command
#[tauri::command]
async fn get_comments(task_id: String) -> Result<Vec<CommentData>, String>

// Frontend invocation
const result = await ipcClient.invoke<GetCommentsResult>('get_comments', { taskId });
```

**Error Handling**:
- Empty array: No comments for this task
- `Error`: Task not found, database error

**Compatibility**: ✅ **EXISTS** in existing backend

---

### 3. Insights & Analysis Commands

#### `get_heatmap`
Returns architectural impact analysis data (churn and complexity).

```typescript
// Command signature
interface GetHeatmapArgs {}

interface GetHeatmapResult {
  items: HeatmapItem[];
}

interface HeatmapItem {
  id: string;        // File or module identifier
  name: string;      // Display name
  impact: 'high' | 'medium' | 'low';
  churn: number;      // Change frequency (recent commits)
  complexity: number;  // Code complexity score
  filePath?: string;  // File path (if applicable)
}

// Tauri command
#[tauri::command]
async fn get_heatmap() -> Result<Vec<HeatmapItemData>, String>

// Frontend invocation
const result = await ipcClient.invoke<GetHeatmapResult>('get_heatmap');
```

**Error Handling**:
- Empty array: No repository loaded or insufficient data
- `Error`: Analysis failed

**Compatibility**: ✅ **EXISTS** in existing backend

---

#### `get_file_tree`
Returns the file tree structure for the current repository.

```typescript
// Command signature
interface GetFileTreeArgs {}

interface GetFileTreeResult {
  tree: FileTreeNode[];
}

interface FileTreeNode {
  id: string;
  name: string;
  path: string;
  type: 'file' | 'directory';
  children?: FileTreeNode[];
  status?: 'unmodified' | 'modified' | 'added' | 'deleted';
}

// Tauri command
#[tauri::command]
async fn get_file_tree() -> Result<Vec<FileTreeNodeData>, String>

// Frontend invocation
const result = await ipcClient.invoke<GetFileTreeResult>('get_file_tree');
```

**Error Handling**:
- Empty tree: No repository loaded
- `Error`: Failed to read repository

**Compatibility**: ✅ **EXISTS** in existing backend

---

#### `get_checklist`
Returns a smart checklist generated based on file types in the current PR.

```typescript
// Command signature
interface GetChecklistArgs {}

interface GetChecklistResult {
  items: ChecklistItem[];
}

interface ChecklistItem {
  id: string;
  category: string;          // e.g., "Security", "Performance", "Best Practices"
  title: string;
  description: string;
  priority: 'high' | 'medium' | 'low';
  checked: boolean;
  applicableFiles: string[];
}

// Tauri command
#[tauri::command]
async fn get_checklist() -> Result<Vec<ChecklistItemData>, String>

// Frontend invocation
const result = await ipcClient.invoke<GetChecklistResult>('get_checklist');
```

**Error Handling**:
- Empty array: No files in current change
- `Error`: Failed to generate checklist

**Compatibility**: ✅ **EXISTS** in existing backend

---

#### `get_blame`
Fetches git blame information for the current file context.

```typescript
// Command signature
interface GetBlameArgs {
  fileId: string;
  lineNumber?: number;
}

interface GetBlameResult {
  info: BlameInfo;
}

interface BlameInfo {
  author: string;
  email: string;
  timestamp: number;
  commitHash: string;
  commitMessage: string;
}

// Tauri command
#[tauri::command]
async fn get_blame(file_id: String, line_number: Option<u32>) -> Result<BlameInfoData, String>

// Frontend invocation
const result = await ipcClient.invoke<GetBlameResult>('get_blame', { fileId, lineNumber });
```

**Error Handling**:
- `Error`: File not found, blame computation failed

**Compatibility**: ✅ **EXISTS** in existing backend

---

#### `read_file_content`
Reads the content of a file from the repository.

```typescript
// Command signature
interface ReadFileContentArgs {
  filePath: string;
}

interface ReadFileContentResult {
  content: string;
}

// Tauri command
#[tauri::command]
async fn read_file_content(file_path: String) -> Result<String, String>

// Frontend invocation
const result = await ipcClient.invoke<ReadFileContentResult>('read_file_content', { filePath });
```

**Error Handling**:
- `Error`: File not found, permission denied

**Compatibility**: ✅ **EXISTS** in existing backend

---

### 4. Remote Review Commands (Gerrit)

#### `gerrit_get_instances_simple`
Retrieves configured Gerrit server instances.

```typescript
// Command signature
interface GerritGetInstancesSimpleArgs {}

interface GerritGetInstancesSimpleResult {
  instances: GerritInstance[];
}

interface GerritInstance {
  id: string;
  name: string;
  url: string;
  username: string;
  isActive: boolean;
  lastConnected: number | null;
}

// Tauri command
#[tauri::command]
async fn gerrit_get_instances_simple() -> Result<Vec<GerritInstanceData>, String>

// Frontend invocation
const result = await ipcClient.invoke<GerritGetInstancesSimpleResult>('gerrit_get_instances_simple');
```

**Error Handling**:
- Empty array: No instances configured
- `Error`: Failed to read configuration

**Compatibility**: ✅ **EXISTS** in existing backend

---

#### `gerrit_create_instance_simple`
Creates or updates a Gerrit server instance.

```typescript
// Command signature
interface GerritCreateInstanceSimpleArgs {
  name: string;
  url: string;
  username: string;
  password: string;  // Sent to backend for secure storage
}

interface GerritCreateInstanceSimpleResult {
  instanceId: string;
  success: boolean;
}

// Tauri command
#[tauri::command]
async fn gerrit_create_instance_simple(
  name: String,
  url: String,
  username: String,
  password: String
) -> Result<GerritInstanceData, String>

// Frontend invocation
const result = await ipcClient.invoke<GerritCreateInstanceSimpleResult>(
  'gerrit_create_instance_simple',
  { name, url, username, password }
);
```

**Error Handling**:
- `Error`: Invalid URL, connection failed, instance already exists

**Compatibility**: ✅ **EXISTS** in existing backend

---

#### `gerrit_search_changes_simple`
Searches for changes (PRs/MRs) on a Gerrit server.

```typescript
// Command signature
interface GerritSearchChangesSimpleArgs {
  query: string;     // Search query (can be empty)
  status?: string;    // Optional status filter
  limit?: number;      // Optional limit (default: 20)
}

interface GerritSearchChangesSimpleResult {
  changes: GerritChange[];
}

interface GerritChange {
  id: string;
  title: string;
  author: string;
  status: string;
  createdAt: number;
  updatedAt: number;
  projectId: string;
}

// Tauri command
#[tauri::command]
async fn gerrit_search_changes_simple(
  query: String,
  status: Option<String>,
  limit: Option<u32>
) -> Result<Vec<GerritChangeData>, String>

// Frontend invocation
const result = await ipcClient.invoke<GerritSearchChangesSimpleResult>(
  'gerrit_search_changes_simple',
  { query, status, limit }
);
```

**Error Handling**:
- Empty array: No matching changes
- `Error`: Connection failed, search query invalid

**Compatibility**: ✅ **EXISTS** in existing backend

---

#### `gerrit_import_change_simple`
Imports a Gerrit change for local review.

```typescript
// Command signature
interface GerritImportChangeSimpleArgs {
  changeId: string;
  downloadFiles?: boolean; // Whether to download file contents (default: true)
}

interface GerritImportChangeSimpleResult {
  changeId: string;
  sessionId: string;    // Created review session ID
  files: ImportedFile[];
}

interface ImportedFile {
  id: string;
  path: string;
  status: string;
  isDownloaded: boolean;
}

// Tauri command
#[tauri::command]
async fn gerrit_import_change_simple(
  change_id: String,
  download_files: Option<bool>
) -> Result<GerritImportChangeData, String>

// Frontend invocation
const result = await ipcClient.invoke<GerritImportChangeSimpleResult>(
  'gerrit_import_change_simple',
  { changeId, downloadFiles: true }
);
```

**Error Handling**:
- `Error`: Change not found, import failed, network error

**Compatibility**: ✅ **EXISTS** in existing backend

---

### 5. Task Management Commands

#### `create_task`
Creates a new local review task.

```typescript
// Command signature
interface CreateTaskArgs {
  payload: {
    name: string;
    repoPath: string;
    baseRef: string;
    itemsText: string;  // Task definition in text format
  };
}

interface CreateTaskResult {
  taskId: string;
  success: boolean;
}

// Tauri command
#[tauri::command]
async fn create_task(payload: CreateTaskPayload) -> Result<TaskData, String>

// Frontend invocation
const result = await ipcClient.invoke<CreateTaskResult>('create_task', { payload });
```

**Error Handling**:
- `Error`: Invalid task definition, repository not found

**Compatibility**: ✅ **EXISTS** in existing backend

---

#### `list_tasks`
Lists local review tasks.

```typescript
// Command signature
interface ListTasksArgs {
  status?: 'all' | 'active' | 'pending' | 'completed';
}

interface ListTasksResult {
  tasks: LocalTask[];
}

interface LocalTask {
  id: string;
  title: string;
  status: 'active' | 'pending' | 'completed' | 'blocked';
  createdAt: number;
  updatedAt: number;
  priority?: 'high' | 'medium' | 'low';
}

// Tauri command
#[tauri::command]
async fn list_tasks(status: Option<String>) -> Result<Vec<TaskData>, String>

// Frontend invocation
const result = await ipcClient.invoke<ListTasksResult>('list_tasks', { status });
```

**Error Handling**:
- Empty array: No tasks matching filter
- `Error`: Database query failed

**Compatibility**: ✅ **EXISTS** in existing backend

---

### 6. Settings & Configuration Commands

#### `get_review_templates`
Returns predefined comment templates for code review.

```typescript
// Command signature
interface GetReviewTemplatesArgs {}

interface GetReviewTemplatesResult {
  templates: ReviewTemplate[];
}

interface ReviewTemplate {
  id: string;
  title: string;
  category: string;
  content: string;
  tags?: string[];
}

// Tauri command
#[tauri::command]
async fn get_review_templates() -> Result<Vec<ReviewTemplateData>, String>

// Frontend invocation
const result = await ipcClient.invoke<GetReviewTemplatesResult>('get_review_templates');
```

**Error Handling**:
- Empty array: No templates configured
- `Error`: Failed to read templates

**Compatibility**: ✅ **EXISTS** in existing backend

---

#### `create_template`
Creates a new review template.

```typescript
// Command signature
interface CreateTemplateArgs {
  title: string;
  category: string;
  content: string;
  tags?: string[];
}

interface CreateTemplateResult {
  templateId: string;
  success: boolean;
}

// Tauri command
#[tauri::command]
async fn create_template(
  title: String,
  category: String,
  content: String,
  tags: Option<Vec<String>>
) -> Result<TemplateData, String>

// Frontend invocation
const result = await ipcClient.invoke<CreateTemplateResult>('create_template', {
  title, category, content, tags
});
```

**Error Handling**:
- `Error`: Template creation failed, duplicate template

**Compatibility**: ✅ **EXISTS** in existing backend

---

### 7. Persistence Commands

#### `save_user_setting`
Saves a user setting to persistent storage.

```typescript
// Command signature
interface SaveUserSettingArgs {
  key: string;
  value: any;  // JSON-serializable value
}

interface SaveUserSettingResult {
  success: boolean;
}

// Tauri command
#[tauri::command]
async fn save_user_setting(key: String, value: JsonValue) -> Result<bool, String>

// Frontend invocation
const result = await ipcClient.invoke<SaveUserSettingResult>('save_user_setting', { key, value });
```

**Error Handling**:
- `Error`: Storage failed, invalid value

**Compatibility**: ✅ **EXISTS** in existing backend

---

#### `get_user_setting`
Retrieves a user setting from persistent storage.

```typescript
// Command signature
interface GetUserSettingArgs {
  key: string;
}

interface GetUserSettingResult {
  value: any | null;  // Returns null if not found
}

// Tauri command
#[tauri::command]
async fn get_user_setting(key: String) -> Result<Option<JsonValue>, String>

// Frontend invocation
const result = await ipcClient.invoke<GetUserSettingResult>('get_user_setting', { key });
```

**Error Handling**:
- `null` result: Setting not found
- `Error`: Storage read failed

**Compatibility**: ✅ **EXISTS** in existing backend

---

## Error Handling Convention

All Tauri commands follow the `Result<T, String>` pattern:

```typescript
// Success case
Ok(data)

// Error case
Err(error_message)
```

### Frontend Error Handling

```typescript
import { invoke } from '@tauri-apps/api/tauri';

async function invokeCommand<TArgs = any, TResult = any>(
  command: string,
  args?: TArgs
): Promise<TResult> {
  try {
    const result = await invoke<TResult>(command, args);
    return result;
  } catch (error) {
    const err = error as any;
    
    // Handle "command not found" errors
    if (err?.message?.includes('command not found') ||
        err?.message?.includes('handler not found')) {
      throw new IPCCommandNotFoundError(command, err.message);
    }
    
    // Handle other errors
    throw new IPCError(err.message, command);
  }
}

class IPCCommandNotFoundError extends IPCError {
  constructor(command: string, message: string) {
    super(`Command '${command}' not implemented: ${message}`, command);
    this.name = 'IPCCommandNotFoundError';
  }
}

class IPCError extends Error {
  constructor(message: string, public command: string) {
    super(message);
    this.name = 'IPCError';
  }
}
```

---

## Graceful Degradation

For commands that may not be available (e.g., new Remote features), use the graceul degradation pattern:

```typescript
// Check if command is available before invoking
async function conditionalInvoke<TResult>(
  command: string,
  args?: any,
  options: {
    fallback?: () => Promise<TResult>;
    defaultValue?: TResult;
    showMessage?: boolean;
  } = {}
): Promise<TResult> {
  try {
    return await ipcClient.invoke<TResult>(command, args);
  } catch (error) {
    const err = error as IPCError;
    
    // Check if command not found
    if (err instanceof IPCCommandNotFoundError) {
      if (options.fallback) {
        console.warn(`Using fallback for ${command}`);
        return await options.fallback();
      }
      
      if (options.defaultValue !== undefined) {
        console.warn(`Using default value for ${command}`);
        return options.defaultValue;
      }
      
      if (options.showMessage !== false) {
        showErrorToast(`Feature '${command}' is not available in this version`);
      }
      
      throw err;
    }
    
    // Re-throw other errors
    throw error;
  }
}

// Usage example
const branches = await conditionalInvoke(
  'get_branches',
  {},
  {
    fallback: async () => [],
    defaultValue: [],
    showMessage: true
  }
);
```

---

## Future Extensions

### CodeArts Integration (Not Yet Implemented)

These commands will be added in future features:

- `codearts_get_instances`: Similar to `gerrit_get_instances_simple`
- `codearts_create_instance`: Configure CodeArts server
- `codearts_search_changes`: Search CodeArts changes
- `codearts_import_change`: Import CodeArts change

### GitLab Integration (Not Yet Implemented)

These commands will be added in future features:

- `gitlab_get_instances`: Similar to `gerrit_get_instances_simple`
- `gitlab_create_instance`: Configure GitLab server
- `gitlab_search_changes`: Search GitLab MRs
- `gitlab_import_change`: Import GitLab MR

---

## Migration Compatibility

### From Existing Frontend to Merged Frontend

| Existing Command | Merged Command | Notes |
|-----------------|-----------------|-------|
| `get_branches` | `get_branches` | ✅ Same signature |
| `get_file_diff` | `get_file_diff` | ✅ Same signature |
| `add_comment` | `add_comment` | ✅ Same signature |
| `gerrit_get_instances_simple` | `gerrit_get_instances_simple` | ✅ Same signature |

All existing commands maintain compatibility. No breaking changes required.

### New Commands Added

| Command | Availability |
|----------|---------------|
| `get_review_templates` | ✅ Available |
| `create_template` | ✅ Available |
| `save_user_setting` | ✅ Available |
| `get_user_setting` | ✅ Available |

---

## Versioning

Current IPC Interface Version: **1.0**

Breaking changes will increment the major version (2.0, 3.0, etc.).
Non-breaking additions will increment the minor version (1.1, 1.2, etc.).
Bug fixes will increment the patch version (1.0.1, 1.0.2, etc.).
