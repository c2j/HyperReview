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
}
