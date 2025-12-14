---

description: "Task list template for frontend-backend integration feature"
---

# Tasks: Frontend-Backend Integration

**Input**: Design documents from `/specs/002-frontend-backend-integration/`
**Prerequisites**: plan.md (required), spec.md (required), data-model.md (required), contracts/ipc-commands.md (required), research.md, quickstart.md

**Tests**: The examples below include test tasks. Tests are OPTIONAL - only include them if explicitly requested in the feature specification.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Tauri app**: `src-tauri/src/` for Rust code (already complete)
- **Frontend modules**: `src/` for React frontend
- **API layer**: `src/api/client.ts` for IPC integration
- **Types**: `src/api/types/` for TypeScript interfaces
- **Components**: `src/components/` for React components
- **Hooks**: `src/hooks/` for custom React hooks
- **Tests**: `src/__tests__/` for frontend tests
- **Integration tests**: `tests/` for Tauri integration tests
- Paths shown below use standard frontend structure - adjust based on project structure

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and configuration for Tauri integration

**⚠️ CRITICAL**: Backend is already complete - only frontend integration needed

- [ ] T001 Create TypeScript API types matching backend models in src/api/types/index.ts
- [ ] T002 [P] Set up Zustand state management store in src/store/reviewStore.ts
- [ ] T003 [P] Create custom IPC hooks for Tauri invoke calls in src/hooks/useIPC.ts
- [ ] T004 Create error handling utilities with toast notifications in src/utils/errorHandler.ts
- [ ] T005 Set up loading state management context in src/contexts/LoadingContext.tsx
- [ ] T006 Configure environment variables for Tauri integration in .env.local

**Checkpoint**: Frontend infrastructure ready for IPC integration

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core integration infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T007 Update API client to use real Tauri IPC instead of mocks in src/api/client.ts
- [ ] T008 [P] Create API service layer with all 21 IPC command wrappers in src/api/service.ts
- [ ] T009 [P] Implement data validation for all API responses in src/utils/validation.ts
- [ ] T010 Create global error boundary component in src/components/ErrorBoundary.tsx
- [ ] T011 Set up virtual scrolling library (react-window) for large diffs in src/components/DiffViewer.tsx
- [ ] T012 Create repository state management hooks in src/hooks/useRepository.ts
- [ ] T013 Implement caching layer for frequently accessed data in src/utils/cache.ts
- [ ] T014 Add performance monitoring for frontend operations in src/utils/metrics.ts

**Checkpoint**: Foundation ready - IPC integration, state management, and performance optimizations complete. User story implementation can now begin in parallel.

---

## Phase 3: User Story 1 - Repository Management (Priority: P1) 🎯 MVP

**Goal**: Enable users to open, browse, and manage Git repositories with real backend data

**Independent Test**: User selects a Git repository via file picker, system displays repository metadata, branch list, and recent repositories from SQLite backend. All data is real (no mocks).

### Implementation for User Story 1

- [ ] T015 [P] [US1] Implement open_repo_dialog integration in src/api/client.ts
- [ ] T016 [P] [US1] Implement get_recent_repos integration in src/api/client.ts
- [ ] T017 [P] [US1] Implement get_branches integration in src/api/client.ts
- [ ] T018 [P] [US1] Implement load_repo integration in src/api/client.ts
- [ ] T019 [US1] Create repository selector component in src/components/RepositorySelector.tsx
- [ ] T020 [US1] Create branch list component in src/components/BranchList.tsx
- [ ] T021 [US1] Create recent repositories component in src/components/RecentRepos.tsx
- [ ] T022 [US1] Update App.tsx to use real repository data instead of mocks
- [ ] T023 [US1] Add error handling for repository operations in errorHandler.ts

### Tests for User Story 1 (OPTIONAL - only if tests requested) ⚠️

- [ ] T024 [P] [US1] Unit test for repository API functions in src/__tests__/api.test.ts
- [ ] T025 [P] [US1] Unit test for repository hooks in src/__tests__/hooks.test.ts
- [ ] T026 Integration test for repository opening workflow in tests/integration/repo-management.test.ts

**Checkpoint**: At this point, User Story 1 should be fully functional - users can open repositories and browse metadata with real data.

---

## Phase 4: User Story 2 - Code Review & Comments (Priority: P1) 🎯 MVP

**Goal**: Enable zero-latency diff viewing with inline commenting and real backend integration

**Independent Test**: User opens any file and sees actual diff from backend, can scroll smoothly with virtual scrolling, add comments that persist in SQLite. Delivers core value proposition of superior diff viewing with real data.

### Implementation for User Story 2

- [ ] T027 [P] [US2] Implement get_file_diff integration in src/api/client.ts
- [ ] T028 [P] [US2] Implement add_comment integration in src/api/client.ts
- [ ] T029 [P] [US2] Implement virtual scrolling for large diffs in src/components/DiffViewer.tsx
- [ ] T030 [US2] Create comment form component in src/components/CommentForm.tsx
- [ ] T031 [US2] Create diff line component with inline analysis in src/components/DiffLine.tsx
- [ ] T032 [US2] Update DiffViewer to use real diff data with virtual scrolling
- [ ] T033 [US2] Add comment thread display in src/components/CommentThread.tsx
- [ ] T034 [US2] Implement comment persistence and real-time updates
- [ ] T035 [US2] Add diff performance optimizations for 10k+ line files

### Tests for User Story 2 (OPTIONAL - only if tests requested) ⚠️

- [ ] T036 [P] [US2] Unit test for diff API functions in src/__tests__/api.test.ts
- [ ] T037 [P] [US2] Unit test for virtual scrolling component in src/__tests__/DiffViewer.test.tsx
- [ ] T038 Integration test for comment workflow in tests/integration/review-workflow.test.ts

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently - users can open repos and review diffs with real comments.

---

## Phase 5: User Story 3 - Task Management & Analytics (Priority: P2)

**Goal**: Enable task management, progress tracking, and quality gate monitoring with real data

**Independent Test**: User views pending tasks from backend, tracks review statistics calculated from real repository state, monitors quality gates, and uses review templates. Delivers professional workflow management with SQLite persistence.

### Implementation for User Story 3

- [ ] T039 [P] [US3] Implement get_tasks integration in src/api/client.ts
- [ ] T040 [P] [US3] Implement get_review_stats integration in src/api/client.ts
- [ ] T041 [P] [US3] Implement get_quality_gates integration in src/api/client.ts
- [ ] T042 [P] [US3] Implement get_review_templates integration in src/api/client.ts
- [ ] T043 [P] [US3] Implement create_template integration in src/api/client.ts
- [ ] T044 [US3] Create task tree component in src/components/TaskTree.tsx
- [ ] T045 [US3] Create review statistics dashboard in src/components/ReviewStats.tsx
- [ ] T046 [US3] Create quality gates panel in src/components/QualityGates.tsx
- [ ] T047 [US3] Create template manager in src/components/TemplateManager.tsx
- [ ] T048 [US3] Update RightPanel to display real task and stats data

### Tests for User Story 3 (OPTIONAL - only if tests requested) ⚠️

- [ ] T049 [P] [US3] Unit test for task API functions in src/__tests__/api.test.ts
- [ ] T050 [P] [US3] Unit test for statistics calculation in src/__tests__/stats.test.ts
- [ ] T051 Integration test for task workflow in tests/integration/task-management.test.ts

**Checkpoint**: At this point, User Stories 1, 2, AND 3 should all work independently with real backend data.

---

## Phase 6: User Story 4 - Analysis & Insights (Priority: P2)

**Goal**: Provide architectural impact analysis and intelligent review assistance from real backend

**Independent Test**: User views heatmap of file impacts from backend analysis, receives smart checklists based on actual file types, views git blame information. Delivers advanced reviewer assistance with real data.

### Implementation for User Story 4

- [ ] T052 [P] [US4] Implement get_heatmap integration in src/api/client.ts
- [ ] T053 [P] [US4] Implement get_checklist integration in src/api/client.ts
- [ ] T054 [P] [US4] Implement get_blame integration in src/api/client.ts
- [ ] T055 [P] [US4] Implement analyze_complexity integration in src/api/client.ts
- [ ] T056 [P] [US4] Implement scan_security integration in src/api/client.ts
- [ ] T057 [US4] Create heatmap visualization component in src/components/Heatmap.tsx
- [ ] T058 [US4] Create smart checklist component in src/components/SmartChecklist.tsx
- [ ] T059 [US4] Create blame viewer component in src/components/BlameViewer.tsx
- [ ] T060 [US4] Create complexity metrics display in src/components/ComplexityMetrics.tsx
- [ ] T061 [US4] Create security scan results panel in src/components/SecurityScan.tsx

### Tests for User Story 4 (OPTIONAL - only if tests requested) ⚠️

- [ ] T062 [P] [US4] Unit test for analysis API functions in src/__tests__/api.test.ts
- [ ] T063 [P] [US4] Unit test for heatmap visualization in src/__tests__/Heatmap.test.tsx
- [ ] T064 Integration test for insights workflow in tests/integration/insights.test.ts

**Checkpoint**: At this point, User Stories 1-4 should all work independently with complete backend integration.

---

## Phase 7: User Story 5 - External Integration (Priority: P3)

**Goal**: Integrate with external systems (GitLab, Gerrit) for review submission

**Independent Test**: User submits review comments to external systems with authentication and proper formatting via backend. Delivers workflow integration with existing tools.

### Implementation for User Story 5

- [ ] T065 [P] [US5] Implement submit_review integration in src/api/client.ts
- [ ] T066 [P] [US5] Implement sync_repo integration in src/api/client.ts
- [ ] T067 [US5] Create submit review modal in src/components/SubmitReviewModal.tsx
- [ ] T068 [US5] Create sync status modal in src/components/SyncStatusModal.tsx
- [ ] T069 [US5] Add credential management UI in src/components/CredentialManager.tsx
- [ ] T070 [US5] Implement error handling for network failures with offline draft preservation

### Tests for User Story 5 (OPTIONAL - only if tests requested) ⚠️

- [ ] T071 [P] [US5] Unit test for external integration APIs in src/__tests__/api.test.ts
- [ ] T072 [US5] Integration test for review submission in tests/integration/external-submission.test.ts

**Checkpoint**: All user stories complete with external system integration.

---

## Phase 8: User Story 6 - Search & Configuration (Priority: P3)

**Goal**: Enable fast search across repository and manage configuration with real backend

**Independent Test**: User searches files/symbols/commits and receives real results from backend within 500ms, manages tags and templates with SQLite persistence. Delivers enhanced productivity tools.

### Implementation for User Story 6

- [ ] T073 [P] [US6] Implement search integration in src/api/client.ts
- [ ] T074 [P] [US6] Implement get_commands integration in src/api/client.ts
- [ ] T075 [P] [US6] Implement get_tags integration in src/api/client.ts
- [ ] T076 [P] [US6] Implement create_tag integration in src/api/client.ts
- [ ] T077 [US6] Create search component with real backend results in src/components/SearchBox.tsx
- [ ] T078 [US6] Create command palette in src/components/CommandPalette.tsx
- [ ] T079 [US6] Create tag manager in src/components/TagManager.tsx
- [ ] T080 [US6] Update ActionBar to use real search and command data

### Tests for User Story 6 (OPTIONAL - only if tests requested) ⚠️

- [ ] T081 [P] [US6] Unit test for search API functions in src/__tests__/api.test.ts
- [ ] T082 [P] [US6] Unit test for search component in src/__tests__/SearchBox.test.tsx
- [ ] T083 Integration test for search workflow in tests/integration/search.test.ts

**Checkpoint**: All user stories complete with search and configuration tools.

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Performance optimization, testing, and final integration

- [ ] T084 [P] Add comprehensive React component tests (target 90% coverage) across all components in src/__tests__/
- [ ] T085 [P] Implement E2E tests for complete review workflow in tests/e2e/review-workflow.spec.ts
- [ ] T086 [P] Add performance monitoring for frontend operations in src/utils/metrics.ts
- [ ] T087 [P] Optimize bundle size with code splitting and lazy loading in vite.config.ts
- [ ] T088 Create user onboarding guide with tour in src/components/TourGuide.tsx
- [ ] T089 Add accessibility improvements (ARIA labels, keyboard navigation) across all components
- [ ] T090 Implement offline mode with local caching in src/utils/offlineCache.ts
- [ ] T091 Add documentation for all custom hooks in src/hooks/README.md
- [ ] T092 Create deployment checklist in DEPLOYMENT.md

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **User Stories (Phases 3-8)**: All depend on Foundational phase completion
  - US1 and US2 (both P1) can run in parallel after Foundational
  - US3 and US4 (both P2) can run in parallel after P1 stories complete
  - US5 and US6 (both P3) can run in parallel after P2 stories complete
- **Polish (Phase 9)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
  - Can run in parallel with US1
- **User Story 3 (P2)**: Can start after US1 and US2 complete - May integrate but independently testable
- **User Story 4 (P2)**: Can start after US1 and US2 complete - May integrate but independently testable
  - Can run in parallel with US3
- **User Story 5 (P3)**: Can start after US2 complete - Depends on commenting functionality
- **User Story 6 (P3)**: Can start after Foundational (Phase 2) - No dependencies on other stories
  - Can run in parallel with US5

### Within Each User Story

- Tests (if included) MUST be written and FAIL before implementation
- API integration before components
- Components before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel (T001-T006)
- All Foundational tasks marked [P] can run in parallel (T008-T014)
- US1 and US2 can run in parallel after Foundational
- US3 and US4 can run in parallel after P1 stories
- US5 and US6 can run in parallel (US5 needs US2, US6 is independent)
- All Polish tasks marked [P] can run in parallel (T084-T092)

---

## Parallel Example: User Story 1

```bash
# Launch all API integrations for User Story 1 together:
Task: "Implement open_repo_dialog integration in src/api/client.ts"
Task: "Implement get_recent_repos integration in src/api/client.ts"
Task: "Implement get_branches integration in src/api/client.ts"
Task: "Implement load_repo integration in src/api/client.ts"

# Then implement components:
Task: "Create repository selector component in src/components/RepositorySelector.tsx"
Task: "Create branch list component in src/components/BranchList.tsx"
Task: "Create recent repositories component in src/components/RecentRepos.tsx"
```

---

## Implementation Strategy

### MVP First (User Stories 1 & 2 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. Complete Phase 4: User Story 2
5. **STOP and VALIDATE**: Test User Stories 1 & 2 independently
6. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo (Complete MVP)
4. Add User Story 3 → Test independently → Deploy/Demo
5. Add User Story 4 → Test independently → Deploy/Demo
6. Add User Story 5 → Test independently → Deploy/Demo
7. Add User Story 6 → Test independently → Deploy/Demo
8. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (repository management)
   - Developer B: User Story 2 (diff viewing and comments)
3. Stories complete and integrate independently
4. Continue with remaining stories in parallel pairs

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence

**Total Tasks**: 92
**Setup**: 6 tasks
**Foundational**: 8 tasks
**User Stories**: 66 tasks (11 tasks per story average)
**Polish**: 9 tasks

**Task Distribution by User Story**:
- US1 (P1): 11 tasks (T015-T026)
- US2 (P1): 12 tasks (T027-T038)
- US3 (P2): 13 tasks (T039-T051)
- US4 (P2): 13 tasks (T052-T064)
- US5 (P3): 8 tasks (T065-T072)
- US6 (P3): 9 tasks (T073-T083)

---

## Key Success Criteria

### MVP Scope (US1 + US2)
- ✅ Users can open real Git repositories (not mocks)
- ✅ Repository metadata loads from SQLite backend
- ✅ Branch list displays real branches with commit data
- ✅ File diffs show actual changes from git2-rs
- ✅ Comments persist in SQLite and survive app restart
- ✅ Virtual scrolling handles 10k+ line files smoothly
- ✅ No mock data visible anywhere in UI

### Complete Integration (US1-US6)
- ✅ All 21 IPC commands integrated and functional
- ✅ Task management with real backend data
- ✅ Analysis and insights from actual repository state
- ✅ External system integration (GitLab/Gerrit)
- ✅ Search across repository with real results
- ✅ Tag and template management with persistence
- ✅ 90%+ test coverage
- ✅ E2E tests passing
- ✅ Performance SLA met (<200ms for typical operations)
