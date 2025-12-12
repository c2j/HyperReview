<!--
=== SYNC IMPACT REPORT ===
Version Change: N/A (new) → 1.0.0
Bump Rationale: MAJOR - Initial constitution ratification

Modified Principles: N/A (initial creation)

Added Sections:
- Core Principles (5 principles)
  - I. Performance-First (零延迟)
  - II. Offline-First (离线优先)
  - III. Memory Safety & Type Safety
  - IV. GPUI Component Architecture
  - V. Simplicity & Strategic Reuse
- Technology Stack Constraints
- Development Workflow Standards
- Governance

Removed Sections: N/A

Templates Requiring Updates:
- .specify/templates/plan-template.md: ✅ Compatible (Constitution Check section exists)
- .specify/templates/spec-template.md: ✅ Compatible (No constitution-specific sections)
- .specify/templates/tasks-template.md: ✅ Compatible (Phase structure aligns)

Follow-up TODOs: None
===========================
-->

# HyperReview Constitution

## Core Principles

### I. Performance-First (零延迟)

Every user interaction MUST feel instantaneous. The application targets sub-100ms response
times for all UI operations.

**Non-Negotiable Rules:**
- All diff operations MUST use prefetching: when a PR is selected (j/k navigation), the
  background thread MUST immediately fetch required git objects (head_sha, base_sha) and
  pre-compute the diff
- UI rendering MUST leverage GPU acceleration via GPUI; CPU-bound rendering is prohibited
  for visible elements
- Git operations MUST use `git2-rs` for in-memory object manipulation; shelling out to
  git CLI is prohibited except for operations not supported by libgit2
- Syntax highlighting MUST use `tree-sitter` with incremental parsing; full re-parse on
  every keystroke is prohibited
- Lists MUST use virtual rendering (UniformList) for datasets exceeding 100 items

**Rationale:** A code review tool that feels slow will be abandoned. Users will revert to
web interfaces if the native app doesn't deliver tangible performance benefits.

### II. Offline-First (离线优先)

The application MUST remain fully functional without network connectivity. All data
required for review operations MUST exist locally.

**Non-Negotiable Rules:**
- All PR metadata, comments, and diffs MUST be persisted to local SQLite database
- User-generated content (comments, approvals) MUST be queued in `pending_comments` table
  with `sync_status` tracking
- Sync operations MUST be idempotent and conflict-aware; failed syncs MUST NOT corrupt
  local state
- The application MUST clearly indicate sync status (pending/synced/failed) in the UI
- Network failures MUST NOT block user workflows; degraded mode MUST be seamless

**Rationale:** Developers often work in environments with unreliable connectivity (flights,
trains, coffee shops). The app must be a reliable companion regardless of network state.

### III. Memory Safety & Type Safety

Rust's safety guarantees MUST be preserved throughout the codebase. Undefined behavior is
unacceptable.

**Non-Negotiable Rules:**
- `unsafe` blocks are prohibited unless accompanied by:
  - A safety comment explaining invariants
  - Approval in code review with documented justification
  - Wrapper function that presents a safe API
- All public APIs MUST use strong typing; `String` for semantic identifiers is prohibited
  (use newtypes: `PrId(String)`, `RepoId(String)`)
- Error handling MUST use `Result<T, E>` with domain-specific error types; `unwrap()` and
  `expect()` are prohibited in library code (allowed only in tests and binary entry points
  with explicit panic context)
- All async operations MUST properly handle cancellation via Tokio's CancellationToken or
  equivalent patterns

**Rationale:** Rust was chosen for memory safety. Undermining this with careless unsafe
usage or panic-prone code defeats the purpose of the language choice.

### IV. GPUI Component Architecture

UI development MUST follow GPUI's View-based architecture with centralized state management.

**Non-Negotiable Rules:**
- All UI elements MUST be implemented as GPUI Views
- Application state MUST be centralized in `AppState` struct; component-local state is
  permitted only for transient UI state (hover, focus)
- State mutations MUST flow through Actions/Commands; direct state modification from
  event handlers is prohibited
- Backend operations (Git, Sync, DB) MUST execute on Tokio async runtime; blocking the
  GPUI event loop is prohibited
- The Editor component MUST be reused from GPUI; implementing custom text rendering is
  prohibited unless GPUI Editor is provably insufficient

**Rationale:** GPUI's architecture enables GPU-accelerated, 60fps UI. Breaking the
architecture (blocking main thread, bypassing state management) destroys these benefits.

### V. Simplicity & Strategic Reuse

Features MUST be implemented with minimal complexity. Existing solutions MUST be leveraged
before building custom implementations.

**Non-Negotiable Rules:**
- Before implementing any component, developers MUST document what existing GPUI/ecosystem
  components were evaluated and why they were insufficient
- Diff rendering MUST use GPUI Editor with Text Highlights API and Block Decorations;
  implementing a parallel diff viewer with two synchronized text areas is prohibited
- Database schema additions MUST be justified by concrete user stories; speculative
  schema design is prohibited
- Each new dependency MUST be justified in PR description with: purpose, maintenance
  status, security audit status

**Rationale:** Complexity is the enemy of reliability and performance. Every custom
implementation is a maintenance burden and potential bug source.

## Technology Stack Constraints

The following technology choices are constitutional and MUST NOT be changed without
formal amendment process:

| Component | Selection | Constraint Level |
|-----------|-----------|------------------|
| Language | Rust (Edition 2024) | Constitutional - cannot change |
| UI Framework | GPUI | Constitutional - cannot change |
| Async Runtime | Tokio | Constitutional - cannot change |
| Git Operations | git2-rs | Constitutional - cannot change |
| Local Database | SQLite + sqlx | Constitutional - cannot change |
| Syntax Highlighting | tree-sitter | Constitutional - cannot change |
| Markdown Rendering | pulldown-cmark | Recommended - can change with justification |

**Adding New Dependencies:**
- MUST be pure Rust (no C bindings without explicit approval)
- MUST have active maintenance (commit within last 6 months)
- MUST NOT duplicate functionality of existing dependencies
- Security-sensitive dependencies MUST have undergone audit or be from trusted sources
  (rust-lang, tokio-rs, etc.)

## Development Workflow Standards

### Async Patterns

- All I/O operations MUST be async
- CPU-intensive operations (diff computation, syntax parsing) MUST be spawned on
  `tokio::task::spawn_blocking` or dedicated thread pool
- Async tasks MUST communicate state changes to UI via GPUI's notification system
- Long-running operations MUST be cancellable

### Error Handling

- All errors MUST be categorized: Recoverable (show user message, allow retry) vs
  Fatal (log, graceful shutdown)
- User-facing error messages MUST be actionable ("Failed to fetch PR. Check network
  connection and retry." not "Error: network")
- All errors MUST be logged with context (operation, relevant IDs, timestamp)

### Testing Requirements

- All service modules MUST have unit tests
- Git operations MUST be tested against fixture repositories
- Database operations MUST be tested with in-memory SQLite
- UI components SHOULD have snapshot tests where GPUI supports them

## Governance

This constitution supersedes all other development practices for the HyperReview project.
Compliance is mandatory.

**Amendment Process:**
1. Propose amendment via GitHub Issue with [CONSTITUTION] prefix
2. Amendment MUST include: rationale, impact analysis, migration plan
3. Amendment requires explicit approval from project maintainers
4. All affected code MUST be migrated within one release cycle of amendment ratification

**Compliance Verification:**
- All PRs MUST include a Constitution Compliance statement confirming relevant
  principles were followed
- Code reviews MUST verify principle adherence before approval
- Violations discovered post-merge MUST be addressed in subsequent PR with priority

**Version**: 1.0.0 | **Ratified**: 2025-11-23 | **Last Amended**: 2025-11-23
