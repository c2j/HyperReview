// Heatmap generation module
// Architectural impact analysis

use crate::models::{HeatmapItem, HeatmapCategory};
use std::collections::HashMap;

pub struct HeatmapGenerator {
    file_stats: HashMap<String, FileStats>,
}

#[derive(Debug, Clone)]
struct FileStats {
    lines_of_code: u32,
    change_count: u32,
    complexity_score: f32,
}

impl HeatmapGenerator {
    pub fn new() -> Self {
        Self {
            file_stats: HashMap::new(),
        }
    }

    /// Record file statistics for heatmap generation
    pub fn record_file(&mut self, file_path: &str, loc: u32, complexity: f32) {
        let stats = self.file_stats.entry(file_path.to_string()).or_insert(FileStats {
            lines_of_code: loc,
            change_count: 0,
            complexity_score: complexity,
        });
        stats.change_count += 1;
        stats.lines_of_code = loc;
        stats.complexity_score = complexity;
    }

    /// Generate heatmap from git log data
    pub fn generate_from_git(&self, repo_path: &str) -> Result<Vec<HeatmapItem>, Box<dyn std::error::Error>> {
        log::info!("Generating heatmap for repository: {}", repo_path);

        let repo = git2::Repository::open(repo_path)?;

        // Walk through recent commits to analyze file churn
        let mut file_churn: HashMap<String, u32> = HashMap::new();
        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;

        // Limit to last 100 commits for performance
        let mut commit_count = 0;
        for oid in revwalk {
            if commit_count >= 100 {
                break;
            }
            commit_count += 1;

            let oid = oid?;
            let commit = repo.find_commit(oid)?;

            // Get the diff for this commit
            if let Some(parent) = commit.parents().next() {
                let parent_tree = parent.tree()?;
                let commit_tree = commit.tree()?;

                let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&commit_tree), None)?;

                diff.foreach(
                    &mut |delta, _progress| {
                        if let Some(path) = delta.new_file().path() {
                            let path_str = path.to_string_lossy().to_string();
                            *file_churn.entry(path_str).or_insert(0) += 1;
                        }
                        true
                    },
                    None,
                    None,
                    None,
                )?;
            }
        }

        // Convert to heatmap items
        let mut items: Vec<HeatmapItem> = file_churn
            .into_iter()
            .map(|(file_path, change_frequency)| {
                // Calculate scores
                let churn_score = (change_frequency as f32 / 100.0).min(1.0);
                let complexity_score = self.file_stats
                    .get(&file_path)
                    .map(|s| s.complexity_score)
                    .unwrap_or(0.5);
                let lines_of_code = self.file_stats
                    .get(&file_path)
                    .map(|s| s.lines_of_code)
                    .unwrap_or(100);

                // Calculate impact score (weighted average)
                let impact_score = churn_score * 0.4 + complexity_score * 0.6;

                // Determine category
                let category = if impact_score > 0.7 {
                    HeatmapCategory::High
                } else if impact_score > 0.4 {
                    HeatmapCategory::Medium
                } else {
                    HeatmapCategory::Low
                };

                HeatmapItem {
                    file_path,
                    impact_score,
                    churn_score,
                    complexity_score,
                    change_frequency,
                    lines_of_code,
                    category,
                }
            })
            .collect();

        // Sort by impact score (descending)
        items.sort_by(|a, b| b.impact_score.partial_cmp(&a.impact_score).unwrap_or(std::cmp::Ordering::Equal));

        Ok(items)
    }

    /// Generate heatmap for changed files in current diff
    pub fn generate_for_diff(&self, changed_files: &[String]) -> Vec<HeatmapItem> {
        changed_files
            .iter()
            .enumerate()
            .map(|(idx, file_path)| {
                // Simple impact calculation based on position and extension
                let is_source_file = file_path.ends_with(".rs")
                    || file_path.ends_with(".ts")
                    || file_path.ends_with(".js")
                    || file_path.ends_with(".java");

                let complexity_score = if is_source_file { 0.6 } else { 0.3 };
                let churn_score = 0.5;
                let impact_score = (complexity_score + churn_score) / 2.0;

                let category = if idx < 3 {
                    HeatmapCategory::High
                } else if idx < 10 {
                    HeatmapCategory::Medium
                } else {
                    HeatmapCategory::Low
                };

                HeatmapItem {
                    file_path: file_path.clone(),
                    impact_score,
                    churn_score,
                    complexity_score,
                    change_frequency: 1,
                    lines_of_code: 100,
                    category,
                }
            })
            .collect()
    }
}

impl Default for HeatmapGenerator {
    fn default() -> Self {
        Self::new()
    }
}
