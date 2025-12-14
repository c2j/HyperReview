/**
 * Offline Cache Utility
 * Handles network failures and preserves drafts for offline submission
 */

export interface OfflineDraft {
  id: string;
  type: 'review_submission' | 'comment' | 'sync';
  timestamp: number;
  data: any;
  system?: string; // For external system submissions
  status: 'pending' | 'failed' | 'synced';
  retryCount: number;
  lastError?: string;
}

const DRAFT_STORAGE_KEY = 'hyperreview_offline_drafts';
const MAX_RETRY_COUNT = 3;
const DRAFT_EXPIRY_MS = 7 * 24 * 60 * 60 * 1000; // 7 days

class OfflineCache {
  private drafts: OfflineDraft[] = [];

  constructor() {
    this.loadDrafts();
  }

  /**
   * Load drafts from localStorage
   */
  private loadDrafts(): void {
    try {
      const stored = localStorage.getItem(DRAFT_STORAGE_KEY);
      if (stored) {
        this.drafts = JSON.parse(stored);
        // Filter out expired drafts
        this.drafts = this.drafts.filter(draft => {
          const isExpired = Date.now() - draft.timestamp > DRAFT_EXPIRY_MS;
          if (isExpired) {
            console.log('Removing expired draft:', draft.id);
          }
          return !isExpired;
        });
        this.saveDrafts();
      }
    } catch (error) {
      console.error('Failed to load offline drafts:', error);
      this.drafts = [];
    }
  }

  /**
   * Save drafts to localStorage
   */
  private saveDrafts(): void {
    try {
      localStorage.setItem(DRAFT_STORAGE_KEY, JSON.stringify(this.drafts));
    } catch (error) {
      console.error('Failed to save offline drafts:', error);
    }
  }

  /**
   * Create a new offline draft
   */
  createDraft(type: OfflineDraft['type'], data: any, system?: string): OfflineDraft {
    const draft: OfflineDraft = {
      id: `draft_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      type,
      timestamp: Date.now(),
      data,
      system,
      status: 'pending',
      retryCount: 0
    };

    this.drafts.push(draft);
    this.saveDrafts();
    return draft;
  }

  /**
   * Get all drafts
   */
  getDrafts(): OfflineDraft[] {
    return [...this.drafts];
  }

  /**
   * Get drafts by type
   */
  getDraftsByType(type: OfflineDraft['type']): OfflineDraft[] {
    return this.drafts.filter(draft => draft.type === type);
  }

  /**
   * Get pending drafts
   */
  getPendingDrafts(): OfflineDraft[] {
    return this.drafts.filter(draft => draft.status === 'pending');
  }

  /**
   * Update draft status
   */
  updateDraftStatus(id: string, status: OfflineDraft['status'], error?: string): void {
    const draft = this.drafts.find(d => d.id === id);
    if (draft) {
      draft.status = status;
      if (error) {
        draft.lastError = error;
        draft.retryCount += 1;
      }
      this.saveDrafts();
    }
  }

  /**
   * Mark draft as synced
   */
  markAsSynced(id: string): void {
    this.updateDraftStatus(id, 'synced');
  }

  /**
   * Mark draft as failed
   */
  markAsFailed(id: string, error: string): void {
    this.updateDraftStatus(id, 'failed', error);
  }

  /**
   * Remove draft
   */
  removeDraft(id: string): void {
    this.drafts = this.drafts.filter(draft => draft.id !== id);
    this.saveDrafts();
  }

  /**
   * Clear all drafts
   */
  clearDrafts(): void {
    this.drafts = [];
    this.saveDrafts();
  }

  /**
   * Clear synced drafts (keep only pending and failed)
   */
  clearSyncedDrafts(): void {
    this.drafts = this.drafts.filter(draft => draft.status !== 'synced');
    this.saveDrafts();
  }

  /**
   * Retry failed drafts
   */
  async retryFailedDrafts(retryFn: (draft: OfflineDraft) => Promise<void>): Promise<void> {
    const failedDrafts = this.drafts.filter(
      draft => draft.status === 'failed' && draft.retryCount < MAX_RETRY_COUNT
    );

    for (const draft of failedDrafts) {
      try {
        console.log(`Retrying draft ${draft.id} (attempt ${draft.retryCount + 1})`);
        await retryFn(draft);
        this.markAsSynced(draft.id);
      } catch (error) {
        console.error(`Failed to retry draft ${draft.id}:`, error);
        this.markAsFailed(draft.id, error instanceof Error ? error.message : String(error));
      }
    }
  }

  /**
   * Get draft statistics
   */
  getStats(): {
    total: number;
    pending: number;
    failed: number;
    synced: number;
  } {
    return {
      total: this.drafts.length,
      pending: this.drafts.filter(d => d.status === 'pending').length,
      failed: this.drafts.filter(d => d.status === 'failed').length,
      synced: this.drafts.filter(d => d.status === 'synced').length
    };
  }

  /**
   * Check if online
   */
  isOnline(): boolean {
    return navigator.onLine;
  }

  /**
   * Monitor online status
   */
  onOnline(callback: () => void): () => void {
    const handleOnline = () => callback();
    window.addEventListener('online', handleOnline);
    return () => window.removeEventListener('online', handleOnline);
  }

  /**
   * Monitor offline status
   */
  onOffline(callback: () => void): () => void {
    const handleOffline = () => callback();
    window.addEventListener('offline', handleOffline);
    return () => window.removeEventListener('offline', handleOffline);
  }
}

// Export singleton instance
export const offlineCache = new OfflineCache();

/**
 * Hook for using offline cache
 */
export function useOfflineCache() {
  return {
    createDraft: (type: OfflineDraft['type'], data: any, system?: string) =>
      offlineCache.createDraft(type, data, system),
    getDrafts: () => offlineCache.getDrafts(),
    getPendingDrafts: () => offlineCache.getPendingDrafts(),
    markAsSynced: (id: string) => offlineCache.markAsSynced(id),
    markAsFailed: (id: string, error: string) => offlineCache.markAsFailed(id, error),
    removeDraft: (id: string) => offlineCache.removeDraft(id),
    retryFailedDrafts: (retryFn: (draft: OfflineDraft) => Promise<void>) =>
      offlineCache.retryFailedDrafts(retryFn),
    getStats: () => offlineCache.getStats(),
    isOnline: () => offlineCache.isOnline(),
    onOnline: (callback: () => void) => offlineCache.onOnline(callback),
    onOffline: (callback: () => void) => offlineCache.onOffline(callback)
  };
}

export default offlineCache;
