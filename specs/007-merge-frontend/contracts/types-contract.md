# TypeScript Types Contract

**Feature**: 007-merge-frontend
**Generated**: 2025-01-16
**Purpose**: TypeScript type definitions for IPC interface and data model

## Overview

This document defines all TypeScript types used in the HyperReview frontend. These types must match the Rust backend serialization format exactly.

## IPC Command Types

### Command Responses

```typescript
/**
 * Successful IPC response
 */
interface SuccessResponse<T> {
  status: 'success';
  data: T;
}

/**
 * IPC error response (thrown by Tauri invoke)
 */
interface ErrorResponse {
  code: string;
  message: string;
}

/**
 * Generic IPC result (success or error)
 */
type IpcResult<T> = SuccessResponse<T> | ErrorResponse;
```

---

## Repository Types

```typescript
/**
 * Local Git repository
 */
interface Repository {
  path: string;              // Absolute path on disk
  branch: string;            // Current active branch
  lastOpened: string;        // Human readable timestamp (e.g., "2 mins ago")
  isLoaded: boolean;         // Whether repository is currently loaded
}

/**
 * Git branch information
 */
interface Branch {
  name: string;              // Branch name
  isRemote: boolean;         // Whether this is a remote branch
  isCurrent: boolean;        // Whether this branch is currently checked out
  lastCommit?: string;       // Last commit hash
  lastCommitMessage?: string; // Last commit message
  lastCommitDate?: string;   // Last commit date
}

/**
 * Recent repository with open date
 */
interface RecentRepository extends Repository {
  openedAt: string;          // ISO timestamp when last opened
}
```

---

## Diff Types

```typescript
/**
 * Code diff context for comparison
 */
interface DiffContext {
  base: string;              // Base revision (branch name or commit hash)
  head: string;              // Head revision (branch name or commit hash)
  filePath?: string;         // Currently selected file path
  mode: ApplicationMode;      // Mode that created this diff context
}

/**
 * Single line in a code diff
 */
interface DiffLine {
  oldLineNumber?: number;    // Line number in base revision (0-based)
  newLineNumber?: number;    // Line number in head revision (0-based)
  content: string;          // Line content
  type: DiffLineType;      // Diff line type
  severity?: SeverityLevel; // Optional static analysis severity
  message?: string;        // Optional analysis message for this line
}

/**
 * Diff line type
 */
type DiffLineType = 'added' | 'removed' | 'context' | 'header';

/**
 * Static analysis severity level
 */
type SeverityLevel = 'ERROR' | 'WARNING' | 'INFO' | 'SUCCESS';

/**
 * Git blame information for a line
 */
interface BlameInfo {
  line: number;             // Line number (0-based)
  commitHash: string;       // Commit hash
  author: string;          // Author name
  authorEmail: string;     // Author email
  timestamp: string;       // Commit timestamp (ISO format)
  commitMessage: string;   // Commit message
}

/**
 * Blame information for entire file
 */
interface FileBlame {
  filePath: string;         // File path
  lines: BlameInfo[];      // Blame info for each line
}
```

---

## Gerrit Types

```typescript
/**
 * Gerrit change (remote review task)
 */
interface GerritChange {
  changeNumber: number;      // Gerrit change number (unique identifier)
  project: string;           // Gerrit project name
  subject: string;           // Change title/subject
  status: GerritChangeStatus; // Change status
  owner: string;            // Change owner name/email
  patchSetNumber: number;    // Current patch set number
  files: GerritFile[];       // Modified files in this change
  isImported: boolean;       // Whether change is imported for review
  unreadCount?: number;      // Optional number of unread comments
  url?: string;             // Optional Gerrit web URL
}

/**
 * Gerrit change status
 */
type GerritChangeStatus = 'NEW' | 'MERGED' | 'ABANDONED' | 'DRAFT';

/**
 * Gerrit file in a change
 */
interface GerritFile {
  path: string;             // File path (relative to project root)
  status: GerritFileStatus; // File status
  oldPath?: string;         // Old file path (for renamed files)
  patchSet: number;         // Patch set number
  insertions: number;       // Number of lines inserted
  deletions: number;       // Number of lines deleted
}

/**
 * Gerrit file status
 */
type GerritFileStatus = 'ADDED' | 'MODIFIED' | 'DELETED' | 'RENAMED' | 'COPIED';

/**
 * Gerrit server configuration
 */
interface GerritServerConfig {
  url: string;             // Gerrit server URL (e.g., 'https://gerrit.example.com')
  username: string;         // Username for authentication
  password?: string;        // Password or API token (optional)
  port?: number;           // SSH port (default: 29418)
  project?: string;         // Default project filter
}

/**
 * Gerrit review submission
 */
interface GerritReviewSubmission {
  label: number;           // Review score (-2, -1, 0, +1, +2)
  message: string;         // Review message
  comments?: InlineComment[]; // Optional inline comments
}

/**
 * Gerrit review score
 */
type GerritReviewScore = -2 | -1 | 0 | 1 | 2;

/**
 * Inline comment for Gerrit review
 */
interface InlineComment {
  filePath: string;         // File path
  line: number;            // Line number (0-based)
  message: string;         // Comment text
}

/**
 * Gerrit review result
 */
interface GerritReviewResult {
  success: boolean;
  reviewId?: string;       // Review identifier (if successful)
}
```

---

## Task Types

```typescript
/**
 * Application mode (local or remote)
 */
type ApplicationMode = 'local' | 'remote';

/**
 * Generic review task (local branch or Gerrit change)
 */
interface Task {
  id: string;              // Unique task identifier
  title: string;           // Task title/subject
  status: TaskStatus;      // Task status
  unreadCount?: number;    // Optional number of unread items
  mode: ApplicationMode;   // Task mode (local or remote)
  createdAt: string;       // Creation timestamp (ISO format)
  updatedAt: string;       // Last update timestamp (ISO format)

  // Mode-specific data
  repository?: Repository;  // Repository (local mode only)
  gerritChange?: GerritChange; // Gerrit change (remote mode only)
}

/**
 * Task status
 */
type TaskStatus = 'active' | 'pending' | 'completed' | 'blocked';

/**
 * Task filter options
 */
interface TaskFilter {
  mode?: ApplicationMode;   // Filter by mode
  status?: TaskStatus;     // Filter by status
  unread?: boolean;       // Filter by unread status
}
```

---

## Review Types

```typescript
/**
 * Review comment
 */
interface ReviewComment {
  id: string;              // Unique comment identifier
  taskId: string;          // Associated task ID
  filePath: string;        // File path
  lineNumber: number;       // Line number (0-based)
  content: string;          // Comment text
  status: CommentStatus;    // Comment status
  author?: string;         // Comment author name
  authorEmail?: string;     // Comment author email
  timestamp?: string;      // Creation timestamp (ISO format)
  updatedAt?: string;      // Last update timestamp (ISO format)
}

/**
 * Comment status
 */
type CommentStatus = 'draft' | 'submitted' | 'resolved';

/**
 * Review statistics
 */
interface ReviewStats {
  totalFiles: number;      // Total files in task
  reviewedFiles: number;    // Number of files reviewed
  totalLinesAdded: number;  // Total lines added across all files
  totalLinesRemoved: number; // Total lines removed across all files
  totalComments: number;    // Total comments added
  severeIssues: number;     // Lines with ERROR severity
  warnings: number;        // Lines with WARNING severity
  infoMessages: number;     // Lines with INFO severity
}

/**
 * Quick tag for code review
 */
interface Tag {
  id: string;              // Unique tag identifier
  label: string;           // Display label (e.g., "N+1 Problem")
  color: string;           // Color code (hex format, e.g., "#ff0000")
  description?: string;     // Optional description
}

/**
 * Review template (canned response)
 */
interface ReviewTemplate {
  id: string;              // Unique template identifier
  title: string;           // Template title
  content: string;          // Template content (supports placeholders like {author}, {file}, etc.)
  category?: string;       // Optional category for grouping
}
```

---

## Quality & Analysis Types

```typescript
/**
 * Quality gate status check
 */
interface QualityGate {
  name: string;            // Quality gate name (e.g., "CI Pipeline", "Test Coverage")
  status: QualityGateStatus; // Gate status
  details?: string;        // Additional details
  url?: string;            // Link to CI/scan result
  icon?: string;           // Icon identifier
}

/**
 * Quality gate status
 */
type QualityGateStatus = 'passed' | 'failed' | 'pending' | 'skipped';

/**
 * Heatmap item (architectural impact analysis)
 */
interface HeatmapItem {
  filePath: string;         // File path
  impactScore: number;     // Impact score (0-100)
  changeCount: number;     // Number of changes in this file
  riskLevel: RiskLevel;    // Risk level
}

/**
 * Risk level
 */
type RiskLevel = 'low' | 'medium' | 'high' | 'critical';

/**
 * Checklist item (smart review checklist)
 */
interface ChecklistItem {
  id: string;              // Unique checklist item identifier
  category: string;        // Category (e.g., "Security", "Performance", "Code Style")
  description: string;     // Checklist description
  checked: boolean;        // Whether item is checked
  priority: ChecklistPriority; // Item priority
}

/**
 * Checklist priority
 */
type ChecklistPriority = 'low' | 'medium' | 'high' | 'critical';
```

---

## UI Types

```typescript
/**
 * Panel configuration
 */
interface PanelConfig {
  left: {
    width: number;         // Left panel width in pixels (min: 200px)
    visible: boolean;      // Left panel visibility
  };
  right: {
    width: number;         // Right panel width in pixels (min: 200px)
    visible: boolean;      // Right panel visibility
  };
}

/**
 * User settings
 */
interface UserSettings {
  language: string;        // UI language code (e.g., 'en', 'zh')
  fontSize: number;        // Editor font size in pixels (10-24)
  ligatures: boolean;      // Enable font ligatures
  vimMode: boolean;       // Enable vim keybindings
  theme: Theme;            // UI theme
  panels: PanelConfig;     // Panel configuration
}

/**
 * UI theme
 */
type Theme = 'light' | 'dark' | 'system';

/**
 * Notification
 */
interface Notification {
  id: string;              // Unique notification identifier
  type: NotificationType;   // Notification type
  message: string;         // Notification message
  timestamp: string;       // Timestamp (ISO format)
  duration?: number;       // Auto-dismiss duration in milliseconds (optional)
}

/**
 * Notification type
 */
type NotificationType = 'info' | 'success' | 'warning' | 'error';

/**
 * Modal state
 */
interface ModalState {
  isOpen: boolean;         // Whether modal is open
  title?: string;          // Modal title
  content?: React.ReactNode; // Modal content
  onClose?: () => void;    // On close callback
  size?: ModalSize;        // Modal size
}

/**
 * Modal size
 */
type ModalSize = 'small' | 'medium' | 'large' | 'fullscreen';

/**
 * Command palette search result
 */
interface SearchResult {
  id: string;              // Unique result identifier
  type: SearchResultType;  // Result type
  label: string;           // Display label
  description?: string;     // Optional description
  icon?: string;           // Icon identifier
  shortcut?: string;       // Keyboard shortcut
  action?: () => void;     // Action function (not serializable)
}
```

---

## Command Palette Types

```typescript
/**
 * Command palette search result type
 */
type SearchResultType = 'file' | 'symbol' | 'command' | 'setting' | 'recent';

/**
 * Command palette search query
 */
interface SearchQuery {
  query?: string;          // Search query string
  type?: SearchResultType; // Filter by result type
  limit?: number;          // Maximum results (default: 20)
}
```

---

## Error Types

```typescript
/**
 * Application error
 */
interface AppError {
  code: ErrorCode;         // Error code
  message: string;         // Error message
  details?: string;        // Additional details
  stack?: string;          // Stack trace (development only)
}

/**
 * Application error code
 */
type ErrorCode =
  // Repository errors
  | 'INVALID_REPO'
  | 'ACCESS_DENIED'
  | 'REPO_NOT_FOUND'
  | 'FILE_NOT_FOUND'
  | 'INVALID_REVISION'
  | 'DIFF_TOO_LARGE'
  // Gerrit errors
  | 'GERRIT_CONNECTION_FAILED'
  | 'CHANGE_NOT_FOUND'
  | 'AUTH_FAILED'
  | 'NETWORK_ERROR'
  | 'INVALID_REVIEW'
  // Task errors
  | 'TASK_NOT_FOUND'
  | 'INVALID_LINE_NUMBER'
  // Settings errors
  | 'INVALID_SETTINGS'
  | 'INVALID_TAGS'
  // General errors
  | 'UNKNOWN_ERROR';

/**
 * Error with user-friendly message
 */
interface UserFacingError {
  title: string;           // Error title
  message: string;         // User-friendly error message
  action?: string;        // Suggested action for user
}
```

---

## Utility Types

```typescript
/**
 * Partial with recursive support
 */
type DeepPartial<T> = {
  [P in keyof T]?: T[P] extends object ? DeepPartial<T[P]> : T[P];
};

/**
 * Required keys
 */
type RequiredKeys<T, K extends keyof T> = T & Required<Pick<T, K>>;

/**
 * Optional keys
 */
type OptionalKeys<T, K extends keyof T> = Omit<T, K> & Partial<Pick<T, K>>;

/**
 * Make all keys optional recursively
 */
type DeepOptional<T> = {
  [P in keyof T]?: T[P] extends object ? DeepOptional<T[P]> : T[P];
};

/**
 * Extract keys by value type
 */
type KeysByValue<T, V> = {
  [K in keyof T]: T[K] extends V ? K : never;
}[keyof T];
```

---

## Export Summary

```typescript
// Main exports
export type {
  // Core types
  ApplicationMode,
  Theme,
  SeverityLevel,
  RiskLevel,

  // Repository
  Repository,
  Branch,
  RecentRepository,

  // Diff
  DiffContext,
  DiffLine,
  DiffLineType,
  BlameInfo,
  FileBlame,

  // Gerrit
  GerritChange,
  GerritChangeStatus,
  GerritFile,
  GerritFileStatus,
  GerritServerConfig,
  GerritReviewSubmission,
  GerritReviewScore,
  InlineComment,
  GerritReviewResult,

  // Task
  Task,
  TaskStatus,
  TaskFilter,

  // Review
  ReviewComment,
  CommentStatus,
  ReviewStats,
  Tag,
  ReviewTemplate,

  // Quality & Analysis
  QualityGate,
  QualityGateStatus,
  HeatmapItem,
  ChecklistItem,
  ChecklistPriority,

  // UI
  PanelConfig,
  UserSettings,
  Notification,
  NotificationType,
  ModalState,
  ModalSize,
  SearchResult,
  SearchResultType,
  SearchQuery,

  // Error
  AppError,
  ErrorCode,
  UserFacingError,

  // Utility
  DeepPartial,
  RequiredKeys,
  OptionalKeys,
  DeepOptional,
  KeysByValue,
};

export interface {
  // IPC
  SuccessResponse,
  ErrorResponse,
  IpcResult,
};
```

---

**End of TypeScript Types Contract**
