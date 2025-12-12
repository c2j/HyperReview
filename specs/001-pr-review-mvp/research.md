# Research: HyperReview MVP

**Feature**: 001-pr-review-mvp
**Date**: 2025-11-23
**Status**: Complete

## Research Questions

### RQ-1: GPUI Component Architecture for Diff Rendering

**Question**: How should we implement diff rendering using GPUI's Editor component with Text Highlights and Block Decorations?

**Decision**: Use GPUI's Editor in read-only mode with:
- `TextHighlight` API for line-level background colors (green/red for add/delete)
- `BlockDecoration` for expandable hunk headers between changed sections
- `UniformList` for virtual scrolling of large diffs

**Rationale**:
- Constitution V mandates reusing GPUI Editor; custom text rendering is prohibited
- Editor already handles text layout, selection, and GPU-accelerated rendering
- Highlights API provides per-range background styling without modifying content
- Block decorations allow inserting UI elements (expand buttons) between lines

**Alternatives Considered**:
1. **Dual-pane synchronized Editor** - Rejected: Constitution V prohibits "parallel diff viewer with two synchronized text areas"
2. **Custom GPU text rendering** - Rejected: Constitution IV prohibits "implementing custom text rendering"
3. **HTML/WebView rendering** - Rejected: Constitution requires native GPU rendering

**Implementation Notes**:
- Create `DiffEditor` wrapper that extends Editor with diff-specific state
- Pre-compute highlight ranges from git2-rs diff output
- Use Editor's existing text buffer with annotations for add/delete classification

---

### RQ-2: git2-rs Integration Patterns

**Question**: How should we integrate git2-rs for diff computation and repository operations?

**Decision**: Create a `GitService` that:
- Opens repositories with `Repository::open()` or `Repository::clone()` for shallow clones
- Computes diffs using `Diff::tree_to_tree()` between base and head commits
- Runs on `tokio::task::spawn_blocking` to avoid blocking GPUI event loop
- Caches repository handles per local path

**Rationale**:
- Constitution I mandates git2-rs over CLI git for performance
- Constitution IV requires async operations on Tokio runtime
- Shallow clones minimize disk/memory usage per Constitution performance constraints

**Alternatives Considered**:
1. **CLI git subprocess** - Rejected: Constitution I prohibits "shelling out to git CLI"
2. **GitHub API diffs** - Rejected: Spec FR-011 requires "actual git objects (not API-provided diffs)"
3. **Full repository clones** - Rejected: Would violate <500MB memory constraint

**Implementation Notes**:
```rust
// Shallow clone for PR review
let mut builder = RepoBuilder::new();
builder.fetch_options(fetch_opts_with_depth(1));
builder.clone(url, &local_path)?;

// Diff computation
let base_tree = repo.find_commit(base_oid)?.tree()?;
let head_tree = repo.find_commit(head_oid)?.tree()?;
let diff = repo.diff_tree_to_tree(Some(&base_tree), Some(&head_tree), None)?;
```

---

### RQ-3: GitHub OAuth2 Flow for Desktop Application

**Question**: How should we implement OAuth2 authentication for a native desktop app?

**Decision**: Use GitHub's Device Flow (OAuth 2.0 Device Authorization Grant):
1. Request device code from `https://github.com/login/device/code`
2. Display user code and prompt user to visit `https://github.com/login/device`
3. Poll `https://github.com/login/oauth/access_token` until user completes
4. Store token securely using platform keychain (keyring-rs)

**Rationale**:
- Device flow is GitHub's recommended approach for CLIs and desktop apps
- No need to embed client secrets in binary
- Works without localhost callback server

**Alternatives Considered**:
1. **Authorization Code Flow with localhost** - Rejected: Requires running HTTP server, firewall issues
2. **Personal Access Tokens** - Rejected: Poor UX, requires manual token management
3. **GitHub App installation** - Rejected: Over-engineered for single-user desktop app

**Implementation Notes**:
- Use reqwest for HTTP requests
- Store refresh tokens in system keychain via keyring-rs crate
- Token refresh handled transparently by GitHub client wrapper

---

### RQ-4: SQLite Schema for Offline-First Storage

**Question**: How should we structure SQLite tables for offline-first PR review?

**Decision**: Schema following Constitution II requirements:

```sql
-- Account/Auth
CREATE TABLE accounts (
    id TEXT PRIMARY KEY,  -- provider:username
    provider TEXT NOT NULL,  -- 'github' | 'gitlab'
    username TEXT NOT NULL,
    avatar_url TEXT,
    access_token_encrypted BLOB,
    refresh_token_encrypted BLOB,
    token_expires_at DATETIME
);

-- Repositories
CREATE TABLE repositories (
    id TEXT PRIMARY KEY,  -- provider:owner/name
    provider TEXT NOT NULL,
    owner TEXT NOT NULL,
    name TEXT NOT NULL,
    local_clone_path TEXT,
    last_synced_at DATETIME
);

-- Pull Requests
CREATE TABLE pull_requests (
    id TEXT PRIMARY KEY,  -- provider:owner/repo#number
    repo_id TEXT NOT NULL REFERENCES repositories(id),
    number INTEGER NOT NULL,
    title TEXT NOT NULL,
    author_username TEXT NOT NULL,
    author_avatar_url TEXT,
    status TEXT NOT NULL,  -- 'open' | 'closed' | 'merged'
    head_sha TEXT NOT NULL,
    base_sha TEXT NOT NULL,
    ci_status TEXT,  -- 'pending' | 'success' | 'failure'
    body_markdown TEXT,
    is_read BOOLEAN DEFAULT 0,
    is_archived BOOLEAN DEFAULT 0,
    updated_at DATETIME NOT NULL,
    created_at DATETIME NOT NULL,
    last_synced_at DATETIME
);

-- Comments (including pending offline)
CREATE TABLE comments (
    id TEXT PRIMARY KEY,  -- local UUID or provider ID
    pr_id TEXT NOT NULL REFERENCES pull_requests(id),
    remote_id TEXT,  -- NULL if not yet synced
    file_path TEXT,
    line_number INTEGER,
    content TEXT NOT NULL,
    author_username TEXT NOT NULL,
    sync_status TEXT NOT NULL DEFAULT 'synced',  -- 'pending' | 'synced' | 'failed'
    sync_error TEXT,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);

-- Reviews
CREATE TABLE reviews (
    id TEXT PRIMARY KEY,
    pr_id TEXT NOT NULL REFERENCES pull_requests(id),
    remote_id TEXT,
    decision TEXT NOT NULL,  -- 'approve' | 'request_changes' | 'comment'
    body TEXT,
    sync_status TEXT NOT NULL DEFAULT 'synced',
    submitted_at DATETIME
);

-- Indexes for common queries
CREATE INDEX idx_prs_repo ON pull_requests(repo_id);
CREATE INDEX idx_prs_status ON pull_requests(status);
CREATE INDEX idx_comments_pr ON comments(pr_id);
CREATE INDEX idx_comments_sync ON comments(sync_status);
```

**Rationale**:
- Constitution II requires SQLite + sqlx
- sync_status column enables pending comment queue
- Encrypted token storage for security
- Indexes on sync_status for efficient "pending items" queries

**Alternatives Considered**:
1. **File-based JSON cache** - Rejected: No transactions, no query capability for sync status
2. **Embedded key-value store** - Rejected: Constitution mandates SQLite

---

### RQ-5: tree-sitter Integration for Syntax Highlighting

**Question**: How should we integrate tree-sitter with GPUI Editor for syntax highlighting?

**Decision**: Use tree-sitter with language grammars loaded at runtime:
- Bundle common grammars (Rust, TypeScript, Python, Go, Java, etc.)
- Parse file content incrementally on background thread
- Map syntax nodes to highlight ranges for Editor

**Rationale**:
- Constitution I mandates tree-sitter with incremental parsing
- Incremental parsing avoids re-parsing entire file on every change (read-only diffs don't change, but future edit support benefits)
- Semantic highlighting is more accurate than regex-based

**Alternatives Considered**:
1. **Regex-based highlighting** - Rejected: Constitution explicitly prohibits (less accurate, slower)
2. **LSP-based highlighting** - Rejected: Requires external server, not offline-friendly

**Implementation Notes**:
- Use tree-sitter crate with pre-compiled grammar .so files
- Language detection from file extension
- Highlight query files from tree-sitter-highlight crate

---

### RQ-6: Keyboard Shortcut System

**Question**: How should we implement the keyboard-driven workflow?

**Decision**: Use GPUI's built-in action system:
- Define Actions as unit structs (e.g., `struct MoveDown;`)
- Bind keys to actions in keymap configuration
- Actions dispatch through GPUI's event system to appropriate View handlers

**Rationale**:
- Constitution IV requires state mutations via Actions/Commands
- GPUI's action system provides centralized key binding and dispatch
- Follows Zed's established patterns

**Key Bindings**:
```
j         -> MoveDown
k         -> MoveUp
Enter     -> OpenSelected
n         -> NextHunk
p         -> PrevHunk
r         -> StartComment
Cmd+Enter -> SubmitComment
Cmd+K     -> OpenCommandPalette
x         -> ToggleSelect
e         -> Archive
```

---

## Dependency Evaluation

| Dependency | Purpose | Maintenance Status | Constitution Compliance |
|------------|---------|-------------------|------------------------|
| gpui | UI framework | Active (Zed) | Constitutional requirement |
| tokio | Async runtime | Active (tokio-rs) | Constitutional requirement |
| git2 | Git operations | Active | Constitutional requirement |
| sqlx | Database | Active | Constitutional requirement |
| tree-sitter | Syntax parsing | Active | Constitutional requirement |
| reqwest | HTTP client | Active (seanmonstar) | Required for GitHub API |
| keyring | Secure storage | Active | Recommended for token storage |
| pulldown-cmark | Markdown | Active | Constitutional (recommended) |
| serde + serde_json | Serialization | Active (rust-lang) | Standard dependency |
| thiserror | Error types | Active | Standard for domain errors |
| uuid | ID generation | Active | Standard for local IDs |
| chrono | Date/time | Active | Standard for timestamps |

All dependencies are pure Rust or have approved C bindings (git2, tree-sitter).

---

## Open Questions (Resolved)

All research questions have been resolved. No NEEDS CLARIFICATION items remain.
