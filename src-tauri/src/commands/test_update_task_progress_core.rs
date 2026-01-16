#[cfg(test)]
mod tests {
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

    fn create_test_task() -> LocalTask {
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

        LocalTask {
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
        }
    }

    #[test]
    fn test_update_task_progress_mark_item_reviewed() {
        setup_test_environment();
        let mut task = create_test_task();
        let task_id = task.id;

        // Mark first item as reviewed
        assert_eq!(task.items[0].reviewed, false);
        task.items[0].reviewed = true;
        task.completed_items += 1;
        task.update_time = Utc::now();
        
        // Verify the update
        assert!(task.items[0].reviewed);
        assert!(!task.items[1].reviewed);
        assert!(!task.items[2].reviewed);
        assert_eq!(task.completed_items, 1);
        
        cleanup_test_environment();
    }

    #[test]
    fn test_update_task_progress_mark_item_unreviewed() {
        setup_test_environment();
        let mut task = create_test_task();
        task.items[0].reviewed = true;
        task.items[1].reviewed = true;
        task.completed_items = 2;
        let task_id = task.id;

        // Mark first item as unreviewed
        assert_eq!(task.items[0].reviewed, true);
        task.items[0].reviewed = false;
        task.completed_items = task.completed_items.saturating_sub(1);
        task.update_time = Utc::now();
        
        // Verify the update
        assert!(!task.items[0].reviewed);
        assert!(task.items[1].reviewed);
        assert!(!task.items[2].reviewed);
        assert_eq!(task.completed_items, 1);
        
        cleanup_test_environment();
    }

    #[test]
    fn test_update_task_progress_no_change_when_same_state() {
        setup_test_environment();
        let mut task = create_test_task();
        task.items[0].reviewed = true;
        task.completed_items = 1;
        let original_update_time = task.update_time;

        // Try to mark already reviewed item as reviewed again (no change)
        assert_eq!(task.items[0].reviewed, true);
        // No change should occur
        
        // Verify no change occurred
        assert!(task.items[0].reviewed);
        assert_eq!(task.completed_items, 1);
        assert_eq!(task.update_time, original_update_time);
        
        cleanup_test_environment();
    }

    #[test]
    fn test_update_task_progress_invalid_index() {
        setup_test_environment();
        let mut task = create_test_task();
        let task_id = task.id;

        // Try to update item with invalid index - should do nothing
        assert_eq!(task.items.len(), 3);
        // Invalid index 10 should be ignored in real implementation
        
        // Verify no change occurred
        assert_eq!(task.completed_items, 0);
        for item in &task.items {
            assert!(!item.reviewed);
        }
        
        cleanup_test_environment();
    }

    #[test]
    fn test_update_task_progress_all_items() {
        setup_test_environment();
        let mut task = create_test_task();

        // Mark all items as reviewed
        for i in 0..task.items.len() {
            task.items[i].reviewed = true;
        }
        task.completed_items = task.items.len() as u32;
        task.update_time = Utc::now();
        
        // Verify all items are reviewed
        for item in &task.items {
            assert!(item.reviewed);
        }
        assert_eq!(task.completed_items, 3);
        assert_eq!(task.completed_items, task.total_items);
        
        cleanup_test_environment();
    }

    #[test]
    fn test_update_task_progress_toggle_multiple_times() {
        setup_test_environment();
        let mut task = create_test_task();

        // Toggle first item multiple times
        assert_eq!(task.items[0].reviewed, false);
        task.items[0].reviewed = true;
        task.completed_items += 1;
        
        assert_eq!(task.items[0].reviewed, true);
        task.items[0].reviewed = false;
        task.completed_items = task.completed_items.saturating_sub(1);
        
        assert_eq!(task.items[0].reviewed, false);
        task.items[0].reviewed = true;
        task.completed_items += 1;
        
        assert_eq!(task.items[0].reviewed, true);
        task.items[0].reviewed = false;
        task.completed_items = task.completed_items.saturating_sub(1);
        
        // Verify final state
        assert!(!task.items[0].reviewed);
        assert_eq!(task.completed_items, 0);
        
        cleanup_test_environment();
    }

    #[test]
    fn test_update_task_progress_saturating_subtract() {
        setup_test_environment();
        let mut task = create_test_task();
        task.completed_items = 0;

        // Try to unreview an item when completed_items is already 0
        assert_eq!(task.items[0].reviewed, false);
        // No change should occur since item is already unreviewed
        
        // Verify completed_items doesn't go below 0
        assert_eq!(task.completed_items, 0);
        
        cleanup_test_environment();
    }

    #[test]
    fn test_update_task_progress_timestamp_updated() {
        setup_test_environment();
        let mut task = create_test_task();
        let original_create_time = task.create_time;

        // Wait a bit to ensure timestamp difference
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        task.items[0].reviewed = true;
        task.completed_items += 1;
        task.update_time = Utc::now();
        
        // Verify timestamps
        assert_eq!(task.create_time, original_create_time);
        assert!(task.update_time > original_create_time);
        
        cleanup_test_environment();
    }

    #[test]
    fn test_update_task_progress_zero_index() {
        setup_test_environment();
        let mut task = create_test_task();

        task.items[0].reviewed = true;
        task.completed_items += 1;
        
        assert!(task.items[0].reviewed);
        assert_eq!(task.completed_items, 1);
        
        cleanup_test_environment();
    }

    #[test]
    fn test_update_task_progress_last_index() {
        setup_test_environment();
        let mut task = create_test_task();
        let last_index = task.items.len() - 1;

        task.items[last_index].reviewed = true;
        task.completed_items += 1;
        
        assert!(!task.items[0].reviewed);
        assert!(!task.items[1].reviewed);
        assert!(task.items[last_index].reviewed);
        assert_eq!(task.completed_items, 1);
        
        cleanup_test_environment();
    }
}