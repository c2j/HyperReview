import React, { createContext, useContext, useState, ReactNode } from 'react';
import { useErrorStore } from '../utils/errorHandler';
import { createWithLoading } from './LoadingHelpers';

export interface LoadingContextValue {
  // Global loading states
  isRepositoryLoading: boolean;
  isDiffLoading: boolean;
  isTaskLoading: boolean;
  isAnalysisLoading: boolean;

  // Actions
  setRepositoryLoading: (loading: boolean) => void;
  setDiffLoading: (loading: boolean) => void;
  setTaskLoading: (loading: boolean) => void;
  setAnalysisLoading: (loading: boolean) => void;

  // Combined state
  isAnyLoading: boolean;
}

const LoadingContext = createContext<LoadingContextValue | undefined>(undefined);

export const LoadingProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [repositoryLoading, setRepositoryLoading] = useState(false);
  const [diffLoading, setDiffLoading] = useState(false);
  const [taskLoading, setTaskLoading] = useState(false);
  const [analysisLoading, setAnalysisLoading] = useState(false);

  // Update global loading state in error store
  const updateGlobalLoading = () => {
    const isAnyLoading =
      repositoryLoading ||
      diffLoading ||
      taskLoading ||
      analysisLoading;

    useErrorStore.getState().setLoading(isAnyLoading);
  };

  const handleSetRepositoryLoading = (loading: boolean) => {
    setRepositoryLoading(loading);
    updateGlobalLoading();
  };

  const handleSetDiffLoading = (loading: boolean) => {
    setDiffLoading(loading);
    updateGlobalLoading();
  };

  const handleSetTaskLoading = (loading: boolean) => {
    setTaskLoading(loading);
    updateGlobalLoading();
  };

  const handleSetAnalysisLoading = (loading: boolean) => {
    setAnalysisLoading(loading);
    updateGlobalLoading();
  };

  const isAnyLoading =
    repositoryLoading ||
    diffLoading ||
    taskLoading ||
    analysisLoading;

  const value: LoadingContextValue = {
    isRepositoryLoading: repositoryLoading,
    isDiffLoading: diffLoading,
    isTaskLoading: taskLoading,
    isAnalysisLoading: analysisLoading,
    setRepositoryLoading: handleSetRepositoryLoading,
    setDiffLoading: handleSetDiffLoading,
    setTaskLoading: handleSetTaskLoading,
    setAnalysisLoading: handleSetAnalysisLoading,
    isAnyLoading
  };

  return (
    <LoadingContext.Provider value={value}>
      {children}
    </LoadingContext.Provider>
  );
};

export const useLoading = (): LoadingContextValue => {
  const context = useContext(LoadingContext);
  if (!context) {
    throw new Error('useLoading must be used within a LoadingProvider');
  }
  return context;
};

// Hook for specific loading states
export const useRepositoryLoading = () => {
  const { isRepositoryLoading, setRepositoryLoading } = useLoading();
  return { isLoading: isRepositoryLoading, setLoading: setRepositoryLoading };
};

export const useDiffLoading = () => {
  const { isDiffLoading, setDiffLoading } = useLoading();
  return { isLoading: isDiffLoading, setLoading: setDiffLoading };
};

export const useTaskLoading = () => {
  const { isTaskLoading, setTaskLoading } = useLoading();
  return { isLoading: isTaskLoading, setLoading: setTaskLoading };
};

export const useAnalysisLoading = () => {
  const { isAnalysisLoading, setAnalysisLoading } = useLoading();
  return { isLoading: isAnalysisLoading, setLoading: setAnalysisLoading };
};

// Combined hook
export const useAnyLoading = () => {
  const { isAnyLoading } = useLoading();
  return isAnyLoading;
};

// Hook to wrap async operations with loading state
export const useWithLoading = () => {
  const { setRepositoryLoading, setDiffLoading, setTaskLoading, setAnalysisLoading } = useLoading();

  const helpers = createWithLoading(
    setRepositoryLoading,
    setDiffLoading,
    setTaskLoading,
    setAnalysisLoading
  );

  return helpers;
};
