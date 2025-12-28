use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    InProgress,
    Completed,
    Archived,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskSeverity {
    Error,
    Warning,
    Question,
    Ok,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LineRange {
    pub start: Option<u32>,
    pub end: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Comment {
    pub id: Uuid,
    pub author: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub line_number: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskItem {
    pub file: String,
    pub line_range: Option<LineRange>,
    pub preset_comment: Option<String>,
    pub severity: Option<TaskSeverity>,
    pub tags: Vec<String>,
    pub reviewed: bool,
    pub comments: Vec<Comment>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalTask {
    pub id: Uuid,
    pub name: String,
    pub repo_path: String,
    pub base_ref: String,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
    pub status: TaskStatus,
    pub total_items: u32,
    pub completed_items: u32,
    pub items: Vec<TaskItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub name: String,
    pub repo_path: String,
    pub base_ref: String,
    pub items_text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskSummary {
    pub id: Uuid,
    pub name: String,
    pub status: TaskStatus,
    pub progress: f32,
}

