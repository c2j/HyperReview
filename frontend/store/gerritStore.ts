import { create } from 'zustand';
import { tauriClient } from '../services/tauri-client';
import type {
  GerritChange
} from '../api/types/gerrit';

// 共享存储用于协调多个组件访问
export interface GerritChangesStore {
  gerritChanges: GerritChange[];
  loading: boolean;
  error: string | null;
  lastFetch: number;
  fetchChanges: (offset?: number, limit?: number) => Promise<void>;
  getChangeFileContent: (changeId: string, patchSetNumber: number, filePath: string) => Promise<string>;
  clearData: () => void;
}

export const useGerritChangesStore = create<GerritChangesStore>((set, get) => ({
  gerritChanges: [],
  loading: false,
  error: null,
  lastFetch: 0,

  fetchChanges: async (offset?: number, limit?: number) => {
    const now = Date.now();
    const MIN_FETCH_INTERVAL = 10000;

    if (now - get().lastFetch >= MIN_FETCH_INTERVAL) {
      console.log(`[GerritChangesStore] Fetching changes with debounce: offset=${offset}, limit=${limit}, lastFetch=${get().lastFetch}`);

      try {
        const changes = await tauriClient.invokeCommand<GerritChange[]>('gerrit_get_gerrit_changes_simple', {
          offset: offset || 0,
          limit: limit || 25,
        });

        console.log(`[GerritChangesStore] Fetched ${changes.length} changes from Gerrit`);
        console.log(`[GerritChangesStore] Changes:`, JSON.stringify(changes, null, 2));

        set(() => ({
          gerritChanges: changes,
          loading: false,
          error: null,
          lastFetch: now,
        }));

      } catch (error) {
        console.error('[GerritChangesStore] Error fetching changes:', error);
        set(() => ({
          loading: false,
          error: error instanceof Error ? error.message : String(error),
          lastFetch: get().lastFetch,
        }));
      }
    }
  },

  getChangeFileContent: async (changeId: string, patchSetNumber: number, filePath: string) => {
    try {
      const content = await tauriClient.invokeCommand<string>('gerrit_get_file_content_simple', {
        changeId,
        patchSetNumber,
        filePath,
      });

      console.log(`[GerritChangesStore] Fetched file content for ${filePath}, length: ${content.length}`);
      return content;

    } catch (error) {
      console.error('[GerritChangesStore] Error fetching file content:', error);
      throw error;
    }
  },

  clearData: () => {
    console.log('[GerritChangesStore] Clearing Gerrit changes data');
    set(() => ({
      gerritChanges: [],
      loading: false,
      error: null,
      lastFetch: 0,
    }));
  },
}));

export default useGerritChangesStore;
