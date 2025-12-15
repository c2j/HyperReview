/**
 * Helper functions for loading state management
 * Extracted to separate file to avoid TSX interpretation issues
 */


// ============================================================================
// Type definitions for loading helpers
// ============================================================================

export type AsyncOperation<T> = () => Promise<T>;
export type LoadingSetter = (loading: boolean) => void;

// ============================================================================
// Loading helper functions
// ============================================================================

export const createWithLoading = (
  setRepositoryLoading: LoadingSetter,
  setDiffLoading: LoadingSetter,
  setTaskLoading: LoadingSetter,
  setAnalysisLoading: LoadingSetter
) => {
  /**
   * Wrap an async operation with loading state management
   */
  const withLoading = async <T>(
    operation: AsyncOperation<T>,
    loadingSetter: LoadingSetter
  ): Promise<T | null> => {
    loadingSetter(true);
    try {
      const result = await operation();
      return result;
    } catch (error) {
      // Error will be handled by IPC error handler
      console.error('Operation failed:', error);
      return null;
    } finally {
      loadingSetter(false);
    }
  };

  /**
   * Wrap operation with repository loading state
   */
  const withRepositoryLoading = <T>(operation: AsyncOperation<T>) =>
    withLoading(operation, setRepositoryLoading);

  /**
   * Wrap operation with diff loading state
   */
  const withDiffLoading = <T>(operation: AsyncOperation<T>) =>
    withLoading(operation, setDiffLoading);

  /**
   * Wrap operation with task loading state
   */
  const withTaskLoading = <T>(operation: AsyncOperation<T>) =>
    withLoading(operation, setTaskLoading);

  /**
   * Wrap operation with analysis loading state
   */
  const withAnalysisLoading = <T>(operation: AsyncOperation<T>) =>
    withLoading(operation, setAnalysisLoading);

  return {
    withRepositoryLoading,
    withDiffLoading,
    withTaskLoading,
    withAnalysisLoading
  };
};

// ============================================================================
// Export convenience types
// ============================================================================

export type WithLoadingHelpers = ReturnType<typeof createWithLoading>;
