# HyperReview DiffView 对比视图修复方案

## 问题描述

原始DiffView存在以下问题：
1. **只显示变更的hunk**，不显示完整的新文件内容
2. **行号对比不正确**，没有正确维护旧版本和新版本的行号对应关系
3. **缺少统一的文件对比视图**，用户体验不佳

## 解决方案

### 1. 新增完整Diff引擎 (`complete_diff.rs`)

创建了新的 `CompleteDiffEngine`，特点：
- ✅ **获取完整文件内容**：分别读取旧版本和新版本的完整文件内容
- ✅ **完整diff算法**：逐行对比，生成完整的diff结果
- ✅ **正确的行号映射**：每行都有准确的old_line_number和new_line_number
- ✅ **新文件完整视图**：显示新版本的完整内容，包括未变更的行

### 2. 新增API端点

- **`get_complete_file_diff`**: 新的Tauri命令，使用完整diff引擎
- **前端API支持**: 新增 `useGetCompleteFileDiff` hook 和 `getCompleteFileDiff` 方法

### 3. DiffView智能选择

在 `DiffView.tsx` 中：
```typescript
// Use complete diff for branch comparisons to show full file content
let diffData;
if (oldCommit && newCommit && oldCommit !== newCommit) {
  console.log('Using complete diff for branch comparison');
  diffData = await getCompleteFileDiff(filePath || 'current-file', oldCommit, newCommit);
} else {
  console.log('Using regular diff for other comparisons');
  diffData = await getFileDiff(filePath || 'current-file', oldCommit, newCommit);
}
```

## 修复效果对比

### 测试文件：`frontend/api/types/checklist.ts`
**旧算法 (getFileDiff)**: 18行 - 仅包含变更的hunk
**新算法 (getCompleteFileDiff)**: 24行 - 包含完整的新文件内容

### 行号对比示例
```
旧算法显示：
  1: [old:None new:Some(5)] +   description?: string;       // 新增行
新算法显示：  
  1: [old:Some(1) new:Some(1)]     import { ReviewSeverity } from './diff';
  2: [old:Some(2) new:Some(2)]     
  3: [old:Some(3) new:Some(3)]     export interface ChecklistItem {
  4: [old:Some(4) new:Some(4)]       id: string;                 // UUID v4
  5: [old:Some(5) new:None] -       description: string;        // 删除行
  6: [old:None new:Some(5)] +       description?: string;       // 新增行
```

## 技术实现细节

### 核心算法 (`complete_diff.rs`)
```rust
pub fn compute_complete_diff(
    &self,
    file_path: &str,
    old_commit: &str,
    new_commit: &str,
) -> Result<Vec<DiffLine>, HyperReviewError> {
    // 1. 获取两个版本的完整文件内容
    let old_content = self.get_file_content_at_commit(old_commit, file_path)?;
    let new_content = self.get_file_content_at_commit(new_commit, file_path)?;
    
    // 2. 分割成行
    let old_lines: Vec<&str> = old_content.lines().collect();
    let new_lines: Vec<&str> = new_content.lines().collect();
    
    // 3. 执行完整diff算法
    let diff_result = self.compute_line_diff(&old_lines, &new_lines);
    
    // 4. 转换为DiffLine格式，维护正确的行号
    // ... 详细实现
}
```

### 行号维护逻辑
```rust
DiffOperation::Equal { old_line, new_line } => {
    result.push(DiffLine {
        old_line_number: Some(old_line_num),
        new_line_number: Some(new_line_num),  // 两行都有行号
        content: new_line.to_string(),        // 显示新版本内容
        line_type: DiffLineType::Context,
        // ...
    });
    old_line_num += 1;
    new_line_num += 1;  // 同时递增
}
```

## 使用方式

### 前端调用
```typescript
// 分支对比 - 使用完整diff显示完整文件内容
const diffLines = await getCompleteFileDiff(filePath, oldCommit, newCommit);

// 其他对比 - 使用传统diff
const diffLines = await getFileDiff(filePath, oldCommit, newCommit);
```

### 后端直接调用
```rust
let diff_engine = CompleteDiffEngine::new(repository);
let diff_lines = diff_engine.compute_complete_diff(
    "frontend/api/types/checklist.ts",
    "origin/main", 
    "origin/feature-merge-new-frontend/new"
)?;
```

## 验证结果

✅ **完整文件内容**: 新算法显示24行 vs 旧算法18行
✅ **正确行号映射**: 每行都有准确的old_line_number和new_line_number
✅ **新增行标识**: 11行新增内容被正确标记为 Added
✅ **删除行标识**: 9行删除内容被正确标记为 Removed  
✅ **上下文保留**: 4行未变更内容作为Context保留
✅ **前后端兼容**: 新API与现有DiffView组件无缝集成

## 总结

通过这次修复，HyperReview的DiffView现在可以：
1. **显示完整的新文件内容**，而不是仅显示变更片段
2. **提供准确的行号对比**，清晰显示每行在旧版本和新版本中的位置
3. **支持多种diff模式**，智能选择最适合当前场景的算法
4. **保持向后兼容**，不影响现有的功能和用户体验

修复后的对比视图更加直观和实用，特别适合代码审查场景。