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
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Active,
    Pending,
    Completed,
    Blocked,
}

/// Task Type
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TaskType {
    Code,
    Sql,
    Security,
}

/// File Status for Task Files
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FileStatus {
    Modified,
    Added,
    Deleted,
}

/// File Review Status
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FileReviewStatus {
    Pending,
    Approved,
    Concern,
    MustChange,
    Question,
}

/// Task File - File associated with a task
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskFile {
    pub id: String,
    pub path: String,
    pub name: String,
    pub status: FileStatus,
    #[serde(rename = "reviewStatus")]
    pub review_status: Option<FileReviewStatus>,
    #[serde(rename = "reviewComment")]
    pub review_comment: Option<String>,
}

/// Update File Review Status Parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateFileReviewParams {
    pub task_id: String,
    pub file_id: String,
    #[serde(rename = "reviewStatus")]
    pub review_status: FileReviewStatus,
    #[serde(rename = "reviewComment")]
    pub review_comment: Option<String>,
}

/// File Review Comment - Historical record of all review comments for a file
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileReviewComment {
    pub id: String,
    pub task_id: String,
    pub file_id: String,
    #[serde(rename = "reviewStatus")]
    pub review_status: String,
    #[serde(rename = "reviewComment")]
    pub review_comment: String,
    #[serde(rename = "submittedBy")]
    pub submitted_by: String,
    #[serde(rename = "submittedAt")]
    pub submitted_at: String,
}

/// Create Local Task Parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLocalTaskParams {
    pub title: String,
    pub task_type: TaskType,
    pub files: Vec<String>, // List of file paths
}

/// Task Entity (extended with files and type)
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
    #[serde(rename = "type")]
    pub task_type: Option<TaskType>,
    #[serde(default)]
    pub files: Vec<TaskFile>,
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
    pub id: String,
    pub name: String,
    pub path: String,
    pub impact: HeatmapCategory,
    pub score: u32,  // Impact score as integer (0-100)
    pub exists: bool,  // Whether the file exists in the working directory
}

/// File Node for file tree
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileNode {
    pub id: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "type")]
    pub file_type: String,  // "file" or "folder"
    pub status: String,     // "modified", "added", "deleted", "none"
    pub children: Option<Vec<FileNode>>,
    pub stats: Option<FileStats>,
    pub exists: bool,       // Whether the file exists in the working directory
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileStats {
    pub added: u32,
    pub removed: u32,
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

/// Review Guide Categories
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ReviewGuideCategory {
    Security,
    Performance,
    Style,
    Logic,
}

/// Review Guide Severity
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ReviewGuideSeverity {
    High,
    Medium,
    Low,
}

/// Review Guide Item
/// category 使用字符串类型以支持中文类别
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReviewGuideItem {
    pub id: String,
    pub category: String, // 直接使用字符串，支持中文如 "安全性", "性能优化", "代码规范" 等
    pub title: String,
    pub description: String,
    pub severity: ReviewGuideSeverity,
    #[serde(rename = "referenceUrl")]
    pub reference_url: Option<String>,
    #[serde(rename = "applicableExtensions")]
    pub applicable_extensions: Vec<String>,
}

/// Checklist JSON Item (from docs/CheckList.json)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChecklistJsonItem {
    pub id: String,
    pub category: String,
    pub subcategory: String,
    pub description: String,
    #[serde(rename = "implementationStatus")]
    pub implementation_status: String,
}

/// Helper function to map checklist category to review guide category
pub fn map_checklist_category(category: &str) -> ReviewGuideCategory {
    match category {
        "安全性" | "Security" => ReviewGuideCategory::Security,
        "性能优化" | "性能" | "Performance" => ReviewGuideCategory::Performance,
        "可扩展性" | "可维护性/可复用性" | "并发与防重" => ReviewGuideCategory::Logic,
        "通用原则" | "数据库" | "代码规范" | "可读性" | "消除重复" | "异常处理"
        | "日志打印" | "多线程" | "接口检查" | "内存" | "输入校验"
        | "接口调用" | "版本基础" | "其他" | "前端" | "程序提交"
        | "前端代码提交" | "General" => ReviewGuideCategory::Style,
        _ => ReviewGuideCategory::Style,
    }
}

/// Helper function to map checklist severity based on subcategory
pub fn map_checklist_severity(subcategory: &str) -> ReviewGuideSeverity {
    match subcategory {
        "安全性" => ReviewGuideSeverity::High,
        "性能" | "并发与防重" | "内存" => ReviewGuideSeverity::Medium,
        "SQL编写" | "索引规约" => ReviewGuideSeverity::High,
        "异常处理" | "多线程" => ReviewGuideSeverity::Medium,
        _ => ReviewGuideSeverity::Low,
    }
}

/// Helper function to get applicable extensions based on subcategory
pub fn get_applicable_extensions(subcategory: &str) -> Vec<String> {
    match subcategory {
        "安全性" | "多线程" | "接口调用" => vec![".java".to_string(), ".ts".to_string(), ".go".to_string(), ".py".to_string()],
        "SQL编写" | "索引规约" | "建表规约" | "数据库" => vec![".sql".to_string(), ".java".to_string(), ".xml".to_string()],
        "前端代码提交" | "前端" => vec![".tsx".to_string(), ".jsx".to_string(), ".ts".to_string(), ".js".to_string(), ".html".to_string()],
        "日志打印" | "可读性" | "代码规范" => vec![".java".to_string(), ".ts".to_string(), ".go".to_string(), ".py".to_string(), ".rs".to_string()],
        _ => vec![".java".to_string(), ".ts".to_string(), ".tsx".to_string(), ".go".to_string(), ".py".to_string(), ".rs".to_string()],
    }
}
