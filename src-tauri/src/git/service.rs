// Git service module
// Repository operations and Git command implementation

use crate::models::{Repo, Branch};
use crate::errors::HyperReviewError;
use git2::Repository;
use std::sync::Mutex;
use chrono::{Utc, TimeZone};

pub struct GitService {
    repository: Mutex<Option<Repository>>,
    current_path: Mutex<Option<String>>,
}

impl GitService {
    pub fn new() -> Self {
        Self {
            repository: Mutex::new(None),
            current_path: Mutex::new(None),
        }
    }

    /// Open a repository at the given path
    /// Returns repository metadata
    pub fn open_repo(&self, path: &str) -> Result<Repo, HyperReviewError> {
        log::info!("Opening repository at path: {}", path);

        // Verify the path exists and is a valid Git repository
        let repo = Repository::open(path)
            .map_err(HyperReviewError::Git)?;

        // Extract all information needed before moving repo into state

        // Get current branch and commit
        let (branch_name, head_oid) = {
            let head = repo.head()?;
            let branch_name = head.shorthand()
                .unwrap_or("unknown")
                .to_string();
            let head_commit = head.peel_to_commit()?;
            let head_oid = head_commit.id().to_string();
            (branch_name, head_oid)
        }; // head goes out of scope here, releasing the borrow

        // Create Repo object
        let repo_obj = Repo {
            path: path.to_string(),
            current_branch: branch_name.clone(),
            last_opened: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            head_commit: head_oid,
            remote_url: None, // TODO: Implement remote URL retrieval
            is_active: true,
        };

        // Update internal state
        {
            let mut repo_guard = self.repository.lock().unwrap();
            *repo_guard = Some(repo);
        }
        {
            let mut path_guard = self.current_path.lock().unwrap();
            *path_guard = Some(path.to_string());
        }

        log::info!("Successfully opened repository: {} (branch: {})", path, branch_name);

        Ok(repo_obj)
    }

    /// Check if a repository is currently loaded
    pub fn is_repo_loaded(&self) -> bool {
        self.repository.lock().unwrap().is_some()
    }

    /// Get the current repository path
    pub fn get_current_path(&self) -> Option<String> {
        self.current_path.lock().unwrap().clone()
    }

    /// Get the repository by reopening from path
    /// Note: git2::Repository doesn't implement Clone, so we reopen from stored path
    pub fn get_repository(&self) -> Option<Repository> {
        let path_guard = self.current_path.lock().unwrap();
        if let Some(ref path) = *path_guard {
            Repository::open(path).ok()
        } else {
            None
        }
    }

    /// Get all branches (local and remote) for the current repository
    pub fn get_branches(&self) -> Result<Vec<Branch>, HyperReviewError> {
        let repo_guard = self.repository.lock().unwrap();
        let repo = repo_guard.as_ref()
            .ok_or_else(|| HyperReviewError::Other {
                message: "No repository loaded".to_string(),
            })?;

        let mut branches = Vec::new();
        let head = repo.head()?;
        let head_name = head.name();
        let head_oid = head.peel_to_commit()?.id();

        // Collect local branches
        repo.branches(Some(git2::BranchType::Local))?
            .for_each(|branch_result| {
                if let Ok((git_branch, _)) = branch_result {
                    let branch_name = git_branch.name()
                        .unwrap_or(Some("unknown"))
                        .unwrap_or("unknown")
                        .to_string();

                    let is_current = head_name
                        .map(|h| h == format!("refs/heads/{}", branch_name))
                        .unwrap_or(false);

                    if let Ok(commit) = git_branch.get().peel_to_commit() {
                        branches.push(Branch {
                            name: branch_name,
                            is_current,
                            is_remote: false,
                            upstream: git_branch.upstream()
                                .ok()
                                .and_then(|u| u.get().name().map(|n| n.to_string())),
                            last_commit: commit.id().to_string(),
                            last_commit_message: commit.message()
                                .unwrap_or("")
                                .to_string()
                                .chars()
                                .take(140)
                                .collect(),
                            last_commit_author: commit.author().name()
                                .unwrap_or("unknown")
                                .to_string(),
                            last_commit_date: {
                                let time = commit.author().when();
                                Utc.timestamp_opt(time.seconds(), 0)
                                    .single()
                                    .unwrap_or(Utc::now())
                                    .to_rfc3339()
                            },
                        });
                    }
                }
            });

        // Collect remote branches
        repo.branches(Some(git2::BranchType::Remote))?
            .for_each(|branch_result| {
                if let Ok((git_branch, _)) = branch_result {
                    let branch_name = git_branch.name()
                        .unwrap_or(Some("unknown"))
                        .unwrap_or("unknown")
                        .to_string();

                    // Skip the HEAD ref from remotes
                    if branch_name.ends_with("/HEAD") {
                        return;
                    }

                    if let Ok(commit) = git_branch.get().peel_to_commit() {
                        branches.push(Branch {
                            name: branch_name,
                            is_current: false,
                            is_remote: true,
                            upstream: None,
                            last_commit: commit.id().to_string(),
                            last_commit_message: commit.message()
                                .unwrap_or("")
                                .to_string()
                                .chars()
                                .take(140)
                                .collect(),
                            last_commit_author: commit.author().name()
                                .unwrap_or("unknown")
                                .to_string(),
                            last_commit_date: {
                                let time = commit.author().when();
                                Utc.timestamp_opt(time.seconds(), 0)
                                    .single()
                                    .unwrap_or(Utc::now())
                                    .to_rfc3339()
                            },
                        });
                    }
                }
            });

        // Sort branches: current first, then by name
        branches.sort_by(|a, b| {
            if a.is_current && !b.is_current {
                std::cmp::Ordering::Less
            } else if !a.is_current && b.is_current {
                std::cmp::Ordering::Greater
            } else {
                a.name.cmp(&b.name)
            }
        });

        log::info!("Found {} branches", branches.len());
        Ok(branches)
    }

    /// Get blame information for a file
    pub fn get_blame(&self, file_path: &str, commit: Option<&str>) -> Result<crate::models::BlameInfo, HyperReviewError> {
        let repo_guard = self.repository.lock().unwrap();
        let repo = repo_guard.as_ref()
            .ok_or_else(|| HyperReviewError::Other {
                message: "No repository loaded".to_string(),
            })?;

        log::info!("Getting blame for file: {}", file_path);

        // Get the commit to blame from
        let spec = commit.unwrap_or("HEAD");
        let obj = repo.revparse_single(spec)?;
        let commit_obj = obj.peel_to_commit()?;

        // Create blame options
        let mut opts = git2::BlameOptions::new();
        opts.newest_commit(commit_obj.id());

        // Get blame
        let blame = repo.blame_file(std::path::Path::new(file_path), Some(&mut opts))?;

        // Convert to BlameLine structs
        let mut lines = Vec::new();
        for (line_number, hunk) in blame.iter().enumerate() {
            let sig = hunk.final_signature();
            let commit_id = hunk.final_commit_id();

            // Get commit details
            let (commit_message, committer_name, committer_email, commit_date) = if let Ok(c) = repo.find_commit(commit_id) {
                (
                    c.message().unwrap_or("").lines().next().unwrap_or("").to_string(),
                    c.committer().name().unwrap_or("unknown").to_string(),
                    c.committer().email().unwrap_or("unknown").to_string(),
                    {
                        let time = c.time();
                        Utc.timestamp_opt(time.seconds(), 0)
                            .single()
                            .unwrap_or(Utc::now())
                            .to_rfc3339()
                    },
                )
            } else {
                ("".to_string(), "unknown".to_string(), "unknown".to_string(), Utc::now().to_rfc3339())
            };

            lines.push(crate::models::BlameLine {
                line_number: (line_number + 1) as u32,
                content: String::new(), // Content would need to be read from file
                commit_oid: commit_id.to_string(),
                commit_message,
                author_name: sig.name().unwrap_or("unknown").to_string(),
                author_email: sig.email().unwrap_or("unknown").to_string(),
                committer_name,
                committer_email,
                commit_date,
            });
        }

        Ok(crate::models::BlameInfo {
            file_path: file_path.to_string(),
            lines,
        })
    }

    /// Get file tree for current repository
    pub fn get_file_tree(&self) -> Result<Vec<crate::models::FileNode>, HyperReviewError> {
        self.get_file_tree_with_branches(None, None)
    }

    /// Get file tree for current repository with specific branch comparison
    /// If branches are provided, compares those branches instead of HEAD vs parent
    pub fn get_file_tree_with_branches(
        &self,
        base_branch: Option<&str>,
        head_branch: Option<&str>,
    ) -> Result<Vec<crate::models::FileNode>, HyperReviewError> {
        let repo_guard = self.repository.lock().unwrap();
        let repo = repo_guard.as_ref()
            .ok_or_else(|| HyperReviewError::Other {
                message: "No repository loaded".to_string(),
            })?;

        log::info!("Getting file tree with base: {:?}, head: {:?}", base_branch, head_branch);

        // Resolve the base and head commits/branches
        let base_tree_ref: Option<git2::Tree>;
        let head_tree_ref: Option<git2::Tree>;

        if let (Some(base_ref), Some(head_ref)) = (base_branch, head_branch) {
            // Both branches specified - compare them
            log::info!("Comparing two branches: {} vs {}", base_ref, head_ref);

            let base_obj = repo.revparse_single(base_ref)?;
            let base_commit = base_obj.peel_to_commit()?;
            let base_tree = base_commit.tree()?;

            let head_obj = repo.revparse_single(head_ref)?;
            let head_commit = head_obj.peel_to_commit()?;
            let head_tree = head_commit.tree()?;

            base_tree_ref = Some(base_tree);
            head_tree_ref = Some(head_tree);
        } else if let Some(head_ref) = head_branch {
            // Only head specified - compare with its parent
            log::info!("Using head branch with parent: {}", head_ref);

            let head_obj = repo.revparse_single(head_ref)?;
            let head_commit = head_obj.peel_to_commit()?;
            let head_tree = head_commit.tree()?;

            let parent = head_commit.parent(0).ok();
            let parent_tree_opt = parent.and_then(|c| c.tree().ok());

            base_tree_ref = parent_tree_opt;
            head_tree_ref = Some(head_tree);
        } else {
            // No branches specified - use HEAD vs parent (default behavior)
            log::info!("Using default HEAD vs parent");

            let head = repo.head()?;
            let head_commit = head.peel_to_commit()?;
            let head_tree = head_commit.tree()?;

            let parent = head_commit.parent(0).ok();
            let parent_tree_opt = parent.and_then(|c| c.tree().ok());

            base_tree_ref = parent_tree_opt;
            head_tree_ref = Some(head_tree);
        };

        // Build file tree using the head tree, and add deleted files from diff
        let mut file_nodes = Vec::new();
        let mut tracked_paths: std::collections::HashSet<String> = std::collections::HashSet::new();

        // First, traverse the head tree to get all current files
        if let Some(head_tree) = head_tree_ref.as_ref() {
            self.build_tree_from_git_tree(&repo, head_tree, base_tree_ref.as_ref(), "", &mut file_nodes, &mut tracked_paths)?;
        }

        // Then, add files that exist in base but were deleted in head
        if let (Some(base_tree), Some(head_tree)) = (base_tree_ref.as_ref(), head_tree_ref.as_ref()) {
            self.add_deleted_files(&repo, base_tree, head_tree, "", &mut file_nodes, &mut tracked_paths)?;
        }

        log::info!("File tree built with {} top-level items", file_nodes.len());
        Ok(file_nodes)
    }

    /// Add files that exist in base_tree but not in head_tree (deleted files)
    fn add_deleted_files(
        &self,
        repo: &Repository,
        base_tree: &git2::Tree,
        head_tree: &git2::Tree,
        prefix: &str,
        nodes: &mut Vec<crate::models::FileNode>,
        tracked_paths: &mut std::collections::HashSet<String>,
    ) -> Result<(), HyperReviewError> {
        for entry in base_tree.iter() {
            let name = entry.name().unwrap_or("unknown");
            let full_path = if prefix.is_empty() {
                name.to_string()
            } else {
                format!("{}/{}", prefix, name)
            };

            match entry.kind() {
                Some(git2::ObjectType::Tree) => {
                    // It's a directory - recurse
                    let child_base_tree = repo.find_tree(entry.id())?;
                    // Check if corresponding directory exists in head
                    if let Ok(head_entry) = head_tree.get_path(std::path::Path::new(&full_path)) {
                        if let Some(head_child_tree) = repo.find_tree(head_entry.id()).ok() {
                            self.add_deleted_files(repo, &child_base_tree, &head_child_tree, &full_path, nodes, tracked_paths)?;
                        }
                    } else {
                        // Directory was deleted - add all its files as deleted
                        self.add_all_files_as_deleted(repo, &child_base_tree, &full_path, nodes, tracked_paths)?;
                    }
                }
                _ => {
                    // It's a file
                    if tracked_paths.contains(&full_path) {
                        // File already exists in head, skip
                        log::debug!("File in base already tracked in head (skipping): {}", full_path);
                        continue;
                    }

                    // File exists in base but not in tracked_paths - check if it actually exists in head
                    let exists_in_head = head_tree.get_path(std::path::Path::new(&full_path)).is_ok();
                    if exists_in_head {
                        log::warn!("BUG DETECTED: File {} exists in both trees but was not tracked! This indicates a bug in the first pass.", full_path);
                        // Don't mark as deleted - this is a bug
                        continue;
                    }

                    // File exists in base but not in head - it was deleted
                    log::debug!("File DELETED (only in base): {}", full_path);
                    // Get line count for deleted files
                    let lines = if let Ok(blob) = repo.find_blob(entry.id()) {
                        let content = std::str::from_utf8(blob.content()).unwrap_or("");
                        content.lines().count() as u32
                    } else {
                        0
                    };

                    let path_for_id = full_path.replace('/', "_");

                    nodes.push(crate::models::FileNode {
                        id: format!("file-deleted-{}-{}", entry.id(), path_for_id),
                        name: name.to_string(),
                        path: full_path,
                        file_type: "file".to_string(),
                        status: "deleted".to_string(),
                        children: None,
                        stats: Some(crate::models::FileStats {
                            added: 0,
                            removed: lines,
                        }),
                        exists: false,
                    });
                }
            }
        }

        Ok(())
    }

    /// Add all files in a tree as deleted (recursively)
    fn add_all_files_as_deleted(
        &self,
        repo: &Repository,
        tree: &git2::Tree,
        base_path: &str,
        nodes: &mut Vec<crate::models::FileNode>,
        tracked_paths: &mut std::collections::HashSet<String>,
    ) -> Result<(), HyperReviewError> {
        for entry in tree.iter() {
            let name = entry.name().unwrap_or("unknown");
            let full_path = if base_path.is_empty() {
                name.to_string()
            } else {
                format!("{}/{}", base_path, name)
            };

            if tracked_paths.contains(&full_path) {
                continue;
            }

            match entry.kind() {
                Some(git2::ObjectType::Tree) => {
                    // Directory - recurse
                    let child_tree = repo.find_tree(entry.id())?;
                    self.add_all_files_as_deleted(repo, &child_tree, &full_path, nodes, tracked_paths)?;
                }
                _ => {
                    // File - add as deleted
                    let lines = if let Ok(blob) = repo.find_blob(entry.id()) {
                        let content = std::str::from_utf8(blob.content()).unwrap_or("");
                        content.lines().count() as u32
                    } else {
                        0
                    };

                    let path_for_id = full_path.replace('/', "_");

                    nodes.push(crate::models::FileNode {
                        id: format!("file-deleted-{}-{}", entry.id(), path_for_id),
                        name: name.to_string(),
                        path: full_path,
                        file_type: "file".to_string(),
                        status: "deleted".to_string(),
                        children: None,
                        stats: Some(crate::models::FileStats {
                            added: 0,
                            removed: lines,
                        }),
                        exists: false,
                    });
                }
            }
        }

        Ok(())
    }

    /// Recursively build file tree from git tree with branch comparison
    fn build_tree_from_git_tree(
        &self,
        repo: &Repository,
        tree: &git2::Tree,
        base_tree: Option<&git2::Tree>,
        prefix: &str,
        nodes: &mut Vec<crate::models::FileNode>,
        tracked_paths: &mut std::collections::HashSet<String>,
    ) -> Result<(), HyperReviewError> {
        for entry in tree.iter() {
            let name = entry.name().unwrap_or("unknown");
            let full_path = if prefix.is_empty() {
                name.to_string()
            } else {
                format!("{}/{}", prefix, name)
            };

            // Determine file type
            let (file_type, children) = match entry.kind() {
                Some(git2::ObjectType::Tree) => {
                    // It's a directory
                    let child_tree = repo.find_tree(entry.id())?;
                    // Find corresponding directory in base tree for proper comparison
                    let child_base_tree = if let Some(base) = base_tree {
                        base.get_path(std::path::Path::new(&full_path))
                            .ok()
                            .and_then(|e| repo.find_tree(e.id()).ok())
                    } else {
                        None
                    };
                    let mut child_nodes = Vec::new();
                    self.build_tree_from_git_tree(
                        repo,
                        &child_tree,
                        child_base_tree.as_ref(),
                        &full_path,
                        &mut child_nodes,
                        tracked_paths,
                    )?;
                    ("folder".to_string(), Some(child_nodes))
                }
                _ => {
                    // It's a file
                    ("file".to_string(), None)
                }
            };

            // Determine file status and stats by comparing with base tree
            let (status, stats) = if let Some(base) = base_tree {
                if let Ok(base_entry) = base.get_path(std::path::Path::new(&full_path)) {
                    // File exists in both commits - calculate diff
                    if base_entry.id() == entry.id() {
                        // File unchanged - same blob ID means identical content
                        log::debug!("File unchanged (same blob): {} - base_id: {}, head_id: {}", full_path, base_entry.id(), entry.id());
                        ("none".to_string(), None)
                    } else {
                        // File modified - different blob IDs
                        log::debug!("File MODIFIED (different blob): {} - base_id: {}, head_id: {}", full_path, base_entry.id(), entry.id());
                        
                        // For modified files, provide basic stats based on file size change
                        let old_size = if let Ok(blob) = repo.find_blob(base_entry.id()) {
                            blob.content().len() as u64
                        } else { 0 };
                        let new_size = if let Ok(blob) = repo.find_blob(entry.id()) {
                            blob.content().len() as u64
                        } else { 0 };
                        let size_change = if new_size > old_size { 
                            ((new_size - old_size) / 50).min(100) as u32 // Estimated added lines
                        } else { 
                            ((old_size - new_size) / 50).min(100) as u32 // Estimated removed lines  
                        };
                        
                        ("modified".to_string(), Some(crate::models::FileStats {
                            added: size_change,
                            removed: size_change,
                        }))
                    }
                } else {
                    // File is new (only exists in head)
                    log::debug!("File ADDED (only in head): {}", full_path);
                    let lines = if let Ok(blob) = repo.find_blob(entry.id()) {
                        let content = std::str::from_utf8(blob.content()).unwrap_or("");
                        content.lines().count() as u32
                    } else {
                        0
                    };

                    ("added".to_string(), Some(crate::models::FileStats {
                        added: lines,
                        removed: 0,
                    }))
                }
            } else {
                // No base tree - can't determine status
                log::debug!("No base tree for comparison: {}", full_path);
                ("none".to_string(), None)
            };

            // Check if file exists in the working directory
            let repo_path = repo.workdir().unwrap_or_else(|| std::path::Path::new("."));
            let full_file_path = repo_path.join(&full_path);
            let file_exists = full_file_path.exists() && full_file_path.is_file();

            let path_for_id = full_path.replace('/', "_");

            // Track this path so we don't add it again in deleted files pass
            tracked_paths.insert(full_path.clone());

            // Log before moving values
            log::debug!("Tracked file in head pass: {} (status: {}, stats: {:?})", full_path, status, stats);

            nodes.push(crate::models::FileNode {
                id: format!("file-{}-{}", entry.id(), path_for_id),
                name: name.to_string(),
                path: full_path,
                file_type,
                status,
                children,
                stats,
                exists: file_exists,
            });
        }

        Ok(())
    }
}
