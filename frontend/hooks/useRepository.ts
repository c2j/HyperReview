/**
 * Repository State Management Hooks
 * Manages repository, branches, and recent repositories state
 */

import { useState, useEffect, useCallback } from 'react';
import { useReviewStore } from '../store/reviewStore';
import { useApiClient } from '../api/client';
import { handleAsyncErrorWithToast } from '../utils/errorHandler';
import { useLoading } from '../context/LoadingContext';
import { useErrorStore, ErrorFormatter, ErrorSeverity } from '../utils/errorHandler';

// ============================================================================
// Repository Loading Hook
// ============================================================================

/**
 * Hook for loading and managing current repository
 */
export const useCurrentRepository = () => {
  const {
    currentRepo,
    loading,
    error,
    setCurrentRepo,
    clearError,
    setTasks,
    setReviewStats,
    setHeatmap,
    setChecklist,
    resetAll,
    // setLoading and setError are not used directly, handled by setRepositoryLoading and handleAsyncErrorWithToast
  } = useReviewStore();

  const { setRepositoryLoading } = useLoading();
  const { loadRepo } = useApiClient();

  const loadRepository = useCallback(
    async (path: string) => {
      clearError();
      setRepositoryLoading(true);

      const result = await handleAsyncErrorWithToast(
        async () => {
          const repo = await loadRepo(path);
          return repo;
        },
        `Repository loaded: ${path.split('/').pop()}`,
      );

      setRepositoryLoading(false);

      if (result) {
        // Clear all repository-specific data before setting new repo
        setTasks([]);
        setReviewStats(null as any);
        setHeatmap([]);
        setChecklist([]);
        resetAll();

        setCurrentRepo(result);
      }

      return result;
    },
    [
      clearError,
      setRepositoryLoading,
      setCurrentRepo,
      setTasks,
      setReviewStats,
      setHeatmap,
      setChecklist,
      resetAll,
      handleAsyncErrorWithToast,
    ],
  );

  const clearRepository = () => {
    setCurrentRepo(null);
    clearError();
    // Also clear all related data
    setTasks([]);
    setReviewStats(null as any);
    setHeatmap([]);
    setChecklist([]);
  };

  return {
    currentRepo,
    loading,
    error,
    loadRepository,
    clearRepository,
  };
};

// ============================================================================
// Recent Repositories Hook
// ============================================================================

/**
 * Hook for managing recent repositories list
 */
export const useRecentRepositories = () => {
  const {
    recentRepos,
    loading,
    error,
    setRecentRepos,
    clearError,
    // setLoading and setError are not used directly, handled by setRepositoryLoading and handleAsyncErrorWithToast
  } = useReviewStore();

  const { setRepositoryLoading } = useLoading();
  const { getRecentRepos } = useApiClient();

  const loadRecentRepos = useCallback(async () => {
    clearError();
    setRepositoryLoading(true);

    const result = await handleAsyncErrorWithToast(async () => {
      const repos = await getRecentRepos();
      return repos;
    }, 'Recent repositories loaded');

    setRepositoryLoading(false);

    if (result) {
      setRecentRepos(result);
    }

    return result;
  }, [clearError, setRepositoryLoading, setRecentRepos, handleAsyncErrorWithToast, getRecentRepos]);

  const refreshRecentRepos = useCallback(async () => {
    return loadRecentRepos();
  }, [loadRecentRepos]);

  return {
    recentRepos,
    loading,
    error,
    loadRecentRepos,
    refreshRecentRepos,
  };
};

// ============================================================================
// Branches Hook
// ============================================================================

/**
 * Hook for managing repository branches
 */
export const useBranches = () => {
  const {
    branches,
    loading,
    error,
    setBranches,
    clearError,
    // setLoading and setError are not used directly, handled by setRepositoryLoading and handleAsyncErrorWithToast
  } = useReviewStore();

  const { setRepositoryLoading } = useLoading();
  const { getBranches } = useApiClient();

  const loadBranches = useCallback(async () => {
    clearError();
    setRepositoryLoading(true);

    const result = await handleAsyncErrorWithToast(async () => {
      const branchesList = await getBranches();
      return branchesList;
    }, 'Branches loaded');

    setRepositoryLoading(false);

    if (result) {
      setBranches(result);
    }

    return result;
  }, [clearError, setRepositoryLoading, setBranches, handleAsyncErrorWithToast, getBranches]);

  const refreshBranches = useCallback(async () => {
    return loadBranches();
  }, [loadBranches]);

  // Get current branch
  const currentBranch = branches.find((branch) => branch.is_current);

  // Filter remote branches
  const remoteBranches = branches.filter((branch) => branch.is_remote);

  // Filter local branches
  const localBranches = branches.filter((branch) => !branch.is_remote);

  return {
    branches,
    currentBranch,
    localBranches,
    remoteBranches,
    loading,
    error,
    loadBranches,
    refreshBranches,
  };
};

// ============================================================================
// Repository Dialog Hook
// ============================================================================

/**
 * Hook for opening repository selection dialog
 */
export const useRepoDialog = () => {
  const { setRepositoryLoading } = useLoading();
  const { openRepoDialog } = useApiClient();

  const openDialog = useCallback(async (): Promise<string | null> => {
    setRepositoryLoading(true);

    try {
      const path = await openRepoDialog();

      if (path) {
        // Only show success if we got a path
        useErrorStore.getState().showToast({
          severity: ErrorSeverity.SUCCESS,
          title: 'Success',
          message: 'Repository selected successfully',
        });
      }

      return path;
    } catch (error) {
      // Handle errors
      const formatted = ErrorFormatter.format(error);
      useErrorStore.getState().showToast({
        severity: formatted.severity,
        title: formatted.title,
        message: formatted.message,
      });
      return null;
    } finally {
      setRepositoryLoading(false);
    }
  }, [setRepositoryLoading, openRepoDialog]);

  return { openDialog };
};

// ============================================================================
// Repository Actions Hook
// ============================================================================

/**
 * Combined hook for repository actions
 */
export const useRepositoryActions = () => {
  const { currentRepo, loadRepository, clearRepository } = useCurrentRepository();
  const { recentRepos, loadRecentRepos } = useRecentRepositories();
  const { branches, loadBranches, currentBranch } = useBranches();
  const { openDialog } = useRepoDialog();

  // Open repository from dialog
  const openRepository = useCallback(async () => {
    const path = await openDialog();
    if (path) {
      const repo = await loadRepository(path);
      if (repo) {
        // Load branches after successful repo load
        await loadBranches();
        // Refresh recent repos
        await loadRecentRepos();
      }
      return repo;
    }
    return null;
  }, [openDialog, loadRepository, loadBranches, loadRecentRepos]);

  // Switch to a different repository
  const switchRepository = useCallback(
    async (path: string) => {
      const repo = await loadRepository(path);
      if (repo) {
        await loadBranches();
        await loadRecentRepos();
      }
      return repo;
    },
    [loadRepository, loadBranches, loadRecentRepos],
  );

  // Refresh current repository data
  const refreshRepository = useCallback(async () => {
    if (currentRepo) {
      await loadBranches();
      await loadRecentRepos();
    }
  }, [currentRepo, loadBranches, loadRecentRepos]);

  return {
    // State
    currentRepo,
    recentRepos,
    branches,
    currentBranch,

    // Actions
    openRepository,
    switchRepository,
    loadRepository,
    clearRepository,
    refreshRepository,

    // Utilities
    openDialog,
    loadRecentRepos,
    loadBranches,
  };
};

// ============================================================================
// Repository Status Hook
// ============================================================================

/**
 * Hook for checking repository status and health
 */
export const useRepositoryStatus = () => {
  const { currentRepo, branches } = useReviewStore();

  const isRepoLoaded = currentRepo !== null;
  const isBranchSelected = branches.some((branch) => branch.is_current);

  const getRepositoryInfo = useCallback(() => {
    if (!currentRepo) {
      return null;
    }

    return {
      name: currentRepo.path.split('/').pop() || 'Unknown',
      path: currentRepo.path,
      currentBranch: currentRepo.current_branch,
      headCommit: currentRepo.head_commit,
      isActive: currentRepo.is_active,
      branchCount: branches.length,
      lastCommitDate: currentRepo.last_opened,
    };
  }, [currentRepo, branches]);

  return {
    isRepoLoaded,
    isBranchSelected,
    getRepositoryInfo,
  };
};

// ============================================================================
// Repository Initialization Hook
// ============================================================================

/**
 * Hook for initializing repository on app start
 */
export const useInitializeRepository = () => {
  const { loadRecentRepos, openRepository } = useRepositoryActions();
  const [initialized, setInitialized] = useState(false);
  const [showDialog, setShowDialog] = useState(false);

  useEffect(() => {
    const init = async () => {
      await loadRecentRepos();
      setInitialized(true);
      setShowDialog(true);
    };

    init();
  }, [loadRecentRepos]);

  const handleRepositorySelected = useCallback(async () => {
    setShowDialog(false);
    await openRepository();
  }, [setShowDialog, openRepository]);

  const handleSkip = useCallback(() => {
    setShowDialog(false);
  }, [setShowDialog]);

  return {
    initialized,
    showDialog,
    handleRepositorySelected,
    handleSkip,
  };
};
