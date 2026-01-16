// File Tree Navigation Service
// Handles hierarchical file tree structure, filtering, and search for change files

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use log::{info, debug};

use crate::errors::HyperReviewError;
use crate::models::gerrit::{ChangeFile, FileChangeType};

/// File tree navigation service
pub struct FileTreeService {
    config: FileTreeConfig,
}

/// Configuration for file tree display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTreeConfig {
    pub show_hidden_files: bool,
    pub group_by_directory: bool,
    pub sort_order: SortOrder,
    pub filter_by_status: Vec<FileChangeType>,
    pub expand_all: bool,
    pub max_depth: Option<u32>,
}

impl Default for FileTreeConfig {
    fn default() -> Self {
        Self {
            show_hidden_files: false,
            group_by_directory: true,
            sort_order: SortOrder::Alphabetical,
            filter_by_status: vec![
                FileChangeType::Added,
                FileChangeType::Modified,
                FileChangeType::Deleted,
                FileChangeType::Renamed,
                FileChangeType::Copied,
                FileChangeType::Rewritten,
            ],
            expand_all: false,
            max_depth: None,
        }
    }
}

/// Sort order for file tree
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SortOrder {
    Alphabetical,
    ByChangeType,
    ByFileSize,
    ByPath,
}

/// File tree node representing a file or directory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTreeNode {
    pub id: String,
    pub name: String,
    pub path: String,
    pub node_type: NodeType,
    pub change_info: Option<FileChangeInfo>,
    pub children: Vec<FileTreeNode>,
    pub is_expanded: bool,
    pub depth: u32,
    pub parent_path: Option<String>,
}

/// Type of tree node
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeType {
    Directory,
    File,
}

/// File change information for tree nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChangeInfo {
    pub change_type: FileChangeType,
    pub file_size: u64,
    pub lines_added: u32,
    pub lines_removed: u32,
    pub is_binary: bool,
    pub has_comments: bool,
    pub comment_count: u32,
    pub review_status: FileReviewStatus,
}

/// Review status for files in the tree
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FileReviewStatus {
    Unreviewed,
    InProgress,
    Reviewed,
    HasComments,
    Approved,
    NeedsWork,
}

/// File tree statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTreeStats {
    pub total_files: u32,
    pub total_directories: u32,
    pub files_by_type: HashMap<FileChangeType, u32>,
    pub files_by_status: HashMap<FileReviewStatus, u32>,
    pub total_lines_added: u32,
    pub total_lines_removed: u32,
    pub binary_files: u32,
}

/// Search criteria for file tree filtering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSearchCriteria {
    pub name_pattern: Option<String>,
    pub path_pattern: Option<String>,
    pub change_types: Vec<FileChangeType>,
    pub review_statuses: Vec<FileReviewStatus>,
    pub has_comments: Option<bool>,
    pub is_binary: Option<bool>,
    pub min_size: Option<u64>,
    pub max_size: Option<u64>,
}

impl FileTreeService {
    /// Create a new file tree service
    pub fn new(config: FileTreeConfig) -> Self {
        Self { config }
    }

    /// Build file tree from change files
    pub fn build_tree(&self, files: &[ChangeFile]) -> Result<FileTreeNode, HyperReviewError> {
        info!("Building file tree from {} files", files.len());

        let mut root = FileTreeNode {
            id: "root".to_string(),
            name: "Root".to_string(),
            path: "/".to_string(),
            node_type: NodeType::Directory,
            change_info: None,
            children: Vec::new(),
            is_expanded: self.config.expand_all,
            depth: 0,
            parent_path: None,
        };

        // Filter files based on configuration
        let filtered_files: Vec<&ChangeFile> = files.iter()
            .filter(|file| self.should_include_file(file))
            .collect();

        debug!("Filtered to {} files", filtered_files.len());

        // Build tree structure
        for file in filtered_files {
            self.insert_file_into_tree(&mut root, file)?;
        }

        // Sort tree nodes
        self.sort_tree(&mut root);

        // Apply depth limits
        if let Some(max_depth) = self.config.max_depth {
            self.limit_tree_depth(&mut root, max_depth);
        }

        info!("File tree built successfully with {} top-level items", root.children.len());
        Ok(root)
    }

    /// Search files in the tree
    pub fn search_files<'a>(
        &self,
        tree: &'a FileTreeNode,
        criteria: &FileSearchCriteria,
    ) -> Vec<&'a FileTreeNode> {
        debug!("Searching files with criteria: {:?}", criteria);

        let mut results = Vec::new();
        self.search_recursive(tree, criteria, &mut results);

        info!("Found {} files matching search criteria", results.len());
        results
    }

    /// Filter tree based on search criteria
    pub fn filter_tree(
        &self,
        tree: &FileTreeNode,
        criteria: &FileSearchCriteria,
    ) -> Result<FileTreeNode, HyperReviewError> {
        debug!("Filtering tree with criteria: {:?}", criteria);

        let filtered_tree = self.filter_node_recursive(tree, criteria)?;
        
        info!("Tree filtered successfully");
        Ok(filtered_tree)
    }

    /// Get file tree statistics
    pub fn get_tree_stats(&self, tree: &FileTreeNode) -> FileTreeStats {
        debug!("Calculating tree statistics");

        let mut stats = FileTreeStats {
            total_files: 0,
            total_directories: 0,
            files_by_type: HashMap::new(),
            files_by_status: HashMap::new(),
            total_lines_added: 0,
            total_lines_removed: 0,
            binary_files: 0,
        };

        self.calculate_stats_recursive(tree, &mut stats);

        info!("Tree statistics calculated: {} files, {} directories", 
              stats.total_files, stats.total_directories);
        stats
    }

    /// Expand or collapse tree node
    pub fn toggle_node_expansion(
        &self,
        tree: &mut FileTreeNode,
        node_path: &str,
    ) -> Result<bool, HyperReviewError> {
        debug!("Toggling expansion for node: {}", node_path);

        if let Some(node) = self.find_node_mut(tree, node_path) {
            node.is_expanded = !node.is_expanded;
            info!("Node {} is now {}", node_path, if node.is_expanded { "expanded" } else { "collapsed" });
            Ok(node.is_expanded)
        } else {
            Err(HyperReviewError::other(format!("Node not found: {}", node_path)))
        }
    }

    /// Get flattened list of visible nodes (respecting expansion state)
    pub fn get_visible_nodes<'a>(&self, tree: &'a FileTreeNode) -> Vec<&'a FileTreeNode> {
        let mut visible = Vec::new();
        self.collect_visible_nodes(tree, &mut visible, true);
        visible
    }

    // Private helper methods

    fn should_include_file(&self, file: &ChangeFile) -> bool {
        // Check if file type is in filter
        if !self.config.filter_by_status.contains(&file.change_type) {
            return false;
        }

        // Check hidden files
        if !self.config.show_hidden_files {
            let file_name = Path::new(&file.file_path)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("");
            
            if file_name.starts_with('.') {
                return false;
            }
        }

        true
    }

    fn insert_file_into_tree(
        &self,
        root: &mut FileTreeNode,
        file: &ChangeFile,
    ) -> Result<(), HyperReviewError> {
        let path_parts: Vec<&str> = file.file_path.split('/').collect();
        let mut current_node = root;

        // Create directory nodes for path components (except the last one which is the file)
        for (i, part) in path_parts.iter().enumerate() {
            if i == path_parts.len() - 1 {
                // This is the file itself
                let file_node = FileTreeNode {
                    id: file.id.clone(),
                    name: part.to_string(),
                    path: file.file_path.clone(),
                    node_type: NodeType::File,
                    change_info: Some(FileChangeInfo {
                        change_type: file.change_type.clone(),
                        file_size: file.file_size,
                        lines_added: 0, // TODO: Calculate from diff
                        lines_removed: 0, // TODO: Calculate from diff
                        is_binary: false, // TODO: Detect binary files
                        has_comments: false, // TODO: Check for comments
                        comment_count: 0, // TODO: Count comments
                        review_status: FileReviewStatus::Unreviewed, // TODO: Get actual status
                    }),
                    children: Vec::new(),
                    is_expanded: false,
                    depth: current_node.depth + 1,
                    parent_path: Some(current_node.path.clone()),
                };
                current_node.children.push(file_node);
            } else {
                // This is a directory component
                let dir_path = path_parts[..=i].join("/");
                
                // Check if directory already exists
                let dir_exists = current_node.children.iter()
                    .any(|child| child.path == dir_path && child.node_type == NodeType::Directory);

                if dir_exists {
                    // Find the existing directory
                    current_node = current_node.children.iter_mut()
                        .find(|child| child.path == dir_path && child.node_type == NodeType::Directory)
                        .unwrap();
                } else {
                    // Create new directory node
                    let dir_node = FileTreeNode {
                        id: format!("dir_{}", dir_path.replace('/', "_")),
                        name: part.to_string(),
                        path: dir_path,
                        node_type: NodeType::Directory,
                        change_info: None,
                        children: Vec::new(),
                        is_expanded: self.config.expand_all,
                        depth: current_node.depth + 1,
                        parent_path: Some(current_node.path.clone()),
                    };
                    current_node.children.push(dir_node);
                    current_node = current_node.children.last_mut().unwrap();
                }
            }
        }

        Ok(())
    }

    fn sort_tree(&self, node: &mut FileTreeNode) {
        // Sort children recursively
        for child in &mut node.children {
            self.sort_tree(child);
        }

        // Sort current node's children
        match self.config.sort_order {
            SortOrder::Alphabetical => {
                node.children.sort_by(|a, b| {
                    // Directories first, then files
                    match (&a.node_type, &b.node_type) {
                        (NodeType::Directory, NodeType::File) => std::cmp::Ordering::Less,
                        (NodeType::File, NodeType::Directory) => std::cmp::Ordering::Greater,
                        _ => a.name.cmp(&b.name),
                    }
                });
            }
            SortOrder::ByChangeType => {
                node.children.sort_by(|a, b| {
                    match (&a.change_info, &b.change_info) {
                        (Some(info_a), Some(info_b)) => info_a.change_type.to_string().cmp(&info_b.change_type.to_string()),
                        (Some(_), None) => std::cmp::Ordering::Less,
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        (None, None) => a.name.cmp(&b.name),
                    }
                });
            }
            SortOrder::ByFileSize => {
                node.children.sort_by(|a, b| {
                    match (&a.change_info, &b.change_info) {
                        (Some(info_a), Some(info_b)) => info_b.file_size.cmp(&info_a.file_size), // Descending
                        (Some(_), None) => std::cmp::Ordering::Less,
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        (None, None) => a.name.cmp(&b.name),
                    }
                });
            }
            SortOrder::ByPath => {
                node.children.sort_by(|a, b| a.path.cmp(&b.path));
            }
        }
    }

    fn limit_tree_depth(&self, node: &mut FileTreeNode, max_depth: u32) {
        if node.depth >= max_depth {
            node.children.clear();
            return;
        }

        for child in &mut node.children {
            self.limit_tree_depth(child, max_depth);
        }
    }

    fn search_recursive<'a>(
        &self,
        node: &'a FileTreeNode,
        criteria: &FileSearchCriteria,
        results: &mut Vec<&'a FileTreeNode>,
    ) {
        if self.matches_criteria(node, criteria) {
            results.push(node);
        }

        for child in &node.children {
            self.search_recursive(child, criteria, results);
        }
    }

    fn matches_criteria(&self, node: &FileTreeNode, criteria: &FileSearchCriteria) -> bool {
        // Only match files, not directories
        if node.node_type != NodeType::File {
            return false;
        }

        // Check name pattern
        if let Some(pattern) = &criteria.name_pattern {
            if !node.name.contains(pattern) {
                return false;
            }
        }

        // Check path pattern
        if let Some(pattern) = &criteria.path_pattern {
            if !node.path.contains(pattern) {
                return false;
            }
        }

        // Check change info criteria
        if let Some(change_info) = &node.change_info {
            // Check change types
            if !criteria.change_types.is_empty() && !criteria.change_types.contains(&change_info.change_type) {
                return false;
            }

            // Check review statuses
            if !criteria.review_statuses.is_empty() && !criteria.review_statuses.contains(&change_info.review_status) {
                return false;
            }

            // Check has comments
            if let Some(has_comments) = criteria.has_comments {
                if change_info.has_comments != has_comments {
                    return false;
                }
            }

            // Check is binary
            if let Some(is_binary) = criteria.is_binary {
                if change_info.is_binary != is_binary {
                    return false;
                }
            }

            // Check file size range
            if let Some(min_size) = criteria.min_size {
                if change_info.file_size < min_size {
                    return false;
                }
            }

            if let Some(max_size) = criteria.max_size {
                if change_info.file_size > max_size {
                    return false;
                }
            }
        }

        true
    }

    fn filter_node_recursive(
        &self,
        node: &FileTreeNode,
        criteria: &FileSearchCriteria,
    ) -> Result<FileTreeNode, HyperReviewError> {
        let mut filtered_node = node.clone();
        filtered_node.children.clear();

        for child in &node.children {
            if child.node_type == NodeType::Directory {
                // For directories, include if they have matching children
                let filtered_child = self.filter_node_recursive(child, criteria)?;
                if !filtered_child.children.is_empty() {
                    filtered_node.children.push(filtered_child);
                }
            } else if self.matches_criteria(child, criteria) {
                // For files, include if they match criteria
                filtered_node.children.push(child.clone());
            }
        }

        Ok(filtered_node)
    }

    fn calculate_stats_recursive(&self, node: &FileTreeNode, stats: &mut FileTreeStats) {
        match node.node_type {
            NodeType::Directory => {
                stats.total_directories += 1;
            }
            NodeType::File => {
                stats.total_files += 1;
                
                if let Some(change_info) = &node.change_info {
                    *stats.files_by_type.entry(change_info.change_type.clone()).or_insert(0) += 1;
                    *stats.files_by_status.entry(change_info.review_status.clone()).or_insert(0) += 1;
                    
                    stats.total_lines_added += change_info.lines_added;
                    stats.total_lines_removed += change_info.lines_removed;
                    
                    if change_info.is_binary {
                        stats.binary_files += 1;
                    }
                }
            }
        }

        for child in &node.children {
            self.calculate_stats_recursive(child, stats);
        }
    }

    fn find_node_mut<'a>(&self, node: &'a mut FileTreeNode, path: &str) -> Option<&'a mut FileTreeNode> {
        if node.path == path {
            return Some(node);
        }

        for child in &mut node.children {
            if let Some(found) = self.find_node_mut(child, path) {
                return Some(found);
            }
        }

        None
    }

    fn collect_visible_nodes<'a>(
        &self,
        node: &'a FileTreeNode,
        visible: &mut Vec<&'a FileTreeNode>,
        include_self: bool,
    ) {
        if include_self {
            visible.push(node);
        }

        if node.is_expanded {
            for child in &node.children {
                self.collect_visible_nodes(child, visible, true);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::gerrit::FileDiff;

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
            ChangeFile {
                id: "file3".to_string(),
                change_id: "change1".to_string(),
                patch_set_number: 1,
                file_path: "tests/integration_test.rs".to_string(),
                change_type: FileChangeType::Added,
                old_content: None,
                new_content: Some("test content".to_string()),
                diff: FileDiff::default(),
                file_size: 150,
                downloaded_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            },
        ]
    }

    #[test]
    fn test_file_tree_service_creation() {
        let config = FileTreeConfig::default();
        let service = FileTreeService::new(config);
        assert_eq!(service.config.sort_order, SortOrder::Alphabetical);
        assert!(service.config.group_by_directory);
    }

    #[test]
    fn test_build_tree() {
        let service = FileTreeService::new(FileTreeConfig::default());
        let files = create_test_files();

        let tree = service.build_tree(&files).unwrap();
        
        assert_eq!(tree.name, "Root");
        assert_eq!(tree.node_type, NodeType::Directory);
        assert_eq!(tree.children.len(), 2); // src and tests directories
        
        // Check src directory
        let src_dir = tree.children.iter().find(|child| child.name == "src").unwrap();
        assert_eq!(src_dir.node_type, NodeType::Directory);
        assert_eq!(src_dir.children.len(), 2); // main.rs and lib.rs
        
        // Check tests directory
        let tests_dir = tree.children.iter().find(|child| child.name == "tests").unwrap();
        assert_eq!(tests_dir.node_type, NodeType::Directory);
        assert_eq!(tests_dir.children.len(), 1); // integration_test.rs
    }

    #[test]
    fn test_tree_statistics() {
        let service = FileTreeService::new(FileTreeConfig::default());
        let files = create_test_files();
        let tree = service.build_tree(&files).unwrap();

        let stats = service.get_tree_stats(&tree);
        
        assert_eq!(stats.total_files, 3);
        assert_eq!(stats.total_directories, 3); // root, src, tests
        assert_eq!(stats.files_by_type.get(&FileChangeType::Modified), Some(&1));
        assert_eq!(stats.files_by_type.get(&FileChangeType::Added), Some(&2));
    }

    #[test]
    fn test_search_files() {
        let service = FileTreeService::new(FileTreeConfig::default());
        let files = create_test_files();
        let tree = service.build_tree(&files).unwrap();

        let criteria = FileSearchCriteria {
            name_pattern: Some("main".to_string()),
            path_pattern: None,
            change_types: vec![],
            review_statuses: vec![],
            has_comments: None,
            is_binary: None,
            min_size: None,
            max_size: None,
        };

        let results = service.search_files(&tree, &criteria);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "main.rs");
    }

    #[test]
    fn test_visible_nodes() {
        let mut config = FileTreeConfig::default();
        config.expand_all = true;
        let service = FileTreeService::new(config);
        let files = create_test_files();
        let tree = service.build_tree(&files).unwrap();

        let visible = service.get_visible_nodes(&tree);
        // Should include root + 2 dirs + 3 files = 6 nodes
        assert_eq!(visible.len(), 6);
    }
}