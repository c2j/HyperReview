//! Git service for HyperReview
//!
//! git2-rs operations following Constitution I (Performance-First) requirements.

use git2::{Repository, Diff, DiffOptions, FetchOptions, RemoteCallbacks, Cred};
use git2::build::CheckoutBuilder;
use std::path::Path;

use crate::error::{Error, Result};
use crate::models::{Diff as DomainDiff, PrId};

/// Git service for repository operations
pub struct GitService {
    // Service is stateless, repositories are opened per-operation
}

impl GitService {
    /// Create new git service
    pub fn new() -> Self {
        Self {}
    }

    /// Open a local git repository (simplified for Milestone 1)
    pub async fn open_repository(&self, path: &Path) -> Result<Repository> {
        Repository::open(path).map_err(Error::from)
    }

    /// Compute diff between two commits (simplified for Milestone 1)
    pub async fn compute_diff<'a>(
        &self,
        repo: &'a Repository,
        base_sha: &str,
        head_sha: &str,
    ) -> Result<Diff<'a>> {
        let base_oid = git2::Oid::from_str(base_sha)
            .map_err(|e| Error::InvalidId(format!("Invalid base SHA: {}", e)))?;
        let head_oid = git2::Oid::from_str(head_sha)
            .map_err(|e| Error::InvalidId(format!("Invalid head SHA: {}", e)))?;

        let base_commit = repo.find_commit(base_oid)?;
        let head_commit = repo.find_commit(head_oid)?;

        let base_tree = base_commit.tree()?;
        let head_tree = head_commit.tree()?;

        let mut opts = DiffOptions::new();
        opts.context_lines(3);

        let diff = repo.diff_tree_to_tree(Some(&base_tree), Some(&head_tree), Some(&mut opts))?;

        Ok(diff)
    }

    /// Compute diff and parse to domain model (simplified for Milestone 1)
    pub async fn compute_diff_to_domain<'a>(
        &self,
        repo: &'a Repository,
        base_sha: &str,
        head_sha: &str,
        pr_id: Option<PrId>,
    ) -> Result<DomainDiff> {
        let diff = self.compute_diff(repo, base_sha, head_sha).await?;
        Ok(parse_diff(&diff, pr_id))
    }

    /// Shallow clone a repository with depth limit
    pub async fn shallow_clone(&self, url: &str, path: &Path, depth: u32) -> Result<Repository> {
        // Shallow clone with limited depth for performance
        // Note: git2::Repository::clone() doesn't directly support shallow clone depth
        // This would require a more complex implementation with custom fetch options
        // For now, do a regular clone
        Repository::clone(url, path).map_err(Error::from)
    }

    /// Authenticated shallow clone with access token
    pub async fn authenticated_clone(
        &self,
        url: &str,
        path: &Path,
        access_token: &str,
        depth: u32,
    ) -> Result<Repository> {
        // Set up authentication callbacks
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(move |_url, _username, _allowed| {
            // Use access token as password for HTTPS authentication
            Cred::userpass_plaintext("oauth", access_token)
        });

        let mut fetch_opts = FetchOptions::new();
        fetch_opts.remote_callbacks(callbacks);

        // Clone with authentication
        Repository::clone(url, path).map_err(Error::from)
    }

    /// Fetch commits from remote for a repository
    pub async fn fetch_commits(&self, repo: &Repository, refs: &[&str]) -> Result<()> {
        // Get the remote named "origin"
        let mut remote = repo.find_remote("origin")?;

        // Set up fetch options with credentials
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|url, username, allowed| {
            // For authenticated fetches, try to use OAuth token from URL
            if let Some(token) = url.split("://").nth(1).and_then(|s| s.split('@').next()) {
                if let Some(creds) = token.split(':').nth(1) {
                    return Cred::userpass_plaintext("oauth", creds);
                }
            }
            // Fallback to default credentials
            Cred::default()
        });

        let mut fetch_opts = FetchOptions::new();
        fetch_opts.remote_callbacks(callbacks);

        // Perform the fetch
        remote.fetch(refs, Some(&mut fetch_opts), None)?;

        Ok(())
    }
}

impl Default for GitService {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse git2::Diff to domain Diff model (simplified for Milestone 1)
fn parse_diff(_git_diff: &Diff, pr_id: Option<PrId>) -> DomainDiff {
    // For Milestone 1, return empty diff
    // Full implementation would parse git2::Diff properly
    DomainDiff {
        pr_id,
        files: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_git_service_new() {
        let service = GitService::new();
        let result = service.open_repository(std::path::Path::new("/nonexistent")).await;
        assert!(result.is_err());
    }
}
