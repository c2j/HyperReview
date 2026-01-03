# Gerrit Integration Debug Test Script

## 🎯 测试目标
验证Gerrit集成功能是否正常工作，并诊断任何问题。

## 🔍 调试步骤

### 步骤1: 检查控制台日志
1. 打开应用并按F12打开开发者工具
2. 切换到Console标签页
3. 观察以下日志输出：

```javascript
// 你应该看到这些日志：
"SettingsModal: Loading Gerrit instances..."
"SimpleGerritService: Getting instances..."
"SimpleGerritService: Using test mode data"
"SettingsModal: Loaded instances: [Array of instances]"
"SettingsModal: Rendering instance: {...}"
```

### 步骤2: 手动测试API调用
在控制台中执行以下命令：

```javascript
// 测试1: 直接调用Tauri命令
await window.__TAURI__.tauri.invoke('gerrit_get_instances_simple')
  .then(result => console.log('✅ API Success:', result))
  .catch(error => console.error('❌ API Error:', error));

// 测试2: 调用简化服务
await simpleGerritService.getInstances()
  .then(instances => console.log('✅ Service Success:', instances))
  .catch(error => console.error('❌ Service Error:', error));

// 测试3: 检查服务状态
console.log('Test mode:', simpleGerritService.isTestMode());
console.log('Service available:', typeof simpleGerritService);
```

### 步骤3: 检查UI状态
在控制台中执行：

```javascript
// 检查SettingsModal状态
console.log('SettingsModal state:', {
  activeTab: document.querySelector('.settings-modal')?.dataset?.activeTab,
  instancesCount: document.querySelectorAll('.gerrit-instance').length,
  loadingState: document.querySelector('.loading-indicator')?.textContent
});
```

### 步骤4: 强制显示测试数据
如果正常加载失败，可以手动添加测试数据：

```javascript
// 手动添加测试实例
const testInstance = {
  id: "manual-test-1",
  name: "Manual Test Instance",
  url: "https://manual-test.com",
  username: "testuser",
  is_active: true,
  status: "Connected"
};

// 如果SettingsModal组件暴露了这个函数，可以调用：
if (window.settingsModal) {
  window.settingsModal.addTestInstance(testInstance);
}
```

## 🧪 完整测试流程

### 测试1: 基础连接性
1. 打开应用 → 设置 → External Systems
2. 观察控制台日志
3. 点击"🧪 Test Service"按钮
4. 验证输出

### 测试2: 创建实例
1. 点击"Configure"按钮
2. 填写测试数据：
   ```
   URL: https://test-gerrit.com
   Username: testuser  
   Password: testpass
   Name: Test Instance
   ```
3. 点击保存
4. 观察控制台输出

### 测试3: 实例管理
1. 验证实例是否出现在列表中
2. 点击"Test"按钮
3. 点击"Set Active"按钮
4. 观察状态变化

## 📊 预期结果

### ✅ 成功情况
```
SettingsModal: Loading Gerrit instances...
SimpleGerritService: Getting instances...
SimpleGerritService: Using test mode data
SettingsModal: Loaded instances: [Array(1)]
SettingsModal: Rendering instance: {id: "test-instance-1", name: "Test Gerrit Server", ...}
✅ Gerrit实例创建成功！
```

### ❌ 失败情况
```
SettingsModal: Loading Gerrit instances...
SimpleGerritService: Getting instances...
❌ Failed to get instances: Unknown error
SettingsModal: Failed to load Gerrit instances: [Error object]
```

## 🛠️ 故障排除

### 问题1: "Unknown error" 在API调用
**解决方案**: 
1. 检查Tauri命令是否正确注册
2. 验证命令名称拼写
3. 检查后端是否编译成功

### 问题2: 界面不更新
**解决方案**:
1. 检查React状态更新
2. 验证useEffect依赖
3. 强制重新渲染组件

### 问题3: 权限错误
**解决方案**:
1. 检查Tauri allowlist配置
2. 验证对话框权限
3. 使用console.log代替alert

## 🎯 验证清单

- [ ] External Systems标签页可见
- [ ] "Configure"按钮可点击
- [ ] CredentialManager对话框正常弹出
- [ ] 可以输入Gerrit配置信息
- [ ] 保存后控制台显示成功日志
- [ ] 实例出现在列表中
- [ ] Test按钮可以测试连接
- [ ] Set Active按钮可以切换状态

## 📞 需要帮助？

如果测试失败，请提供：
1. 完整的控制台日志
2. 具体的错误信息
3. 操作步骤
4. 浏览器环境信息

我会根据具体情况提供进一步的诊断和解决方案。