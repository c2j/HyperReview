// Performance monitoring utilities
// Track command response times and performance metrics

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Performance metrics collector
pub struct MetricsCollector {
    command_durations: HashMap<String, Vec<Duration>>,
    command_counts: HashMap<String, u64>,
    start_time: Instant,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            command_durations: HashMap::new(),
            command_counts: HashMap::new(),
            start_time: Instant::now(),
        }
    }

    /// Start timing a command execution
    pub fn start_timing(&self) -> CommandTimer {
        CommandTimer {
            start: Instant::now(),
        }
    }

    /// Record command execution time
    pub fn record_command(&mut self, command_name: &str, duration: Duration) {
        let durations = self.command_durations.entry(command_name.to_string()).or_insert_with(Vec::new);
        durations.push(duration);

        // Keep only last 100 measurements to prevent memory bloat
        if durations.len() > 100 {
            durations.remove(0);
        }

        *self.command_counts.entry(command_name.to_string()).or_insert(0) += 1;
    }

    /// Get average execution time for a command
    pub fn get_average_duration(&self, command_name: &str) -> Option<Duration> {
        self.command_durations.get(command_name).and_then(|durations| {
            if durations.is_empty() {
                None
            } else {
                let total: Duration = durations.iter().sum();
                Some(total / durations.len() as u32)
            }
        })
    }

    /// Get 95th percentile execution time
    pub fn get_p95_duration(&self, command_name: &str) -> Option<Duration> {
        self.command_durations.get(command_name).and_then(|durations| {
            if durations.is_empty() {
                None
            } else {
                let mut sorted_durations = durations.clone();
                sorted_durations.sort_unstable();
                let idx = (sorted_durations.len() as f32 * 0.95) as usize;
                sorted_durations.get(idx).copied()
            }
        })
    }

    /// Check if command meets performance SLA (< 200ms)
    pub fn meets_sla(&self, command_name: &str, sla_ms: u64) -> bool {
        self.get_average_duration(command_name)
            .map(|d| d.as_millis() <= sla_ms as u128)
            .unwrap_or(false)
    }

    /// Get total uptime
    pub fn get_uptime(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Get command statistics
    pub fn get_command_stats(&self, command_name: &str) -> Option<CommandStats> {
        let count = *self.command_counts.get(command_name)?;
        let durations = self.command_durations.get(command_name)?;

        if durations.is_empty() {
            return None;
        }

        let mut sorted_durations = durations.clone();
        sorted_durations.sort_unstable();

        let total: Duration = durations.iter().sum();
        let average = total / durations.len() as u32;
        let min = sorted_durations.first().copied().unwrap();
        let max = sorted_durations.last().copied().unwrap();

        let p95_idx = (durations.len() as f32 * 0.95) as usize;
        let p95 = sorted_durations.get(p95_idx).copied().unwrap_or(max);

        let meets_sla = average.as_millis() <= 200;

        Some(CommandStats {
            command_name: command_name.to_string(),
            count,
            average_ms: average.as_millis() as u64,
            min_ms: min.as_millis() as u64,
            max_ms: max.as_millis() as u64,
            p95_ms: p95.as_millis() as u64,
            total_ms: total.as_millis() as u64,
            meets_sla_200ms: meets_sla,
        })
    }

    /// Get all command statistics
    pub fn get_all_stats(&self) -> Vec<CommandStats> {
        self.command_counts.keys()
            .filter_map(|cmd| self.get_command_stats(cmd))
            .collect()
    }

    /// Reset all metrics
    pub fn reset(&mut self) {
        self.command_durations.clear();
        self.command_counts.clear();
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// RAII timer for measuring command execution
pub struct CommandTimer {
    start: Instant,
}

impl CommandTimer {
    pub fn end(self) -> Duration {
        self.start.elapsed()
    }
}

impl Drop for CommandTimer {
    fn drop(&mut self) {
        // Timer is automatically dropped and duration is measured
        // User should call end() explicitly to get duration
    }
}

/// Command performance statistics
#[derive(Debug, Clone)]
pub struct CommandStats {
    pub command_name: String,
    pub count: u64,
    pub average_ms: u64,
    pub min_ms: u64,
    pub max_ms: u64,
    pub p95_ms: u64,
    pub total_ms: u64,
    pub meets_sla_200ms: bool,
}

impl CommandStats {
    /// Format as human-readable string
    pub fn format_summary(&self) -> String {
        format!(
            "Command: {}\n  Count: {}\n  Avg: {}ms | Min: {}ms | Max: {}ms | P95: {}ms\n  SLA (200ms): {}",
            self.command_name,
            self.count,
            self.average_ms,
            self.min_ms,
            self.max_ms,
            self.p95_ms,
            if self.meets_sla_200ms { "✓ PASS" } else { "✗ FAIL" }
        )
    }
}

/// Global metrics collector instance using OnceLock for lazy initialization
use std::sync::{Arc, Mutex};
use std::sync::OnceLock;

static GLOBAL_METRICS: OnceLock<Arc<Mutex<MetricsCollector>>> = OnceLock::new();

fn get_global_metrics() -> &'static Arc<Mutex<MetricsCollector>> {
    GLOBAL_METRICS.get_or_init(|| {
        Arc::new(Mutex::new(MetricsCollector::new()))
    })
}

/// Record command execution time globally
pub fn record_global_command(command_name: &str, duration: Duration) {
    let metrics = get_global_metrics();
    let mut collector = metrics.lock().unwrap();
    collector.record_command(command_name, duration);
}

/// Get command statistics globally
pub fn get_global_command_stats(command_name: &str) -> Option<CommandStats> {
    let metrics = get_global_metrics();
    let collector = metrics.lock().unwrap();
    collector.get_command_stats(command_name)
}

/// Get all command statistics globally
pub fn get_all_global_stats() -> Vec<CommandStats> {
    let metrics = get_global_metrics();
    let collector = metrics.lock().unwrap();
    collector.get_all_stats()
}

/// Check if command meets SLA globally
pub fn global_meets_sla(command_name: &str) -> bool {
    let metrics = get_global_metrics();
    let collector = metrics.lock().unwrap();
    collector.meets_sla(command_name, 200)
}
