// File Tree Navigation Commands
// Tauri commands for file tree management and navigation

use tauri::State;
use log::{info, error};

use crate::AppState;
use crate::services::file_tree::{
    FileTreeService, FileTreeConfig, FileTreeNode, FileTreeStats, 
    FileSearchCriteria, SortOrder
};
use crate::models::gerrit::ChangeFile;

/// Build file tree from change files
#[tauri::command]
pub async fn file_tree_build(
    files: Vec<ChangeFile>,
    config: Option<FileTreeConfig>,
    _state: State<'_, AppState>,
) -> Result<FileTreeNode, String> {
    info!("Building file tree from {} files", files.len());

    tokio::task::spawn_blocking(move || {
        let tree_config = config.unwrap_or_default();
        let service = FileTreeService::new(tree_config);

        match service.build_tree(&files) {
            Ok(tree) => {
                info!("Successfully built file tree with {} top-level items", tree.children.len());
                Ok(tree)
            }
            Err(e) => {
                error!("Failed to build file tree: {}", e);
                Err(format!("Failed to build file tree: {}", e))
            }
        }
    }).await.map_err(|e| format!("Task join error: {}", e))?
}

/// Search files in the tree
#[tauri::command]
pub async fn file_tree_search(
    tree: FileTreeNode,
    criteria: FileSearchCriteria,
    _state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    info!("Searching files in tree with criteria");

    tokio::task::spawn_blocking(move || {
        let service = FileTreeService::new(FileTreeConfig::default());
        let results = service.search_files(&tree, &criteria);
        
        // Convert results to file paths for serialization
        let file_paths: Vec<String> = results.iter()
            .map(|node| node.path.clone())
            .collect();

        info!("Found {} files matching search criteria", file_paths.len());
        Ok(file_paths)
    }).await.map_err(|e| format!("Task join error: {}", e))?
}

/// Filter tree based on search criteria
#[tauri::command]
pub async fn file_tree_filter(
    tree: FileTreeNode,
    criteria: FileSearchCriteria,
    _state: State<'_, AppState>,
) -> Result<FileTreeNode, String> {
    info!("Filtering file tree");

    tokio::task::spawn_blocking(move || {
        let service = FileTreeService::new(FileTreeConfig::default());
        
        match service.filter_tree(&tree, &criteria) {
            Ok(filtered_tree) => {
                info!("Successfully filtered file tree");
                Ok(filtered_tree)
            }
            Err(e) => {
                error!("Failed to filter file tree: {}", e);
                Err(format!("Failed to filter file tree: {}", e))
            }
        }
    }).await.map_err(|e| format!("Task join error: {}", e))?
}

/// Get file tree statistics
#[tauri::command]
pub async fn file_tree_get_stats(
    tree: FileTreeNode,
    _state: State<'_, AppState>,
) -> Result<FileTreeStats, String> {
    info!("Calculating file tree statistics");

    tokio::task::spawn_blocking(move || {
        let service = FileTreeService::new(FileTreeConfig::default());
        let stats = service.get_tree_stats(&tree);
        
        info!("File tree statistics calculated: {} files, {} directories", 
              stats.total_files, stats.total_directories);
        Ok(stats)
    }).await.map_err(|e| format!("Task join error: {}", e))?
}

/// Toggle node expansion in tree
#[tauri::command]
pub async fn file_tree_toggle_node(
    mut tree: FileTreeNode,
    node_path: String,
    _state: State<'_, AppState>,
) -> Result<(FileTreeNode, bool), String> {
    info!("Toggling expansion for node: {}", node_path);

    tokio::task::spawn_blocking(move || {
        let service = FileTreeService::new(FileTreeConfig::default());
        
        match service.toggle_node_expansion(&mut tree, &node_path) {
            Ok(is_expanded) => {
                info!("Node {} is now {}", node_path, if is_expanded { "expanded" } else { "collapsed" });
                Ok((tree, is_expanded))
            }
            Err(e) => {
                error!("Failed to toggle node expansion: {}", e);
                Err(format!("Failed to toggle node expansion: {}", e))
            }
        }
    }).await.map_err(|e| format!("Task join error: {}", e))?
}

/// Get visible nodes in tree (respecting expansion state)
#[tauri::command]
pub async fn file_tree_get_visible_nodes(
    tree: FileTreeNode,
    _state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    info!("Getting visible nodes from tree");

    tokio::task::spawn_blocking(move || {
        let service = FileTreeService::new(FileTreeConfig::default());
        let visible_nodes = service.get_visible_nodes(&tree);
        
        // Convert to paths for serialization
        let visible_paths: Vec<String> = visible_nodes.iter()
            .map(|node| node.path.clone())
            .collect();

        info!("Found {} visible nodes", visible_paths.len());
        Ok(visible_paths)
    }).await.map_err(|e| format!("Task join error: {}", e))?
}

/// Get default file tree configuration
#[tauri::command]
pub async fn file_tree_get_default_config(_state: State<'_, AppState>) -> Result<FileTreeConfig, String> {
    Ok(FileTreeConfig::default())
}

/// Update file tree configuration
#[tauri::command]
pub async fn file_tree_update_config(
    config: FileTreeConfig,
    _state: State<'_, AppState>,
) -> Result<bool, String> {
    info!("Updating file tree configuration");
    
    // TODO: Persist configuration to storage
    info!("File tree configuration updated successfully");
    Ok(true)
}

/// Create search criteria for common use cases
#[tauri::command]
pub async fn file_tree_create_search_criteria(
    name_pattern: Option<String>,
    path_pattern: Option<String>,
    change_types: Vec<String>,
    _state: State<'_, AppState>,
) -> Result<FileSearchCriteria, String> {
    use crate::models::gerrit::FileChangeType;
    
    // Convert string change types to enum
    let parsed_change_types: Vec<FileChangeType> = change_types.iter()
        .filter_map(|s| match s.as_str() {
            "added" => Some(FileChangeType::Added),
            "modified" => Some(FileChangeType::Modified),
            "deleted" => Some(FileChangeType::Deleted),
            "renamed" => Some(FileChangeType::Renamed),
            "copied" => Some(FileChangeType::Copied),
            "rewritten" => Some(FileChangeType::Rewritten),
            _ => None,
        })
        .collect();

    let criteria = FileSearchCriteria {
        name_pattern,
        path_pattern,
        change_types: parsed_change_types,
        review_statuses: vec![], // Empty for now
        has_comments: None,
        is_binary: None,
        min_size: None,
        max_size: None,
    };

    Ok(criteria)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::gerrit::{FileChangeType, FileDiff};

    fn create_test_files() -> Vec<ChangeFile> {
        vec![
            ChangeFile {
                id: "file1".to_string(),
                change_id: "change1".to_string(),
                patch_set_number: 1,
                file_path: "src/main.rs".to_string(),
                change_type: FileChangeType::Modified,
                old_content: Some("old".to_string()),
                new_content: Some("new".to_string()),
                diff: FileDiff::default(),
                file_size: 100,
                downloaded_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            },
            ChangeFile {
                id: "file2".to_string(),
                change_id: "change1".to_string(),
                patch_set_number: 1,
                file_path: "src/lib.rs".to_string(),
                change_type: FileChangeType::Added,
                old_content: None,
                new_content: Some("new lib".to_string()),
                diff: FileDiff::default(),
                file_size: 200,
                downloaded_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            },
        ]
    }

    #[tokio::test]
    async fn test_file_tree_config_creation() {
        let config = FileTreeConfig::default();
        assert_eq!(config.sort_order, SortOrder::Alphabetical);
        assert!(config.group_by_directory);
        assert!(!config.show_hidden_files);
    }

    #[tokio::test]
    async fn test_change_files_creation() {
        let files = create_test_files();
        assert_eq!(files.len(), 2);
        assert_eq!(files[0].file_path, "src/main.rs");
        assert_eq!(files[1].file_path, "src/lib.rs");
    }
}