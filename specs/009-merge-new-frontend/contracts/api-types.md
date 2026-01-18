# API Type Definitions

**Feature**: 001-merge-new-frontend
**Date**: 2025-01-18
**Purpose**: Shared TypeScript type definitions for frontend and Tauri IPC communication

## Overview

This file defines all TypeScript interfaces used across the merged frontend application. These types ensure type safety between the React frontend and Rust backend via Tauri IPC.

## Core Types

### ReviewMode

Application review context (Local or Remote).

```typescript
export type ReviewMode = 'local' | 'remote';

export interface ReviewModeState {
  mode: ReviewMode;
  lastMode?: ReviewMode;
  modeSwitchTimestamp: number;
}
```

---

### EditorSettings

User preferences for editor display and behavior.

```typescript
export interface EditorSettings {
  fontSize: number;           // 12-24px, default: 14
  enableLigatures: boolean;  // Font ligatures, default: true
  vimMode: boolean;          // Block cursor and Vim-like behavior, default: false
  theme: 'light' | 'dark' | 'system'; // Color theme, default: 'system'
  lineWrapping: boolean;     // Wrap long lines, default: false
  showLineNumbers: boolean;   // Show line numbers in diffs, default: true
  syntaxHighlighting: boolean; // Enable code highlighting, default: true
}
```

---

### CommandResult

Standard result wrapper for all Tauri commands.

```typescript
export type CommandResult<T> = {
  success: true;
  data: T;
};

export type CommandError = {
  success: false;
  error: string;
  code?: string;  // Optional error code for specific handling
};

export type ApiResponse<T> = CommandResult<T> | CommandError;

export function isCommandResult<T>(result: ApiResponse<T>): result is CommandResult<T> {
  return result.success === true;
}

export function isCommandError(result: ApiResponse<unknown>): result is CommandError {
  return result.success === false;
}
```

---

## Repository Types

### Repo

Information about a git repository.

```typescript
export interface Repo {
  path: string;       // Absolute path on disk
  branch: string;     // Current active branch
  lastOpened: string; // Human readable string (e.g., "2 mins ago")
}
```

---

### Branch

Git branch information.

```typescript
export interface Branch {
  name: string;     // Branch name
  isCurrent: boolean; // Whether this is the currently checked out branch
  isLocal: boolean;   // Local or remote tracking branch
}
```

---

### LoadRepoResult

Result from loading a repository.

```typescript
export interface LoadRepoResult {
  success: boolean;
  repoName: string | null;
  currentBranch: string | null;
  availableBranches: string[];
}
```

---

## Review & Diff Types

### DiffLine

A single line in a file diff.

```typescript
export type DiffLineType = 'added' | 'removed' | 'context' | 'header';

export interface DiffLine {
  oldLineNumber?: number;
  newLineNumber?: number;
  content: string;
  type: DiffLineType;
  severity?: Severity;
  message?: string; // Analysis message for the specific line
}
```

---

### Severity

Severity levels for analysis messages.

```typescript
export type Severity = 'INFO' | 'WARNING' | 'ERROR' | 'SUCCESS';
```

---

### DiffContext

Context for viewing file differences.

```typescript
export interface DiffContext {
  files: DiffFile[];
  
  viewing: {
    activeFileId: string | null;
    selectedLineNumbers: Set<number>;
    highlightedSeverities: Set<Severity>;
  };
  
  configuration: {
    contextLines: number;    // Lines of context to show
    ignoreWhitespace: boolean;
    wordDiff: boolean;         // Show word-level differences
    sideBySide: boolean;     // Display side-by-side or unified
  };
}
```

---

### DiffFile

A file with changes.

```typescript
export interface DiffFile {
  id: string;     // File identifier
  path: string;   // File path
  status: FileStatus;
  
  lines: DiffLine[];
  
  metadata?: {
    oldPath?: string; // For renamed files
    newPath?: string;
    addLineCount: number;
    removeLineCount: number;
    complexity?: number; // Calculated complexity
    churn?: number;       // Recent change frequency
  };
}
```

---

### FileStatus

Status of a file in a diff.

```typescript
export type FileStatus = 'added' | 'modified' | 'deleted' | 'renamed';
```

---

## Comment Types

### Comment

A review comment on a file or specific line.

```typescript
export type CommentType = 'file-level' | 'line-level' | 'inline';

export interface Comment {
  id: string;
  type: CommentType;
  
  content: {
    message: string;
    severity: Severity;
    tags?: string[];
  };
  
  location: {
    filePath: string;
    lineNumber?: number;
    startLine?: number; // For inline multi-line comments
    endLine?: number;
  };
  
  context: {
    mode: ReviewMode;       // Which review mode
    taskId?: string;        // Associated task (local) or change (remote)
    commitId?: string;      // Git commit hash
  };
  
  state: {
    isDraft: boolean;
    isSubmitted: boolean;
    createdAt: number;
    updatedAt: number;
  };
  
  author: {
    userId: string;
    username: string;
    email?: string;
  };
}
```

---

### CommentInput

Input for creating or updating a comment.

```typescript
export interface CommentInput {
  taskId: string;
  filePath: string;
  lineNumber?: number;
  content: string;
  type?: CommentType;
  severity?: Severity;
  tags?: string[];
}
```

---

## Local Review Types

### LocalTask

Represents a code review task for local git repository changes.

```typescript
export interface LocalTask {
  id: string;
  title: string;
  description?: string;
  
  state: {
    status: TaskStatus;
    createdAt: number;
    updatedAt: number;
  };
  
  scope: {
    repoPath: string;
    baseBranch: string;
    headBranch: string;
  };
  
  files: TaskFile[];
  
  progress: {
    reviewedFileCount: number;
    totalFileCount: number;
    commentCount: number;
    severityCounts: SeverityCounts;
  };
  
  metadata: {
    tags: string[];
    priority: 'high' | 'medium' | 'low';
    estimatedTime?: number; // Estimated review time in minutes
  };
}

export interface TaskFile {
  path: string;
  status: 'unreviewed' | 'reviewing' | 'completed';
  commentCount: number;
  severityCounts: SeverityCounts;
}

export interface SeverityCounts {
  error: number;
  warning: number;
  info: number;
}

export type TaskStatus = 'active' | 'pending' | 'completed' | 'blocked';
```

---

### LocalReviewContext

State for local review mode.

```typescript
export interface LocalReviewContext {
  repository: {
    path: string | null;
    name: string | null;
    isLoaded: boolean;
  };
  
  branch: {
    base: string | null;
    head: string | null;
    availableBranches: string[];
  };
  
  tasks: {
    activeTaskId: string | null;
    tasks: LocalTask[];
    filters: {
      status?: 'all' | 'active' | 'completed';
      searchQuery: string;
    };
  };
  
  selection: {
    selectedFilePath: string | null;
    selectedTaskId: string | null;
    selectedLines: Set<number>;
  };
}
```

---

## Remote Review Types

### RemoteChange

Represents a change/PR/MR from a remote platform.

```typescript
export type RemotePlatform = 'gerrit' | 'codearts' | 'gitlab';

export interface RemoteChange {
  id: string;
  platform: RemotePlatform;
  
  metadata: {
    title: string;
    description: string;
    author: string;
    authorEmail: string;
    createdAt: number;
    updatedAt: number;
  };
  
  state: {
    status: ChangeStatus;
    reviewStatus?: ReviewStatus;
    isDownloaded: boolean;
  };
  
  scope: {
    projectName: string;
    sourceBranch: string;
    targetBranch: string;
    commitId?: string;
  };
  
  files: RemoteFile[];
  comments: RemoteComment[];
  
  review: {
    sessionId: string | null;
    reviewStatus: ReviewProgressStatus;
    progress: {
      reviewedFileCount: number;
      totalFileCount: number;
      localCommentCount: number;
    };
  };
}

export type ChangeStatus = 'pending' | 'in-progress' | 'submitted' | 'merged' | 'abandoned';
export type ReviewStatus = 'draft' | 'active' | 'reviewed';
export type ReviewProgressStatus = 'not-started' | 'in-progress' | 'ready-to-submit' | 'submitted';
```

---

### RemoteFile

A file in a remote change.

```typescript
export interface RemoteFile {
  id: string;
  path: string;
  status: string;
  patchSetNumber?: number;
  
  localCopy: {
    exists: boolean;
    filePath?: string;
  };
}
```

---

### RemoteComment

A comment from a remote platform.

```typescript
export interface RemoteComment {
  id: string;
  author: string;
  message: string;
  timestamp: number;
  filePath: string;
  lineNumber?: number;
  isDraft: boolean;
}
```

---

### RemoteReviewContext

State for remote review mode.

```typescript
export interface RemoteReviewContext {
  platform: {
    type: RemotePlatform | null;
    instanceId: string | null;
    instanceName: string | null;
    isConfigured: boolean;
  };
  
  connection: {
    url: string | null;
    credentials: {
      username: string | null;
      token: string | null;
    };
    isConnected: boolean;
    lastSyncTimestamp: number | null;
  };
  
  changes: {
    activeChangeId: string | null;
    changes: RemoteChange[];
    filters: {
      status?: 'all' | 'pending' | 'reviewed';
      searchQuery: string;
    };
  };
  
  selection: {
    selectedChangeId: string | null;
    selectedFilePath: string | null;
    selectedLines: Set<number>;
  };
}
```

---

## Gerrit Types

### GerritInstance

Represents a Gerrit server instance configuration.

```typescript
export interface GerritInstance {
  id: string;
  name: string;
  
  connection: {
    url: string;
    username: string;
    credentialsId: string;
  };
  
  state: {
    isActive: boolean;
    lastConnected: number;
    isHealthy: boolean;
  };
  
  metadata: {
    createdAt: number;
    updatedAt: number;
  };
}
```

---

### GerritChange

A change from Gerrit.

```typescript
export interface GerritChange {
  id: string;
  
  metadata: {
    title: string;
    description: string;
    author: string;
    authorEmail: string;
    createdAt: number;
    updatedAt: number;
  };
  
  state: {
    status: string;
    isDownloaded: boolean;
  };
  
  scope: {
    projectName: string;
    sourceBranch: string;
    targetBranch: string;
  };
}
```

---

## Analysis Types

### HeatmapItem

Architectural impact analysis data.

```typescript
export interface HeatmapItem {
  id: string;
  name: string;
  impact: 'high' | 'medium' | 'low';
  churn: number;       // Change frequency (recent commits)
  complexity: number;  // Code complexity score
  filePath?: string;
}
```

---

### BlameInfo

Git blame information.

```typescript
export interface BlameInfo {
  author: string;
  email: string;
  timestamp: number;
  commitHash: string;
  commitMessage: string;
}
```

---

### ChecklistItem

Smart checklist item for code review.

```typescript
export interface ChecklistItem {
  id: string;
  category: string;
  title: string;
  description: string;
  priority: 'high' | 'medium' | 'low';
  checked: boolean;
  applicableFiles: string[];
}
```

---

## Review Session Types

### ReviewSession

Tracks progress of reviewing a remote change locally.

```typescript
export interface ReviewSession {
  id: string;
  
  change: {
    changeId: string;
    platform: RemotePlatform;
  };
  
  state: {
    status: SessionStatus;
    startedAt: number;
    completedAt?: number;
  };
  
  progress: {
    reviewedFiles: ReviewProgress[];
    totalFileCount: number;
    overallProgress: number; // 0-100 percentage
  };
  
  localComments: Comment[];
  draftSubmitted: boolean;
}

export interface ReviewProgress {
  filePath: string;
  status: 'unreviewed' | 'reviewing' | 'completed';
  commentCount: number;
  lastReviewedAt: number;
}

export type SessionStatus = 'active' | 'paused' | 'abandoned' | 'completed';
```

---

## Template Types

### ReviewTemplate

Predefined comment template.

```typescript
export interface ReviewTemplate {
  id: string;
  title: string;
  category: string;
  content: string;
  tags?: string[];
}
```

---

## File Tree Types

### FileTreeNode

Node in the file tree structure.

```typescript
export interface FileTreeNode {
  id: string;
  name: string;
  path: string;
  type: 'file' | 'directory';
  children?: FileTreeNode[];
  status?: FileStatus;
  expanded?: boolean;
}
```

---

## UI Component Types

### ModalProps

Common props for modal components.

```typescript
export interface ModalProps {
  isOpen: boolean;
  onClose: () => void;
  title?: string;
  size?: 'small' | 'medium' | 'large' | 'fullscreen';
}

export interface ConfirmModalProps extends ModalProps {
  message: string;
  onConfirm: () => void;
  onCancel?: () => void;
  confirmButtonText?: string;
  cancelButtonText?: string;
  variant?: 'danger' | 'warning' | 'info' | 'success';
}
```

---

### ToastNotification

Toast notification structure.

```typescript
export interface ToastNotification {
  id: string;
  type: 'info' | 'success' | 'warning' | 'error';
  title: string;
  message: string;
  duration?: number;
  action?: {
    label: string;
    onClick: () => void;
  };
}
```

---

## Filter Types

### TaskFilter

Filter for task list.

```typescript
export interface TaskFilter {
  status?: 'all' | 'active' | 'pending' | 'completed';
  searchQuery: string;
  priority?: 'all' | 'high' | 'medium' | 'low';
  tags?: string[];
}
```

---

### ChangeFilter

Filter for remote change list.

```typescript
export interface ChangeFilter {
  status?: 'all' | 'pending' | 'reviewed';
  searchQuery: string;
  author?: string;
  project?: string;
}
```

---

## Utility Types

### Pagination

Pagination information.

```typescript
export interface Pagination<T> {
  items: T[];
  totalCount: number;
  currentPage: number;
  pageSize: number;
  hasMore: boolean;
}
```

---

### LoadingState

Standardized loading state.

```typescript
export type LoadingState = 'idle' | 'loading' | 'success' | 'error';

export interface AsyncState<T> {
  data: T | null;
  loadingState: LoadingState;
  error: string | null;
}

export function isLoading<T>(state: AsyncState<T>): boolean {
  return state.loadingState === 'loading';
}

export function isError<T>(state: AsyncState<T>): boolean {
  return state.loadingState === 'error';
}

export function isSuccess<T>(state: AsyncState<T>): boolean {
  return state.loadingState === 'success';
}
```

---

## Export Type Index

```typescript
// Core Types
export type {
  ReviewMode,
  EditorSettings,
  CommandResult,
  CommandError,
  ApiResponse,
};

// Repository Types
export type {
  Repo,
  Branch,
  LoadRepoResult,
};

// Review & Diff Types
export type {
  DiffLine,
  DiffLineType,
  Severity,
  DiffContext,
  DiffFile,
  FileStatus,
};

// Comment Types
export type {
  Comment,
  CommentInput,
};

// Local Review Types
export type {
  LocalTask,
  TaskFile,
  SeverityCounts,
  TaskStatus,
  LocalReviewContext,
};

// Remote Review Types
export type {
  RemoteChange,
  RemotePlatform,
  ChangeStatus,
  ReviewStatus,
  ReviewProgressStatus,
  RemoteFile,
  RemoteComment,
  RemoteReviewContext,
};

// Gerrit Types
export type {
  GerritInstance,
  GerritChange,
};

// Analysis Types
export type {
  HeatmapItem,
  BlameInfo,
  ChecklistItem,
};

// Review Session Types
export type {
  ReviewSession,
  ReviewProgress,
  SessionStatus,
};

// Template Types
export type {
  ReviewTemplate,
};

// File Tree Types
export type {
  FileTreeNode,
};

// UI Component Types
export type {
  ModalProps,
  ConfirmModalProps,
  ToastNotification,
};

// Filter Types
export type {
  TaskFilter,
  ChangeFilter,
};

// Utility Types
export type {
  Pagination,
  LoadingState,
  AsyncState,
};
```

---

## Type Guards

```typescript
export function isReviewMode(value: unknown): value is ReviewMode {
  return value === 'local' || value === 'remote';
}

export function isSeverity(value: unknown): value is Severity {
  return value === 'INFO' || value === 'WARNING' || value === 'ERROR' || value === 'SUCCESS';
}

export function isRemotePlatform(value: unknown): value is RemotePlatform {
  return value === 'gerrit' || value === 'codearts' || value === 'gitlab';
}

export function isTaskStatus(value: unknown): value is TaskStatus {
  return value === 'active' || value === 'pending' || value === 'completed' || value === 'blocked';
}
```

---

## Type Utilities

```typescript
export type DeepPartial<T> = {
  [P in keyof T]?: T[P] extends object ? DeepPartial<T[P]> : T[P];
};

export type Nullable<T> = T | null;

export type Optional<T> = T | undefined;

export type Values<T> = T[keyof T][keyof T];
```
