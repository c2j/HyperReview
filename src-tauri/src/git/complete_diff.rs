use crate::models::{DiffLine, DiffLineType};
use crate::errors::HyperReviewError;
use git2::Repository;

/// Enhanced diff engine that provides complete file comparison with proper line numbering
pub struct CompleteDiffEngine {
    repository: Repository,
}

impl CompleteDiffEngine {
    pub fn new(repository: Repository) -> Self {
        Self { repository }
    }

    /// Compute complete diff showing new file version with proper line number mapping
    /// This replaces the old algorithm that only showed changed hunks
    pub fn compute_complete_diff(
        &self,
        file_path: &str,
        old_commit: &str,
        new_commit: &str,
    ) -> Result<Vec<DiffLine>, HyperReviewError> {
        log::info!("Computing complete diff for file: {} between {} and {}", file_path, old_commit, new_commit);

        // Check if file is binary
        if self.is_binary_file(file_path) {
            return Err(HyperReviewError::Other {
                message: "Cannot compute diff for binary file".to_string(),
            });
        }

        // Get file content from both commits
        let old_content = self.get_file_content_at_commit(old_commit, file_path)?;
        let new_content = self.get_file_content_at_commit(new_commit, file_path)?;

        // Split into lines
        let old_lines: Vec<&str> = old_content.lines().collect();
        let new_lines: Vec<&str> = new_content.lines().collect();

        // Compute diff using Myers' algorithm or similar
        let diff_result = self.compute_line_diff(&old_lines, &new_lines);

        // Convert to DiffLine format with proper line numbering
        let mut result = Vec::new();
        let mut old_line_num = 1u32;
        let mut new_line_num = 1u32;

        for diff_op in diff_result {
            match diff_op {
                DiffOperation::Equal { old_line: _, new_line } => {
                    result.push(DiffLine {
                        old_line_number: Some(old_line_num),
                        new_line_number: Some(new_line_num),
                        content: new_line.to_string(),
                        line_type: DiffLineType::Context,
                        severity: None,
                        message: None,
                        hunk_header: None,
                    });
                    old_line_num += 1;
                    new_line_num += 1;
                }
                DiffOperation::Delete { old_line } => {
                    result.push(DiffLine {
                        old_line_number: Some(old_line_num),
                        new_line_number: None,
                        content: old_line.to_string(),
                        line_type: DiffLineType::Removed,
                        severity: None,
                        message: None,
                        hunk_header: None,
                    });
                    old_line_num += 1;
                }
                DiffOperation::Insert { new_line } => {
                    result.push(DiffLine {
                        old_line_number: None,
                        new_line_number: Some(new_line_num),
                        content: new_line.to_string(),
                        line_type: DiffLineType::Added,
                        severity: None,
                        message: None,
                        hunk_header: None,
                    });
                    new_line_num += 1;
                }
            }
        }

        log::info!("Complete diff computed: {} lines", result.len());
        Ok(result)
    }

    /// Get file content at specific commit
    fn get_file_content_at_commit(&self, commit_ref: &str, file_path: &str) -> Result<String, HyperReviewError> {
        let object = self.repository.revparse_single(commit_ref)?;
        let commit = self.repository.find_commit(object.id())?;
        let tree = commit.tree()?;
        
        // Try to find the file in the tree
        match tree.get_path(std::path::Path::new(file_path)) {
            Ok(tree_entry) => {
                let object = tree_entry.to_object(&self.repository)?;
                if let Some(blob) = object.as_blob() {
                    let content = std::str::from_utf8(blob.content())
                        .map_err(|_| HyperReviewError::Other {
                            message: "File content is not valid UTF-8".to_string(),
                        })?;
                    Ok(content.to_string())
                } else {
                    Err(HyperReviewError::Other {
                        message: "Path is not a file".to_string(),
                    })
                }
            }
            Err(_) => {
                // File doesn't exist in this commit, return empty content
                Ok(String::new())
            }
        }
    }

    /// Simple line-based diff algorithm (Myers' algorithm would be better)
    fn compute_line_diff<'a>(&self, old_lines: &[&'a str], new_lines: &[&'a str]) -> Vec<DiffOperation<'a>> {
        let mut result = Vec::new();
        let mut old_idx = 0;
        let mut new_idx = 0;

        // Simple greedy algorithm - can be improved with Myers' algorithm
        while old_idx < old_lines.len() || new_idx < new_lines.len() {
            if old_idx < old_lines.len() && new_idx < new_lines.len() && old_lines[old_idx] == new_lines[new_idx] {
                // Lines are equal
                result.push(DiffOperation::Equal {
                    old_line: old_lines[old_idx],
                    new_line: new_lines[new_idx],
                });
                old_idx += 1;
                new_idx += 1;
            } else if old_idx < old_lines.len() && (new_idx >= new_lines.len() || self.should_delete_first(&old_lines[old_idx..], &new_lines[new_idx..])) {
                // Line was deleted
                result.push(DiffOperation::Delete {
                    old_line: old_lines[old_idx],
                });
                old_idx += 1;
            } else if new_idx < new_lines.len() {
                // Line was added
                result.push(DiffOperation::Insert {
                    new_line: new_lines[new_idx],
                });
                new_idx += 1;
            }
        }

        result
    }

    /// Heuristic to determine if we should delete first or insert first
    fn should_delete_first(&self, old_remaining: &[&str], new_remaining: &[&str]) -> bool {
        // Simple heuristic: if the current old line appears later in new, delete it first
        if old_remaining.len() > 1 {
            for (i, &new_line) in new_remaining.iter().enumerate() {
                if new_line == old_remaining[0] {
                    return i > 0; // Delete first if new line appears after some additions
                }
            }
        }
        true
    }

    /// Check if a file is binary by checking its extension
    pub fn is_binary_file(&self, path: &str) -> bool {
        let file_path = std::path::Path::new(path);

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

/// Diff operations for line-based diff
#[derive(Debug)]
enum DiffOperation<'a> {
    Equal { old_line: &'a str, new_line: &'a str },
    Delete { old_line: &'a str },
    Insert { new_line: &'a str },
}