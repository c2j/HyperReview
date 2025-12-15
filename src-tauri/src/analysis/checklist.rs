// Smart checklist engine
// Intelligent review assistance

use crate::models::{ChecklistItem, ChecklistCategory, Severity};
use std::collections::HashMap;

pub struct ChecklistEngine {
    templates: HashMap<String, Vec<ChecklistTemplate>>,
}

#[derive(Debug, Clone)]
struct ChecklistTemplate {
    description: String,
    category: ChecklistCategory,
    severity: Severity,
    file_patterns: Vec<String>,
    content_patterns: Vec<String>,
    is_auto_checkable: bool,
}

impl ChecklistEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            templates: HashMap::new(),
        };
        engine.load_default_templates();
        engine
    }

    /// Load default checklist templates
    fn load_default_templates(&mut self) {
        // Security templates
        self.templates.insert("security".to_string(), vec![
            ChecklistTemplate {
                description: "Check for hardcoded credentials or API keys".to_string(),
                category: ChecklistCategory::Security,
                severity: Severity::Error,
                file_patterns: vec!["*.rs".to_string(), "*.ts".to_string(), "*.js".to_string()],
                content_patterns: vec!["password".to_string(), "api_key".to_string(), "secret".to_string()],
                is_auto_checkable: true,
            },
            ChecklistTemplate {
                description: "Verify SQL queries use parameterized statements".to_string(),
                category: ChecklistCategory::Security,
                severity: Severity::Error,
                file_patterns: vec!["*.rs".to_string()],
                content_patterns: vec!["execute".to_string(), "query".to_string()],
                is_auto_checkable: true,
            },
            ChecklistTemplate {
                description: "Check for proper input validation".to_string(),
                category: ChecklistCategory::Security,
                severity: Severity::Warning,
                file_patterns: vec!["*.rs".to_string(), "*.ts".to_string()],
                content_patterns: vec!["user_input".to_string(), "request".to_string()],
                is_auto_checkable: false,
            },
        ]);

        // Performance templates
        self.templates.insert("performance".to_string(), vec![
            ChecklistTemplate {
                description: "Check for N+1 query patterns".to_string(),
                category: ChecklistCategory::Performance,
                severity: Severity::Warning,
                file_patterns: vec!["*.rs".to_string()],
                content_patterns: vec!["for".to_string(), "query".to_string()],
                is_auto_checkable: false,
            },
            ChecklistTemplate {
                description: "Verify async operations are properly handled".to_string(),
                category: ChecklistCategory::Performance,
                severity: Severity::Info,
                file_patterns: vec!["*.rs".to_string(), "*.ts".to_string()],
                content_patterns: vec!["async".to_string(), "await".to_string()],
                is_auto_checkable: false,
            },
        ]);

        // Style templates
        self.templates.insert("style".to_string(), vec![
            ChecklistTemplate {
                description: "Verify consistent naming conventions".to_string(),
                category: ChecklistCategory::Style,
                severity: Severity::Info,
                file_patterns: vec!["*.rs".to_string(), "*.ts".to_string()],
                content_patterns: vec![],
                is_auto_checkable: false,
            },
            ChecklistTemplate {
                description: "Check for proper error handling".to_string(),
                category: ChecklistCategory::Style,
                severity: Severity::Warning,
                file_patterns: vec!["*.rs".to_string()],
                content_patterns: vec!["unwrap".to_string(), "expect".to_string()],
                is_auto_checkable: true,
            },
        ]);

        // Architecture templates
        self.templates.insert("architecture".to_string(), vec![
            ChecklistTemplate {
                description: "Verify separation of concerns".to_string(),
                category: ChecklistCategory::Architecture,
                severity: Severity::Info,
                file_patterns: vec!["*.rs".to_string()],
                content_patterns: vec![],
                is_auto_checkable: false,
            },
            ChecklistTemplate {
                description: "Check for circular dependencies".to_string(),
                category: ChecklistCategory::Architecture,
                severity: Severity::Warning,
                file_patterns: vec!["*.rs".to_string()],
                content_patterns: vec!["use crate".to_string()],
                is_auto_checkable: false,
            },
        ]);

        // Testing templates
        self.templates.insert("testing".to_string(), vec![
            ChecklistTemplate {
                description: "Verify test coverage for new code".to_string(),
                category: ChecklistCategory::Testing,
                severity: Severity::Warning,
                file_patterns: vec!["*.rs".to_string()],
                content_patterns: vec!["pub fn".to_string()],
                is_auto_checkable: false,
            },
            ChecklistTemplate {
                description: "Check edge cases are tested".to_string(),
                category: ChecklistCategory::Testing,
                severity: Severity::Info,
                file_patterns: vec!["*_test.rs".to_string(), "*test*.rs".to_string()],
                content_patterns: vec!["#[test]".to_string()],
                is_auto_checkable: false,
            },
        ]);

        // Documentation templates
        self.templates.insert("documentation".to_string(), vec![
            ChecklistTemplate {
                description: "Verify public functions have doc comments".to_string(),
                category: ChecklistCategory::Documentation,
                severity: Severity::Info,
                file_patterns: vec!["*.rs".to_string()],
                content_patterns: vec!["pub fn".to_string()],
                is_auto_checkable: true,
            },
        ]);
    }

    /// Generate checklist for a specific file
    pub fn generate_checklist(&self, file_path: &str, content: Option<&str>) -> Vec<ChecklistItem> {
        let mut items = Vec::new();
        let mut item_id = 0;

        for (_, templates) in &self.templates {
            for template in templates {
                // Check if file matches any pattern
                let matches_file = template.file_patterns.is_empty() ||
                    template.file_patterns.iter().any(|pattern| {
                        Self::matches_glob(file_path, pattern)
                    });

                if !matches_file {
                    continue;
                }

                // Check if content matches any pattern (if content provided)
                let matches_content = template.content_patterns.is_empty() ||
                    content.map(|c| {
                        template.content_patterns.iter().any(|p| c.contains(p))
                    }).unwrap_or(true);

                if !matches_content {
                    continue;
                }

                item_id += 1;
                items.push(ChecklistItem {
                    id: format!("CL{:04}", item_id),
                    description: template.description.clone(),
                    category: template.category.clone(),
                    severity: template.severity.clone(),
                    applicable_file_types: template.file_patterns.clone(),
                    applicable_patterns: template.content_patterns.clone(),
                    is_checked: false,
                    is_auto_checkable: template.is_auto_checkable,
                    related_file: Some(file_path.to_string()),
                });
            }
        }

        items
    }

    /// Simple glob-style pattern matching
    fn matches_glob(path: &str, pattern: &str) -> bool {
        if pattern.starts_with('*') {
            let suffix = &pattern[1..];
            path.ends_with(suffix)
        } else if pattern.ends_with('*') {
            let prefix = &pattern[..pattern.len() - 1];
            path.starts_with(prefix)
        } else if pattern.contains('*') {
            // Simple wildcard match
            let parts: Vec<&str> = pattern.split('*').collect();
            if parts.len() == 2 {
                path.starts_with(parts[0]) && path.ends_with(parts[1])
            } else {
                path == pattern
            }
        } else {
            path == pattern
        }
    }

    /// Get checklist by category
    pub fn get_by_category(&self, category: &ChecklistCategory, file_path: &str) -> Vec<ChecklistItem> {
        let category_key = match category {
            ChecklistCategory::Security => "security",
            ChecklistCategory::Performance => "performance",
            ChecklistCategory::Style => "style",
            ChecklistCategory::Architecture => "architecture",
            ChecklistCategory::Testing => "testing",
            ChecklistCategory::Documentation => "documentation",
        };

        if let Some(templates) = self.templates.get(category_key) {
            templates.iter()
                .enumerate()
                .filter(|(_, t)| t.file_patterns.is_empty() || t.file_patterns.iter().any(|p| Self::matches_glob(file_path, p)))
                .map(|(idx, t)| ChecklistItem {
                    id: format!("{}_{:04}", category_key.to_uppercase(), idx + 1),
                    description: t.description.clone(),
                    category: t.category.clone(),
                    severity: t.severity.clone(),
                    applicable_file_types: t.file_patterns.clone(),
                    applicable_patterns: t.content_patterns.clone(),
                    is_checked: false,
                    is_auto_checkable: t.is_auto_checkable,
                    related_file: Some(file_path.to_string()),
                })
                .collect()
        } else {
            Vec::new()
        }
    }
}

impl Default for ChecklistEngine {
    fn default() -> Self {
        Self::new()
    }
}
