# Gerrit Integration UI Test

## 测试目标
验证Gerrit集成功能在UI中是否可见和可用。

## 当前状态 ✅

### 已完成的功能：
1. **设置窗口** - 添加了"External Systems"标签页
2. **Gerrit实例管理** - 基本UI组件已集成
3. **凭证管理器** - 已存在并可调用
4. **后端API** - 基本命令已注册并可调用

### 如何访问Gerrit功能：

#### 方法1：通过设置窗口
1. 打开HyperReview
2. 点击设置图标（通常在右上角）
3. 选择"External Systems"标签页
4. 点击"Configure"按钮配置Gerrit

#### 方法2：通过任务提交
1. 创建或打开一个任务
2. 点击"Submit"或"Export"按钮
3. 选择"External System"选项
4. 选择"Gerrit"作为目标系统

## 预期界面元素

### 设置窗口 - External Systems标签页
```
┌─────────────────────────────────────────┐
│ Settings                                  │
├─────────────────────────────────────────┤
│ [General] [Editor] [Shortcuts] [AI] [External Systems] │
├─────────────────────────────────────────┤
│ External Systems                          │
│                                           │
│ Gerrit Code Review                        │
│ Configure Gerrit server instances for     │
│ code review integration                   │
│                    [Configure]            │
│                                           │
│ No Gerrit instances configured            │
│ Click "Configure" to add your first      │
│ Gerrit server                              │
│                                           │
│ Note: HTTP passwords can be generated    │
│ in your Gerrit account settings...       │
└─────────────────────────────────────────┘
```

### 凭证管理器对话框
```
┌─────────────────────────────────────────┐
│ Credential Manager                        │
├─────────────────────────────────────────┤
│ System: [Gerrit ▼]                       │
│                                           │
│ Gerrit URL: [____________________]       │
│ Username:   [____________________]       │
│ Password:   [____________________]       │
│                                           │
│                    [Cancel] [Save]       │
└─────────────────────────────────────────┘
```

## 测试步骤

### 测试1：访问设置中的External Systems
1. 启动HyperReview
2. 打开设置窗口
3. 验证"External Systems"标签页是否存在
4. 点击"Configure"按钮
5. 验证凭证管理器是否打开

### 测试2：创建Gerrit实例
1. 在凭证管理器中选择"Gerrit"
2. 输入测试信息：
   - URL: `https://gerrit.example.com`
   - Username: `testuser`
   - Password: `testpass`
3. 点击保存
4. 验证是否显示成功消息

### 测试3：测试连接
1. 在设置页面中点击"Test"按钮
2. 验证连接测试结果

## 已知限制
- 当前为模拟实现，不会真正连接到Gerrit服务器
- 实例数据存储在内存中，重启应用后会丢失
- 完整的CRUD功能将在Phase 3实现

## 下一步
1. 测试UI界面是否按预期显示
2. 收集用户反馈
3. 实现真实的后端数据库集成
4. 添加更多Gerrit功能（变更导入、离线评审等）

需要我帮您测试具体的功能或者创建更详细的测试用例吗？