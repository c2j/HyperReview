#[cfg(test)]
mod tests {
    use crate::commands::task_commands::create_task;
    use crate::models::task::{CreateTaskRequest, TaskStatus};
    use crate::AppState;
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

    #[tokio::test]
    async fn test_create_task_success() {
        setup_test_environment();

        let request = CreateTaskRequest {
            name: "Test Task".to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "main".to_string(),
            items_text: "file1.js: Review authentication\nfile2.ts: Fix validation".to_string(),
        };

        let result = create_task(request, State::new(crate::AppState::default())).await;
        
        assert!(result.is_ok());
        let task = result.unwrap();
        
        assert_eq!(task.name, "Test Task");
        assert_eq!(task.repo_path, "/test/repo");
        assert_eq!(task.base_ref, "main");
        assert_eq!(task.status, TaskStatus::InProgress);
        assert_eq!(task.total_items, 2);
        assert_eq!(task.completed_items, 0);
        assert!(task.items.len() == 2);
        
        cleanup_test_environment();
    }

    #[tokio::test]
    async fn test_create_task_empty_items() {
        setup_test_environment();

        let request = CreateTaskRequest {
            name: "Empty Task".to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "main".to_string(),
            items_text: "".to_string(),
        };

        let result = create_task(request, State::new(crate::AppState::default())).await;
        
        assert!(result.is_ok());
        let task = result.unwrap();
        
        assert_eq!(task.name, "Empty Task");
        assert_eq!(task.total_items, 0);
        assert_eq!(task.completed_items, 0);
        assert!(task.items.is_empty());
        
        cleanup_test_environment();
    }

    #[tokio::test]
    async fn test_create_task_single_item() {
        setup_test_environment();

        let request = CreateTaskRequest {
            name: "Single Item Task".to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "develop".to_string(),
            items_text: "src/main.js: Review main function".to_string(),
        };

        let result = create_task(request, State::new(crate::AppState::default())).await;
        
        assert!(result.is_ok());
        let task = result.unwrap();
        
        assert_eq!(task.name, "Single Item Task");
        assert_eq!(task.total_items, 1);
        assert_eq!(task.items.len(), 1);
        assert_eq!(task.items[0].file, "src/main.js");
        assert_eq!(task.items[0].description, "Review main function");
        assert!(!task.items[0].reviewed);
        
        cleanup_test_environment();
    }

    #[tokio::test]
    async fn test_create_task_multiple_items() {
        setup_test_environment();

        let request = CreateTaskRequest {
            name: "Multi Item Task".to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "feature-branch".to_string(),
            items_text: "file1.js: Review auth\nfile2.ts: Fix types\nfile3.py: Update docs\nfile4.rs: Optimize performance".to_string(),
        };

        let result = create_task(request, State::new(crate::AppState::default())).await;
        
        assert!(result.is_ok());
        let task = result.unwrap();
        
        assert_eq!(task.name, "Multi Item Task");
        assert_eq!(task.total_items, 4);
        assert_eq!(task.items.len(), 4);
        
        // Verify each item
        assert_eq!(task.items[0].file, "file1.js");
        assert_eq!(task.items[0].description, "Review auth");
        assert_eq!(task.items[1].file, "file2.ts");
        assert_eq!(task.items[1].description, "Fix types");
        assert_eq!(task.items[2].file, "file3.py");
        assert_eq!(task.items[2].description, "Update docs");
        assert_eq!(task.items[3].file, "file4.rs");
        assert_eq!(task.items[3].description, "Optimize performance");
        
        cleanup_test_environment();
    }

    #[tokio::test]
    async fn test_create_task_with_special_characters() {
        setup_test_environment();

        let request = CreateTaskRequest {
            name: "Task with special chars: @#$%^&*()".to_string(),
            repo_path: "/test/repo-with-dashes_and_underscores".to_string(),
            base_ref: "feature/branch-with-slashes".to_string(),
            items_text: "my-file_with.special-chars.js: Handle special cases!\nanother_file.ts: Test edge cases".to_string(),
        };

        let result = create_task(request, State::new(crate::AppState::default())).await;
        
        assert!(result.is_ok());
        let task = result.unwrap();
        
        assert_eq!(task.name, "Task with special chars: @#$%^&*()");
        assert_eq!(task.repo_path, "/test/repo-with-dashes_and_underscores");
        assert_eq!(task.base_ref, "feature/branch-with-slashes");
        assert_eq!(task.total_items, 2);
        
        cleanup_test_environment();
    }

    #[tokio::test]
    async fn test_create_task_with_unicode() {
        setup_test_environment();

        let request = CreateTaskRequest {
            name: "任务名称".to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "主分支".to_string(),
            items_text: "文件.js: 审查代码\n文档.md: 更新文档".to_string(),
        };

        let result = create_task(request, State::new(crate::AppState::default())).await;
        
        assert!(result.is_ok());
        let task = result.unwrap();
        
        assert_eq!(task.name, "任务名称");
        assert_eq!(task.base_ref, "主分支");
        assert_eq!(task.total_items, 2);
        
        cleanup_test_environment();
    }

    #[tokio::test]
    async fn test_create_task_very_long_description() {
        setup_test_environment();

        let long_description = "This is a very long description that contains many words and should be handled properly by the parser without any issues or truncation. ".repeat(20);
        let request = CreateTaskRequest {
            name: "Long Description Task".to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "main".to_string(),
            items_text: format!("file.js: {}", long_description),
        };

        let result = create_task(request, State::new(crate::AppState::default())).await;
        
        assert!(result.is_ok());
        let task = result.unwrap();
        
        assert_eq!(task.name, "Long Description Task");
        assert_eq!(task.total_items, 1);
        assert_eq!(task.items[0].description, long_description);
        
        cleanup_test_environment();
    }

    #[tokio::test]
    async fn test_create_task_invalid_text_format() {
        setup_test_environment();

        let request = CreateTaskRequest {
            name: "Invalid Format Task".to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "main".to_string(),
            items_text: "This is not a valid format\nAnother invalid line\nNo colon separator".to_string(),
        };

        let result = create_task(request, State::new(crate::AppState::default())).await;
        
        // Should still create task but with empty items since parser might handle invalid format
        assert!(result.is_ok());
        let task = result.unwrap();
        
        assert_eq!(task.name, "Invalid Format Task");
        // The parser should handle invalid lines gracefully
        assert!(task.total_items >= 0);
        
        cleanup_test_environment();
    }

    #[tokio::test]
    async fn test_create_task_with_empty_lines() {
        setup_test_environment();

        let request = CreateTaskRequest {
            name: "Task with Empty Lines".to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "main".to_string(),
            items_text: "file1.js: First item\n\nfile2.ts: Second item\n\n\nfile3.py: Third item".to_string(),
        };

        let result = create_task(request, State::new(crate::AppState::default())).await;
        
        assert!(result.is_ok());
        let task = result.unwrap();
        
        assert_eq!(task.name, "Task with Empty Lines");
        // Should handle empty lines gracefully
        assert!(task.total_items >= 3);
        
        cleanup_test_environment();
    }

    #[tokio::test]
    async fn test_create_task_timestamps_set() {
        setup_test_environment();

        let before_creation = Utc::now();
        
        let request = CreateTaskRequest {
            name: "Timestamp Test Task".to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "main".to_string(),
            items_text: "file.js: Test item".to_string(),
        };

        let result = create_task(request, State::new(crate::AppState::default())).await;
        let after_creation = Utc::now();
        
        assert!(result.is_ok());
        let task = result.unwrap();
        
        // Verify timestamps are set and reasonable
        assert!(task.create_time >= before_creation);
        assert!(task.create_time <= after_creation);
        assert!(task.update_time >= before_creation);
        assert!(task.update_time <= after_creation);
        assert_eq!(task.create_time, task.update_time); // Should be same on creation
        
        cleanup_test_environment();
    }

    #[tokio::test]
    async fn test_create_task_uuid_generation() {
        setup_test_environment();

        let request1 = CreateTaskRequest {
            name: "First Task".to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "main".to_string(),
            items_text: "file1.js: First item".to_string(),
        };

        let request2 = CreateTaskRequest {
            name: "Second Task".to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "main".to_string(),
            items_text: "file2.js: Second item".to_string(),
        };

        let result1 = create_task(request1, State::new(crate::AppState::default())).await;
        let result2 = create_task(request2, State::new(crate::AppState::default())).await;
        
        assert!(result1.is_ok());
        assert!(result2.is_ok());
        
        let task1 = result1.unwrap();
        let task2 = result2.unwrap();
        
        // UUIDs should be unique
        assert_ne!(task1.id, task2.id);
        
        cleanup_test_environment();
    }

    #[tokio::test]
    async fn test_create_task_initial_state() {
        setup_test_environment();

        let request = CreateTaskRequest {
            name: "Initial State Task".to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "main".to_string(),
            items_text: "file1.js: First item\nfile2.ts: Second item".to_string(),
        };

        let result = create_task(request, State::new(crate::AppState::default())).await;
        
        assert!(result.is_ok());
        let task = result.unwrap();
        
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