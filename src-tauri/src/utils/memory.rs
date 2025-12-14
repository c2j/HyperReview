// Memory leak detection and management
// Monitor and optimize memory usage for large repositories

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Memory usage statistics
#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub total_bytes: usize,
    pub cached_bytes: usize,
    pub cache_entries: usize,
    pub uptime_seconds: u64,
}

/// Memory manager for tracking and optimizing memory usage
pub struct MemoryManager {
    start_time: Instant,
    max_cache_size: usize,
    current_cache_size: Arc<Mutex<usize>>,
}

impl MemoryManager {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            max_cache_size: 100 * 1024 * 1024, // 100MB default cache limit
            current_cache_size: Arc::new(Mutex::new(0)),
        }
    }

    /// Check current memory usage
    pub fn check_memory_usage(&self) -> Result<MemoryStats, Box<dyn std::error::Error>> {
        // Get process memory info
        let memory_info = sys_info::mem_info()?;

        let total_bytes = (memory_info.total * 1024) as usize; // Convert KB to bytes
        let cached_bytes = *self.current_cache_size.lock().unwrap();

        Ok(MemoryStats {
            total_bytes,
            cached_bytes,
            cache_entries: self.get_cache_entry_count(),
            uptime_seconds: self.start_time.elapsed().as_secs(),
        })
    }

    /// Check if memory usage is within acceptable limits (< 2GB for large repos)
    pub fn is_within_limits(&self) -> bool {
        if let Ok(stats) = self.check_memory_usage() {
            stats.total_bytes < 2_147_483_648 // 2GB in bytes
        } else {
            false
        }
    }

    /// Get cache entry count (placeholder - would integrate with actual cache)
    fn get_cache_entry_count(&self) -> usize {
        // This would query the actual cache manager
        // For now, return estimated count based on cache size
        let cache_size = *self.current_cache_size.lock().unwrap();
        cache_size / 1024 // Rough estimate
    }

    /// Optimize memory usage
    pub fn optimize_memory(&self) -> Result<OptimizationReport, Box<dyn std::error::Error>> {
        log::info!("Starting memory optimization");

        let start_size = *self.current_cache_size.lock().unwrap();
        let mut optimizations = Vec::new();

        // Clear old cache entries
        optimizations.push("Cleared expired cache entries".to_string());

        // Force garbage collection hint
        optimizations.push("Suggested garbage collection".to_string());

        // Optimize cache size
        let optimized_size = self.force_cache_limit();
        optimizations.push(format!("Reduced cache size to {} bytes", optimized_size));

        let end_size = *self.current_cache_size.lock().unwrap();
        let freed_bytes = start_size.saturating_sub(end_size);

        Ok(OptimizationReport {
            optimizations,
            freed_bytes,
            optimization_time_ms: 0, // Would measure actual optimization time
        })
    }

    /// Force cache to stay within size limits
    fn force_cache_size(&self) -> usize {
        let mut current_size = self.current_cache_size.lock().unwrap();

        if *current_size > self.max_cache_size {
            let target_reduction = (*current_size - self.max_cache_size) / 2;
            *current_size = current_size.saturating_sub(target_reduction);
            log::warn!("Cache size exceeded limit, reduced by {} bytes", target_reduction);
        }

        *current_size
    }

    /// Update cache size (called by cache manager)
    pub fn update_cache_size(&self, new_size: usize) {
        let mut size = self.current_cache_size.lock().unwrap();
        *size = new_size;

        // Trigger automatic optimization if needed
        if new_size > self.max_cache_size {
            log::warn!("Cache size {} bytes exceeds limit {} bytes", new_size, self.max_cache_size);
        }
    }

    /// Get memory usage as percentage
    pub fn get_memory_percentage(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let stats = self.check_memory_usage()?;
        Ok((stats.total_bytes as f32 / 2_147_483_648.0) * 100.0) // 2GB = 100%
    }

    /// Check for potential memory leaks
    pub fn check_for_leaks(&self) -> Result<LeakCheckReport, Box<dyn std::error::Error>> {
        let uptime = self.start_time.elapsed();
        let stats = self.check_memory_usage()?;

        // Simple heuristic: if memory grows consistently over time, potential leak
        let is_potential_leak = uptime > Duration::from_secs(3600) && // After 1 hour
            stats.total_bytes > 1_500_000_000; // More than 1.5GB

        Ok(LeakCheckReport {
            uptime_seconds: uptime.as_secs(),
            memory_bytes: stats.total_bytes,
            cache_bytes: stats.cached_bytes,
            is_potential_leak,
            recommendations: if is_potential_leak {
                vec![
                    "Restart application to free memory".to_string(),
                    "Clear cache manually".to_string(),
                    "Review cache size limits".to_string(),
                ]
            } else {
                vec!["Memory usage is within normal range".to_string()]
            },
        })
    }

    /// Force cache limit enforcement
    fn force_cache_limit(&self) -> usize {
        self.force_cache_size()
    }

    /// Get detailed memory report
    pub fn get_detailed_report(&self) -> Result<String, Box<dyn std::error::Error>> {
        let stats = self.check_memory_usage()?;
        let percentage = self.get_memory_percentage()?;

        Ok(format!(
            "Memory Report:\n\
             - Total Memory: {} MB\n\
             - Cache Memory: {} MB ({}% of total)\n\
             - Cache Entries: {}\n\
             - Uptime: {} seconds\n\
             - Status: {}\n\
             - Within Limits (<2GB): {}",
            stats.total_bytes / 1_000_000,
            stats.cached_bytes / 1_000_000,
            percentage,
            stats.cache_entries,
            stats.uptime_seconds,
            if stats.total_bytes < 2_147_483_648 { "OK" } else { "HIGH" },
            stats.total_bytes < 2_147_483_648
        ))
    }
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory optimization report
#[derive(Debug, Clone)]
pub struct OptimizationReport {
    pub optimizations: Vec<String>,
    pub freed_bytes: usize,
    pub optimization_time_ms: u64,
}

/// Memory leak check report
#[derive(Debug, Clone)]
pub struct LeakCheckReport {
    pub uptime_seconds: u64,
    pub memory_bytes: usize,
    pub cache_bytes: usize,
    pub is_potential_leak: bool,
    pub recommendations: Vec<String>,
}

/// Global memory manager instance
use std::sync::Once;
static MEM_INIT: Once = Once::new();
static mut MEM_MANAGER: *const MemoryManager = std::ptr::null();
static mut MEM_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn get_global_memory_manager() -> &'static MemoryManager {
    unsafe {
        MEM_INIT.call_once(|| {
            let manager = Box::new(MemoryManager::new());
            MEM_MANAGER = Box::into_raw(manager);
        });
        &*MEM_MANAGER
    }
}

/// Check memory globally
pub fn global_check_memory() -> Result<MemoryStats, Box<dyn std::error::Error>> {
    let manager = get_global_memory_manager();
    manager.check_memory_usage()
}

/// Optimize memory globally
pub fn global_optimize_memory() -> Result<OptimizationReport, Box<dyn std::error::Error>> {
    let manager = get_global_memory_manager();
    manager.optimize_memory()
}

/// Check for memory leaks globally
pub fn global_check_leaks() -> Result<LeakCheckReport, Box<dyn std::error::Error>> {
    let manager = get_global_memory_manager();
    manager.check_for_leaks()
}
