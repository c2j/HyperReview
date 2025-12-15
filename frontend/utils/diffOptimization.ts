/**
 * Diff performance optimizations for large files
 * Handles 10k+ line files efficiently
 */

import type { DiffLine } from '../api/types';

interface DiffOptimizationOptions {
  maxVisibleLines?: number;
  chunkSize?: number;
  enableSyntaxHighlighting?: boolean;
  enableFolding?: boolean;
}

interface DiffChunk {
  id: string;
  lines: DiffLine[];
  startLine: number;
  endLine: number;
  isFolded: boolean;
  isVisible: boolean;
}

/**
 * Optimizes large diffs by chunking and lazy loading
 */
export class DiffOptimizer {
  private options: DiffOptimizationOptions;
  private chunks: DiffChunk[] = [];

  constructor(options: DiffOptimizationOptions = {}) {
    this.options = {
      maxVisibleLines: 1000,
      chunkSize: 500,
      enableSyntaxHighlighting: false,
      enableFolding: true,
      ...options
    };
  }

  /**
   * Process diff lines into optimized chunks
   */
  processDiff(lines: DiffLine[]): DiffChunk[] {
    if (lines.length <= this.options.chunkSize!) {
      // Small diff, no chunking needed
      return [{
        id: 'chunk-0',
        lines,
        startLine: 0,
        endLine: lines.length - 1,
        isFolded: false,
        isVisible: true
      }];
    }

    // Chunk large diffs
    const chunks: DiffChunk[] = [];
    let currentChunk: DiffLine[] = [];
    let chunkStart = 0;
    let chunkId = 0;

    for (let i = 0; i < lines.length; i++) {
      const line = lines[i];
      currentChunk.push(line);

      // Create chunk at boundaries or when chunk size is reached
      const shouldChunk = this.shouldCreateChunk(currentChunk, lines, i);

      if (shouldChunk || currentChunk.length >= this.options.chunkSize!) {
        chunks.push({
          id: `chunk-${chunkId}`,
          lines: [...currentChunk],
          startLine: chunkStart,
          endLine: i,
          isFolded: this.shouldFoldChunk(currentChunk),
          isVisible: chunkId === 0 // Only first chunk visible initially
        });

        currentChunk = [];
        chunkStart = i + 1;
        chunkId++;
      }
    }

    // Handle remaining lines
    if (currentChunk.length > 0) {
      chunks.push({
        id: `chunk-${chunkId}`,
        lines: currentChunk,
        startLine: chunkStart,
        endLine: lines.length - 1,
        isFolded: false,
        isVisible: false
      });
    }

    this.chunks = chunks;
    return chunks;
  }

  /**
   * Determine if a chunk should be created at current position
   */
  private shouldCreateChunk(currentChunk: DiffLine[], allLines: DiffLine[], currentIndex: number): boolean {
    if (!this.options.enableFolding) return false;

    const lastLine = currentChunk[currentChunk.length - 1];
    const nextLine = allLines[currentIndex + 1];

    // Create chunk at natural boundaries
    if (lastLine?.line_type === 'Header') return true;
    if (lastLine?.line_type === 'Context' && nextLine?.line_type !== 'Context') return true;

    return false;
  }

  /**
   * Determine if a chunk should be folded by default
   */
  private shouldFoldChunk(chunk: DiffLine[]): boolean {
    if (!this.options.enableFolding) return false;

    // Fold chunks that are mostly context lines
    const contextLines = chunk.filter(l => l.line_type === 'Context').length;
    return contextLines > chunk.length * 0.8;
  }

  /**
   * Get visible lines for rendering
   */
  getVisibleLines(): DiffLine[] {
    const visibleLines: DiffLine[] = [];

    this.chunks.forEach(chunk => {
      if (chunk.isVisible && !chunk.isFolded) {
        visibleLines.push(...chunk.lines);
      } else if (chunk.isFolded) {
        // Add placeholder for folded chunk
        visibleLines.push({
          content: `... ${chunk.lines.length} lines folded ...`,
          line_type: 'Header',
          old_line_number: undefined,
          new_line_number: undefined,
          isFoldPlaceholder: true,
          onClick: () => this.toggleChunk(chunk.id)
        } as DiffLine);
      }
    });

    return visibleLines;
  }

  /**
   * Toggle chunk visibility
   */
  toggleChunk(chunkId: string): void {
    const chunk = this.chunks.find(c => c.id === chunkId);
    if (chunk) {
      chunk.isFolded = !chunk.isFolded;

      // Load adjacent chunks when expanding
      if (!chunk.isFolded) {
        this.loadAdjacentChunks(chunk);
      }
    }
  }

  /**
   * Load chunks adjacent to the given chunk
   */
  private loadAdjacentChunks(chunk: DiffChunk): void {
    const chunkIndex = this.chunks.indexOf(chunk);

    // Load previous chunk
    if (chunkIndex > 0) {
      this.chunks[chunkIndex - 1].isVisible = true;
    }

    // Load next chunk
    if (chunkIndex < this.chunks.length - 1) {
      this.chunks[chunkIndex + 1].isVisible = true;
    }
  }

  /**
   * Optimize syntax highlighting for large files
   */
  optimizeSyntaxHighlighting(lines: DiffLine[]): DiffLine[] {
    if (!this.options.enableSyntaxHighlighting) return lines;

    // Only highlight visible chunks to improve performance
    return lines.map(line => {
      if (line.isFoldPlaceholder) return line;

      // Skip syntax highlighting for very long lines
      if (line.content.length > 1000) return line;

      // Add syntax highlighting flag
      return {
        ...line,
        needsSyntaxHighlight: true
      };
    });
  }

  /**
   * Get memory usage statistics
   */
  getMemoryStats(): { totalChunks: number; visibleChunks: number; estimatedMemoryMB: number } {
    const visibleChunkCount = this.chunks.filter(c => c.isVisible).length;
    const totalLines = this.chunks.reduce((sum, chunk) => sum + chunk.lines.length, 0);

    // Rough estimation: ~1KB per line
    const estimatedMemoryMB = (totalLines * 1024) / (1024 * 1024);

    return {
      totalChunks: this.chunks.length,
      visibleChunks: visibleChunkCount,
      estimatedMemoryMB
    };
  }
}

/**
 * Debounce function for search and other operations
 */
export function debounce<T extends (...args: any[]) => any>(
  func: T,
  wait: number
): (...args: Parameters<T>) => void {
  let timeout: NodeJS.Timeout | null = null;

  return (...args: Parameters<T>) => {
    if (timeout) clearTimeout(timeout);

    timeout = setTimeout(() => {
      func(...args);
    }, wait);
  };
}

/**
 * Throttle function for scroll and resize events
 */
export function throttle<T extends (...args: any[]) => any>(
  func: T,
  limit: number
): (...args: Parameters<T>) => void {
  let inThrottle = false;

  return (...args: Parameters<T>) => {
    if (!inThrottle) {
      func(...args);
      inThrottle = true;
      setTimeout(() => inThrottle = false, limit);
    }
  };
}

/**
 * Measure and report performance metrics
 */
export class PerformanceMonitor {
  private metrics: Map<string, number> = new Map();

  startTimer(name: string): void {
    this.metrics.set(`${name}_start`, performance.now());
  }

  endTimer(name: string): number {
    const start = this.metrics.get(`${name}_start`);
    if (!start) return 0;

    const duration = performance.now() - start;
    this.metrics.set(name, duration);

    console.log(`Performance: ${name} took ${duration.toFixed(2)}ms`);
    return duration;
  }

  getMetrics(): Record<string, number> {
    return Object.fromEntries(this.metrics);
  }
}

/**
 * Cache for expensive computations
 */
export class DiffCache {
  private cache = new Map<string, { data: any; timestamp: number }>();
  private maxAge: number;

  constructor(maxAgeMinutes = 5) {
    this.maxAge = maxAgeMinutes * 60 * 1000;
  }

  get<T>(key: string): T | undefined {
    const item = this.cache.get(key);

    if (!item) return undefined;

    const now = Date.now();
    if (now - item.timestamp > this.maxAge) {
      this.cache.delete(key);
      return undefined;
    }

    return item.data;
  }

  set<T>(key: string, data: T): void {
    this.cache.set(key, {
      data,
      timestamp: Date.now()
    });
  }

  clear(): void {
    this.cache.clear();
  }

  getSize(): number {
    return this.cache.size;
  }
}

// Default export
export default {
  DiffOptimizer,
  PerformanceMonitor,
  DiffCache,
  debounce,
  throttle
};