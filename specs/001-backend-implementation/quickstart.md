# HyperReview Backend Implementation Quickstart

**Date**: 2025-12-13
**Version**: 1.0.0
**Purpose**: Developer guide for implementing and testing HyperReview backend

---

## Overview

This guide helps developers quickly set up, implement, and test the HyperReview backend based on the implementation plan, design documents, and API contracts.

**Prerequisites**:
- Rust 1.75+ installed
- Tauri v2 CLI installed: `cargo install tauri-cli`
- Git repository for testing
- Basic understanding of Git operations

---

## 1. Project Setup

### 1.1 Initialize Tauri Application

```bash
# From project root
npm install
cargo tauri init
```

### 1.2 Add Dependencies

Add to `src-tauri/Cargo.toml`:

```toml
[dependencies]
# Git operations
git2 = "0.18"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Code analysis
tree-sitter = "0.20"
tree-sitter-java = "0.20"  # Add language grammars as needed
tree-sitter-javascript = "0.20"
tree-sitter-rust = "0.20"
tree-sitter-xml = "0.20"
tree-sitter-sql = "0.20"

# Storage
rusqlite = { version = "0.32", features = ["bundled"] }

# HTTP client
reqwest = { version = "0.11", features = ["json"] }

# Concurrency
rayon = "1.8"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Async runtime
tokio = { version = "1.0", features = ["full"] }

# Validation
validator = { version = "0.18", features = ["derive"] }

# UUID
uuid = { version = "1.0", features = ["v4", "serde"] }

# Date/time
chrono = { version = "0.4", features = ["serde"] }
```

### 1.3 Configure Tauri

Update `src-tauri/tauri.conf.json`:

```json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devPath": "http://localhost:1420",
    "distDir": "../dist",
    "withGlobalTauri": false
  },
  "package": {
    "productName": "HyperReview",
    "version": "1.0.0"
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "shell": {
        "all": false,
        "open": true
      },
      "dialog": {
        "all": false,
        "open": true
      },
      "fs": {
        "all": false,
        "scope": ["**"]
      },
      "protocol": {
        "asset": true,
        "assetScope": ["**"]
      }
    },
    "bundle": {
      "active": true,
      "category": "DeveloperTool",
      "copyright": "",
      "deb": {
        "depends": ["libgtk-3-0", "libwebkit2gtk-4.0-37"]
      },
      "externalBin": [],
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ],
      "identifier": "com.hyperreview.app",
      "longDescription": "Zero-latency code review desktop application",
      "macOS": {
        "entitlements": null,
        "exceptionDomain": "",
        "frameworks": [],
        "providerShortName": null,
        "signingIdentity": null
      },
      "resources": [],
      "shortDescription": "Zero-latency code review",
      "targets": "all",
      "windows": {
        "certificateThumbprint": null,
        "digestAlgorithm": "sha256",
        "timestampUrl": "",
        "wix": {
          "language": ["en-US"],
          "template": "main.wxs"
        }
      }
    },
    "security": {
      "csp": null,
      "dangerousRemoteDomainIpcAccess": []
    },
    "updater": {
      "active": false
    },
    "windows": [
      {
        "fullscreen": false,
        "resizable": true,
        "title": "HyperReview",
        "width": 1400,
        "height": 900,
        "minWidth": 1000,
        "minHeight": 700
      }
    ]
  }
}
```

---

## 2. Implementation Phases

### 2.1 Phase 1: Core Repository Management (Week 1-2)

**Goal**: Enable opening repositories and basic Git operations

**Tasks**:
1. ✅ Implement `open_repo_dialog` command
2. ✅ Implement `get_recent_repos` command
3. ✅ Implement `get_branches` command
4. ✅ Set up SQLite database with `repos` table
5. ✅ Create repository state management

**Code Structure**:

```rust
// src-tauri/src/lib.rs
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_state = AppState::new();
            app.manage(app_state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open_repo_dialog,
            get_recent_repos,
            get_branches
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

```rust
// src-tauri/src/models.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Repo {
    pub path: String,
    pub branch: String,
    pub last_opened: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub is_current: bool,
    pub is_remote: bool,
    pub upstream: Option<String>,
    pub last_commit: String,
    pub last_commit_message: String,
    pub last_commit_author: String,
    pub last_commit_date: String,
}
```

```rust
// src-tauri/src/git/service.rs
use git2::Repository;
use std::sync::Mutex;

pub struct GitService {
    repository: Mutex<Option<Repository>>,
}

impl GitService {
    pub fn new() -> Self {
        Self {
            repository: Mutex::new(None),
        }
    }

    pub fn open_repo(&self, path: &str) -> Result<Repo, git2::Error> {
        let repo = Repository::open(path)?;
        let head = repo.head()?;
        let branch = head.shorthand().unwrap_or("unknown").to_string();

        // Update state
        *self.repository.lock().unwrap() = Some(repo);

        Ok(Repo {
            path: path.to_string(),
            branch,
            last_opened: chrono::Utc::now()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        })
    }

    pub fn get_branches(&self) -> Result<Vec<Branch>, git2::Error> {
        let repo = self.repository.lock().unwrap();
        let repo = repo.as_ref().ok_or(git2::Error::from_str("No repository loaded"))?;

        let mut branches = Vec::new();

        // Local branches
        repo.branches(Some(git2::BranchType::Local))?.for_each(|branch| {
            let (b, _) = branch.unwrap();
            let name = b.name().unwrap().unwrap_or("unknown").to_string();
            let is_current = repo.head().map_or(false, |h| {
                b.name().map_or(false, |n| h.name().map_or(false, |hn| n == hn))
            }).unwrap_or(false);

            if let Some(commit) = b.get().peel_to_commit().ok() {
                branches.push(Branch {
                    name: name.clone(),
                    is_current,
                    is_remote: false,
                    upstream: b.upstream().ok().and_then(|u| u.get().name().map(|n| n.to_string())),
                    last_commit: commit.id().to_string(),
                    last_commit_message: commit.message().unwrap_or("").to_string(),
                    last_commit_author: commit.author().name().unwrap_or("").to_string(),
                    last_commit_date: commit.author().when().to_rfc3339(),
                });
            }
        });

        Ok(branches)
    }
}
```

**Testing**:

```bash
# Run in development mode
npm run tauri dev

# Test commands
curl -X POST http://localhost:1420/ipc \
  -H "Content-Type: application/json" \
  -d '{"cmd": "get_recent_repos"}'
```

### 2.2 Phase 2: Diff Viewing & Comments (Week 3-4)

**Goal**: Enable diff computation and inline commenting

**Tasks**:
1. Implement `get_file_diff` command
2. Implement `add_comment` command
3. Create SQLite schema for comments
4. Add static analysis integration

**Implementation Notes**:

```rust
#[tauri::command]
pub async fn get_file_diff(file_id: String) -> Result<Vec<DiffLine>, String> {
    let repo = state.repository.lock().unwrap();
    let repo = repo.as_ref().ok_or("No repository loaded")?;

    // Compute diff using git2
    let diff = compute_diff(repo, &file_id)?;

    // Add static analysis
    let analyzed_lines = analyze_diff_lines(&diff)?;

    Ok(analyzed_lines)
}

fn analyze_diff_lines(lines: &[DiffLine]) -> Result<Vec<DiffLine>, String> {
    let mut analyzed = Vec::new();

    for line in lines {
        let mut analyzed_line = line.clone();

        // Check for TODO/FIXME
        if line.content.contains("TODO") || line.content.contains("FIXME") {
            analyzed_line.severity = Some(Severity::Info);
            analyzed_line.message = Some("TODO/FIXME comment found".to_string());
        }

        // Check for potential secrets
        if line.content.contains("password") || line.content.contains("api_key") {
            analyzed_line.severity = Some(Severity::Warning);
            analyzed_line.message = Some("Potential hardcoded credential".to_string());
        }

        analyzed.push(analyzed_line);
    }

    Ok(analyzed)
}
```

### 2.3 Phase 3: Insights & Analysis (Week 5-6)

**Goal**: Implement heatmap, checklists, and complexity analysis

**Tasks**:
1. Implement `get_heatmap` command
2. Implement `get_checklist` command
3. Implement `analyze_complexity` command
4. Add tree-sitter integration

**Implementation Notes**:

```rust
#[tauri::command]
pub async fn get_heatmap() -> Result<Vec<HeatmapItem>, String> {
    let repo = state.repository.lock().unwrap();
    let repo = repo.as_ref().ok_or("No repository loaded")?;

    // Get file modification history
    let mut file_changes = HashMap::new();

    // Use git log to get change frequency
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    for oid in revwalk {
        let commit = repo.find_commit(oid?)?;
        let tree = commit.tree()?;

        // Count changes per file (simplified)
        // In production, use git2::Diff to compare trees

        if let Some(file_path) = /* extract file path */ {
            *file_changes.entry(file_path).or_insert(0) += 1;
        }
    }

    // Calculate complexity using tree-sitter
    let mut heatmap_items = Vec::new();
    for (file_path, change_freq) in file_changes {
        let complexity = calculate_complexity(&file_path)?;
        let impact_score = (change_freq as f32 * 0.6) + (complexity * 0.4);

        heatmap_items.push(HeatmapItem {
            file_path,
            impact_score,
            churn_score: change_freq as f32 / 100.0, // Normalize
            complexity_score: complexity,
            change_frequency: change_freq,
            category: if impact_score > 0.7 {
                "high".to_string()
            } else if impact_score > 0.4 {
                "medium".to_string()
            } else {
                "low".to_string()
            },
        });
    }

    Ok(heatmap_items)
}
```

### 2.4 Phase 4: External Integration (Week 7-8)

**Goal**: Integrate with GitLab, Gerrit, CodeArts

**Tasks**:
1. Implement `submit_review` command
2. Implement credential storage (Keychain)
3. Add API clients for external systems
4. Implement `sync_repo` command

---

## 3. Testing Strategy

### 3.1 Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repo_opening() {
        let git_service = GitService::new();
        let result = git_service.open_repo("/path/to/test/repo");
        assert!(result.is_ok());
    }

    #[test]
    fn test_diff_computation() {
        // Test diff generation
    }

    #[test]
    fn test_static_analysis() {
        // Test analysis rules
    }
}
```

Run tests:
```bash
cd src-tauri
cargo test
```

### 3.2 Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    use tauri::api::invoke;

    #[tokio::test]
    async fn test_open_repo_dialog() {
        // Test Tauri command invocation
    }

    #[tokio::test]
    async fn test_add_comment() {
        // Test comment workflow
    }
}
```

### 3.3 E2E Tests

Use Playwright for end-to-end testing:

```typescript
// tests/e2e/review-workflow.spec.ts
import { test, expect } from '@playwright/test';

test('open repository and view diff', async ({ page }) => {
  await page.goto('http://localhost:1420');

  // Open repository
  await page.click('[data-testid="open-repo-button"]');
  await page.waitForSelector('[data-testid="repo-loaded"]');

  // View diff
  await page.click('[data-testid="file-tree"] >> text="UserService.java"');
  await expect(page.locator('[data-testid="diff-view"]')).toBeVisible();

  // Add comment
  await page.click('[data-testid="diff-line"] >> nth=0');
  await page.fill('[data-testid="comment-input"]', 'Looks good!');
  await page.click('[data-testid="save-comment"]');

  await expect(page.locator('[data-testid="comment"]')).toBeVisible();
});
```

Run E2E tests:
```bash
npx playwright test
```

---

## 4. Performance Optimization

### 4.1 Caching Strategy

```rust
use lru::LruCache;
use std::sync::Mutex;

pub struct CacheManager {
    diff_cache: Mutex<LruCache<String, Vec<DiffLine>>>,
    blame_cache: Mutex<LruCache<String, Vec<BlameLine>>>,
}

impl CacheManager {
    pub fn new() -> Self {
        Self {
            diff_cache: Mutex::new(LruCache::new(100)),
            blame_cache: Mutex::new(LruCache::new(50)),
        }
    }

    pub fn get_diff(&self, key: &str) -> Option<Vec<DiffLine>> {
        self.diff_cache.lock().unwrap().get(key).cloned()
    }

    pub fn put_diff(&self, key: String, value: Vec<DiffLine>) {
        self.diff_cache.lock().unwrap().put(key, value);
    }
}
```

### 4.2 Background Processing

```rust
use tokio::task;

#[tauri::command]
pub async fn load_repo_async(path: String) -> Result<Repo, String> {
    let git_service = state.git_service.clone();

    // Spawn background task for expensive operations
    task::spawn_blocking(move || {
        git_service.open_repo(&path)
    }).await.map_err(|e| e.to_string())?
}
```

---

## 5. Common Issues & Solutions

### 5.1 Git Repository Locked

**Problem**: "Repository is locked"

**Solution**:
```rust
// Ensure repository is properly closed
drop(repo);

// Or use try_lock for non-blocking access
let repo = self.repository.try_lock().map_err(|_| "Repository busy")?;
```

### 5.2 Large File Handling

**Problem**: Out of memory with large diffs

**Solution**:
```rust
#[tauri::command]
pub async fn get_file_diff(file_id: String) -> Result<Vec<DiffLine>, String> {
    // Check file size first
    let metadata = fs::metadata(&file_id)?;
    if metadata.len() > 10_000_000 { // 10MB
        return Err("File too large for diff".to_string());
    }

    // Stream diff for large files
    // ...
}
```

### 5.3 Tree-sitter Grammar Loading

**Problem**: Slow startup due to grammar loading

**Solution**:
```rust
// Lazy load grammars
static LANGUAGE_REGISTRY: once_cell::sync::OnceCell<LanguageRegistry> =
    once_cell::sync::OnceCell::new();

pub fn get_language(lang: &str) -> Option<Language> {
    let registry = LANGUAGE_REGISTRY.get_or_init(|| {
        let mut r = LanguageRegistry::new();
        r.add_language(tree_sitter_javascript::language());
        r.add_language(tree_sitter_java::language());
        // Add more as needed
        r
    });

    registry.language_for_name(lang).ok()
}
```

---

## 6. Debugging

### 6.1 Enable Debug Logging

```rust
// src-tauri/src/main.rs
fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .init();

    tauri::Builder::default()
        // ...
}
```

### 6.2 IPC Inspector

Use Tauri DevTools to inspect IPC calls:

```bash
npm run tauri dev -- --debug
```

### 6.3 Performance Profiling

```rust
use std::time::Instant;

#[tauri::command]
pub async fn get_file_diff(file_id: String) -> Result<Vec<DiffLine>, String> {
    let start = Instant::now();

    let result = compute_diff(&file_id)?;

    let elapsed = start.elapsed();
    log::debug!("get_file_diff took {:?}", elapsed);

    Ok(result)
}
```

---

## 7. Build & Release

### 7.1 Development Build

```bash
npm run tauri dev
```

### 7.2 Production Build

```bash
# Build for current platform
npm run tauri build

# Build for specific platform
cargo tauri build --target x86_64-pc-windows-msvc
cargo tauri build --target x86_64-apple-darwin
cargo tauri build --target x86_64-unknown-linux-gnu
```

### 7.3 Bundle Size Optimization

Add to `src-tauri/Cargo.toml`:

```toml
[profile.release]
lto = true
codegen-units = 1
panic = "abort"
opt-level = "s"
strip = true
```

---

## 8. Security Checklist

- ✅ All user input validated in Rust backend
- ✅ File paths canonicalized and sandboxed
- ✅ No secrets in frontend code
- ✅ Credentials stored in OS Keychain
- ✅ IPC allowlist minimally scoped
- ✅ Result<T, String> error handling
- ✅ SQL injection prevention (parameterized queries)
- ✅ Command injection prevention (no shell execution)

---

## 9. Next Steps

After completing implementation:

1. ✅ Run full test suite (unit + integration + E2E)
2. ✅ Performance testing (100k file repo)
3. ✅ Bundle size verification (<120MB)
4. ✅ Security audit (dependency vulnerabilities)
5. ✅ User acceptance testing
6. ✅ Documentation review
7. ✅ Prepare for release

---

## 10. Resources

**Documentation**:
- [Tauri v2 Book](https://tauri.app/v1/guides/)
- [git2-rs Documentation](https://docs.rs/git2/)
- [tree-sitter Documentation](https://tree-sitter.github.io/tree-sitter/)
- [Rust SQLite](https://docs.rs/rusqlite/)

**Tools**:
- [Tauri CLI](https://tauri.app/v1/guides/building/cross-platform)
- [Playwright](https://playwright.dev/) (E2E testing)
- [cargo-expand](https://github.com/dtolnay/cargo-expand) (debugging)
- [cargo-geiger](https://github.com/geiger-rs/cargo-geiger) (security)

**Community**:
- [Tauri Discord](https://discord.com/invite/tauri)
- [Rust Community Discord](https://discord.gg/rust-lang)

---

**Quickstart Version**: 1.0.0
**Last Updated**: 2025-12-13
**Next Review**: After Phase 1 implementation
