# Feature Specification: Local Task Management

**Feature Branch**: `003-local-task-management`
**Created**: 2025-12-15
**Status**: Draft
**Input**: User description: "完善"创建本地任务"功能以及对应的代码审核任务管理机制。当点击"创建本地任务"按钮时，可从文本中导入对当前仓库的审核任务，文本包括多行任务描述，包含如下字段：仓库、分支、文件相对目录、行号（可选）、审核问题（可选）等。应能对任务文本进行解析、保存，然后作为待审核项推进后续审核任务。具体需求说明如下：# HyperReview 「创建本地任务」功能完整需求规格书
—— Local Review Task Management（v1.0 Final · 2025 年 12 月 15 日定稿）

## Clarifications

### Session 2025-12-15

- Q: What happens when importing text contains invalid file paths? → A: System validates file existence and marks invalid entries with error indicators, allowing users to correct or skip them
- Q: What is the maximum number of tasks a user can create? → A: No explicit limit on number of tasks, only per-task item limit of 10,000
- Q: Who can edit or delete a local task? → A: Only the task creator can edit or delete their own tasks
- Q: How are tasks from different repositories isolated? → A: Tasks are stored in separate JSON files with repository path metadata, preventing cross-referencing
- Q: What is the data retention policy for archived tasks? → A: Archived tasks remain in ~/.hyperreview/local_tasks/ indefinitely until manually deleted by user
- Q: How should system handle repository access permissions when creating local tasks? → A: System does not validate permissions, users can import any locally accessible repository path
- Q: How should the system handle concurrent edits to the same task file? → A: System uses file locking to prevent simultaneous edits, with last-write-wins policy and user notification of conflicts
- Q: How should system handle file permissions when exporting review reports? → A: System saves exported files to default download directory using system default file permissions
- Q: What backup and recovery mechanism should system provide for task data? → A: System does not provide backup functionality, task data is entirely managed by users
- Q: How should system handle text encoding when importing from different sources? → A: System only supports UTF-8 encoding, other encodings may result in garbled text or parsing errors
- Q: How should system handle task ownership transfer when creator leaves or changes permissions? → A: Tasks cannot be transferred, tasks become permanently read-only when creator leaves

## 1. 功能概述（一针见血）

**核心使命**：让顶级 Tech Lead 彻底摆脱 PR/MR 束缚，能够随时随地离线审查任意仓库的任意指定文件/行范围。
这是 HyperReview 与所有浏览器工具、AI Bot 的终极差异化——**PR 只能审"别人推上来的"，本地任务能审"你想审的任何一行历史烂代码"**。

不做这个功能，HyperReview 只是"更快一点的网页版"；
做了，才配叫"让剩下 20% 的顶级 reviewer 成神"。

## 2. 目标用户与典型场景

| 用户角色             | 典型场景                                      | 痛点解决                              |
|----------------------|-----------------------------------------------|---------------------------------------|
| Tech Lead / 架构师  | 安全审计、遗留代码清理、事务边界专项审查      | 不用等 PR，直接批量导入雷区清单      |
| DBA / SQL 专家       | 存储过程性能/安全审查                         | 离线圈选 + 预设问题，一键打标签      |
| 技术委员会成员       | 重构预审、历史代码漂移审计                    | 支持行范围精准定位，进度持久化       |

## 3. 功能入口

| 入口位置                     | 操作方式                          | 备注 |
|------------------------------|-----------------------------------|------|
| 左侧任务区底部               | `+ 创建本地审查任务` 按钮        | 主入口 |
| 工具栏                       | `+导入任务` 快捷按钮             | 次入口 |
| 左侧任务区空白处右键         | "创建本地审查任务"               | 快捷 |

## 4. 创建任务弹窗（模态对话框，800×600px）

布局结构（锁死）：

```
任务名称（必填）：_______________________________

目标仓库（必填）：
  [浏览本地路径]  /path/to/payment-service   [打开仓库验证]

分支/Tag/Commit（必填）：___________________   [下拉切换分支]

任务描述文本（支持直接粘贴多行）：
┌──────────────────────────────────────────────────────────────┐
│# 文件相对路径<TAB>行范围<TAB>预设问题<TAB>严重程度<TAB>标签   │
│src/main/java/com/pay/Retry.java    124-189    N+1风险    ✗    N+1,硬编码   │
│src/main/resources/mapper/Payment.xml        MyBatis不一致    ⚠        │
│db/procedure/pkg_payment.pkb    200-500    缺少异常捕获    ✗    存储过程   │
└──────────────────────────────────────────────────────────────┘
[导入模板]  [清空]  [解析预览（127 项，3 项格式错误）]

                                 [取消]          [创建任务]
```

## 5. 文本格式与解析规则（锁死）

每行一个任务项，支持 Tab 或 2+ 空格分隔，字段顺序：

| 字段 | 必填 | 格式示例                          | 说明 |
|------|------|-----------------------------------|------|
| 1 文件相对路径 | 必填 | src/main/java/com/pay/Retry.java | 相对仓库根目录 |
| 2 行范围       | 可选 | 124-189 / 124 / 124- / -189 / 空 | 空或"-"=全文件 |
| 3 预设问题描述 | 可选 | 潜在N+1风险                       | 导入后自动填充意见 |
| 4 严重程度     | 可选 | ✗ / ⚠ / ❓ / ✓                    | 自动打标签 |
| 5 自定义标签   | 可选 | N+1,硬编码SQL                     | 逗号分隔，多标签 |

**容错与提示**：
- 自动 trim 空格
- 行首 # = 注释，忽略
- 空行忽略
- 解析失败行在预览区红底高亮 + 提示"第 X 行：字段不足或格式错误"

**内置模板**（点击 [导入模板] 填充）：
```
# 文件相对路径    行范围    预设问题    严重程度    标签
src/main/java/com/alipay/payment/RetryServiceImpl.java    124-189    潜在N+1风险    ✗    N+1,性能
src/main/resources/mapper/PaymentMapper.xml        MyBatis XML 与接口不一致    ⚠    MyBatis
db/procedure/pkg_payment_retry.pkb    200-500    缺少异常捕获和日志    ✗    存储过程,异常处理
```

## 6. 任务存储机制

| 项目               | 规格 |
|--------------------|------|
| 存储路径          | ~/.hyperreview/local_tasks/{task_id}.json（task_id = UUID） |
| 任务元数据        | { id, name, repo_path, base_ref (branch/commit), create_time, update_time, status: "in_progress"|"completed"|"archived", total_items, completed_items } |
| 任务项数组        | [ { file: string, line_range?: {start?:number, end?:number}, preset_comment?: string, severity?: "error"|"warning"|"question"|"ok", tags?: string[], reviewed: boolean, comments: Comment[] } ] |
| 进度持久化        | 每次审完一个文件实时保存，关闭软件后下次打开直接恢复进度 |

## 7. 左侧任务区显示与标识（狠区分 PR）

```
▼ 待我审核 (PR/MR)                  ← 蓝色组标题
  ● PR#2877 ...
  ○ PR#2869 ...

▼ 本地任务 (5)                      ← 橙色组标题 + 📁 图标
  📁 支付系统雷区清理（73/127） 进行中    ← 橙色垂直条 + 进度
  📁 SQL性能审计 Q4（18/45） 进行中
  📁 事务边界专项（已完成） ✓          ← 绿色勾

▼ 我关注的 ...
▼ 历史审查 ...
```

**右键菜单区别**：
- PR：刷新、打开 GitHub 等
- 本地任务独有：编辑任务、重新导入文本、导出为报告、标记完成、删除、归档

## 8. 审核流程集成（零延迟）

1. 点击本地任务 → 自动加载仓库（若未打开则后台 clone/open）
2. 按任务项顺序逐个加载文件到中央 Diff 主战场（对比 base_ref 的 HEAD）
3. 若有行范围 → 自动滚动并高亮该范围
4. 若有预设问题/严重程度 → 圈选后自动填充意见框 + 打标签
5. 审完当前文件 → 自动标记 ✓ + 跳下一文件（Ctrl+Enter）
6. 全任务完成 → 状态栏提示"本地任务已完成，可一键推送审查报告"
7. 推送支持：批量生成 JSON 报告，通过 OpenAPI 推送到 Gerrit/CodeArts/自建系统（带文件路径、行范围、意见、补丁）

## 9. 非功能需求（红线）

| 项目                     | 指标要求                  |
|--------------------------|---------------------------|
| 解析 2000 行任务文本     | ≤ 500ms                  |
| 切换任务项加载文件       | ≤ 300ms                  |
| 单任务最大项数           | 10000 项                 |
| 进度恢复准确率           | 100%                     |
| 仓库切换时任务隔离       | 不同仓库的任务互不干扰   |

"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Create and Review Local Task from Text Import (Priority: P1)

As a Tech Lead, I need to create a local review task by importing a list of files and review criteria from text, so I can systematically review specific code sections without waiting for pull requests.

**Why this priority**: This is the core value proposition - enabling reviewers to audit arbitrary code sections离线. It differentiates HyperReview from all PR-based tools and directly addresses the primary user pain point.

**Independent Test**: Can be fully tested by importing a text file with 10-20 file entries, creating the task, and reviewing at least 3 files in sequence. Delivers immediate value even if no other features exist.

**Acceptance Scenarios**:

1. **Given** I have a text file with task entries, **When** I click "创建本地审查任务" and paste the text, **Then** the system parses it, validates the format, and shows a preview with success/error indicators.

2. **Given** I have filled in task name, selected a valid repository and branch, **When** I click "创建任务", **Then** the task is saved, appears in the left sidebar under "本地任务", and I can click it to start reviewing.

3. **Given** I am reviewing a local task item, **When** I navigate to the next file using Ctrl+Enter, **Then** the system automatically marks the current file as reviewed and loads the next file with any preset comments and severity tags pre-populated.

4. **Given** I have multiple local tasks, **When** I view the left sidebar, **Then** each task shows progress (completed/total items), status (进行中/已完成), and is visually distinguished from PR/MR items with orange styling and 📁 icon.

5. **Given** I close and reopen the application, **When** I click on a local task, **Then** my previous progress is restored - all reviewed files remain marked and I continue from the next un-reviewed item.

---

### User Story 2 - Manage and Track Task Progress (Priority: P2)

As a reviewer, I need to manage multiple local tasks, track my progress, and organize them by status, so I can efficiently handle various review assignments without losing context.

**Why this priority**: Once users start creating tasks, they need to manage them effectively. This enables power users to handle dozens of tasks simultaneously and maintain productivity across long audit cycles.

**Independent Test**: Can be fully tested by creating 3 tasks with different statuses (进行中, 已完成, archived), using right-click menus to edit/manage them, and verifying progress tracking works correctly. Adds significant value for active users.

**Acceptance Scenarios**:

1. **Given** I have an active local task, **When** I right-click it in the sidebar, **Then** I see options to edit task, re-import text, export report, mark complete, delete, or archive.

2. **Given** I need to update a task's scope, **When** I select "重新导入文本", **Then** I can replace the task items while preserving metadata like creation time and repository.

3. **Given** I complete a task, **When** I mark it as completed, **Then** it shows with a green checkmark (✓) and is visually separated from active tasks.

4. **Given** I no longer need a task, **When** I archive it, **Then** it remains in the system for history but is hidden from the main task list.

---

### User Story 3 - Export and Share Review Results (Priority: P3)

As a technical leader, I need to export my local task review results in a standard format, so I can share findings with team members or integrate with external review systems.

**Why this priority**: Enables collaboration and reporting. Allows review insights to flow back into the development process, making the tool valuable beyond just individual productivity.

**Independent Test**: Can be fully tested by completing a task and generating an exportable JSON report with file paths, line ranges, comments, and severity ratings. Provides value for team-based workflows.

**Acceptance Scenarios**:

1. **Given** I have completed review items in a task, **When** I export the task, **Then** I receive a JSON file containing all file paths, line ranges, comments, severity levels, and tags for each reviewed item.

2. **Given** I need to integrate with external systems, **When** I export, **Then** the JSON follows a standard schema compatible with Gerrit, CodeArts, or other review platforms.

---

### Edge Cases

- What happens when the imported text contains invalid file paths that don't exist in the repository?
- How does the system handle repositories that are not yet cloned or opened?
- What occurs when a task contains more than 10,000 items (the maximum)?
- How are tasks from different repositories isolated and managed?
- What happens when branch/commits referenced in a task no longer exist?
- How does the system handle concurrent edits to the same task file?
- What happens when disk space is low and tasks cannot be saved?
- How does the system behave when parsing text with special characters or encoding issues?
- How should the system handle repositories that the user doesn't have permission to access?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide a "创建本地审查任务" button in the left task panel that opens a modal dialog for task creation.

- **FR-002**: System MUST support importing task descriptions via plain text with tab or space-separated values containing: file path (required), line range (optional), preset question (optional), severity (optional), tags (optional).

- **FR-003**: System MUST parse imported text within 500ms for up to 2000 lines, automatically ignoring comment lines (starting with #), empty lines, and trimming whitespace.

- **FR-004**: System MUST validate task entries and highlight parsing errors with specific line numbers and error descriptions.

- **FR-005**: System MUST require users to specify task name, repository path, and branch/commit reference before saving a task.

- **FR-006**: System MUST save tasks as JSON files in the user's home directory under `~/.hyperreview/local_tasks/{uuid}.json` with metadata (id, name, repo_path, base_ref, create_time, update_time, status, progress).

- **FR-007**: System MUST display local tasks in the left sidebar grouped separately from PR/MR items, using orange color scheme and 📁 icon for visual distinction.

- **FR-008**: System MUST show task progress as "completed_items/total_items" and status (进行中, 已完成, archived) for each local task.

- **FR-009**: System MUST support task lifecycle operations: edit, re-import text, export, mark complete, delete, and archive via right-click context menu.

- **FR-010**: System MUST automatically persist review progress after each file is marked as reviewed, enabling 100% accurate recovery after application restart.

- **FR-011**: System MUST load files from the correct branch/commit when starting a review, automatically scrolling to specified line ranges and highlighting them.

- **FR-012**: System MUST pre-populate review interface with preset comments and severity tags when available for each task item.

- **FR-013**: System MUST support keyboard shortcut (Ctrl+Enter) to mark current file reviewed and advance to next item.

- **FR-014**: System MUST generate exportable JSON reports containing file paths, line ranges, comments, severity ratings, and tags for completed tasks.

- **FR-015**: System MUST enforce task isolation by repository, ensuring tasks from different repositories don't interfere with each other.

- **FR-016**: System MUST support up to 10,000 items per task and display appropriate warnings when approaching this limit.

- **FR-017**: System MUST provide built-in text templates for common review scenarios (Java, SQL, XML files).

- **FR-018**: System MUST switch between task items and load files within 300ms for optimal user experience.

### Key Entities

- **Local Task**: Represents a collection of review items for a specific repository and branch/commit. Contains metadata (id, name, repository path, base reference, creation date, status, total items, completed items) and a list of task items.

- **Task Item**: Represents a single file review unit with attributes: file path (relative to repository root), optional line range (start/end line numbers), optional preset comment, optional severity level (error/warning/question/ok), optional tags array, reviewed status, and comments array.

- **Task Repository**: Represents the association between a local task and its target git repository, including path, branch/commit reference, and clone/open status.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can parse and validate 2000 lines of task import text in under 500ms, enabling quick task creation even for large audits.

- **SC-002**: Users can navigate between task items and load files in under 300ms, maintaining review flow without interruption.

- **SC-003**: System achieves 100% accurate progress recovery after application restart or crash, ensuring no review work is lost.

- **SC-004**: System supports single tasks containing up to 10,000 items without performance degradation or data loss.

- **SC-005**: Tasks from different repositories are completely isolated, preventing data mixing or cross-contamination.

- **SC-006**: 95% of users successfully create and complete their first local task without requiring external assistance.

- **SC-007**: Users complete task reviews 40% faster than equivalent PR-based reviews due to offline capability and batch import.

- **SC-008**: 90% of task import attempts with valid text format succeed on first try, with clear error messages for invalid entries.

- **SC-009**: System maintains task data integrity even with concurrent access from multiple review sessions.

- **SC-010**: Review completion rate for local tasks reaches 85% or higher, indicating user engagement and task relevance.
