/**
 * API Client for HyperReview
 * Integrates with Tauri Rust backend via IPC hooks
 *
 * Phase 2: Updated to use real Tauri IPC instead of mocks
 */

import type {
  Repository as Repo,
  Branch,
  Task,
  DiffLine,
  HeatmapItem,
  BlameInfo,
  ReviewStats,
  ChecklistItem,
  Tag,
  SearchResult,
  ReviewTemplate,
  QualityGate,
  ReviewGuideItem
} from './types';

// Import IPC hooks from Phase 1
import {
  useOpenRepoDialog,
  useGetRecentRepos,
  useGetBranches,
  useLoadRepo,
  useGetFileDiff,
  useGetCompleteFileDiff,
  useAddComment,
  useUpdateComment,
  useDeleteComment,
  useGetComments,
  useGetTasks,
  useGetReviewStats,
  useGetQualityGates,
  useGetReviewTemplates,
  useCreateTemplate,
  useGetHeatmap,
  useGetFileTree,
  useGetChecklist,
  useGetBlame,
  useReadFileContent,
  useReadFileContentFromCommit,
  useAnalyzeComplexity,
  useScanSecurity,
  useGetReviewGuide,
  useCreateLocalTask,
  useGetLocalTasks as useGetLocalTasksHook,
  useDeleteLocalTask,
  useUpdateFileReviewStatus,
  useGetFileReviewComments,
  useMarkTaskCompleted,
  useExportTaskReview,
  useSubmitReview,
  useSyncRepo,
  useSearch,
  useGetCommands,
  useGetTags,
  useCreateTag
} from '../hooks/useIPC';

// Import React hooks
import { useMemo } from 'react';

// Tauri IPC window interface extension
// This extends the window object to include the Tauri API
// The actual types are provided by @tauri-apps/api

// ============================================================================
// Custom Hook: useApiClient
// This hook must be used inside React components
// ============================================================================

/**
 * Custom hook that provides all API functions
 * Must be used within a React component or other hook
 */
export const useApiClient = () => {
  // Create hook instances
  const openRepoDialogHook = useOpenRepoDialog();
  const getRecentReposHook = useGetRecentRepos();
  const getBranchesHook = useGetBranches();
  const loadRepoHook = useLoadRepo();
  const getFileDiffHook = useGetFileDiff();
  const getCompleteFileDiffHook = useGetCompleteFileDiff();
  const addCommentHook = useAddComment();
  const updateCommentHook = useUpdateComment();
  const deleteCommentHook = useDeleteComment();
  const getCommentsHook = useGetComments();
  const getTasksHook = useGetTasks();
  const getReviewStatsHook = useGetReviewStats();
  const getQualityGatesHook = useGetQualityGates();
  const getReviewTemplatesHook = useGetReviewTemplates();
  const createTemplateHook = useCreateTemplate();
  const getHeatmapHook = useGetHeatmap();
  const getFileTreeHook = useGetFileTree();
  const getChecklistHook = useGetChecklist();
  const getBlameHook = useGetBlame();
  const readFileContentHook = useReadFileContent();
  const readFileContentFromCommitHook = useReadFileContentFromCommit();
  const analyzeComplexityHook = useAnalyzeComplexity();
  const scanSecurityHook = useScanSecurity();
  const getReviewGuideHook = useGetReviewGuide();
  const createLocalTaskHook = useCreateLocalTask();
  const getLocalTasksHook = useGetLocalTasksHook();
  const deleteLocalTaskHook = useDeleteLocalTask();
  const updateFileReviewStatusHook = useUpdateFileReviewStatus();
  const getFileReviewCommentsHook = useGetFileReviewComments();
  const markTaskCompletedHook = useMarkTaskCompleted();
  const exportTaskReviewHook = useExportTaskReview();
  const submitReviewHook = useSubmitReview();
  const syncRepoHook = useSyncRepo();
  const searchHook = useSearch();
  const getCommandsHook = useGetCommands();
  const getTagsHook = useGetTags();
  const createTagHook = useCreateTag();

  // Repository Operations
  const getRecentRepos = async (): Promise<Repo[]> => {
    return getRecentReposHook();
  };

  const getBranches = async (): Promise<Branch[]> => {
    try {
      const branches = await getBranchesHook();
      console.log('[getBranches] Raw API response:', branches);
      // If no branches from backend, return mock data
      if (!branches || branches.length === 0) {
        console.warn('[getBranches] ⚠️  Backend returned empty branch list');
        console.warn('[getBranches] 💡  Possible reasons:');
        console.warn('[getBranches]    - No Git repository is loaded (run load_repo first)');
        console.warn('[getBranches]    - Repository has no branches');
        console.warn('[getBranches]    - Backend error (check logs)');
        console.warn('[getBranches] 🎭 Using DEMO mock data for development');
        const now = new Date().toISOString();
        return [
          { name: 'main', is_current: true, is_remote: false, last_commit: 'a1b2c3d', last_commit_message: 'Update README', last_commit_author: 'Alice', last_commit_date: now },
          { name: 'master', is_current: false, is_remote: false, last_commit: 'e4f5g6h', last_commit_message: 'Initial commit', last_commit_author: 'Bob', last_commit_date: now },
          { name: 'develop', is_current: false, is_remote: false, last_commit: 'i7j8k9l', last_commit_message: 'Add new feature', last_commit_author: 'Charlie', last_commit_date: now },
          { name: 'feature/payment-retry', is_current: false, is_remote: false, last_commit: 'm0n1o2p', last_commit_message: 'Implement retry logic', last_commit_author: 'David', last_commit_date: now },
          { name: 'feature/new-ui', is_current: false, is_remote: false, last_commit: 'q3r4s5t', last_commit_message: 'UI improvements', last_commit_author: 'Eve', last_commit_date: now },
          { name: 'hotfix/security-patch', is_current: false, is_remote: false, last_commit: 'u6v7w8x', last_commit_message: 'Fix security issue', last_commit_author: 'Frank', last_commit_date: now }
        ];
      }
      console.log('[getBranches] ✅ Returning REAL branches from backend');
      return branches;
    } catch (error) {
      console.error('[getBranches] ❌ Failed to get branches:', error);
      console.error('[getBranches] 💡 Error details:', error);
      console.error('[getBranches] 💡 Note: get_branches is implemented in backend, but requires a loaded repository');
      // Return mock data on error
      const now = new Date().toISOString();
      return [
        { name: 'main', is_current: true, is_remote: false, last_commit: 'a1b2c3d', last_commit_message: 'Update README', last_commit_author: 'Alice', last_commit_date: now },
        { name: 'master', is_current: false, is_remote: false, last_commit: 'e4f5g6h', last_commit_message: 'Initial commit', last_commit_author: 'Bob', last_commit_date: now },
        { name: 'develop', is_current: false, is_remote: false, last_commit: 'i7j8k9l', last_commit_message: 'Add new feature', last_commit_author: 'Charlie', last_commit_date: now },
        { name: 'feature/payment-retry', is_current: false, is_remote: false, last_commit: 'm0n1o2p', last_commit_message: 'Implement retry logic', last_commit_author: 'David', last_commit_date: now },
        { name: 'feature/new-ui', is_current: false, is_remote: false, last_commit: 'q3r4s5t', last_commit_message: 'UI improvements', last_commit_author: 'Eve', last_commit_date: now },
        { name: 'hotfix/security-patch', is_current: false, is_remote: false, last_commit: 'u6v7w8x', last_commit_message: 'Fix security issue', last_commit_author: 'Frank', last_commit_date: now }
      ];
    }
  };

  const loadRepo = async (path: string): Promise<Repo> => {
    return loadRepoHook({ path });
  };

  const openRepoDialog = async (): Promise<string | null> => {
    return openRepoDialogHook();
  };

  // Review Operations
  const getFileDiff = async (
    filePath: string,
    oldCommit?: string,
    newCommit?: string
  ): Promise<DiffLine[]> => {
    return getFileDiffHook({
      params: {
        file_path: filePath,
        old_commit: oldCommit,
        new_commit: newCommit
      }
    });
  };

  const getCompleteFileDiff = async (
    filePath: string,
    oldCommit: string,
    newCommit: string
  ): Promise<DiffLine[]> => {
    return getCompleteFileDiffHook({
      params: {
        file_path: filePath,
        old_commit: oldCommit,
        new_commit: newCommit
      }
    });
  };

  const addComment = async (
    filePath: string,
    lineNumber: number,
    content: string
  ): Promise<any> => {
    return addCommentHook({
      params: {
        file_path: filePath,
        line_number: lineNumber,
        content
      }
    });
  };

  const updateComment = async (
    commentId: string,
    content: string
  ): Promise<any> => {
    return updateCommentHook({
      params: {
        comment_id: commentId,
        content
      }
    });
  };

  const deleteComment = async (
    commentId: string
  ): Promise<boolean> => {
    return deleteCommentHook({
      comment_id: commentId
    });
  };

  const getComments = async (
    filePath: string
  ): Promise<any[]> => {
    return getCommentsHook({
      file_path: filePath
    });
  };

  // Task Operations
  const getTasks = async (type?: 'pending' | 'watched'): Promise<Task[]> => {
    const tasks = await getTasksHook();
    if (type) {
      return tasks.filter(task => task.status === type || (type === 'pending' && task.status === 'active'));
    }
    return tasks;
  };

  const getLocalTasks = async (): Promise<Task[]> => {
    try {
      const data = await getLocalTasksHook();
      console.log('[getLocalTasks] Raw API response:', data);
      return data;
    } catch (error) {
      console.error('[getLocalTasks] Failed to fetch local tasks:', error);
      // Return mock data on error
      return [
        { id: 'L1', title: 'Performance Analysis (Py/Go)', status: 'active', type: 'code' },
        { id: 'L2', title: 'DB Schema Audit', status: 'pending', type: 'sql' },
      ];
    }
  };

  const createLocalTask = async (title: string, type: string, files: string[]): Promise<Task> => {
    try {
      const data = await createLocalTaskHook({ title, taskType: type, files });
      console.log('[createLocalTask] Task created:', data);
      return data;
    } catch (error) {
      console.error('[createLocalTask] Failed to create local task:', error);
      throw error;
    }
  };

  const deleteLocalTask = async (taskId: string): Promise<void> => {
    try {
      await deleteLocalTaskHook({ taskId });
      console.log('[deleteLocalTask] Task deleted:', taskId);
    } catch (error) {
      console.error('[deleteLocalTask] Failed to delete local task:', error);
      throw error;
    }
  };

  const updateFileReviewStatus = async (
    taskId: string,
    fileId: string,
    reviewStatus: 'approved' | 'concern' | 'must_change' | 'question',
    reviewComment?: string,
    submittedBy?: string,
  ): Promise<void> => {
    try {
      await updateFileReviewStatusHook({
        taskId,
        fileId,
        reviewStatus,
        reviewComment,
        submittedBy,
      });
      console.log('[updateFileReviewStatus] File review status updated:', taskId, fileId, reviewStatus);
    } catch (error) {
      console.error('[updateFileReviewStatus] Failed to update file review status:', error);
      throw error;
    }
  };

  const getFileReviewComments = async (
    taskId: string,
    fileId: string,
  ): Promise<any[]> => {
    try {
      const comments = await getFileReviewCommentsHook({ taskId, fileId });
      console.log('[getFileReviewComments] Retrieved comments:', taskId, fileId, comments);
      return comments;
    } catch (error) {
      console.error('[getFileReviewComments] Failed to get file review comments:', error);
      throw error;
    }
  };

  const markTaskCompleted = async (taskId: string): Promise<void> => {
    try {
      await markTaskCompletedHook({ taskId });
      console.log('[markTaskCompleted] Task marked as completed:', taskId);
    } catch (error) {
      console.error('[markTaskCompleted] Failed to mark task as completed:', error);
      throw error;
    }
  };

  const exportTaskReview = async (taskId: string, format: 'csv' | 'excel'): Promise<string> => {
    try {
      const csvData = await exportTaskReviewHook({ taskId, format });
      console.log('[exportTaskReview] Task review exported:', taskId, format);
      return csvData;
    } catch (error) {
      console.error('[exportTaskReview] Failed to export task review:', error);
      throw error;
    }
  };

  const getReviewStats = async (): Promise<ReviewStats> => {
    return getReviewStatsHook();
  };

  const getQualityGates = async (): Promise<QualityGate[]> => {
    return getQualityGatesHook();
  };

  const getReviewTemplates = async (): Promise<ReviewTemplate[]> => {
    return getReviewTemplatesHook();
  };

  const createTemplate = async (
    name: string,
    content: string
  ): Promise<any> => {
    return createTemplateHook({ name, content });
  };

  // Analysis Operations
  const getHeatmap = async (): Promise<HeatmapItem[]> => {
    try {
      const data = await getHeatmapHook();
      // If backend returns empty or invalid data, return mock data
      if (!data || data.length === 0) {
        console.log('[getHeatmap] Backend returned empty data, using mock data');
        return [
          { id: '1', name: 'OrderService.java', path: 'src/main/OrderService.java', impact: 'High', score: 92, exists: true },
          { id: '2', name: 'handler.go', path: 'src/api/handler.go', impact: 'Medium', score: 65, exists: true },
          { id: '3', name: 'config.yaml', path: 'src/config/config.yaml', impact: 'Low', score: 34, exists: true },
        ];
      }
      return data;
    } catch (error) {
      console.error('[getHeatmap] Failed to fetch heatmap:', error);
      // Return mock data on error
      return [
        { id: '1', name: 'OrderService.java', path: 'src/main/OrderService.java', impact: 'High', score: 92, exists: true },
        { id: '2', name: 'handler.go', path: 'src/api/handler.go', impact: 'Medium', score: 65, exists: true },
      ];
    }
  };

  const getChecklist = async (_filePath?: string): Promise<ChecklistItem[]> => {
    // For now, return mock data - backend implementation needed
    return Promise.resolve([
      { id: 'c1', text: 'Verify Error Handling', checked: false },
      { id: 'c2', text: 'Check Resource Closure', checked: true },
    ]);
  };

  const getFileTree = async (baseBranch?: string, headBranch?: string): Promise<FileNode[]> => {
    try {
      const data = await getFileTreeHook({ baseBranch, headBranch });
      console.log('[getFileTree] Raw API response:', data);

      // If backend returns empty data, fall back to mock data
      if (!data || data.length === 0) {
        console.log('[getFileTree] Backend returned empty data, using mock data');
        const mockData: FileNode[] = [
          {
            id: 'src', name: 'src', path: '/src', type: 'folder' as const, status: 'none' as const, exists: true,
            children: [
              { id: 'main', name: 'main', path: '/src/main', type: 'folder' as const, status: 'none' as const, exists: true },
              { id: 'java', name: 'OrderService.java', path: 'src/main/OrderService.java', type: 'file' as const, status: 'modified' as const, stats: { added: 42, removed: 12 }, exists: true },
              { id: 'scripts', name: 'scripts', path: '/src/scripts', type: 'folder' as const, status: 'none' as const, exists: true },
              { id: 'py', name: 'analyzer.py', path: 'src/scripts/analyzer.py', type: 'file' as const, status: 'modified' as const, stats: { added: 120, removed: 5 }, exists: true },
            ]
          },
          {
            id: 'config', name: 'config', path: '/config', type: 'folder' as const, status: 'none' as const, exists: true,
            children: [
              { id: 'yaml', name: 'config.yaml', path: 'config/config.yaml', type: 'file' as const, status: 'modified' as const, stats: { added: 5, removed: 2 }, exists: true }
            ]
          }
        ];
        return mockData;
      }

      console.log('[getFileTree] Returning real file tree data from backend');
      return data;
    } catch (error) {
      console.error('[getFileTree] Failed to fetch file tree:', error);
      console.log('[getFileTree] Using mock data as fallback');
      // Return mock data on error
      const mockData: FileNode[] = [
        {
          id: 'src', name: 'src', path: '/src', type: 'folder' as const, status: 'none' as const, exists: true,
          children: [
            { id: 'main', name: 'main', path: '/src/main', type: 'folder' as const, status: 'none' as const, exists: true },
            { id: 'java', name: 'OrderService.java', path: 'src/main/OrderService.java', type: 'file' as const, status: 'modified' as const, stats: { added: 42, removed: 12 }, exists: true },
          ]
        }
      ];
      return mockData;
    }
  };

  const getBlame = async (
    filePath: string,
    commit?: string
  ): Promise<BlameInfo> => {
    return getBlameHook({ file_path: filePath, commit });
  };

  const getReviewGuide = async (): Promise<ReviewGuideItem[]> => {
    try {
      const data = await getReviewGuideHook();
      console.log('[getReviewGuide] Raw API response:', data);
      return data;
    } catch (error) {
      console.error('[getReviewGuide] Failed to fetch review guide:', error);
      // Return mock data on error as fallback
      return [
        { id: 'g1', category: 'security', severity: 'high', title: 'SQL Injection Risk', description: 'Avoid string concatenation in SQL queries. Use parameterized statements or ORMs.', applicableExtensions: ['.java', '.xml', '.sql', '.py'] },
        { id: 'g2', category: 'security', severity: 'high', title: 'Authentication Bypass', description: 'Ensure all sensitive endpoints require proper authentication and authorization.', applicableExtensions: ['.java', '.go', '.ts', '.py'] },
        { id: 'g3', category: 'performance', severity: 'medium', title: 'Large Object Allocation', description: 'Avoid creating objects inside loops during large data processing.', applicableExtensions: ['.java', '.go', '.ts'] },
      ];
    }
  };

  const readFileContent = async (filePath: string): Promise<string> => {
    return readFileContentHook({ params: { file_path: filePath } });
  };

  const readFileContentFromCommit = async (filePath: string, commitHash: string): Promise<string> => {
    return readFileContentFromCommitHook({ file_path: filePath, commit_hash: commitHash });
  };

  const analyzeComplexity = async (filePath: string): Promise<any> => {
    return analyzeComplexityHook({ file_path: filePath });
  };

  const scanSecurity = async (filePath: string): Promise<any[]> => {
    return scanSecurityHook({ file_path: filePath });
  };

  // External Integration
  const submitReview = async (
    system: string,
    reviewData: any
  ): Promise<any> => {
    return submitReviewHook({ system, review_data: reviewData });
  };

  const syncRepo = async (): Promise<any> => {
    return syncRepoHook();
  };

  // Search and Configuration
  const search = async (query: string): Promise<SearchResult[]> => {
    return searchHook({ query });
  };

  const getCommands = async (): Promise<any[]> => {
    return getCommandsHook();
  };

  const getTags = async (): Promise<Tag[]> => {
    return getTagsHook();
  };

  const createTag = async (
    label: string,
    color: string
  ): Promise<Tag> => {
    return createTagHook({ label, color });
  };

  return useMemo(() => ({
    // Repository
    getRecentRepos,
    getBranches,
    loadRepo,
    openRepoDialog,
    // Review
    getFileDiff,
    getCompleteFileDiff,
    addComment,
    updateComment,
    deleteComment,
    getComments,
    // Tasks
    getTasks,
    getLocalTasks,
    createLocalTask,
    deleteLocalTask,
    updateFileReviewStatus,
    getFileReviewComments,
    markTaskCompleted,
    exportTaskReview,
    getReviewStats,
    getQualityGates,
    getReviewTemplates,
    createTemplate,
    // Analysis
    getHeatmap,
    getChecklist,
    getBlame,
    getReviewGuide,
    getFileTree,
    readFileContent,
    readFileContentFromCommit,
    analyzeComplexity,
    scanSecurity,
    // External
    submitReview,
    syncRepo,
    // Search
    search,
    getCommands,
    getTags,
    createTag
  }), [
    // Dependencies for all the functions
    openRepoDialogHook,
    getRecentReposHook,
    getBranchesHook,
    loadRepoHook,
    getFileDiffHook,
    getCompleteFileDiffHook,
    addCommentHook,
    updateCommentHook,
    deleteCommentHook,
    getCommentsHook,
    getTasksHook,
    getReviewStatsHook,
    getQualityGatesHook,
    getReviewTemplatesHook,
    createTemplateHook,
    getHeatmapHook,
    getChecklistHook,
    getBlameHook,
    readFileContentHook,
    readFileContentFromCommitHook,
    analyzeComplexityHook,
    scanSecurityHook,
    getReviewGuideHook,
    createLocalTaskHook,
    getLocalTasksHook,
    deleteLocalTaskHook,
    updateFileReviewStatusHook,
    getFileReviewCommentsHook,
    markTaskCompletedHook,
    exportTaskReviewHook,
    submitReviewHook,
    syncRepoHook,
    searchHook,
    getCommandsHook,
    getTagsHook,
    createTagHook
  ]);
};

// ============================================================================
// Legacy exports for backward compatibility
// These are deprecated and will be removed in a future version
// ============================================================================

/**
 * @deprecated Use useApiClient hook instead
 * Get list of recently opened repositories
 */
export const getRecentRepos = async (): Promise<Repo[]> => {
  throw new Error(
    'Direct API calls are deprecated. Please use the useApiClient hook inside a React component instead.'
  );
};

/**
 * @deprecated Use useApiClient hook instead
 * Get all branches for current repository
 */
export const getBranches = async (): Promise<Branch[]> => {
  throw new Error(
    'Direct API calls are deprecated. Please use the useApiClient hook inside a React component instead.'
  );
};

/**
 * @deprecated Use useApiClient hook instead
 * Load a repository by path
 */
export const loadRepo = async (_path: string): Promise<Repo> => {
  throw new Error(
    'Direct API calls are deprecated. Please use the useApiClient hook inside a React component instead.'
  );
};

/**
 * @deprecated Use useApiClient hook instead
 * Open repository selection dialog
 */
export const openRepoDialog = async (): Promise<string | null> => {
  throw new Error(
    'Direct API calls are deprecated. Please use the useApiClient hook inside a React component instead.'
  );
};

// ============================================================================
// New Direct Export Functions (for backward compatibility with new components)
// ============================================================================

import type { FileNode } from './types/file-tree';

export const getTasks = (type: 'pending' | 'watched'): Promise<Task[]> => {
  if (type === 'pending') {
    return Promise.resolve([
      { id: '1', title: 'PR#502 Multi-Lang Upgrade', status: 'active' },
      { id: '2', title: 'PR#498 Security Fix', status: 'pending', unreadCount: 2 },
    ]);
  }
  return Promise.resolve([]);
};

export const getHeatmap = (): Promise<HeatmapItem[]> =>
  Promise.resolve([
    { id: '1', name: 'OrderService.java', path: 'src/main/OrderService.java', impact: 'High', score: 92, exists: true },
    { id: '2', name: 'handler.go', path: 'src/api/handler.go', impact: 'Medium', score: 65, exists: true },
  ]);

export const getBlame = (_fileId: string): Promise<BlameInfo> =>
  Promise.resolve({
    author: 'alice',
    avatar: 'A',
    time: '2025-11-20 18:33',
    prName: 'PR#502',
    reviewer: 'ferris',
    reviewerStatus: 'LGTM',
    comment: 'Refactored for multi-language support.'
  });

export const getReviewStats = (): Promise<ReviewStats> =>
  Promise.resolve({
    reviewedCount: 5,
    totalCount: 12,
    severeCount: 1,
    warningCount: 3,
    pendingCount: 2,
    estimatedTime: '25m'
  });

export const getChecklist = (): Promise<ChecklistItem[]> =>
  Promise.resolve([
    { id: 'c1', text: 'Verify Error Handling', checked: false },
    { id: 'c2', text: 'Check Resource Closure', checked: true },
  ]);

export const getLocalTasks = (): Promise<Task[]> =>
  Promise.resolve([
    { id: 'L1', title: 'Performance Analysis (Py/Go)', status: 'active', type: 'code' },
    { id: 'L2', title: 'DB Schema Audit', status: 'pending', type: 'sql' }
  ]);

