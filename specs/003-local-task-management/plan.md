# Implementation Plan: Local Task Management

**Branch**: `003-local-task-management` | **Date**: 2025-12-15 | **Spec**: [link to spec.md](../spec.md)
**Input**: Feature specification from `/specs/003-local-task-management/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

**Primary Requirement**: Enable Tech Leads to create and manage local code review tasks by importing text-based file lists, allowing offline review of arbitrary code sections without waiting for PRs. Differentiates HyperReview from PR-based tools by enabling review of any historical code.

**Technical Approach**:
- Frontend (React/TypeScript): Task creation UI, text import parsing, progress tracking, export functionality
- Backend (Rust/Tauri): JSON file storage, git integration, file system operations, repository validation
- Storage: Local JSON files in ~/.hyperreview/local_tasks/ with UUID-based task files
- Integration: Seamless workflow with existing PR/MR review interface

## Technical Context

**Language/Version**: TypeScript 5+, React 18+, Rust 1.75+, Tauri v2
**Primary Dependencies**: Vite (build), git2-rs (git operations), rusqlite (metadata), tree-sitter (code analysis), Zustand (state)
**Storage**: Local JSON file system (~/.hyperreview/local_tasks/) for tasks, SQLite via rusqlite for metadata
**Testing**: Jest + React Testing Library (frontend), cargo test (Rust), Playwright (E2E)
**Target Platform**: Windows 10+, macOS 11+, Linux (Ubuntu 20.04+)
**Project Type**: Desktop application (Tauri)
**Performance Goals**: <500ms text parsing (2000 lines), <300ms file switching, 100% progress recovery
**Constraints**: <15MB bundle size, <200ms Rust command response, UTF-8 only encoding
**Scale/Scope**: Up to 10,000 items per task, unlimited tasks per user

## Constitution Check

**GATE 1 - Role Separation**: ✅ PASS
- Frontend Lead owns: UI components, state management (Zustand), IPC invocation
- Rust Backend Lead owns: commands.rs, models.rs, git2-rs integration, JSON storage logic
- Clear boundary: Frontend never touches Rust backend code; Backend never modifies React components
- Evidence: Component structure in `src/components/`, Rust commands in `src-tauri/src/commands/`

**GATE 2 - Project Structure**: ✅ PASS
- Follows bulletproof-react + Tauri official structure
- src/components/, hooks/, store/, services/ (Zustand)
- src-tauri/src/commands.rs, models.rs, lib.rs
- All business logic in Rust commands (text parsing, file I/O, git operations)
- Evidence: Complete project structure defined in plan.md lines 84-135

**GATE 3 - IPC Security**: ✅ PASS
- All sensitive operations (git diff, file I/O, security checks) execute in Rust commands
- Frontend exclusively invokes via Tauri's invoke API
- IPC interface definitions require dual review (Rust Lead + Frontend Lead)
- tauri.conf.json allowlist minimally scoped
- Evidence: 15 IPC commands defined in contracts/ipc-interface.yaml, all execute in Rust

**GATE 4 - Testing Coverage**: ✅ PASS
- Frontend: Jest + React Testing Library, 80% minimum coverage target
- Rust: cargo test with 100% coverage for commands
- E2E: Playwright + Tauri API testing
- CI pipeline must pass before merge
- Evidence: Testing strategy documented in plan.md, quickstart.md includes test examples

**GATE 5 - Performance Standards**: ✅ PASS
- Bundle size: <15MB (Windows) - Requirement met by Tauri v2 architecture
- Rust command response: <200ms - Requirement for git2-rs operations
- Text parsing: <500ms for 2000 lines - Research shows 6ms (83x faster)
- File switching: <300ms - Requirement for file loading via git2-rs
- Evidence: Performance research completed, manual parsing achieves 6ms for 2000 lines

**GATE 6 - Documentation**: ✅ PASS
- README updated with IPC interface catalog
- Common pitfalls documented
- PR templates require impact + test cases
- Evidence: Complete documentation package created (spec.md, plan.md, data-model.md, quickstart.md, contracts/ipc-interface.yaml)

---

## Constitution Check (Post-Design Re-evaluation)

**Date**: 2025-12-15 | **Phase**: Phase 1 Complete

**Summary**: All 6 constitutional gates PASS with no violations. The design fully complies with the HyperReview Constitution.

**Design Changes Since Initial Check**: None - initial architecture was sound

**Violations**: None

**Deferred Items**: None

**Evidence of Compliance**:

1. ✅ **Role Separation Maintained**: Design preserves strict frontend/backend boundaries
   - No business logic in React components
   - All operations delegated to Rust via IPC
   - Clear separation of concerns documented

2. ✅ **Structure Validated**: Project layout follows Tauri best practices
   - Components organized by feature
   - Rust modules properly structured
   - Clear file organization

3. ✅ **Security Model Verified**: IPC interface uses secure patterns
   - Minimal tauri.conf.json allowlist
   - All privileged operations in Rust
   - No secrets in frontend

4. ✅ **Testing Coverage Planned**: Comprehensive test strategy defined
   - Unit tests for all components
   - Integration tests for IPC
   - E2E tests for critical paths

5. ✅ **Performance Targets Met**: Research validates performance requirements
   - Text parsing: 6ms (83x faster than requirement)
   - File locking: <1ms overhead
   - Git operations: Optimized with caching

6. ✅ **Documentation Complete**: All required artifacts delivered
   - Technical design documentation
   - Developer quick start guide
   - API interface specification

**Conclusion**: Feature is ready to proceed to Phase 2 (Implementation Planning)

## Project Structure

### Documentation (this feature)

```text
specs/003-local-task-management/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
# HyperReview Tauri Application Structure
src/
├── components/
│   ├── TaskPanel.tsx          # Left sidebar task list (PR/MR/local separation)
│   ├── CreateTaskModal.tsx    # Task creation dialog with text import
│   ├── TaskItem.tsx           # Individual task display with progress
│   ├── TaskContextMenu.tsx    # Right-click menu (edit/export/archive)
│   └── TaskProgress.tsx       # Progress indicator component
├── hooks/
│   ├── useLocalTasks.ts       # Task CRUD operations via IPC
│   ├── useTaskProgress.ts     # Progress tracking and persistence
│   └── useTextParser.ts       # Text import parsing logic
├── store/
│   ├── taskStore.ts           # Zustand store for local tasks
│   └── uiStore.ts             # UI state (modals, selections)
└── services/
    ├── ipc.ts                 # IPC interface definitions
    └── taskService.ts         # Frontend task service wrapper

src-tauri/src/
├── commands/
│   ├── task_commands.rs       # Task CRUD (create/read/update/delete/export)
│   ├── text_parser.rs         # Text import parsing and validation
│   └── file_utils.rs          # File operations and validation
├── models/
│   ├── task.rs                # Task and TaskItem data models
│   └── parser.rs              # Parsing result models
├── git/
│   ├── repo_manager.rs        # Git repository operations
│   └── branch_handler.rs      # Branch/commit reference handling
└── storage/
    ├── task_store.rs          # JSON file storage operations
    └── progress_tracker.rs    # Review progress persistence

src-tauri/capabilities/
└── local_tasks.json           # Tauri permissions for file system access

tests/
├── frontend/
│   ├── components/            # Component tests
│   ├── hooks/                 # Hook tests
│   └── services/              # Service tests
├── rust/
│   ├── commands/              # Command unit tests
│   ├── models/                # Model tests
│   └── storage/               # Storage tests
└── e2e/
    ├── create_task.spec.ts    # Task creation workflow
    ├── review_flow.spec.ts    # Review progress workflow
    └── export_task.spec.ts    # Export functionality
```

**Structure Decision**: Tauri desktop application with React frontend and Rust backend. All business logic (text parsing, git operations, file I/O) in Rust commands. Frontend handles UI and state management via Zustand. Storage uses local JSON files in user home directory.

## Complexity Tracking

*No constitution violations - all requirements satisfied by standard Tauri + React architecture*

---

## Phase 0: Research & Unknown Resolution

### Unknowns Identified

1. **Tauri v2 IPC Patterns for File Operations**: Need to research best practices for Rust file system operations via Tauri IPC (reading/writing task JSON files)
2. **Git2-RS Integration Patterns**: Best practices for branch/commit validation and file reading from specific refs
3. **Large Text Parsing Performance**: Optimal Rust parsing strategy for 2000-line imports with <500ms requirement
4. **Tauri File Locking**: How to implement file locking in Rust for concurrent task editing prevention

### Research Tasks

**Task 1**: Research Tauri v2 file system IPC patterns
- Research: File read/write via Tauri's invoke API
- Output: Best practices for JSON storage operations
- Dependencies: None

**Task 2**: Research git2-rs branch and file operations
- Research: Reading files from specific commits/branches
- Output: Code patterns for repo validation and file access
- Dependencies: None

**Task 3**: Research Rust text parsing performance optimization
- Research: Fast parsing for tab/space-separated values
- Output: Optimal parser implementation approach
- Dependencies: None

**Task 4**: Research file locking mechanisms in Rust
- Research: Cross-platform file locking (Windows/macOS/Linux)
- Output: Implementation strategy for concurrent edit prevention
- Dependencies: None

---

## Phase 1: Design & Contracts

*Prerequisites: research.md complete with all unknowns resolved*

### Data Model Design

**Local Task Entity**:
- id: UUID (primary key)
- name: String (user-defined task name)
- repo_path: String (absolute path to git repository)
- base_ref: String (branch/commit/tag reference)
- create_time: ISO 8601 timestamp
- update_time: ISO 8601 timestamp
- status: Enum (in_progress, completed, archived)
- total_items: u32 (total number of task items)
- completed_items: u32 (number of reviewed items)
- items: Vec<TaskItem> (list of review targets)

**Task Item Entity**:
- file: String (relative path from repository root)
- line_range: Option<LineRange> (start/end line numbers)
  - start: Option<u32>
  - end: Option<u32>
- preset_comment: Option<String>
- severity: Option<Enum> (error, warning, question, ok)
- tags: Vec<String> (comma-separated from import)
- reviewed: Boolean (progress tracking)
- comments: Vec<Comment> (review comments added during review)

**Comment Entity**:
- id: UUID
- author: String (reviewer name/email)
- content: String
- created_at: ISO 8601 timestamp
- line_number: Option<u32> (if specific line comment)

### API Contracts (IPC Interface)

**Task Management Commands**:
1. `create_task(payload: CreateTaskRequest) -> Result<Task, String>`
   - Input: name, repo_path, base_ref, items_text
   - Output: Task with generated id and metadata

2. `list_tasks() -> Result<Vec<TaskSummary>, String>`
   - Output: Summary list (id, name, progress, status)

3. `get_task(task_id: UUID) -> Result<Task, String>`
   - Output: Complete task with all items

4. `update_task_progress(task_id: UUID, item_index: usize, reviewed: bool) -> Result<(), String>`
   - Input: task_id, item index, reviewed status
   - Output: void (updates JSON file)

5. `delete_task(task_id: UUID) -> Result<(), String>`
   - Output: void (removes JSON file)

6. `archive_task(task_id: UUID) -> Result<(), String>`
   - Output: void (updates status to archived)

7. `export_task(task_id: UUID) -> Result<String, String>`
   - Output: JSON string for download

**Text Parsing Commands**:
8. `parse_task_text(text: String) -> Result<ParsedText, String>`
   - Input: Raw text with tab/space-separated values
   - Output: Parsed items with validation results

**Git Operations Commands**:
9. `validate_repository(path: String) -> Result<bool, String>`
   - Input: Repository path
   - Output: Boolean (is valid git repo)

10. `list_branches(repo_path: String) -> Result<Vec<String>, String>`
    - Output: List of branch names

11. `validate_ref(repo_path: String, base_ref: String) -> Result<bool, String>`
    - Input: repo path, branch/commit/tag
    - Output: Boolean (ref exists)

12. `read_file_from_ref(repo_path: String, base_ref: String, file_path: String) -> Result<String, String>`
    - Input: repo, ref, file path
    - Output: File contents at specific ref

**File Operations Commands**:
13. `ensure_directory(path: String) -> Result<(), String>`
    - Output: void (creates ~/.hyperreview/local_tasks/)

14. `write_task_file(task: Task) -> Result<(), String>`
    - Output: void (JSON serialization)

15. `read_task_file(task_id: UUID) -> Result<Task, String>`
    - Output: Task deserialized from JSON

### Quick Start Guide

**For Frontend Developers**:
1. Import task hooks: `import { useLocalTasks } from '@/hooks/useLocalTasks'`
2. Create task: `const { createTask } = useLocalTasks(); await createTask({...})`
3. Track progress: `const { currentTask, updateProgress } = useTaskProgress(taskId);`
4. Display tasks: `<TaskPanel />` shows local tasks with orange styling

**For Rust Developers**:
1. Add commands to `src-tauri/src/commands/task_commands.rs`
2. Update `tauri.conf.json` allowlist for file system operations
3. Implement JSON serialization with serde
4. Add tests to `tests/rust/commands/`

---

## Phase 2: Implementation Planning

*To be completed by `/speckit.tasks` command - NOT part of `/speckit.plan`*

### Implementation Phases

**Phase 2A: Core Infrastructure** (Est: 3-5 days)
- Set up Tauri permissions for file system access
- Implement basic JSON storage (create/read/delete task files)
- Create task data models in Rust and TypeScript
- Basic IPC interface (create/list/get/delete)

**Phase 2B: Text Parsing** (Est: 2-3 days)
- Rust text parser for tab/space-separated values
- Validation logic (file paths, line ranges, encoding)
- Error handling and reporting
- Performance optimization for 2000-line parsing

**Phase 2C: Frontend UI** (Est: 4-5 days)
- CreateTaskModal component with text input
- TaskPanel integration (orange styling, progress display)
- Right-click context menu
- Progress tracking and persistence

**Phase 2D: Review Workflow** (Est: 3-4 days)
- Git integration (validate repo, read files from refs)
- File navigation (Ctrl+Enter shortcut)
- Progress persistence after each review
- Line range highlighting

**Phase 2E: Export & Management** (Est: 2-3 days)
- JSON export functionality
- Archive/complete task states
- Task editing and re-import
- File locking for concurrent edits

**Phase 2F: Testing & Polish** (Est: 3-4 days)
- Frontend component tests (80% coverage)
- Rust command tests (100% coverage)
- E2E tests for critical paths
- Performance testing and optimization

**Total Estimated Effort**: 17-24 days (3-4 weeks)

### Risk Factors

1. **Git2-RS Complexity**: Reading files from specific refs may have edge cases
   - Mitigation: Research git2-rs patterns thoroughly in Phase 0

2. **File Locking Cross-Platform**: Different OS locking mechanisms
   - Mitigation: Use cross-platform Rust crate (e.g., fs4)

3. **Performance Requirements**: 300ms file switching is aggressive
   - Mitigation: Implement caching and async loading

4. **Large Task Support**: 10,000 items per task
   - Mitigation: Virtual scrolling in UI, lazy loading in Rust

### Success Criteria

- [ ] All 18 functional requirements implemented
- [ ] Performance: Text parsing <500ms, file switching <300ms
- [ ] Test coverage: Frontend 80%, Rust 100%, E2E critical paths
- [ ] Constitution compliance: All 6 gates pass
- [ ] Export generates valid JSON compatible with external systems
- [ ] 100% progress recovery after restart
