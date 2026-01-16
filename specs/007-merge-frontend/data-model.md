# Data Model: Merge HyperReview Frontend

**Feature**: 007-merge-frontend
**Generated**: 2025-01-16
**Purpose**: Define entities, relationships, and state transitions for merged frontend

## Overview

This document describes the data model for the merged HyperReview frontend. The merge preserves all existing entities and state management from the current frontend while integrating the mode-based architecture (local/remote) from HyperReview_Frontend.

## Core Entities

### 1. Application Mode

**Purpose**: Distinguish between local repository review and remote Gerrit review contexts

```typescript
type ApplicationMode = 'local' | 'remote';
```

**Attributes**:
- `mode`: ApplicationMode - Current active mode
- `isGerritConfigured`: boolean - Whether Gerrit server is configured (remote mode availability)

**State Transitions**:
- Initial → local (default)
- local ↔ remote (mode switching via toolbar or command palette)

**Behavior**:
- Mode switching preserves user configuration (panel widths, language settings)
- Mode switching resets task and diff state
- IPC operations dispatched based on current mode

---

### 2. Repository (Local Mode)

**Purpose**: Represents a local Git repository for code review

```typescript
interface Repository {
  path: string;              // Absolute path on disk
  branch: string;            // Current active branch
  lastOpened: string;        // Human readable timestamp
  isLoaded: boolean;         // Whether repository is currently loaded
}
```

**Attributes**:
- `path`: Unique identifier, absolute filesystem path
- `branch`: Current git branch name
- `lastOpened`: Display string (e.g., "2 mins ago")
- `isLoaded`: Load status for UI state

**Validation Rules**:
- `path` must be valid filesystem path
- `branch` must exist in repository
- `isLoaded` must be true before accessing repository data

**Relationships**:
- One-to-many: Repository → DiffContext (multiple branch comparisons)
- One-to-many: Repository → Task (local review tasks)

---

### 3. Gerrit Change (Remote Mode)

**Purpose**: Represents a remote Gerrit changeset for review

```typescript
interface GerritChange {
  changeNumber: number;      // Gerrit change number (unique)
  project: string;           // Project name
  subject: string;            // Change title/subject
  status: 'NEW' | 'MERGED' | 'ABANDONED';
  owner: string;             // Change owner name/email
  patchSetNumber: number;     // Current patch set
  files: GerritFile[];        // Modified files in this change
  isImported: boolean;        // Whether change is imported for review
  unreadCount?: number;       // Number of unread comments
}
```

**Attributes**:
- `changeNumber`: Unique identifier from Gerrit server
- `project`: Gerrit project name
- `subject`: Change description
- `status`: Current Gerrit workflow status
- `owner`: Change owner
- `patchSetNumber`: Latest patch set version
- `files`: List of modified files
- `isImported`: Import status for UI state
- `unreadCount`: Optional comment count

**Validation Rules**:
- `changeNumber` must be positive integer
- `status` must be valid Gerrit status value
- `patchSetNumber` must be >= 1
- `files` list cannot be empty

**Relationships**:
- One-to-many: GerritChange → GerritFile
- Many-to-one: GerritChange → GerritServer (via project)

---

### 4. Diff Context

**Purpose**: Represents a code diff comparison between two revisions

```typescript
interface DiffContext {
  base: string;              // Base revision (branch name or commit)
  head: string;              // Head revision (branch name or commit)
  filePath?: string;         // Currently selected file path
  mode: ApplicationMode;      // Mode that created this diff context
}
```

**Attributes**:
- `base`: Source revision for comparison
- `head`: Target revision for comparison
- `filePath`: Optional currently selected file
- `mode`: Application mode (local or remote)

**Validation Rules**:
- `base` and `head` must be valid git references
- `filePath` must exist in the diff if specified

**Relationships**:
- Many-to-one: DiffContext → Repository (local mode)
- Many-to-one: DiffContext → GerritChange (remote mode)

---

### 5. Diff Line

**Purpose**: Represents a single line in a code diff with analysis annotations

```typescript
interface DiffLine {
  oldLineNumber?: number;    // Line number in base revision
  newLineNumber?: number;    // Line number in head revision
  content: string;          // Line content
  type: 'added' | 'removed' | 'context' | 'header';
  severity?: 'ERROR' | 'WARNING' | 'INFO' | 'SUCCESS';
  message?: string;         // Analysis message for this line
}
```

**Attributes**:
- `oldLineNumber`: Optional line number in base
- `newLineNumber`: Optional line number in head
- `content`: Actual line text
- `type`: Diff line type
- `severity`: Optional static analysis severity
- `message`: Optional analysis message

**Validation Rules**:
- At least one of `oldLineNumber` or `newLineNumber` must be set
- `content` cannot be empty for non-header lines
- If `severity` is set, `message` must also be set

**Relationships**:
- Many-to-one: DiffLine → DiffContext (via file path)

---

### 6. Task

**Purpose**: Represents a reviewable item (local branch or Gerrit change)

```typescript
interface Task {
  id: string;                // Unique task identifier
  title: string;             // Task title/subject
  status: 'active' | 'pending' | 'completed' | 'blocked';
  unreadCount?: number;       // Number of unread items
  mode: ApplicationMode;      // Task mode (local or remote)
  repository?: Repository;    // Repository (local mode)
  gerritChange?: GerritChange; // Gerrit change (remote mode)
}
```

**Attributes**:
- `id`: Unique identifier
- `title`: Display title
- `status`: Current task status
- `unreadCount`: Optional unread count
- `mode`: Task mode
- `repository`: Optional repository (local mode)
- `gerritChange`: Optional Gerrit change (remote mode)

**Validation Rules**:
- Exactly one of `repository` or `gerritChange` must be set based on `mode`
- `status` must be valid task status value

**Relationships**:
- One-to-many: Task → ReviewComment
- Many-to-one: Task → Repository (local mode)
- Many-to-one: Task → GerritChange (remote mode)

---

### 7. Review Comment

**Purpose**: Represents a user annotation on code

```typescript
interface ReviewComment {
  id: string;                // Unique comment identifier
  taskId: string;            // Associated task
  filePath: string;          // File path
  lineNumber: number;         // Line number (0-based)
  content: string;           // Comment text
  status: 'draft' | 'submitted';
  author?: string;           // Comment author
  timestamp?: string;        // Creation timestamp
}
```

**Attributes**:
- `id`: Unique identifier
- `taskId`: Associated task ID
- `filePath`: File path
- `lineNumber`: Line number (0-based)
- `content`: Comment text
- `status`: Draft or submitted
- `author`: Optional author
- `timestamp`: Optional creation timestamp

**Validation Rules**:
- `filePath` must be valid file path
- `lineNumber` must be >= 0
- `content` cannot be empty
- `status` must be valid comment status

**Relationships**:
- Many-to-one: ReviewComment → Task
- Many-to-one: ReviewComment → DiffContext (via filePath)

---

### 8. Panel Configuration

**Purpose**: Stores resizable panel dimensions and visibility

```typescript
interface PanelConfig {
  left: {
    width: number;           // Left panel width in pixels
    visible: boolean;        // Left panel visibility
  };
  right: {
    width: number;           // Right panel width in pixels
    visible: boolean;        // Right panel visibility
  };
}
```

**Attributes**:
- `left.width`: Left panel width (default: 260px)
- `left.visible`: Left panel visibility
- `right.width`: Right panel width (default: 300px)
- `right.visible`: Right panel visibility

**Validation Rules**:
- `width` values must be >= 200px (minimum usable size)
- `width` values must be <= 50% of viewport width (maximum)

**Behavior**:
- Preserved across mode switching
- Persisted to application settings
- Main content area adjusts based on panel visibility

---

### 9. User Settings

**Purpose**: Stores user preferences and configuration

```typescript
interface UserSettings {
  language: string;          // UI language (e.g., 'en', 'zh')
  fontSize: number;           // Editor font size in pixels
  ligatures: boolean;        // Enable font ligatures
  vimMode: boolean;           // Enable vim keybindings
  theme: 'light' | 'dark';   // UI theme
  panels: PanelConfig;        // Panel configuration
}
```

**Attributes**:
- `language`: UI language code
- `fontSize`: Font size in pixels (default: 14)
- `ligatures`: Font ligatures enabled
- `vimMode`: Vim keybindings enabled
- `theme`: UI theme
- `panels`: Panel configuration

**Validation Rules**:
- `fontSize` must be between 10px and 24px
- `language` must be supported language code

**Behavior**:
- Preserved across mode switching
- Persisted to application storage
- Changes applied immediately to UI

---

### 10. Gerrit Server Configuration

**Purpose**: Stores Gerrit server connection details

```typescript
interface GerritServerConfig {
  url: string;               // Gerrit server URL
  username: string;          // Username for authentication
  password?: string;         // Password (optional, may use token)
  port?: number;             // Port number (default: 29418)
  project?: string;           // Default project filter
}
```

**Attributes**:
- `url`: Server URL
- `username`: Authentication username
- `password`: Optional password or API token
- `port`: Optional port (default: 29418)
- `project`: Optional project filter

**Validation Rules**:
- `url` must be valid URL format
- `username` cannot be empty
- `port` must be between 1 and 65535 if specified

**Behavior**:
- Required for remote mode functionality
- Credentials stored securely (via Tauri IPC)
- Used for all Gerrit operations

---

## Entity Relationships

### Relationship Diagram

```
ApplicationMode
    ├─→ Repository (local mode)
    │   └─→ DiffContext
    │       └─→ DiffLine[]
    │   └─→ Task
    │       └─→ ReviewComment[]
    │
    └─→ GerritChange (remote mode)
        └─→ GerritFile[]
        └─→ Task
            └─→ ReviewComment[]
        └─→ GerritServerConfig (via project)

UserSettings
    ├─→ PanelConfig
    └─→ ApplicationMode (current mode)

DiffContext
    ├─→ Repository (local) OR GerritChange (remote)
    └─→ DiffLine[]
```

### Relationship Constraints

1. **Task Mode Consistency**: A Task must have exactly one of `repository` (local mode) or `gerritChange` (remote mode) based on its `mode` attribute.

2. **Diff Context Ownership**: A DiffContext belongs to either a Repository (local mode) or a GerritChange (remote mode), not both.

3. **Review Comment Association**: All ReviewComments must be associated with a valid Task.

4. **Panel Configuration Scope**: PanelConfig is shared across both local and remote modes (preserved during mode switching).

5. **User Settings Scope**: UserSettings are application-wide and not mode-specific.

---

## State Management

### State Layers

#### 1. Application-Level State (Zustand)

```typescript
interface AppState {
  // Mode
  mode: ApplicationMode;
  setMode: (mode: ApplicationMode) => void;

  // User Settings
  settings: UserSettings;
  updateSettings: (settings: Partial<UserSettings>) => void;

  // Panels
  panelConfig: PanelConfig;
  updatePanelConfig: (config: Partial<PanelConfig>) => void;

  // Notifications
  notification: string | null;
  showNotification: (message: string) => void;
  clearNotification: () => void;
}
```

**Persistence**: Application settings persisted to local storage via Tauri IPC.

#### 2. Mode-Specific State (Zustand)

##### Local Mode State

```typescript
interface LocalModeState {
  // Repository
  isRepoLoaded: boolean;
  selectedRepoPath: string | null;
  setSelectedRepoPath: (path: string | null) => void;

  // Diff Context
  diffContext: DiffContext;
  setDiffContext: (context: Partial<DiffContext>) => void;

  // Current File
  activeFilePath: string | null;
  setActiveFilePath: (path: string | null) => void;
}
```

**Persistence**: Not persisted (reset on mode switch or application restart).

##### Remote Mode State

```typescript
interface RemoteModeState {
  // Gerrit Changes
  gerritChanges: GerritChange[];
  setGerritChanges: (changes: GerritChange[]) => void;

  // Selected Change
  selectedChangeNumber: number | null;
  setSelectedChangeNumber: (number: number | null) => void;

  // Gerrit Config
  isGerritConfigured: boolean;
  setIsGerritConfigured: (configured: boolean) => void;
}
```

**Persistence**: Gerrit server configuration persisted; change list refreshed on application load.

#### 3. Component-Level State (React useState)

Transient UI state:
- Modal open/closed states
- Form input values
- Resizing operations
- Loading/error states

**Persistence**: Not persisted.

---

## State Transitions

### Mode Switching

```
[Local Mode Active]
    └─→ User switches to remote
        ├─→ Save local mode transient state (optional)
        ├─→ Preserve user settings (panels, language, etc.)
        ├─→ Reset local mode state (diff context, active file)
        └─→ [Remote Mode Active]

[Remote Mode Active]
    └─→ User switches to local
        ├─→ Save remote mode transient state (optional)
        ├─→ Preserve user settings (panels, language, etc.)
        ├─→ Reset remote mode state (selected change)
        └─→ [Local Mode Active]
```

**Constraints**:
- User settings always preserved
- Mode-specific state (diff context, selected items) reset
- Transition must complete within 500ms (SC-003)
- No state corruption or data loss

### Repository Loading

```
[No Repository Loaded]
    └─→ User opens repository
        ├─→ Validate repository path
        ├─→ Load repository metadata via IPC
        ├─→ Set isRepoLoaded = true
        ├─→ Set selectedRepoPath
        └─→ [Repository Loaded]
```

**Constraints**:
- Must validate repository is accessible
- Must load branches and commits via IPC
- Must handle invalid repositories gracefully

### Gerrit Change Import

```
[Gerrit Server Configured]
    └─→ User imports change
        ├─→ Fetch change details via IPC
        ├─→ Create GerritChange entity
        ├─→ Load file list via IPC
        ├─→ Set isImported = true
        └─→ [Change Ready for Review]
```

**Constraints**:
- Must validate Gerrit server configuration
- Must handle network failures gracefully
- Must fetch patch set files via IPC

---

## Data Flow Examples

### Example 1: Local Repository Diff View

1. User opens repository → `selectedRepoPath` set
2. User selects base and head branches → `diffContext` updated
3. User selects file → `activeFilePath` set
4. IPC call: `get_file_diff({ filePath })` → `DiffLine[]`
5. DiffView renders `DiffLine[]` with syntax highlighting
6. User adds comment → `ReviewComment` created via IPC
7. Comment saved to backend via IPC

### Example 2: Remote Gerrit Review

1. User switches to remote mode → `mode` = 'remote'
2. Gerrit changes loaded → `gerritChanges` populated
3. User selects change → `selectedChangeNumber` set
4. User selects file → IPC call to fetch Gerrit diff → `DiffLine[]`
5. DiffView renders with Gerrit-specific metadata
6. User adds inline comment → IPC call to Gerrit API
7. Comment submitted to Gerrit server via IPC

---

## Validation Rules Summary

| Entity | Key Validation Rules |
|--------|---------------------|
| ApplicationMode | Must be 'local' or 'remote' |
| Repository | `path` must be valid, `isLoaded` must be true for operations |
| GerritChange | `changeNumber` unique, `status` valid, `files` non-empty |
| DiffContext | `base` and `head` must be valid git references |
| DiffLine | At least one line number set, `content` non-empty (non-header) |
| Task | Exactly one of `repository` or `gerritChange` based on `mode` |
| ReviewComment | `filePath` valid, `lineNumber` >= 0, `content` non-empty |
| PanelConfig | `width` >= 200px and <= 50% viewport |
| UserSettings | `fontSize` between 10-24px, `language` supported |
| GerritServerConfig | `url` valid, `username` non-empty, `port` 1-65535 |

---

## Migration Strategy

### From Current Frontend

1. **Preserve** existing Zustand stores and entities
2. **Add** `ApplicationMode` type and mode switching logic
3. **Add** `RemoteModeState` store for Gerrit operations
4. **Update** `LocalModeState` (if needed) for consistency
5. **Preserve** all existing IPC integrations

### From HyperReview_Frontend

1. **Adopt** component structure (LocalToolBar, RemoteToolBar, etc.)
2. **Adopt** mode-based UI rendering logic
3. **Preserve** panel resizing and configuration patterns
4. **Integrate** with existing IPC client and services

---

**End of Data Model**
