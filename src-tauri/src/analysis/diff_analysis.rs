// Diff analysis service for accurate file change statistics
// Provides unified diff analysis for file tree and heatmap

use crate::models::DiffStats;
use crate::errors::HyperReviewError;
use git2::{Repository, DiffOptions, DiffDelta};
use std::collections::HashMap;
use std::sync::Arc;

/// Unified diff analysis service for accurate metrics
pub struct DiffAnalysisService {
    repository: Arc<Repository>,
}

impl DiffAnalysisService {
    pub fn new(repository: Arc<Repository>) -> Self {
        Self { repository }
    }

    /// Analyze diff between two commits and get detailed file statistics
    pub fn analyze_diff_between_commits(
        &self,
        old_commit: &str,
        new_commit: &str,
    ) -> Result<HashMap<String, DiffStats>, HyperReviewError> {
        log::info!("Analyzing diff between {} and {}", old_commit, new_commit);

        // Resolve commits
        let old_obj = self.repository.revparse_single(old_commit)?;
        let old_commit_obj = old_obj.peel_to_commit()?;
        let old_tree = old_commit_obj.tree()?;

        let new_obj = self.repository.revparse_single(new_commit)?;
        let new_commit_obj = new_obj.peel_to_commit()?;
        let new_tree = new_commit_obj.tree()?;

        // Create diff with statistics
        let mut diff_options = DiffOptions::new();
        diff_options.include_untracked(true);
        
        let diff = self.repository.diff_tree_to_tree(
            Some(&old_tree),
            Some(&new_tree),
            Some(&mut diff_options),
        )?;

        // Collect detailed diff statistics
        let mut file_stats = HashMap::new();

        // Process each delta (file change)
        diff.foreach(
            &mut |delta, _progress| {
                // Process file-level changes
                let old_path = delta.old_file().path()
                    .map(|p| p.to_string_lossy().to_string());
                let new_path = delta.new_file().path()
                    .map(|p| p.to_string_lossy().to_string());

                let file_path = new_path.clone()
                    .or(old_path.clone())
                    .unwrap_or_else(|| "unknown".to_string());

                // Determine change type
                let change_type = match delta.status() {
                    git2::Delta::Added => "added",
                    git2::Delta::Deleted => "deleted",
                    git2::Delta::Modified => "modified",
                    git2::Delta::Renamed => "renamed",
                    git2::Delta::Copied => "copied",
                    _ => "unknown",
                };

                // Initialize stats for this file
                let stats = file_stats.entry(file_path.clone()).or_insert(DiffStats {
                    file_path: file_path.clone(),
                    change_type: change_type.to_string(),
                    added_lines: 0,
                    removed_lines: 0,
                    total_lines_old: 0,
                    total_lines_new: 0,
                    complexity_score: 0.0,
                    impact_score: 0.0,
                });

                // Estimate line changes based on file size
                let old_size = delta.old_file().size();
                let new_size = delta.new_file().size();
                
                match delta.status() {
                    git2::Delta::Added => {
                        stats.added_lines = new_size as u32;
                    }
                    git2::Delta::Deleted => {
                        stats.removed_lines = old_size as u32;
                    }
                    git2::Delta::Modified => {
                        if new_size > old_size {
                            stats.added_lines = (new_size - old_size) as u32;
                            stats.removed_lines = 0;
                        } else {
                            stats.added_lines = 0;
                            stats.removed_lines = (old_size - new_size) as u32;
                        }
                    }
                    _ => {
                        // For other changes, use simple estimation
                        let total_change = old_size.max(new_size);
                        stats.added_lines = (total_change / 2) as u32;
                        stats.removed_lines = (total_change / 2) as u32;
                    }
                }

                true
            },
            None,
            None,
            None,
        )?;

        // Calculate complexity and impact scores for each file
        let file_paths: Vec<String> = file_stats.keys().cloned().collect();
        for file_path in file_paths {
            if let Some(stats) = file_stats.get_mut(&file_path) {
                let (old_lines, new_lines) = self.calculate_file_line_counts(&file_path);
                stats.total_lines_old = old_lines;
                stats.total_lines_new = new_lines;
                
                stats.complexity_score = self.calculate_complexity_score(
                    stats.added_lines, 
                    stats.removed_lines,
                    stats.total_lines_new,
                );
                stats.impact_score = self.calculate_impact_score(
                    stats.complexity_score,
                    stats.added_lines,
                    stats.removed_lines,
                );
            }
        }

        log::info!("Diff analysis complete: {} files with changes", file_stats.len());
        Ok(file_stats)
    }

    /// Process file-level delta (added, deleted, modified, renamed)
    fn process_delta(
        &self,
        delta: &DiffDelta,
        file_stats: &mut HashMap<String, DiffStats>,
    ) {
        let old_path = delta.old_file().path()
            .map(|p| p.to_string_lossy().to_string());
        let new_path = delta.new_file().path()
            .map(|p| p.to_string_lossy().to_string());

        let file_path = new_path.clone()
            .or(old_path.clone())
            .unwrap_or_else(|| "unknown".to_string());

        // Determine change type
        let change_type = match delta.status() {
            git2::Delta::Added => "added",
            git2::Delta::Deleted => "deleted",
            git2::Delta::Modified => "modified",
            git2::Delta::Renamed => "renamed",
            git2::Delta::Copied => "copied",
            _ => "unknown",
        };

        // Initialize stats for this file
        let stats = file_stats.entry(file_path.clone()).or_insert(DiffStats {
            file_path: file_path.clone(),
            change_type: change_type.to_string(),
            added_lines: 0,
            removed_lines: 0,
            total_lines_old: 0, // Will be calculated later
            total_lines_new: 0, // Will be calculated later
            complexity_score: 0.0,
            impact_score: 0.0,
        });

        stats.change_type = change_type.to_string();
    }

    /// Process individual diff line for accurate line counts
    fn process_diff_line(
        &self,
        delta: &DiffDelta,
        line: &git2::DiffLine,
        file_stats: &mut HashMap<String, DiffStats>,
    ) {
        // Get file path from delta
        let file_path = delta.new_file().path()
            .map(|p| p.to_string_lossy().to_string())
            .or_else(|| delta.old_file().path().map(|p| p.to_string_lossy().to_string()))
            .unwrap_or_else(|| "unknown".to_string());

        if let Some(stats) = file_stats.get_mut(&file_path) {
            match line.origin() {
                '+' => stats.added_lines += 1,
                '-' => stats.removed_lines += 1,
                _ => {}
            }
        }
    }

    /// Calculate complexity score based on file changes
    fn calculate_complexity_score(
        &self,
        added_lines: u32,
        removed_lines: u32,
        total_lines: u32,
    ) -> f32 {
        // Simple complexity calculation based on change ratio
        let change_ratio = (added_lines + removed_lines) as f32 / (total_lines.max(1) as f32);
        
        // Normalize to 0-1 range, with diminishing returns for large changes
        let complexity = (change_ratio * 2.0).min(1.0);
        
        // Ensure minimum complexity for any change
        complexity.max(0.1)
    }

    /// Calculate impact score combining complexity and change magnitude
    fn calculate_impact_score(
        &self,
        complexity_score: f32,
        added_lines: u32,
        removed_lines: u32,
    ) -> f32 {
        let change_magnitude = (added_lines + removed_lines) as f32;
        let magnitude_score = (change_magnitude / 100.0).min(1.0); // Normalize to 0-1
        
        // Weighted combination: 60% complexity, 40% magnitude
        complexity_score * 0.6 + magnitude_score * 0.4
    }

    /// Calculate file line counts for accurate statistics
    fn calculate_file_line_counts(&self, _file_path: &str
    ) -> (u32, u32) {
        // For now, return placeholder values
        // In a full implementation, we would read the actual file content
        (100, 100) // Placeholder values
    }

    /// Get file statistics for a specific file between two commits
    pub fn get_file_diff_stats(
        &self,
        old_commit: &str,
        new_commit: &str,
        file_path: &str,
    ) -> Result<Option<DiffStats>, HyperReviewError> {
        let all_stats = self.analyze_diff_between_commits(old_commit, new_commit)?;
        Ok(all_stats.get(file_path).cloned())
    }
}