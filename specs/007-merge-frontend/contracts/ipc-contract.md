# IPC Interface Contract

**Feature**: 007-merge-frontend
**Generated**: 2025-01-16
**Purpose**: Define Tauri IPC interface between frontend (React) and backend (Rust)

## Overview

This document defines the Inter-Process Communication (IPC) interface used by the HyperReview frontend to invoke Tauri backend commands. All IPC operations preserve existing implementations from the current frontend.

**Note**: This merge does not modify the Rust backend. The IPC interface remains unchanged. This contract documents the existing interface for reference during frontend merge.

## IPC Command Structure

### Invocation Pattern

```typescript
// Frontend invokes Rust commands via Tauri
const result = await invoke<ReturnType>('command_name', {
  arg1: value1,
  arg2: value2,
});

// Error handling
try {
  const result = await invoke('command_name', { args });
} catch (error) {
  // Error is thrown as string or structured error object
  console.error('IPC error:', error);
}
```

### Response Format

**Success**:
```typescript
interface SuccessResponse<T> {
  status: 'success';
  data: T;
}
```

**Error**:
```typescript
interface ErrorResponse {
  code: string;        // Error code (e.g., 'GIT_ERROR', 'IO_ERROR')
  message: string;     // Human-readable error message
}
```

## IPC Commands

### 1. Repository Management

#### 1.1 Open Repository Dialog

**Command**: `open_repo_dialog`
**Arguments**: None
**Returns**: `string | null` (absolute path or null if cancelled)

**Description**: Opens native OS directory picker for repository selection.

**Usage**:
```typescript
const repoPath = await invoke<string | null>('open_repo_dialog');
if (repoPath) {
  // Repository selected, load via load_repo command
}
```

**Errors**:
- `DIALOG_CANCELLED`: User cancelled dialog (returns null)

---

#### 1.2 Load Repository

**Command**: `load_repo`
**Arguments**:
```typescript
{
  path: string;  // Absolute path to repository
}
```
**Returns**: `Repository` object

**Description**: Loads repository metadata and validates repository.

**Usage**:
```typescript
const repo = await invoke<Repository>('load_repo', { path: repoPath });
// repo: { path, branch, lastOpened }
```

**Errors**:
- `INVALID_REPO`: Path is not a valid Git repository
- `ACCESS_DENIED`: Cannot read repository (permissions)

---

#### 1.3 Get Recent Repositories

**Command**: `get_recent_repos`
**Arguments**: None
**Returns**: `Repository[]`

**Description**: Returns list of recently opened repositories from local config.

**Usage**:
```typescript
const repos = await invoke<Repository[]>('get_recent_repos');
```

**Errors**: None (returns empty array if no repos)

---

#### 1.4 Get Branches

**Command**: `get_branches`
**Arguments**:
```typescript
{
  path: string;  // Repository path
}
```
**Returns**: `Branch[]`

**Description**: Returns list of local and remote branches for repository.

**Branch Interface**:
```typescript
interface Branch {
  name: string;           // Branch name
  isRemote: boolean;      // Whether remote branch
  isCurrent: boolean;     // Whether currently checked out
  lastCommit?: string;     // Last commit hash
}
```

**Usage**:
```typescript
const branches = await invoke<Branch[]>('get_branches', { path: repoPath });
```

**Errors**:
- `REPO_NOT_FOUND`: Repository not loaded

---

### 2. Diff Operations

#### 2.1 Get File Diff

**Command**: `get_file_diff`
**Arguments**:
```typescript
{
  path: string;        // Repository path
  base: string;        // Base revision (branch or commit)
  head: string;        // Head revision (branch or commit)
  filePath: string;    // File path relative to repo root
}
```
**Returns**: `DiffLine[]`

**Description**: Computes diff for specific file with optional static analysis annotations.

**DiffLine Interface**:
```typescript
interface DiffLine {
  oldLineNumber?: number;
  newLineNumber?: number;
  content: string;
  type: 'added' | 'removed' | 'context' | 'header';
  severity?: 'ERROR' | 'WARNING' | 'INFO' | 'SUCCESS';
  message?: string;
}
```

**Usage**:
```typescript
const diffLines = await invoke<DiffLine[]>('get_file_diff', {
  path: repoPath,
  base: 'master',
  head: 'feature/payment-retry',
  filePath: 'src/main/OrderService.java',
});
```

**Errors**:
- `FILE_NOT_FOUND`: File does not exist in revision
- `INVALID_REVISION`: Base or head revision invalid
- `DIFF_TOO_LARGE`: Diff exceeds size limit (virtual scrolling required)

---

#### 2.2 Get Blame Information

**Command**: `get_blame`
**Arguments**:
```typescript
{
  path: string;        // Repository path
  revision: string;    // Git revision
  filePath: string;    // File path
  line?: number;       // Optional specific line (0-based)
}
```
**Returns**: `BlameInfo | BlameInfo[]`

**Description**: Fetches git blame information for file or specific line.

**BlameInfo Interface**:
```typescript
interface BlameInfo {
  line: number;
  commitHash: string;
  author: string;
  authorEmail: string;
  timestamp: string;
  commitMessage: string;
}
```

**Usage**:
```typescript
// Blame for entire file
const blameInfo = await invoke<BlameInfo[]>('get_blame', {
  path: repoPath,
  revision: 'HEAD',
  filePath: 'src/main/OrderService.java',
});

// Blame for specific line
const blame = await invoke<BlameInfo>('get_blame', {
  path: repoPath,
  revision: 'HEAD',
  filePath: 'src/main/OrderService.java',
  line: 42,
});
```

**Errors**:
- `FILE_NOT_FOUND`: File does not exist
- `INVALID_REVISION`: Revision invalid

---

### 3. Gerrit Integration

#### 3.1 Import Gerrit Change

**Command**: `import_gerrit_change`
**Arguments**:
```typescript
{
  serverUrl: string;        // Gerrit server URL
  changeNumber: number;     // Change number
  username?: string;         // Optional username override
  password?: string;         // Optional password override
}
```
**Returns**: `GerritChange` object

**Description**: Imports Gerrit change with all metadata and file list.

**GerritChange Interface**:
```typescript
interface GerritChange {
  changeNumber: number;
  project: string;
  subject: string;
  status: 'NEW' | 'MERGED' | 'ABANDONED';
  owner: string;
  patchSetNumber: number;
  files: GerritFile[];
  isImported: boolean;
  unreadCount?: number;
}

interface GerritFile {
  path: string;
  status: 'ADDED' | 'MODIFIED' | 'DELETED' | 'RENAMED';
  oldPath?: string;  // For renamed files
  patchSet: number;
}
```

**Usage**:
```typescript
const change = await invoke<GerritChange>('import_gerrit_change', {
  serverUrl: 'https://gerrit.example.com',
  changeNumber: 12345,
  username: 'reviewer',
  password: 'token',
});
```

**Errors**:
- `GERRIT_CONNECTION_FAILED`: Cannot connect to Gerrit server
- `CHANGE_NOT_FOUND`: Change number does not exist
- `AUTH_FAILED`: Authentication failed
- `NETWORK_ERROR`: Network connectivity issue

---

#### 3.2 Get Gerrit Changes

**Command**: `get_gerrit_changes`
**Arguments**:
```typescript
{
  serverUrl: string;        // Gerrit server URL
  username?: string;         // Optional username override
  password?: string;         // Optional password override
  status?: string;          // Optional status filter ('NEW', 'MERGED', etc.)
  project?: string;         // Optional project filter
}
```
**Returns**: `GerritChange[]`

**Description**: Fetches list of Gerrit changes filtered by criteria.

**Usage**:
```typescript
const changes = await invoke<GerritChange[]>('get_gerrit_changes', {
  serverUrl: 'https://gerrit.example.com',
  status: 'NEW',
  project: 'my-project',
});
```

**Errors**:
- `GERRIT_CONNECTION_FAILED`: Cannot connect to Gerrit server
- `AUTH_FAILED`: Authentication failed
- `NETWORK_ERROR`: Network connectivity issue

---

#### 3.3 Submit Gerrit Review

**Command**: `submit_gerrit_review`
**Arguments**:
```typescript
{
  serverUrl: string;        // Gerrit server URL
  changeNumber: number;     // Change number
  patchSetNumber: number;   // Patch set number
  review: {
    label: number;          // Review score (-2, -1, 0, +1, +2)
    message: string;       // Review message
    comments?: ReviewComment[];  // Optional inline comments
  }
}
```
**Returns**: `{ success: boolean; reviewId?: string }`

**Description**: Submits review with optional inline comments to Gerrit.

**Usage**:
```typescript
const result = await invoke<{ success: boolean; reviewId?: string }>('submit_gerrit_review', {
  serverUrl: 'https://gerrit.example.com',
  changeNumber: 12345,
  patchSetNumber: 2,
  review: {
    label: +1,
    message: 'LGTM, looks good',
    comments: [
      {
        filePath: 'src/main/OrderService.java',
        line: 42,
        message: 'Consider using Optional here',
      },
    ],
  },
});
```

**Errors**:
- `GERRIT_CONNECTION_FAILED`: Cannot connect to Gerrit server
- `CHANGE_NOT_FOUND`: Change does not exist
- `AUTH_FAILED`: Authentication failed
- `INVALID_REVIEW`: Review score or message invalid
- `NETWORK_ERROR`: Network connectivity issue

---

### 4. Review Operations

#### 4.1 Get Review Statistics

**Command**: `get_review_stats`
**Arguments**:
```typescript
{
  taskId: string;      // Task identifier (local or remote)
  mode: 'local' | 'remote';
}
```
**Returns**: `ReviewStats`

**Description**: Returns aggregate statistics for current review session.

**ReviewStats Interface**:
```typescript
interface ReviewStats {
  totalFiles: number;
  reviewedFiles: number;
  totalLinesAdded: number;
  totalLinesRemoved: number;
  totalComments: number;
  severeIssues: number;  // Lines with ERROR severity
  warnings: number;      // Lines with WARNING severity
}
```

**Usage**:
```typescript
const stats = await invoke<ReviewStats>('get_review_stats', {
  taskId: 'task-123',
  mode: 'local',
});
```

**Errors**:
- `TASK_NOT_FOUND`: Task does not exist

---

#### 4.2 Add Review Comment

**Command**: `add_review_comment`
**Arguments**:
```typescript
{
  taskId: string;       // Task identifier
  filePath: string;     // File path
  lineNumber: number;    // Line number (0-based)
  content: string;      // Comment text
  status: 'draft' | 'submitted';
}
```
**Returns**: `{ commentId: string }`

**Description**: Saves review comment (draft or submitted).

**Usage**:
```typescript
const result = await invoke<{ commentId: string }>('add_review_comment', {
  taskId: 'task-123',
  filePath: 'src/main/OrderService.java',
  lineNumber: 42,
  content: 'Consider using Optional here',
  status: 'draft',
});
```

**Errors**:
- `TASK_NOT_FOUND`: Task does not exist
- `FILE_NOT_FOUND`: File does not exist in task
- `INVALID_LINE_NUMBER`: Line number out of range

---

#### 4.3 Get Review Comments

**Command**: `get_review_comments`
**Arguments**:
```typescript
{
  taskId: string;       // Task identifier
  filePath?: string;    // Optional file filter
}
```
**Returns**: `ReviewComment[]`

**Description**: Retrieves all comments for task or specific file.

**Usage**:
```typescript
// All comments for task
const comments = await invoke<ReviewComment[]>('get_review_comments', {
  taskId: 'task-123',
});

// Comments for specific file
const fileComments = await invoke<ReviewComment[]>('get_review_comments', {
  taskId: 'task-123',
  filePath: 'src/main/OrderService.java',
});
```

**Errors**:
- `TASK_NOT_FOUND`: Task does not exist

---

### 5. Configuration & Settings

#### 5.1 Get User Settings

**Command**: `get_user_settings`
**Arguments**: None
**Returns**: `UserSettings`

**Description**: Loads user preferences and configuration from storage.

**UserSettings Interface**:
```typescript
interface UserSettings {
  language: string;
  fontSize: number;
  ligatures: boolean;
  vimMode: boolean;
  theme: 'light' | 'dark';
  panels: {
    left: { width: number; visible: boolean };
    right: { width: number; visible: boolean };
  };
}
```

**Usage**:
```typescript
const settings = await invoke<UserSettings>('get_user_settings');
```

**Errors**: None (returns default settings if none saved)

---

#### 5.2 Save User Settings

**Command**: `save_user_settings`
**Arguments**:
```typescript
{
  settings: Partial<UserSettings>;
}
```
**Returns**: `{ success: boolean }`

**Description**: Saves user preferences to storage.

**Usage**:
```typescript
const result = await invoke<{ success: boolean }>('save_user_settings', {
  settings: {
    fontSize: 16,
    theme: 'dark',
  },
});
```

**Errors**:
- `INVALID_SETTINGS`: Settings validation failed

---

#### 5.3 Get Tags

**Command**: `get_tags`
**Arguments**: None
**Returns**: `Tag[]`

**Description**: Returns configured quick tags for code review.

**Tag Interface**:
```typescript
interface Tag {
  id: string;
  label: string;
  color: string;
  description?: string;
}
```

**Usage**:
```typescript
const tags = await invoke<Tag[]>('get_tags');
```

**Errors**: None (returns empty array if no tags configured)

---

#### 5.4 Save Tags

**Command**: `save_tags`
**Arguments**:
```typescript
{
  tags: Tag[];
}
```
**Returns**: `{ success: boolean }`

**Description**: Saves quick tag configuration.

**Usage**:
```typescript
const result = await invoke<{ success: boolean }>('save_tags', {
  tags: [
    {
      id: 'n-plus-1',
      label: 'N+1 Problem',
      color: '#ff0000',
      description: 'Select N+1 query detected',
    },
  ],
});
```

**Errors**:
- `INVALID_TAGS`: Tags validation failed

---

### 6. Quality & Analysis

#### 6.1 Get Quality Gates

**Command**: `get_quality_gates`
**Arguments**:
```typescript
{
  taskId: string;      // Task identifier
  mode: 'local' | 'remote';
}
```
**Returns**: `QualityGate[]`

**Description**: Checks status of CI pipelines, test coverage, security scanners.

**QualityGate Interface**:
```typescript
interface QualityGate {
  name: string;
  status: 'passed' | 'failed' | 'pending' | 'skipped';
  details?: string;
  url?: string;        // Link to CI/scan result
}
```

**Usage**:
```typescript
const gates = await invoke<QualityGate[]>('get_quality_gates', {
  taskId: 'task-123',
  mode: 'local',
});
```

**Errors**:
- `TASK_NOT_FOUND`: Task does not exist

---

#### 6.2 Get Heatmap

**Command**: `get_heatmap`
**Arguments**:
```typescript
{
  taskId: string;      // Task identifier
  mode: 'local' | 'remote';
}
```
**Returns**: `HeatmapItem[]`

**Description**: Returns architectural impact analysis data.

**HeatmapItem Interface**:
```typescript
interface HeatmapItem {
  filePath: string;
  impactScore: number;
  changeCount: number;
  riskLevel: 'low' | 'medium' | 'high';
}
```

**Usage**:
```typescript
const heatmap = await invoke<HeatmapItem[]>('get_heatmap', {
  taskId: 'task-123',
  mode: 'local',
});
```

**Errors**:
- `TASK_NOT_FOUND`: Task does not exist

---

#### 6.3 Get Checklist

**Command**: `get_checklist`
**Arguments**:
```typescript
{
  filePaths: string[];  // Modified files in PR
}
```
**Returns**: `ChecklistItem[]`

**Description**: Returns smart checklist generated based on file types.

**ChecklistItem Interface**:
```typescript
interface ChecklistItem {
  id: string;
  category: string;
  description: string;
  checked: boolean;
  priority: 'low' | 'medium' | 'high';
}
```

**Usage**:
```typescript
const checklist = await invoke<ChecklistItem[]>('get_checklist', {
  filePaths: [
    'src/main/OrderService.java',
    'src/test/OrderServiceTest.java',
  ],
});
```

**Errors**: None (returns empty array if no checklist items)

---

### 7. Command Palette

#### 7.1 Get Commands

**Command**: `get_commands`
**Arguments**:
```typescript
{
  query?: string;  // Optional search query
}
```
**Returns**: `SearchResult[]`

**Description**: Returns indexable items for command palette (files, symbols, commands).

**SearchResult Interface**:
```typescript
interface SearchResult {
  id: string;
  type: 'file' | 'symbol' | 'command' | 'setting';
  label: string;
  description?: string;
  icon?: string;
  action: () => void;  // Not serializable, client-side only
}
```

**Usage**:
```typescript
const results = await invoke<SearchResult[]>('get_commands', {
  query: 'open repo',
});
```

**Errors**: None (returns empty array if no results)

---

## Error Handling

### Error Codes

| Code | Description | Resolution |
|------|-------------|-------------|
| `DIALOG_CANCELLED` | User cancelled dialog operation | Handle gracefully (null return) |
| `INVALID_REPO` | Path is not a valid Git repository | Show error message to user |
| `ACCESS_DENIED` | Cannot read repository (permissions) | Show error, suggest checking permissions |
| `REPO_NOT_FOUND` | Repository not loaded | Show error, suggest loading repository |
| `FILE_NOT_FOUND` | File does not exist in revision | Show error, check file path |
| `INVALID_REVISION` | Base or head revision invalid | Show error, check revision name |
| `DIFF_TOO_LARGE` | Diff exceeds size limit | Suggest using virtual scrolling |
| `GERRIT_CONNECTION_FAILED` | Cannot connect to Gerrit server | Show error, check network/server |
| `CHANGE_NOT_FOUND` | Change does not exist | Show error, verify change number |
| `AUTH_FAILED` | Authentication failed | Show error, check credentials |
| `NETWORK_ERROR` | Network connectivity issue | Show error, check internet connection |
| `INVALID_REVIEW` | Review score or message invalid | Show error, validate input |
| `TASK_NOT_FOUND` | Task does not exist | Show error, check task ID |
| `INVALID_LINE_NUMBER` | Line number out of range | Show error, check line number |
| `INVALID_SETTINGS` | Settings validation failed | Show error, validate input |
| `INVALID_TAGS` | Tags validation failed | Show error, validate input |

### Error Response Format

```typescript
// Try-catch pattern
try {
  const result = await invoke('command_name', { args });
  // Handle success
} catch (error) {
  // Error is thrown as string
  const errorCode = error.code || 'UNKNOWN_ERROR';
  const errorMessage = error.message || 'An unknown error occurred';

  // Show user-friendly error message
  showErrorToast(errorMessage);

  // Log error for debugging
  console.error(`IPC Error [${errorCode}]:`, errorMessage);
}
```

### Retry Strategy

- **Network errors** (GERRIT_CONNECTION_FAILED, NETWORK_ERROR): Retry up to 3 times with exponential backoff
- **Authentication errors** (AUTH_FAILED): Do not retry, prompt user for credentials
- **Validation errors** (INVALID_REVISION, INVALID_SETTINGS, etc.): Do not retry, show validation error
- **Filesystem errors** (ACCESS_DENIED, FILE_NOT_FOUND): Do not retry, show error message

---

## Performance Requirements

### Response Time Targets

| Command | Target | Maximum |
|---------|--------|----------|
| `open_repo_dialog` | <100ms | <200ms |
| `load_repo` | <100ms | <200ms |
| `get_branches` | <50ms | <100ms |
| `get_file_diff` | <150ms | <300ms (small files), <1000ms (large files with virtual scrolling) |
| `get_blame` | <100ms | <200ms |
| `import_gerrit_change` | <500ms | <2000ms (depends on network) |
| `get_gerrit_changes` | <500ms | <2000ms (depends on network) |
| `submit_gerrit_review` | <500ms | <2000ms (depends on network) |
| `get_review_stats` | <100ms | <200ms |
| `add_review_comment` | <100ms | <200ms |
| `get_review_comments` | <100ms | <200ms |
| `get_user_settings` | <50ms | <100ms |
| `save_user_settings` | <50ms | <100ms |
| `get_tags` | <50ms | <100ms |
| `save_tags` | <50ms | <100ms |
| `get_quality_gates` | <500ms | <2000ms (depends on external CI) |
| `get_heatmap` | <200ms | <500ms |
| `get_checklist` | <100ms | <200ms |
| `get_commands` | <100ms | <200ms |

### Virtual Scrolling

Diffs with more than 5000 lines must use virtual scrolling (TanStack Virtual). Frontend should:
- Detect diff size before rendering
- Switch to virtual scrolling component for large diffs
- Lazy-load diff lines as user scrolls
- Maintain performance with <16ms per frame (60fps)

---

## Security Considerations

### Authentication

- **Gerrit credentials** must be stored securely (via Tauri secure storage)
- **Passwords/tokens** must never be logged or exposed in console
- **Username/password** are optional overrides; use stored credentials when not provided

### Input Sanitization

- **All user input** is sanitized by Rust backend before use
- **Frontend** must validate input format before IPC calls (for better UX)
- **File paths** must be validated to prevent directory traversal attacks

### Allowlist

- **Tauri allowlist** in `tauri.conf.json` must be minimally scoped
- **Filesystem access**: Only to user-selected directories
- **Network access**: Only to configured Gerrit servers
- **No direct access** to filesystem or network from frontend

### Dependency Scanning

- **Run `cargo deny`** weekly to check for Rust dependency vulnerabilities
- **Run `npm audit`** regularly to check for Node.js dependency vulnerabilities
- **Address vulnerabilities** before merging to main branch

---

## Type Definitions

All TypeScript interfaces for IPC commands are defined in:
```
frontend/api/types/
├── ipc-types.ts      # IPC command request/response types
├── repo-types.ts      # Repository-related types
├── gerrit-types.ts    # Gerrit-related types
└── review-types.ts    # Review-related types
```

**Note**: These type definitions must match the Rust backend serialization format exactly. Use `serde` annotations in Rust to ensure compatibility.

---

## Migration Notes

### Frontend Merge

During the frontend merge (007-merge-frontend):
- **All IPC commands remain unchanged** (backend not modified)
- **Frontend must preserve** all existing IPC client code (`frontend/api/client.ts`)
- **HyperReview_Frontend components** must invoke IPC commands via existing client
- **No new IPC commands** will be added
- **No existing IPC commands** will be removed

### Component Integration

HyperReview_Frontend components (LocalToolBar, RemoteToolBar, etc.) must:
- Import IPC client from `frontend/api/client.ts`
- Use existing service layers (gerrit-simple-service, reviewService, etc.)
- Follow the same error handling patterns
- Maintain the same retry strategies for network operations

---

**End of IPC Interface Contract**
