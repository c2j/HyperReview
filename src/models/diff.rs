//! Diff domain models for HyperReview
//!
//! Represents computed changes between commits (not persisted to database).

use std::ops::Range;

use crate::models::{FileStatus, LineType, PrId};

/// Computed diff between base and head commits
#[derive(Debug, Clone)]
pub struct Diff {
    /// Associated PR ID (if any)
    pub pr_id: Option<PrId>,

    /// Changed files
    pub files: Vec<FileDiff>,
}

impl Diff {
    /// Calculate total additions across all files
    pub fn total_additions(&self) -> u32 {
        self.files.iter().map(|f| f.additions).sum()
    }

    /// Calculate total deletions across all files
    pub fn total_deletions(&self) -> u32 {
        self.files.iter().map(|f| f.deletions).sum()
    }
}

/// File-level diff information
#[derive(Debug, Clone)]
pub struct FileDiff {
    /// File path in repository
    pub path: String,

    /// Original path if renamed
    pub old_path: Option<String>,

    /// File status (Added, Modified, Deleted, Renamed)
    pub status: FileStatus,

    /// Changed regions (hunks)
    pub hunks: Vec<Hunk>,

    /// Whether file is binary
    pub is_binary: bool,

    /// Number of lines added
    pub additions: u32,

    /// Number of lines deleted
    pub deletions: u32,
}

impl FileDiff {
    /// Get display path (handles renames)
    pub fn display_path(&self) -> String {
        match (&self.old_path, &self.status) {
            (Some(old), FileStatus::Renamed) => format!("{} → {}", old, self.path),
            _ => self.path.clone(),
        }
    }
}

/// Hunk represents a continuous block of changes
#[derive(Debug, Clone)]
pub struct Hunk {
    /// Starting line number in old file
    pub old_start: u32,

    /// Number of lines in old file
    pub old_lines: u32,

    /// Starting line number in new file
    pub new_start: u32,

    /// Number of lines in new file
    pub new_lines: u32,

    /// Hunk header text (e.g., "@@ -10,5 +10,7 @@ function name")
    pub header: String,

    /// Individual line changes
    pub lines: Vec<DiffLine>,
}

impl Hunk {
    /// Get header display string
    pub fn header_display(&self) -> String {
        format!(
            "@@ -{},{} +{},{} @@",
            self.old_start, self.old_lines, self.new_start, self.new_lines
        )
    }

    /// Count additions in this hunk
    pub fn additions(&self) -> u32 {
        self.lines
            .iter()
            .filter(|l| l.line_type == LineType::Addition)
            .count() as u32
    }

    /// Count deletions in this hunk
    pub fn deletions(&self) -> u32 {
        self.lines
            .iter()
            .filter(|l| l.line_type == LineType::Deletion)
            .count() as u32
    }
}

/// Individual line in a diff
#[derive(Debug, Clone)]
pub struct DiffLine {
    /// Line type (Context, Addition, Deletion)
    pub line_type: LineType,

    /// Line number in old file (None for additions)
    pub old_line_num: Option<u32>,

    /// Line number in new file (None for deletions)
    pub new_line_num: Option<u32>,

    /// Line content (without +/- prefix)
    pub content: String,

    /// Syntax highlight ranges (populated by HighlightService)
    pub highlight_ranges: Vec<HighlightRange>,
}

/// Syntax highlight range with style information
#[derive(Debug, Clone)]
pub struct HighlightRange {
    /// Byte range in the line content
    pub range: Range<usize>,

    /// Highlight category (keyword, string, comment, etc.)
    pub category: HighlightCategory,
}

/// Syntax highlight categories from tree-sitter
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightCategory {
    Keyword,
    Function,
    Type,
    Variable,
    String,
    Number,
    Comment,
    Operator,
    Punctuation,
    Property,
    Constant,
}

impl HighlightCategory {
    /// Get RGB color for this category (Catppuccin Mocha theme)
    pub fn color(&self) -> (u8, u8, u8) {
        match self {
            HighlightCategory::Keyword => (0xc6, 0xa0, 0xf6), // Mauve
            HighlightCategory::Function => (0x89, 0xb4, 0xfa), // Blue
            HighlightCategory::Type => (0xf9, 0xe2, 0xaf),     // Yellow
            HighlightCategory::Variable => (0xcd, 0xd6, 0xf4), // Text
            HighlightCategory::String => (0xa6, 0xe3, 0xa1),   // Green
            HighlightCategory::Number => (0xfa, 0xb3, 0x87),   // Peach
            HighlightCategory::Comment => (0x6c, 0x70, 0x86),  // Overlay0
            HighlightCategory::Operator => (0x94, 0xe2, 0xd5), // Sky
            HighlightCategory::Punctuation => (0xba, 0xc2, 0xde), // Subtext1
            HighlightCategory::Property => (0x89, 0xdc, 0xeb), // Sapphire
            HighlightCategory::Constant => (0xf5, 0xc2, 0xe7), // Pink
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_totals() {
        let diff = Diff {
            pr_id: None,
            files: vec![
                FileDiff {
                    path: "file1.rs".into(),
                    old_path: None,
                    status: FileStatus::Modified,
                    hunks: vec![],
                    is_binary: false,
                    additions: 10,
                    deletions: 5,
                },
                FileDiff {
                    path: "file2.rs".into(),
                    old_path: None,
                    status: FileStatus::Added,
                    hunks: vec![],
                    is_binary: false,
                    additions: 20,
                    deletions: 0,
                },
            ],
        };

        assert_eq!(diff.total_additions(), 30);
        assert_eq!(diff.total_deletions(), 5);
    }

    #[test]
    fn test_file_diff_display_path() {
        let renamed = FileDiff {
            path: "new.rs".into(),
            old_path: Some("old.rs".into()),
            status: FileStatus::Renamed,
            hunks: vec![],
            is_binary: false,
            additions: 0,
            deletions: 0,
        };

        assert_eq!(renamed.display_path(), "old.rs → new.rs");

        let normal = FileDiff {
            path: "file.rs".into(),
            old_path: None,
            status: FileStatus::Modified,
            hunks: vec![],
            is_binary: false,
            additions: 0,
            deletions: 0,
        };

        assert_eq!(normal.display_path(), "file.rs");
    }

    #[test]
    fn test_hunk_counts() {
        let hunk = Hunk {
            old_start: 10,
            old_lines: 5,
            new_start: 10,
            new_lines: 7,
            header: "function test".into(),
            lines: vec![
                DiffLine {
                    line_type: LineType::Context,
                    old_line_num: Some(10),
                    new_line_num: Some(10),
                    content: "line 1".into(),
                    highlight_ranges: vec![],
                },
                DiffLine {
                    line_type: LineType::Deletion,
                    old_line_num: Some(11),
                    new_line_num: None,
                    content: "old line".into(),
                    highlight_ranges: vec![],
                },
                DiffLine {
                    line_type: LineType::Addition,
                    old_line_num: None,
                    new_line_num: Some(11),
                    content: "new line 1".into(),
                    highlight_ranges: vec![],
                },
                DiffLine {
                    line_type: LineType::Addition,
                    old_line_num: None,
                    new_line_num: Some(12),
                    content: "new line 2".into(),
                    highlight_ranges: vec![],
                },
            ],
        };

        assert_eq!(hunk.additions(), 2);
        assert_eq!(hunk.deletions(), 1);
    }
}
