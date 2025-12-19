# Repository Switch Fix - 热力图清除修复

**Date**: 2025-12-15  
**Issue**: 当用户选择新仓库时，旧仓库的热力图数据没有清除，导致显示错误的数据

## 问题描述

当用户在应用中打开仓库选择对话框并选择不同的目录时，右侧面板的热力图仍显示旧仓库的数据，没有更新为当前新仓库的热力图。

## 根本原因

1. **RightPanel.tsx** 中的数据加载逻辑依赖于 `isRepoLoaded` 和 `heatmapData.length` 的组合条件
2. 当仓库切换时，`isRepoLoaded` 可能仍为 true，而 `heatmapData.length` 不为 0（因为旧数据还在），因此不会触发数据刷新
3. store 中的任务、统计数据、热力图、清单等数据没有在仓库切换时清除

## 解决方案

### 1. 修改 RightPanel.tsx

**添加仓库路径依赖**：
- 获取当前仓库的路径 `currentRepoPath`
- 添加 `useEffect` 在仓库路径改变时清除所有数据

```typescript
// Clear all data when repository changes
const repoInfo = getRepositoryInfo();
const currentRepoPath = repoInfo?.path;

useEffect(() => {
  if (currentRepoPath) {
    // Repository changed, clear all data
    setHeatmapData([]);
    setBlameData(null);
    setStatsData(null);
    setListItems([]);
  }
}, [currentRepoPath]);
```

**更新加载逻辑**：
- 在初始加载和 tab 切换时添加 `currentRepoPath` 依赖
- 确保仓库切换时总是加载新数据

### 2. 修改 useRepository.ts

**在仓库切换时清除 store 数据**：
- 导入 `useTaskStore` 和 `useReviewStore`
- 在 `loadRepository` 中清除所有仓库相关数据

```typescript
const { setTasks, setReviewStats, setHeatmap, setChecklist } = useTaskStore();
const { resetAll } = useReviewStore();

if (result) {
  // Clear all repository-specific data before setting new repo
  setTasks([]);
  setReviewStats(null as any);
  setHeatmap([]);
  setChecklist([]);
  resetAll();
  
  setCurrentRepo(result);
}
```

## 修改的文件

1. **frontend/components/RightPanel.tsx**
   - 添加 `currentRepoPath` 依赖
   - 在仓库切换时清除所有面板数据
   - 更新初始加载和 tab 切换逻辑

2. **frontend/hooks/useRepository.ts**
   - 导入 store hooks
   - 在 `loadRepository` 中清除 task store 数据
   - 在 `clearRepository` 中清除相关数据

## 测试验证

✅ 构建成功：TypeScript 编译通过，Vite 构建成功  
✅ 仓库切换时热力图正确清除  
✅ 新仓库的热力图数据正确加载  
✅ 所有相关数据（blame、stats、checklist）正确清除和更新

## 影响范围

此修复确保：
- ✅ 热力图数据在新仓库加载时正确清除和更新
- ✅ 右侧面板的所有标签页数据都能正确清除
- ✅ store 中的任务、统计数据、清单等数据在仓库切换时保持同步
- ✅ 不会影响其他功能正常工作

## 总结

通过添加仓库路径作为依赖并在仓库切换时主动清除相关数据，成功解决了热力图显示旧仓库数据的问题。现在当用户选择新仓库时，所有相关数据都会被正确清除并加载新仓库的数据。
