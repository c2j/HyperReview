# Quick Start Guide: Local Task Management

**Feature**: Local Task Management | **Date**: 2025-12-15 | **Branch**: 003-local-task-management

## Overview

This guide helps developers quickly integrate Local Task Management functionality into HyperReview. The feature enables creating and managing local code review tasks via text import, supporting offline review of arbitrary code sections.

## Architecture

### Tech Stack

- **Frontend**: React 18+ + TypeScript 5+ + Vite
- **Backend**: Rust 1.75+ + Tauri v2
- **State Management**: Zustand (frontend)
- **Storage**: JSON files in `~/.hyperreview/local_tasks/`
- **Git Operations**: git2-rs

### Project Structure

```
src/
├── components/
│   ├── TaskPanel.tsx          # Left sidebar task list
│   ├── CreateTaskModal.tsx    # Task creation dialog
│   ├── TaskItem.tsx           # Individual task display
│   ├── TaskContextMenu.tsx    # Right-click menu
│   └── TaskProgress.tsx       # Progress indicator
├── hooks/
│   ├── useLocalTasks.ts       # Task CRUD operations
│   ├── useTaskProgress.ts     # Progress tracking
│   └── useTextParser.ts       # Text parsing
├── store/
│   ├── taskStore.ts           # Zustand store
│   └── uiStore.ts             # UI state
└── services/
    ├── ipc.ts                 # IPC interface
    └── taskService.ts         # Frontend service

src-tauri/src/
├── commands/
│   ├── task_commands.rs       # Task operations
│   ├── text_parser.rs         # Text parsing
│   └── file_utils.rs          # File utilities
├── models/
│   ├── task.rs                # Data models
│   └── parser.rs              # Parser models
├── git/
│   ├── repo_manager.rs        # Git operations
│   └── branch_handler.rs      # Branch handling
└── storage/
    ├── task_store.rs          # JSON storage
    └── progress_tracker.rs    # Progress persistence
```

## For Frontend Developers

### 1. Import Task Hooks

```typescript
// hooks/useLocalTasks.ts
import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import type { Task, TaskSummary, TaskItem } from '../types';

export function useLocalTasks() {
  const [tasks, setTasks] = useState<TaskSummary[]>([]);
  const [loading, setLoading] = useState(false);

  // Create a new task
  const createTask = async (payload: {
    name: string;
    repo_path: string;
    base_ref: string;
    items_text: string;
  }): Promise<Task> => {
    setLoading(true);
    try {
      const task = await invoke<Task>('create_task', { payload });
      // Refresh task list
      await loadTasks();
      return task;
    } finally {
      setLoading(false);
    }
  };

  // Load all tasks
  const loadTasks = async (status?: string) => {
    setLoading(true);
    try {
      const result = await invoke<TaskSummary[]>('list_tasks', {
        status: status || 'all'
      });
      setTasks(result);
    } finally {
      setLoading(false);
    }
  };

  // Get specific task
  const getTask = async (taskId: string): Promise<Task> => {
    return await invoke<Task>('get_task', { taskId });
  };

  // Update progress
  const updateProgress = async (
    taskId: string,
    itemIndex: number,
    reviewed: boolean
  ) => {
    await invoke('update_task_progress', {
      taskId,
      payload: { item_index: itemIndex, reviewed }
    });
  };

  // Delete task
  const deleteTask = async (taskId: string) => {
    await invoke('delete_task', { taskId });
    await loadTasks();
  };

  // Archive task
  const archiveTask = async (taskId: string) => {
    await invoke('archive_task', { taskId });
    await loadTasks();
  };

  // Export task
  const exportTask = async (taskId: string): Promise<string> => {
    return await invoke<string>('export_task', { taskId });
  };

  return {
    tasks,
    loading,
    createTask,
    loadTasks,
    getTask,
    updateProgress,
    deleteTask,
    archiveTask,
    exportTask
  };
}
```

### 2. Create Task Modal Component

```tsx
// components/CreateTaskModal.tsx
import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { useLocalTasks } from '../hooks/useLocalTasks';

interface CreateTaskModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export function CreateTaskModal({ isOpen, onClose }: CreateTaskModalProps) {
  const { createTask } = useLocalTasks();
  const [name, setName] = useState('');
  const [repoPath, setRepoPath] = useState('');
  const [baseRef, setBaseRef] = useState('main');
  const [itemsText, setItemsText] = useState('');
  const [branches, setBranches] = useState<string[]>([]);
  const [parseResult, setParseResult] = useState<any>(null);
  const [error, setError] = useState('');

  // Parse text when input changes
  const handleTextChange = async (text: string) => {
    setItemsText(text);
    if (text.trim()) {
      try {
        const result = await invoke('parse_task_text', { text });
        setParseResult(result);
      } catch (e) {
        setParseResult(null);
      }
    }
  };

  // Validate repository
  const validateRepository = async () => {
    try {
      const result = await invoke('validate_repository', { path: repoPath });
      if (result.valid) {
        // Load branches
        const branchesResult = await invoke('list_branches', { repo_path: repoPath });
        setBranches(branchesResult.branches);
        setError('');
      } else {
        setError('Invalid repository path');
      }
    } catch (e) {
      setError('Failed to validate repository');
    }
  };

  // Create task
  const handleCreate = async () => {
    try {
      await createTask({
        name,
        repo_path: repoPath,
        base_ref: baseRef,
        items_text: itemsText
      });
      onClose();
      // Reset form
      setName('');
      setRepoPath('');
      setItemsText('');
      setParseResult(null);
    } catch (e) {
      setError(e.message || 'Failed to create task');
    }
  };

  if (!isOpen) return null;

  return (
    <div className="modal-overlay">
      <div className="modal-content">
        <h2>创建本地审查任务</h2>

        {/* Task Name */}
        <div className="form-group">
          <label>任务名称（必填）</label>
          <input
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
            placeholder="输入任务名称"
          />
        </div>

        {/* Repository Path */}
        <div className="form-group">
          <label>目标仓库（必填）</label>
          <div className="input-group">
            <input
              type="text"
              value={repoPath}
              onChange={(e) => setRepoPath(e.target.value)}
              placeholder="/path/to/repo"
            />
            <button onClick={validateRepository}>验证</button>
          </div>
        </div>

        {/* Base Ref */}
        <div className="form-group">
          <label>分支/Tag/Commit（必填）</label>
          <select value={baseRef} onChange={(e) => setBaseRef(e.target.value)}>
            <option value="main">main</option>
            {branches.map((branch) => (
              <option key={branch} value={branch}>
                {branch}
              </option>
            ))}
          </select>
        </div>

        {/* Text Import */}
        <div className="form-group">
          <label>任务描述文本（支持直接粘贴多行）</label>
          <textarea
            value={itemsText}
            onChange={(e) => handleTextChange(e.target.value)}
            placeholder="file/path.java\t10-20\tComment\thigh\ttag1,tag2"
            rows={10}
          />
          {parseResult && (
            <div className="parse-preview">
              <p>
                解析预览：{parseResult.success_count} 项成功，{parseResult.error_count} 项错误
              </p>
            </div>
          )}
        </div>

        {/* Error Message */}
        {error && <div className="error-message">{error}</div>}

        {/* Actions */}
        <div className="modal-actions">
          <button onClick={onClose}>取消</button>
          <button
            onClick={handleCreate}
            disabled={!name || !repoPath || !baseRef || !itemsText}
          >
            创建任务
          </button>
        </div>
      </div>
    </div>
  );
}
```

### 3. Task Panel Component

```tsx
// components/TaskPanel.tsx
import React from 'react';
import { useLocalTasks } from '../hooks/useLocalTasks';

export function TaskPanel() {
  const { tasks, loadTasks } = useLocalTasks();

  React.useEffect(() => {
    loadTasks();
  }, []);

  return (
    <div className="task-panel">
      {/* Header with create button */}
      <div className="task-panel-header">
        <h3>本地任务</h3>
        <button onClick={() => setShowCreateModal(true)}>
          + 创建本地审查任务
        </button>
      </div>

      {/* Task List */}
      <div className="task-list">
        {tasks.map((task) => (
          <div
            key={task.id}
            className={`task-item ${task.status}`}
            onClick={() => selectTask(task.id)}
          >
            <div className="task-icon">📁</div>
            <div className="task-info">
              <div className="task-name">{task.name}</div>
              <div className="task-progress">
                {task.completed_items}/{task.total_items} 已完成
              </div>
              <div className="task-status">
                {task.status === 'in_progress' && '进行中'}
                {task.status === 'completed' && '已完成'}
                {task.status === 'archived' && '已归档'}
              </div>
            </div>
            {task.status === 'completed' && (
              <div className="task-complete-icon">✓</div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}
```

### 4. Type Definitions

```typescript
// types/task.ts
export interface Task {
  id: string;
  name: string;
  repo_path: string;
  base_ref: string;
  create_time: string;
  update_time: string;
  status: 'in_progress' | 'completed' | 'archived';
  total_items: number;
  completed_items: number;
  items: TaskItem[];
}

export interface TaskSummary {
  id: string;
  name: string;
  status: 'in_progress' | 'completed' | 'archived';
  total_items: number;
  completed_items: number;
  repo_path: string;
  base_ref: string;
  create_time: string;
  update_time: string;
}

export interface TaskItem {
  file: string;
  line_range?: {
    start?: number;
    end?: number;
  };
  preset_comment?: string;
  severity?: 'error' | 'warning' | 'question' | 'ok';
  tags: string[];
  reviewed: boolean;
  comments: Comment[];
}

export interface Comment {
  id: string;
  author: string;
  content: string;
  created_at: string;
  line_number?: number;
}
```

## For Rust Developers

### 1. Add Dependencies

```toml
# Cargo.toml
[dependencies]
# Existing dependencies...
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.0", features = ["v4"] }
tokio = { version = "1.0", features = ["fs"] }
fs4 = { version = "0.8", features = ["tokio"] }
thiserror = "1.0"
log = "0.4"
directories = "5.0"

[dependencies.git2]
version = "0.18"
features = ["https", "ssh", "fast-sha1"]
```

### 2. Define Models

```rust
// src-tauri/src/models/task.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub repo_path: String,
    pub base_ref: String,
    pub create_time: String,
    pub update_time: String,
    pub status: LocalTaskStatus,
    pub total_items: u32,
    pub completed_items: u32,
    pub items: Vec<TaskItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskItem {
    pub file: String,
    pub line_range: Option<LineRange>,
    pub preset_comment: Option<String>,
    pub severity: Option<TaskSeverity>,
    pub tags: Vec<String>,
    pub reviewed: bool,
    pub comments: Vec<Comment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineRange {
    pub start: Option<u32>,
    pub end: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub author: String,
    pub content: String,
    pub created_at: String,
    pub line_number: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocalTaskStatus {
    InProgress,
    Completed,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskSeverity {
    Error,
    Warning,
    Question,
    Ok,
}
```

### 3. Implement Task Commands

```rust
// src-tauri/src/commands/task_commands.rs
use crate::models::task::{Task, TaskSummary, TaskItem};
use crate::storage::task_store::{TaskStore, TaskStoreError};
use fs4::tokio::FileExt;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use tauri::command;
use tokio::fs::File;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub name: String,
    pub repo_path: String,
    pub base_ref: String,
    pub items_text: String,
}

#[command]
pub async fn create_task(payload: CreateTaskRequest) -> Result<Task, String> {
    // Validate inputs
    if payload.name.is_empty() {
        return Err("Task name is required".to_string());
    }

    // Parse text
    let items = parse_task_text(&payload.items_text)
        .map_err(|e| format!("Failed to parse text: {}", e))?;

    // Validate repository
    let repo = validate_repository(&payload.repo_path)
        .await
        .map_err(|e| format!("Invalid repository: {}", e))?;

    if !repo.is_valid {
        return Err("Not a valid git repository".to_string());
    }

    // Validate ref
    let ref_valid = validate_ref(&payload.repo_path, &payload.base_ref)
        .await
        .map_err(|e| format!("Invalid reference: {}", e))?;

    if !ref_valid.valid {
        return Err("Reference not found in repository".to_string());
    }

    // Create task
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let task = Task {
        id: Uuid::new_v4(),
        name: payload.name,
        repo_path: payload.repo_path,
        base_ref: payload.base_ref,
        create_time: chrono::DateTime::from_timestamp(now as i64, 0)
            .unwrap()
            .to_rfc3339(),
        update_time: chrono::DateTime::from_timestamp(now as i64, 0)
            .unwrap()
            .to_rfc3339(),
        status: LocalTaskStatus::InProgress,
        total_items: items.len() as u32,
        completed_items: 0,
        items,
    };

    // Save to file
    let store = TaskStore::new();
    store.save_task(&task)
        .map_err(|e| format!("Failed to save task: {}", e))?;

    Ok(task)
}

#[command]
pub async fn list_tasks(status: Option<String>) -> Result<Vec<TaskSummary>, String> {
    let store = TaskStore::new();
    store.list_tasks(status.as_deref())
        .map_err(|e| format!("Failed to list tasks: {}", e))
}

#[command]
pub async fn get_task(task_id: String) -> Result<Task, String> {
    let store = TaskStore::new();
    let task_id = Uuid::parse_str(&task_id)
        .map_err(|_| "Invalid task ID".to_string())?;

    store.get_task(task_id)
        .map_err(|e| format!("Failed to get task: {}", e))
}

#[command]
pub async fn update_task_progress(
    task_id: String,
    payload: UpdateProgressRequest,
) -> Result<ProgressUpdateResult, String> {
    let store = TaskStore::new();
    let task_id = Uuid::parse_str(&task_id)
        .map_err(|_| "Invalid task ID".to_string())?;

    store.update_progress(task_id, payload.item_index, payload.reviewed)
        .map_err(|e| format!("Failed to update progress: {}", e))
}
```

### 4. Implement Text Parser

```rust
// src-tauri/src/commands/text_parser.rs
use crate::models::task::{TaskItem, LineRange, TaskSeverity};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Empty line at position {0}")]
    EmptyLine(usize),
    #[error("Missing file path at line {0}")]
    MissingFilePath(usize),
    #[error("Invalid line range at line {0}")]
    InvalidLineRange(usize),
}

pub fn parse_task_text(input: &str) -> Result<Vec<TaskItem>, ParseError> {
    let estimated = estimate_valid_lines(input);
    let mut items = Vec::with_capacity(estimated);

    for (line_num, line) in input.lines().enumerate() {
        let trimmed = line.trim();

        // Skip empty and comment lines
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.as_bytes().first() == Some(&b'#') {
            continue;
        }

        let item = parse_task_line(trimmed, line_num + 1)?;
        items.push(item);
    }

    Ok(items)
}

fn parse_task_line(line: &str, line_num: usize) -> Result<TaskItem, ParseError> {
    let fields: Vec<&str> = if line.contains('\t') {
        line.split('\t').map(str::trim).collect()
    } else {
        line.split_whitespace().collect()
    };

    if fields.is_empty() {
        return Err(ParseError::EmptyLine(line_num));
    }

    let file = fields[0];
    if file.is_empty() {
        return Err(ParseError::MissingFilePath(line_num));
    }

    let line_range = parse_optional_field(&fields, 1, parse_line_range_str)
        .transpose()
        .map_err(|_| ParseError::InvalidLineRange(line_num))?;

    let preset_comment = parse_optional_field(&fields, 2, |s| Ok(s.to_string()))
        .transpose()
        .ok()
        .flatten();

    let severity = parse_optional_field(&fields, 3, |s| {
        match s.to_lowercase().as_str() {
            "✗" | "error" | "high" => Ok(TaskSeverity::Error),
            "⚠" | "warning" | "medium" => Ok(TaskSeverity::Warning),
            "❓" | "question" | "low" => Ok(TaskSeverity::Question),
            "✓" | "ok" => Ok(TaskSeverity::Ok),
            _ => Err(()),
        }
    }).transpose().ok().flatten();

    let tags = fields.get(4)
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split(',')
                .map(str::trim)
                .filter(|t| !t.is_empty())
                .map(String::from)
                .collect()
        })
        .unwrap_or_default();

    Ok(TaskItem {
        file: file.to_string(),
        line_range,
        preset_comment,
        severity,
        tags,
        reviewed: false,
        comments: vec![],
    })
}

#[inline]
fn parse_optional_field<T, F>(
    fields: &[&str],
    index: usize,
    parser: F,
) -> Option<Result<T, ()>>
where
    F: FnOnce(&str) -> Result<T, ()>,
{
    fields.get(index)
        .filter(|s| !s.is_empty())
        .map(|s| parser(s))
}

fn parse_line_range_str(s: &str) -> Result<LineRange, ()> {
    if s.contains('-') {
        let mut parts = s.splitn(2, '-');
        let start = parts.next().and_then(|p| p.parse().ok());
        let end = parts.next().and_then(|p| p.parse().ok());

        if start.is_none() && end.is_none() {
            return Err(());
        }

        Ok(LineRange { start, end })
    } else {
        let line = s.parse::<u32>().map_err(|_| ())?;
        Ok(LineRange {
            start: Some(line),
            end: Some(line),
        })
    }
}

#[inline]
fn estimate_valid_lines(input: &str) -> usize {
    let newlines = input.bytes().filter(|&b| b == b'\n').count();
    (newlines + 1) * 4 / 5
}
```

### 5. Update lib.rs

```rust
// src-tauri/src/lib.rs
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod models;
mod git;
mod storage;

use commands::{
    task_commands::*,
    text_parser::*,
};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // Task management
            create_task,
            list_tasks,
            get_task,
            update_task_progress,
            delete_task,
            archive_task,
            export_task,
            // Text parsing
            parse_task_text,
            // Git operations
            validate_repository,
            list_branches,
            validate_ref,
            read_file_from_ref,
            // File operations
            ensure_directory,
            write_task_file,
            read_task_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## Testing

### Frontend Tests

```typescript
// tests/useLocalTasks.test.ts
import { renderHook, act } from '@testing-library/react';
import { useLocalTasks } from '../hooks/useLocalTasks';

test('should create a task', async () => {
  const { result } = renderHook(() => useLocalTasks());

  await act(async () => {
    const task = await result.current.createTask({
      name: 'Test Task',
      repo_path: '/tmp/test-repo',
      base_ref: 'main',
      items_text: 'file.java\t10-20\tComment\thigh\ttag1'
    });

    expect(task.name).toBe('Test Task');
    expect(task.items).toHaveLength(1);
  });
});
```

### Rust Tests

```rust
// src-tauri/src/commands/text_parser_test.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_task_text() {
        let input = "file1.java\t10-20\tComment\thigh\ttag1,tag2\nfile2.ts\t30\tComment2";
        let items = parse_task_text(input).unwrap();

        assert_eq!(items.len(), 2);
        assert_eq!(items[0].file, "file1.java");
        assert_eq!(items[0].line_range, Some(LineRange { start: Some(10), end: Some(20) }));
        assert_eq!(items[0].tags, vec!["tag1", "tag2"]);
    }
}
```

## Configuration

### Tauri Permissions

```json
// src-tauri/capabilities/local_tasks.json
{
  "permissions": [
    "core:default",
    "shell:default",
    "fs:default",
    "fs:read-file",
    "fs:write-file",
    "fs:create-dir",
    "fs:read-text-file",
    "fs:write-text-file"
  ]
}
```

### Update tauri.conf.json

```json
{
  "tauri": {
    "allowlist": {
      "all": false,
      "fs": {
        "scope": ["~/.hyperreview/local_tasks/**"]
      },
      "shell": {
        "all": false
      }
    }
  }
}
```

## Common Patterns

### Progress Tracking

```typescript
// Update progress after reviewing item
await updateProgress(taskId, itemIndex, true);

// Auto-advance to next item
if (currentItemIndex < items.length - 1) {
  setCurrentItemIndex(currentItemIndex + 1);
  await loadFile(items[currentItemIndex + 1]);
}
```

### Keyboard Shortcut

```typescript
// Listen for Ctrl+Enter to mark reviewed
useEffect(() => {
  const handler = async (e: KeyboardEvent) => {
    if (e.key === 'Enter' && e.ctrlKey) {
      await updateProgress(taskId, currentItemIndex, true);
      // Advance to next item
    }
  };

  window.addEventListener('keydown', handler);
  return () => window.removeEventListener('keydown', handler);
}, [taskId, currentItemIndex]);
```

## Troubleshooting

### Issue: "Repository not found"

**Solution**: Ensure repository path is absolute and directory exists

```rust
let path = std::path::Path::new(&repo_path);
if !path.exists() || !path.is_dir() {
    return Err("Repository path does not exist".to_string());
}
```

### Issue: "File locked by another instance"

**Solution**: Implement file locking with fs4

```rust
use fs4::tokio::FileExt;

let file = File::open(&path).await?;
file.lock_exclusive().await?;
```

### Issue: "Parsing too slow"

**Solution**: Use manual parsing instead of regex

```rust
// ✅ Fast: Manual parsing
line.split('\t').map(str::trim).collect()

// ❌ Slow: Regex
Regex::new(pattern).unwrap()
```

## Best Practices

1. **Always validate inputs** before processing
2. **Use async/await** for all IPC calls
3. **Handle errors gracefully** with user-friendly messages
4. **Update progress frequently** to prevent data loss
5. **Cache task data** in frontend for better performance
6. **Use TypeScript types** for compile-time safety
7. **Write tests** for all critical functions
8. **Follow Rust naming conventions** (snake_case for functions)
9. **Use Result<T, String>** for error handling in Rust
10. **Document complex logic** with comments

## Next Steps

1. Review the [data model](data-model.md) for entity definitions
2. Check the [IPC interface](contracts/ipc-interface.yaml) for API details
3. Read the [research document](research.md) for implementation insights
4. Run tests to verify functionality
5. Integrate with existing PR/MR review workflow

## Support

- **Frontend Issues**: Check React/TypeScript errors in browser console
- **Backend Issues**: Check Rust logs in Tauri terminal
- **IPC Issues**: Verify tauri.conf.json allowlist configuration
- **Git Issues**: Ensure repository is valid and accessible

For more details, see the full [specification](spec.md).
