import { useState, useCallback, useEffect } from 'react';
import { simpleGerritService, SimpleChange } from '../services/gerrit-simple-service';

export interface UseGerritChangesResult {
  changes: SimpleChange[];
  loading: boolean;
  error: string | null;
  refreshChanges: () => Promise<void>;
  importChange: (changeId: string) => Promise<SimpleChange | null>;
  searchChanges: (query: string) => Promise<SimpleChange[]>;
  removeChange: (changeId: string) => Promise<boolean>;
}

export const useGerritChanges = (instanceId?: string): UseGerritChangesResult => {
  const [changes, setChanges] = useState<SimpleChange[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const refreshChanges = useCallback(async () => {
    setLoading(true);
    setError(null);

    try {
      const fetchedChanges = await simpleGerritService.searchChanges('status:open');
      setChanges(fetchedChanges);
    } catch (err) {
      console.error('Failed to refresh changes:', err);
      setError('Failed to load changes: ' + (err as Error).message);
    } finally {
      setLoading(false);
    }
  }, []);

  const importChange = useCallback(async (changeId: string) => {
    setLoading(true);
    setError(null);

    try {
      const change = await simpleGerritService.importChange(changeId);
      if (change) {
        setChanges(prev => [change, ...prev.filter(c => c.id !== change.id)]);
      }
      return change;
    } catch (err) {
      console.error('Failed to import change:', err);
      setError('Failed to import change: ' + (err as Error).message);
      return null;
    } finally {
      setLoading(false);
    }
  }, []);

  const searchChanges = useCallback(async (query: string) => {
    setLoading(true);
    setError(null);

    try {
      const results = await simpleGerritService.searchChanges(query);
      setChanges(results);
      return results;
    } catch (err) {
      console.error('Failed to search changes:', err);
      setError('Failed to search changes: ' + (err as Error).message);
      return [];
    } finally {
      setLoading(false);
    }
  }, []);

  const removeChange = useCallback(async (changeId: string) => {
    setChanges(prev => prev.filter(c => c.id !== changeId));
    return true;
  }, []);

  useEffect(() => {
    refreshChanges();
  }, [instanceId, refreshChanges]);

  return {
    changes,
    loading,
    error,
    refreshChanges,
    importChange,
    searchChanges,
    removeChange,
  };
};

export default useGerritChanges;
