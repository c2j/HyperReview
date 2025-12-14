// Review statistics aggregation
// Metrics and progress tracking

use crate::models::{ReviewStats, Task, TaskStatus, Comment};

pub struct StatsAggregator {
    total_files: u32,
    reviewed_files: u32,
    start_time: Option<std::time::Instant>,
}

impl StatsAggregator {
    pub fn new() -> Self {
        Self {
            total_files: 0,
            reviewed_files: 0,
            start_time: None,
        }
    }

    /// Start tracking review progress
    pub fn start_tracking(&mut self, total_files: u32) {
        self.total_files = total_files;
        self.reviewed_files = 0;
        self.start_time = Some(std::time::Instant::now());
    }

    /// Mark a file as reviewed
    pub fn mark_file_reviewed(&mut self) {
        self.reviewed_files += 1;
    }

    /// Calculate review statistics from tasks and comments
    pub fn calculate_stats(&self, tasks: &[Task], comments: &[Comment]) -> ReviewStats {
        let total_files = self.total_files;
        let reviewed_files = self.reviewed_files;
        let pending_files = total_files.saturating_sub(reviewed_files);

        // Count comments and severe issues
        let total_comments = comments.len() as u32;
        let severe_issues = tasks.iter()
            .filter(|t| t.priority >= 3 && matches!(t.status, TaskStatus::Active | TaskStatus::Pending))
            .count() as u32;

        // Calculate completion percentage
        let completion_percentage = if total_files > 0 {
            (reviewed_files as f32 / total_files as f32) * 100.0
        } else {
            0.0
        };

        // Calculate files per hour and estimate remaining time
        let (files_per_hour, estimated_time_remaining) = if let Some(start) = self.start_time {
            let elapsed_hours = start.elapsed().as_secs_f32() / 3600.0;
            let rate = if elapsed_hours > 0.0 {
                reviewed_files as f32 / elapsed_hours
            } else {
                0.0
            };

            let remaining = if rate > 0.0 {
                Some((pending_files as f32 / rate * 60.0) as u32) // minutes
            } else {
                None
            };

            (rate, remaining)
        } else {
            (0.0, None)
        };

        ReviewStats {
            total_files,
            reviewed_files,
            pending_files,
            total_comments,
            severe_issues,
            completion_percentage,
            estimated_time_remaining,
            files_per_hour,
        }
    }

    /// Calculate statistics from database data
    pub fn calculate_from_db(
        &self,
        tasks: Vec<Task>,
        comments: Vec<Comment>,
        total_files: u32,
        reviewed_files: u32,
    ) -> ReviewStats {
        let pending_files = total_files.saturating_sub(reviewed_files);

        let total_comments = comments.len() as u32;
        let severe_issues = tasks.iter()
            .filter(|t| t.priority >= 3 && matches!(t.status, TaskStatus::Active | TaskStatus::Pending))
            .count() as u32;

        let completion_percentage = if total_files > 0 {
            (reviewed_files as f32 / total_files as f32) * 100.0
        } else {
            100.0
        };

        // Active tasks count
        let active_tasks = tasks.iter()
            .filter(|t| matches!(t.status, TaskStatus::Active))
            .count() as u32;

        // Estimate based on active tasks
        let files_per_hour = if active_tasks > 0 { 5.0 } else { 0.0 }; // Default estimate
        let estimated_time_remaining = if pending_files > 0 && files_per_hour > 0.0 {
            Some((pending_files as f32 / files_per_hour * 60.0) as u32)
        } else {
            None
        };

        ReviewStats {
            total_files,
            reviewed_files,
            pending_files,
            total_comments,
            severe_issues,
            completion_percentage,
            estimated_time_remaining,
            files_per_hour,
        }
    }
}

impl Default for StatsAggregator {
    fn default() -> Self {
        Self::new()
    }
}
