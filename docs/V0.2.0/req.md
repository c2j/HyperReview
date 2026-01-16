# HyperReview + Gerrit Code Review 集成功能需求规格书  
—— Gerrit REST API 对接模块（v0.2.0 · 2025 年 12 月 30 日定稿）

## 1. 功能使命（一针见血）

**让 HyperReview 成为 Gerrit 的核弹级外挂**：Gerrit 管流程/权限/存档，HyperReview 专杀高质量人工审查（架构热图、行级圈选、批量批注、离线审、补丁生成）。  
Tech Lead 在 HyperReview 里离线审 300 文件 Gerrit Change，审完 Shift+Enter 推 47 条批注 + 补丁 + +2 分，Gerrit Web 直接进垃圾桶。

## 2. 目标用户场景（企业 Gerrit 标配）

| 用户角色 | 典型场景 | 痛点解决 |
|----------|----------|----------|
| Tech Lead | 审 Gerrit Change #12345 的 127 个文件 | Gerrit Web 卡顿、评论分散、历史遗毒难找 |
| 架构师 | 批量审支付系统重构，推架构债批注 | 离线热图审 + 一键推行级补丁 |
| DBA | 审 45 个存储过程，推性能/安全批注 | 行范围精准定位 + 批量推 Gerrit |

## 3. 核心功能清单（MVP → v1.0）

| 功能模块 | MVP（1 周） | v1.0（3 周） | Gerrit REST API 端点 |
|----------|-------------|--------------|---------------------|
| Gerrit 配置（URL + Token） | 支持（设置页 + 测试连接） | 支持（加密存储 + 多 Gerrit 实例） | GET /a/config/server/version |
| 导入 Gerrit Change | 支持（Change ID / 搜索导入） | 支持（Webhook 自动导入） | GET /changes/{id}, GET /changes/?q=... |
| 加载 Change diff + 文件列表 | 支持（行级 diff + 现有评论） | 支持（分批加载大 Change） | GET /changes/{id}/files/{file}/diff |
| 离线审查（热图/圈选/补丁） | 支持（复用现有功能） | 支持（Gerrit 评论橙色高亮） | — |
| 一键推批注（行级/文件级） | 支持（批量 POST） | 支持（冲突检测 + 重试） | POST /changes/{id}/revisions/{rev}/comments |
| 推评分（+2/-2）+ 消息 | 支持 | 支持 | POST /changes/{id}/revisions/{rev}/review |
| 推补丁（Patch Set） | — | 支持（git push 或 /patch:apply） | POST /changes/{id}/patch:apply |
| 实时同步（新评论/状态） | — | 支持（5min 轮询 + toast） | GET /changes/{id}/messages |

## 4. UI 集成规格（原 UI 零破坏）

### 4.1 工具栏新增（最顶行）
```
+打开仓库   +导入任务   🔌 从 Gerrit 导入 Change   提交到 ▾ Gerrit   模式 ▾ Java+SQL
```
- 点击 → 弹窗输入 Change ID（如 #12345）或搜索条件（如 `status:open project:payment`）
- 成功导入 → 左侧任务区自动新增 Gerrit Change 组

### 4.2 左侧任务区新增组（视觉隔离）
```
▼ 本地任务 (3)
▼ 🔌 Gerrit Change (5)                    ← 紫色组标题
  🔌 #12345 支付超时补偿（73/127）进行中   ← 紫色条 + 🔌 + 进度
  🔌 #12341 SQL 存储过程（18/45）进行中
```
- **右键菜单**：
  ```
  ├─ 刷新最新 Patch Set
  ├─ 推 47 条批注
  ├─ 推补丁 +2 分
  ├─ 打开 Gerrit Web
  └─ 标记已审
  ```

### 4.3 状态栏动态显示
```
[/payment-service] 127文件 → 已圈选47处 就绪 → 推到 Gerrit #12345 (Patch Set 3)
```
- 当前任务是 Gerrit Change 时动态显示 Change ID + Patch Set

### 4.4 全局操作条增强
```
Ctrl+Enter 下一文件   Shift+Enter 整批审完并推到 Gerrit   / 搜索
```
- 当前是 Gerrit Change 时，Shift+Enter 弹确认窗：
  ```
  确认推送 Gerrit #12345？
  [推 47 条批注] [推补丁 +2 分] [只推批注] [取消]
  ```

## 5. 数据流（端到端流程锁死）

```
1. 用户点击 "从 Gerrit 导入" → 输入 #12345
2. Tauri invoke('gerrit_get_change', { changeId: '12345' })
3. Rust GET /a/changes/12345 → 解析文件列表 + 当前 Revision ID
4. React 左侧任务区新增 "🔌 #12345 支付超时补偿（0/127）"
5. 点击任务 → invoke('gerrit_get_diff') → 中央 Diff 加载第一个文件
6. 圈选打标签 → 本地 Zustand 存意见
7. 审完 Shift+Enter → invoke('gerrit_post_comments') → Gerrit 新增 47 条批注
8. Gerrit 自动加 Patch Set + 通知作者
```

## 6. Gerrit REST API 端点规格（3.13.1 版）

| 操作 | HTTP 方法 | 端点 | 参数 | 返回 |
|------|-----------|------|------|------|
| 测试连接 | GET | `/a/config/server/version` | — | `{"version": "3.13.1"}` |
| 搜索 Change | GET | `/a/changes/` | `q=status:open&o=DETAILED_LABELS` | ChangeInfo[] |
| 获取 Change | GET | `/a/changes/{change-id}` | `o=CURRENT_REVISION,CURRENT_FILES` | ChangeInfo |
| 获取文件 diff | GET | `/a/changes/{id}/revisions/{rev}/files/{file}/diff` | — | DiffContent |
| 推送批注 | POST | `/a/changes/{id}/revisions/{rev}/comments` | `{"comments": [...]}` | 204 No Content |
| 推送评分 | POST | `/a/changes/{id}/revisions/{rev}/review` | `{"labels": {"Code-Review": "2"}}` | 204 No Content |

## 7. 认证 & 安全（企业标配）

| 项目 | 规格 |
|------|------|
| 认证方式 | HTTP Basic Auth（username:password） |
| Token 存储 | Tauri `app.path_resolver().app_local_data_dir()` + JSON + AES 加密 |
| 多实例支持 | 支持配置多个 Gerrit（prod/dev），默认选最近用 |
| 权限要求 | Read + Push + Label（Code-Review） |

## 8. 非功能需求（红线）

| 项目 | 指标 |
|------|------|
| 拉 Change 详情（127 文件） | ≤ 3 秒 |
| 拉单个文件 diff（5000 行） | ≤ 1 秒 |
| 推送 47 条批注 | ≤ 2 秒 |
| 离线模式 | 100% 支持（审完再推） |
| Gerrit 版本兼容 | 3.6+（推荐 3.13+） |

## 9. 异常处理（企业级）

| 场景 | 处理 |
|------|------|
| 401 Token 过期 | 弹窗重新输入密码 |
| 409 评论冲突 | 拉最新评论 → 本地合并 → 提示用户 |
| 网络超时 | 离线继续审，恢复网络自动重试 |
| 大 Change（>500 文件） | 分批加载 + 进度条 |

## 10. 终极宣言

这个功能做出来，HyperReview 就从"代码审查加速器"进化成**"企业 Gerrit 的核武器外挂"**。  
Gerrit Web 继续管仪式感，HyperReview 让 Tech Lead 审代码效率起飞 5-10 倍。
