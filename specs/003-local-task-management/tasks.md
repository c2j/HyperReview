# Tasks: Local Task Management

**Input**: Design documents from `/specs/003-local-task-management/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure for Tauri desktop application

- [ ] T001 Create source directory structure per implementation plan (src/, src-tauri/src/, tests/)
- [ ] T002 [P] Add Tauri v2 dependencies to Cargo.toml (serde, serde_json, uuid, fs4, thiserror, git2, tokio)
- [ ] T003 [P] Add frontend dependencies to package.json (Zustand, @tauri-apps/api)
- [ ] T004 [P] Create Tauri capabilities file at src-tauri/capabilities/local_tasks.json
- [ ] T005 [P] Configure TypeScript types for task entities in src/types/
- [ ] T006 [P] Setup ESLint and Prettier configuration for React/TypeScript

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T007 Create Rust data models in src-tauri/src/models/task.rs (LocalTask, TaskItem, Comment, LineRange, enums)
- [ ] T008 [P] Create TypeScript type definitions in src/types/task.ts (matching Rust models)
- [ ] T009 [P] Implement JSON storage module in src-tauri/src/storage/task_store.rs (create/read/write/delete task files)
- [ ] T010 [P] Setup git2-rs integration module in src-tauri/src/git/repo_manager.rs (validate repo, list branches, read files)
- [ ] T011 Register Tauri commands in src-tauri/src/lib.rs (command handlers registration)
- [ ] T012 Update tauri.conf.json allowlist for file system and git operations
- [ ] T013 Create file locking utility in src-tauri/src/storage/file_lock.rs using fs4 crate
- [ ] T014 [P] Setup logging infrastructure for task operations

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Create and Review Local Task from Text Import (Priority: P1) 🎯 MVP

**Goal**: Enable Tech Leads to create local review tasks by importing text-based file lists, allowing offline review without waiting for PRs

**Independent Test**: Import a text file with 10-20 file entries, create the task, and review at least 3 files in sequence

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T015 [P] [US1] Unit test for text parser in src-tauri/src/commands/text_parser_test.rs
- [ ] T016 [P] [US1] Integration test for create_task IPC in tests/rust/commands/test_create_task.rs
- [ ] T017 [P] [US1] React component test for CreateTaskModal in tests/frontend/components/CreateTaskModal.test.tsx

### Implementation for User Story 1

- [ ] T018 [P] [US1] Implement text parser in src-tauri/src/commands/text_parser.rs (parse tab/space-separated values, validate format)
- [ ] T019 [US1] Implement parse_task_text command handler in src-tauri/src/commands/task_commands.rs
- [ ] T020 [US1] Implement create_task command handler in src-tauri/src/commands/task_commands.rs (validate repo/ref, parse text, save task)
- [ ] T021 [P] [US1] Create CreateTaskModal component in src/components/CreateTaskModal.tsx (form, text input, validation, preview)
- [ ] T022 [P] [US1] Create TaskPanel component in src/components/TaskPanel.tsx (display local tasks with orange styling, progress)
- [ ] T023 [US1] Create useLocalTasks hook in src/hooks/useLocalTasks.ts (CRUD operations via IPC)
- [ ] T024 [US1] Create taskStore Zustand store in src/store/taskStore.ts (state management)
- [ ] T025 [P] [US1] Implement list_tasks command handler in src-tauri/src/commands/task_commands.rs
- [ ] T026 [P] [US1] Implement get_task command handler in src-tauri/src/commands/task_commands.rs
- [ ] T027 [US1] Implement update_task_progress command in src-tauri/src/commands/task_commands.rs (mark items reviewed)
- [ ] T028 [P] [US1] Create TaskItem component in src/components/TaskItem.tsx (individual task display with progress)
- [ ] T029 [US1] Add keyboard shortcut handler (Ctrl+Enter) for next item navigation
- [ ] T030 [US1] Integrate with existing PR/MR review workflow (load files from git ref)

**Checkpoint**: At this point, User Story 1 should be fully functional - users can create tasks from text and review files

---

## Phase 4: User Story 2 - Manage and Track Task Progress (Priority: P2)

**Goal**: Enable reviewers to manage multiple local tasks, track progress, and organize by status

**Independent Test**: Create 3 tasks with different statuses, use right-click menus to edit/manage, verify progress tracking

### Tests for User Story 2

- [ ] T031 [P] [US2] Unit test for task status transitions in tests/rust/models/test_task_status.rs
- [ ] T032 [P] [US2] Integration test for archive/delete operations in tests/rust/commands/test_task_management.rs
- [ ] T033 [P] [US2] React component test for TaskContextMenu in tests/frontend/components/TaskContextMenu.test.tsx

### Implementation for User Story 2

- [ ] T034 [P] [US2] Implement delete_task command handler in src-tauri/src/commands/task_commands.rs
- [ ] T035 [P] [US2] Implement archive_task command handler in src-tauri/src/commands/task_commands.rs
- [ ] T036 [US2] Implement re-import text feature (update task items while preserving metadata)
- [ ] T037 [P] [US2] Create TaskContextMenu component in src/components/TaskContextMenu.tsx (edit, export, complete, delete, archive)
- [ ] T038 [P] [US2] Create TaskProgress component in src/components/TaskProgress.tsx (progress indicator with completion percentage)
- [ ] T039 [US2] Implement progress persistence (save after each item review, recover on restart)
- [ ] T040 [P] [US2] Add filtering by status in task list (in_progress, completed, archived)
- [ ] T041 [US2] Implement task editing functionality (modify name, description, items)

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Export and Share Review Results (Priority: P3)

**Goal**: Enable technical leaders to export local task review results in standard format for sharing

**Independent Test**: Complete a task and generate an exportable JSON report with file paths, line ranges, comments

### Tests for User Story 3

- [ ] T042 [P] [US3] Unit test for export functionality in tests/rust/commands/test_export_task.rs
- [ ] T043 [P] [US3] Integration test for export JSON schema in tests/integration/test_export_schema.py

### Implementation for User Story 3

- [ ] T044 [P] [US3] Implement export_task command handler in src-tauri/src/commands/task_commands.rs (generate JSON with all data)
- [ ] T045 [P] [US3] Create export dialog component in src/components/ExportDialog.tsx (select format, download)
- [ ] T046 [US3] Add export option to TaskContextMenu (right-click → export)
- [ ] T047 [US3] Implement JSON schema validation for exported data (file paths, line ranges, comments, severity)
- [ ] T048 [P] [US3] Add batch export capability (multiple tasks at once)
- [ ] T049 [US3] Integrate with external review systems (Gerrit, CodeArts, custom APIs)

**Checkpoint**: All user stories should now be independently functional

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T050 [P] Frontend component tests with 80% coverage in tests/frontend/
- [ ] T051 [P] Rust command tests with 100% coverage in tests/rust/commands/
- [ ] T052 [P] E2E tests for critical workflows in tests/e2e/ (create_task, review_flow, export_task)
- [ ] T053 [P] Performance optimization: verify text parsing <500ms for 2000 lines
- [ ] T054 [P] Performance optimization: verify file switching <300ms
- [ ] T055 [P] Security hardening: validate all file paths, prevent directory traversal
- [ ] T056 [P] File locking validation across Windows/macOS/Linux
- [ ] T057 [P] Add error boundaries and user-friendly error messages in React components
- [ ] T058 [P] Update README.md with local task feature documentation
- [ ] T059 [P] Add loading states and progress indicators for async operations
- [ ] T060 [P] Virtual scrolling for large task lists (>100 items)
- [ ] T061 [P] Bundle size optimization (target <15MB)
- [ ] T062 [P] Run clippy and fix all warnings
- [ ] T063 [P] Final integration test: complete end-to-end review workflow

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Builds on US1 but independently testable
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - Can run parallel to US1/US2

### Within Each User Story

- Tests (if included) MUST be written and FAIL before implementation
- Models before services
- Services before commands
- Commands before UI components
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel (T001-T006)
- All Foundational tasks marked [P] can run in parallel (T008-T010, T014)
- Once Foundational is done:
  - Developer A: User Story 1
  - Developer B: User Story 2
  - Developer C: User Story 3
- All tests for a user story marked [P] can run in parallel
- Models and components within a story marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
Task: "Unit test for text parser in src-tauri/src/commands/text_parser_test.rs"
Task: "Integration test for create_task IPC in tests/rust/commands/test_create_task.rs"
Task: "React component test for CreateTaskModal in tests/frontend/components/CreateTaskModal.test.tsx"

# Launch all parallel implementations for User Story 1:
Task: "Implement text parser in src-tauri/src/commands/text_parser.rs"
Task: "Create TypeScript type definitions in src/types/task.ts"
Task: "Create CreateTaskModal component in src/components/CreateTaskModal.tsx"
Task: "Create TaskPanel component in src/components/TaskPanel.tsx"
Task: "Implement list_tasks command handler"
Task: "Implement get_task command handler"
Task: "Create TaskItem component"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001-T006)
2. Complete Phase 2: Foundational (T007-T014) - CRITICAL - blocks all stories
3. Complete Phase 3: User Story 1 (T015-T030)
4. **STOP and VALIDATE**: Test User Story 1 independently
   - Create a task from text
   - Review at least 3 files
   - Verify progress tracking
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Polish phase → Final release

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup (T001-T006) together
2. Team completes Foundational (T007-T014) together
3. Once Foundational is done:
   - Developer A: User Story 1 (T015-T030)
   - Developer B: User Story 2 (T031-T041)
   - Developer C: User Story 3 (T042-T049)
4. Stories complete and integrate independently
5. Team reconverges for Polish phase (T050-T063)

---

## Success Criteria Validation

- [ ] All 18 functional requirements from spec.md implemented
- [ ] Performance: Text parsing <500ms for 2000 lines (research shows 6ms achievable)
- [ ] Performance: File switching <300ms
- [ ] Test coverage: Frontend 80%, Rust 100%, E2E critical paths
- [ ] Constitution compliance: All 6 gates pass
- [ ] Export generates valid JSON compatible with external systems
- [ ] 100% progress recovery after restart
- [ ] Support for up to 10,000 items per task
- [ ] Cross-platform compatibility (Windows 10+, macOS 11+, Linux Ubuntu 20.04+)

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence

**Total Task Count**: 63 tasks
- Setup: 6 tasks
- Foundational: 8 tasks
- User Story 1 (P1): 16 tasks
- User Story 2 (P2): 11 tasks
- User Story 3 (P3): 8 tasks
- Polish: 14 tasks
