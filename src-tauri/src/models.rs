// HyperReview Data Models
// Data structures and serialization for all entities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Repository Entity
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Repository {
    pub path: String,
    pub current_branch: String,
    pub last_opened: String,
    pub head_commit: String,
    pub remote_url: Option<String>,
    pub is_active: bool,
}

/// Branch Entity
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Branch {
    pub name: String,
    pub is_current: bool,
    pub is_remote: bool,
    pub upstream: Option<String>,
    pub last_commit: String,
    pub last_commit_message: String,
    pub last_commit_author: String,
    pub last_commit_date: String,
}

/// Commit Entity
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Commit {
    pub oid: String,
    pub message: String,
    pub author_name: String,
    pub author_email: String,
    pub committer_name: String,
    pub committer_email: String,
    pub timestamp: String,
    pub parents: Vec<String>,
    pub tree_oid: String,
}

/// Diff Line Types
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DiffLineType {
    Added,
    Removed,
    Context,
    Header,
}

/// Severity Levels
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Success,
}

/// Diff Line Entity
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiffLine {
    pub old_line_number: Option<u32>,
    pub new_line_number: Option<u32>,
    pub content: String,
    pub line_type: DiffLineType,
    pub severity: Option<Severity>,
    pub message: Option<String>,
    pub hunk_header: Option<String>,
}

/// Comment Status
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CommentStatus {
    Draft,
    Submitted,
    Rejected,
}

/// Comment Entity
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Comment {
    pub id: String,
    pub file_path: String,
    pub line_number: u32,
    pub content: String,
    pub author: String,
    pub created_at: String,
    pub updated_at: String,
    pub status: CommentStatus,
    pub parent_id: Option<String>,
    pub tags: Vec<String>,
}

/// Tag Entity
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    pub id: String,
    pub label: String,
    pub color: String,
    pub description: Option<String>,
    pub usage_count: u32,
    pub created_at: String,
    pub updated_at: String,
}

/// Task Status
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TaskStatus {
    Active,
    Pending,
    Completed,
    Blocked,
}

/// Task Entity
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: u32,
    pub assignee: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub due_date: Option<String>,
    pub metadata: HashMap<String, String>,
}

/// Review Statistics
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReviewStats {
    pub total_files: u32,
    pub reviewed_files: u32,
    pub pending_files: u32,
    pub total_comments: u32,
    pub severe_issues: u32,
    pub completion_percentage: f32,
    pub estimated_time_remaining: Option<u32>,
    pub files_per_hour: f32,
}

/// Heatmap Categories
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum HeatmapCategory {
    High,
    Medium,
    Low,
}

/// Heatmap Item
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HeatmapItem {
    pub file_path: String,
    pub impact_score: f32,
    pub churn_score: f32,
    pub complexity_score: f32,
    pub change_frequency: u32,
    pub lines_of_code: u32,
    pub category: HeatmapCategory,
}

/// Checklist Categories
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChecklistCategory {
    Security,
    Performance,
    Style,
    Architecture,
    Testing,
    Documentation,
}

/// Checklist Item
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChecklistItem {
    pub id: String,
    pub description: String,
    pub category: ChecklistCategory,
    pub severity: Severity,
    pub applicable_file_types: Vec<String>,
    pub applicable_patterns: Vec<String>,
    pub is_checked: bool,
    pub is_auto_checkable: bool,
    pub related_file: Option<String>,
}

/// Blame Line
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlameLine {
    pub line_number: u32,
    pub content: String,
    pub commit_oid: String,
    pub commit_message: String,
    pub author_name: String,
    pub author_email: String,
    pub committer_name: String,
    pub committer_email: String,
    pub commit_date: String,
}

/// Blame Information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlameInfo {
    pub lines: Vec<BlameLine>,
    pub file_path: String,
}

/// Quality Gate Status
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum QualityGateStatus {
    Passing,
    Failing,
    Pending,
    Unknown,
}

/// Quality Gate
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QualityGate {
    pub name: String,
    pub status: QualityGateStatus,
    pub details: Option<String>,
    pub last_checked: String,
    pub url: Option<String>,
    pub metadata: HashMap<String, String>,
}

/// Search Result Types
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SearchResultType {
    File,
    Symbol,
    Commit,
    Command,
}

/// Search Result
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResult {
    pub result_type: SearchResultType,
    pub file_path: Option<String>,
    pub line_number: Option<u32>,
    pub content: String,
    pub highlight: Option<String>,
    pub score: f32,
}

/// Review Template
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReviewTemplate {
    pub id: String,
    pub name: String,
    pub content: String,
    pub placeholders: Vec<String>,
    pub category: Option<String>,
    pub usage_count: u32,
    pub created_at: String,
    pub updated_at: String,
}

/// Command Info for Command Palette
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommandInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
}

/// Complexity Metrics
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComplexityMetrics {
    pub file_path: String,
    pub cyclomatic_complexity: u32,
    pub cognitive_complexity: u32,
    pub lines_of_code: u32,
    pub function_count: u32,
    pub class_count: u32,
}

/// Security Issue
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecurityIssue {
    pub severity: Severity,
    pub message: String,
    pub line_number: Option<u32>,
    pub file_path: String,
    pub rule_id: String,
}

/// Submit Result
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubmitResult {
    pub success: bool,
    pub message: String,
    pub external_id: Option<String>,
    pub url: Option<String>,
}

/// Sync Result
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncResult {
    pub success: bool,
    pub message: String,
    pub commits_ahead: Option<u32>,
    pub commits_behind: Option<u32>,
}

// Convenience aliases for backward compatibility
pub type Repo = Repository;

/// Diff Parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct DiffParams {
    pub file_path: String,
    pub old_commit: Option<String>,
    pub new_commit: Option<String>,
}

/// File Path Parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct FilePathParams {
    pub file_path: String,
}

/// Comment Parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct CommentParams {
    pub file_path: String,
    pub line_number: u32,
    pub content: String,
}

/// Update Comment Parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCommentParams {
    pub comment_id: String,
    pub content: String,
}
