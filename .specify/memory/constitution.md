<!--
Sync Impact Report - Constitution v1.0.0
=========================================

VERSION CHANGE: N/A (new) → 1.0.0
RATIFICATION DATE: 2025-12-13
LAST AMENDED: 2025-12-13

MODIFIED PRINCIPLES:
- NEW: I. Team Roles & Responsibilities (from user input section 1)
- NEW: II. Project Structure & Code Standards (consolidated from sections 2-3)
- NEW: III. IPC Interaction & Security (consolidated from sections 4-5)
- NEW: IV. Testing & CI/CD (consolidated from sections 6-7)
- NEW: V. Documentation, Performance & Accountability (consolidated from sections 8-10)

ADDED SECTIONS:
- Security & Performance Standards (detailed tech stack, constraints, security model)
- Development Workflow & Quality Gates (branching, code review, testing, build pipeline)
- Governance (constitution supremacy, amendment process, compliance, enforcement)

REMOVED SECTIONS: None (new document)

TEMPLATES REQUIRING UPDATES:
✅ .specify/templates/plan-template.md - Constitution Check section already uses placeholder, compatible
✅ .specify/templates/spec-template.md - No changes needed, structure compatible
✅ .specify/templates/tasks-template.md - No changes needed, structure compatible

FILES FLAGGED FOR MANUAL FOLLOW-UP:
- README.md - Current README is basic Tauri template, consider adding constitution reference
- .specify/templates/commands/*.md - No command templates found to update

NOTES:
- Consolidated 10 sections from user input into 5 Core Principles + 2 additional sections
- All placeholder tokens [BRACKETED] successfully replaced
- Constitution follows Tauri + React best practices from 2025 community guidelines
- Enforcement penalties clearly defined (warning → coffee → supervision)
- Version bump rationale: Initial creation (v1.0.0)
--># HyperReview Constitution
<!-- Tauri + React Development Team Constitution (2025 Edition) -->

## Core Principles

### I. Team Roles & Responsibilities
Strict role separation prevents "dual-stack chaos" and eliminates互怼 between frontend and backend teams. Frontend Lead (React/TS expert) owns UI, state management, and IPC invocation patterns. Rust Backend Lead (Rust expert) owns all commands, security, git2/libgit2 integration, and core business logic. Full-stack Engineers (2-3 people) focus on frontend with Rust invoke knowledge. DevOps/Build Engineer handles CI/CD, tauri.conf.json, and cross-platform packaging. Tech Lead/PM reviews IPC interface design and blocks security vulnerabilities. Clear boundaries: frontend never touches Rust backend code; backend never modifies React components. This separation is non-negotiable to prevent the "mutual blame" anti-pattern that kills Tauri projects.

### II. Project Structure & Code Standards
Strict adherence to bulletproof-react + Tauri official structure: `src/` contains components/, hooks/, store/, services/ with Zustand for state management. `src-tauri/src/` contains commands.rs, models.rs, lib.rs with clear separation. `src-tauri/capabilities/` holds all permission files. All business logic and computationally expensive operations MUST remain in Rust—never in frontend. Code standards: Frontend uses ESLint 9 flat config + Prettier + TypeScript strict mode + Tailwind class sorting. Rust uses rustfmt + clippy with `cargo clippy -- -D warnings`. Conventional Commits required (feat:/fix:/chore:). Pre-commit hooks with husky + lint-staged automatically run ESLint/Prettier/clippy. Unified standards prevent PR review hell—HyperReview is a review tool, our own code must set the standard.

### III. IPC Interaction & Security
All sensitive operations (git diff, file I/O, security checks) MUST execute in Rust commands. Frontend exclusively invokes via Tauri's invoke API and never contains secrets. IPC interface definitions require dual review: Rust Lead + Frontend Lead must approve all interface changes. Use Result<T, String> for error returns—Frontend统一 toast error handling. Security standards: tauri.conf.json allowlist must be minimally scoped. All user input sanitized in Rust. Frontend prohibited from direct filesystem/network access. Run `cargo deny` weekly to check dependency vulnerabilities. IPC is the core of Tauri's security model—writing it poorly creates both security holes and performance bottlenecks.

### IV. Testing & CI/CD
Frontend: Jest + React Testing Library required with 80% minimum coverage for UI components. Rust: `cargo test` with 100% coverage for all commands. E2E: Playwright + Tauri API testing. CI pipeline must pass all tests before merge—no exceptions. CI/CD workflow: GitHub Actions run lint → test → build across all platforms. `npm run check:all` runs lint + format + test + clippy. Direct push to main forbidden—all changes via PR with minimum 1 approval. Versioning: Semantic versioning (SemVer) + changeset. Cross-platform builds are the most failure-prone part of Tauri development—lack of automation means Windows users will complain first.

### V. Documentation, Performance & Accountability
Documentation standards: README must contain startup guide, IPC interface catalog, and common pitfalls. Discord/Slack channels: #frontend, #rust, #review. Weekly meetings: IPC interface review + blocker breakdown. PR templates require description + impact + test cases. Performance red lines: Diffs >5000 lines require virtual scrolling (TanStack Virtual). Rust command response time <200ms. Bundle size <15MB (Windows). Run `tauri size` weekly. HyperReview promises "zero latency"—our own sluggishness would be hypocrisy. Violation penalties: First offense = warning + review 5 PRs. Second = code rollback + buy coffee for team. Third = Tech Lead personal review. Strict discipline creates exceptional products—HyperReview makes reviewers powerful, our team must be even more powerful.

## Security & Performance Standards

**Technology Stack**: Tauri 2.x + React 18+ + TypeScript 5.x + Rust 1.75+.
**State Management**: Zustand for frontend, native Rust structs for backend.
**Build System**: Vite for frontend, Cargo for Rust, tauri-cli for packaging.
**Target Platforms**: Windows 10+, macOS 11+, Linux (Ubuntu 20.04+).
**Performance Constraints**: <15MB Windows bundle, <200ms Rust command response, virtual scrolling for >5000 line diffs, 60fps UI interactions.
**Security Model**: Rust handles all privileged operations, frontend invokes via IPC only, tauri.conf.json allowlist minimally scoped, all user input sanitized in Rust, cargo deny runs weekly for vulnerability scanning.
**Compliance**: All security-relevant changes require Tech Lead approval. Performance regression testing mandatory before each release.

## Development Workflow & Quality Gates

**Branching Strategy**: Feature branches (feature/name), PRs require minimum 1 approval, direct main pushes forbidden.
**Code Review**: All PRs must verify constitution compliance, complexity must be justified with documented alternatives, ESLint/Prettier/clippy must pass, test coverage requirements enforced.
**Testing Gates**: Frontend 80% UI coverage, Rust 100% command coverage, E2E tests required for critical paths, CI must pass all tests before merge.
**Build Pipeline**: GitHub Actions execute lint → test → build across Windows/macOS/Linux, automated versioning with changeset, release artifacts published automatically.
**IPC Interface Changes**: Dual approval required (Rust Lead + Frontend Lead), interface versioning documented, backward compatibility assessed, migration guide required for breaking changes.
**Quality Gates**: Constitution compliance check in every PR template, performance regression testing before releases, security review for all IPC changes, documentation must accompany new features.
**Team Coordination**: Weekly IPC interface review meetings, #frontend/#rust/#review Discord channels for async communication, PR descriptions must include impact analysis and test scenarios.

## Governance

**Constitution Supremacy**: This constitution supersedes all other development practices. Any practice conflicting with these principles MUST be halted and resolved.
**Amendment Process**: Constitution amendments require: (1) Proposed changes documented with rationale, (2) Impact analysis on existing workflows, (3) Team review period (1 week minimum), (4) Tech Lead final approval, (5) Version bump according to semantic rules, (6) Migration plan for any required changes.
**Versioning Policy**: Constitution follows semantic versioning (MAJOR.MINOR.PATCH). MAJOR: Backward-incompatible governance changes or principle removal. MINOR: New principles added or materially expanded guidance. PATCH: Clarifications, wording, non-semantic refinements. Version displayed in header.
**Compliance Review**: Every PR must include constitution compliance checklist. PR reviewers must verify adherence to relevant principles. Tech Lead reviews all security-relevant and IPC changes. Quarterly constitution effectiveness review conducted by team.
**Enforcement**: Violations tracked per team member. First offense: warning + mandatory review of 5 team PRs. Second offense: code rollback + team coffee purchase. Third offense: Tech Lead personal supervision required. Severe violations (security breaches) result in immediate suspension of merge privileges until remediation.

**Version**: 1.0.0 | **Ratified**: 2025-12-13 | **Last Amended**: 2025-12-13
