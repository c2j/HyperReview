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
    pub fn record_file(&mut self, 
        file_path: &str, 
        loc: u32, 
        complexity: f32
    ) {
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
    pub fn generate_from_git(
        &self, 
        repo_path: &str
    ) -> Result<Vec<HeatmapItem>, Box<dyn std::error::Error>> {
        log::info!("Generating heatmap for repository: {}", repo_path);

        let repo = match git2::Repository::open(repo_path) {
            Ok(repo) => repo,
            Err(e) => {
                log::error!("Failed to open repository at {}: {}", repo_path, e);
                return Err(Box::new(e));
            }
        };

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

        log::info!("Analyzed {} commits, found {} changed files", commit_count, file_churn.len());

        // Convert to heatmap items
        log::info!("Converting file churn to heatmap items...");
        let mut items: Vec<HeatmapItem> = file_churn
            .into_iter()
            .enumerate()
            .map(|(idx, (file_path, change_frequency))| {
                // Calculate scores
                let churn_score = (change_frequency as f32 / 100.0).min(1.0);
                let complexity_score = self.file_stats
                    .get(&file_path)
                    .map(|s| s.complexity_score)
                    .unwrap_or(0.5);
                let _lines_of_code = self.file_stats
                    .get(&file_path)
                    .map(|s| s.lines_of_code)
                    .unwrap_or(100);

                // Calculate impact score (weighted average)
                let impact_score_float = churn_score * 0.4 + complexity_score * 0.6;

                // Convert to 0-100 scale
                let score = (impact_score_float * 100.0) as u32;

                // Determine category
                let category = if impact_score_float > 0.7 {
                    HeatmapCategory::High
                } else if impact_score_float > 0.4 {
                    HeatmapCategory::Medium
                } else {
                    HeatmapCategory::Low
                };

                // Extract file name from path
                let name = std::path::Path::new(&file_path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or(&file_path)
                    .to_string();

                // Check if file exists in working directory
                let full_file_path = std::path::Path::new(repo_path).join(&file_path);
                let file_exists = full_file_path.exists() && full_file_path.is_file();

                HeatmapItem {
                    id: format!("heatmap-{}", idx),
                    name,
                    path: file_path,
                    impact: category,
                    score,
                    exists: file_exists,
                }
            })
            .collect();

        // Sort by impact score (descending)
        items.sort_by(|a, b| b.score.cmp(&a.score));

        log::info!("Successfully generated {} heatmap items", items.len());
        Ok(items)
    }

    /// Generate heatmap for changed files in current diff with improved metrics
    pub fn generate_for_diff(
        &self, 
        changed_files: &[String], 
        repo_path: Option<&str>
    ) -> Vec<HeatmapItem> {
        if changed_files.is_empty() {
            return Vec::new();
        }

        log::info!("Generating heatmap for {} changed files", changed_files.len());

        // Sort by file path length as a simple complexity indicator
        let mut sorted_files: Vec<&String> = changed_files.iter().collect();
        sorted_files.sort_by(|a, b| {
            // Primary sort: by file extension (source files first)
            let a_ext = std::path::Path::new(a).extension().and_then(|e| e.to_str()).unwrap_or("");
            let b_ext = std::path::Path::new(b).extension().and_then(|e| e.to_str()).unwrap_or("");
            
            let is_source_a = matches!(a_ext, "rs" | "ts" | "js" | "java" | "py" | "go");
            let is_source_b = matches!(b_ext, "rs" | "ts" | "js" | "java" | "py" | "go");
            
            if is_source_a != is_source_b {
                is_source_b.cmp(&is_source_a) // Source files first
            } else {
                // Secondary sort: by path depth (deeper paths = more complex)
                let depth_a = a.matches('/').count();
                let depth_b = b.matches('/').count();
                depth_b.cmp(&depth_a) // Deeper paths first
            }
        });

        sorted_files
            .into_iter()
            .enumerate()
            .map(|(idx, file_path)| {
                // More accurate impact calculation based on file characteristics
                let file_ext = std::path::Path::new(file_path).extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("");
                
                let is_source_file = matches!(file_ext, "rs" | "ts" | "js" | "java" | "py" | "go" | "cpp" | "c");
                let is_config_file = matches!(file_ext, "json" | "yaml" | "yml" | "toml" | "xml");
                let is_doc_file = matches!(file_ext, "md" | "txt" | "rst");
                
                // Calculate complexity score based on file type and depth
                let complexity_score = if is_source_file {
                    0.8 // Source files are high complexity
                } else if is_config_file {
                    0.6 // Config files are medium complexity
                } else if is_doc_file {
                    0.3 // Documentation is low complexity
                } else {
                    0.4 // Other files are low-medium complexity
                };
                
                // Calculate churn score based on file path characteristics
                let path_depth = file_path.matches('/').count() as f32;
                let path_length_score = (path_depth / 10.0).min(0.5); // Max 50% from path depth
                let churn_score = 0.3 + path_length_score; // Base 30% + up to 50% from depth
                
                let impact_score_float = (complexity_score + churn_score) / 2.0;

                // Convert to 0-100 scale
                let score = (impact_score_float * 100.0) as u32;

                // Better category classification based on actual impact
                let category = if impact_score_float >= 0.7 {
                    HeatmapCategory::High
                } else if impact_score_float >= 0.4 {
                    HeatmapCategory::Medium
                } else {
                    HeatmapCategory::Low
                };

                // Extract file name from path
                let name = std::path::Path::new(file_path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or(file_path)
                    .to_string();

                // Check if file exists in working directory
                let file_exists = if let Some(repo_path) = repo_path {
                    let full_file_path = std::path::Path::new(repo_path).join(file_path);
                    full_file_path.exists() && full_file_path.is_file()
                } else {
                    true // Default to true if no repo path provided
                };

                HeatmapItem {
                    id: format!("diff-{}", idx),
                    name,
                    path: file_path.clone(),
                    impact: category,
                    score,
                    exists: file_exists,
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