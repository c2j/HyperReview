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
  QualityGate
} from './types';

// Import IPC hooks from Phase 1
import {
  useOpenRepoDialog,
  useGetRecentRepos,
  useGetBranches,
  useLoadRepo,
  useGetFileDiff,
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
  useGetChecklist,
  useGetBlame,
  useReadFileContent,
  useAnalyzeComplexity,
  useScanSecurity,
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
declare global {
  interface Window {
    __TAURI__: any;
  }
}

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
  const getChecklistHook = useGetChecklist();
  const getBlameHook = useGetBlame();
  const readFileContentHook = useReadFileContent();
  const analyzeComplexityHook = useAnalyzeComplexity();
  const scanSecurityHook = useScanSecurity();
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
    return getBranchesHook();
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
  const getTasks = async (): Promise<Task[]> => {
    return getTasksHook();
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
    return getHeatmapHook();
  };

  const getChecklist = async (filePath: string): Promise<ChecklistItem[]> => {
    return getChecklistHook({ file_path: filePath });
  };

  const getBlame = async (
    filePath: string,
    commit?: string
  ): Promise<BlameInfo> => {
    return getBlameHook({ file_path: filePath, commit });
  };

  const readFileContent = async (filePath: string): Promise<string> => {
    return readFileContentHook({ params: { file_path: filePath } });
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
    addComment,
    updateComment,
    deleteComment,
    getComments,
    // Tasks
    getTasks,
    getReviewStats,
    getQualityGates,
    getReviewTemplates,
    createTemplate,
    // Analysis
    getHeatmap,
    getChecklist,
    getBlame,
    readFileContent,
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
    analyzeComplexityHook,
    scanSecurityHook,
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
