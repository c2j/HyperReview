// Security audit utilities
// Command injection prevention and input sanitization

use regex::Regex;

/// Security audit result
#[derive(Debug, Clone)]
pub struct SecurityAudit {
    pub is_safe: bool,
    pub issues: Vec<SecurityIssue>,
    pub risk_level: RiskLevel,
}

/// Security issue found
#[derive(Debug, Clone)]
pub struct SecurityIssue {
    pub severity: Severity,
    pub description: String,
    pub recommendation: String,
}

/// Risk levels
#[derive(Debug, Clone, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Severity levels
#[derive(Debug, Clone, PartialEq)]
pub enum Severity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Security auditor for input validation
pub struct SecurityAuditor {
    dangerous_patterns: Vec<DangerousPattern>,
}

struct DangerousPattern {
    name: String,
    pattern: Regex,
    severity: Severity,
    description: String,
    recommendation: String,
}

impl SecurityAuditor {
    pub fn new() -> Self {
        Self {
            dangerous_patterns: vec![
                DangerousPattern {
                    name: "Command Injection".to_string(),
                    pattern: Regex::new(r"[;&|`$()\\]").unwrap(),
                    severity: Severity::Critical,
                    description: "Possible command injection vulnerability detected".to_string(),
                    recommendation: "Use parameterized commands and sanitize user input".to_string(),
                },
                DangerousPattern {
                    name: "SQL Injection".to_string(),
                    pattern: Regex::new(r"('|(\\x27|\\x23)|(\\x3b|;)|(\\x3d|=)|(\\x2d|-)|(\\x2f|/)|(\\x5c|\\\\)|(\\x28|\\()|(\\x29|\\))|(\\x2a|\\*)").unwrap(),
                    severity: Severity::Critical,
                    description: "Possible SQL injection vulnerability detected".to_string(),
                    recommendation: "Use parameterized queries and input validation".to_string(),
                },
                DangerousPattern {
                    name: "Path Traversal".to_string(),
                    pattern: Regex::new(r"(\.\./|\.\.\\\\)").unwrap(),
                    severity: Severity::Error,
                    description: "Possible path traversal vulnerability detected".to_string(),
                    recommendation: "Validate and sanitize file paths".to_string(),
                },
                DangerousPattern {
                    name: "XSS Patterns".to_string(),
                    pattern: Regex::new(r"(<script|javascript:|on\w+\s*=)").unwrap(),
                    severity: Severity::Warning,
                    description: "Possible XSS vulnerability detected".to_string(),
                    recommendation: "Escape user input and use Content Security Policy".to_string(),
                },
                DangerousPattern {
                    name: "Hardcoded Secrets".to_string(),
                    pattern: Regex::new(r"(password|api[_-]?key|secret|token)\s*[:=]\s*['\"][^'\"]{10,}['\"]").unwrap(),
                    severity: Severity::Warning,
                    description: "Possible hardcoded secret detected".to_string(),
                    recommendation: "Use environment variables or secure credential storage".to_string(),
                },
                DangerousPattern {
                    name: "Unsafe Eval".to_string(),
                    pattern: Regex::new(r"eval\s*\(|exec\s*\(").unwrap(),
                    severity: Severity::Critical,
                    description: "Unsafe eval/exec usage detected".to_string(),
                    recommendation: "Avoid dynamic code execution".to_string(),
                },
                DangerousPattern {
                    name: "Network Request".to_string(),
                    pattern: Regex::new(r"(http://|https://|ftp://)").unwrap(),
                    severity: Severity::Info,
                    description: "External network request detected".to_string(),
                    recommendation: "Validate and sanitize URLs".to_string(),
                },
            ],
        }
    }

    /// Audit input for security issues
    pub fn audit_input(&self, input: &str, context: AuditContext) -> SecurityAudit {
        let mut issues = Vec::new();

        for pattern in &self.dangerous_patterns {
            if pattern.pattern.is_match(input) {
                issues.push(SecurityIssue {
                    severity: pattern.severity.clone(),
                    description: format!("{}: {}", pattern.name, pattern.description),
                    recommendation: pattern.recommendation.clone(),
                });
            }
        }

        // Additional context-specific checks
        if context == AuditContext::FilePath {
            issues.extend(self.audit_file_path(input));
        } else if context == AuditContext::GitCommand {
            issues.extend(self.audit_git_command(input));
        }

        let risk_level = self.calculate_risk_level(&issues);
        let is_safe = issues.is_empty() || matches!(risk_level, RiskLevel::Low);

        SecurityAudit {
            is_safe,
            issues,
            risk_level,
        }
    }

    /// Audit file path for security issues
    fn audit_file_path(&self, path: &str) -> Vec<SecurityIssue> {
        let mut issues = Vec::new();

        // Check for absolute paths outside expected directories
        if path.starts_with("/") && !path.starts_with("/tmp/") && !path.starts_with("/home/") {
            issues.push(SecurityIssue {
                severity: Severity::Warning,
                description: "Absolute path outside common directories".to_string(),
                recommendation: "Use relative paths or restrict to safe directories".to_string(),
            });
        }

        // Check for null bytes
        if path.contains('\0') {
            issues.push(SecurityIssue {
                severity: Severity::Critical,
                description: "Null byte injection detected".to_string(),
                recommendation: "Remove null bytes from input".to_string(),
            });
        }

        // Check for dangerous characters
        if path.contains('*') || path.contains('?') {
            issues.push(SecurityIssue {
                severity: Severity::Warning,
                description: "Wildcard characters in file path".to_string(),
                recommendation: "Escape or validate wildcard characters".to_string(),
            });
        }

        issues
    }

    /// Audit git command for security issues
    fn audit_git_command(&self, command: &str) -> Vec<SecurityIssue> {
        let mut issues = Vec::new();

        // Check for shell metacharacters
        if command.contains('|') || command.contains('&') || command.contains(';') {
            issues.push(SecurityIssue {
                severity: Severity::Critical,
                description: "Shell metacharacters in git command".to_string(),
                recommendation: "Use git2 library API instead of shell commands".to_string(),
            });
        }

        // Check for command substitution
        if command.contains('$(') || command.contains('`') {
            issues.push(SecurityIssue {
                severity: Severity::Critical,
                description: "Command substitution detected".to_string(),
                recommendation: "Avoid dynamic command execution".to_string(),
            });
        }

        issues
    }

    /// Calculate overall risk level
    fn calculate_risk_level(&self, issues: &[SecurityIssue]) -> RiskLevel {
        if issues.is_empty() {
            return RiskLevel::Low;
        }

        if issues.iter().any(|i| matches!(i.severity, Severity::Critical)) {
            RiskLevel::Critical
        } else if issues.iter().any(|i| matches!(i.severity, Severity::Error)) {
            RiskLevel::High
        } else if issues.iter().any(|i| matches!(i.severity, Severity::Warning)) {
            RiskLevel::Medium
        } else {
            RiskLevel::Low
        }
    }

    /// Sanitize input by removing dangerous characters
    pub fn sanitize_input(&self, input: &str, context: AuditContext) -> String {
        let mut sanitized = input.to_string();

        // Remove null bytes
        sanitized = sanitized.replace('\0', "");

        // Context-specific sanitization
        match context {
            AuditContext::FilePath => {
                // Remove path traversal attempts
                sanitized = sanitized.replace("../", "");
                sanitized = sanitized.replace("..\\", "");
                // Remove dangerous characters
                sanitized = sanitized.replace(|c: char| !c.is_alphanumeric() && !matches!(c, '/' | '.' | '_' | '-'), "");
            }
            AuditContext::GitCommand => {
                // Remove shell metacharacters
                sanitized = sanitized.replace(['|', '&', ';', '$', '(', ')', '`'], "");
            }
            AuditContext::General => {
                // General sanitization
                sanitized = sanitized.replace(['<', '>', '&', '"', '\''], "");
            }
            AuditContext::SQL => {
                // Remove SQL injection patterns
                sanitized = sanitized.replace('\'', "''");
            }
        }

        sanitized
    }

    /// Validate repository path
    pub fn validate_repo_path(&self, path: &str) -> Result<(), SecurityIssue> {
        let audit = self.audit_input(path, AuditContext::FilePath);

        if !audit.is_safe {
            if let Some(issue) = audit.issues.first() {
                return Err(issue.clone());
            }
        }

        // Additional validations
        if !std::path::Path::new(path).exists() {
            return Err(SecurityIssue {
                severity: Severity::Warning,
                description: "Repository path does not exist".to_string(),
                recommendation: "Verify the repository path is correct".to_string(),
            });
        }

        Ok(())
    }

    /// Generate security report
    pub fn generate_report(&self, audits: &[SecurityAudit]) -> String {
        let total_audits = audits.len();
        let safe_audits = audits.iter().filter(|a| a.is_safe).count();
        let unsafe_audits = total_audits - safe_audits;

        let mut report = format!("Security Audit Report\n");
        report.push_str("========================\n\n");
        report.push_str(&format!("Total Audits: {}\n", total_audits));
        report.push_str(&format!("Safe: {}\n", safe_audits));
        report.push_str(&format!("Unsafe: {}\n", unsafe_audits));
        report.push_str(&format!("Safety Rate: {:.1}%\n\n", (safe_audits as f64 / total_audits as f64) * 100.0));

        for (i, audit) in audits.iter().enumerate() {
            report.push_str(&format!("Audit #{}: {} Risk\n", i + 1, format!("{:?}", audit.risk_level)));
            if !audit.issues.is_empty() {
                for issue in &audit.issues {
                    report.push_str(&format!("  - {}: {}\n", format!("{:?}", issue.severity), issue.description));
                }
            }
            report.push('\n');
        }

        report
    }
}

/// Audit context
#[derive(Debug, Clone, PartialEq)]
pub enum AuditContext {
    General,
    FilePath,
    GitCommand,
    SQL,
}

/// Input validator
pub struct InputValidator {
    auditor: SecurityAuditor,
}

impl InputValidator {
    pub fn new() -> Self {
        Self {
            auditor: SecurityAuditor::new(),
        }
    }

    /// Validate and sanitize file path
    pub fn validate_file_path(&self, path: &str) -> Result<String, SecurityIssue> {
        let audit = self.auditor.audit_input(path, AuditContext::FilePath);

        if !audit.is_safe {
            if let Some(issue) = audit.issues.first() {
                return Err(issue.clone());
            }
        }

        Ok(self.auditor.sanitize_input(path, AuditContext::FilePath))
    }

    /// Validate and sanitize git command
    pub fn validate_git_command(&self, command: &str) -> Result<String, SecurityIssue> {
        let audit = self.auditor.audit_input(command, AuditContext::GitCommand);

        if !audit.is_safe {
            if let Some(issue) = audit.issues.first() {
                return Err(issue.clone());
            }
        }

        Ok(self.auditor.sanitize_input(command, AuditContext::GitCommand))
    }

    /// Check if input is safe
    pub fn is_safe(&self, input: &str, context: AuditContext) -> bool {
        let audit = self.auditor.audit_input(input, context);
        audit.is_safe
    }
}

impl Default for SecurityAuditor {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for InputValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_command_injection() {
        let auditor = SecurityAuditor::new();
        let audit = auditor.audit_input("file; rm -rf /", AuditContext::General);

        assert!(!audit.is_safe);
        assert!(audit.issues.iter().any(|i| i.description.contains("Command Injection")));
    }

    #[test]
    fn test_detect_path_traversal() {
        let auditor = SecurityAuditor::new();
        let audit = auditor.audit_input("../../../etc/passwd", AuditContext::FilePath);

        assert!(!audit.is_safe);
        assert!(audit.issues.iter().any(|i| i.description.contains("Path Traversal")));
    }

    #[test]
    fn test_sanitize_input() {
        let auditor = SecurityAuditor::new();
        let sanitized = auditor.sanitize_input("../../../etc/passwd", AuditContext::FilePath);

        assert!(!sanitized.contains(".."));
    }

    #[test]
    fn test_safe_input() {
        let auditor = SecurityAuditor::new();
        let audit = auditor.audit_input("hello world", AuditContext::General);

        assert!(audit.is_safe);
    }
}
