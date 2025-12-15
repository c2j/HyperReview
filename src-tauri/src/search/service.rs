// Search service with ripgrep integration
// Fast repository search

use crate::models::{SearchResult, SearchResultType};
use std::process::Command;
use std::path::Path;

pub struct SearchService {
    repo_path: Option<String>,
}

impl SearchService {
    pub fn new() -> Self {
        Self {
            repo_path: None,
        }
    }

    pub fn with_repo(mut self, path: String) -> Self {
        self.repo_path = Some(path);
        self
    }

    /// Search for text in repository files
    pub fn search(&self, query: &str, file_pattern: Option<&str>) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
        let repo_path = self.repo_path.as_ref()
            .ok_or("Repository path not set")?;

        log::info!("Searching for '{}' in {}", query, repo_path);

        // Try to use ripgrep if available, fall back to simple search
        if let Ok(results) = self.search_with_ripgrep(query, repo_path, file_pattern) {
            return Ok(results);
        }

        // Fallback to simple recursive search
        self.search_simple(query, repo_path)
    }

    /// Search using ripgrep (rg) for better performance
    fn search_with_ripgrep(&self, query: &str, path: &str, file_pattern: Option<&str>) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
        let mut cmd = Command::new("rg");
        cmd.arg("--json")
           .arg("--line-number")
           .arg("--max-count").arg("100")
           .arg(query)
           .arg(path);

        if let Some(pattern) = file_pattern {
            cmd.arg("--glob").arg(pattern);
        }

        let output = cmd.output()?;

        if !output.status.success() && output.stdout.is_empty() {
            return Err("ripgrep search failed or no results".into());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut results = Vec::new();
        let mut score = 1.0;

        for line in stdout.lines() {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                if json["type"] == "match" {
                    if let Some(data) = json.get("data") {
                        let file_path = data["path"]["text"].as_str().map(|s| s.to_string());
                        let line_number = data["line_number"].as_u64().map(|n| n as u32);
                        let content = data["lines"]["text"].as_str()
                            .unwrap_or("")
                            .trim()
                            .to_string();

                        results.push(SearchResult {
                            result_type: SearchResultType::File,
                            file_path,
                            line_number,
                            content,
                            highlight: Some(query.to_string()),
                            score,
                        });

                        score *= 0.99; // Decrease score for subsequent results
                    }
                }
            }
        }

        Ok(results)
    }

    /// Simple fallback search without ripgrep
    fn search_simple(&self, query: &str, path: &str) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
        let mut results = Vec::new();
        let query_lower = query.to_lowercase();

        self.search_directory(Path::new(path), &query_lower, &mut results)?;

        Ok(results)
    }

    fn search_directory(&self, dir: &Path, query: &str, results: &mut Vec<SearchResult>) -> Result<(), Box<dyn std::error::Error>> {
        if results.len() >= 100 {
            return Ok(()); // Limit results
        }

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            // Skip hidden files and common ignore patterns
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with('.') || name == "node_modules" || name == "target" || name == "build" {
                    continue;
                }
            }

            if path.is_dir() {
                self.search_directory(&path, query, results)?;
            } else if path.is_file() {
                // Only search text files
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if matches!(ext, "rs" | "ts" | "js" | "tsx" | "jsx" | "json" | "toml" | "yaml" | "yml" | "md" | "txt" | "html" | "css") {
                        if let Ok(content) = std::fs::read_to_string(&path) {
                            for (line_num, line) in content.lines().enumerate() {
                                if line.to_lowercase().contains(query) {
                                    results.push(SearchResult {
                                        result_type: SearchResultType::File,
                                        file_path: Some(path.to_string_lossy().to_string()),
                                        line_number: Some((line_num + 1) as u32),
                                        content: line.trim().to_string(),
                                        highlight: Some(query.to_string()),
                                        score: 0.8,
                                    });

                                    if results.len() >= 100 {
                                        return Ok(());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Search for files by name
    pub fn search_files(&self, pattern: &str) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
        let repo_path = self.repo_path.as_ref()
            .ok_or("Repository path not set")?;

        let mut results = Vec::new();
        self.search_files_in_dir(Path::new(repo_path), pattern, &mut results)?;

        Ok(results)
    }

    fn search_files_in_dir(&self, dir: &Path, pattern: &str, results: &mut Vec<SearchResult>) -> Result<(), Box<dyn std::error::Error>> {
        if results.len() >= 50 {
            return Ok(());
        }

        let pattern_lower = pattern.to_lowercase();

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with('.') || name == "node_modules" || name == "target" {
                    continue;
                }

                if name.to_lowercase().contains(&pattern_lower) {
                    results.push(SearchResult {
                        result_type: SearchResultType::File,
                        file_path: Some(path.to_string_lossy().to_string()),
                        line_number: None,
                        content: name.to_string(),
                        highlight: Some(pattern.to_string()),
                        score: if name.to_lowercase() == pattern_lower { 1.0 } else { 0.7 },
                    });
                }
            }

            if path.is_dir() {
                self.search_files_in_dir(&path, pattern, results)?;
            }
        }

        Ok(())
    }

    /// Search commits
    pub fn search_commits(&self, query: &str) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
        let repo_path = self.repo_path.as_ref()
            .ok_or("Repository path not set")?;

        let repo = git2::Repository::open(repo_path)?;
        let mut results = Vec::new();

        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;

        let query_lower = query.to_lowercase();

        for oid in revwalk.take(100) {
            let oid = oid?;
            let commit = repo.find_commit(oid)?;

            let message = commit.message().unwrap_or("");
            if message.to_lowercase().contains(&query_lower) {
                results.push(SearchResult {
                    result_type: SearchResultType::Commit,
                    file_path: None,
                    line_number: None,
                    content: format!("{}: {}", &oid.to_string()[..7], message.lines().next().unwrap_or("")),
                    highlight: Some(query.to_string()),
                    score: 0.9,
                });

                if results.len() >= 20 {
                    break;
                }
            }
        }

        Ok(results)
    }
}

impl Default for SearchService {
    fn default() -> Self {
        Self::new()
    }
}
