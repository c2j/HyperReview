# Data Model: Local Task Management

**Feature**: Local Task Management | **Date**: 2025-12-15 | **Branch**: 003-local-task-management

## Entity Relationship Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                        LocalTask                                 │
├─────────────────────────────────────────────────────────────────┤
│ - id: UUID (PK)                                                  │
│ - name: String                                                   │
│ - repo_path: String                                              │
│ - base_ref: String                                               │
│ - create_time: ISO 8601 String                                   │
│ - update_time: ISO 8601 String                                   │
│ - status: LocalTaskStatus (Enum)                                │
│ - total_items: u32                                               │
│ - completed_items: u32                                           │
│ - items: Vec<TaskItem> (1:N)                                     │
└─────────────────────────────────────────────────────────────────┘
                            │
                            │ has many
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                        TaskItem                                  │
├─────────────────────────────────────────────────────────────────┤
│ - file: String                                                   │
│ - line_range: Option<LineRange>                                  │
│ - preset_comment: Option<String>                                 │
│ - severity: Option<TaskSeverity> (Enum)                          │
│ - tags: Vec<String>                                              │
│ - reviewed: Boolean                                              │
│ - comments: Vec<Comment> (1:N)                                   │
└─────────────────────────────────────────────────────────────────┘
                            │
                            │ has many
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                         Comment                                  │
├─────────────────────────────────────────────────────────────────┤
│ - id: UUID                                                       │
│ - author: String                                                 │
│ - content: String                                                │
│ - created_at: ISO 8601 String                                    │
│ - line_number: Option<u32>                                       │
└─────────────────────────────────────────────────────────────────┘
```

## Core Entities

### 1. LocalTask

**Description**: Represents a collection of review items for a specific repository and branch/commit.

**Attributes**:
- `id`: UUID (Primary Key)
  - Type: UUID v4
  - Generated automatically when task is created
  - Used as filename: `{id}.json`
  - Unique across all tasks

- `name`: String
  - Type: UTF-8 String
  - Required field
  - User-defined task name
  - Example: "支付系统雷区清理", "SQL性能审计 Q4"
  - Max length: 255 characters

- `repo_path`: String
  - Type: Absolute file system path
  - Required field
  - Path to git repository root
  - Example: "/Users/alice/projects/payment-service"
  - Format: Valid absolute path, directory must exist

- `base_ref`: String
  - Type: String (git reference)
  - Required field
  - Branch, commit hash, or tag name
  - Example: "main", "develop", "a1b2c3d4e5f6"
  - Must exist in the repository

- `create_time`: ISO 8601 String
  - Type: RFC 3339 formatted timestamp
  - Required field
  - Auto-generated when task is created
  - Example: "2025-12-15T10:30:00Z"
  - UTC timezone

- `update_time`: ISO 8601 String
  - Type: RFC 3339 formatted timestamp
  - Required field
  - Auto-updated when task is modified
  - Example: "2025-12-15T11:45:30Z"
  - UTC timezone

- `status`: LocalTaskStatus (Enum)
  - Type: Enumeration
  - Required field
  - Values: `in_progress`, `completed`, `archived`
  - Default: `in_progress`

- `total_items`: u32
  - Type: Unsigned 32-bit integer
  - Required field
  - Total number of task items
  - Range: 1 to 10,000
  - Cannot be 0

- `completed_items`: u32
  - Type: Unsigned 32-bit integer
  - Required field
  - Number of reviewed items
  - Range: 0 to `total_items`
  - Default: 0

- `items`: Vec<TaskItem>
  - Type: Vector of TaskItem
  - Required field
  - List of review targets
  - Owned by task (lifecycle tied to task)

**Validation Rules**:
- `name` cannot be empty
- `repo_path` must be an absolute path
- `repo_path` directory must exist and be a valid git repository
- `base_ref` must exist in the repository
- `completed_items` cannot exceed `total_items`
- `total_items` must be between 1 and 10,000
- UUID must be unique

**State Transitions**:
```
in_progress ──→ completed ──→ archived
    ↑              │
    │              │
    └──────────────┘
```

**JSON Representation**:
```json
{
  "id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "name": "支付系统雷区清理",
  "repo_path": "/Users/alice/projects/payment-service",
  "base_ref": "main",
  "create_time": "2025-12-15T10:30:00Z",
  "update_time": "2025-12-15T11:45:30Z",
  "status": "in_progress",
  "total_items": 127,
  "completed_items": 73,
  "items": [...]
}
```

---

### 2. TaskItem

**Description**: Represents a single file review unit within a local task.

**Attributes**:
- `file`: String
  - Type: Relative file path
  - Required field
  - Relative to repository root
  - Example: "src/main/java/com/pay/Retry.java"
  - Format: Unix-style path (forward slashes)
  - Must exist in the repository at `base_ref`

- `line_range`: Option<LineRange>
  - Type: Optional struct
  - Optional field
  - Specific line range to review
  - Example: `{ "start": 124, "end": 189 }`
  - Format: `{ start: Option<u32>, end: Option<u32> }`

- `preset_comment`: Option<String>
  - Type: Optional UTF-8 String
  - Optional field
  - Pre-filled review question/comment
  - Example: "潜在N+1风险"
  - Max length: 1000 characters
  - Can be empty string (treated as None)

- `severity`: Option<TaskSeverity> (Enum)
  - Type: Optional enumeration
  - Optional field
  - Pre-assigned severity level
  - Values: `error`, `warning`, `question`, `ok`
  - Example: "error" (represented as ✗ in UI)

- `tags`: Vec<String>
  - Type: Vector of UTF-8 Strings
  - Optional field (empty vector allowed)
  - Custom tags for categorization
  - Example: ["N+1", "性能"]
  - Max tags: 10 per item
  - Max tag length: 50 characters

- `reviewed`: Boolean
  - Type: Boolean
  - Required field
  - Whether this item has been reviewed
  - Default: false
  - Used for progress tracking

- `comments`: Vec<Comment>
  - Type: Vector of Comment
  - Optional field (empty vector allowed)
  - Review comments added during review
  - Chronological order (by created_at)

**LineRange Struct**:
```rust
struct LineRange {
    start: Option<u32>,  // Starting line number (1-indexed)
    end: Option<u32>,    // Ending line number (1-indexed)
}
```

**Validation Rules**:
- `file` cannot be empty
- `file` must exist in repository at `base_ref`
- If `line_range` is Some:
  - If both `start` and `end` are present: `start <= end`
  - `start` must be >= 1
  - `end` must be >= 1
  - If only `start` is present: review single line
  - If only `end` is present: review from line 1 to `end`
- `tags` vector cannot exceed 10 items
- Each tag must be non-empty and <= 50 chars

**JSON Representation**:
```json
{
  "file": "src/main/java/com/pay/Retry.java",
  "line_range": {
    "start": 124,
    "end": 189
  },
  "preset_comment": "潜在N+1风险",
  "severity": "error",
  "tags": ["N+1", "性能"],
  "reviewed": false,
  "comments": [...]
}
```

---

### 3. Comment

**Description**: Represents a review comment added to a task item.

**Attributes**:
- `id`: UUID
  - Type: UUID v4
  - Required field
  - Unique identifier for comment
  - Generated automatically

- `author`: String
  - Type: UTF-8 String
  - Required field
  - Reviewer name or email
  - Example: "Alice Chen", "alice@example.com"
  - Max length: 255 characters

- `content`: String
  - Type: UTF-8 String
  - Required field
  - Comment text
  - Example: "这里使用了N+1查询，建议使用JOIN优化"
  - Max length: 5000 characters

- `created_at`: ISO 8601 String
  - Type: RFC 3339 formatted timestamp
  - Required field
  - Auto-generated when comment is created
  - Example: "2025-12-15T10:35:00Z"
  - UTC timezone

- `line_number`: Option<u32>
  - Type: Optional unsigned 32-bit integer
  - Optional field
  - Specific line within the file
  - Example: 156
  - Must be >= 1
  - Corresponds to line in `file` path

**Validation Rules**:
- `author` cannot be empty
- `content` cannot be empty
- If `line_number` is present: must be >= 1
- Comments are immutable once created (no update/delete)

**JSON Representation**:
```json
{
  "id": "b2c3d4e5-f6a7-8901-bcde-f01234567890",
  "author": "Alice Chen",
  "content": "这里使用了N+1查询，建议使用JOIN优化",
  "created_at": "2025-12-15T10:35:00Z",
  "line_number": 156
}
```

---

## Enums

### LocalTaskStatus

**Values**:
- `in_progress`: Task is actively being reviewed
- `completed`: All items have been reviewed
- `archived`: Task is completed and archived (historical record)

**Usage**:
- Used in `LocalTask.status` field
- Determines UI display and available actions
- Affects filtering and sorting in task list

---

### TaskSeverity

**Values**:
- `error`: Critical issue requiring immediate attention (✗)
- `warning`: Potential issue that should be addressed (⚠)
- `question`: Point requiring clarification or discussion (❓)
- `ok`: No issues found, approved (✓)

**Usage**:
- Used in `TaskItem.severity` field
- Pre-assigned during task import
- Can be updated during review
- Affects UI color coding and filtering

**Severity Priority** (for sorting):
1. `error` (highest priority)
2. `warning`
3. `question`
4. `ok` (lowest priority)

---

## Storage Model

### File Structure

```
~/.hyperreview/
└── local_tasks/
    ├── {task_id}.json    # Individual task file
    └── index.json        # Task index (optional)
```

### Task File Schema

Each task is stored as a JSON file with the following structure:

```json
{
  "id": "UUID",
  "name": "String",
  "repo_path": "String",
  "base_ref": "String",
  "create_time": "ISO 8601",
  "update_time": "ISO 8601",
  "status": "in_progress|completed|archived",
  "total_items": 127,
  "completed_items": 73,
  "items": [
    {
      "file": "String",
      "line_range": {
        "start": 124,
        "end": 189
      },
      "preset_comment": "String",
      "severity": "error|warning|question|ok",
      "tags": ["tag1", "tag2"],
      "reviewed": false,
      "comments": [
        {
          "id": "UUID",
          "author": "String",
          "content": "String",
          "created_at": "ISO 8601",
          "line_number": 156
        }
      ]
    }
  ]
}
```

### Index File Schema (Optional)

For faster task listing:

```json
{
  "version": 1,
  "last_updated": "ISO 8601",
  "tasks": [
    {
      "id": "UUID",
      "name": "String",
      "status": "in_progress|completed|archived",
      "total_items": 127,
      "completed_items": 73,
      "repo_path": "String",
      "base_ref": "String",
      "create_time": "ISO 8601",
      "update_time": "ISO 8601"
    }
  ]
}
```

---

## Relationships

### 1. LocalTask to TaskItem (1:N)

- A LocalTask has zero or more TaskItems (in practice: 1 to 10,000)
- Each TaskItem belongs to exactly one LocalTask
- When a LocalTask is deleted, all its TaskItems are deleted
- TaskItems cannot exist without a parent LocalTask

### 2. TaskItem to Comment (1:N)

- A TaskItem has zero or more Comments
- Each Comment belongs to exactly one TaskItem
- When a TaskItem is deleted, all its Comments are deleted
- Comments cannot exist without a parent TaskItem

### 3. TaskItem to Git File (1:1)

- Each TaskItem references one file in a git repository
- The file must exist at the specified `base_ref`
- TaskItem does not own the file content (read-only reference)

---

## Data Integrity Constraints

### Referential Integrity

1. **LocalTask → Git Repository**
   - `repo_path` must point to an existing directory
   - Directory must be a valid git repository
   - Must be accessible by the user

2. **LocalTask → Base Reference**
   - `base_ref` must exist in the repository
   - Can be branch name, commit hash, or tag

3. **TaskItem → File**
   - `file` path must exist in the repository at `base_ref`
   - File path is relative to repository root

4. **Comment → TaskItem**
   - If `line_number` is present, it must be a valid line in the file

### Business Rules

1. **Progress Tracking**
   - `completed_items` <= `total_items`
   - When all items are reviewed, status should be `completed`

2. **Unique Constraints**
   - Task `id` must be globally unique
   - Comment `id` must be unique within task

3. **Data Validation**
   - All timestamps must be valid ISO 8601 format
   - UUIDs must be valid v4 format
   - Line numbers must be positive integers

4. **File Size Limits**
   - Maximum 10,000 items per task
   - Maximum 10 tags per item
   - Maximum 1000 characters per preset comment
   - Maximum 5000 characters per comment

---

## Migration & Versioning

### Schema Evolution

If data model changes in future versions:

1. **Backward Compatibility**: Older task files should still be readable
2. **Migration Strategy**: Automatic migration on first access
3. **Version Field**: Include schema version in task file
4. **Default Values**: Use sensible defaults for optional fields

### Example Migration

```json
{
  "version": 2,
  "id": "UUID",
  "data": { ... }
}
```

---

## Performance Considerations

### Indexing

For efficient querying:
- Index by `status` for filtering active tasks
- Index by `repo_path` for repository-based grouping
- Index by `update_time` for recent activity sorting

### Caching

For frequently accessed data:
- Cache task list in memory (Zustand store)
- Cache task content when actively reviewing
- Invalidate cache on task updates

### Large Datasets

For tasks with many items (up to 10,000):
- Lazy load comments
- Virtual scrolling in UI
- Batch operations for progress updates
- Async file loading

---

## Summary

This data model provides:

✅ **Clear entity relationships** with proper cardinality
✅ **Validation rules** to ensure data integrity
✅ **Storage format** optimized for file-based storage
✅ **Extensibility** through optional fields and enums
✅ **Performance** considerations for large datasets
✅ **Versioning** strategy for future evolution

The model supports all 18 functional requirements while maintaining simplicity and clarity.
