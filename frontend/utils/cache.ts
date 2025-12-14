/**
 * Caching Layer for Frequently Accessed Data
 * Implements LRU cache with TTL for optimal performance
 */

import { getPerformanceConfig } from '../config/environment';

// ============================================================================
// Cache Entry Types
// ============================================================================

interface CacheEntry<T> {
  data: T;
  timestamp: number;
  accessCount: number;
  lastAccessed: number;
  key: string;
}

interface CacheOptions {
  ttl?: number; // Time to live in milliseconds
  maxSize?: number; // Maximum number of entries
  maxAge?: number; // Maximum age in milliseconds
}

// ============================================================================
// LRU Cache Implementation
// ============================================================================

class LRUCache<T> {
  private cache = new Map<string, CacheEntry<T>>();
  private ttl: number;
  private maxSize: number;
  private maxAge: number;

  constructor(options: CacheOptions = {}) {
    const config = getPerformanceConfig();
    this.ttl = options.ttl ?? config.cacheTTL * 1000; // Convert to milliseconds
    this.maxSize = options.maxSize ?? config.maxCacheSize;
    this.maxAge = options.maxAge ?? this.ttl;
  }

  /**
   * Get value from cache
   */
  get(key: string): T | null {
    const entry = this.cache.get(key);

    if (!entry) {
      return null;
    }

    const now = Date.now();

    // Check if expired
    if (now - entry.timestamp > this.ttl) {
      this.cache.delete(key);
      return null;
    }

    // Check if too old
    if (now - entry.lastAccessed > this.maxAge) {
      this.cache.delete(key);
      return null;
    }

    // Update access tracking
    entry.accessCount++;
    entry.lastAccessed = now;

    // Move to end (most recently used)
    this.cache.delete(key);
    this.cache.set(key, entry);

    return entry.data;
  }

  /**
   * Set value in cache
   */
  set(key: string, data: T): void {
    const now = Date.now();

    // Remove oldest entry if at capacity
    if (this.cache.size >= this.maxSize) {
      const oldestKey = this.cache.keys().next().value;
      if (oldestKey !== undefined) {
        this.cache.delete(oldestKey);
      }
    }

    // Create entry
    const entry: CacheEntry<T> = {
      data,
      timestamp: now,
      accessCount: 1,
      lastAccessed: now,
      key
    };

    this.cache.set(key, entry);
  }

  /**
   * Check if key exists and is valid
   */
  has(key: string): boolean {
    const entry = this.cache.get(key);
    if (!entry) {
      return false;
    }

    const now = Date.now();
    if (now - entry.timestamp > this.ttl) {
      this.cache.delete(key);
      return false;
    }

    if (now - entry.lastAccessed > this.maxAge) {
      this.cache.delete(key);
      return false;
    }

    return true;
  }

  /**
   * Delete entry from cache
   */
  delete(key: string): boolean {
    return this.cache.delete(key);
  }

  /**
   * Clear all entries
   */
  clear(): void {
    this.cache.clear();
  }

  /**
   * Get cache size
   */
  size(): number {
    return this.cache.size;
  }

  /**
   * Get cache statistics
   */
  getStats() {
    const now = Date.now();
    let validEntries = 0;
    let expiredEntries = 0;
    let totalAccessCount = 0;

    for (const entry of this.cache.values()) {
      if (now - entry.timestamp > this.ttl || now - entry.lastAccessed > this.maxAge) {
        expiredEntries++;
      } else {
        validEntries++;
      }
      totalAccessCount += entry.accessCount;
    }

    return {
      totalEntries: this.cache.size,
      validEntries,
      expiredEntries,
      totalAccessCount,
      hitRate: validEntries > 0 ? (totalAccessCount / validEntries).toFixed(2) : '0'
    };
  }

  /**
   * Clean expired entries
   */
  cleanup(): number {
    const now = Date.now();
    let cleaned = 0;

    for (const [key, entry] of this.cache.entries()) {
      if (now - entry.timestamp > this.ttl || now - entry.lastAccessed > this.maxAge) {
        this.cache.delete(key);
        cleaned++;
      }
    }

    return cleaned;
  }
}

// ============================================================================
// Cache Manager
// ============================================================================

class CacheManager {
  private caches = new Map<string, LRUCache<any>>();

  /**
   * Get or create a named cache
   */
  getCache<T>(name: string, options?: CacheOptions): LRUCache<T> {
    if (!this.caches.has(name)) {
      this.caches.set(name, new LRUCache<T>(options));
    }
    return this.caches.get(name) as LRUCache<T>;
  }

  /**
   * Clear a specific cache
   */
  clearCache(name: string): void {
    const cache = this.caches.get(name);
    if (cache) {
      cache.clear();
    }
  }

  /**
   * Clear all caches
   */
  clearAll(): void {
    for (const cache of this.caches.values()) {
      cache.clear();
    }
  }

  /**
   * Get statistics for all caches
   */
  getAllStats() {
    const stats: Record<string, any> = {};

    for (const [name, cache] of this.caches.entries()) {
      stats[name] = cache.getStats();
    }

    return stats;
  }

  /**
   * Clean expired entries in all caches
   */
  cleanupAll(): number {
    let totalCleaned = 0;

    for (const cache of this.caches.values()) {
      totalCleaned += cache.cleanup();
    }

    return totalCleaned;
  }

  /**
   * Delete a cache entirely
   */
  deleteCache(name: string): boolean {
    return this.caches.delete(name);
  }
}

// ============================================================================
// Singleton Instance
// ============================================================================

export const cacheManager = new CacheManager();

// ============================================================================
// Pre-configured Caches
// ============================================================================

// Repository cache
export const repositoryCache = cacheManager.getCache<any>('repositories', {
  ttl: 5 * 60 * 1000, // 5 minutes
  maxSize: 50
});

// Branches cache
export const branchesCache = cacheManager.getCache<any>('branches', {
  ttl: 2 * 60 * 1000, // 2 minutes
  maxSize: 100
});

// Diff cache (large files)
export const diffCache = cacheManager.getCache<any>('diffs', {
  ttl: 10 * 60 * 1000, // 10 minutes
  maxSize: 20
});

// Heatmap cache
export const heatmapCache = cacheManager.getCache<any>('heatmaps', {
  ttl: 15 * 60 * 1000, // 15 minutes
  maxSize: 30
});

// Blame cache
export const blameCache = cacheManager.getCache<any>('blame', {
  ttl: 30 * 60 * 1000, // 30 minutes
  maxSize: 50
});

// Search results cache
export const searchCache = cacheManager.getCache<any>('search', {
  ttl: 5 * 60 * 1000, // 5 minutes
  maxSize: 100
});

// ============================================================================
// Cache Decorator
// ============================================================================

/**
 * Decorator function for caching async operations
 */
export function withCache<T, Args extends any[]>(
  cache: LRUCache<T>,
  keyGenerator: (...args: Args) => string,
  _ttl?: number // Reserved for future TTL implementation
) {
  return function (
    _target: any,
    _propertyKey: string,
    descriptor: PropertyDescriptor
  ) {
    const originalMethod = descriptor.value;

    descriptor.value = async function (...args: Args): Promise<T> {
      const cacheKey = keyGenerator(...args);
      const cached = cache.get(cacheKey);

      if (cached) {
        return cached;
      }

      const result = await originalMethod.apply(this, args);

      if (result !== null && result !== undefined) {
        cache.set(cacheKey, result);
      }

      return result;
    };

    return descriptor;
  };
}

// ============================================================================
// Utility Functions
// ============================================================================

/**
 * Generate cache key from multiple parameters
 */
export function generateCacheKey(...parts: (string | number)[]): string {
  return parts.join(':');
}

/**
 * Generate repository-specific cache key
 */
export function getRepositoryCacheKey(repoPath: string, ...extra: (string | number)[]): string {
  return generateCacheKey('repo', repoPath, ...extra);
}

/**
 * Generate file-specific cache key
 */
export function getFileCacheKey(repoPath: string, filePath: string, ...extra: (string | number)[]): string {
  return generateCacheKey('file', repoPath, filePath, ...extra);
}

/**
 * Invalidate cache for a repository
 */
export function invalidateRepositoryCache(repoPath: string): void {
  const prefix = `repo:${repoPath}`;

  // Clear repository cache
  const repoCache = cacheManager.getCache<any>('repositories');
  for (const key of repoCache['cache'].keys()) {
    if (key.startsWith(prefix)) {
      repoCache.delete(key);
    }
  }

  // Clear branches cache
  const branches = cacheManager.getCache<any>('branches');
  for (const key of branches['cache'].keys()) {
    if (key.startsWith(prefix)) {
      branches.delete(key);
    }
  }

  // Clear diff cache
  const diffs = cacheManager.getCache<any>('diffs');
  for (const key of diffs['cache'].keys()) {
    if (key.startsWith(prefix)) {
      diffs.delete(key);
    }
  }
}

/**
 * Invalidate cache for a specific file
 */
export function invalidateFileCache(repoPath: string, filePath: string): void {
  const prefix = `file:${repoPath}:${filePath}`;

  // Clear diff cache for this file
  const diffs = cacheManager.getCache<any>('diffs');
  for (const key of diffs['cache'].keys()) {
    if (key.startsWith(prefix)) {
      diffs.delete(key);
    }
  }

  // Clear blame cache for this file
  const blame = cacheManager.getCache<any>('blame');
  for (const key of blame['cache'].keys()) {
    if (key.startsWith(prefix)) {
      blame.delete(key);
    }
  }
}

// ============================================================================
// Cache Monitoring
// ============================================================================

/**
 * Start periodic cache cleanup
 */
let cleanupInterval: NodeJS.Timeout | null = null;

export function startCacheMonitoring(intervalMs: number = 5 * 60 * 1000): void {
  if (cleanupInterval) {
    clearInterval(cleanupInterval);
  }

  cleanupInterval = setInterval(() => {
    const cleaned = cacheManager.cleanupAll();
    if (cleaned > 0) {
      console.log(`Cache cleanup: removed ${cleaned} expired entries`);
    }
  }, intervalMs);
}

/**
 * Stop cache monitoring
 */
export function stopCacheMonitoring(): void {
  if (cleanupInterval) {
    clearInterval(cleanupInterval);
    cleanupInterval = null;
  }
}

/**
 * Get comprehensive cache statistics
 */
export function getCacheStatistics() {
  const stats = cacheManager.getAllStats();
  const totalEntries = Object.values(stats).reduce((sum, stat) => sum + stat.totalEntries, 0);
  const totalValid = Object.values(stats).reduce((sum, stat) => sum + stat.validEntries, 0);
  const totalExpired = Object.values(stats).reduce((sum, stat) => sum + stat.expiredEntries, 0);

  return {
    caches: stats,
    summary: {
      totalCaches: Object.keys(stats).length,
      totalEntries,
      totalValid,
      totalExpired,
      hitRate: totalValid > 0 ? (totalValid / totalEntries).toFixed(2) : '0'
    }
  };
}
