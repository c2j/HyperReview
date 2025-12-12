# Implementation Plan: HyperReview MVP

**Branch**: `001-pr-review-mvp` | **Date**: 2025-11-23 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-pr-review-mvp/spec.md`

## Summary

Build a native, GPU-accelerated PR review application using GPUI (Zed's UI framework). The application provides a unified inbox for GitHub PRs, a high-performance diff viewer with syntax highlighting, and keyboard-driven workflow for power users. Development follows three incremental milestones: Reader (local diff viewing), Connector (GitHub integration), and Reviewer (interactive comments).

## Technical Context

**Language/Version**: Rust (Edition 2024)
**Primary Dependencies**: GPUI, Tokio, git2-rs, sqlx, tree-sitter, reqwest, pulldown-cmark
**Storage**: SQLite via sqlx (local-first architecture)
**Testing**: cargo test with in-memory SQLite fixtures and git repository fixtures
**Target Platform**: Desktop (macOS, Windows, Linux)
**Project Type**: Single native desktop application
**Performance Goals**: <500ms cold start, <10ms input latency, 120fps scroll, <500MB memory
**Constraints**: Offline-capable, GPU-accelerated rendering, no CLI git dependency

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Requirement | Plan Compliance | Status |
|-----------|-------------|-----------------|--------|
| I. Performance-First | Prefetching on j/k navigation | Phase 2 implements prefetch when PR selected | ✅ PASS |
| I. Performance-First | GPU rendering via GPUI | All UI implemented as GPUI Views | ✅ PASS |
| I. Performance-First | git2-rs for Git ops | Phase 1 uses git2-rs, no CLI git | ✅ PASS |
| I. Performance-First | tree-sitter for syntax | Diff view uses tree-sitter highlighting | ✅ PASS |
| I. Performance-First | UniformList for >100 items | Inbox uses UniformList component | ✅ PASS |
| II. Offline-First | SQLite persistence | Phase 3 implements SQLite for all data | ✅ PASS |
| II. Offline-First | pending_comments queue | Phase 3 implements comment queue with sync_status | ✅ PASS |
| II. Offline-First | Idempotent sync | Sync service uses conflict-aware merging | ✅ PASS |
| III. Memory Safety | No unsafe without justification | No unsafe blocks planned | ✅ PASS |
| III. Memory Safety | Newtypes for IDs | PrId, RepoId, CommentId defined | ✅ PASS |
| III. Memory Safety | Result<T,E> error handling | Domain error types defined | ✅ PASS |
| IV. GPUI Architecture | All UI as Views | Workspace, InboxView, ReviewView, DiffView | ✅ PASS |
| IV. GPUI Architecture | Centralized AppState | Global state with Actions/Commands | ✅ PASS |
| IV. GPUI Architecture | Async on Tokio, no blocking | spawn_blocking for CPU work | ✅ PASS |
| IV. GPUI Architecture | Reuse GPUI Editor | DiffView wraps Editor with decorations | ✅ PASS |
| V. Simplicity | Document component evaluation | research.md captures GPUI component analysis | ✅ PASS |
| V. Simplicity | Editor + Highlights for diff | No dual-pane sync implementation | ✅ PASS |

**Gate Status**: ✅ ALL PASSED - Proceed to Phase 0

## Project Structure

### Documentation (this feature)

```text
specs/001-pr-review-mvp/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   └── github-api.md    # GitHub API integration patterns
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
src/
├── main.rs              # Application entry point
├── app.rs               # AppState and GPUI app setup
├── models/
│   ├── mod.rs
│   ├── repository.rs    # Repository entity
│   ├── pull_request.rs  # PullRequest entity
│   ├── comment.rs       # Comment entity
│   ├── review.rs        # Review entity
│   └── ids.rs           # Newtype IDs (PrId, RepoId, etc.)
├── services/
│   ├── mod.rs
│   ├── git.rs           # git2-rs operations
│   ├── github.rs        # GitHub API client
│   ├── sync.rs          # Offline sync service
│   └── db.rs            # SQLite operations via sqlx
├── views/
│   ├── mod.rs
│   ├── workspace.rs     # Root View
│   ├── inbox.rs         # PR list with UniformList
│   ├── review.rs        # PR detail view
│   ├── diff.rs          # Diff rendering with Editor
│   ├── comment_input.rs # Inline comment widget
│   └── command_palette.rs
├── actions/
│   ├── mod.rs
│   ├── navigation.rs    # j/k/n/p/Enter actions
│   ├── review.rs        # r/Cmd+Enter actions
│   └── inbox.rs         # x/e actions
└── lib.rs

tests/
├── fixtures/
│   └── repos/           # Git fixture repositories
├── unit/
│   ├── models/
│   └── services/
└── integration/
    ├── git_tests.rs
    └── sync_tests.rs

migrations/
└── 001_initial_schema.sql
```

**Structure Decision**: Single desktop application structure. All code lives under `src/` with clear separation between models (data), services (business logic), views (UI), and actions (state mutations). Tests use git fixture repositories and in-memory SQLite.

## Development Phases

### Phase 1: "The Reader" (只读版) - Milestone 1

**Goal**: Prove the diff rendering is faster and more readable than `git diff`

**Deliverables**:
- GPUI application skeleton with Workspace view
- Hardcoded local repository path configuration
- git2-rs integration to compute diff between two commits
- DiffView component using GPUI Editor with:
  - Green/red background highlights for additions/deletions
  - Collapsed unchanged sections with expandable hunk headers
  - tree-sitter syntax highlighting

**Acceptance Criteria**:
- Application starts in <500ms
- Diff renders with colors and syntax highlighting
- Scrolling 10,000 lines maintains 60fps+
- No external git CLI calls

### Phase 2: "The Connector" (联网版) - Milestone 2

**Goal**: View real PRs from GitHub

**Deliverables**:
- OAuth2 authentication flow for GitHub
- GitHub API client using reqwest
- InboxView with UniformList rendering PR cards
- PR selection triggers:
  - Background shallow clone if needed
  - Diff prefetch on j/k navigation
- Local SQLite database for PR metadata caching
- Offline mode: cached PRs accessible without network

**Acceptance Criteria**:
- OAuth2 flow completes successfully
- All PRs where user is reviewer/mentioned appear
- PR list shows: repo, title, avatar, CI status, timestamp
- Selected PR loads diff in <2 seconds (warm cache)
- Offline: previously viewed PRs accessible

### Phase 3: "The Reviewer" (交互版) - Milestone 3

**Goal**: Complete the review loop - usable for actual work

**Deliverables**:
- Inline comment input widget (triggered by `r` key)
- Comment local storage in SQLite with sync_status
- Comment sync service (queue → GitHub API)
- Review submission (Approve/Request Changes/Comment)
- Keyboard navigation: j/k, n/p, Enter, x, e, Cmd+K

**Acceptance Criteria**:
- Comments added offline queue with "pending" status
- Comments sync within 60s of connectivity
- Review submission posts all pending comments
- Full keyboard workflow functional
- Zero data loss on crash (SQLite durability)

## Complexity Tracking

> No Constitution violations requiring justification.

| Design Decision | Justification |
|----------------|---------------|
| Single Editor for Unified diff | Constitution V mandates reusing GPUI Editor; split view would require synchronized scroll which adds complexity |
| SQLite over file-based cache | Constitution II requires offline-first with conflict tracking; SQLite provides transactions and sync_status queries |
| Shallow clone only | Constitution I performance requirement; full clone of large repos would violate <500MB memory constraint |
