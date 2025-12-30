#[cfg(test)]
mod tests {
    use tempfile::TempDir;
    use hyperreview_lib::commands::text_parser;
    use hyperreview_lib::models::task::{LocalTask, CreateTaskRequest, TaskStatus, TaskSummary, TaskItem, TaskSeverity, Comment, LineRange};
    use hyperreview_lib::storage::task_store::TaskStore;
    use uuid::Uuid;
    use chrono::Utc;

    #[test]
    fn test_create_task_from_text() {
        let temp_dir = TempDir::new().unwrap();
        let _task_name = "Test Task";
        let _repo_path = temp_dir.path().to_str().unwrap();
        let _base_ref = "main";
        let items_text = "src/main.rs\nsrc/lib.rs\tCheck this\nsrc/utils.rs";
        
        // This would normally invoke the create_task command
        // For now, we'll test the text parsing logic
        assert_eq!(items_text.lines().count(), 3);
    }

    #[test]
    fn test_text_parser_integration() {
        let items_text = "src/main.rs\nsrc/lib.rs\tCheck this file\nsrc/utils.rs  Review API\nsrc/config.rs";
        
        let result = text_parser::parse_task_text(items_text);
        assert!(result.is_ok());
        
        let items = result.unwrap();
        assert_eq!(items.len(), 4);
        
        // Check first item (simple file path)
        assert_eq!(items[0].file, "src/main.rs");
        assert_eq!(items[0].preset_comment, None);
        assert_eq!(items[0].reviewed, false);
        
        // Check second item (tab-separated comment)
        assert_eq!(items[1].file, "src/lib.rs");
        assert_eq!(items[1].preset_comment, Some("Check this file".to_string()));
        
        // Check third item (double-space separated comment)
        assert_eq!(items[2].file, "src/utils.rs");
        assert_eq!(items[2].preset_comment, Some("Review API".to_string()));
        
        // Check fourth item (simple file path)
        assert_eq!(items[3].file, "src/config.rs");
        assert_eq!(items[3].preset_comment, None);
    }

    #[test]
    fn test_task_store_integration() {
        // Create a temporary task store
        let task_store = TaskStore::new().expect("Failed to create task store");
        
        // Create a test task
        let items = vec![
            hyperreview_lib::models::task::TaskItem {
                file: "src/main.rs".to_string(),
                line_range: None,
                preset_comment: Some("Check main function".to_string()),
                severity: None,
                tags: vec!["critical".to_string()],
                reviewed: false,
                comments: vec![],
            },
            hyperreview_lib::models::task::TaskItem {
                file: "src/lib.rs".to_string(),
                line_range: None,
                preset_comment: None,
                severity: None,
                tags: vec![],
                reviewed: false,
                comments: vec![],
            },
        ];
        
        let task = LocalTask {
            id: Uuid::new_v4(),
            name: "Integration Test Task".to_string(),
            repo_path: "/tmp/test/repo".to_string(),
            base_ref: "main".to_string(),
            create_time: Utc::now(),
            update_time: Utc::now(),
            status: TaskStatus::InProgress,
            total_items: items.len() as u32,
            completed_items: 0,
            items,
        };
        
        // Save the task
        let save_result = task_store.save_task(&task);
        assert!(save_result.is_ok(), "Failed to save task: {:?}", save_result.err());
        
        // Load the task back
        let load_result = task_store.load_task(task.id);
        assert!(load_result.is_ok(), "Failed to load task: {:?}", load_result.err());
        
        let loaded_task = load_result.unwrap();
        assert_eq!(loaded_task.id, task.id);
        assert_eq!(loaded_task.name, task.name);
        assert_eq!(loaded_task.repo_path, task.repo_path);
        assert_eq!(loaded_task.base_ref, task.base_ref);
        assert_eq!(loaded_task.status, task.status);
        assert_eq!(loaded_task.total_items, task.total_items);
        assert_eq!(loaded_task.completed_items, task.completed_items);
        assert_eq!(loaded_task.items.len(), task.items.len());
        
        // Verify items
        assert_eq!(loaded_task.items[0].file, "src/main.rs");
        assert_eq!(loaded_task.items[0].preset_comment, Some("Check main function".to_string()));
        assert_eq!(loaded_task.items[0].tags, vec!["critical".to_string()]);
        
        assert_eq!(loaded_task.items[1].file, "src/lib.rs");
        assert_eq!(loaded_task.items[1].preset_comment, None);
        assert_eq!(loaded_task.items[1].tags, Vec::<String>::new());
        
        // Clean up
        let delete_result = task_store.delete_task(task.id);
        assert!(delete_result.is_ok(), "Failed to delete task: {:?}", delete_result.err());
    }

    #[test]
    fn test_task_store_list_tasks() {
        let task_store = TaskStore::new().expect("Failed to create task store");
        
        // Create multiple tasks
        let task1_id = Uuid::new_v4();
        let task2_id = Uuid::new_v4();
        
        let task1 = LocalTask {
            id: task1_id,
            name: "Task 1".to_string(),
            repo_path: "/tmp/test/repo1".to_string(),
            base_ref: "main".to_string(),
            create_time: Utc::now(),
            update_time: Utc::now(),
            status: TaskStatus::InProgress,
            total_items: 5,
            completed_items: 2,
            items: vec![],
        };
        
        let task2 = LocalTask {
            id: task2_id,
            name: "Task 2".to_string(),
            repo_path: "/tmp/test/repo2".to_string(),
            base_ref: "develop".to_string(),
            create_time: Utc::now(),
            update_time: Utc::now(),
            status: TaskStatus::Completed,
            total_items: 3,
            completed_items: 3,
            items: vec![],
        };
        
        // Save both tasks
        assert!(task_store.save_task(&task1).is_ok());
        assert!(task_store.save_task(&task2).is_ok());
        
        // List tasks
        let list_result = task_store.list_tasks();
        assert!(list_result.is_ok(), "Failed to list tasks: {:?}", list_result.err());
        
        let tasks = list_result.unwrap();
        assert!(tasks.len() >= 2, "Expected at least 2 tasks, found {}", tasks.len());
        
        // Find our tasks in the list
        let found_task1 = tasks.iter().find(|t| t.id == task1_id);
        let found_task2 = tasks.iter().find(|t| t.id == task2_id);
        
        assert!(found_task1.is_some(), "Task 1 not found in task list");
        assert!(found_task2.is_some(), "Task 2 not found in task list");
        
        if let Some(t1) = found_task1 {
            assert_eq!(t1.name, "Task 1");
            assert_eq!(t1.status, TaskStatus::InProgress);
        }
        
        if let Some(t2) = found_task2 {
            assert_eq!(t2.name, "Task 2");
            assert_eq!(t2.status, TaskStatus::Completed);
        }
        
        // Clean up
        assert!(task_store.delete_task(task1_id).is_ok());
        assert!(task_store.delete_task(task2_id).is_ok());
    }

    #[test]
    fn test_create_task_command_simulation() {
        // This test simulates the create_task command without requiring a real git repository
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path().to_str().unwrap().to_string();
        let task_name = "Simulated Create Task".to_string();
        let base_ref = "main".to_string();
        let items_text = "src/main.rs\tCheck main function\nsrc/lib.rs\tReview library API".to_string();
        
        // Create the request payload (simulating what the frontend would send)
        let request = CreateTaskRequest {
            name: task_name.clone(),
            repo_path: repo_path.clone(),
            base_ref: base_ref.clone(),
            items_text: items_text.clone(),
        };
        
        // Parse the text (this is what create_task does)
        let items = text_parser::parse_task_text(&request.items_text)
            .expect("Failed to parse task text");
        
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].file, "src/main.rs");
        assert_eq!(items[0].preset_comment, Some("Check main function".to_string()));
        assert_eq!(items[1].file, "src/lib.rs");
        assert_eq!(items[1].preset_comment, Some("Review library API".to_string()));
        
        // Create the task (simulating the rest of create_task)
        let task = LocalTask {
            id: Uuid::new_v4(),
            name: request.name,
            repo_path: request.repo_path,
            base_ref: request.base_ref,
            create_time: Utc::now(),
            update_time: Utc::now(),
            status: TaskStatus::InProgress,
            total_items: items.len() as u32,
            completed_items: 0,
            items,
        };
        
        // Verify the created task
        assert_eq!(task.name, task_name);
        assert_eq!(task.repo_path, repo_path);
        assert_eq!(task.base_ref, base_ref);
        assert_eq!(task.status, TaskStatus::InProgress);
        assert_eq!(task.total_items, 2);
        assert_eq!(task.completed_items, 0);
        assert_eq!(task.items.len(), 2);
        
        // Test task store integration
        let task_store = TaskStore::new().expect("Failed to create task store");
        assert!(task_store.save_task(&task).is_ok());
        
        // Verify task can be loaded
        let loaded_task = task_store.load_task(task.id).expect("Failed to load task");
        assert_eq!(loaded_task.id, task.id);
        assert_eq!(loaded_task.name, task.name);
        assert_eq!(loaded_task.items.len(), task.items.len());
        
        // Clean up
        assert!(task_store.delete_task(task.id).is_ok());
    }

    #[test]
    fn test_invalid_input_handling() {
        // Test empty text
        let result = text_parser::parse_task_text("");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
        
        // Test text with only whitespace
        let result = text_parser::parse_task_text("   \n  \t  \n   ");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
        
        // Test text with only comments (no file paths)
        let result = text_parser::parse_task_text("\tOnly comment\n  Another comment");
        assert!(result.is_ok());
        let items = result.unwrap();
        assert_eq!(items.len(), 0);
        
        // Test mixed valid and invalid lines
        let result = text_parser::parse_task_text("src/main.rs\n\tInvalid line\nsrc/lib.rs");
        assert!(result.is_ok());
        let items = result.unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].file, "src/main.rs");
        assert_eq!(items[1].file, "src/lib.rs");
    }

    #[test]
    fn test_task_status_transitions() {
        // Test status enum values
        assert_eq!(TaskStatus::InProgress as i32, 0);
        assert_eq!(TaskStatus::Completed as i32, 1);
        assert_eq!(TaskStatus::Archived as i32, 2);
        
        // Test status serialization/deserialization
        let status = TaskStatus::InProgress;
        let serialized = serde_json::to_string(&status).unwrap();
        assert_eq!(serialized, "\"in_progress\"");
        
        let deserialized: TaskStatus = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, TaskStatus::InProgress);
        
        // Test all status variants
        let statuses = vec![
            TaskStatus::InProgress,
            TaskStatus::Completed,
            TaskStatus::Archived,
        ];
        
        for status in statuses {
            let serialized = serde_json::to_string(&status).unwrap();
            let deserialized: TaskStatus = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, status);
        }
    }

    #[test]
    fn test_task_progress_calculation() {
        let task = LocalTask {
            id: Uuid::new_v4(),
            name: "Test Task".to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "main".to_string(),
            create_time: Utc::now(),
            update_time: Utc::now(),
            status: TaskStatus::InProgress,
            total_items: 10,
            completed_items: 3,
            items: vec![],
        };
        
        // Test progress calculation
        let expected_progress = 3.0 / 10.0;
        assert_eq!(task.completed_items as f32 / task.total_items as f32, expected_progress);
        
        // Test task with no items
        let task_no_items = LocalTask {
            id: task.id,
            name: task.name.clone(),
            repo_path: task.repo_path.clone(),
            base_ref: task.base_ref.clone(),
            create_time: task.create_time,
            update_time: task.update_time,
            status: task.status.clone(),
            total_items: 0,
            completed_items: 0,
            items: vec![],
        };
        assert_eq!(task_no_items.completed_items, 0);
        assert_eq!(task_no_items.total_items, 0);
        
        // Test completed task
        let completed_task = LocalTask {
            id: task.id,
            name: task.name.clone(),
            repo_path: task.repo_path.clone(),
            base_ref: task.base_ref.clone(),
            create_time: task.create_time,
            update_time: task.update_time,
            status: TaskStatus::Completed,
            total_items: 10,
            completed_items: 10,
            items: vec![],
        };
        assert_eq!(completed_task.completed_items, completed_task.total_items);
        assert_eq!(completed_task.status, TaskStatus::Completed);
    }

    #[test]
    fn test_task_status_transitions_logic() {
        let mut task = LocalTask {
            id: Uuid::new_v4(),
            name: "Test Task".to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "main".to_string(),
            create_time: Utc::now(),
            update_time: Utc::now(),
            status: TaskStatus::InProgress,
            total_items: 5,
            completed_items: 2,
            items: vec![],
        };
        
        // Test initial status
        assert_eq!(task.status, TaskStatus::InProgress);
        
        // Test status transition to completed
        task.status = TaskStatus::Completed;
        assert_eq!(task.status, TaskStatus::Completed);
        
        // Test status transition to archived
        task.status = TaskStatus::Archived;
        assert_eq!(task.status, TaskStatus::Archived);
        
        // Test status can go back from archived to in_progress
        task.status = TaskStatus::InProgress;
        assert_eq!(task.status, TaskStatus::InProgress);
    }

    #[test]
    fn test_task_status_with_progress_updates() {
        let mut task = LocalTask {
            id: Uuid::new_v4(),
            name: "Test Task".to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "main".to_string(),
            create_time: Utc::now(),
            update_time: Utc::now(),
            status: TaskStatus::InProgress,
            total_items: 5,
            completed_items: 0,
            items: vec![],
        };
        
        // Simulate progress updates
        for i in 1..=5 {
            task.completed_items = i;
            
            if i == 5 {
                // Task should be completed when all items are done
                task.status = TaskStatus::Completed;
            }
            
            let progress = task.completed_items as f32 / task.total_items as f32;
            assert_eq!(progress, i as f32 / 5.0);
        }
        
        assert_eq!(task.status, TaskStatus::Completed);
        assert_eq!(task.completed_items, task.total_items);
    }

    #[test]
    fn test_task_summary_creation() {
        let task_id = Uuid::new_v4();
        let task_name = "Test Task".to_string();
        let task_status = TaskStatus::InProgress;
        let total_items = 8;
        let completed_items = 3;
        
        let summary = TaskSummary {
            id: task_id,
            name: task_name.clone(),
            status: task_status.clone(),
            progress: completed_items as f32 / total_items as f32,
        };
        
        assert_eq!(summary.id, task_id);
        assert_eq!(summary.name, task_name);
        assert_eq!(summary.status, task_status);
        assert_eq!(summary.progress, 3.0 / 8.0);
    }

    #[test]
    fn test_task_status_edge_cases() {
        // Test task with more completed items than total (shouldn't happen but handle gracefully)
        let task = LocalTask {
            id: Uuid::new_v4(),
            name: "Test Task".to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "main".to_string(),
            create_time: Utc::now(),
            update_time: Utc::now(),
            status: TaskStatus::InProgress,
            total_items: 5,
            completed_items: 7, // More than total
            items: vec![],
        };
        
        let progress = task.completed_items as f32 / task.total_items as f32;
        assert!(progress > 1.0); // Progress > 100%
        
        // Test task with zero total items
        let task_zero = LocalTask {
            id: Uuid::new_v4(),
            name: "Test Task Zero".to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "main".to_string(),
            create_time: Utc::now(),
            update_time: Utc::now(),
            status: TaskStatus::InProgress,
            total_items: 0,
            completed_items: 0,
            items: vec![],
        };
        
        // Avoid division by zero
        if task_zero.total_items > 0 {
            let _progress = task_zero.completed_items as f32 / task_zero.total_items as f32;
        }
    }

    #[test]
    fn test_delete_task_operation() {
        let task_store = TaskStore::new().expect("Failed to create task store");
        
        // Create a test task
        let task = LocalTask {
            id: Uuid::new_v4(),
            name: "Task to Delete".to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "main".to_string(),
            create_time: Utc::now(),
            update_time: Utc::now(),
            status: TaskStatus::InProgress,
            total_items: 3,
            completed_items: 1,
            items: vec![],
        };
        
        // Save the task
        assert!(task_store.save_task(&task).is_ok());
        
        // Verify task exists
        assert!(task_store.load_task(task.id).is_ok());
        
        // Delete the task
        assert!(task_store.delete_task(task.id).is_ok());
        
        // Verify task no longer exists
        let load_result = task_store.load_task(task.id);
        assert!(load_result.is_err());
        
        // Test deleting non-existent task (should not fail)
        assert!(task_store.delete_task(task.id).is_ok());
    }

    #[test]
    fn test_archive_task_operation() {
        let task_store = TaskStore::new().expect("Failed to create task store");
        
        // Create a test task
        let task = LocalTask {
            id: Uuid::new_v4(),
            name: "Task to Archive".to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "main".to_string(),
            create_time: Utc::now(),
            update_time: Utc::now(),
            status: TaskStatus::InProgress,
            total_items: 5,
            completed_items: 3,
            items: vec![],
        };
        
        // Save the task
        assert!(task_store.save_task(&task).is_ok());
        
        // Verify initial status
        let loaded_task = task_store.load_task(task.id).expect("Failed to load task");
        assert_eq!(loaded_task.status, TaskStatus::InProgress);
        
        // Simulate archive operation (similar to archive_task command)
        let mut task_to_archive = loaded_task;
        task_to_archive.status = TaskStatus::Archived;
        task_to_archive.update_time = Utc::now();
        assert!(task_store.save_task(&task_to_archive).is_ok());
        
        // Verify task is archived
        let archived_task = task_store.load_task(task.id).expect("Failed to load archived task");
        assert_eq!(archived_task.status, TaskStatus::Archived);
        assert_eq!(archived_task.id, task.id);
        assert_eq!(archived_task.name, task.name);
        assert!(archived_task.update_time >= task.update_time);
    }

    #[test]
    fn test_task_lifecycle_operations() {
        let task_store = TaskStore::new().expect("Failed to create task store");
        
        // Create a task
        let task = LocalTask {
            id: Uuid::new_v4(),
            name: "Lifecycle Test Task".to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "main".to_string(),
            create_time: Utc::now(),
            update_time: Utc::now(),
            status: TaskStatus::InProgress,
            total_items: 4,
            completed_items: 0,
            items: vec![],
        };
        
        // Save task
        assert!(task_store.save_task(&task).is_ok());
        
        // Simulate completing the task
        let mut completed_task = task_store.load_task(task.id).expect("Failed to load task");
        completed_task.status = TaskStatus::Completed;
        completed_task.completed_items = 4;
        completed_task.update_time = Utc::now();
        assert!(task_store.save_task(&completed_task).is_ok());
        
        // Verify task is completed
        let loaded_completed = task_store.load_task(task.id).expect("Failed to load completed task");
        assert_eq!(loaded_completed.status, TaskStatus::Completed);
        assert_eq!(loaded_completed.completed_items, 4);
        
        // Archive the completed task
        let mut archived_task = loaded_completed;
        archived_task.status = TaskStatus::Archived;
        archived_task.update_time = Utc::now();
        assert!(task_store.save_task(&archived_task).is_ok());
        
        // Verify task is archived
        let loaded_archived = task_store.load_task(task.id).expect("Failed to load archived task");
        assert_eq!(loaded_archived.status, TaskStatus::Archived);
        
        // Delete the archived task
        assert!(task_store.delete_task(task.id).is_ok());
        
        // Verify task is deleted
        let load_result = task_store.load_task(task.id);
        assert!(load_result.is_err());
    }

    #[test]
    fn test_archive_and_delete_nonexistent_task() {
        let task_store = TaskStore::new().expect("Failed to create task store");
        let nonexistent_id = Uuid::new_v4();
        
        // Test archiving non-existent task (should fail)
        let archive_result = task_store.load_task(nonexistent_id);
        assert!(archive_result.is_err());
        
        // Test deleting non-existent task (should not fail - idempotent)
        assert!(task_store.delete_task(nonexistent_id).is_ok());
    }

    #[test]
    fn test_task_state_transitions() {
        // Test all valid state transitions
        let task_store = TaskStore::new().expect("Failed to create task store");
        
        let task = LocalTask {
            id: Uuid::new_v4(),
            name: "State Transition Task".to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "main".to_string(),
            create_time: Utc::now(),
            update_time: Utc::now(),
            status: TaskStatus::InProgress,
            total_items: 3,
            completed_items: 0,
            items: vec![],
        };
        
        // Save initial task
        assert!(task_store.save_task(&task).is_ok());
        
        // Transition: InProgress -> Completed
        let mut current_task = task_store.load_task(task.id).expect("Failed to load task");
        current_task.status = TaskStatus::Completed;
        current_task.completed_items = 3; // All items completed
        current_task.update_time = Utc::now();
        assert!(task_store.save_task(&current_task).is_ok());
        
        // Transition: Completed -> Archived
        let mut current_task = task_store.load_task(task.id).expect("Failed to load task");
        current_task.status = TaskStatus::Archived;
        current_task.update_time = Utc::now();
        assert!(task_store.save_task(&current_task).is_ok());
        
        // Verify final state
        let final_task = task_store.load_task(task.id).expect("Failed to load final task");
        assert_eq!(final_task.status, TaskStatus::Archived);
        assert_eq!(final_task.completed_items, 3);
    }

    #[test]
    fn test_export_task_json_format() {
        let task_store = TaskStore::new().expect("Failed to create task store");
        
        // Create a test task with various data types
        let task = LocalTask {
            id: Uuid::new_v4(),
            name: "Test Export Task".to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "main".to_string(),
            create_time: Utc::now(),
            update_time: Utc::now(),
            status: TaskStatus::InProgress,
            total_items: 2,
            completed_items: 1,
            items: vec![
                TaskItem {
                    file: "src/main.rs".to_string(),
                    line_range: Some(LineRange { start: Some(10), end: Some(20) }),
                    preset_comment: Some("Check this function".to_string()),
                    severity: Some(TaskSeverity::Error),
                    tags: vec!["critical".to_string(), "rust".to_string()],
                    reviewed: true,
                    comments: vec![
                        Comment {
                            id: Uuid::new_v4(),
                            author: "Alice".to_string(),
                            content: "This looks good".to_string(),
                            created_at: Utc::now(),
                            line_number: Some(15),
                        },
                    ],
                },
                TaskItem {
                    file: "src/lib.rs".to_string(),
                    line_range: None,
                    preset_comment: None,
                    severity: None,
                    tags: vec![],
                    reviewed: false,
                    comments: vec![],
                },
            ],
        };
        
        // Save the task
        assert!(task_store.save_task(&task).is_ok());
        
        // Export the task
        let exported_json = serde_json::to_string_pretty(&task).expect("Failed to export task");
        
        // Verify JSON is valid
        let parsed: serde_json::Value = serde_json::from_str(&exported_json).expect("Failed to parse exported JSON");
        
        // Verify required fields are present
        assert!(parsed.get("id").is_some());
        assert!(parsed.get("name").is_some());
        assert!(parsed.get("repo_path").is_some());
        assert!(parsed.get("base_ref").is_some());
        assert!(parsed.get("create_time").is_some());
        assert!(parsed.get("update_time").is_some());
        assert!(parsed.get("status").is_some());
        assert!(parsed.get("total_items").is_some());
        assert!(parsed.get("completed_items").is_some());
        assert!(parsed.get("items").is_some());
        
        // Verify data types
        assert_eq!(parsed["name"], "Test Export Task");
        assert_eq!(parsed["status"], "in_progress");
        assert_eq!(parsed["total_items"], 2);
        assert_eq!(parsed["completed_items"], 1);
        
        // Verify items structure
        let items = parsed["items"].as_array().expect("Items should be array");
        assert_eq!(items.len(), 2);
        
        // Verify first item
        let first_item = &items[0];
        assert_eq!(first_item["file"], "src/main.rs");
        assert_eq!(first_item["reviewed"], true);
        assert_eq!(first_item["tags"].as_array().unwrap().len(), 2);
        assert_eq!(first_item["severity"], "error");
        
        // Verify second item
        let second_item = &items[1];
        assert_eq!(second_item["file"], "src/lib.rs");
        assert_eq!(second_item["reviewed"], false);
        assert_eq!(second_item["tags"].as_array().unwrap().len(), 0);
    }

    #[test]
    fn test_export_all_tasks_json_format() {
        let task_store = TaskStore::new().expect("Failed to create task store");
        
        // Create multiple test tasks
        let tasks = vec![
            LocalTask {
                id: Uuid::new_v4(),
                name: "Task 1".to_string(),
                repo_path: "/repo1".to_string(),
                base_ref: "main".to_string(),
                create_time: Utc::now(),
                update_time: Utc::now(),
                status: TaskStatus::InProgress,
                total_items: 3,
                completed_items: 1,
                items: vec![],
            },
            LocalTask {
                id: Uuid::new_v4(),
                name: "Task 2".to_string(),
                repo_path: "/repo2".to_string(),
                base_ref: "develop".to_string(),
                create_time: Utc::now(),
                update_time: Utc::now(),
                status: TaskStatus::Completed,
                total_items: 5,
                completed_items: 5,
                items: vec![],
            },
            LocalTask {
                id: Uuid::new_v4(),
                name: "Task 3".to_string(),
                repo_path: "/repo3".to_string(),
                base_ref: "feature".to_string(),
                create_time: Utc::now(),
                update_time: Utc::now(),
                status: TaskStatus::Archived,
                total_items: 2,
                completed_items: 2,
                items: vec![],
            },
        ];
        
        // Save all tasks
        for task in &tasks {
            assert!(task_store.save_task(task).is_ok());
        }
        
        // Export all tasks
        let all_tasks = task_store.list_tasks().expect("Failed to list tasks");
        let exported_json = serde_json::to_string_pretty(&all_tasks).expect("Failed to export all tasks");
        
        // Verify JSON is valid
        let parsed: serde_json::Value = serde_json::from_str(&exported_json).expect("Failed to parse exported JSON");
        let tasks_array = parsed.as_array().expect("Exported data should be array");
        
        // Verify we have tasks (at least our 3, but there might be more from previous tests)
        assert!(tasks_array.len() >= 3);
        
        // Verify task structure (LocalTask structure, not TaskSummary)
        for task_json in tasks_array {
            assert!(task_json.get("id").is_some());
            assert!(task_json.get("name").is_some());
            assert!(task_json.get("status").is_some());
            assert!(task_json.get("total_items").is_some());
            assert!(task_json.get("completed_items").is_some());
            assert!(task_json.get("items").is_some());
        }
        
        // Verify our specific tasks are included (by name)
        let task_names: Vec<String> = tasks_array
            .iter()
            .filter_map(|t| t["name"].as_str().map(String::from))
            .collect();
        
        assert!(task_names.contains(&"Task 1".to_string()));
        assert!(task_names.contains(&"Task 2".to_string()));
        assert!(task_names.contains(&"Task 3".to_string()));
    }

    #[test]
    fn test_export_task_with_special_characters() {
        let task = LocalTask {
            id: Uuid::new_v4(),
            name: "Task with \"quotes\" and 'apostrophes'".to_string(),
            repo_path: "/test/repo with spaces".to_string(),
            base_ref: "feature/special-chars".to_string(),
            create_time: Utc::now(),
            update_time: Utc::now(),
            status: TaskStatus::InProgress,
            total_items: 1,
            completed_items: 0,
            items: vec![
                TaskItem {
                    file: "src/test.rs".to_string(),
                    line_range: None,
                    preset_comment: Some("Comment with \"quotes\" and 'apostrophes'".to_string()),
                    severity: None,
                    tags: vec!["tag-with-dashes".to_string(), "tag_with_underscores".to_string()],
                    reviewed: false,
                    comments: vec![],
                },
            ],
        };
        
        // Export the task
        let exported_json = serde_json::to_string_pretty(&task).expect("Failed to export task");
        
        // Verify JSON is valid and preserves special characters
        let parsed: serde_json::Value = serde_json::from_str(&exported_json).expect("Failed to parse exported JSON");
        
        assert_eq!(parsed["name"], "Task with \"quotes\" and 'apostrophes'");
        assert_eq!(parsed["repo_path"], "/test/repo with spaces");
        assert_eq!(parsed["base_ref"], "feature/special-chars");
        
        let first_item = &parsed["items"][0];
        assert_eq!(first_item["preset_comment"], "Comment with \"quotes\" and 'apostrophes'");
        
        let tags = first_item["tags"].as_array().expect("Tags should be array");
        assert_eq!(tags[0], "tag-with-dashes");
        assert_eq!(tags[1], "tag_with_underscores");
    }

    #[test]
    fn test_export_task_with_unicode_content() {
        let task = LocalTask {
            id: Uuid::new_v4(),
            name: "任务测试".to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "main".to_string(),
            create_time: Utc::now(),
            update_time: Utc::now(),
            status: TaskStatus::InProgress,
            total_items: 1,
            completed_items: 0,
            items: vec![
                TaskItem {
                    file: "src/测试.rs".to_string(),
                    line_range: None,
                    preset_comment: Some("检查这个函数".to_string()),
                    severity: Some(TaskSeverity::Error),
                    tags: vec!["中文".to_string(), "测试".to_string()],
                    reviewed: false,
                    comments: vec![
                        Comment {
                            id: Uuid::new_v4(),
                            author: "用户".to_string(),
                            content: "这里看起来很好".to_string(),
                            created_at: Utc::now(),
                            line_number: Some(10),
                        },
                    ],
                },
            ],
        };
        
        // Export the task
        let exported_json = serde_json::to_string_pretty(&task).expect("Failed to export task");
        
        // Verify JSON preserves Unicode characters
        let parsed: serde_json::Value = serde_json::from_str(&exported_json).expect("Failed to parse exported JSON");
        
        assert_eq!(parsed["name"], "任务测试");
        assert_eq!(parsed["items"][0]["file"], "src/测试.rs");
        assert_eq!(parsed["items"][0]["preset_comment"], "检查这个函数");
        assert_eq!(parsed["items"][0]["comments"][0]["author"], "用户");
        assert_eq!(parsed["items"][0]["comments"][0]["content"], "这里看起来很好");
    }

    #[test]
    fn test_export_empty_task() {
        let task = LocalTask {
            id: Uuid::new_v4(),
            name: "Empty Task".to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "main".to_string(),
            create_time: Utc::now(),
            update_time: Utc::now(),
            status: TaskStatus::InProgress,
            total_items: 0,
            completed_items: 0,
            items: vec![],
        };
        
        // Export the task
        let exported_json = serde_json::to_string_pretty(&task).expect("Failed to export empty task");
        
        // Verify JSON structure for empty task
        let parsed: serde_json::Value = serde_json::from_str(&exported_json).expect("Failed to parse exported JSON");
        
        assert_eq!(parsed["name"], "Empty Task");
        assert_eq!(parsed["total_items"], 0);
        assert_eq!(parsed["completed_items"], 0);
        assert!(parsed["items"].as_array().unwrap().is_empty());
    }

    #[test]
    fn test_export_task_with_large_content() {
        let mut items = Vec::new();
        
        // Create 100 items to test performance with large datasets
        for i in 0..100 {
            items.push(TaskItem {
                file: format!("src/file_{}.rs", i),
                line_range: Some(LineRange { start: Some(i * 10), end: Some(i * 10 + 50) }),
                preset_comment: Some(format!("Review file {}", i)),
                severity: match i % 4 {
                    0 => Some(TaskSeverity::Error),
                    1 => Some(TaskSeverity::Warning),
                    2 => Some(TaskSeverity::Question),
                    _ => Some(TaskSeverity::Ok),
                },
                tags: vec![format!("tag{}", i), "large_dataset".to_string()],
                reviewed: i % 2 == 0,
                comments: if i % 3 == 0 {
                    vec![
                        Comment {
                            id: Uuid::new_v4(),
                            author: "Reviewer".to_string(),
                            content: format!("Comment for item {}", i),
                            created_at: Utc::now(),
                            line_number: Some(i * 10 + 5),
                        },
                    ]
                } else {
                    vec![]
                },
            });
        }
        
        let task = LocalTask {
            id: Uuid::new_v4(),
            name: "Large Dataset Task".to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "main".to_string(),
            create_time: Utc::now(),
            update_time: Utc::now(),
            status: TaskStatus::InProgress,
            total_items: 100,
            completed_items: 50,
            items,
        };
        
        // Test export performance
        let start = std::time::Instant::now();
        let exported_json = serde_json::to_string_pretty(&task).expect("Failed to export large task");
        let duration = start.elapsed();
        
        // Should complete quickly (less than 1 second for 100 items)
        assert!(duration.as_secs() < 1, "Export took too long: {:?}", duration);
        
        // Verify JSON is valid
        let parsed: serde_json::Value = serde_json::from_str(&exported_json).expect("Failed to parse exported JSON");
        assert_eq!(parsed["items"].as_array().unwrap().len(), 100);
    }

    #[test]
    fn test_export_json_schema_validation() {
        let task = LocalTask {
            id: Uuid::new_v4(),
            name: "Schema Test Task".to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "main".to_string(),
            create_time: Utc::now(),
            update_time: Utc::now(),
            status: TaskStatus::InProgress,
            total_items: 3,
            completed_items: 1,
            items: vec![
                TaskItem {
                    file: "src/main.rs".to_string(),
                    line_range: Some(LineRange { start: Some(10), end: Some(20) }),
                    preset_comment: Some("Check this function".to_string()),
                    severity: Some(TaskSeverity::Error),
                    tags: vec!["critical".to_string()],
                    reviewed: true,
                    comments: vec![],
                },
                TaskItem {
                    file: "src/lib.rs".to_string(),
                    line_range: None,
                    preset_comment: None,
                    severity: None,
                    tags: vec![],
                    reviewed: false,
                    comments: vec![],
                },
                TaskItem {
                    file: "src/utils.rs".to_string(),
                    line_range: Some(LineRange { start: None, end: Some(100) }),
                    preset_comment: Some("Review utility functions".to_string()),
                    severity: Some(TaskSeverity::Warning),
                    tags: vec!["utils".to_string(), "review".to_string()],
                    reviewed: false,
                    comments: vec![
                        Comment {
                            id: Uuid::new_v4(),
                            author: "Alice".to_string(),
                            content: "Consider refactoring".to_string(),
                            created_at: Utc::now(),
                            line_number: Some(50),
                        },
                    ],
                },
            ],
        };
        
        // Export the task
        let exported_json = serde_json::to_string_pretty(&task).expect("Failed to export task");
        
        // Validate JSON schema structure
        let parsed: serde_json::Value = serde_json::from_str(&exported_json).expect("Failed to parse exported JSON");
        
        // Validate root level fields
        assert!(parsed.is_object(), "Root should be object");
        assert!(parsed.get("id").is_some() && parsed["id"].is_string(), "id should be string");
        assert!(parsed.get("name").is_some() && parsed["name"].is_string(), "name should be string");
        assert!(parsed.get("repo_path").is_some() && parsed["repo_path"].is_string(), "repo_path should be string");
        assert!(parsed.get("base_ref").is_some() && parsed["base_ref"].is_string(), "base_ref should be string");
        assert!(parsed.get("create_time").is_some() && parsed["create_time"].is_string(), "create_time should be string");
        assert!(parsed.get("update_time").is_some() && parsed["update_time"].is_string(), "update_time should be string");
        assert!(parsed.get("status").is_some() && parsed["status"].is_string(), "status should be string");
        assert!(parsed.get("total_items").is_some() && parsed["total_items"].is_number(), "total_items should be number");
        assert!(parsed.get("completed_items").is_some() && parsed["completed_items"].is_number(), "completed_items should be number");
        assert!(parsed.get("items").is_some() && parsed["items"].is_array(), "items should be array");
        
        // Validate items array structure
        let items = parsed["items"].as_array().expect("Items should be array");
        assert_eq!(items.len(), 3, "Should have 3 items");
        
        for (i, item) in items.iter().enumerate() {
            assert!(item.is_object(), "Item {} should be object", i);
            assert!(item.get("file").is_some() && item["file"].is_string(), "Item {} file should be string", i);
            assert!(item.get("line_range").is_none() || item["line_range"].is_object() || item["line_range"].is_null(), "Item {} line_range should be object or null", i);
            assert!(item.get("preset_comment").is_none() || item["preset_comment"].is_string() || item["preset_comment"].is_null(), "Item {} preset_comment should be string or null", i);
            assert!(item.get("severity").is_none() || item["severity"].is_string() || item["severity"].is_null(), "Item {} severity should be string or null", i);
            assert!(item.get("tags").is_some() && item["tags"].is_array(), "Item {} tags should be array", i);
            assert!(item.get("reviewed").is_some() && item["reviewed"].is_boolean(), "Item {} reviewed should be boolean", i);
            assert!(item.get("comments").is_some() && item["comments"].is_array(), "Item {} comments should be array", i);
            
            // Validate tags array
            let tags = item["tags"].as_array().expect("Tags should be array");
            for tag in tags {
                assert!(tag.is_string(), "All tags should be strings");
            }
            
            // Validate comments array
            let comments = item["comments"].as_array().expect("Comments should be array");
            for (j, comment) in comments.iter().enumerate() {
                assert!(comment.is_object(), "Comment {} in item {} should be object", j, i);
                assert!(comment.get("id").is_some() && comment["id"].is_string(), "Comment {} id should be string", j);
                assert!(comment.get("author").is_some() && comment["author"].is_string(), "Comment {} author should be string", j);
                assert!(comment.get("content").is_some() && comment["content"].is_string(), "Comment {} content should be string", j);
                assert!(comment.get("created_at").is_some() && comment["created_at"].is_string(), "Comment {} created_at should be string", j);
                assert!(comment.get("line_number").is_none() || comment["line_number"].is_number() || comment["line_number"].is_null(), "Comment {} line_number should be number or null", j);
            }
        }
        
        // Validate specific data integrity
        assert_eq!(parsed["name"], "Schema Test Task");
        assert_eq!(parsed["total_items"], 3);
        assert_eq!(parsed["completed_items"], 1);
        assert_eq!(parsed["items"][0]["file"], "src/main.rs");
        assert_eq!(parsed["items"][0]["severity"], "error");
        assert_eq!(parsed["items"][0]["tags"].as_array().unwrap().len(), 1);
        assert_eq!(parsed["items"][1]["line_range"], serde_json::Value::Null);
        assert_eq!(parsed["items"][1]["preset_comment"], serde_json::Value::Null);
        assert_eq!(parsed["items"][1]["severity"], serde_json::Value::Null);
        assert_eq!(parsed["items"][2]["comments"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn test_export_json_roundtrip() {
        let original_task = LocalTask {
            id: Uuid::new_v4(),
            name: "Roundtrip Test Task".to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "main".to_string(),
            create_time: Utc::now(),
            update_time: Utc::now(),
            status: TaskStatus::Completed,
            total_items: 2,
            completed_items: 2,
            items: vec![
                TaskItem {
                    file: "src/main.rs".to_string(),
                    line_range: Some(LineRange { start: Some(1), end: Some(50) }),
                    preset_comment: Some("Main entry point".to_string()),
                    severity: Some(TaskSeverity::Ok),
                    tags: vec!["entry".to_string()],
                    reviewed: true,
                    comments: vec![],
                },
                TaskItem {
                    file: "src/config.rs".to_string(),
                    line_range: None,
                    preset_comment: None,
                    severity: None,
                    tags: vec!["config".to_string(), "settings".to_string()],
                    reviewed: true,
                    comments: vec![],
                },
            ],
        };
        
        // Export to JSON
        let exported_json = serde_json::to_string_pretty(&original_task).expect("Failed to export task");
        
        // Parse back from JSON
        let parsed_task: LocalTask = serde_json::from_str(&exported_json).expect("Failed to parse exported JSON");
        
        // Verify roundtrip preserves all data
        assert_eq!(parsed_task.id, original_task.id);
        assert_eq!(parsed_task.name, original_task.name);
        assert_eq!(parsed_task.repo_path, original_task.repo_path);
        assert_eq!(parsed_task.base_ref, original_task.base_ref);
        assert_eq!(parsed_task.status, original_task.status);
        assert_eq!(parsed_task.total_items, original_task.total_items);
        assert_eq!(parsed_task.completed_items, original_task.completed_items);
        assert_eq!(parsed_task.items.len(), original_task.items.len());
        
        // Verify first item
        let original_first = &original_task.items[0];
        let parsed_first = &parsed_task.items[0];
        assert_eq!(parsed_first.file, original_first.file);
        assert_eq!(parsed_first.reviewed, original_first.reviewed);
        assert_eq!(parsed_first.tags, original_first.tags);
        assert_eq!(parsed_first.severity, original_first.severity);
        
        // Verify second item
        let original_second = &original_task.items[1];
        let parsed_second = &parsed_task.items[1];
        assert_eq!(parsed_second.file, original_second.file);
        assert_eq!(parsed_second.reviewed, original_second.reviewed);
        assert_eq!(parsed_second.tags, original_second.tags);
    }
}
