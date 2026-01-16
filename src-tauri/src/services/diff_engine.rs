// Diff Generation Engine
// Handles unified and side-by-side diff views with syntax highlighting

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use log::{info, warn, error, debug};

use crate::errors::HyperReviewError;
use crate::models::gerrit::{ChangeFile, FileDiff, DiffHunk, DiffLine, DiffLineType};

/// Diff engine for generating and processing file diffs
pub struct DiffEngine {
    syntax_highlighter: SyntaxHighlighter,
    diff_config: DiffConfig,
}

/// Configuration for diff generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffConfig {
    pub context_lines: u32,
    pub tab_size: u32,
    pub show_whitespace: bool,
    pub ignore_whitespace: bool,
    pub word_wrap: bool,
    pub max_line_length: u32,
}

impl Default for DiffConfig {
    fn default() -> Self {
        Self {
            context_lines: 3,
            tab_size: 4,
            show_whitespace: false,
            ignore_whitespace: false,
            word_wrap: false,
            max_line_length: 120,
        }
    }
}

/// Syntax highlighter for code files
pub struct SyntaxHighlighter {
    language_map: HashMap<String, String>,
}

/// Diff view types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DiffViewType {
    Unified,
    SideBySide,
    Split,
}

/// Processed diff for display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedDiff {
    pub view_type: DiffViewType,
    pub file_path: String,
    pub old_file_name: Option<String>,
    pub new_file_name: String,
    pub language: Option<String>,
    pub hunks: Vec<ProcessedHunk>,
    pub stats: DiffStats,
}

/// Processed diff hunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedHunk {
    pub header: String,
    pub old_start: u32,
    pub old_count: u32,
    pub new_start: u32,
    pub new_count: u32,
    pub lines: Vec<ProcessedLine>,
}

/// Processed diff line with highlighting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedLine {
    pub line_type: DiffLineType,
    pub old_line_number: Option<u32>,
    pub new_line_number: Option<u32>,
    pub content: String,
    pub highlighted_content: Option<String>,
    pub is_highlighted: bool,
    pub has_comments: bool,
    pub comment_count: u32,
}

/// Diff statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffStats {
    pub lines_added: u32,
    pub lines_removed: u32,
    pub lines_modified: u32,
    pub total_hunks: u32,
    pub file_size_old: u64,
    pub file_size_new: u64,
}

/// Line mapping for navigation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineMapping {
    pub old_to_new: HashMap<u32, u32>,
    pub new_to_old: HashMap<u32, u32>,
    pub diff_line_to_old: HashMap<u32, Option<u32>>,
    pub diff_line_to_new: HashMap<u32, Option<u32>>,
}

impl DiffEngine {
    /// Create a new diff engine
    pub fn new(config: DiffConfig) -> Self {
        Self {
            syntax_highlighter: SyntaxHighlighter::new(),
            diff_config: config,
        }
    }

    /// Generate unified diff view
    pub fn generate_unified_diff(
        &self,
        change_file: &ChangeFile,
    ) -> Result<ProcessedDiff, HyperReviewError> {
        info!("Generating unified diff for file: {}", change_file.file_path);

        let language = self.detect_language(&change_file.file_path);
        let mut processed_hunks = Vec::new();
        let mut stats = DiffStats::default();

        for hunk in &change_file.diff.hunks {
            let processed_hunk = self.process_hunk_unified(hunk, &language)?;
            self.update_stats(&mut stats, &processed_hunk);
            processed_hunks.push(processed_hunk);
        }

        stats.total_hunks = processed_hunks.len() as u32;
        stats.file_size_old = change_file.old_content.as_ref().map_or(0, |c| c.len() as u64);
        stats.file_size_new = change_file.new_content.as_ref().map_or(0, |c| c.len() as u64);

        Ok(ProcessedDiff {
            view_type: DiffViewType::Unified,
            file_path: change_file.file_path.clone(),
            old_file_name: None, // TODO: Handle renames
            new_file_name: change_file.file_path.clone(),
            language,
            hunks: processed_hunks,
            stats,
        })
    }

    /// Generate side-by-side diff view
    pub fn generate_side_by_side_diff(
        &self,
        change_file: &ChangeFile,
    ) -> Result<ProcessedDiff, HyperReviewError> {
        info!("Generating side-by-side diff for file: {}", change_file.file_path);

        let language = self.detect_language(&change_file.file_path);
        let mut processed_hunks = Vec::new();
        let mut stats = DiffStats::default();

        for hunk in &change_file.diff.hunks {
            let processed_hunk = self.process_hunk_side_by_side(hunk, &language)?;
            self.update_stats(&mut stats, &processed_hunk);
            processed_hunks.push(processed_hunk);
        }

        stats.total_hunks = processed_hunks.len() as u32;
        stats.file_size_old = change_file.old_content.as_ref().map_or(0, |c| c.len() as u64);
        stats.file_size_new = change_file.new_content.as_ref().map_or(0, |c| c.len() as u64);

        Ok(ProcessedDiff {
            view_type: DiffViewType::SideBySide,
            file_path: change_file.file_path.clone(),
            old_file_name: None,
            new_file_name: change_file.file_path.clone(),
            language,
            hunks: processed_hunks,
            stats,
        })
    }

    /// Create line mapping for navigation
    pub fn create_line_mapping(&self, diff: &ProcessedDiff) -> LineMapping {
        debug!("Creating line mapping for file: {}", diff.file_path);

        let mut old_to_new = HashMap::new();
        let mut new_to_old = HashMap::new();
        let mut diff_line_to_old = HashMap::new();
        let mut diff_line_to_new = HashMap::new();

        let mut diff_line_index = 0u32;

        for hunk in &diff.hunks {
            for line in &hunk.lines {
                match line.line_type {
                    DiffLineType::Context => {
                        if let (Some(old_num), Some(new_num)) = (line.old_line_number, line.new_line_number) {
                            old_to_new.insert(old_num, new_num);
                            new_to_old.insert(new_num, old_num);
                            diff_line_to_old.insert(diff_line_index, Some(old_num));
                            diff_line_to_new.insert(diff_line_index, Some(new_num));
                        }
                    }
                    DiffLineType::Removed => {
                        if let Some(old_num) = line.old_line_number {
                            diff_line_to_old.insert(diff_line_index, Some(old_num));
                            diff_line_to_new.insert(diff_line_index, None);
                        }
                    }
                    DiffLineType::Added => {
                        if let Some(new_num) = line.new_line_number {
                            diff_line_to_old.insert(diff_line_index, None);
                            diff_line_to_new.insert(diff_line_index, Some(new_num));
                        }
                    }
                    _ => {}
                }
                diff_line_index += 1;
            }
        }

        LineMapping {
            old_to_new,
            new_to_old,
            diff_line_to_old,
            diff_line_to_new,
        }
    }

    /// Navigate to specific line in diff
    pub fn navigate_to_line(
        &self,
        diff: &ProcessedDiff,
        line_number: u32,
        is_old_file: bool,
    ) -> Option<(usize, usize)> { // Returns (hunk_index, line_index)
        debug!("Navigating to line {} in {} file", line_number, if is_old_file { "old" } else { "new" });

        for (hunk_index, hunk) in diff.hunks.iter().enumerate() {
            for (line_index, line) in hunk.lines.iter().enumerate() {
                let target_line = if is_old_file {
                    line.old_line_number
                } else {
                    line.new_line_number
                };

                if target_line == Some(line_number) {
                    return Some((hunk_index, line_index));
                }
            }
        }

        None
    }

    /// Get diff context around a specific line
    pub fn get_context_around_line<'a>(
        &self,
        diff: &'a ProcessedDiff,
        line_number: u32,
        is_old_file: bool,
        context_size: u32,
    ) -> Option<Vec<&'a ProcessedLine>> {
        if let Some((hunk_index, line_index)) = self.navigate_to_line(diff, line_number, is_old_file) {
            let hunk = &diff.hunks[hunk_index];
            let start = line_index.saturating_sub(context_size as usize);
            let end = std::cmp::min(line_index + context_size as usize + 1, hunk.lines.len());
            
            return Some(hunk.lines[start..end].iter().collect());
        }

        None
    }

    // Private helper methods

    fn process_hunk_unified(
        &self,
        hunk: &DiffHunk,
        language: &Option<String>,
    ) -> Result<ProcessedHunk, HyperReviewError> {
        let header = format!("@@ -{},{} +{},{} @@", hunk.old_start, hunk.old_count, hunk.new_start, hunk.new_count);
        let mut processed_lines = Vec::new();

        for line in &hunk.lines {
            let highlighted_content = if let Some(lang) = language {
                self.syntax_highlighter.highlight(&line.content, lang)?
            } else {
                None
            };

            let is_highlighted = highlighted_content.is_some();

            processed_lines.push(ProcessedLine {
                line_type: line.line_type.clone(),
                old_line_number: line.old_line_number,
                new_line_number: line.new_line_number,
                content: line.content.clone(),
                highlighted_content,
                is_highlighted,
                has_comments: false, // TODO: Check for comments
                comment_count: 0,    // TODO: Count comments
            });
        }

        Ok(ProcessedHunk {
            header,
            old_start: hunk.old_start,
            old_count: hunk.old_count,
            new_start: hunk.new_start,
            new_count: hunk.new_count,
            lines: processed_lines,
        })
    }

    fn process_hunk_side_by_side(
        &self,
        hunk: &DiffHunk,
        language: &Option<String>,
    ) -> Result<ProcessedHunk, HyperReviewError> {
        // For side-by-side, we need to align lines properly
        // This is more complex than unified diff
        self.process_hunk_unified(hunk, language) // Simplified for now
    }

    fn detect_language(&self, file_path: &str) -> Option<String> {
        self.syntax_highlighter.detect_language(file_path)
    }

    fn update_stats(&self, stats: &mut DiffStats, hunk: &ProcessedHunk) {
        for line in &hunk.lines {
            match line.line_type {
                DiffLineType::Added => stats.lines_added += 1,
                DiffLineType::Removed => stats.lines_removed += 1,
                DiffLineType::Context => {}, // Context lines don't count as changes
                _ => {},
            }
        }
    }
}

impl SyntaxHighlighter {
    fn new() -> Self {
        let mut language_map = HashMap::new();
        
        // Common file extensions to language mappings
        language_map.insert("rs".to_string(), "rust".to_string());
        language_map.insert("java".to_string(), "java".to_string());
        language_map.insert("js".to_string(), "javascript".to_string());
        language_map.insert("ts".to_string(), "typescript".to_string());
        language_map.insert("py".to_string(), "python".to_string());
        language_map.insert("cpp".to_string(), "cpp".to_string());
        language_map.insert("c".to_string(), "c".to_string());
        language_map.insert("h".to_string(), "c".to_string());
        language_map.insert("hpp".to_string(), "cpp".to_string());
        language_map.insert("go".to_string(), "go".to_string());
        language_map.insert("rb".to_string(), "ruby".to_string());
        language_map.insert("php".to_string(), "php".to_string());
        language_map.insert("html".to_string(), "html".to_string());
        language_map.insert("css".to_string(), "css".to_string());
        language_map.insert("scss".to_string(), "scss".to_string());
        language_map.insert("json".to_string(), "json".to_string());
        language_map.insert("xml".to_string(), "xml".to_string());
        language_map.insert("yaml".to_string(), "yaml".to_string());
        language_map.insert("yml".to_string(), "yaml".to_string());
        language_map.insert("md".to_string(), "markdown".to_string());
        language_map.insert("sh".to_string(), "bash".to_string());
        language_map.insert("sql".to_string(), "sql".to_string());

        Self { language_map }
    }

    fn detect_language(&self, file_path: &str) -> Option<String> {
        if let Some(extension) = std::path::Path::new(file_path).extension() {
            if let Some(ext_str) = extension.to_str() {
                return self.language_map.get(ext_str).cloned();
            }
        }
        None
    }

    fn highlight(&self, content: &str, language: &str) -> Result<Option<String>, HyperReviewError> {
        // TODO: Implement actual syntax highlighting using tree-sitter
        // For now, return None to indicate no highlighting
        debug!("Syntax highlighting requested for language: {}", language);
        Ok(None)
    }
}

impl Default for DiffStats {
    fn default() -> Self {
        Self {
            lines_added: 0,
            lines_removed: 0,
            lines_modified: 0,
            total_hunks: 0,
            file_size_old: 0,
            file_size_new: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::gerrit::{FileChangeType, FileDiff, DiffHunk, DiffLine, DiffLineType};

    fn create_test_change_file() -> ChangeFile {
        let mut diff = FileDiff::default();
        diff.hunks = vec![
            DiffHunk {
                old_start: 1,
                old_count: 3,
                new_start: 1,
                new_count: 4,
                lines: vec![
                    DiffLine {
                        line_type: DiffLineType::Context,
                        old_line_number: Some(1),
                        new_line_number: Some(1),
                        content: "fn main() {".to_string(),
                    },
                    DiffLine {
                        line_type: DiffLineType::Removed,
                        old_line_number: Some(2),
                        new_line_number: None,
                        content: "    println!(\"Hello\");".to_string(),
                    },
                    DiffLine {
                        line_type: DiffLineType::Added,
                        old_line_number: None,
                        new_line_number: Some(2),
                        content: "    println!(\"Hello, World!\");".to_string(),
                    },
                    DiffLine {
                        line_type: DiffLineType::Context,
                        old_line_number: Some(3),
                        new_line_number: Some(3),
                        content: "}".to_string(),
                    },
                ],
            },
        ];

        ChangeFile {
            id: "test-file-id".to_string(),
            change_id: "test-change".to_string(),
            patch_set_number: 1,
            file_path: "src/main.rs".to_string(),
            change_type: FileChangeType::Modified,
            old_content: Some("fn main() {\n    println!(\"Hello\");\n}".to_string()),
            new_content: Some("fn main() {\n    println!(\"Hello, World!\");\n}".to_string()),
            diff,
            file_size: 42,
            downloaded_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }

    #[test]
    fn test_diff_engine_creation() {
        let config = DiffConfig::default();
        let engine = DiffEngine::new(config);
        assert_eq!(engine.diff_config.context_lines, 3);
        assert_eq!(engine.diff_config.tab_size, 4);
    }

    #[test]
    fn test_language_detection() {
        let highlighter = SyntaxHighlighter::new();
        assert_eq!(highlighter.detect_language("main.rs"), Some("rust".to_string()));
        assert_eq!(highlighter.detect_language("App.java"), Some("java".to_string()));
        assert_eq!(highlighter.detect_language("script.js"), Some("javascript".to_string()));
        assert_eq!(highlighter.detect_language("unknown.xyz"), None);
    }

    #[test]
    fn test_unified_diff_generation() {
        let engine = DiffEngine::new(DiffConfig::default());
        let change_file = create_test_change_file();

        let result = engine.generate_unified_diff(&change_file);
        assert!(result.is_ok());

        let diff = result.unwrap();
        assert_eq!(diff.view_type, DiffViewType::Unified);
        assert_eq!(diff.file_path, "src/main.rs");
        assert_eq!(diff.language, Some("rust".to_string()));
        assert_eq!(diff.hunks.len(), 1);
        assert_eq!(diff.stats.lines_added, 1);
        assert_eq!(diff.stats.lines_removed, 1);
    }

    #[test]
    fn test_line_mapping_creation() {
        let engine = DiffEngine::new(DiffConfig::default());
        let change_file = create_test_change_file();
        let diff = engine.generate_unified_diff(&change_file).unwrap();

        let mapping = engine.create_line_mapping(&diff);
        
        // Check context line mapping
        assert_eq!(mapping.old_to_new.get(&1), Some(&1));
        assert_eq!(mapping.new_to_old.get(&1), Some(&1));
        
        // Check that we have mappings
        assert!(!mapping.diff_line_to_old.is_empty());
        assert!(!mapping.diff_line_to_new.is_empty());
    }

    #[test]
    fn test_navigation_to_line() {
        let engine = DiffEngine::new(DiffConfig::default());
        let change_file = create_test_change_file();
        let diff = engine.generate_unified_diff(&change_file).unwrap();

        // Navigate to line 1 in old file (context line)
        let result = engine.navigate_to_line(&diff, 1, true);
        assert_eq!(result, Some((0, 0))); // First hunk, first line

        // Navigate to line 2 in old file (removed line)
        let result = engine.navigate_to_line(&diff, 2, true);
        assert_eq!(result, Some((0, 1))); // First hunk, second line

        // Navigate to non-existent line
        let result = engine.navigate_to_line(&diff, 100, true);
        assert_eq!(result, None);
    }

    #[test]
    fn test_context_around_line() {
        let engine = DiffEngine::new(DiffConfig::default());
        let change_file = create_test_change_file();
        let diff = engine.generate_unified_diff(&change_file).unwrap();

        let context = engine.get_context_around_line(&diff, 2, true, 1);
        assert!(context.is_some());
        
        let lines = context.unwrap();
        assert!(lines.len() >= 1); // Should have at least the target line
    }
}