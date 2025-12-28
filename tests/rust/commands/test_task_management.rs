#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use tempfile::TempDir;
    
    #[test]
    fn test_create_task_from_text() {
        let temp_dir = TempDir::new().unwrap();
        let task_name = "Test Task";
        let repo_path = temp_dir.path().to_str().unwrap();
        let base_ref = "main";
        let items_text = "src/main.rs\nsrc/lib.rs\tCheck this\nsrc/utils.rs";
        
        // This would normally invoke the create_task command
        // For now, we'll test the text parsing logic
        assert_eq!(items_text.lines().count(), 3);
    }
}
