#[cfg(test)]
mod tests {
    use crate::models::task::{LocalTask, TaskStatus, TaskItem, Comment, LineRange};
    use uuid::Uuid;
    use chrono::Utc;
    use serde_json::Value;
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

    fn create_test_task(name: &str, status: TaskStatus, completed_items: u32, total_items: u32) -> LocalTask {
        let items: Vec<TaskItem> = (0..total_items).map(|i| TaskItem {
            file: format!("file{}.js", i + 1),
            line_range: None,
            preset_comment: Some(format!("Review file {}", i + 1)),
            severity: None,
            tags: Vec::new(),
            reviewed: i < completed_items,
            comments: if i == 0 {
                vec![Comment {
                    id: Uuid::new_v4(),
                    content: "Test comment".to_string(),
                    author: "tester".to_string(),
                    created_at: Utc::now(),
                    line_number: Some(10),
                }]
            } else {
                Vec::new()
            },
        }).collect();

        LocalTask {
            id: Uuid::new_v4(),
            name: name.to_string(),
            repo_path: format!("/test/{}", name.to_lowercase().replace(" ", "_")),
            base_ref: "main".to_string(),
            create_time: Utc::now(),
            update_time: Utc::now(),
            status,
            total_items,
            completed_items,
            items,
        }
    }

    #[test]
    fn test_export_single_task_json() {
        setup_test_environment();
        let task = create_test_task("Test Task", TaskStatus::InProgress, 1, 3);
        let task_id = task.id;

        // Simulate export by serializing to JSON
        let json_str = serde_json::to_string_pretty(&task).unwrap();
        
        // Parse the JSON to verify structure
        let parsed: Value = serde_json::from_str(&json_str).unwrap();
        
        assert_eq!(parsed["id"], task_id.to_string());
        assert_eq!(parsed["name"], "Test Task");
        assert_eq!(parsed["repo_path"], "/test/test_task");
        assert_eq!(parsed["base_ref"], "main");
        assert_eq!(parsed["status"], "in_progress");
        assert_eq!(parsed["total_items"], 3);
        assert_eq!(parsed["completed_items"], 1);
        assert_eq!(parsed["items"].as_array().unwrap().len(), 3);
        
        // Verify first item has comment
        assert_eq!(parsed["items"][0]["comments"].as_array().unwrap().len(), 1);
        assert_eq!(parsed["items"][0]["comments"][0]["content"], "Test comment");
        assert_eq!(parsed["items"][0]["comments"][0]["author"], "tester");
        
        cleanup_test_environment();
    }

    #[test]
    fn test_export_completed_task_json() {
        setup_test_environment();
        let task = create_test_task("Completed Task", TaskStatus::Completed, 5, 5);

        let json_str = serde_json::to_string_pretty(&task).unwrap();
        let parsed: Value = serde_json::from_str(&json_str).unwrap();
        
        assert_eq!(parsed["name"], "Completed Task");
        assert_eq!(parsed["status"], "completed");
        assert_eq!(parsed["total_items"], 5);
        assert_eq!(parsed["completed_items"], 5);
        
        // All items should be reviewed
        for item in parsed["items"].as_array().unwrap() {
            assert_eq!(item["reviewed"], true);
        }
        
        cleanup_test_environment();
    }

    #[test]
    fn test_export_archived_task_json() {
        setup_test_environment();
        let task = create_test_task("Archived Task", TaskStatus::Archived, 2, 4);

        let json_str = serde_json::to_string_pretty(&task).unwrap();
        let parsed: Value = serde_json::from_str(&json_str).unwrap();
        
        assert_eq!(parsed["name"], "Archived Task");
        assert_eq!(parsed["status"], "archived");
        assert_eq!(parsed["total_items"], 4);
        assert_eq!(parsed["completed_items"], 2);
        
        cleanup_test_environment();
    }

    #[test]
    fn test_export_task_with_unicode_json() {
        setup_test_environment();
        
        let task = LocalTask {
            id: Uuid::new_v4(),
            name: "任务名称".to_string(),
            repo_path: "/测试/仓库".to_string(),
            base_ref: "主分支".to_string(),
            create_time: Utc::now(),
            update_time: Utc::now(),
            status: TaskStatus::InProgress,
            total_items: 1,
            completed_items: 0,
            items: vec![TaskItem {
                file: "文件.js".to_string(),
                line_range: None,
                preset_comment: Some("审查代码".to_string()),
                severity: None,
                tags: Vec::new(),
                reviewed: false,
                comments: vec![Comment {
                    id: Uuid::new_v4(),
                    content: "测试评论".to_string(),
                    author: "测试者".to_string(),
                    created_at: Utc::now(),
                    line_number: Some(5),
                }],
            }],
        };

        let json_str = serde_json::to_string_pretty(&task).unwrap();
        let parsed: Value = serde_json::from_str(&json_str).unwrap();
        
        assert_eq!(parsed["name"], "任务名称");
        assert_eq!(parsed["repo_path"], "/测试/仓库");
        assert_eq!(parsed["base_ref"], "主分支");
        assert_eq!(parsed["items"][0]["file"], "文件.js");
        assert_eq!(parsed["items"][0]["preset_comment"], "审查代码");
        assert_eq!(parsed["items"][0]["comments"][0]["content"], "测试评论");
        assert_eq!(parsed["items"][0]["comments"][0]["author"], "测试者");
        
        cleanup_test_environment();
    }

    #[test]
    fn test_export_task_with_special_characters_json() {
        setup_test_environment();
        
        let task = LocalTask {
            id: Uuid::new_v4(),
            name: "Task with @#$%^&*() special chars".to_string(),
            repo_path: "/test/repo-with_special.chars".to_string(),
            base_ref: "feature/branch-with_slashes-and-dashes".to_string(),
            create_time: Utc::now(),
            update_time: Utc::now(),
            status: TaskStatus::InProgress,
            total_items: 1,
            completed_items: 0,
            items: vec![TaskItem {
                file: "my-file_with.special-chars.js".to_string(),
                line_range: None,
                preset_comment: Some("Handle special cases: @#$%^&*()".to_string()),
                severity: None,
                tags: Vec::new(),
                reviewed: false,
                comments: vec![],
            }],
        };

        let json_str = serde_json::to_string_pretty(&task).unwrap();
        let parsed: Value = serde_json::from_str(&json_str).unwrap();
        
        assert_eq!(parsed["name"], "Task with @#$%^&*() special chars");
        assert_eq!(parsed["repo_path"], "/test/repo-with_special.chars");
        assert_eq!(parsed["base_ref"], "feature/branch-with_slashes-and-dashes");
        assert_eq!(parsed["items"][0]["file"], "my-file_with.special-chars.js");
        assert_eq!(parsed["items"][0]["preset_comment"], "Handle special cases: @#$%^&*()");
        
        cleanup_test_environment();
    }

    #[test]
    fn test_export_task_json_format() {
        setup_test_environment();
        let task = create_test_task("Format Test Task", TaskStatus::InProgress, 1, 2);

        let json_str = serde_json::to_string_pretty(&task).unwrap();
        
        // Verify JSON is properly formatted (pretty printed)
        assert!(json_str.contains("\n")); // Should have newlines for pretty printing
        assert!(json_str.contains("  ")); // Should have indentation
        
        // Verify it's valid JSON
        let parsed: Value = serde_json::from_str(&json_str).unwrap();
        assert!(parsed.is_object());
        
        cleanup_test_environment();
    }

    #[test]
    fn test_export_task_with_empty_items_json() {
        setup_test_environment();
        
        let task = LocalTask {
            id: Uuid::new_v4(),
            name: "Empty Items Task".to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "main".to_string(),
            create_time: Utc::now(),
            update_time: Utc::now(),
            status: TaskStatus::InProgress,
            total_items: 0,
            completed_items: 0,
            items: vec![],
        };

        let json_str = serde_json::to_string_pretty(&task).unwrap();
        let parsed: Value = serde_json::from_str(&json_str).unwrap();
        
        assert_eq!(parsed["total_items"], 0);
        assert_eq!(parsed["completed_items"], 0);
        assert!(parsed["items"].as_array().unwrap().is_empty());
        
        cleanup_test_environment();
    }

    #[test]
    fn test_export_task_preserves_timestamps_json() {
        setup_test_environment();
        
        let create_time = Utc::now();
        let update_time = create_time + chrono::Duration::hours(1);
        
        let task = LocalTask {
            id: Uuid::new_v4(),
            name: "Timestamp Test Task".to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "main".to_string(),
            create_time,
            update_time,
            status: TaskStatus::InProgress,
            total_items: 1,
            completed_items: 0,
            items: vec![TaskItem {
                file: "file.js".to_string(),
                line_range: None,
                preset_comment: Some("Test item".to_string()),
                severity: None,
                tags: Vec::new(),
                reviewed: false,
                comments: vec![],
            }],
        };

        let json_str = serde_json::to_string_pretty(&task).unwrap();
        let parsed: Value = serde_json::from_str(&json_str).unwrap();
        
        // Verify timestamps are preserved in ISO format
        assert!(parsed["create_time"].as_str().unwrap().contains("T"));
        assert!(parsed["update_time"].as_str().unwrap().contains("T"));
        
        cleanup_test_environment();
    }

    #[test]
    fn test_export_all_tasks_json() {
        setup_test_environment();
        
        let task1 = create_test_task("Task 1", TaskStatus::InProgress, 2, 4);
        let task2 = create_test_task("Task 2", TaskStatus::Completed, 3, 3);
        let task3 = create_test_task("Task 3", TaskStatus::Archived, 1, 5);

        // Simulate exporting all tasks
        let all_tasks = vec![task1, task2, task3];
        let json_str = serde_json::to_string_pretty(&all_tasks).unwrap();
        
        let parsed: Value = serde_json::from_str(&json_str).unwrap();
        
        assert!(parsed.is_array());
        assert_eq!(parsed.as_array().unwrap().len(), 3);
        
        // Verify all tasks are present
        let task_names: Vec<String> = parsed.as_array().unwrap()
            .iter()
            .map(|task| task["name"].as_str().unwrap().to_string())
            .collect();
        
        assert!(task_names.contains(&"Task 1".to_string()));
        assert!(task_names.contains(&"Task 2".to_string()));
        assert!(task_names.contains(&"Task 3".to_string()));
        
        // Verify individual task data
        let task1_data = parsed.as_array().unwrap().iter()
            .find(|task| task["name"] == "Task 1")
            .unwrap();
        assert_eq!(task1_data["status"], "in_progress");
        assert_eq!(task1_data["total_items"], 4);
        assert_eq!(task1_data["completed_items"], 2);
        
        cleanup_test_environment();
    }
}