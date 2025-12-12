//! Test demonstrating Milestone 1: The Reader
//! This test runs without GPUI dependencies

#[cfg(test)]
mod milestone1_demo {
    use hyperreview::models::{Diff, FileDiff, Hunk, DiffLine, FileStatus, LineType};
    use hyperreview::services::HighlightService;

    #[test]
    fn test_diff_models() {
        println!("\n🧪 Testing Diff Domain Models\n");

        // Create a sample diff
        let diff = Diff {
            pr_id: None,
            files: vec![FileDiff {
                path: "src/main.rs".to_string(),
                old_path: None,
                status: FileStatus::Modified,
                hunks: vec![Hunk {
                    old_start: 10,
                    old_lines: 5,
                    new_start: 10,
                    new_lines: 7,
                    header: "@@ -10,5 +10,7 @@ function main".to_string(),
                    lines: vec![
                        DiffLine {
                            line_type: LineType::Context,
                            old_line_num: Some(10),
                            new_line_num: Some(10),
                            content: "fn main() {".to_string(),
                            highlight_ranges: vec![],
                        },
                        DiffLine {
                            line_type: LineType::Addition,
                            old_line_num: None,
                            new_line_num: Some(11),
                            content: "    println!(\"Hello\");".to_string(),
                            highlight_ranges: vec![],
                        },
                        DiffLine {
                            line_type: LineType::Addition,
                            old_line_num: None,
                            new_line_num: Some(12),
                            content: "    println!(\"World\");".to_string(),
                            highlight_ranges: vec![],
                        },
                    ],
                }],
                is_binary: false,
                additions: 2,
                deletions: 0,
            }],
        };

        // Test total additions
        assert_eq!(diff.total_additions(), 2);
        println!("  ✅ total_additions() = {}", diff.total_additions());

        // Test total deletions
        assert_eq!(diff.total_deletions(), 0);
        println!("  ✅ total_deletions() = {}", diff.total_deletions());

        // Test hunk counts
        let file = &diff.files[0];
        let hunk = &file.hunks[0];
        assert_eq!(hunk.additions(), 2);
        assert_eq!(hunk.deletions(), 0);
        println!("  ✅ hunk.additions() = {}", hunk.additions());
        println!("  ✅ hunk.deletions() = {}", hunk.deletions());

        println!("\n  🎉 All diff model tests passed!\n");
    }

    #[test]
    fn test_highlight_service() {
        println!("\n🧪 Testing Syntax Highlighting (Static Tests)\n");

        let service = HighlightService::new();

        // Test language detection (no GPUI dependency)
        assert_eq!(
            service.detect_language("test.rs"),
            Some(hyperreview::services::highlight::SupportedLanguage::Rust)
        );
        println!("  ✅ Detected Rust file from extension");

        assert_eq!(
            service.detect_language("test.ts"),
            Some(hyperreview::services::highlight::SupportedLanguage::TypeScript)
        );
        println!("  ✅ Detected TypeScript file from extension");

        assert_eq!(service.detect_language("test.txt"), None);
        println!("  ✅ Unknown file type returns None");

        println!("  ✅ Language detection works without GPUI");

        println!("\n  🎉 All highlight service tests passed!\n");
    }

    #[test]
    fn test_display_path() {
        println!("\n🧪 Testing File Display Path\n");

        // Test normal file
        let normal_file = FileDiff {
            path: "src/main.rs".to_string(),
            old_path: None,
            status: FileStatus::Modified,
            hunks: vec![],
            is_binary: false,
            additions: 0,
            deletions: 0,
        };
        assert_eq!(normal_file.display_path(), "src/main.rs");
        println!("  ✅ Normal file path: {}", normal_file.display_path());

        // Test renamed file
        let renamed_file = FileDiff {
            path: "src/new_name.rs".to_string(),
            old_path: Some("src/old_name.rs".to_string()),
            status: FileStatus::Renamed,
            hunks: vec![],
            is_binary: false,
            additions: 0,
            deletions: 0,
        };
        assert_eq!(renamed_file.display_path(), "src/old_name.rs → src/new_name.rs");
        println!("  ✅ Renamed file path: {}", renamed_file.display_path());

        println!("\n  🎉 All display path tests passed!\n");
    }
}
