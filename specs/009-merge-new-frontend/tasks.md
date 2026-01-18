# Tasks: Merge New Frontend with Local/Remote Review Mode Separation

**Feature**: 009-merge-new-frontend
**Branch**: `009-merge-new-frontend`
**Date**: 2025-01-18
**Input**: Design documents from `/specs/009-merge-new-frontend/`

---

## Summary

- **Total Tasks**: 32
- **Estimated Effort**: 3-4 weeks
- **Parallel Opportunities**: T001-T003 (create directories), T004-T006 (move components) can run in parallel
- **Suggested MVP Scope**: Phase 1 + Phase 2 + User Story 1 (Local mode) = minimum viable product that delivers value
- **Independent Test Criteria**:
  - Each user story has complete, independently testable implementation
  - Mode switching works without requiring repository connections
  - Settings changes take effect and persist without requiring app restart

---

## Phase 1: Setup - Core Infrastructure

**Purpose**: Project initialization and basic structure for mode-separated frontend

### Goal
Establish the foundational components and state management needed for Local/Remote mode separation.

### Tasks

- [ ] T001 Create component subdirectories for mode-specific organization
- [ ] T002 Create IPC abstraction layer directory and client
- [ ] T003 Create contexts directory and React contexts
- [ ] T004 Add CSS variables for editor settings to index.css
- [ ] T005 Create hooks directory and custom React hooks
- [ ] T006 Update package.json with Tauri Store plugin dependency
- [ ] T007 Create services directory and API type definitions
- [ ] T008 Create settings context with Zustand persistence
- [ ] T009 Create review mode context with mode state management

### Success Criteria
- All core directories created with proper structure
- IPC client abstraction layer implemented with error handling
- CSS variables defined for global editor settings
- Tauri Store plugin installed and configured
- All TypeScript interfaces defined and exported
- Context providers created with proper TypeScript typing

---

## Phase 2: User Story 1 - Switch Between Local and Remote Review Modes (Priority: P1)

**Goal**: Users can toggle between Local review mode (for reviewing local git repository changes) and Remote review mode (for reviewing changes from remote platforms like Gerrit, CodeArts, or GitLab) within application. The interface adapts to show appropriate tools, task trees, and panels for each mode.

### Tasks

#### Setup Phase

- [ ] T010 [US1] Create SettingsContext with EditorSettings interface and Tauri Store integration
  File: `frontend/src/contexts/SettingsContext.tsx`

- [ ] T011 [US1] Create ReviewModeContext with mode state and mode persistence
  File: `frontend/src/contexts/ReviewModeContext.tsx`

- [ ] T012 [US1] Create useAppStore with Zustand store for global app state
  File: `frontend/src/store/useAppStore.ts`

#### Implementation Phase

- [ ] T013 [US1] Implement ModeToggle component with segmented control (Local/Remote)
  File: `frontend/src/components/ModeToggle.tsx`

- [ ] T014 [US1] Refactor App.tsx to wrap mode-specific components with conditional rendering
  File: `frontend/src/App.tsx`

- [ ] T015 [US1] Add mode state provider wrapper in App.tsx
  File: `frontend/src/App.tsx`

#### Integration Phase

- [ ] T016 [US1] Update TitleBar to display current mode indicator
  File: `frontend/src/components/TitleBar.tsx`

- [ ] T017 [US1] Integrate mode toggle into application header/toolbar
  File: `frontend/src/App.tsx`

#### Testing Phase

- [ ] T018 [US1] Test mode switching from Local to Remote
- [ ] T019 [US1] Test mode switching from Remote to Local
- [ ] T020 [US1] Verify mode persists across application restarts
- [ ] T021 [US1] Test that existing Local review components still function in Local mode
- [ ] T022 [US1] Test keyboard shortcuts for mode switching (if implemented)

### Success Criteria
- Mode toggle button successfully switches between Local and Remote modes within 1 second
- UI updates correctly to show mode-specific components for each mode
- Mode selection persists to localStorage/Tauri Store and is restored on app restart
- Existing Local review functionality works uninterrupted in Local mode
- No crashes or errors when switching modes
- User can switch modes multiple times without issues

---

## Phase 3: User Story 2 - Review Local Repository Changes (Priority: P1)

**Goal**: Users can open a local git repository and review code changes using the dedicated Local review interface. The interface provides task management, diff viewing, comment creation, and analysis tools optimized for local code review workflows.

### Tasks

#### Setup Phase

- [ ] T023 [US1] Move LocalToolBar.tsx from new frontend to components/toolbars/
  Source: `tobemerged/HyperReview_Frontend/components/LocalToolBar.tsx`
  Destination: `frontend/src/components/toolbars/LocalToolBar.tsx`

- [ ] T024 [US1] Move LocalTaskTree.tsx from new frontend to components/task-trees/
  Source: `tobemerged/HyperReview_Frontend/components/LocalTaskTree.tsx`
  Destination: `frontend/src/components/task-trees/LocalTaskTree.tsx`

- [ ] T025 [US1] Move LocalRightPanel.tsx from new frontend to components/panels/
  Source: `tobemerged/HyperReview_Frontend/components/LocalRightPanel.tsx`
  Destination: `frontend/src/components/panels/LocalRightPanel.tsx`

- [ ] T026 [US1] Move WelcomeView.tsx from new frontend to components/
  Source: `tobemerged/HyperReview_Frontend/components/WelcomeView.tsx`
  Destination: `frontend/src/components/WelcomeView.tsx`

#### Implementation Phase

- [ ] T027 [US1] Refactor existing ToolBar.tsx to use mode context and conditionally render LocalToolBar or RemoteToolBar
  File: `frontend/src/components/ToolBar.tsx`

- [ ] T028 [US1] Refactor existing TaskTree.tsx to use mode context and conditionally render LocalTaskTree or RemoteTaskTree
  File: `frontend/src/components/TaskTree.tsx`

- [ ] T029 [US1] Refactor existing RightPanel.tsx to use mode context and conditionally render LocalRightPanel or RemoteRightPanel
  File: `frontend/src/components/RightPanel.tsx`

- [ ] T030 [US1] Update App.tsx to integrate WelcomeView when no repository is loaded
  File: `frontend/src/App.tsx`

- [ ] T031 [US1] Create useLocalReviewStore with local repository and task management state
  File: `frontend/src/store/useLocalReviewStore.ts`

#### Integration Phase

- [ ] T032 [US1] Integrate LocalToolBar with existing OpenRepoModal
  File: `frontend/src/App.tsx`

- [ ] T033 [US1] Integrate LocalTaskTree with DiffView component
  File: `frontend/src/App.tsx`

- [ ] T034 [US1] Integrate LocalRightPanel with comment management
  File: `frontend/src/App.tsx`

- [ ] T035 [US1] Connect mode context to ensure Local mode shows Local-specific components
  File: `frontend/src/App.tsx`

#### Testing Phase

- [ ] T036 [US1] Test opening a local git repository via OpenRepoModal
- [ ] T037 [US1] Test viewing diffs with Local mode active
- [ ] T038 [US1] Test creating comments on diffs in Local mode
- [ ] T039 [US1] Test viewing branches and switching between branches
- [ ] T040 [US1] Test task tree functionality in Local mode
- [ ] T041 [US1] Test that all existing Local review features still work (blame, checklist, heatmap)
- [ ] T042 [US1] Test error handling when repository path is invalid or not a git repository

### Success Criteria
- Users can successfully open a local git repository in Local mode
- Local-specific toolbar shows correct actions for Local review
- Local task tree displays and allows navigation through files and tasks
- Diff viewer shows file changes with proper syntax highlighting
- Comments can be created, edited, and deleted on specific lines
- All analysis tools (blame, checklist, heatmap) work using existing Tauri backend commands
- Existing test suite (if any) continues to pass
- No regressions compared to current Local review functionality

---

## Phase 4: User Story 3 - Access Remote Review Interface (Priority: P2)

**Goal**: Users can access the Remote review interface to prepare for reviewing changes from remote platforms. The interface provides placeholders for remote-specific workflows and integrates with existing Gerrit configuration where backend support exists.

### Tasks

#### Setup Phase

- [ ] T043 [US3] Move RemoteToolBar.tsx from new frontend to components/toolbars/
  Source: `tobemerged/HyperReview_Frontend/components/RemoteToolBar.tsx`
  Destination: `frontend/src/components/toolbars/RemoteToolBar.tsx`

- [ ] T044 [US3] Move RemoteTaskTree.tsx from new frontend to components/task-trees/
  Source: `tobemerged/HyperReview_Frontend/components/RemoteTaskTree.tsx`
  Destination: `frontend/src/components/task-trees/RemoteTaskTree.tsx`

- [ ] T045 [US3] Move RemoteRightPanel.tsx from new frontend to components/panels/
  Source: `tobemerged/HyperReview_Frontend/components/RemoteRightPanel.tsx`
  Destination: `frontend/src/components/panels/RemoteRightPanel.tsx`

#### Implementation Phase

- [ ] T046 [US3] Integrate Remote-specific components into mode-aware wrappers
  File: `frontend/src/components/ToolBar.tsx`, `TaskTree.tsx`, `RightPanel.tsx` (already done in Phase 3)

- [ ] T047 [US3] Ensure Remote mode components only display when mode === 'remote'

- [ ] T048 [US3] Create useRemoteReviewStore for remote platform and change management state
  File: `frontend/src/store/useRemoteReviewStore.ts`

- [ ] T049 [US3] Integrate Gerrit configuration dialogs (GerritServerModal, GerritImportModal) into Remote mode
  File: `frontend/src/components/` (from existing or new as needed)

#### Integration Phase

- [ ] T050 [US3] Verify existing GerritServerModal and GerritImportModal work in Remote mode context
  File: `frontend/src/components/`

- [ ] T051 [US3] Test that Gerrit-related Tauri commands work with new Remote mode UI
  File: Test with `npm run tauri dev`

#### Testing Phase

- [ ] T052 [US3] Test switching to Remote mode displays Remote-specific components
- [ ] T053 [US3] Test Gerrit server configuration UI in Remote mode
- [ ] T054 [US3] Test Gerrit change import functionality in Remote mode
- [ ] T055 [US3] Test that Gerrit commands show graceful degradation when backend features are not yet implemented
- [ ] T056 [US3] Verify that existing Local mode functionality is not affected when in Remote mode

### Success Criteria

- Remote mode successfully displays Remote-specific components (toolbar, task tree, right panel)
- Gerrit configuration dialogs (server setup, change import) work in Remote mode
- Gerrit-related Tauri commands (gerrit_get_instances_simple, gerrit_create_instance_simple, gerrit_import_change_simple) work correctly
- Graceful error messages shown when attempting to use unimplemented backend features
- Local mode functionality remains accessible when switching back from Remote mode
- No crashes or errors when accessing Remote review interface

---

## Phase 5: User Story 4 - Access Enhanced Settings (Priority: P3)

**Goal**: Users can access application settings with new features like font size, ligatures, and Vim mode configuration that were added in the new frontend. The SettingsModal integrates these new options with existing settings.

### Tasks

#### Setup Phase

- [ ] T057 [US4] Add CSS variables for editor settings to index.css
  File: `frontend/src/index.css`

- [ ] T058 [US4] Create useEditorStyles custom hook for dynamic CSS variable updates
  File: `frontend/src/hooks/useEditorStyles.ts`

#### Implementation Phase

- [ ] T059 [US4] Update SettingsModal.tsx to integrate font size slider (12-24px range)
  File: `frontend/src/components/SettingsModal.tsx`

- [ ] T060 [US4] Update SettingsModal.tsx to add ligatures toggle
  File: `frontend/src/components/SettingsModal.tsx`

- [ ] T061 [US4] Update SettingsModal.tsx to add Vim mode toggle
  File: `frontend/src/components/SettingsModal.tsx`

- [ ] T062 [US4] Ensure SettingsModal works in both Local and Remote modes
  File: `frontend/src/components/SettingsModal.tsx`

#### Integration Phase

- [ ] T063 [US4] Integrate useEditorStyles into App.tsx for global style application
  File: `frontend/src/App.tsx`

- [ ] T064 [US4] Ensure CSS variables are properly defined and scoped
  File: `frontend/src/index.css`

#### Testing Phase

- [ ] T065 [US4] Test that font size slider changes DiffView font size immediately
- [ ] T066 [US4] Test that ligatures toggle affects code display in DiffView
- [ ] T067 [US4] Test that Vim mode toggle changes cursor to block style
- [ ] T068 [US4] Test that settings persist across application restarts via Tauri Store
- [ ] T069 [US4] Test that settings modal works in both Local and Remote modes
- [ ] T070 [US4] Verify that all code display components use CSS variables correctly

### Success Criteria

- Font size can be adjusted from 12px to 24px via slider
- Ligatures can be toggled on/off and affects code display
- Vim mode can be enabled/disabled and changes cursor to block style
- Settings changes take effect immediately without page reload
- Settings persist across application restarts via Tauri Store
- Settings modal works correctly in both Local and Remote modes
- All code display components (DiffView, etc.) use CSS variables for styling
- No regressions to existing settings functionality

---

## Phase 6: Final Polish

**Purpose**: Testing, documentation, and cleanup

### Tasks

- [ ] T071 Update App.tsx to integrate SettingsProvider for settings context
  File: `frontend/src/App.tsx`

- [ ] T072 Update App.tsx to integrate ReviewModeProvider for mode context
  File: `frontend/src/App.tsx`

- [ ] T073 Integrate all shared components into App.tsx (TitleBar, WelcomeView, Modals)
  File: `frontend/src/App.tsx`

- [ ] T074 Create IPC client abstraction in services/tauri-client.ts
  File: `frontend/src/services/tauri-client.ts`

- [ ] T075 Update all components to use IPC client abstraction instead of direct Tauri invoke
  Files: All components that call Tauri commands

- [ ] T076 Test mode switching with all components integrated
  Test: Manual testing with `npm run tauri dev`

- [ ] T077 Test Local review workflow end-to-end
  Test: Manual testing with `npm run tauri dev`

- [ ] T078 Test Remote review workflow end-to-end (as much as possible with existing backend)
  Test: Manual testing with `npm run tauri dev`

- [ ] T079 Test editor settings functionality
  Test: Manual testing with `npm run tauri dev`

- [ ] T080 Verify all existing tests pass
  Test: `npm test` (if tests exist)

- [ ] T081 Run type checking and linting
  Test: `npm run typecheck && npm run lint`

### Success Criteria

- All components integrated into App.tsx with proper context providers
- IPC client abstraction working with error handling and graceful degradation
- Mode switching works smoothly with all components rendering correctly
- Local review workflow works without regressions
- Remote review interface works (where backend support exists) with appropriate error messages
- Editor settings work correctly and persist properly
- All existing tests pass
- No TypeScript or linting errors
- Application launches successfully with `npm run tauri dev`

---

## Dependencies

| Task | Dependencies | Blocked By |
|------|-------------|-----------|
| T001-T009 | None | - |
| T010-T012 | T001-T009 (complete first) | T001-T009 |
| T013-T022 | T010-T012 (complete first) | T010-T012 |
| T023-T042 | T013-T022 (complete first) | T013-T022 |
| T043-T056 | T023-T042 (complete first) | T023-T042 |
| T057-T070 | T043-T056 (complete first) | T043-T056 |

## Implementation Strategy

### MVP Approach

**Phase 1 + Phase 2 + User Story 1** provides the minimum viable product:
- Core infrastructure in place (contexts, stores, IPC abstraction)
- Mode switching works (can toggle between Local and Remote)
- Local review mode fully functional with migrated components
- Basic editor settings implemented

This allows users to:
1. Use existing Local review functionality with improved mode separation
2. Switch between Local and Remote modes
3. Configure basic editor settings

**Incremental Delivery**:
- Each user story adds value independently
- Can deliver User Story 1 (mode switching + Local review) as MVP
- User Stories 2 and 3 add Remote interface and enhanced settings
- Final Phase ensures polish and quality

### Parallel Execution Opportunities

**Can be executed in parallel** (no dependencies):
- T001 (create component subdirectories)
- T002 (create IPC directory)
- T003 (create contexts directory)
- T004 (create hooks directory)
- T005 (create services directory)
- T006 (add CSS variables)
- T007 (create useAppStore)
- T008 (create useLocalReviewStore)
- T009 (create useRemoteReviewStore)

**Must be sequential**:
- T010-T012: Cannot start until component subdirectories created (T001-T009)
- T013-T022: Cannot start until contexts created (T007-T009)
- T023-T042: Cannot start until components moved (T023-T025)
- T043-T056: Cannot start until mode-aware wrappers created (T027-T030)

---

## File Paths

All file paths are relative to the project root (`/home/c2j/workspace/CR/HyperReview_NEWAgain/HyperReview/`):

### Phase 1
- `frontend/src/components/` (new directory)
- `frontend/src/ipc/` (new directory)
- `frontend/src/contexts/` (new directory)
- `frontend/src/hooks/` (new directory)
- `frontend/src/services/` (new directory)
- `frontend/src/api/` (update existing files)
- `frontend/src/store/useAppStore.ts` (new file)
- `frontend/src/store/useLocalReviewStore.ts` (new file)
- `frontend/src/store/useRemoteReviewStore.ts` (new file)
- `frontend/src/index.css` (update existing)
- `frontend/package.json` (update dependencies)

### Phase 2 (US1)
- `frontend/src/components/ModeToggle.tsx` (new file)
- `frontend/src/contexts/SettingsContext.tsx` (new file)
- `frontend/src/contexts/ReviewModeContext.tsx` (new file)
- `frontend/src/App.tsx` (update existing)
- `frontend/src/components/toolbars/LocalToolBar.tsx` (moved)
- `frontend/src/components/task-trees/LocalTaskTree.tsx` (moved)
- `frontend/src/components/panels/LocalRightPanel.tsx` (moved)
- `frontend/src/components/WelcomeView.tsx` (moved)

### Phase 3 (US1)
- `frontend/src/components/ToolBar.tsx` (refactored)
- `frontend/src/components/TaskTree.tsx` (refactored)
- `frontend/src/components/RightPanel.tsx` (refactored)

### Phase 4 (US3)
- `frontend/src/components/toolbars/RemoteToolBar.tsx` (moved)
- `frontend/src/components/task-trees/RemoteTaskTree.tsx` (moved)
- `frontend/src/components/panels/RemoteRightPanel.tsx` (moved)
- `frontend/src/store/useRemoteReviewStore.ts` (new file)

### Phase 5 (US4)
- `frontend/src/hooks/useEditorStyles.ts` (new file)
- `frontend/src/components/SettingsModal.tsx` (updated)

### Phase 6
- `frontend/src/services/tauri-client.ts` (new file)
- All component updates for IPC abstraction
- `frontend/src/App.tsx` (final integration)

---

## Testing Notes

### Existing Test Suite
Run `npm test` to verify that existing frontend tests continue to pass after merge.

### Manual Testing Checklist

- [ ] Mode switching works correctly
- [ ] Local review workflow works end-to-end
- [ ] Remote review interface works (where backend support exists)
- [ ] Editor settings persist and apply correctly
- [ ] Application launches without crashes
- [ ] No console errors or warnings during normal usage
- [ ] Performance meets constitution requirements (<200ms command response, <1s mode switching, 60fps UI)

---

## Completion Criteria

**Phase 1 Complete**: When all core infrastructure (contexts, stores, IPC, CSS) is implemented
**Phase 2 Complete**: When User Story 1 can be fully tested independently
**Phase 3 Complete**: When User Story 1 and User Story 2 are both working
**Phase 4 Complete**: When User Story 3 can be fully tested independently
**Phase 5 Complete**: When User Story 4 can be fully tested independently
**Phase 6 Complete**: When all polish, testing, and cleanup tasks are done

**Feature Complete**: When all phases are complete and success criteria for each user story are met
