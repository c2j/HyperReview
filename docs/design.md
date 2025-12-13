# HyperReview 设计规格书（FSD）  
—— Final Specification Document · v1.0（2025 年 12 月 12 日定稿）  
此文档可直接交给 Rust 前端、Tauri 工程师、UI 实现团队，一行代码不用再问“应该怎么做”。

## 1. 技术栈锁死（不允许再讨论）

| 层级          | 技术选型                                  | 版本（2025 年 12 月） | 理由                                   |
|---------------|-------------------------------------------|-----------------------|----------------------------------------|
| 桌面框架      | Tauri 2.0 + iced（纯 Rust UI）           | Tauri 2.0.1  iced 0.13 | 包体积最小、GPU 加速、无 Electron 臃肿 |
| Git 底层      | libgit2 + git2-rs                         | 0.19                  | 纯 Rust、无 Python/Node 依赖           |
| 代码高亮      | syntect + wgpu 手动渲染                   | syntect 5.2           | 10 万行仍 60fps                        |
| LSP 支持      | rust-analyzer（Java 用 tree-sitter + jdt.ls 改造版） | —                     | 跳定义、hover 零延迟                   |
| 向量搜索      | Qdrant（本地嵌入模式）+ DeepSeek-Coder-6.7B-Instruct |               | 语义搜索                               |
| 数据库/存储   | sled（嵌入式 KV）                         | 0.34                 | 审查记录本地加密存储                   |
| 打包体积目标  | Windows ≤ 85MB macOS ≤ 70MB Linux ≤ 80MB |                       |                                        |

## 2. 全局窗口规格（像素级锁死）

| 项目                   | 数值                  | 备注                                      |
|------------------------|-----------------------|-------------------------------------------|
| 最小窗口尺寸           | 1400 × 900           | 小于此尺寸自动提示“屏幕太小，神器无法施展” |
| 默认窗口尺寸           | 1920 × 1080           | 首次启动全屏后记住                        |
| 三大面板宽度比例       | 15% │ 55% │ 30%   | 可拖拽调节，记住用户偏好                  |
| 标题栏高度            | 36px                 |                                           |
| 搜索栏高度            | 46px                 |                                           |
| 
| 工具栏+仓库信息高度   | 52px                 |                                           |
| 全局操作条高度         | 42px                 |                                           |
| 状态栏高度            | 28px                 |                                           |
| 字体                   | JetBrains Mono Medium 14px（代码区）/界面区 13px） | 支持 120% / 150% DPI 缩放 |

## 3. 界面分层与组件树（iced 实现结构）

```
App
├── TitleBar (Custom)
│   ├── SearchBar (普通 + 语义)
│   └── WindowControls
├── ToolBar (打开仓库 / 导入任务 / 提交目标 / 模式切换 / 快捷标签)
├── CurrentRepoBar (仓库路径 + 分支 + commit + 统计)
├── SplitPane (三个可拖拽面板)
│   ├── LeftPane (15%)          → TaskTree
│   ├── CenterPane (55%)        → DiffView
│   └── RightPane (30%)         → TabbedPane (热图 / Blame / 统计 / 清单)
├── GlobalActionBar (固定底部上方)
└── StatusBar
```

## 4. 中央 DiffView 详细规格（核心中的核心）

| 项目                    | 规格要求                                      |
|-------------------------|-----------------------------------------------|
| 渲染引擎                | iced_wgpu + custom shader                     |
| 行高                    | 22px                                         |
| 行号区宽度              | 60px                                         |
| 变更热力条宽度          | 12px                                         |
| 最大同时显示行数        | 10 万行不卡（虚拟化滚动）                    |
| 折叠规则               | 自动折叠：import、lombok 注解、use 语句、SQL 注释块 |
| 圈选方式               | Shift+拖拽 或 Vim 视觉模式 v + hjkl          |
| 右键菜单出现延迟        | ≤ 120ms                                       |
| 专项警告渲染            | 事务缺失 → 红色波浪下划线 + 行尾 ⚠ 图标       |

## 5. 右侧四面板具体规格

| Tab          | 刷新频率   | 数据来源                     | 交互要求                  |
|--------------|------------|------------------------------|---------------------------|
| 架构热图    | 文件切换即时 | tree-sitter 解析 + Cargo/Spring 分层规则 | 点击模块 → 跳转文件列表    |
| Blame        | 悬停 150ms | libgit2 blame                | 缓存最近 500 行          |
| 审查统计    | 实时       | 本地 sled                   | 饼图 + 预计剩余时间      |
| 待审清单    | 手动拖拽排序 | glob + 行范围解析           | 支持右键移除/跳转         |

## 6. 数据存储格式（本地零云端）

| 数据类型         | 存储路径                          | 格式         |
|------------------|-----------------------------------|--------------|
| 审查意见        | ~/.hyperreview/reviews/{repo_hash}/{commit}.json | JSON + 加密 |
| 用户快捷标签模板 | ~/.hyperreview/templates.json      | TOML         |
| 窗口布局偏好    | ~/.hyperreview/layout.ron         | RON          |
| Qdrant 向量库   | ~/.hyperreview/qdrant             | 本地         |

## 7. OpenAPI 推送规范（支持主流第三方系统）

统一 JSON 结构体（已兼容 Gerrit / GitLab / CodeArts）：

```json
{
  "change_id": "Iabcd1234",
  "revision": "3c9fa1b",
  "comments": [
    {
      "file": "/src/main/java/.../RetryServiceImpl.java",
      "line": 128,
      "range": {"start": 124, "end": 129},
      "severity": "ERROR",           // ERROR / WARNING / INFO
      "message": "缺少 @Transactional，存在部分失败风险",
      "tag": "事务缺失",
      "patch": "base64 encoded unified diff（可选）"
    }
  ]
}
```

## 8. 性能指标（红线，不可妥协）

| 场景                         | 指标               |
|------------------------------|--------------------|
| 冷启动到显示第一个 diff      | ≤ 3.8 秒          |
| 切换文件                     | ≤ 180ms           |
| 全局搜索 10 万行            | ≤ 80ms            |
| 语义搜索返回前 20 条        | ≤ 1.2 秒         |
| 100 条意见一次性推送         | ≤ 4 秒（网络正常）|
