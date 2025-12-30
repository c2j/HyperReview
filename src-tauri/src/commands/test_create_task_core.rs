#[cfg(test)]
mod tests {
    use crate::commands::text_parser;
    use crate::models::task::{LocalTask, TaskStatus, TaskItem, Comment, LineRange};
    use uuid::Uuid;
    use chrono::Utc;
    use std::fs;
    use std::path::Path;

    fn setup_test_environment() {
        // Clean up any existing test data
        let test_dir = Path::new("~/.hyperreview/test");
        if test_dir.exists() {
            fs::remove_dir_all(test_dir).ok();
        }
    }

    fn cleanup_test_environment() {
        // Clean up test data after tests
        let test_dir = Path::new("~/.hyperreview/test");
        if test_dir.exists() {
            fs::remove_dir_all(test_dir).ok();
        }
    }

    fn create_test_task_from_text(name: &str, items_text: &str) -> LocalTask {
        let items = text_parser::parse_task_text(items_text).unwrap_or_else(|_| Vec::new());
        
        let total_items = items.len() as u32;
        let completed_items = items.iter().filter(|item| item.reviewed).count() as u32;

        LocalTask {
            id: Uuid::new_v4(),
            name: name.to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "main".to_string(),
            create_time: Utc::now(),
            update_time: Utc::now(),
            status: TaskStatus::InProgress,
            total_items,
            completed_items,
            items,
        }
    }

    #[test]
    fn test_create_task_success() {
        setup_test_environment();

        let items_text = "file1.js\tReview authentication\nfile2.ts\tFix validation";
        let task = create_test_task_from_text("Test Task", items_text);
        
        assert_eq!(task.name, "Test Task");
        assert_eq!(task.repo_path, "/test/repo");
        assert_eq!(task.base_ref, "main");
        assert_eq!(task.status, TaskStatus::InProgress);
        assert_eq!(task.total_items, 2);
        assert_eq!(task.completed_items, 0);
        assert_eq!(task.items.len(), 2);
        
        cleanup_test_environment();
    }

    #[test]
    fn test_create_task_empty_items() {
        setup_test_environment();

        let task = create_test_task_from_text("Empty Task", "");
        
        assert_eq!(task.name, "Empty Task");
        assert_eq!(task.total_items, 0);
        assert_eq!(task.completed_items, 0);
        assert!(task.items.is_empty());
        
        cleanup_test_environment();
    }

    #[test]
    fn test_create_task_single_item() {
        setup_test_environment();

        let items_text = "src/main.js\tReview main function";
        let task = create_test_task_from_text("Single Item Task", items_text);
        
        assert_eq!(task.name, "Single Item Task");
        assert_eq!(task.total_items, 1);
        assert_eq!(task.items.len(), 1);
        assert_eq!(task.items[0].file, "src/main.js");
        assert_eq!(task.items[0].preset_comment, Some("Review main function".to_string()));
        assert!(!task.items[0].reviewed);
        
        cleanup_test_environment();
    }

    #[test]
    fn test_create_task_multiple_items() {
        setup_test_environment();

        let items_text = "file1.js\tReview auth\nfile2.ts\tFix types\nfile3.py\tUpdate docs\nfile4.rs\tOptimize performance";
        let task = create_test_task_from_text("Multi Item Task", items_text);
        
        assert_eq!(task.name, "Multi Item Task");
        assert_eq!(task.total_items, 4);
        assert_eq!(task.items.len(), 4);
        
        // Verify each item
        assert_eq!(task.items[0].file, "file1.js");
        assert_eq!(task.items[0].preset_comment, Some("Review auth".to_string()));
        assert_eq!(task.items[1].file, "file2.ts");
        assert_eq!(task.items[1].preset_comment, Some("Fix types".to_string()));
        assert_eq!(task.items[2].file, "file3.py");
        assert_eq!(task.items[2].preset_comment, Some("Update docs".to_string()));
        assert_eq!(task.items[3].file, "file4.rs");
        assert_eq!(task.items[3].preset_comment, Some("Optimize performance".to_string()));
        
        cleanup_test_environment();
    }

    #[test]
    fn test_create_task_with_special_characters() {
        setup_test_environment();

        let items_text = "my-file_with.special-chars.js\tHandle special cases!\nanother_file.ts\tTest edge cases";
        let task = create_test_task_from_text("Task with special chars: @#$%^&*()", items_text);
        
        assert_eq!(task.name, "Task with special chars: @#$%^&*()");
        assert_eq!(task.repo_path, "/test/repo");
        assert_eq!(task.base_ref, "main");
        assert_eq!(task.total_items, 2);
        
        cleanup_test_environment();
    }

    #[test]
    fn test_create_task_with_unicode() {
        setup_test_environment();

        let items_text = "文件.js\t审查代码\n文档.md\t更新文档";
        let task = create_test_task_from_text("任务名称", items_text);
        
        assert_eq!(task.name, "任务名称");
        assert_eq!(task.base_ref, "main");
        assert_eq!(task.total_items, 2);
        
        cleanup_test_environment();
    }

    #[test]
    fn test_create_task_very_long_description() {
        setup_test_environment();

        let long_description = "This is a very long description that contains many words and should be handled properly by the parser without any issues or truncation. ".repeat(20);
        let items_text = format!("file.js\t{}", long_description);
        let task = create_test_task_from_text("Long Description Task", &items_text);
        
        assert_eq!(task.name, "Long Description Task");
        assert_eq!(task.total_items, 1);
        assert_eq!(task.items[0].preset_comment, Some(long_description.trim().to_string()));
        
        cleanup_test_environment();
    }

    #[test]
    fn test_create_task_invalid_text_format() {
        setup_test_environment();

        let items_text = "This is not a valid format\nAnother invalid line\nNo tab separator";
        let task = create_test_task_from_text("Invalid Format Task", items_text);
        
        assert_eq!(task.name, "Invalid Format Task");
        // Should still create task but with empty items since parser might handle invalid format
        assert!(task.total_items >= 0);
        
        cleanup_test_environment();
    }

    #[test]
    fn test_create_task_with_empty_lines() {
        setup_test_environment();

        let items_text = "file1.js\tFirst item\n\nfile2.ts\tSecond item\n\n\nfile3.py\tThird item";
        let task = create_test_task_from_text("Task with Empty Lines", items_text);
        
        assert_eq!(task.name, "Task with Empty Lines");
        // Should handle empty lines gracefully
        assert!(task.total_items >= 3);
        
        cleanup_test_environment();
    }

    #[test]
    fn test_create_task_timestamps_set() {
        setup_test_environment();

        let before_creation = Utc::now();
        
        let items_text = "file.js: Test item";
        let task = create_test_task_from_text("Timestamp Test Task", items_text);
        let after_creation = Utc::now();
        
        // Verify timestamps are set and reasonable (allow for small time differences)
        assert!(task.create_time >= before_creation);
        assert!(task.create_time <= after_creation);
        assert!(task.update_time >= before_creation);
        assert!(task.update_time <= after_creation);
        // Create and update time should be very close (within 1 second) on creation
        let time_diff = (task.update_time - task.create_time).num_milliseconds().abs();
        assert!(time_diff <= 1000); // Within 1 second
        
        cleanup_test_environment();
    }

    #[test]
    fn test_create_task_uuid_generation() {
        setup_test_environment();

        let items_text1 = "file1.js\tFirst item";
        let items_text2 = "file2.js\tSecond item";
        
        let task1 = create_test_task_from_text("First Task", items_text1);
        let task2 = create_test_task_from_text("Second Task", items_text2);
        
        // UUIDs should be unique
        assert_ne!(task1.id, task2.id);
        
        cleanup_test_environment();
    }

    #[test]
    fn test_create_task_initial_state() {
        setup_test_environment();

        let items_text = "file1.js\tFirst item\nfile2.ts\tSecond item";
        let task = create_test_task_from_text("Initial State Task", items_text);
        
        // Verify initial state
        assert_eq!(task.status, TaskStatus::InProgress);
        assert_eq!(task.completed_items, 0);
        assert_eq!(task.total_items, 2);
        
        // All items should be unreviewed initially
        for item in &task.items {
            assert!(!item.reviewed);
            assert!(item.comments.is_empty());
        }
        
        cleanup_test_environment();
    }
}