# Implementation Plan: Merge New Frontend with Local/Remote Review Mode Separation

**Branch**: `009-merge-new-frontend` | **Date**: 2025-01-18 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/009-merge-new-frontend/spec.md`

## Summary

Merge the new frontend codebase from `tobemerged/HyperReview_Frontend` into the existing `frontend` directory while maintaining compatibility with the existing Tauri 1.8 backend. The new frontend provides clear Local/Remote review mode separation with dedicated UI components (LocalToolBar/LocalTaskTree/LocalRightPanel vs RemoteToolBar/RemoteTaskTree/RemoteRightPanel) and enhanced editor settings (font size, ligatures, Vim mode). The merge must preserve all existing Local review functionality, integrate with existing Tauri IPC commands, and provide seamless mode transitions without breaking the application.

## Technical Context

**Language/Version**: TypeScript 5.x + React 18+ (Frontend), Rust 1.75+ (Backend)
**Primary Dependencies**: Tauri 1.8/2.x, Vite, Zustand, git2-rs, rusqlite, reqwest, serde, thiserror
**Storage**: SQLite for backend metadata (hyper_review.db), localStorage for frontend user preferences
**Testing**: Jest + React Testing Library (80% UI coverage), cargo test (100% command coverage)
**Target Platform**: Desktop (Windows 10+, macOS 11+, Linux Ubuntu 20.04+)
**Project Type**: Desktop application (single Tauri app with React frontend and Rust backend)
**Performance Goals**: <200ms Rust command response, <15MB Windows bundle, 60fps UI interactions, <1s mode switching, virtual scrolling for >5000 line diffs
**Constraints**: Tauri IPC security model (no direct frontend filesystem access), maintain backward compatibility with existing backend commands, preserve all existing Local review functionality
**Scale/Scope**: Single desktop application, merging two React codebases (~50+ components combined), maintaining ~50+ existing Tauri IPC commands

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Role Compliance

**Status**: ✅ PASS

- Frontend Lead (React/TS expert) will own UI, state management, and IPC invocation patterns
- Rust Backend Lead will own all commands, security, git2 integration
- Clear boundaries maintained: frontend never modifies Rust backend code
- All business logic remains in Rust commands

### Project Structure Compliance

**Status**: ✅ PASS

- Frontend structure: `src/` with components/, hooks/, store/, services/, api/
- Backend structure: `src-tauri/src/` with commands.rs, models.rs, lib.rs
- Code standards: ESLint 9 flat config, Prettier, TypeScript strict mode, Tailwind, rustfmt, clippy
- Conventional Commits required

### IPC & Security Compliance

**Status**: ✅ PASS

- All sensitive operations (git diff, file I/O) execute in Rust commands
- Frontend invokes via Tauri invoke API only
- IPC interface changes require dual review (Rust Lead + Frontend Lead)
- tauri.conf.json allowlist will be minimally scoped
- All user input sanitized in Rust

### Testing & CI/CD Compliance

**Status**: ✅ PASS

- Jest + React Testing Library for frontend (80% coverage target)
- cargo test for backend (100% command coverage)
- All existing tests must pass after merge
- No direct pushes to main - all changes via PR

### Performance Standards

**Status**: ✅ PASS

- <15MB Windows bundle target
- <200ms Rust command response time
- Virtual scrolling for diffs >5000 lines
- Mode switching within 1 second

### Quality Gates

**Status**: ✅ PASS

- No breaking changes to Tauri backend required
- All existing functionality must be preserved
- IPC interface compatibility maintained
- Performance regressions prevented

**Result**: All gates passed. Proceed with Phase 0 research.

## Project Structure

### Documentation (this feature)

```text
specs/009-merge-new-frontend/
 ├── plan.md              # This file (/speckit.plan command output)
 ├── research.md          # Phase 0 output (/speckit.plan command)
 ├── data-model.md        # Phase 1 output (/speckit.plan command)
 ├── quickstart.md        # Phase 1 output (/speckit.plan command)
 ├── contracts/           # Phase 1 output (/speckit.plan command)
 │   ├── ipc-interfaces.md  # Tauri IPC command contracts
 │   └── api-types.md     # TypeScript type definitions
 └── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
frontend/
 ├── src/
 │   ├── components/          # React components (merged from both codebases)
 │   │   ├── local/         # Local mode specific components
 │   │   │   ├── LocalToolBar.tsx
 │   │   │   ├── LocalTaskTree.tsx
 │   │   │   └── LocalRightPanel.tsx
 │   │   ├── remote/        # Remote mode specific components
 │   │   │   ├── RemoteToolBar.tsx
 │   │   │   ├── RemoteTaskTree.tsx
 │   │   │   └── RemoteRightPanel.tsx
 │   │   ├── shared/        # Shared components (existing + new)
 │   │   │   ├── DiffView.tsx
 │   │   │   ├── TitleBar.tsx
 │   │   │   ├── WelcomeView.tsx
 │   │   │   └── Modal.tsx
 │   │   └── settings/      # Settings-related components
 │   │       └── SettingsModal.tsx
 │   ├── hooks/              # Custom React hooks
 │   ├── store/              # Zustand state management
 │   │   ├── useAppStore.ts  # Global app state (mode, settings)
 │   │   └── useReviewStore.ts # Review-specific state
 │   ├── services/           # API services and business logic
 │   │   └── tauri-client.ts # Tauri IPC client abstraction
 │   ├── api/               # API type definitions
 │   │   ├── commands.ts     # Tauri command definitions
 │   │   └── types.ts       # Shared TypeScript types
 │   ├── context/            # React contexts (if needed beyond Zustand)
 │   ├── config/            # Configuration constants
 │   ├── utils/              # Utility functions
 │   ├── types/              # Type definitions
 │   ├── App.tsx             # Main application component
 │   └── main.tsx            # Entry point
 ├── tests/                  # Frontend tests
 │   ├── unit/
 │   ├── integration/
 │   └── e2e/
 ├── package.json
 ├── vite.config.ts
 ├── tsconfig.json
 └── tailwind.config.js

src-tauri/                    # Existing backend (unchanged)
 ├── src/
 │   ├── commands/           # Existing Tauri commands
 │   ├── models/
 │   ├── lib.rs
 │   └── main.rs
 ├── Cargo.toml
 └── tauri.conf.json
```

**Structure Decision**: Adopt Option 2 (Tauri application with frontend/backend separation) which matches the existing project structure. The frontend directory will be reorganized to separate Local/Remote mode components while maintaining shared components. This approach:
- Preserves existing Tauri backend structure without modifications
- Enables clear mode separation in frontend component organization
- Supports the constitution's requirement for clear frontend/backend boundaries
- Allows incremental migration and testing of merged components

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | All gates passed | N/A |
