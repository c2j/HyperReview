import { useEffect } from 'react';
import { useRepositoryStore } from '../store/reviewStore';

const BRANCH_PERSISTENCE_KEY = 'hyperreview_branch_selection';

interface BranchSelection {
  baseBranch: string | null;
  headBranch: string | null;
}

/**
 * Custom hook for persisting branch selection to localStorage
 */
export const useBranchSelectionPersistence = () => {
  const { selectedBaseBranch: storeBase, selectedHeadBranch: storeHead, setSelectedBaseBranch, setSelectedHeadBranch } = useRepositoryStore();

  // Load from localStorage on mount
  useEffect(() => {
    try {
      const stored = localStorage.getItem(BRANCH_PERSISTENCE_KEY);
      if (stored) {
        const selection: BranchSelection = JSON.parse(stored);
        console.log('[BranchSelectionPersistence] Loaded from localStorage:', selection);
        setSelectedBaseBranch(selection.baseBranch);
        setSelectedHeadBranch(selection.headBranch);
      }
    } catch (error) {
      console.error('[BranchSelectionPersistence] Failed to load from localStorage:', error);
    }
  }, [setSelectedBaseBranch, setSelectedHeadBranch]);

  // Save to localStorage when selection changes
  useEffect(() => {
    if (storeBase !== null || storeHead !== null) {
      const selection: BranchSelection = {
        baseBranch: storeBase,
        headBranch: storeHead,
      };
      try {
        localStorage.setItem(BRANCH_PERSISTENCE_KEY, JSON.stringify(selection));
        console.log('[BranchSelectionPersistence] Saved to localStorage:', selection);
      } catch (error) {
        console.error('[BranchSelectionPersistence] Failed to save to localStorage:', error);
      }
    }
  }, [storeBase, storeHead]);

  return {
    baseBranch: storeBase,
    headBranch: storeHead,
  };
};
