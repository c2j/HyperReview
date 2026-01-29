import { useState, useCallback, useEffect, useRef } from 'react';

export interface UseGerritPollingOptions {
  enabled?: boolean;        // Enable/disable polling
  interval?: number;        // Polling interval in ms (default: 30000 = 30s)
  onPoll: () => Promise<void>; // Function to call on each poll
  onError?: (error: Error) => void; // Error callback
  immediate?: boolean;      // Poll immediately on mount (default: true)
}

export interface UseGerritPollingResult {
  isPolling: boolean;
  lastPolled: Date | null;
  pollCount: number;
  startPolling: () => void;
  stopPolling: () => void;
  manualPoll: () => Promise<void>;
}

const DEFAULT_INTERVAL = 30000; // 30 seconds
const DEFAULT_IMMEDIATE = true;

export const useGerritPolling = (options: UseGerritPollingOptions): UseGerritPollingResult => {
  const {
    enabled = true,
    interval = DEFAULT_INTERVAL,
    onPoll,
    onError,
    immediate = DEFAULT_IMMEDIATE
  } = options;

  const [isPolling, setIsPolling] = useState(false);
  const [lastPolled, setLastPolled] = useState<Date | null>(null);
  const [pollCount, setPollCount] = useState(0);

  const intervalRef = useRef<NodeJS.Timeout | null>(null);
  const mountedRef = useRef(true);

  const poll = useCallback(async () => {
    if (!mountedRef.current) return;

    setIsPolling(true);
    try {
      await onPoll();
      if (mountedRef.current) {
        setLastPolled(new Date());
        setPollCount(prev => prev + 1);
      }
    } catch (error) {
      console.error('Gerrit polling error:', error);
      if (mountedRef.current && onError) {
        onError(error as Error);
      }
    } finally {
      if (mountedRef.current) {
        setIsPolling(false);
      }
    }
  }, [onPoll, onError]);

  const startPolling = useCallback(() => {
    // Don't stop and restart if interval is already running
    if (intervalRef.current) {
      return;
    }

    if (immediate) {
      poll();
    }

    intervalRef.current = setInterval(() => {
      if (mountedRef.current) {
        poll();
      }
    }, interval);
  }, [poll, interval, immediate]);

  const stopPolling = useCallback(() => {
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
      intervalRef.current = null;
    }
  }, []);

  const manualPoll = useCallback(async () => {
    await poll();
  }, [poll]);

  useEffect(() => {
    mountedRef.current = true;

    if (enabled) {
      startPolling();
    } else {
      stopPolling();
    }

    return () => {
      mountedRef.current = false;
      stopPolling();
    };
  }, [enabled, startPolling, stopPolling]);

  return {
    isPolling,
    lastPolled,
    pollCount,
    startPolling,
    stopPolling,
    manualPoll
  };
};

export default useGerritPolling;
