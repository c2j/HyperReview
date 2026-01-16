# HyperReview IPC Protocol Documentation

This document describes the Inter-Process Communication (IPC) interface between the HyperReview frontend (React/TypeScript) and the backend Core (Rust/Tauri).

## 1. Overview

Communication is handled via the Tauri `invoke` command.
- **Frontend**: Calls `window.__TAURI__.invoke('command_name', { args })`.
- **Backend**: Rust functions annotated with `#[tauri::command]`.
- **Serialization**: All data is serialized as JSON.

## 2. Shared Data Structures

The TypeScript definitions are located in `src/api/types/`. The Rust backend must serialize data matching these interfaces.

### 2.1 Core Entities

**Repo**
```ts
interface Repo {
  path: string;       // Absolute path on disk
  branch: string;     // Current active branch
  lastOpened: string; // Human readable string (e.g., "2 mins ago")
}
```

**DiffLine**
```ts
interface DiffLine {
  oldLineNumber?: number;
  newLineNumber?: number;
  content: string;
  type: 'added' | 'removed' | 'context' | 'header';
  severity?: 'ERROR' | 'WARNING' | 'INFO' | 'SUCCESS';
  message?: string; // Analysis message for the specific line
}
```

**Task**
```ts
interface Task {
  id: string;
  title: string;
  status: 'active' | 'pending' | 'completed' | 'blocked';
  unreadCount?: number;
}
```

## 3. Command Reference

### 3.1 Repository Management

| Command | Arguments | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `open_repo_dialog` | None | `string | null` | Opens the native OS directory picker. Returns the absolute path of the selected directory or `null` if cancelled. |
| `get_recent_repos` | None | `Repo[]` | Returns a list of recently opened repositories stored in the local config. |
| `get_branches` | None | `Branch[]` | Returns the list of local and remote branches for the currently active repository. |

### 3.2 Task & Review Workflow

| Command | Arguments | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `get_tasks` | `{ type: 'pending' \| 'watched' }` | `Task[]` | Fetches tasks. `pending` returns PRs awaiting review; `watched` returns followed changes. |
| `get_file_diff` | `{ fileId: string }` | `DiffLine[]` | Computes the diff for a specific file. The backend should handle git diff logic and attach static analysis results (severity/message). |
| `get_review_stats` | None | `ReviewStats` | Returns aggregate statistics for the current review session (count of reviewed files, severe issues, etc.). |
| `get_quality_gates` | None | `QualityGate[]` | Checks the status of CI pipelines, test coverage, and security scanners. |

### 3.3 Insights & Analysis

| Command | Arguments | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `get_heatmap` | None | `HeatmapItem[]` | Returns architectural impact analysis data. |
| `get_blame` | `{ fileId: string }` | `BlameInfo` | Fetches git blame information for the current file context. |
| `get_checklist` | None | `ChecklistItem[]` | Returns a smart checklist generated based on the file types modified in the current PR. |

### 3.4 Configuration & Tools

| Command | Arguments | Return Type | Description |
| :--- | :--- | :--- | :--- |
| `get_tags` | None | `Tag[]` | Returns configured quick tags (e.g., "N+1 Problem", "Security Risk"). |
| `get_commands` | None | `SearchResult[]` | Returns indexable items for the Command Palette (files, symbols, commands). |
| `get_review_templates` | None | `ReviewTemplate[]` | Returns canned responses/templates for code review comments. |

## 4. Error Handling

If a command fails, the Promise rejects with an Error object.
```json
{
  "code": "GIT_ERROR",
  "message": "Failed to resolve reference 'HEAD'"
}
```
