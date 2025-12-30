// Diff computation module
// Git diff operations and file comparison

use crate::models::{DiffLine, DiffLineType};
use crate::errors::HyperReviewError;
use git2::{Repository, Diff, DiffFormat};
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct DiffEngine {
    repository: Repository,
}

impl DiffEngine {
    pub fn new(repository: Repository) -> Self {
        Self { repository }
    }

    /// Compute diff for a file between two commits
    /// Returns vector of diff lines with metadata
    pub fn compute_file_diff(
        &self,
        file_path: &str,
        old_commit: Option<&str>,
        new_commit: Option<&str>,
    ) -> Result<Vec<DiffLine>, HyperReviewError> {
        log::info!("=== DIFF ENGINE DEBUG ===");
        log::info!("Computing diff for file: {}", file_path);
        log::info!("Old commit: {:?}, New commit: {:?}", old_commit, new_commit);
        log::info!("Repository workdir: {:?}", self.repository.workdir());

        // Normalize file path - convert absolute to relative path for git operations
        let normalized_path = if file_path.starts_with('/') {
            // Extract relative path from absolute path
            if let Some(repo_path) = self.repository.workdir().and_then(|p| p.to_str()) {
                log::debug!("Repository workdir: {}", repo_path);
                log::debug!("Original file path: {}", file_path);
                
                // Handle trailing slash in repo_path
                let repo_path_clean = repo_path.trim_end_matches('/');
                log::debug!("Repository workdir (clean): {}", repo_path_clean);
                
                let stripped = if file_path.starts_with(repo_path) {
                    file_path.strip_prefix(repo_path).unwrap_or(file_path)
                } else if file_path.starts_with(repo_path_clean) {
                    file_path.strip_prefix(repo_path_clean).unwrap_or(file_path)
                } else {
                    file_path
                };
                
                log::debug!("Stripped path: {}", stripped);
                let normalized = stripped.trim_start_matches('/');
                log::debug!("Final normalized path: {}", normalized);
                normalized
            } else {
                log::debug!("Could not get repository workdir, using original path");
                file_path
            }
        } else {
            log::debug!("Path is already relative: {}", file_path);
            file_path
        };

        // Check if file is binary (using original path for extension check)
        if self.is_binary_file(file_path) {
            return Err(HyperReviewError::Other {
                message: "Cannot compute diff for binary file".to_string(),
            });
        }

        // Get the tree for old commit and new commit
        let old_tree = match old_commit {
            Some(commit) => {
                log::debug!("Resolving old commit: {}", commit);
                let object = self.repository.revparse_single(commit)?;
                log::debug!("Old commit object ID: {}", object.id());
                let commit_obj = self.repository.find_commit(object.id())?;
                log::debug!("Found old commit: {}", commit_obj.id());
                let tree = commit_obj.tree()?;
                log::debug!("Got old tree: {}", tree.id());
                Some(tree)
            }
            None => None,
        };

        let new_tree = match new_commit {
            Some(commit) => {
                log::debug!("Resolving new commit: {}", commit);
                let object = self.repository.revparse_single(commit)?;
                log::debug!("New commit object ID: {}", object.id());
                let commit_obj = self.repository.find_commit(object.id())?;
                log::debug!("Found new commit: {}", commit_obj.id());
                let tree = commit_obj.tree()?;
                log::debug!("Got new tree: {}", tree.id());
                Some(tree)
            }
            None => {
                // Use working directory if no new commit specified - just use None
                log::debug!("No new commit specified, will use working directory");
                None
            }
        };

        // Create diff between trees or against working directory
        let diff = match (old_tree, new_tree) {
            (Some(old), Some(new)) => {
                // Diff between two commits
                log::debug!("Creating diff between two trees with pathspec: {}", normalized_path);
                let mut diff_options = git2::DiffOptions::new();
                diff_options.pathspec(normalized_path);  // Use normalized path for pathspec
                let diff_result = self.repository.diff_tree_to_tree(Some(&old), Some(&new), Some(&mut diff_options))?;
                log::debug!("Diff created successfully");
                diff_result
            }
            (Some(old), None) => {
                // Diff between a commit and working directory
                let mut diff_options = git2::DiffOptions::new();
                diff_options.include_untracked(true);
                diff_options.pathspec(file_path);

                // Create diff between old tree and working directory with index
                self.repository.diff_tree_to_workdir_with_index(Some(&old), Some(&mut diff_options))?
            }
            (None, Some(new)) => {
                // Diff from empty to a commit (shouldn't normally happen)
                let mut diff_options = git2::DiffOptions::new();
                diff_options.pathspec(file_path);
                self.repository.diff_tree_to_tree(None, Some(&new), Some(&mut diff_options))?
            }
            (None, None) => {
                // Both are None - return empty diff
                log::warn!("Both old and new commits are None, returning empty diff");
                return Ok(Vec::new());
            }
        };

        // Parse diff into lines (use normalized path for parsing)
        self.parse_diff(diff, normalized_path)
    }

    /// Parse git diff into structured DiffLine objects for a specific file
    fn parse_diff(&self, diff: Diff, file_path: &str) -> Result<Vec<DiffLine>, HyperReviewError> {
        log::info!("Parsing diff for file: {}", file_path);
        let mut lines = Vec::new();
        let mut old_line_num = 0;
        let mut new_line_num = 0;
        let mut in_target_file = false;

        // Get the patch (unified diff format)
        let mut patch = Vec::new();
        diff.print(DiffFormat::Patch, |_delta, _hunk, line| {
            patch.extend_from_slice(line.content());
            true
        })?;

        // Log the patch content for debugging
        let patch_str = std::str::from_utf8(&patch).unwrap_or("");
        log::info!("Patch content for {}:\n{}", file_path, patch_str);

        let patch_reader = BufReader::new(patch.as_slice());

        for line in patch_reader.lines() {
            let line = line.map_err(|e| HyperReviewError::Other {
                message: format!("Failed to read diff line: {}", e),
            })?;

            // Check if this is the start of a new file diff
            if line.starts_with("diff --git") {
                // Extract filenames from diff header
                // Format: diff --git a/old_path b/new_path
                let parts: Vec<&str> = line.split_whitespace().collect();
                
                // Find parts that start with 'a/' and 'b/'
                let mut old_file: Option<&str> = None;
                let mut new_file: Option<&str> = None;
                
                for part in &parts {
                    if part.starts_with("a/") {
                        old_file = Some(part.strip_prefix("a/").unwrap_or(part));
                    } else if part.starts_with("b/") {
                        new_file = Some(part.strip_prefix("b/").unwrap_or(part));
                    }
                }
                
                if let (Some(old), Some(new)) = (old_file, new_file) {
                    // Check if this diff is for our target file
                    in_target_file = old == file_path || new == file_path;
                    log::debug!("Diff file check: old_file={}, new_file={}, target={}, match={}",
                               old, new, file_path, in_target_file);
                } else {
                    log::debug!("Could not extract file paths from diff header: {}", line);
                }
                continue;
            }

            // Skip lines if we're not in the target file
            if !in_target_file {
                continue;
            }

            // Skip file header lines for the target file
            if line.starts_with("---") || line.starts_with("+++") || line.starts_with("index ") {
                continue;
            }

            // Parse unified diff format
            if line.starts_with("@@") {
                // Hunk header: @@ -old_start,old_lines +new_start,new_lines @@
                if let Some((old_start, new_start)) = self.parse_hunk_header(&line) {
                    old_line_num = old_start;
                    new_line_num = new_start;
                }
            } else if line.starts_with('+') {
                // Added line
                lines.push(DiffLine {
                    old_line_number: None,
                    new_line_number: Some(new_line_num),
                    content: line[1..].to_string(),
                    line_type: DiffLineType::Added,
                    severity: None,
                    message: None,
                    hunk_header: None,
                });
                new_line_num += 1;
            } else if line.starts_with('-') {
                // Removed line
                lines.push(DiffLine {
                    old_line_number: Some(old_line_num),
                    new_line_number: None,
                    content: line[1..].to_string(),
                    line_type: DiffLineType::Removed,
                    severity: None,
                    message: None,
                    hunk_header: None,
                });
                old_line_num += 1;
            } else if line.starts_with(' ') {
                // Context line
                lines.push(DiffLine {
                    old_line_number: Some(old_line_num),
                    new_line_number: Some(new_line_num),
                    content: line[1..].to_string(),
                    line_type: DiffLineType::Context,
                    severity: None,
                    message: None,
                    hunk_header: None,
                });
                old_line_num += 1;
                new_line_num += 1;
            }
            // Skip other lines (e.g., "\ No newline at end of file")
        }

        log::info!("Parsed {} diff lines for {}", lines.len(), file_path);
        Ok(lines)
    }

    /// Parse hunk header to extract line numbers
    fn parse_hunk_header(&self, header: &str) -> Option<(u32, u32)> {
        // Example: @@ -1,5 +1,5 @@
        let parts: Vec<&str> = header.split_whitespace().collect();
        if parts.len() < 2 {
            return None;
        }

        let old_part = parts[1];
        let new_part = parts[2];

        // Parse old line numbers
        let old_start = if let Some(comma_idx) = old_part.find(',') {
            old_part[1..comma_idx].parse().unwrap_or(1)
        } else {
            old_part[1..].parse().unwrap_or(1)
        };

        // Parse new line numbers
        let new_start = if let Some(comma_idx) = new_part.find(',') {
            new_part[1..comma_idx].parse().unwrap_or(1)
        } else {
            new_part[1..].parse().unwrap_or(1)
        };

        Some((old_start, new_start))
    }

    /// Check if a file is binary by checking its extension
    pub fn is_binary_file(&self, path: &str) -> bool {
        let file_path = Path::new(path);

        // Check file extension for common binary file types
        match file_path.extension().and_then(|s| s.to_str()) {
            Some(ext) => {
                match ext.to_lowercase().as_str() {
                    // Image files
                    "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" | "ico" | "webp" | "tiff" | "tif" => true,
                    // Archive files
                    "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar" => true,
                    // Executables
                    "exe" | "dll" | "so" | "dylib" | "bin" | "o" => true,
                    // Other binary formats
                    "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" => true,
                    "woff" | "woff2" | "ttf" | "otf" | "eot" => true,
                    "mp3" | "mp4" | "avi" | "mov" | "wmv" | "flv" | "webm" => true,
                    "db" | "sqlite" | "sqlite3" => true,
                    _ => false, // Assume text file for unknown extensions
                }
            }
            None => false, // No extension, assume text file
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use git2::Repository;

    #[test]
    fn test_path_normalization() {
        // Open the repository (this test assumes we're in a git repo)
        if let Ok(repo) = Repository::open(".") {
            let diff_engine = DiffEngine::new(repo);
            
            // Test absolute path normalization
            let absolute_path = "/some/absolute/path/to/file.ts";
            let normalized = if absolute_path.starts_with('/') {
                // Extract relative path from absolute path
                if let Some(repo_path) = diff_engine.repository.workdir().and_then(|p| p.to_str()) {
                    absolute_path.strip_prefix(repo_path).unwrap_or(absolute_path).trim_start_matches('/')
                } else {
                    absolute_path
                }
            } else {
                absolute_path
            };
            
            // The normalized path should be relative
            println!("Absolute path: {}", absolute_path);
            println!("Normalized path: {}", normalized);
            assert!(!normalized.starts_with('/'));
        }
    }
}

