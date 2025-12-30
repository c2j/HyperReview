use crate::models::task::TaskItem;
use anyhow::Result;

pub fn parse_task_text(text: &str) -> Result<Vec<TaskItem>> {
    let mut items = Vec::new();
    
    for line in text.lines() {
        let original_line = line;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Check for tab separator first (on original line before trim)
        let (file_path, comment) = if let Some((f, c)) = original_line.split_once('\t') {
            // If the part before tab is empty (line starts with tab), skip this line
            if f.is_empty() {
                continue;
            }
            let f_trimmed = f.trim();
            if f_trimmed.is_empty() {
                continue; // Skip lines with empty file path before tab
            }
            (f_trimmed, Some(c.trim()))
        } else {
            // No tab found - check if original line starts with tab (invalid)
            if original_line.starts_with('\t') {
                continue; // Skip lines that start with tab but have no file path
            }
            
            // Try double space as separator on trimmed line
            if let Some(idx) = line.find("  ") {
                let (f, c) = line.split_at(idx);
                let f_trimmed = f.trim();
                if f_trimmed.is_empty() {
                    continue; // Skip lines with empty file path before spaces
                }
                (f_trimmed, Some(c.trim()))
            } else {
                // Check if the entire line is just a comment (no file path)
                // This handles cases like "  Just a comment" or lines that don't look like file paths
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue; // Skip empty lines
                }
                
                // Additional validation: check if this looks like a file path
                // File paths should contain at least one '/' or '\' or have a file extension
                if !trimmed.contains('/') && !trimmed.contains('\\') && !trimmed.contains('.') {
                    // This doesn't look like a file path, skip it
                    continue;
                }
                
                (trimmed, None)
            }
        };

        items.push(TaskItem {
            file: file_path.to_string(),
            line_range: None,
            preset_comment: comment.map(|s| s.to_string()),
            severity: None,
            tags: Vec::new(),
            reviewed: false,
            comments: Vec::new(),
        });
    }
    
    Ok(items)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_list() {
        let input = "src/main.rs\nsrc/lib.rs";
        let result = parse_task_text(input).expect("Failed to parse");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].file, "src/main.rs");
        assert_eq!(result[1].file, "src/lib.rs");
        
        // Verify default values
        assert_eq!(result[0].preset_comment, None);
        assert_eq!(result[0].reviewed, false);
        assert_eq!(result[0].comments.len(), 0);
        assert_eq!(result[0].tags.len(), 0);
    }

    #[test]
    fn test_parse_with_tab_separated_comments() {
        let input = "src/main.rs\tCheck this file\nsrc/lib.rs\tReview API";
        let result = parse_task_text(input).expect("Failed to parse");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].file, "src/main.rs");
        assert_eq!(result[0].preset_comment, Some("Check this file".to_string()));
        assert_eq!(result[1].file, "src/lib.rs");
        assert_eq!(result[1].preset_comment, Some("Review API".to_string()));
    }

    #[test]
    fn test_parse_with_double_space_comments() {
        let input = "src/main.rs  Check this file\nsrc/lib.rs  Review API";
        let result = parse_task_text(input).expect("Failed to parse");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].file, "src/main.rs");
        assert_eq!(result[0].preset_comment, Some("Check this file".to_string()));
        assert_eq!(result[1].file, "src/lib.rs");
        assert_eq!(result[1].preset_comment, Some("Review API".to_string()));
    }

    #[test]
    fn test_parse_mixed_separators() {
        let input = "src/main.rs\tTab comment\nsrc/lib.rs  Space comment\nsrc/utils.rs";
        let result = parse_task_text(input).expect("Failed to parse");
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].preset_comment, Some("Tab comment".to_string()));
        assert_eq!(result[1].preset_comment, Some("Space comment".to_string()));
        assert_eq!(result[2].preset_comment, None);
    }

    #[test]
    fn test_parse_empty_lines() {
        let input = "src/main.rs\n\n\nsrc/lib.rs\n\nsrc/utils.rs";
        let result = parse_task_text(input).expect("Failed to parse");
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].file, "src/main.rs");
        assert_eq!(result[1].file, "src/lib.rs");
        assert_eq!(result[2].file, "src/utils.rs");
    }

    #[test]
    fn test_parse_whitespace_handling() {
        let input = "  src/main.rs  \t  Check this  \n   src/lib.rs   \t   Review API   ";
        let result = parse_task_text(input).expect("Failed to parse");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].file, "src/main.rs");
        assert_eq!(result[0].preset_comment, Some("Check this".to_string()));
        assert_eq!(result[1].file, "src/lib.rs");
        assert_eq!(result[1].preset_comment, Some("Review API".to_string()));
    }

    #[test]
    fn test_parse_complex_file_paths() {
        let input = "src/main/java/com/example/MyClass.java\tCheck constructor\nlib/utils.js\napp/components/UserProfile.tsx  Review props";
        let result = parse_task_text(input).expect("Failed to parse");
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].file, "src/main/java/com/example/MyClass.java");
        assert_eq!(result[0].preset_comment, Some("Check constructor".to_string()));
        assert_eq!(result[1].file, "lib/utils.js");
        assert_eq!(result[1].preset_comment, None);
        assert_eq!(result[2].file, "app/components/UserProfile.tsx");
        assert_eq!(result[2].preset_comment, Some("Review props".to_string()));
    }

    #[test] 
    fn test_debug_parsing() {
        let input = "\tOnly comment";
        println!("Input: {:?}", input);
        
        // Test what split_once does
        let parts = input.split_once('\t');
        println!("split_once result: {:?}", parts);
        if let Some((f, c)) = parts {
            println!("First part: '{:?}' (len: {})", f, f.len());
            println!("Second part: '{:?}' (len: {})", c, c.len());
            println!("First part is empty: {}", f.is_empty());
            println!("First part trimmed: '{:?}' (len: {})", f.trim(), f.trim().len());
        }
        
        let result = parse_task_text(input);
        println!("Parsed result: {:?}", result);
        
        // Should return empty vector since line starts with tab
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_parse_empty_file_path() {
        let input = "\tOnly comment\nsrc/main.rs";
        let result = parse_task_text(input).expect("Failed to parse only comments");
        
        // The first line "\tOnly comment" should be skipped because it starts with tab
        // The second line "src/main.rs" should be parsed as a valid file
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].file, "src/main.rs");
        assert_eq!(result[0].preset_comment, None);
    }

    #[test]
    fn test_parse_large_input() {
        // Test with 1000 lines to ensure performance
        let mut input = String::new();
        for i in 0..1000 {
            input.push_str(&format!("src/file_{}.rs\tComment for file {}\n", i, i));
        }
        
        let result = parse_task_text(&input).expect("Failed to parse large input");
        assert_eq!(result.len(), 1000);
        assert_eq!(result[0].file, "src/file_0.rs");
        assert_eq!(result[999].file, "src/file_999.rs");
        assert_eq!(result[0].preset_comment, Some("Comment for file 0".to_string()));
        assert_eq!(result[999].preset_comment, Some("Comment for file 999".to_string()));
    }

    #[test]
    fn test_parse_unicode_content() {
        let input = "src/main.rs\t检查这个文件\nsrc/lib.rs\tReview 这个 API";
        let result = parse_task_text(input).expect("Failed to parse unicode");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].preset_comment, Some("检查这个文件".to_string()));
        assert_eq!(result[1].preset_comment, Some("Review 这个 API".to_string()));
    }

    #[test]
    fn test_parse_special_characters() {
        let input = "src/main.rs\tCheck: special chars! @#$%^&*()\nsrc/lib.rs\tReview [brackets] and {braces}";
        let result = parse_task_text(input).expect("Failed to parse special chars");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].preset_comment, Some("Check: special chars! @#$%^&*()".to_string()));
        assert_eq!(result[1].preset_comment, Some("Review [brackets] and {braces}".to_string()));
    }

    #[test]
    fn test_parse_single_line() {
        let input = "src/main.rs";
        let result = parse_task_text(input).expect("Failed to parse single line");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].file, "src/main.rs");
        assert_eq!(result[0].preset_comment, None);
    }

    #[test]
    fn test_parse_only_comments() {
        let input = "\tComment without file\n\tAnother comment";
        let result = parse_task_text(input).expect("Failed to parse only comments");
        assert_eq!(result.len(), 0); // Should skip lines with empty file paths
    }

    #[test]
    fn test_parse_windows_paths() {
        let input = "src\\main.rs\tWindows path\nsrc\\lib.rs";
        let result = parse_task_text(input).expect("Failed to parse Windows paths");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].file, "src\\main.rs");
        assert_eq!(result[0].preset_comment, Some("Windows path".to_string()));
        assert_eq!(result[1].file, "src\\lib.rs");
    }

    // Performance test for 2000 lines (requirement: <500ms)
    #[test]
    fn test_parse_performance_2000_lines() {
        use std::time::Instant;
        
        // Create 2000 lines of test data
        let mut input = String::new();
        for i in 0..2000 {
            input.push_str(&format!("src/file_{}.rs\tThis is a comment for file {} that contains some review notes\n", i, i));
        }
        
        let start = Instant::now();
        let result = parse_task_text(&input).expect("Failed to parse 2000 lines");
        let duration = start.elapsed();
        
        assert_eq!(result.len(), 2000);
        assert!(duration.as_millis() < 500, "Parsing 2000 lines took {}ms, should be <500ms", duration.as_millis());
    }
}
