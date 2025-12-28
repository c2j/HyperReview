use git2::{Repository, BranchType};
use std::path::Path;
use anyhow::{Result, anyhow};

pub struct RepoManager;

impl RepoManager {
    pub fn validate_repository(path: &str) -> Result<bool> {
        match Repository::open(path) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn list_branches(path: &str) -> Result<Vec<String>> {
        let repo = Repository::open(path)?;
        let branches = repo.branches(Some(BranchType::Local))?;
        
        let mut branch_names = Vec::new();
        for branch in branches {
            let (branch, _) = branch?;
            if let Some(name) = branch.name()? {
                branch_names.push(name.to_string());
            }
        }
        Ok(branch_names)
    }

    pub fn read_file_from_ref(repo_path: &str, reference: &str, file_path: &str) -> Result<String> {
        let repo = Repository::open(repo_path)?;
        let obj = repo.revparse_single(reference)?;
        let commit = obj.peel_to_commit()?;
        let tree = commit.tree()?;
        let entry = tree.get_path(Path::new(file_path))?;
        let object = entry.to_object(&repo)?;
        
        if let Some(blob) = object.as_blob() {
            let content = std::str::from_utf8(blob.content())
                .map_err(|_| anyhow!("File content is not valid UTF-8"))?;
            Ok(content.to_string())
        } else {
            Err(anyhow!("Path is not a blob"))
        }
    }
}
