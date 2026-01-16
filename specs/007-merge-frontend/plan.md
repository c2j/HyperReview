# Implementation Plan: Merge HyperReview Frontend

**Branch**: `007-merge-frontend` | **Date**: 2025-01-16 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/007-merge-frontend/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Merge the UI components and interface design from `tobemerged/HyperReview_Frontend` into the current `frontend` directory while preserving all existing Tauri IPC integrations, Gerrit services, and backend communication paths. The HyperReview_Frontend serves as the UI standard (mode switching, panel resizing, component organization), while the current frontend's IPC client, services, and business logic are maintained. Key approach: prioritize HyperReview_Frontend components for UI, port IPC integrations from current frontend, maintain directory structure, and provide graceful degradation for features that cannot be reconciled.

## Technical Context

**Language/Version**: Rust 1.75+, TypeScript 5+, React 18+
**Primary Dependencies**: Tauri 2.x, Zustand (state management), git2-rs (Git operations), rusqlite (metadata), reqwest (HTTP client), serde (serialization), Jest + React Testing Library
**Storage**: rusqlite (metadata), file system (repositories)
**Testing**: Jest + React Testing Library (80% UI coverage), cargo test (100% command coverage), Playwright (E2E)
**Target Platform**: Desktop (Windows 10+, macOS 11+, Linux Ubuntu 20.04+)
**Project Type**: Desktop application (Tauri + React)
**Performance Goals**: <200ms IPC command response, 60fps UI interactions, virtual scrolling for >5000 line diffs, <15MB Windows bundle
**Constraints**: No backend modifications (src-tauri unchanged), preserve all existing Tauri IPC integrations, maintain backward compatibility, maintain current frontend directory structure with api/, components/, context/, hooks/, store/, services/, types/, __tests__/
**Scale/Scope**: Merge ~20 UI components from HyperReview_Frontend, integrate with existing Gerrit services, support mode switching (local/remote), preserve ~15 existing components with unique functionality

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Team Roles & Responsibilities
- ✅ **Role Separation**: Frontend merge only - no backend (Rust) modifications planned
- ✅ **Clear Boundaries**: All IPC integrations preserved, frontend components updated
- ✅ **IPC Interface**: Dual review will be required for any IPC interface changes (clarification: no IPC interface changes planned, only frontend UI merge)

### II. Project Structure & Code Standards
- ✅ **Structure**: Maintaining current frontend structure (components/, hooks/, store/, services/, etc.)
- ✅ **State Management**: Zustand preserved from current frontend
- ✅ **Code Standards**: TypeScript strict mode, ESLint 9, Prettier, Tailwind class sorting will be maintained
- ✅ **Pre-commit Hooks**: husky + lint-staged already in place, will continue

### III. IPC Interaction & Security
- ✅ **No Backend Changes**: Explicit constraint that src-tauri will not be modified
- ✅ **Security Model**: Frontend continues to invoke via Tauri's invoke API only, no direct filesystem/network access
- ✅ **Input Sanitization**: All sanitization remains in Rust (unchanged)
- ✅ **IPC Interface**: No new IPC commands planned, all existing integrations preserved

### IV. Testing & CI/CD
- ⚠️ **Testing Coverage**: Clarified priority on core business logic and services (Gerrit, IPC, data models) - 80% coverage target for preserved components and services
- ✅ **CI Pipeline**: Existing GitHub Actions will continue, all tests must pass before merge
- ✅ **Code Review**: Minimum 1 approval required, conventional commits enforced

### V. Documentation, Performance & Accountability
- ✅ **Documentation**: Will merge and update all documentation (IPC.md, OpenAPI.md, design-backend.md) from HyperReview_Frontend
- ✅ **Performance**: <200ms IPC response preserved, 60fps UI interactions maintained
- ✅ **Bundle Size**: <15MB Windows bundle constraint maintained

### Security & Performance Standards
- ✅ **Technology Stack**: Matches constitution (Tauri 2.x + React 18+ + TypeScript 5.x + Rust 1.75+)
- ✅ **State Management**: Zustand for frontend, native Rust structs for backend (preserved)
- ✅ **Performance Constraints**: All constraints met (<200ms Rust response, virtual scrolling, 60fps, <15MB bundle)
- ✅ **Security Model**: No changes to security model, all privileged operations remain in Rust

### Development Workflow & Quality Gates
- ✅ **Branching**: Feature branch created (007-merge-frontend), PR required
- ✅ **Code Review**: All PRs must verify constitution compliance
- ✅ **Testing Gates**: Frontend 80% coverage for preserved services, E2E tests for critical paths
- ✅ **IPC Interface Changes**: No IPC interface changes planned (graceful degradation if needed)
- ✅ **Quality Gates**: Constitution compliance verified

**GATE STATUS**: ✅ PASS (with clarifications documented)

**Notes**:
- Testing coverage clarification: Priority on core business logic and services rather than full UI component coverage
- No backend modifications planned, maintaining strict role separation
- All existing IPC integrations preserved, maintaining security model
- Documentation from HyperReview_Frontend will be merged and updated

## Project Structure

### Documentation (this feature)

```text
specs/007-merge-frontend/
├── spec.md              # Feature specification
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
├── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
└── checklists/
    └── requirements.md  # Specification quality checklist
```

### Source Code (repository root)

```text
src-tauri/                # Rust backend - NO MODIFICATIONS
├── src/
│   ├── commands.rs      # Tauri IPC commands (unchanged)
│   ├── models.rs        # Data models (unchanged)
│   └── lib.rs           # Main library (unchanged)
└── Cargo.toml           # Rust dependencies (unchanged)

frontend/                 # React frontend - MERGE TARGET
├── src/
│   ├── api/
│   │   ├── client.ts    # API client with IPC integrations (preserve + merge)
│   │   └── types/       # TypeScript type definitions (merge from HyperReview_Frontend)
│   ├── components/
│   │   # Merged from HyperReview_Frontend (UI priority) + preserved unique components
│   │   ├── DiffView.tsx
│   │   ├── LocalTaskTree.tsx
│   │   ├── RemoteTaskTree.tsx
│   │   ├── LocalToolBar.tsx
│   │   ├── RemoteToolBar.tsx
│   │   ├── LocalRightPanel.tsx
│   │   ├── RemoteRightPanel.tsx
│   │   ├── GerritImportModal.tsx
│   │   ├── GerritServerModal.tsx
│   │   ├── # ... other HyperReview_Frontend components
│   │   ├── CredentialManager.tsx  # Preserved (unique to current frontend)
│   │   └── ExternalSubmissionDialog.tsx  # Preserved (unique to current frontend)
│   ├── context/
│   │   # Existing context providers (preserved)
│   ├── hooks/
│   │   # Existing custom hooks (preserved)
│   │   └── useIPC.ts  # IPC invocation hook (preserved)
│   ├── services/
│   │   ├── gerrit-simple-service.ts  # Preserved
│   │   ├── gerrit-instance-service.ts # Preserved
│   │   ├── reviewService.ts  # Preserved
│   │   └── # ... other services (preserved)
│   ├── store/
│   │   # Zustand stores (preserved)
│   ├── App.tsx          # Main app - merge from HyperReview_Frontend with IPC integrations
│   ├── i18n.tsx         # Internationalization (preserved + merge translations)
│   └── index.tsx        # Entry point (preserved)
├── __tests__/           # Test files (preserve + update for merged components)
├── package.json         # Dependencies (merge from HyperReview_Frontend if needed)
└── tsconfig.json        # TypeScript config (preserved)

tobemerged/HyperReview_Frontend/  # Source for UI components (merge source, not part of final codebase)
├── App.tsx              # Reference for main app structure
├── components/          # Reference for UI components
├── api/                 # Reference for API types
└── # ... other files used as reference during merge
```

**Structure Decision**: Desktop application (Tauri + React) structure maintained. Frontend merge preserves existing directory structure (api/, components/, context/, hooks/, store/, services/, types/, __tests__/) from current frontend. Components from HyperReview_Frontend integrated as the UI standard. Unique components from current frontend (CredentialManager, ExternalSubmissionDialog, etc.) preserved. All services and IPC integrations from current frontend maintained.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

No violations requiring justification. Constitution check passed with clarifications documented.

---

## Phase 0: Research & Analysis

### Unknowns to Research

**NONE** - All technical context and decisions are clear based on feature spec clarifications and constitution requirements.

### Best Practices

**None requiring additional research** - Feature merge follows established patterns within the codebase and constitution guidelines.

### Integration Patterns

**None requiring additional research** - IPC integration pattern is already established in current frontend and will be preserved during merge.

### Research Tasks

No research tasks required. All decisions are clear from:
- Feature specification with clarifications
- Constitution requirements
- Existing codebase structure
- Merge strategy defined in clarifications

---

## Phase 1: Design & Contracts

### Data Model

See [data-model.md](./data-model.md) for detailed entity definitions, relationships, and state transitions.

### API Contracts

See [contracts/](./contracts/) for IPC interface definitions and TypeScript type definitions.

### Quickstart Guide

See [quickstart.md](./quickstart.md) for development setup and merge workflow.

---

## Phase 2: Implementation Tasks

See [tasks.md](./tasks.md) for detailed task breakdown (generated by `/speckit.tasks` command).

---

**Plan Status**: ✅ Complete - Ready for `/speckit.tasks`
