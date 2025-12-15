# Implementation Plan: Frontend-Backend Integration

**Branch**: `002-frontend-backend-integration` | **Date**: 2025-12-14 | **Spec**: [link to spec.md]
**Input**: Feature specification from `/specs/002-frontend-backend-integration/spec.md`

## Summary

Replace all mock API calls in the React frontend with actual Tauri IPC calls to the Rust backend. This enables full code review functionality including repository management, diff viewing, comment system, task management, and analysis features. The backend is already complete with 21 IPC commands, SQLite storage, and performance monitoring. Focus is on frontend integration, error handling, and state management.

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**: TypeScript 5+ (React), Rust 1.75+ (Tauri v2)
**Primary Dependencies**: Tauri v2, React 18, Vite, git2-rs, rusqlite, tree-sitter
**Storage**: SQLite (via rusqlite) for local metadata; file system for git repositories
**Testing**: Tauri integration tests, React component tests (Vitest/Jest), E2E tests
**Target Platform**: Cross-platform desktop (Windows, macOS, Linux)
**Project Type**: Tauri desktop application with React frontend
**Performance Goals**: 95% of operations < 200ms, 60fps scrolling for large diffs
**Constraints**: <2GB memory for large repos, offline-first, local data only
**Scale/Scope**: Support repositories up to 100k files, handle 10k+ line diffs

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**PASS** - All gates satisfied:

1. ✅ **Team Roles & Responsibilities**: Frontend will only invoke existing Rust commands via Tauri IPC; no backend code modification required. Backend is complete and stable.

2. ✅ **Project Structure & Code Standards**: Existing structure follows Tauri conventions:
   - `src-tauri/src/` contains commands.rs, models.rs, lib.rs
   - Frontend uses React/TypeScript with Tauri invoke API
   - All business logic remains in Rust backend

3. ✅ **IPC Interaction & Security**:
   - All 21 IPC commands already implemented in Rust
   - Frontend only uses invoke() API for communication
   - No secrets in frontend, all operations via backend
   - Security validation already in place (T097 completed)

4. ✅ **Testing**: Will implement React component tests + Tauri integration tests

5. ✅ **Performance**: Backend performance monitoring (T090) already tracks <200ms SLA

### Post-Design Re-evaluation

**PASS** - All gates continue to satisfy constitution after design phase:

1. ✅ **Team Roles**: Design maintains separation - frontend only invokes IPC, no backend changes
2. ✅ **Structure**: Tauri structure confirmed with proper directories (src/, src-tauri/src/)
3. ✅ **IPC Security**: 21 commands documented, all validation in backend
4. ✅ **Testing**: Comprehensive test strategy defined (unit, integration, E2E)
5. ✅ **Performance**: Virtual scrolling and caching strategies align with performance goals

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)
<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
-->

```text
# [REMOVE IF UNUSED] Option 1: Single project (DEFAULT)
src/
├── models/
├── services/
├── cli/
└── lib/

tests/
├── contract/
├── integration/
└── unit/

# [REMOVE IF UNUSED] Option 2: Web application (when "frontend" + "backend" detected)
backend/
├── src/
│   ├── models/
│   ├── services/
│   └── api/
└── tests/

frontend/
├── src/
│   ├── components/
│   ├── pages/
│   └── services/
└── tests/

# [REMOVE IF UNUSED] Option 3: Mobile + API (when "iOS/Android" detected)
api/
└── [same as backend above]

ios/ or android/
└── [platform-specific structure: feature modules, UI flows, platform tests]
```

**Structure Decision**: Tauri desktop application structure
- Frontend: React components in `src/` (or `frontend/` if separate)
- Backend: Rust commands in `src-tauri/src/`
- Database: SQLite schema in `src-tauri/src/storage/sqlite.rs`
- Integration: Tauri IPC commands bridge between React and Rust

Existing repository structure already follows Tauri conventions and constitution requirements.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
