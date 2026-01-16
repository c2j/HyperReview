# Tasks: Merge HyperReview Frontend

**Input**: Design documents from `/specs/007-merge-frontend/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/, quickstart.md

**Tests**: The examples below DO NOT include test tasks. Tests are OPTIONAL - the feature specification does NOT explicitly request tests, so no test tasks are included. Focus is on frontend merge and UI integration.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `src/`, `tests/` at repository root
- **Web app**: `backend/src/`, `frontend/src/`
- **Mobile**: `api/src/`, `ios/src/` or `android/src/`

**Paths shown below**: Desktop application (Tauri + React) - `frontend/src/`, `src-tauri/` (NO MODIFICATIONS)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization, backup, and analysis preparation

- [ ] T001 Backup current frontend directory to frontend/src.backup/
- [ ] T002 [P] Review HyperReview_Frontend component structure in tobemerged/HyperReview_Frontend/components/
- [ ] T003 [P] Review current frontend component structure in frontend/src/components/
- [ ] T004 [P] Review HyperReview_Frontend App.tsx structure in tobemerged/HyperReview_Frontend/App.tsx
- [ ] T005 [P] Review current frontend App.tsx structure in frontend/src/App.tsx
- [ ] T006 [P] List all existing services in frontend/src/services/
- [ ] T007 [P] List all existing IPC integrations in frontend/src/api/client.ts
- [ ] T008 Identify component conflicts between HyperReview_Frontend and current frontend

**Checkpoint**: Setup complete - frontend backed up, both structures analyzed, conflicts identified

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core state management infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T009 [P] Create ApplicationMode type in frontend/src/types/mode.ts (type ApplicationMode = 'local' | 'remote')
- [ ] T010 [P] Update AppState interface in frontend/src/store/appStore.ts to include mode: ApplicationMode and setMode action
- [ ] T011 [P] Create RemoteModeState interface in frontend/src/store/remoteModeStore.ts (gerritChanges, selectedChangeNumber, isGerritConfigured)
- [ ] T012 [P] Create useRemoteModeStore hook in frontend/src/store/remoteModeStore.ts using Zustand
- [ ] T013 [P] Update LocalModeState interface in frontend/src/store/localModeStore.ts if needed for mode consistency
- [ ] T014 [P] Update PanelConfig type in frontend/src/types/panel.ts to support left and right panels with width and visibility
- [ ] T015 [P] Update UserSettings type in frontend/src/types/settings.ts to include language, fontSize, ligatures, vimMode, theme, panels
- [ ] T016 [P] Add mode switching action to appStore.ts that preserves user settings and resets mode-specific state
- [ ] T017 Verify existing IPC client in frontend/src/api/client.ts contains all required commands for local and remote modes
- [ ] T018 Verify all existing services are intact in frontend/src/services/ (gerrit-simple-service.ts, gerrit-instance-service.ts, reviewService.ts)

**Checkpoint**: Foundation ready - state management supports mode switching, IPC and services intact, user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Complete Code Review Workflow (Priority: P1) 🎯 MVP

**Goal**: Perform complete review workflows (opening repositories, viewing diffs, adding comments, submitting reviews) with updated interface while maintaining all existing backend integrations

**Independent Test**: Open a local repository, navigate through diffs, add inline comments, and submit a review. Verify that all Tauri IPC integrations (get_file_diff, add_review_comment, etc.) work correctly.

### Implementation for User Story 1

- [ ] T019 [P] [US1] Merge App.tsx from HyperReview_Frontend with IPC integrations in frontend/src/App.tsx (preserve mode state structure, connect to existing stores)
- [ ] T020 [P] [US1] Copy LocalToolBar.tsx from HyperReview_Frontend to frontend/src/components/LocalToolBar.tsx
- [ ] T021 [P] [US1] Copy LocalTaskTree.tsx from HyperReview_Frontend to frontend/src/components/LocalTaskTree.tsx
- [ ] T022 [P] [US1] Copy LocalRightPanel.tsx from HyperReview_Frontend to frontend/src/components/LocalRightPanel.tsx
- [ ] T023 [P] [US1] Copy DiffView.tsx from HyperReview_Frontend to frontend/src/components/DiffView.tsx (overwrite existing, ensure IPC calls use existing client)
- [ ] T024 [P] [US1] Copy OpenRepoModal.tsx from HyperReview_Frontend to frontend/src/components/OpenRepoModal.tsx (ensure IPC calls use existing open_repo_dialog and load_repo)
- [ ] T025 [P] [US1] Copy NewTaskModal.tsx from HyperReview_Frontend to frontend/src/components/NewTaskModal.tsx (ensure IPC calls use existing services)
- [ ] T026 [P] [US1] Update DiffView.tsx to use existing DiffLine types from frontend/src/api/types/diff.ts
- [ ] T027 [P] [US1] Update DiffView.tsx to invoke get_file_diff via existing IPC client in frontend/src/api/client.ts
- [ ] T028 [P] [US1] Update DiffView.tsx to invoke add_review_comment via existing IPC client for inline comments
- [ ] T029 [P] [US1] Copy ReviewActionModal.tsx from HyperReview_Frontend to frontend/src/components/ReviewActionModal.tsx
- [ ] T030 [P] [US1] Copy SubmitReviewModal.tsx from HyperReview_Frontend to frontend/src/components/SubmitReviewModal.tsx (ensure IPC calls use existing reviewService)
- [ ] T031 [US1] Connect LocalToolBar to appStore.setMode action for mode switching
- [ ] T032 [US1] Connect LocalTaskTree to useLocalModeStore for repository selection and diff context
- [ ] T033 [US1] Connect LocalRightPanel to display review statistics and comments via existing services
- [ ] T034 [US1] Verify local mode components use existing invoke from @tauri-apps/api/core for IPC commands
- [ ] T035 [US1] Test local repository opening via native file dialog (open_repo_dialog IPC command)
- [ ] T036 [US1] Test diff viewing with syntax highlighting (get_file_diff IPC command)
- [ ] T037 [US1] Test inline comment addition and persistence (add_review_comment IPC command)
- [ ] T038 [US1] Test review submission flow (existing reviewService.submitReview method)

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently (local mode code review workflow complete)

---

## Phase 4: User Story 2 - Local/Remote Mode Switching (Priority: P1)

**Goal**: Switch between local repository mode and remote Gerrit mode with proper state management (preserve user settings, reset task and diff state)

**Independent Test**: Switch between local and remote modes, load repositories/changesets in each mode, and verify that user configuration (panel widths, language) is preserved while task and diff state is reset correctly. Verify mode switching completes within 500ms.

### Implementation for User Story 2

- [ ] T039 [P] [US2] Copy RemoteToolBar.tsx from HyperReview_Frontend to frontend/src/components/RemoteToolBar.tsx
- [ ] T040 [P] [US2] Copy RemoteTaskTree.tsx from HyperReview_Frontend to frontend/src/components/RemoteTaskTree.tsx
- [ ] T041 [P] [US2] Copy RemoteRightPanel.tsx from HyperReview_Frontend to frontend/src/components/RemoteRightPanel.tsx
- [ ] T042 [P] [US2] Copy GerritImportModal.tsx from HyperReview_Frontend to frontend/src/components/GerritImportModal.tsx
- [ ] T043 [P] [US2] Copy GerritServerModal.tsx from HyperReview_Frontend to frontend/src/components/GerritServerModal.tsx
- [ ] T044 [P] [US2] Connect RemoteToolBar to appStore.setMode action for switching to remote mode
- [ ] T045 [P] [US2] Connect RemoteTaskTree to useRemoteModeStore for Gerrit change selection
- [ ] T046 [P] [US2] Connect RemoteRightPanel to display Gerrit change details and metadata
- [ ] T047 [P] [US2] Update App.tsx to render LocalToolBar and LocalTaskTree when mode === 'local'
- [ ] T048 [P] [US2] Update App.tsx to render RemoteToolBar and RemoteTaskTree when mode === 'remote'
- [ ] T049 [P] [US2] Implement mode switching logic in App.tsx that preserves userSettings from appStore and resets localModeState/remoteModeState
- [ ] T050 [P] [US2] Add mode toggle button to TitleBar or ToolBar that calls appStore.setMode
- [ ] T051 [P] [US2] Update appStore.setMode action to preserve panelConfig and userSettings while resetting mode-specific state
- [ ] T052 [P] [US2] Connect GerritImportModal to use existing gerrit-simple-service.ts for import operations
- [ ] T053 [P] [US2] Connect GerritServerModal to save GerritServerConfig via existing IPC client
- [ ] T054 [P] [US2] Update LocalModeState to reset diffContext and activeFilePath when switching from local to remote
- [ ] T055 [P] [US2] Update RemoteModeState to reset selectedChangeNumber when switching from remote to local
- [ ] T056 [US2] Test mode switching from local to remote (user settings preserved, local state reset)
- [ ] T057 [US2] Test mode switching from remote to local (user settings preserved, remote state reset)
- [ ] T058 [US2] Measure mode switching performance to ensure <500ms completion (SC-003)

**Checkpoint**: At this point, User Story 2 should be fully functional - mode switching works with proper state management

---

## Phase 5: User Story 3 - Preserve Existing Gerrit Integrations (Priority: P1)

**Goal**: All existing Gerrit functionality (import, server configuration, diff viewing, review submission) continues working seamlessly after frontend merge

**Independent Test**: Configure a Gerrit server, import multiple changes, review them with comments and tags, and submit reviews to Gerrit. Verify all IPC commands (import_gerrit_change, submit_gerrit_review, etc.) work correctly via existing services.

### Implementation for User Story 3

- [ ] T059 [P] [US3] Verify gerrit-simple-service.ts is intact in frontend/src/services/gerrit-simple-service.ts
- [ ] T060 [P] [US3] Verify gerrit-instance-service.ts is intact in frontend/src/services/gerrit-instance-service.ts
- [ ] T061 [P] [US3] Verify reviewService.ts is intact in frontend/src/services/reviewService.ts
- [ ] T062 [P] [US3] Update GerritImportModal to invoke existing gerrit-simple-service.importChange method
- [ ] T063 [P] [US3] Update GerritImportModal to display change list from gerrit-simple-service.getChanges
- [ ] T064 [P] [US3] Update GerritServerModal to save config via existing IPC client (save_user_settings command)
- [ ] T065 [P] [US3] Update RemoteTaskTree to load changes from gerrit-instance-service.getChanges
- [ ] T066 [P] [US3] Update RemoteTaskTree to select change and update useRemoteModeStore.selectedChangeNumber
- [ ] T067 [P] [US3] Update DiffView to handle Gerrit change diffs via existing get_file_diff IPC command
- [ ] T068 [P] [US3] Update SubmitReviewModal to invoke existing reviewService.submitGerritReview method
- [ ] T069 [P] [US3] Update CommentCreator/CommentThread components (if used in remote mode) to work with Gerrit comments
- [ ] T070 [P] [US3] Verify all Gerrit-related IPC commands are available in frontend/src/api/client.ts (import_gerrit_change, get_gerrit_changes, submit_gerrit_review)
- [ ] T071 [P] [US3] Update GerritChange type in frontend/src/types/gerrit.ts to match backend serialization
- [ ] T072 [P] [US3] Update GerritFile type in frontend/src/types/gerrit.ts to match backend serialization
- [ ] T073 [P] [US3] Update GerritServerConfig type in frontend/src/types/gerrit.ts for server configuration
- [ ] T074 [P] [US3] Update RemoteRightPanel to display Gerrit change metadata from useRemoteModeStore
- [ ] T075 [P] [US3] Verify error handling for Gerrit operations matches current frontend patterns (unified toast errors)
- [ ] T076 [US3] Test Gerrit server configuration and credential storage (via existing IPC)
- [ ] T077 [US3] Test Gerrit change import functionality (via existing services)
- [ ] T078 [US3] Test Gerrit diff viewing (via existing IPC)
- [ ] T079 [US3] Test inline comment addition for Gerrit changes (via existing reviewService)
- [ ] T080 [US3] Test review submission to Gerrit server (via existing reviewService)
- [ ] T081 [US3] Test navigation between multiple Gerrit changes without data leakage or corruption

**Checkpoint**: At this point, User Story 3 should be fully functional - all Gerrit integrations preserved and working

---

## Phase 6: User Story 4 - Interface Consistency and Layout (Priority: P2)

**Goal**: Updated interface follows design and layout from HyperReview_Frontend, including panel resizing, modal dialogs, and component organization

**Independent Test**: Interact with resizable panels, open various modals, and verify layout matches HyperReview_Frontend patterns. Verify panel resizing completes smoothly without visual artifacts or layout breakage.

### Implementation for User Story 4

- [ ] T082 [P] [US4] Copy TitleBar.tsx from HyperReview_Frontend to frontend/src/components/TitleBar.tsx (if needed)
- [ ] T083 [P] [US4] Copy StatusBar.tsx from HyperReview_Frontend to frontend/src/components/StatusBar.tsx
- [ ] T084 [P] [US4] Copy Modal.tsx from HyperReview_Frontend to frontend/src/components/Modal.tsx (update if current exists)
- [ ] T085 [P] [US4] Copy CommandPalette.tsx from HyperReview_Frontend to frontend/src/components/CommandPalette.tsx (ensure IPC commands use existing client)
- [ ] T086 [P] [US4] Copy SettingsModal.tsx from HyperReview_Frontend to frontend/src/components/SettingsModal.tsx (ensure settings persistence uses existing IPC)
- [ ] T087 [P] [US4] Copy TagManagerModal.tsx from HyperReview_Frontend to frontend/src/components/TagManagerModal.tsx
- [ ] T088 [P] [US4] Copy BranchCompareModal.tsx from HyperReview_Frontend to frontend/src/components/BranchCompareModal.tsx
- [ ] T089 [P] [US4] Copy SyncStatusModal.tsx from HyperReview_Frontend to frontend/src/components/SyncStatusModal.tsx
- [ ] T090 [P] [US4] Copy TourGuide.tsx from HyperReview_Frontend to frontend/src/components/TourGuide.tsx
- [ ] T091 [P] [US4] Copy WelcomeView.tsx from HyperReview_Frontend to frontend/src/components/WelcomeView.tsx
- [ ] T092 [P] [US4] Implement panel resizing logic in App.tsx for left and right panels (use appStore.updatePanelConfig)
- [ ] T093 [P] [US4] Add panel drag handles to left and right panels for interactive resizing
- [ ] T094 [P] [US4] Implement panel visibility toggle (show/hide) for left and right panels
- [ ] T095 [P] [US4] Update App.tsx layout to handle panel width and visibility changes smoothly
- [ ] T096 [P] [US4] Add keyboard shortcuts for panel toggling (Ctrl+Shift+Left/Right or similar)
- [ ] T097 [P] [US4] Ensure panel resizing respects minimum (200px) and maximum (50% viewport) constraints per data-model.md
- [ ] T098 [P] [US4] Test panel resizing in both local and remote modes (width preserved across mode switch)
- [ ] T099 [P] [US4] Test panel visibility toggling in both local and remote modes
- [ ] T100 [P] [US4] Test that main content area adjusts appropriately when panels are resized or toggled
- [ ] T101 [P] [US4] Test all modals (OpenRepo, NewTask, Settings, GerritServer, etc.) display correctly with proper styling
- [ ] T102 [P] [US4] Test modal open/close animations and backdrop behavior
- [ ] T103 [P] [US4] Verify UI states (loading, error, success) display correctly in StatusBar
- [ ] T104 [P] [US4] Test CommandPalette functionality with existing IPC commands

**Checkpoint**: At this point, User Story 4 should be fully functional - interface consistent with HyperReview_Frontend design

---

## Phase 7: User Story 5 - Documentation and API Consistency (Priority: P3)

**Goal**: Merged codebase maintains consistent API client structure and includes merged and updated documentation from HyperReview_Frontend

**Independent Test**: Review API client files, type definitions, and all documentation files (IPC.md, OpenAPI.md, etc.) to ensure they align with merged implementation. Verify all services work correctly with new components and that established patterns are followed.

### Implementation for User Story 5

- [ ] T105 [P] [US5] Copy IPC.md from HyperReview_Frontend to frontend/docs/IPC.md and update to reflect merged implementation
- [ ] T106 [P] [US5] Copy OpenAPI.md from HyperReview_Frontend to frontend/docs/OpenAPI.md and update to reflect merged implementation
- [ ] T107 [P] [US5] Copy design-backend.md from HyperReview_Frontend to frontend/docs/design-backend.md and update to reflect merged implementation
- [ ] T108 [P] [US5] Update IPC.md to document preserved IPC commands and reference existing client in frontend/src/api/client.ts
- [ ] T109 [P] [US5] Update IPC.md to include mode switching documentation (how commands differ between local and remote modes)
- [ ] T110 [P] [US5] Verify all type definitions in frontend/src/api/types/ match merged implementation
- [ ] T111 [P] [US5] Update frontend/docs/IPC.md with examples of using IPC commands with merged components
- [ ] T112 [P] [US5] Create migration notes in frontend/docs/MIGRATION.md explaining merge changes and migration path for developers
- [ ] T113 [P] [US5] Update package.json in frontend/package.json with any new dependencies from HyperReview_Frontend (if needed)
- [ ] T114 [P] [US5] Review and update tsconfig.json in frontend/tsconfig.json if needed for new components
- [ ] T115 [P] [US5] Verify all services in frontend/src/services/ are documented in IPC.md or design-backend.md
- [ ] T116 [P] [US5] Verify all unique components preserved from current frontend (CredentialManager, ExternalSubmissionDialog, etc.) are documented
- [ ] T117 [P] [US5] Test that existing services (gerrit-simple-service, reviewService, etc.) work correctly with new components
- [ ] T118 [P] [US5] Review IPC.md to ensure all IPC commands are documented with accurate signatures
- [ ] T119 [P] [US5] Verify documentation mentions mode switching behavior and state preservation

**Checkpoint**: At this point, User Story 5 should be fully functional - documentation complete and consistent with merged implementation

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories, final integration, and cleanup

- [ ] T120 [P] Preserve unique components from current frontend in frontend/src/components/ (CredentialManager.tsx, ExternalSubmissionDialog.tsx, CommentCreator.tsx, etc.)
- [ ] T121 [P] Update CredentialManager.tsx to work with merged App.tsx (if integration needed)
- [ ] T122 [P] Update ExternalSubmissionDialog.tsx to work with merged App.tsx (if integration needed)
- [ ] T123 [P] Update CommentCreator.tsx to work with merged DiffView (if integration needed)
- [ ] T124 [P] Update CommentList.tsx to work with merged panels (if integration needed)
- [ ] T125 [P] Update CommentThread.tsx to work with merged components (if integration needed)
- [ ] T126 [P] Merge i18n translations from HyperReview_Frontend in frontend/src/i18n.tsx (preserve all existing translations)
- [ ] T127 [P] Update i18n.tsx to include translations for new mode switching UI elements
- [ ] T128 [P] Verify all existing hooks in frontend/src/hooks/ are intact (useIPC.ts, etc.)
- [ ] T129 [P] Verify all existing context providers in frontend/src/context/ are intact
- [ ] T130 [P] Update App.tsx imports to include all merged components
- [ ] T131 [P] Remove any unused imports from App.tsx after merge
- [ ] T132 [P] Remove duplicate components in frontend/src/components/ (keep HyperReview_Frontend versions, preserve unique current components)
- [ ] T133 [P] Add TODO comments to any features that could not be reconciled (graceful degradation strategy from FR-016)
- [ ] T134 [P] Run TypeScript compiler (tsc) and fix any type errors in merged frontend
- [ ] T135 [P] Run ESLint and fix any linting issues in merged frontend
- [ ] T136 [P] Run Prettier to format all merged code
- [ ] T137 [P] Remove backend backup (frontend/src.backup/) after merge validation
- [ ] T138 [P] Update README.md in frontend/README.md with merge information if applicable
- [ ] T139 [P] Verify application builds successfully with `npm run build`
- [ ] T140 [P] Verify application runs with `npm run tauri dev` and test basic functionality
- [ ] T141 [P] Run `npm run lint` to ensure all linting passes
- [ ] T142 [P] Run `npm run typecheck` if available to ensure TypeScript compilation
- [ ] T143 [P] Run existing tests with `npm test` (if tests exist for preserved services)
- [ ] T144 [P] Verify test coverage for core business logic and services meets 80% target (SC-008)
- [ ] T145 [P] Update AGENTS.md with any new technology or patterns if needed (already done during planning)
- [ ] T146 [P] Finalize commit message following conventional commits format (feat(merge): merge HyperReview_Frontend UI components)

**Checkpoint**: At this point, all work is complete - frontend merged, polished, tested, and ready for PR

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-7)**: All depend on Foundational phase completion
  - User Story 1 (P1): Can start after Foundational - No dependencies on other stories
  - User Story 2 (P1): Can start after Foundational - Integrates with US1 but independently testable
  - User Story 3 (P1): Can start after Foundational - Depends on US2 for remote mode UI
  - User Story 4 (P2): Can start after Foundational - Integrates with US1-US3
  - User Story 5 (P3): Can start after Foundational - Depends on US1-US4 completion
- **Polish (Phase 8)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P1)**: Can start after Foundational (Phase 2) - Integrates with US1 but independently testable
- **User Story 3 (P1)**: Can start after Foundational (Phase 2) - Depends on US2 for remote mode UI, but independently testable
- **User Story 4 (P2)**: Can start after Foundational (Phase 2) - May integrate with US1-US3 but independently testable
- **User Story 5 (P3)**: Can start after Foundational (Phase 2) - Depends on US1-US4 being complete, but documentation can be done in parallel

### Within Each User Story

- Core implementation before integration
- Integration tasks after individual components are merged
- Testing after implementation (manual verification per story)
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel (T001-T008)
- All Foundational tasks marked [P] can run in parallel within Phase 2 (T009-T018)
- Once Foundational phase completes, User Stories 1-3 (all P1) can start in parallel (if team capacity allows)
- User Story 4 (P2) can start in parallel with User Story 1-3 once Foundational is complete
- User Story 5 (P3) can start in parallel once US1-US4 are merged (documentation can begin early)
- All [P] tasks within a story can run in parallel (different components)
- Different user stories can be worked on in parallel by different team members

---

## Parallel Examples

### Parallel Example: Phase 2 (Foundational)

```bash
# Launch all state management tasks together:
Task: "Create ApplicationMode type in frontend/src/types/mode.ts"
Task: "Update AppState interface in frontend/src/store/appStore.ts"
Task: "Create RemoteModeState interface in frontend/src/store/remoteModeStore.ts"
Task: "Create useRemoteModeStore hook in frontend/src/store/remoteModeStore.ts"
Task: "Update LocalModeState interface in frontend/src/store/localModeStore.ts"
Task: "Update PanelConfig type in frontend/src/types/panel.ts"
Task: "Update UserSettings type in frontend/src/types/settings.ts"
```

### Parallel Example: Phase 3 (User Story 1)

```bash
# Launch all component merge tasks together:
Task: "Merge App.tsx from HyperReview_Frontend with IPC integrations"
Task: "Copy LocalToolBar.tsx from HyperReview_Frontend"
Task: "Copy LocalTaskTree.tsx from HyperReview_Frontend"
Task: "Copy LocalRightPanel.tsx from HyperReview_Frontend"
Task: "Copy DiffView.tsx from HyperReview_Frontend"
Task: "Copy OpenRepoModal.tsx from HyperReview_Frontend"
Task: "Copy NewTaskModal.tsx from HyperReview_Frontend"
```

### Parallel Example: Phase 4 (User Story 2)

```bash
# Launch all remote mode component merge tasks together:
Task: "Copy RemoteToolBar.tsx from HyperReview_Frontend"
Task: "Copy RemoteTaskTree.tsx from HyperReview_Frontend"
Task: "Copy RemoteRightPanel.tsx from HyperReview_Frontend"
Task: "Copy GerritImportModal.tsx from HyperReview_Frontend"
Task: "Copy GerritServerModal.tsx from HyperReview_Frontend"
```

### Parallel Example: Phase 6 (User Story 4)

```bash
# Launch all modal and UI component merge tasks together:
Task: "Copy TitleBar.tsx from HyperReview_Frontend"
Task: "Copy StatusBar.tsx from HyperReview_Frontend"
Task: "Copy Modal.tsx from HyperReview_Frontend"
Task: "Copy CommandPalette.tsx from HyperReview_Frontend"
Task: "Copy SettingsModal.tsx from HyperReview_Frontend"
Task: "Copy TagManagerModal.tsx from HyperReview_Frontend"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001-T008)
2. Complete Phase 2: Foundational (T009-T018) - CRITICAL - blocks all stories
3. Complete Phase 3: User Story 1 (T019-T038)
4. **STOP and VALIDATE**: Test User Story 1 independently
   - Open local repository via native dialog
   - View diff with syntax highlighting
   - Add inline comments
   - Submit review
   - Verify all IPC integrations work
5. Deploy/demo if ready
6. Continue to User Stories 2-3 for enhanced functionality

### Incremental Delivery

1. Complete Setup + Foundational (Phase 1-2) → Foundation ready
2. Add User Story 1 (Phase 3) → Test independently → Local mode code review workflow (MVP!)
3. Add User Story 2 (Phase 4) → Test independently → Mode switching capability
4. Add User Story 3 (Phase 5) → Test independently → Gerrit integrations preserved
5. Add User Story 4 (Phase 6) → Test independently → Interface consistency and layout
6. Add User Story 5 (Phase 7) → Test independently → Documentation complete
7. Polish (Phase 8) → Final integration and cleanup
8. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

**Initial Phase** (Setup + Foundational - 1-2 days):
- Team completes Setup (T001-T008) and Foundational (T009-T018) together
- All developers aligned on state management approach

**Phase 3** (User Story 1 - 2-3 days):
- Developer A: App.tsx merge, DiffView integration
- Developer B: Local mode components (ToolBar, TaskTree, RightPanel)
- Developer C: Modal components (OpenRepo, NewTask, SubmitReview)
- Integration: Connect all components to existing stores and IPC

**Phase 4** (User Story 2 - 2-3 days):
- Developer A: Mode switching logic in App.tsx
- Developer B: Remote mode components (ToolBar, TaskTree, RightPanel)
- Developer C: Gerrit modals (Import, Server)
- Integration: Connect remote mode to stores and services

**Phase 5** (User Story 3 - 1-2 days):
- Developer A: Verify all Gerrit services intact
- Developer B: Update Gerrit components to use existing services
- Developer C: Test all Gerrit operations (import, diff, review)

**Phase 6** (User Story 4 - 1-2 days):
- Developer A: Panel resizing and visibility logic
- Developer B: Modal components (Settings, TagManager, etc.)
- Developer C: UI polish and testing

**Phase 7** (User Story 5 - 1 day):
- Developer A: Documentation merge and updates
- Developer B: Type definition verification
- Developer C: Documentation testing

**Phase 8** (Polish - 1 day):
- Team: Unique component preservation, cleanup, final testing
- Team: Linting, formatting, build verification
- Team: Commit and PR preparation

---

## Notes

- **Testing**: The feature specification does NOT explicitly request tests, so no test tasks are included. Focus is on frontend merge and UI integration. Existing tests for preserved services should be verified.
- **[P] tasks**: Different files, no dependencies - can run in parallel
- **[Story] label**: Maps task to specific user story for traceability and independent testability
- **Backend**: NO modifications to src-tauri - all work is frontend only
- **Preservation**: All existing IPC integrations and services MUST be preserved (FR-002, FR-004)
- **Mode Switching**: Must preserve user settings (panels, language) and reset task/diff state (clarification from spec)
- **Component Conflicts**: Use HyperReview_Frontend version as base, port IPC integrations from current frontend (clarification from spec)
- **Graceful Degradation**: Add TODO comments for features that cannot be reconciled (FR-016)
- **Independent Testability**: Each user story should be independently completable and testable
- **Verification**: Stop at any checkpoint to validate story independently before proceeding
- **Avoid**: Vague tasks, same file conflicts without clear resolution, cross-story dependencies that break independence
- **Commit**: Use conventional commits format: feat(merge): [description]
- **Backup**: Frontend backed up before merge - can restore if issues arise
- **MVP**: User Story 1 alone provides value (local code review workflow)
- **Priority**: P1 stories (1-3) provide most value - complete first for early delivery
