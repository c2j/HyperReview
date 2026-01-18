# Data Model: Merge New Frontend with Local/Remote Review Mode Separation

**Feature**: 001-merge-new-frontend
**Date**: 2025-01-18
**Purpose**: Define entities, relationships, and validation rules for merged frontend

## Core Entities

### 1. ReviewMode (Application State)

Represents the current review context (Local or Remote), determining which UI components and workflows are active.

```typescript
interface ReviewMode {
  mode: 'local' | 'remote';
  lastMode?: 'local' | 'remote'; // For transition animations
  modeSwitchTimestamp: number; // For analytics
}
```

**Relationships**:
- Has one: `LocalReviewContext` (when mode === 'local')
- Has one: `RemoteReviewContext` (when mode === 'remote')
- Related to: `EditorSettings` (shared across modes)

**Validation Rules**:
- Mode must be either 'local' or 'remote'
- Mode changes should persist across application restarts
- Mode transitions should preserve unsaved changes or warn user

---

### 2. LocalReviewContext

Contains information about the currently open local git repository, selected branch, and active review tasks.

```typescript
interface LocalReviewContext {
  repository: {
    path: string | null; // Absolute path to repository
    name: string | null; // Repository name
    isLoaded: boolean;
  };
  
  branch: {
    base: string | null; // Base branch for comparison
    head: string | null; // Current/feature branch being reviewed
    availableBranches: string[]; // All available branches
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

**Relationships**:
- Owned by: `ReviewMode` (when mode === 'local')
- Contains multiple: `LocalTask`
- Contains multiple: `Comment` (on files/tasks)
- Related to: `DiffContext` (for viewing changes)

**State Transitions**:
```
[No Repo Loaded] --open_repo--> [Repo Loaded]
[Repo Loaded] --select_branch--> [Branch Selected]
[Branch Selected] --select_task--> [Task Active]
[Task Active] --add_comment--> [Comment Added]
[Task Active] --complete--> [Task Completed]
[Task Completed] --new_task--> [Task Active]
```

**Validation Rules**:
- Repository path must be absolute and valid git repository
- Branch names must exist in available branches
- Only one task can be active at a time
- Selected file must exist in current task's file list

---

### 3. RemoteReviewContext

Contains information about remote platform connections (Gerrit, CodeArts, GitLab), imported changes, and remote-specific tasks.

```typescript
interface RemoteReviewContext {
  platform: {
    type: 'gerrit' | 'codearts' | 'gitlab' | null;
    instanceId: string | null;
    instanceName: string | null;
    isConfigured: boolean;
  };
  
  connection: {
    url: string | null;
    credentials: {
      username: string | null;
      token: string | null; // Stored securely in Tauri backend
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

**Relationships**:
- Owned by: `ReviewMode` (when mode === 'remote')
- Contains multiple: `RemoteChange`
- Related to: `GerritInstance` (for Gerrit platform)
- Related to: `ReviewSession` (for tracking review progress)

**State Transitions**:
```
[Not Configured] --configure_instance--> [Instance Configured]
[Instance Configured] --connect--> [Connected]
[Connected] --import_change--> [Change Imported]
[Change Imported] --select--> [Reviewing Change]
[Reviewing Change] --add_comment--> [Comment Added]
[Reviewing Change] --submit--> [Review Submitted]
```

**Validation Rules**:
- Platform type must be supported (gerrit, codearts, gitlab)
- Instance URL must be valid HTTPS/HTTP URL
- Credentials must be stored securely (never in frontend localStorage)
- Change IDs must be valid identifiers from remote platform

---

### 4. EditorSettings

Stores user preferences for editor display and behavior.

```typescript
interface EditorSettings {
  fontSize: number; // 12-24px, default: 14
  enableLigatures: boolean; // Font ligatures, default: true
  vimMode: boolean; // Block cursor and Vim-like behavior, default: false
  theme: 'light' | 'dark' | 'system'; // Color theme, default: 'system'
  lineWrapping: boolean; // Wrap long lines, default: false
  showLineNumbers: boolean; // Show line numbers in diffs, default: true
  syntaxHighlighting: boolean; // Enable code highlighting, default: true
}
```

**Relationships**:
- Shared across: `LocalReviewContext` and `RemoteReviewContext`
- Applied to: `DiffView`, `VirtualDiffViewer`, and all code display components
- Persisted via: `Tauri Store Plugin` (preferred) or localStorage (fallback)

**Validation Rules**:
- Font size must be between 12 and 24
- Settings must be applied globally via CSS variables
- Settings changes should take effect immediately without page reload
- Settings must persist across application restarts

---

### 5. LocalTask

Represents a code review task for local git repository changes.

```typescript
interface LocalTask {
  id: string; // Unique task identifier
  title: string; // Task title/description
  description?: string; // Detailed task description
  
  state: {
    status: 'active' | 'pending' | 'completed' | 'blocked';
    createdAt: number; // Unix timestamp
    updatedAt: number; // Last modification timestamp
  };
  
  scope: {
    repoPath: string; // Repository absolute path
    baseBranch: string; // Base branch for comparison
    headBranch: string; // Feature branch being reviewed
  };
  
  files: TaskFile[]; // Files included in this task
  
  progress: {
    reviewedFileCount: number;
    totalFileCount: number;
    commentCount: number;
    severityCounts: {
      error: number;
      warning: number;
      info: number;
    };
  };
  
  metadata: {
    tags: string[]; // User-defined tags
    priority: 'high' | 'medium' | 'low'; // Task priority
    estimatedTime?: number; // Estimated review time in minutes
  };
}

interface TaskFile {
  path: string; // Relative path from repository root
  status: 'unreviewed' | 'reviewing' | 'completed';
  commentCount: number;
  severityCounts: {
    error: number;
    warning: number;
    info: number;
  };
}
```

**Relationships**:
- Owned by: `LocalReviewContext`
- Contains multiple: `TaskFile`
- Contains multiple: `Comment` (on files)

**State Transitions**:
```
[Pending] --start--> [Active]
[Active] --complete_file--> [Active] (updated progress)
[Active] --add_comment--> [Active] (updated progress)
[Active] --block--> [Blocked]
[Blocked] --unblock--> [Active]
[Active] --complete--> [Completed]
[Completed] --reopen--> [Active]
```

**Validation Rules**:
- Title must not be empty
- Repository path must be valid git repository
- Base and head branches must exist
- File paths must be relative to repository root
- Progress counts cannot exceed totals

---

### 6. RemoteChange

Represents a change/PR/MR from a remote platform (Gerrit, CodeArts, GitLab).

```typescript
interface RemoteChange {
  id: string; // Platform-specific change ID (e.g., Gerrit change number)
  platform: 'gerrit' | 'codearts' | 'gitlab';
  
  metadata: {
    title: string;
    description: string;
    author: string;
    authorEmail: string;
    createdAt: number;
    updatedAt: number;
  };
  
  state: {
    status: 'pending' | 'in-progress' | 'submitted' | 'merged' | 'abandoned';
    reviewStatus?: 'draft' | 'active' | 'reviewed';
    isDownloaded: boolean; // Whether files are cached locally
  };
  
  scope: {
    projectName: string;
    sourceBranch: string;
    targetBranch: string;
    commitId?: string; // Latest commit
  };
  
  files: RemoteFile[]; // Files in this change
  comments: RemoteComment[]; // Comments from remote platform
  
  review: {
    sessionId: string | null; // Local review session ID
    reviewStatus: 'not-started' | 'in-progress' | 'ready-to-submit' | 'submitted';
    progress: {
      reviewedFileCount: number;
      totalFileCount: number;
      localCommentCount: number; // Unsubmitted local comments
    };
  };
}

interface RemoteFile {
  id: string; // File identifier (path or hash)
  path: string; // File path in repository
  status: string; // Added/Modified/Deleted
  patchSetNumber?: number; // Gerrit patch set number
  
  localCopy: {
    exists: boolean; // Whether downloaded locally
    filePath?: string; // Local file path if downloaded
  };
}

interface RemoteComment {
  id: string;
  author: string;
  message: string;
  timestamp: number;
  filePath: string;
  lineNumber?: number;
  isDraft: boolean; // Unsubmitted local comment
}
```

**Relationships**:
- Owned by: `RemoteReviewContext`
- Related to: `GerritInstance` (via platform type)
- Related to: `ReviewSession` (for tracking local review progress)

**State Transitions**:
```
[Pending] --download--> [Downloaded]
[Downloaded] --start_review--> [Reviewing]
[Reviewing] --add_local_comment--> [Reviewing]
[Reviewing] --submit--> [Ready to Submit]
[Ready to Submit] --publish--> [Submitted]
```

**Validation Rules**:
- Change ID must be valid for the platform
- Status values must be valid platform states
- Review progress cannot exceed total file count
- Local comments must be synchronized before submitting

---

### 7. Comment

Represents a review comment on a file or specific line.

```typescript
interface Comment {
  id: string; // Unique comment identifier
  type: 'file-level' | 'line-level' | 'inline';
  
  content: {
    message: string;
    severity: 'INFO' | 'WARNING' | 'ERROR' | 'SUCCESS';
    tags?: string[]; // User-defined tags (e.g., "N+1 Problem")
  };
  
  location: {
    filePath: string; // File path (relative or absolute)
    lineNumber?: number; // Line number (for line-level comments)
    startLine?: number; // For inline multi-line comments
    endLine?: number;
  };
  
  context: {
    mode: 'local' | 'remote'; // Which review mode
    taskId?: string; // Associated task (local) or change (remote)
    commitId?: string; // Git commit hash
  };
  
  state: {
    isDraft: boolean; // Whether comment is unsaved/unsubmitted
    isSubmitted: boolean; // Whether synced to backend
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

**Relationships**:
- Owned by: `LocalTask` or `RemoteChange`
- Related to: `TaskFile` or `RemoteFile`

**State Transitions**:
```
[Draft] --save--> [Saved Locally]
[Saved Locally] --submit--> [Submitted to Backend]
[Submitted to Backend] --edit--> [Draft] (unsubmitted)
```

**Validation Rules**:
- Message must not be empty
- Line numbers must be valid for the file
- Severity must be one of: INFO, WARNING, ERROR, SUCCESS
- Draft comments cannot be submitted to remote platform

---

### 8. DiffContext

Represents the context for viewing file differences.

```typescript
interface DiffContext {
  files: DiffFile[]; // Files with changes
  
  viewing: {
    activeFileId: string | null; // Currently viewed file
    selectedLineNumbers: Set<number>; // Lines selected by user
    highlightedSeverities: Set<string>; // Severities to highlight
  };
  
  configuration: {
    contextLines: number; // Lines of context to show
    ignoreWhitespace: boolean;
    wordDiff: boolean; // Show word-level differences
    sideBySide: boolean; // Display side-by-side or unified
  };
}

interface DiffFile {
  id: string; // File identifier
  path: string; // File path
  status: 'added' | 'modified' | 'deleted' | 'renamed';
  
  lines: DiffLine[];
  
  metadata: {
    oldPath?: string; // For renamed files
    newPath?: string;
    addLineCount: number;
    removeLineCount: number;
    complexity?: number; // Calculated complexity
    churn?: number; // Recent change frequency
  };
}

interface DiffLine {
  oldLineNumber?: number;
  newLineNumber?: number;
  content: string;
  type: 'added' | 'removed' | 'context' | 'header';
  
  analysis?: {
    severity?: 'ERROR' | 'WARNING' | 'INFO';
    message?: string; // Analysis message (e.g., potential bug, TODO)
    complexityScore?: number;
  };
  
  comments: Comment[]; // Comments on this line
}
```

**Relationships**:
- Related to: `LocalReviewContext` or `RemoteReviewContext`
- Consumed by: `DiffView`, `VirtualDiffViewer`
- Associated with: `LocalTask` or `RemoteChange`

**Validation Rules**:
- Only one file can be active at a time
- Context lines must be positive (recommended: 3-5)
- Line numbers must be sequential and valid
- Analysis results should be computed by backend (not in frontend)

---

### 9. GerritInstance

Represents a Gerrit server instance configuration.

```typescript
interface GerritInstance {
  id: string; // Unique instance identifier
  name: string; // User-defined instance name
  
  connection: {
    url: string; // Gerrit server URL
    username: string;
    credentialsId: string; // Secure credential reference
  };
  
  state: {
    isActive: boolean; // Currently selected instance
    lastConnected: number; // Timestamp of last successful connection
    isHealthy: boolean; // Server reachable
  };
  
  metadata: {
    createdAt: number;
    updatedAt: number;
  };
}
```

**Relationships**:
- Owned by: `RemoteReviewContext` (platform === 'gerrit')
- Related to: `RemoteChange` (via import)
- Stored via: `Tauri backend credentials store`

**Validation Rules**:
- URL must be valid HTTPS/HTTP endpoint
- Username must not be empty
- Only one instance can be active at a time

---

### 10. ReviewSession

Tracks the progress of reviewing a remote change locally.

```typescript
interface ReviewSession {
  id: string; // Session identifier
  
  change: {
    changeId: string; // Associated remote change
    platform: 'gerrit' | 'codearts' | 'gitlab';
  };
  
  state: {
    status: 'active' | 'paused' | 'abandoned' | 'completed';
    startedAt: number;
    completedAt?: number;
  };
  
  progress: {
    reviewedFiles: ReviewProgress[];
    totalFileCount: number;
    overallProgress: number; // 0-100 percentage
  };
  
  localComments: Comment[]; // Unsubmitted comments
  draftSubmitted: boolean; // Whether draft has been pushed
}

interface ReviewProgress {
  filePath: string;
  status: 'unreviewed' | 'reviewing' | 'completed';
  commentCount: number;
  lastReviewedAt: number;
}
```

**Relationships**:
- Owned by: `RemoteChange`
- Related to: `RemoteReviewContext`
- Stores: `Comment` (local, unsubmitted)

**State Transitions**:
```
[Not Started] --start--> [Active]
[Active] --pause--> [Paused]
[Paused] --resume--> [Active]
[Paused] --abandon--> [Abandoned]
[Active] --complete--> [Completed]
[Completed] --reopen--> [Active]
```

**Validation Rules**:
- Change ID must match existing remote change
- Progress percentage must be 0-100
- Reviewed file count cannot exceed total file count
- Comments must be synced before marking as completed

---

## Entity Relationships Summary

```
┌─────────────────────────────────────────────────────────────────┐
│                     ReviewMode (Application State)                │
│  ┌─────────────────────┬─────────────────────┐            │
│  │                     │                     │            │
│  ▼                     ▼                     ▼            │
│ LocalReviewContext   RemoteReviewContext   EditorSettings   │
│  │                     │                     │            │
│  ├─ LocalTask         ├─ RemoteChange                   │
│  │                     │                     │            │
│  └─ TaskFile          └─ RemoteFile                    │
│  │                     │                     │            │
│  ▼                     ▼                     │            │
│  Comment               Comment                           │
│  │                                             │            │
│  └─────────────────────────────────────────────┘            │
│                     ▼                                 │
│                DiffContext (Shared)                         │
│                     │                                 │
│                DiffView, VirtualDiffViewer                │
└─────────────────────────────────────────────────────────────────┘

GerritInstance
    │
    └─ RemoteReviewContext (when platform === 'gerrit')

RemoteChange
    │
    └─ ReviewSession (local review tracking)
```

---

## Data Persistence Strategy

### Frontend Persistence (Tauri Store Plugin)

**Primary Storage**: Tauri Store Plugin for reliable file-based persistence

```typescript
// Stored in settings.json
interface AppSettings {
  reviewMode: ReviewMode;
  editorSettings: EditorSettings;
  localReviewContext?: LocalReviewContext;
  remoteReviewContext?: RemoteReviewContext;
}

// Stored in instances.json
interface StoredInstances {
  gerritInstances: GerritInstance[];
}

// Stored in comments.json
interface StoredComments {
  localComments: Comment[]; // Draft comments for local tasks
  remoteComments: Comment[]; // Unsubmitted comments for remote changes
}
```

### Backend Persistence (SQLite)

**Primary Storage**: `hyper_review.db` via rusqlite

```sql
-- Existing tables (preserved)
CREATE TABLE repos (
  path TEXT PRIMARY KEY,
  branch TEXT,
  last_opened INTEGER
);

CREATE TABLE local_tasks (
  id TEXT PRIMARY KEY,
  title TEXT NOT NULL,
  status TEXT,
  created_at INTEGER,
  updated_at INTEGER
);

-- Remote review tables (existing in Gerrit schema)
CREATE TABLE gerrit_instances (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  url TEXT NOT NULL,
  is_active INTEGER DEFAULT 0
);

CREATE TABLE remote_changes (
  change_id TEXT PRIMARY KEY,
  platform TEXT NOT NULL,
  status TEXT,
  downloaded INTEGER DEFAULT 0
);

CREATE TABLE review_sessions (
  id TEXT PRIMARY KEY,
  change_id TEXT NOT NULL,
  status TEXT,
  started_at INTEGER
);

CREATE TABLE comments (
  id TEXT PRIMARY KEY,
  task_id TEXT,
  file_path TEXT NOT NULL,
  line_number INTEGER,
  message TEXT NOT NULL,
  is_draft INTEGER DEFAULT 1
);
```

### State Management (Zustand)

**Frontend State**: Zustand stores with Tauri Store middleware

```typescript
// frontend/store/appStore.ts - Main application state
interface AppStore {
  reviewMode: ReviewMode;
  editorSettings: EditorSettings;
  
  setReviewMode: (mode: ReviewMode) => void;
  updateEditorSettings: (settings: Partial<EditorSettings>) => void;
  
  // Persistence
  saveSettings: () => Promise<void>;
}

// frontend/store/localReviewStore.ts - Local review state
interface LocalReviewStore {
  repository: LocalReviewContext['repository'];
  branch: LocalReviewContext['branch'];
  tasks: LocalReviewContext['tasks'];
  
  loadRepo: (path: string) => Promise<void>;
  selectBranch: (base: string, head: string) => void;
}

// frontend/store/remoteReviewStore.ts - Remote review state
interface RemoteReviewStore {
  platform: RemoteReviewContext['platform'];
  changes: RemoteReviewContext['changes'];
  
  configureGerrit: (instance: GerritInstance) => Promise<void>;
  importChange: (changeId: string) => Promise<void>;
}
```

---

## Validation Rules Summary

| Entity | Validation Rules | Error Handling |
|---------|----------------|----------------|
| **ReviewMode** | Mode must be 'local' or 'remote' | Default to 'local' |
| **LocalReviewContext** | Valid git repo path, branches exist | Show error dialog, prevent mode switch |
| **RemoteReviewContext** | Valid URL, credentials secure | Show connection error, allow reconfiguration |
| **EditorSettings** | Font size 12-24, valid theme | Clamp values, use defaults |
| **LocalTask** | Non-empty title, valid repo | Prevent creation, show validation error |
| **RemoteChange** | Valid change ID, platform supported | Show import error, fallback to local mode |
| **Comment** | Non-empty message, valid line | Prevent save, show inline error |
| **DiffContext** | Sequential line numbers, valid file | Show parse error, highlight problematic lines |
| **GerritInstance** | Valid URL, secure credentials | Prevent saving, show URL validation |
| **ReviewSession** | Progress 0-100%, valid change ID | Prevent completion, show progress warning |

---

## Migration Strategy

### From Existing Frontend to Merged Frontend

**Phase 1: Maintain Existing Data**
```typescript
// Preserve existing Zustand store structure
// Add new mode state without breaking changes
interface ExtendedAppStore extends ExistingAppStore {
  mode?: 'local' | 'remote'; // New, optional initially
}

// Add with default to prevent breaking
const useAppStore = create<ExtendedAppStore>((set) => ({
  // ... existing state and actions
  mode: 'local', // New default
  setMode: (mode) => set({ mode }), // New action
}));
```

**Phase 2: Migrate localStorage to Tauri Store**
```typescript
// On app startup, migrate existing localStorage settings
const migrateSettings = async () => {
  const fontSize = localStorage.getItem('settings.fontSize');
  const enableLigatures = localStorage.getItem('settings.enableLigatures');
  
  if (fontSize || enableLigatures !== null) {
    await tauriStore.set('editor', {
      fontSize: fontSize ? parseInt(fontSize) : 14,
      enableLigatures: enableLigatures !== null ? JSON.parse(enableLigatures) : true,
    });
    
    // Clear old localStorage after migration
    localStorage.removeItem('settings.fontSize');
    localStorage.removeItem('settings.enableLigatures');
  }
};
```

**Phase 3: Merge Duplicate State**
```typescript
// If both codebases have state for same concept
// Create unified state with deprecation warnings
interface UnifiedTask {
  id: string;
  title: string;
  // ... common fields
  
  // Deprecated fields (warn but still support)
  _deprecated_oldField?: any;
}
```

---

## Security Considerations

1. **Credentials Storage**: All credentials (username, tokens) stored securely via Tauri backend, never in frontend localStorage
2. **Sanitization**: All user input sanitized before storage or Tauri IPC calls
3. **Validation**: All data validated before persistence or backend communication
4. **Encryption**: Sensitive data encrypted at rest (via Tauri backend)
5. **No Secrets in State**: Never store API tokens or passwords in React state or frontend storage

---

## Performance Considerations

1. **Lazy Loading**: Large entities (e.g., `RemoteChange[]`) loaded incrementally or paginated
2. **Memoization**: Expensive computations (e.g., diff analysis) cached
3. **State Updates**: Minimize re-renders by using Zustand selectors
4. **CSS Variables**: Editor settings applied via CSS, not component re-renders
5. **Virtual Scrolling**: Diffs with >5000 lines use virtual scrolling

---

## Future Extensibility

**Adding New Platforms**: `RemoteReviewContext.platform` type extensible
**Adding New Settings**: `EditorSettings` interface can be extended
**Adding New Analysis**: `DiffLine.analysis` supports custom severity types
**Custom Workflows**: `ReviewMode` can support additional modes beyond 'local'/'remote'
