import { create } from 'zustand';

// Network connection state
interface NetworkState {
  isOnline: boolean;
  lastChecked: Date | null;

  // Actions
  setOnline: (isOnline: boolean) => void;
  updateLastChecked: () => void;
}

// Sync status state
interface SyncState {
  isSyncing: boolean;
  lastSyncTime: Date | null;
  pendingOperations: number;
  failedOperations: number;

  // Actions
  setSyncing: (isSyncing: boolean) => void;
  updateLastSync: (time: Date) => void;
  incrementPending: () => void;
  decrementPending: () => void;
  incrementFailed: () => void;
  resetFailed: () => void;
}

// Offline data state
interface OfflineDataState {
  pendingChanges: string[];
  pendingComments: string[];
  pendingReviews: string[];
  conflictResolutions: Map<string, any>;

  // Actions
  addPendingChange: (changeId: string) => void;
  removePendingChange: (changeId: string) => void;
  addPendingComment: (commentId: string) => void;
  removePendingComment: (commentId: string) => void;
  addPendingReview: (reviewId: string) => void;
  removePendingReview: (reviewId: string) => void;
  setConflictResolution: (id: string, resolution: any) => void;
  clearPending: () => void;
}

// Sync progress state
interface SyncProgressState {
  currentOperation: string | null;
  progress: number;
  total: number;
  errors: string[];

  // Actions
  setCurrentOperation: (operation: string | null) => void;
  setProgress: (progress: number, total: number) => void;
  addError: (error: string) => void;
  clearErrors: () => void;
  resetProgress: () => void;
}

// Combined sync store
interface SyncStore extends
  NetworkState,
  SyncState,
  OfflineDataState,
  SyncProgressState {}

// Create combined sync store
export const useSyncStore = create<SyncStore>((set) => ({
  // Network state
  isOnline: true,
  lastChecked: null,

  setOnline: (isOnline) => set({ isOnline }),
  updateLastChecked: () => set({ lastChecked: new Date() }),

  // Sync state
  isSyncing: false,
  lastSyncTime: null,
  pendingOperations: 0,
  failedOperations: 0,

  setSyncing: (isSyncing) => set({ isSyncing }),
  updateLastSync: (time) => set({ lastSyncTime: time }),
  incrementPending: () => set((state) => ({ pendingOperations: state.pendingOperations + 1 })),
  decrementPending: () => set((state) => ({
    pendingOperations: Math.max(0, state.pendingOperations - 1)
  })),
  incrementFailed: () => set((state) => ({ failedOperations: state.failedOperations + 1 })),
  resetFailed: () => set({ failedOperations: 0 }),

  // Offline data state
  pendingChanges: [],
  pendingComments: [],
  pendingReviews: [],
  conflictResolutions: new Map(),

  addPendingChange: (changeId) => set((state) => ({
    pendingChanges: [...state.pendingChanges, changeId]
  })),
  removePendingChange: (changeId) => set((state) => ({
    pendingChanges: state.pendingChanges.filter((id) => id !== changeId)
  })),
  addPendingComment: (commentId) => set((state) => ({
    pendingComments: [...state.pendingComments, commentId]
  })),
  removePendingComment: (commentId) => set((state) => ({
    pendingComments: state.pendingComments.filter((id) => id !== commentId)
  })),
  addPendingReview: (reviewId) => set((state) => ({
    pendingReviews: [...state.pendingReviews, reviewId]
  })),
  removePendingReview: (reviewId) => set((state) => ({
    pendingReviews: state.pendingReviews.filter((id) => id !== reviewId)
  })),
  setConflictResolution: (id, resolution) => set((state) => {
    const newResolutions = new Map(state.conflictResolutions);
    newResolutions.set(id, resolution);
    return { conflictResolutions: newResolutions };
  }),
  clearPending: () => set({
    pendingChanges: [],
    pendingComments: [],
    pendingReviews: [],
    conflictResolutions: new Map()
  }),

  // Sync progress state
  currentOperation: null,
  progress: 0,
  total: 0,
  errors: [],

  setCurrentOperation: (operation) => set({ currentOperation: operation }),
  setProgress: (progress, total) => set({ progress, total }),
  addError: (error) => set((state) => ({ errors: [...state.errors, error] })),
  clearErrors: () => set({ errors: [] }),
  resetProgress: () => set({ currentOperation: null, progress: 0, total: 0, errors: [] })
}));

// Selectors for convenience
export const selectNetworkStatus = (state: SyncStore) => ({
  isOnline: state.isOnline,
  lastChecked: state.lastChecked
});

export const selectSyncStatus = (state: SyncStore) => ({
  isSyncing: state.isSyncing,
  lastSyncTime: state.lastSyncTime,
  pendingOperations: state.pendingOperations,
  failedOperations: state.failedOperations
});

export const selectOfflineData = (state: SyncStore) => ({
  pendingChanges: state.pendingChanges,
  pendingComments: state.pendingComments,
  pendingReviews: state.pendingReviews,
  conflictResolutions: state.conflictResolutions
});

export const selectSyncProgress = (state: SyncStore) => ({
  currentOperation: state.currentOperation,
  progress: state.progress,
  total: state.total,
  errors: state.errors
});
