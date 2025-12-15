/**
 * Performance Monitoring for Frontend Operations
 * Tracks operation timings and performance metrics
 */

import { getPerformanceConfig } from '../config/environment';

// ============================================================================
// Performance Metric Types
// ============================================================================

interface PerformanceMetric {
  name: string;
  duration: number;
  timestamp: number;
  metadata?: Record<string, any>;
}

interface OperationStats {
  count: number;
  totalDuration: number;
  averageDuration: number;
  minDuration: number;
  maxDuration: number;
  lastExecuted: number;
}

interface PerformanceReport {
  operations: Record<string, OperationStats>;
  summary: {
    totalOperations: number;
    totalDuration: number;
    averageDuration: number;
  };
}

// ============================================================================
// Performance Monitor
// ============================================================================

class PerformanceMonitor {
  private metrics: PerformanceMetric[] = [];
  private operationStats = new Map<string, OperationStats>();
  private enableMetrics: boolean;

  constructor() {
    const config = getPerformanceConfig();
    this.enableMetrics = config.enableMetrics;
  }

  /**
   * Start timing an operation
   */
  startOperation(name: string): () => void {
    if (!this.enableMetrics) {
      return () => {};
    }

    const startTime = performance.now();

    return () => {
      const endTime = performance.now();
      const duration = endTime - startTime;

      this.recordMetric(name, duration);
    };
  }

  /**
   * Record a metric directly
   */
  recordMetric(name: string, duration: number, metadata?: Record<string, any>): void {
    if (!this.enableMetrics) {
      return;
    }

    const metric: PerformanceMetric = {
      name,
      duration,
      timestamp: Date.now(),
      metadata
    };

    // Add to metrics array
    this.metrics.push(metric);

    // Keep only last 1000 metrics to prevent memory issues
    if (this.metrics.length > 1000) {
      this.metrics = this.metrics.slice(-1000);
    }

    // Update operation stats
    this.updateOperationStats(name, duration);
  }

  /**
   * Update operation statistics
   */
  private updateOperationStats(name: string, duration: number): void {
    const now = Date.now();

    if (this.operationStats.has(name)) {
      const stats = this.operationStats.get(name)!;
      stats.count++;
      stats.totalDuration += duration;
      stats.averageDuration = stats.totalDuration / stats.count;
      stats.minDuration = Math.min(stats.minDuration, duration);
      stats.maxDuration = Math.max(stats.maxDuration, duration);
      stats.lastExecuted = now;
    } else {
      this.operationStats.set(name, {
        count: 1,
        totalDuration: duration,
        averageDuration: duration,
        minDuration: duration,
        maxDuration: duration,
        lastExecuted: now
      });
    }
  }

  /**
   * Get operation statistics
   */
  getOperationStats(name: string): OperationStats | undefined {
    return this.operationStats.get(name);
  }

  /**
   * Get all operation statistics
   */
  getAllStats(): Record<string, OperationStats> {
    const stats: Record<string, OperationStats> = {};

    for (const [name, stat] of this.operationStats.entries()) {
      stats[name] = { ...stat };
    }

    return stats;
  }

  /**
   * Get performance report
   */
  getReport(): PerformanceReport {
    const operations = this.getAllStats();
    const summary = {
      totalOperations: Object.values(operations).reduce((sum, stat) => sum + stat.count, 0),
      totalDuration: Object.values(operations).reduce((sum, stat) => sum + stat.totalDuration, 0),
      averageDuration: 0
    };

    summary.averageDuration =
      summary.totalOperations > 0 ? summary.totalDuration / summary.totalOperations : 0;

    return {
      operations,
      summary
    };
  }

  /**
   * Get slow operations (above threshold)
   */
  getSlowOperations(thresholdMs: number = 200): Array<{ name: string; stats: OperationStats }> {
    const slow: Array<{ name: string; stats: OperationStats }> = [];

    for (const [name, stats] of this.operationStats.entries()) {
      if (stats.averageDuration > thresholdMs) {
        slow.push({ name, stats });
      }
    }

    // Sort by average duration descending
    slow.sort((a, b) => b.stats.averageDuration - a.stats.averageDuration);

    return slow;
  }

  /**
   * Get recent metrics
   */
  getRecentMetrics(count: number = 100): PerformanceMetric[] {
    return this.metrics.slice(-count);
  }

  /**
   * Clear all metrics
   */
  clear(): void {
    this.metrics = [];
    this.operationStats.clear();
  }

  /**
   * Check if metrics are enabled
   */
  isEnabled(): boolean {
    return this.enableMetrics;
  }

  /**
   * Enable or disable metrics
   */
  setEnabled(enabled: boolean): void {
    this.enableMetrics = enabled;
  }
}

// ============================================================================
// Singleton Instance
// ============================================================================

export const performanceMonitor = new PerformanceMonitor();

// ============================================================================
// Decorator for Automatic Performance Tracking
// ============================================================================

/**
 * Decorator for automatically tracking function performance
 */
export function trackPerformance(
  name?: string,
  metadataExtractor?: (...args: any[]) => Record<string, any>
) {
  return function (target: any, propertyKey: string, descriptor: PropertyDescriptor) {
    const originalMethod = descriptor.value;

    descriptor.value = async function (...args: any[]): Promise<any> {
      const operationName = name || `${target.constructor.name}.${propertyKey}`;
      const stopTiming = performanceMonitor.startOperation(operationName);

      try {
        const result = await originalMethod.apply(this, args);

        // Extract metadata if provided (currently unused in success path)
        // const metadata = metadataExtractor ? metadataExtractor(...args) : undefined;

        // Record successful execution
        stopTiming();

        return result;
      } catch (error) {
        // Record error with metadata
        stopTiming();

        const metadata = {
          error: error instanceof Error ? error.message : String(error),
          ...(metadataExtractor ? metadataExtractor(...args) : {})
        };

        performanceMonitor.recordMetric(`${operationName}:error`, 0, metadata);

        throw error;
      }
    };

    return descriptor;
  };
}

// ============================================================================
// Performance Monitoring Hooks
// ============================================================================

/**
 * Hook for tracking component render performance
 */
export function useRenderTracking(componentName: string) {
  if (!performanceMonitor.isEnabled()) {
    return;
  }

  const renderCount = React.useRef(0);
  renderCount.current++;

  const lastRender = performance.now();

  performanceMonitor.recordMetric(`${componentName}:render`, 0, {
    renderCount: renderCount.current,
    timestamp: lastRender
  });
}

/**
 * Hook for tracking async operation performance
 */
export function useAsyncOperation<T extends any[], R>(
  operationName: string,
  operation: (...args: T) => Promise<R>
) {
  const [loading, setLoading] = React.useState(false);
  const [error, setError] = React.useState<Error | null>(null);
  const [data, setData] = React.useState<R | null>(null);

  const execute = React.useCallback(
    async (...args: T) => {
      setLoading(true);
      setError(null);

      const stopTiming = performanceMonitor.startOperation(operationName);

      try {
        const result = await operation(...args);
        setData(result);
        stopTiming();
        return result;
      } catch (err) {
        setError(err as Error);
        stopTiming();
        throw err;
      } finally {
        setLoading(false);
      }
    },
    [operationName, operation]
  );

  return { execute, loading, error, data };
}

// ============================================================================
// React Integration
// ============================================================================

import React from 'react';

/**
 * Higher-order component for tracking component performance
 */
export function withPerformanceTracking<P extends object>(
  componentName: string,
  Component: React.ComponentType<P>
) {
  const WrappedComponent = (props: P) => {
    useRenderTracking(componentName);
    return <Component {...props} />;
  };

  WrappedComponent.displayName = `withPerformanceTracking(${componentName})`;

  return WrappedComponent;
}

/**
 * Performance monitoring context
 */
const PerformanceContext = React.createContext<PerformanceMonitor | null>(null);

export const PerformanceProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  return (
    <PerformanceContext.Provider value={performanceMonitor}>
      {children}
    </PerformanceContext.Provider>
  );
};

export const usePerformanceMonitor = (): PerformanceMonitor => {
  const context = React.useContext(PerformanceContext);
  if (!context) {
    throw new Error('usePerformanceMonitor must be used within PerformanceProvider');
  }
  return context;
};

// ============================================================================// Utility Functions
// ============================================================================

/**
 * Measure and record execution time
 */
export async function measureExecution<T>(
  name: string,
  fn: () => T | Promise<T>,
  metadata?: Record<string, any>
): Promise<T> {
  const stopTiming = performanceMonitor.startOperation(name);

  try {
    const result = await fn();
    stopTiming();
    return result;
  } catch (error) {
    stopTiming();
    performanceMonitor.recordMetric(`${name}:error`, 0, {
      error: error instanceof Error ? error.message : String(error),
      ...metadata
    });
    throw error;
  }
}

/**
 * Create a performance tracker for manual timing
 */
export function createTracker(name: string, metadata?: Record<string, any>) {
  const startTime = performance.now();

  return {
    stop: () => {
      const duration = performance.now() - startTime;
      performanceMonitor.recordMetric(name, duration, metadata);
    },
    cancel: () => {
      // Tracker cancelled, no metric recorded
    }
  };
}

/**
 * Get performance summary
 */
export function getPerformanceSummary() {
  const report = performanceMonitor.getReport();
  const slowOperations = performanceMonitor.getSlowOperations(200);

  return {
    report,
    slowOperations,
    timestamp: Date.now()
  };
}

/**
 * Log performance summary to console
 */
export function logPerformanceSummary() {
  if (!performanceMonitor.isEnabled()) {
    console.log('Performance monitoring is disabled');
    return;
  }

  const summary = getPerformanceSummary();

  console.group('🚀 Performance Summary');
  console.log('Total Operations:', summary.report.summary.totalOperations);
  console.log('Total Duration:', `${summary.report.summary.totalDuration.toFixed(2)}ms`);
  console.log('Average Duration:', `${summary.report.summary.averageDuration.toFixed(2)}ms`);
  console.log('Slow Operations:', summary.slowOperations.length);

  if (summary.slowOperations.length > 0) {
    console.group('⚠️ Slow Operations (>200ms)');
    summary.slowOperations.forEach(({ name, stats }) => {
      console.log(`${name}:`, {
        avg: `${stats.averageDuration.toFixed(2)}ms`,
        count: stats.count,
        min: `${stats.minDuration.toFixed(2)}ms`,
        max: `${stats.maxDuration.toFixed(2)}ms`
      });
    });
    console.groupEnd();
  }

  console.groupEnd();
}

/**
 * Start periodic performance logging
 */
let performanceInterval: NodeJS.Timeout | null = null;

export function startPerformanceLogging(intervalMs: number = 30000): void {
  if (performanceInterval) {
    clearInterval(performanceInterval);
  }

  performanceInterval = setInterval(() => {
    logPerformanceSummary();
  }, intervalMs);
}

/**
 * Stop periodic performance logging
 */
export function stopPerformanceLogging(): void {
  if (performanceInterval) {
    clearInterval(performanceInterval);
    performanceInterval = null;
  }
}
