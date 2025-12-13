# HyperReview Backend Design Specification (Rust/Tauri)

## 1. 架构概览

HyperReview 后端采用 **Tauri (Rust)** 架构，旨在提供极致的本地 I/O 性能和零延迟的交互体验。后端核心职责是作为“高性能代理层”，接管 Git 操作、文件系统访问、静态代码分析及与远程服务（CodeArts/GitLab）的同步。

### 1.1 系统分层

```mermaid
graph TD
    A[Frontend (React)] <-->|IPC (Tauri Invoke)| B[Tauri Interface Layer]
    B --> C[Core Service Manager]
    
    subgraph "Rust Core (HyperReview Engine)"
        C --> D[Git Service]
        C --> E[Analysis Engine]
        C --> F[Persistence Layer]
        C --> G[Remote Client]
    end
    
    D -->|git2-rs| H[Local Git Repo]
    E -->|tree-sitter| I[Source Code]
    E -->|grep| I
    F -->|SQLite/Sled| J[Local Meta DB]
    G -->|Reqwest| K[CodeArts/GitLab API]
```

## 2. 技术栈选型

*   **Runtime**: Tauri v2 (Rust)
*   **Git Operations**: `git2-rs` (libgit2 的 Rust 绑定，用于高性能 Git 读取、Diff 计算、Blame)
*   **Code Analysis**: `tree-sitter` (增量式语法解析，用于计算复杂度、提取符号)
*   **Search**: `ripgrep` (集成 `grep` crate，用于毫秒级全局搜索)
*   **Concurrency**: `rayon` (数据并行处理，用于 Diff 计算和批量分析)
*   **Local DB**: `rusqlite` (存储本地评审草稿、任务状态、最近打开记录)
*   **HTTP Client**: `reqwest` (处理 OpenAPI 通信)
*   **Error Handling**: `thiserror` + `anyhow`

## 3. 核心模块设计

### 3.1 状态管理 (Session State)

后端需在内存中维护当前的上下文状态，避免前端每次请求都传递完整路径。

```rust
struct AppState {
    // 当前激活的仓库句柄（线程安全）
    active_repo: Mutex<Option<Repository>>, 
    // 当前上下文信息
    context: Mutex<ReviewContext>,
}

struct ReviewContext {
    base_commit: Oid,
    head_commit: Oid,
    workdir: PathBuf,
}
```

### 3.2 Git 服务模块 (GitService)

负责所有版本控制操作，核心通过 `git2` 实现。

*   **Diff 计算 (`get_file_diff`)**:
    *   **逻辑**: 获取 Base 和 Head 的 Tree，计算 `Diff` 对象。
    *   **增强**: 不仅返回文本差异，需结合 Tree-sitter 识别语义范围（如：变更发生在哪个函数内）。
    *   **性能**: 对于大文件 Diff，使用流式处理或分页返回（当前前端暂不支持分页，需后端限制最大行数）。

*   **Blame (`get_blame`)**:
    *   **逻辑**: 使用 `git2::Blame`。
    *   **缓存**: Blame 计算较慢，需在 LRU Cache 中缓存结果，Key 为 `FileID + CommitID`。

*   **同步 (`sync_repo`)**:
    *   执行 `fetch origin`，检查本地与远程的 commit 差异。
    *   向前端流式发送 stdout/stderr 日志（通过 Tauri Event `sync://progress`）。

### 3.3 静态分析引擎 (AnalysisEngine)

这是 HyperReview 的核心差异化功能。

*   **热力图 (`get_heatmap`)**:
    *   **维度 1 (Churn)**: 通过 `git log` 统计文件在近期（如 30 天）的修改频率。
    *   **维度 2 (Complexity)**: 使用 `tree-sitter` 解析文件，计算圈复杂度 (Cyclomatic Complexity)。
    *   **合成**: `Impact = Normalize(Churn) * 0.6 + Normalize(Complexity) * 0.4`。

*   **智能检查清单 (`get_checklist`)**:
    *   **规则引擎**: 基于文件扩展名和内容特征匹配。
    *   **示例**:
        *   若 Diff 包含 `Mapper.xml` -> 添加 "Check SQL Injection risks"。
        *   若 Diff 包含 `@Transactional` 移除 -> 添加 "Verify Transaction boundary"。

### 3.4 持久化层 (Persistence)

使用 SQLite (`hyper_review.db`) 存储非 Git 托管的元数据。

**表结构设计**:

*   `repos`: 存储最近打开的仓库列表 (path, last_opened, bookmark_branch).
*   `local_tasks`: 存储用户创建的本地任务 (id, title, status, created_at).
*   `draft_reviews`: 存储尚未提交到远程的评审意见 (file_path, line, content, type).
*   `tags`: 用户自定义的快捷标签 (id, label, color).

### 3.5 搜索模块 (SearchService)

对应前端 `CommandPalette` 的功能。

*   **符号索引**: 启动仓库时，后台线程使用 `ctags` 或 `tree-sitter` 建立轻量级符号索引（Class, Function, Const）。
*   **全文搜索**: 调用 `ripgrep` 库进行内容搜索。
*   **命令匹配**: 简单的模糊匹配算法。

## 4. 接口实现规范 (Command Implementation)

所有 Command 必须返回 `Result<T, CommandError>`。

### 4.1 `open_repo_dialog` & `load_repo`
1.  调用 Tauri Dialog API 打开目录。
2.  检查目录下是否存在 `.git` 文件夹。
3.  初始化 `git2::Repository` 并存入 `AppState`。
4.  异步触发后台索引任务（热力图计算、符号索引）。
5.  写入 SQLite `repos` 表。

### 4.2 `get_file_diff(file_id)`
1.  从 `AppState` 获取当前 Repo。
2.  解析 `file_id` (相对路径)。
3.  生成 Diff 结构体 `Vec<DiffLine>`。
4.  **静态扫描**: 遍历变更行，简单的正则匹配（如检测硬编码 Secret、TODO 注释），将警告注入 `severity` 和 `message` 字段。

### 4.3 `submit_review`
1.  收集前端传入的 `comments` 和 `status`。
2.  **本地模式**: 存入 `draft_reviews` 表。
3.  **远程模式**:
    *   读取用户配置的 Token (从 Keychain/Keyring 获取，不存明文)。
    *   调用 CodeArts/GitLab OpenAPI `/projects/:id/merge_requests/:mr_iid/discussions`。
    *   若失败，保存为 Draft 并返回错误提示。

## 5. 性能与安全策略

### 5.1 性能优化
*   **骨架屏与预加载**: `open_repo` 后立即返回成功，后台异步计算 `ReviewStats` 和 `Heatmap`。
*   **Diff 限制**: 对超过 1MB 的文件或超过 2000 行的 Diff，默认只返回 Header，用户点击后按需加载（"Load Large Diff"）。
*   **Debounce**: 搜索接口需在前端做防抖，后端做查询取消（Cancel Previous Query）。

### 5.2 安全性
*   **Scope 限制**: `tauri.conf.json` 中严格限制 `fs` 模块的访问范围，仅允许访问用户选定的仓库目录及 AppData 目录。
*   **Token 存储**: 使用 `tauri-plugin-store` 或系统原生 Keyring 存储远程 API Token。
*   **输入清洗**: 防止 Command Injection，所有 Git 命令优先使用 `libgit2` API 而非 `Command::new("git")`。

## 6. 开发路线图 (Backend Phase)

1.  **Phase 1 (MVP)**:
    *   实现 `open_repo`, `get_branches`, `get_file_diff` (纯文本 Diff)。
    *   完成 SQLite 集成，支持 `get_recent_repos`。

2.  **Phase 2 (Analysis)**:
    *   集成 `tree-sitter`，实现 `get_heatmap` 和 Diff 语法高亮元数据。
    *   实现 `get_blame` 缓存机制。

3.  **Phase 3 (Remote)**:
    *   实现与 CodeArts/GitLab 的 API 对接。
    *   实现 `submit_review` 和 `sync_repo`。
