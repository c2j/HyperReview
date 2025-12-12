//! Newtype ID wrappers for HyperReview
//!
//! Following Constitution III: All public APIs MUST use strong typing.

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::Error;

/// Repository ID - format: {provider}:{owner}/{name}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RepoId(String);

impl RepoId {
    /// Create a new RepoId
    pub fn new(provider: &str, owner: &str, name: &str) -> Self {
        Self(format!("{}:{}/{}", provider, owner, name))
    }

    /// Parse from string
    pub fn parse(s: &str) -> Result<Self, Error> {
        // Validate format: provider:owner/name
        if s.contains(':') && s.contains('/') {
            Ok(Self(s.to_string()))
        } else {
            Err(Error::InvalidId(format!(
                "Invalid RepoId format: {}. Expected: provider:owner/name",
                s
            )))
        }
    }

    /// Get the inner string value
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RepoId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Pull Request ID - format: {provider}:{owner}/{repo}#{number}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PrId(String);

impl PrId {
    /// Create a new PrId
    pub fn new(provider: &str, owner: &str, repo: &str, number: u32) -> Self {
        Self(format!("{}:{}/{}#{}", provider, owner, repo, number))
    }

    /// Create a new PrId from RepoId parts
    pub fn new_from_parts(repo_id: &RepoId, number: u32) -> Self {
        let repo_str = repo_id.as_str();
        Self(format!("{}#{}", repo_str, number))
    }

    /// Parse from string
    pub fn parse(s: &str) -> Result<Self, Error> {
        // Validate format: provider:owner/repo#number
        if s.contains(':') && s.contains('/') && s.contains('#') {
            Ok(Self(s.to_string()))
        } else {
            Err(Error::InvalidId(format!(
                "Invalid PrId format: {}. Expected: provider:owner/repo#number",
                s
            )))
        }
    }

    /// Get the inner string value
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PrId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Comment ID - local UUID or remote ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CommentId(String);

impl CommentId {
    /// Create a new local CommentId (UUID)
    pub fn new_local() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }

    /// Create from remote ID
    pub fn from_remote(id: &str) -> Self {
        Self(id.to_string())
    }

    /// Get the inner string value
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CommentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Review ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReviewId(String);

impl ReviewId {
    /// Create a new local ReviewId (UUID)
    pub fn new_local() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }

    /// Create from remote ID
    pub fn from_remote(id: &str) -> Self {
        Self(id.to_string())
    }

    /// Get the inner string value
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ReviewId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repo_id_creation() {
        let id = RepoId::new("github", "owner", "repo");
        assert_eq!(id.as_str(), "github:owner/repo");
    }

    #[test]
    fn test_pr_id_creation() {
        let id = PrId::new("github", "owner", "repo", 123);
        assert_eq!(id.as_str(), "github:owner/repo#123");
    }

    #[test]
    fn test_repo_id_parse_valid() {
        let id = RepoId::parse("github:owner/repo").unwrap();
        assert_eq!(id.as_str(), "github:owner/repo");
    }

    #[test]
    fn test_repo_id_parse_invalid() {
        let result = RepoId::parse("invalid");
        assert!(result.is_err());
    }
}
