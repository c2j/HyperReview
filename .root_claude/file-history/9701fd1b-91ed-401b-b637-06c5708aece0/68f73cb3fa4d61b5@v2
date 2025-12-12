# Quickstart: HyperReview MVP

**Feature**: 001-pr-review-mvp
**Date**: 2025-11-23

## Prerequisites

### System Requirements

- **OS**: macOS 12+, Windows 10+, or Linux (X11/Wayland)
- **GPU**: Metal, Vulkan, or DirectX 12 compatible
- **Memory**: 4GB RAM minimum, 8GB recommended
- **Disk**: 500MB for application, additional space for repository clones

### Development Tools

- **Rust**: 1.75+ (Edition 2024)
- **Cargo**: Latest stable
- **SQLite**: 3.35+ (usually bundled with OS)
- **Git**: 2.30+ (for fixture repos during development)

### Installation

```bash
# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Clone repository
git clone https://github.com/hyperreview/hyperreview.git
cd hyperreview

# Install dependencies (handled by Cargo)
cargo build --release
```

---

## Quick Verification

### Phase 1: "The Reader"

After completing Phase 1, verify with a local repository:

```bash
# Run with a local repo path (hardcoded in Phase 1)
cargo run --release

# Expected behavior:
# 1. Window opens in <500ms
# 2. Diff between last two commits displayed
# 3. Syntax highlighting visible
# 4. Scrolling is smooth (120fps)
```

**Test Diff**:
```bash
# Create a test repo
mkdir /tmp/test-repo && cd /tmp/test-repo
git init
echo "hello" > file.rs
git add . && git commit -m "initial"
echo "hello world" > file.rs
git add . && git commit -m "update"

# Point HyperReview at /tmp/test-repo (update config)
```

### Phase 2: "The Connector"

After completing Phase 2, verify GitHub integration:

```bash
cargo run --release

# Expected flow:
# 1. Window opens
# 2. Click "Connect GitHub" → Device flow starts
# 3. Enter code at github.com/login/device
# 4. PR inbox populates with your review requests
# 5. Select PR → diff loads
# 6. Disconnect network → cached PRs still viewable
```

### Phase 3: "The Reviewer"

After completing Phase 3, verify full workflow:

```bash
cargo run --release

# Expected flow:
# 1. Navigate inbox with j/k
# 2. Press Enter on PR → diff view
# 3. Navigate hunks with n/p
# 4. Press r on line → comment input appears
# 5. Type comment, press Cmd+Enter → comment saved
# 6. Submit review with Approve/Request Changes
# 7. Disconnect network → add comment → "pending" badge
# 8. Reconnect → comment syncs → "synced" badge
```

---

## Configuration

### Application Config

Location: `~/.config/hyperreview/config.toml` (Linux/macOS) or `%APPDATA%\hyperreview\config.toml` (Windows)

```toml
# Default configuration
[appearance]
theme = "dark"  # "dark" | "light"
font_size = 14
font_family = "JetBrains Mono"

[performance]
prefetch_on_select = true
max_cached_repos = 10
shallow_clone_depth = 1

[keyboard]
# Override default keybindings
# move_down = "j"
# move_up = "k"
# open = "enter"
```

### GitHub OAuth App

For development, register an OAuth App at https://github.com/settings/developers:

1. Create new OAuth App
2. Set Authorization callback URL to: `http://localhost:7890/callback` (unused with device flow, but required)
3. Copy Client ID to environment or config
4. **Do NOT embed Client Secret** (device flow doesn't need it)

```bash
export HYPERREVIEW_GITHUB_CLIENT_ID="your_client_id"
```

---

## Database Location

SQLite database stored at:
- **macOS**: `~/Library/Application Support/hyperreview/hyperreview.db`
- **Linux**: `~/.local/share/hyperreview/hyperreview.db`
- **Windows**: `%APPDATA%\hyperreview\hyperreview.db`

### Reset Database

```bash
# Delete database to reset (loses all cached data and pending comments!)
rm ~/Library/Application\ Support/hyperreview/hyperreview.db

# Or use CLI command (when implemented)
hyperreview --reset-db
```

---

## Running Tests

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run specific test module
cargo test services::git

# Run integration tests only
cargo test --test '*'

# Run with in-memory SQLite (default for tests)
DATABASE_URL=":memory:" cargo test
```

### Test Fixtures

Git fixture repositories are in `tests/fixtures/repos/`:

```bash
# Create fixture repo (done once)
./tests/fixtures/create_fixtures.sh

# Fixtures include:
# - simple-repo/       # Basic commits for diff testing
# - large-repo/        # 10,000+ line files for performance
# - rename-repo/       # File renames for detection testing
# - binary-repo/       # Binary file handling
```

---

## Keyboard Shortcuts Reference

| Key | Action | Context |
|-----|--------|---------|
| `j` | Move down | Inbox, Diff |
| `k` | Move up | Inbox, Diff |
| `Enter` | Open selected | Inbox |
| `n` | Next hunk | Diff |
| `p` | Previous hunk | Diff |
| `r` | Start comment | Diff (on line) |
| `Cmd+Enter` | Submit comment/review | Comment input |
| `Cmd+K` | Open command palette | Global |
| `x` | Toggle selection | Inbox |
| `e` | Archive | Inbox (selected) |
| `Esc` | Cancel/Close | Comment input, Palette |
| `?` | Show shortcuts | Global |

---

## Troubleshooting

### "Window is blank"

GPU driver issue. Check:
```bash
# macOS - ensure Metal is available
system_profiler SPDisplaysDataType | grep Metal

# Linux - check Vulkan
vulkaninfo | grep "GPU id"
```

### "OAuth fails with 'authorization_pending'"

User hasn't completed browser flow. Verify:
1. Visit https://github.com/login/device
2. Enter the displayed code
3. Click "Authorize"

### "PR list is empty"

Check inbox filters:
- User must be assigned as reviewer OR mentioned
- PR must have status "Open"
- Try: Cmd+K → "Refresh inbox"

### "Diff loads slowly"

First load requires shallow clone:
- Check network connectivity
- Large repos take longer
- Subsequent views use cache

### "Comments stuck as 'pending'"

Network sync issue:
- Check internet connection
- Look for rate limit message
- Try: Cmd+K → "Retry sync"

### "High memory usage"

Large diffs or many cached repos:
- Reduce `max_cached_repos` in config
- Clear unused repo clones: Cmd+K → "Clear cache"

---

## Development Commands

```bash
# Run in debug mode with logging
RUST_LOG=debug cargo run

# Run with specific log level
RUST_LOG=hyperreview::services::git=trace cargo run

# Profile with flamegraph
cargo flamegraph --release

# Check for unsafe code
cargo +nightly udeps

# Lint
cargo clippy --all-targets
```

---

## Success Criteria Verification

| Criterion | How to Verify |
|-----------|---------------|
| SC-001: Cold start <500ms | `time cargo run --release` (measure window appearance) |
| SC-002: Memory <500MB with 10k lines | Activity Monitor / htop while viewing large diff |
| SC-003: Input latency <10ms | Qualitative - keystrokes feel instant |
| SC-004: 120fps scrolling | GPU profiler or qualitative smoothness |
| SC-005: Launch to diff <30s | Stopwatch from double-click to viewing diff |
| SC-006: Comment in <5s | Stopwatch r → Cmd+Enter |
| SC-008: Offline access | Disconnect network, view cached PRs |
| SC-011: Zero data loss | Force quit during pending comment, relaunch |
