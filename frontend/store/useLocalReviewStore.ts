import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';

export interface DiffContext {
  base: string;
  head: string;
}

export interface LocalReviewState {
  // Repository state
  selectedRepoPath: string | null;
  isRepoLoaded: boolean;
  diffContext: DiffContext;

  // Task state
  activeTaskId: string;

  // File selection state
  selectedFile: string | null;
  activeFilePath: string;

  // Refresh state
  repoRefreshKey: number;

  // Actions - Repository
  setSelectedRepoPath: (path: string | null) => void;
  setIsRepoLoaded: (loaded: boolean) => void;
  setDiffContext: (context: DiffContext) => void;

  // Actions - Task
  setActiveTaskId: (taskId: string) => void;

  // Actions - File selection
  setSelectedFile: (file: string | null) => void;
  setActiveFilePath: (path: string) => void;

  // Actions - Refresh
  triggerRepoRefresh: () => void;

  // Actions - Combined
  loadRepository: (path: string, base: string, head: string) => void;
  reset: () => void;
}

const DEFAULT_DIFF_CONTEXT: DiffContext = {
  base: 'master',
  head: 'feature/payment-retry',
};

const DEFAULT_ACTIVE_FILE_PATH = 'src/main/OrderService.java';

export const useLocalReviewStore = create<LocalReviewState>()(
  persist(
    (set, get) => ({
      // Initial state
      selectedRepoPath: null,
      isRepoLoaded: false,
      diffContext: DEFAULT_DIFF_CONTEXT,

      activeTaskId: '1',

      selectedFile: null,
      activeFilePath: DEFAULT_ACTIVE_FILE_PATH,

      repoRefreshKey: 0,

      // Repository actions
      setSelectedRepoPath: (path) => set({ selectedRepoPath: path }),

      setIsRepoLoaded: (loaded) => set({ isRepoLoaded: loaded }),

      setDiffContext: (context) => set({ diffContext: context }),

      // Task actions
      setActiveTaskId: (taskId) => set({ activeTaskId: taskId }),

      // File selection actions
      setSelectedFile: (file) => set({ selectedFile: file }),

      setActiveFilePath: (path) => set({ activeFilePath: path }),

      // Refresh actions
      triggerRepoRefresh: () => set((state) => ({ repoRefreshKey: state.repoRefreshKey + 1 })),

      // Combined actions
      loadRepository: (path, base, head) => {
        set({
          selectedRepoPath: path,
          diffContext: { base, head },
          isRepoLoaded: true,
          activeFilePath: DEFAULT_ACTIVE_FILE_PATH,
          selectedFile: null,
        });

        // Persist to localStorage
        if (typeof window !== 'undefined') {
          localStorage.setItem('lastRepoPath', path);
          localStorage.setItem('lastBranchBase', base);
          localStorage.setItem('lastBranchHead', head);
        }

        // Trigger refresh
        get().triggerRepoRefresh();
      },

      reset: () =>
        set({
          selectedRepoPath: null,
          isRepoLoaded: false,
          diffContext: DEFAULT_DIFF_CONTEXT,
          activeTaskId: '1',
          selectedFile: null,
          activeFilePath: DEFAULT_ACTIVE_FILE_PATH,
          repoRefreshKey: 0,
        }),
    }),
    {
      name: 'hyperreview-local-review',
      storage: createJSONStorage(() => localStorage),
      partialize: (state) => ({
        selectedRepoPath: state.selectedRepoPath,
        isRepoLoaded: state.isRepoLoaded,
        diffContext: state.diffContext,
        activeTaskId: state.activeTaskId,
        activeFilePath: state.activeFilePath,
      }),
    }
  )
);
