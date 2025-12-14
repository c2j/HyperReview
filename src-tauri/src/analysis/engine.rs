// Static analysis engine
// Code pattern matching and issue detection

use crate::models::{DiffLine, Severity};
use std::collections::HashMap;
use regex::Regex;

pub struct AnalysisEngine {
    /// Regex patterns for detecting potential issues
    security_patterns: Vec<Regex>,
    style_patterns: Vec<Regex>,
    todo_patterns: Vec<Regex>,
}

impl AnalysisEngine {
    pub fn new() -> Self {
        let mut security_patterns = Vec::new();
        let mut style_patterns = Vec::new();
        let mut todo_patterns = Vec::new();

        // Security-related patterns
        if let Ok(regex) = Regex::new(r"(?i)(password|passwd|pwd|secret|api_key|apikey|token)") {
            security_patterns.push(regex);
        }
        if let Ok(regex) = Regex::new(r"(?i)(eval|exec|system|shell_exec|passthru)") {
            security_patterns.push(regex);
        }
        if let Ok(regex) = Regex::new(r"(?i)(hardcoded|hard-code|credential|key)") {
            security_patterns.push(regex);
        }

        // Style and code quality patterns
        if let Ok(regex) = Regex::new(r"// TODO") {
            todo_patterns.push(regex);
        }
        if let Ok(regex) = Regex::new(r"// FIXME") {
            todo_patterns.push(regex);
        }
        if let Ok(regex) = Regex::new(r"// XXX") {
            todo_patterns.push(regex);
        }

        // Console.log, print, etc.
        if let Ok(regex) = Regex::new(r"(?i)(console\.log|print\(|\.print\(|\.puts\()") {
            style_patterns.push(regex);
        }

        // Long lines (> 120 characters)
        if let Ok(regex) = Regex::new(r".{121,}") {
            style_patterns.push(regex);
        }

        Self {
            security_patterns,
            style_patterns,
            todo_patterns,
        }
    }

    /// Analyze diff lines for potential issues
    /// Returns vector of analyzed diff lines with severity and messages
    pub fn analyze_diff_lines(
        &self,
        diff_lines: &mut [DiffLine],
        file_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Analyzing {} diff lines for {}", diff_lines.len(), file_path);

        // Get file extension for language-specific analysis
        let extension = std::path::Path::new(file_path)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        for line in diff_lines.iter_mut() {
            // Skip empty lines
            if line.content.trim().is_empty() {
                continue;
            }

            // Security analysis
            for pattern in &self.security_patterns {
                if pattern.is_match(&line.content) {
                    line.severity = Some(Severity::Warning);
                    line.message = Some("Potential security issue detected".to_string());
                    break;
                }
            }

            // Style analysis
            if line.severity.is_none() {
                for pattern in &self.style_patterns {
                    if pattern.is_match(&line.content) {
                        line.severity = Some(Severity::Info);
                        line.message = Some("Code style issue detected".to_string());
                        break;
                    }
                }
            }

            // TODO/FIXME analysis
            if line.severity.is_none() {
                for pattern in &self.todo_patterns {
                    if pattern.is_match(&line.content) {
                        line.severity = Some(Severity::Info);
                        line.message = Some("TODO/FIXME comment found".to_string());
                        break;
                    }
                }
            }

            // Language-specific analysis
            if line.severity.is_none() {
                self.analyze_language_specific(extension, line);
            }
        }

        log::info!("Analysis complete for {}", file_path);
        Ok(())
    }

    /// Language-specific static analysis
    fn analyze_language_specific(&self, extension: &str, line: &mut DiffLine) {
        match extension {
            "js" | "jsx" | "ts" | "tsx" => {
                // JavaScript/TypeScript specific checks
                if line.content.contains("var ") && !line.content.contains("// var is deprecated") {
                    line.severity = Some(Severity::Info);
                    line.message = Some("Use 'let' or 'const' instead of 'var'".to_string());
                } else if line.content.contains("==") && !line.content.contains("===") {
                    line.severity = Some(Severity::Info);
                    line.message = Some("Use strict equality (===) instead of == (double equals)".to_string());
                } else if line.content.contains("console.log") {
                    line.severity = Some(Severity::Info);
                    line.message = Some("Remove console.log before production".to_string());
                }
            }
            "py" => {
                // Python specific checks
                if line.content.contains("print(") && !line.content.contains("# print(") {
                    line.severity = Some(Severity::Info);
                    line.message = Some("Remove print() statements before production".to_string());
                } else if line.content.contains("except:") {
                    line.severity = Some(Severity::Warning);
                    line.message = Some("Bare except clause - specify exception type".to_string());
                }
            }
            "rs" => {
                // Rust specific checks
                if line.content.contains("unwrap()") && !line.content.contains("// unwrap") {
                    line.severity = Some(Severity::Warning);
                    line.message = Some("Use proper error handling instead of unwrap()".to_string());
                } else if line.content.contains("println!(\"{}\",") {
                    line.severity = Some(Severity::Info);
                    line.message = Some("Consider using format!() instead of println!() for strings".to_string());
                }
            }
            "java" => {
                // Java specific checks
                if line.content.contains("System.out.println") {
                    line.severity = Some(Severity::Info);
                    line.message = Some("Use proper logging framework instead of System.out.println".to_string());
                } else if line.content.contains("catch (Exception e)") {
                    line.severity = Some(Severity::Warning);
                    line.message = Some("Catch specific exceptions instead of generic Exception".to_string());
                }
            }
            _ => {
                // Default checks for unknown languages
                if line.content.len() > 120 {
                    line.severity = Some(Severity::Info);
                    line.message = Some(format!("Line exceeds 120 characters ({} chars)", line.content.len()));
                }
            }
        }
    }

    /// Get statistics about detected issues
    pub fn get_analysis_stats(&self, diff_lines: &[DiffLine]) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        stats.insert("total_lines".to_string(), diff_lines.len());

        let mut security_issues = 0;
        let mut style_issues = 0;
        let mut todo_comments = 0;

        for line in diff_lines {
            if let Some(ref message) = line.message {
                if message.contains("security") {
                    security_issues += 1;
                } else if message.contains("style") {
                    style_issues += 1;
                } else if message.contains("TODO") || message.contains("FIXME") {
                    todo_comments += 1;
                }
            }
        }

        stats.insert("security_issues".to_string(), security_issues);
        stats.insert("style_issues".to_string(), style_issues);
        stats.insert("todo_comments".to_string(), todo_comments);

        stats
    }
}

impl Default for AnalysisEngine {
    fn default() -> Self {
        Self::new()
    }
}

