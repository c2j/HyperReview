#[cfg(test)]
mod tests {
    use crate::commands::task_commands::update_task_progress;
    use crate::models::task::{LocalTask, TaskStatus, TaskItem};
    use crate::storage::task_store::TaskStore;
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

    async fn create_test_task() -> LocalTask {
        let items = vec![
            TaskItem {
                file: "file1.js".to_string(),
                line_range: None,
                preset_comment: Some("Review first file".to_string()),
                severity: None,
                tags: Vec::new(),
                reviewed: false,
                comments: Vec::new(),
            },
            TaskItem {
                file: "file2.ts".to_string(),
                line_range: None,
                preset_comment: Some("Review second file".to_string()),
                severity: None,
                tags: Vec::new(),
                reviewed: false,
                comments: Vec::new(),
            },
            TaskItem {
                file: "file3.py".to_string(),
                line_range: None,
                preset_comment: Some("Review third file".to_string()),
                severity: None,
                tags: Vec::new(),
                reviewed: false,
                comments: Vec::new(),
            },
        ];

        let task = LocalTask {
            id: Uuid::new_v4(),
            name: "Test Task".to_string(),
            repo_path: "/test/repo".to_string(),
            base_ref: "main".to_string(),
            create_time: Utc::now(),
            update_time: Utc::now(),
            status: TaskStatus::InProgress,
            total_items: items.len() as u32,
            completed_items: 0,
            items,
        };

        // Save the task first
        let store = TaskStore::new().unwrap();
        store.save_task(&task).unwrap();
        
        task
    }

    #[tokio::test]
    async fn test_update_task_progress_mark_item_reviewed() {
        setup_test_environment();
        let task = create_test_task().await;
        let task_id = task.id;

        // Mark first item as reviewed
        let result = update_task_progress(task_id, 0, true).await;
        
        assert!(result.is_ok());
        
        // Verify the update
        let store = TaskStore::new().unwrap();
        let updated_task = store.load_task(task_id).unwrap();
        
        assert!(updated_task.items[0].reviewed);
        assert!(!updated_task.items[1].reviewed);
        assert!(!updated_task.items[2].reviewed);
        assert_eq!(updated_task.completed_items, 1);
        assert!(updated_task.update_time > task.update_time);
        
        cleanup_test_environment();
    }

    #[tokio::test]
    async fn test_update_task_progress_mark_item_unreviewed() {
        setup_test_environment();
        let mut task = create_test_task().await;
        task.items[0].reviewed = true;
        task.items[1].reviewed = true;
        task.completed_items = 2;
        
        let store = TaskStore::new().unwrap();
        store.save_task(&task).unwrap();
        let task_id = task.id;

        // Mark first item as unreviewed
        let result = update_task_progress(task_id, 0, false).await;
        
        assert!(result.is_ok());
        
        // Verify the update
        let updated_task = store.load_task(task_id).unwrap();
        
        assert!(!updated_task.items[0].reviewed);
        assert!(updated_task.items[1].reviewed);
        assert!(!updated_task.items[2].reviewed);
        assert_eq!(updated_task.completed_items, 1);
        
        cleanup_test_environment();
    }

    #[tokio::test]
    async fn test_update_task_progress_no_change_when_same_state() {
        setup_test_environment();
        let mut task = create_test_task().await;
        task.items[0].reviewed = true;
        task.completed_items = 1;
        
        let store = TaskStore::new().unwrap();
        store.save_task(&task).unwrap();
        let task_id = task.id;
        let original_update_time = task.update_time;

        // Try to mark already reviewed item as reviewed again
        let result = update_task_progress(task_id, 0, true).await;
        
        assert!(result.is_ok());
        
        // Verify no change occurred
        let updated_task = store.load_task(task_id).unwrap();
        
        assert!(updated_task.items[0].reviewed);
        assert_eq!(updated_task.completed_items, 1);
        assert_eq!(updated_task.update_time, original_update_time);
        
        cleanup_test_environment();
    }

    #[tokio::test]
    async fn test_update_task_progress_invalid_index() {
        setup_test_environment();
        let task = create_test_task().await;
        let task_id = task.id;

        // Try to update item with invalid index
        let result = update_task_progress(task_id, 10, true).await;
        
        assert!(result.is_ok()); // Should not error, just do nothing
        
        // Verify no change occurred
        let updated_task = TaskStore::new().unwrap().load_task(task_id).unwrap();
        
        assert_eq!(updated_task.completed_items, 0);
        for item in &updated_task.items {
            assert!(!item.reviewed);
        }
        
        cleanup_test_environment();
    }

    #[tokio::test]
    async fn test_update_task_progress_all_items() {
        setup_test_environment();
        let task = create_test_task().await;
        let task_id = task.id;

        // Mark all items as reviewed
        for i in 0..task.items.len() {
            let result = update_task_progress(task_id, i, true).await;
            assert!(result.is_ok());
        }
        
        // Verify all items are reviewed
        let updated_task = TaskStore::new().unwrap().load_task(task_id).unwrap();
        
        for item in &updated_task.items {
            assert!(item.reviewed);
        }
        assert_eq!(updated_task.completed_items, 3);
        assert_eq!(updated_task.completed_items, updated_task.total_items);
        
        cleanup_test_environment();
    }

    #[tokio::test]
    async fn test_update_task_progress_toggle_multiple_times() {
        setup_test_environment();
        let task = create_test_task().await;
        let task_id = task.id;

        // Toggle first item multiple times
        let result1 = update_task_progress(task_id, 0, true).await;
        assert!(result1.is_ok());
        
        let result2 = update_task_progress(task_id, 0, false).await;
        assert!(result2.is_ok());
        
        let result3 = update_task_progress(task_id, 0, true).await;
        assert!(result3.is_ok());
        
        let result4 = update_task_progress(task_id, 0, false).await;
        assert!(result4.is_ok());
        
        // Verify final state
        let updated_task = TaskStore::new().unwrap().load_task(task_id).unwrap();
        
        assert!(!updated_task.items[0].reviewed);
        assert_eq!(updated_task.completed_items, 0);
        
        cleanup_test_environment();
    }

    #[tokio::test]
    async fn test_update_task_progress_nonexistent_task() {
        setup_test_environment();

        let nonexistent_task_id = Uuid::new_v4();
        
        let result = update_task_progress(nonexistent_task_id, 0, true).await;
        
        // Should return error for nonexistent task
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Task not found"));
        
        cleanup_test_environment();
    }

    #[tokio::test]
    async fn test_update_task_progress_saturating_subtract() {
        setup_test_environment();
        let mut task = create_test_task().await;
        task.completed_items = 0;
        
        let store = TaskStore::new().unwrap();
        store.save_task(&task).unwrap();
        let task_id = task.id;

        // Try to unreview an item when completed_items is already 0
        let result = update_task_progress(task_id, 0, false).await;
        
        assert!(result.is_ok());
        
        // Verify completed_items doesn't go below 0
        let updated_task = store.load_task(task_id).unwrap();
        assert_eq!(updated_task.completed_items, 0);
        
        cleanup_test_environment();
    }

    #[tokio::test]
    async fn test_update_task_progress_timestamp_updated() {
        setup_test_environment();
        let task = create_test_task().await;
        let task_id = task.id;
        let original_create_time = task.create_time;

        // Wait a bit to ensure timestamp difference
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        
        let result = update_task_progress(task_id, 0, true).await;
        assert!(result.is_ok());
        
        let updated_task = TaskStore::new().unwrap().load_task(task_id).unwrap();
        
        // Create time should remain the same
        assert_eq!(updated_task.create_time, original_create_time);
        // Update time should be newer
        assert!(updated_task.update_time > original_create_time);
        
        cleanup_test_environment();
    }

    #[tokio::test]
    async fn test_update_task_progress_zero_index() {
        setup_test_environment();
        let task = create_test_task().await;
        let task_id = task.id;

        let result = update_task_progress(task_id, 0, true).await;
        assert!(result.is_ok());
        
        let updated_task = TaskStore::new().unwrap().load_task(task_id).unwrap();
        
        assert!(updated_task.items[0].reviewed);
        assert_eq!(updated_task.completed_items, 1);
        
        cleanup_test_environment();
    }

    #[tokio::test]
    async fn test_update_task_progress_last_index() {
        setup_test_environment();
        let task = create_test_task().await;
        let task_id = task.id;
        let last_index = task.items.len() - 1;

        let result = update_task_progress(task_id, last_index, true).await;
        assert!(result.is_ok());
        
        let updated_task = TaskStore::new().unwrap().load_task(task_id).unwrap();
        
        assert!(!updated_task.items[0].reviewed);
        assert!(!updated_task.items[1].reviewed);
        assert!(updated_task.items[last_index].reviewed);
        assert_eq!(updated_task.completed_items, 1);
        
        cleanup_test_environment();
    }
}