// Caching infrastructure
// LRU cache for diff, blame, and analysis data

use crate::models::DiffLine;
use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::Mutex;

pub struct CacheManager {
    diff_cache: Mutex<LruCache<String, Vec<DiffLine>>>,
    blame_cache: Mutex<LruCache<String, Vec<u8>>>,
    analysis_cache: Mutex<LruCache<String, Vec<u8>>>,
}

impl CacheManager {
    pub fn new() -> Self {
        Self {
            diff_cache: Mutex::new(LruCache::new(NonZeroUsize::new(100).unwrap())),
            blame_cache: Mutex::new(LruCache::new(NonZeroUsize::new(50).unwrap())),
            analysis_cache: Mutex::new(LruCache::new(NonZeroUsize::new(200).unwrap())),
        }
    }

    /// Generate cache key for diff request
    /// Format: file_path|old_commit|new_commit
    pub fn generate_diff_key(file_path: &str, old_commit: Option<&str>, new_commit: Option<&str>) -> String {
        format!(
            "{}|{}|{}",
            file_path,
            old_commit.unwrap_or("WORKDIR"),
            new_commit.unwrap_or("HEAD")
        )
    }

    /// Store diff in cache
    pub fn put_diff(&self, key: String, diff_lines: Vec<DiffLine>) {
        let mut cache = self.diff_cache.lock().unwrap();
        cache.put(key, diff_lines);
        log::debug!("Stored diff in cache ({} entries)", cache.len());
    }

    /// Get diff from cache
    pub fn get_diff(&self, key: &str) -> Option<Vec<DiffLine>> {
        let mut cache = self.diff_cache.lock().unwrap();
        let result = cache.get(key).cloned();
        if result.is_some() {
            log::debug!("Cache hit for diff key: {}", key);
        } else {
            log::debug!("Cache miss for diff key: {}", key);
        }
        result
    }

    /// Check if diff is cached
    pub fn has_diff(&self, key: &str) -> bool {
        let cache = self.diff_cache.lock().unwrap();
        cache.contains(key)
    }

    /// Remove diff from cache
    pub fn remove_diff(&self, key: &str) {
        let mut cache = self.diff_cache.lock().unwrap();
        cache.pop(key);
    }

    /// Clear all diffs from cache
    pub fn clear_diff_cache(&self) {
        let mut cache = self.diff_cache.lock().unwrap();
        cache.clear();
        log::info!("Cleared diff cache");
    }

    /// Get cache statistics
    pub fn get_cache_stats(&self) -> CacheStats {
        let diff_cache = self.diff_cache.lock().unwrap();
        let blame_cache = self.blame_cache.lock().unwrap();
        let analysis_cache = self.analysis_cache.lock().unwrap();

        CacheStats {
            diff_cache_size: diff_cache.len(),
            diff_cache_capacity: diff_cache.cap().get(),
            blame_cache_size: blame_cache.len(),
            blame_cache_capacity: blame_cache.cap().get(),
            analysis_cache_size: analysis_cache.len(),
            analysis_cache_capacity: analysis_cache.cap().get(),
        }
    }

    // TODO: Implement blame caching (Task T047)
    pub fn put_blame(&self, key: String, blame_data: Vec<u8>) {
        let mut cache = self.blame_cache.lock().unwrap();
        cache.put(key, blame_data);
    }

    pub fn get_blame(&self, key: &str) -> Option<Vec<u8>> {
        self.blame_cache.lock().unwrap().get(key).cloned()
    }

    // TODO: Implement analysis result caching (Task T048)
    pub fn put_analysis(&self, key: String, analysis_data: Vec<u8>) {
        let mut cache = self.analysis_cache.lock().unwrap();
        cache.put(key, analysis_data);
    }

    pub fn get_analysis(&self, key: &str) -> Option<Vec<u8>> {
        self.analysis_cache.lock().unwrap().get(key).cloned()
    }

    /// Invalidate cache entries for a specific file
    /// This is called when a file is modified
    pub fn invalidate_file(&self, file_path: &str) {
        let mut cache = self.diff_cache.lock().unwrap();

        // Collect keys that match this file
        let keys_to_remove: Vec<String> = cache
            .iter()
            .filter(|(key, _)| key.starts_with(file_path))
            .map(|(key, _)| key.to_string())
            .collect();

        // Remove matching keys
        for key in &keys_to_remove {
            cache.pop(key);
        }

        log::info!("Invalidated cache entries for file: {} (removed {} entries)", file_path, keys_to_remove.len());
    }

    /// Invalidate all caches
    pub fn invalidate_all(&self) {
        {
            let mut cache = self.diff_cache.lock().unwrap();
            cache.clear();
        }
        {
            let mut cache = self.blame_cache.lock().unwrap();
            cache.clear();
        }
        {
            let mut cache = self.analysis_cache.lock().unwrap();
            cache.clear();
        }
        log::info!("Invalidated all caches");
    }
}

/// Cache statistics for monitoring
pub struct CacheStats {
    pub diff_cache_size: usize,
    pub diff_cache_capacity: usize,
    pub blame_cache_size: usize,
    pub blame_cache_capacity: usize,
    pub analysis_cache_size: usize,
    pub analysis_cache_capacity: usize,
}

impl CacheStats {
    /// Get utilization percentage for a cache
    pub fn utilization(&self, cache_size: usize, cache_capacity: usize) -> f32 {
        if cache_capacity == 0 {
            0.0
        } else {
            (cache_size as f32 / cache_capacity as f32) * 100.0
        }
    }

    /// Get formatted summary string
    pub fn summary(&self) -> String {
        format!(
            "Diff: {}/{} ({:.1}%), Blame: {}/{} ({:.1}%), Analysis: {}/{} ({:.1}%)",
            self.diff_cache_size,
            self.diff_cache_capacity,
            self.utilization(self.diff_cache_size, self.diff_cache_capacity),
            self.blame_cache_size,
            self.blame_cache_capacity,
            self.utilization(self.blame_cache_size, self.blame_cache_capacity),
            self.analysis_cache_size,
            self.analysis_cache_capacity,
            self.utilization(self.analysis_cache_size, self.analysis_cache_capacity)
        )
    }
}

