use crate::models::task::{LocalTask, CreateTaskRequest, TaskSummary, TaskStatus, Comment as TaskComment};
use crate::storage::task_store::TaskStore;
use crate::commands::text_parser;
use crate::git::repo_manager::RepoManager;
use tauri::State;
use uuid::Uuid;
use chrono::Utc;
use serde_json::json;

/// Convert Task Comment to API Comment
fn convert_task_comment_to_api_comment(task_comment: TaskComment, file_path: String) -> crate::models::Comment {
    crate::models::Comment {
        id: task_comment.id.to_string(),
        file_path,
        line_number: task_comment.line_number.unwrap_or(0),
        content: task_comment.content,
        author: task_comment.author,
        created_at: task_comment.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        updated_at: task_comment.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        status: crate::models::CommentStatus::Draft,
        parent_id: None,
        tags: Vec::new(),
    }
}

#[tauri::command]
pub async fn parse_task_text(text: String) -> Result<Vec<crate::models::task::TaskItem>, String> {
    text_parser::parse_task_text(&text).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_task(
    payload: CreateTaskRequest,
    _state: State<'_, crate::AppState>,
) -> Result<LocalTask, String> {
    let items = text_parser::parse_task_text(&payload.items_text)
        .map_err(|e| e.to_string())?;

    let task = LocalTask {
        id: Uuid::new_v4(),
        name: payload.name,
        repo_path: payload.repo_path,
        base_ref: payload.base_ref,
        create_time: Utc::now(),
        update_time: Utc::now(),
        status: TaskStatus::InProgress,
        total_items: items.len() as u32,
        completed_items: 0,
        items,
    };

    let store = TaskStore::new().map_err(|e| e.to_string())?;
    store.save_task(&task).map_err(|e| e.to_string())?;

    Ok(task)
}

#[tauri::command]
pub async fn list_tasks() -> Result<Vec<TaskSummary>, String> {
    let store = TaskStore::new().map_err(|e| e.to_string())?;
    let tasks = store.list_tasks().map_err(|e| e.to_string())?;
    
    let summaries = tasks.into_iter().map(|t| {
        let progress = if t.total_items > 0 {
            t.completed_items as f32 / t.total_items as f32 * 100.0
        } else {
            0.0
        };
        
        TaskSummary {
            id: t.id,
            name: t.name,
            status: t.status,
            progress,
        }
    }).collect();
    
    Ok(summaries)
}

#[tauri::command]
pub async fn get_task(task_id: Uuid) -> Result<LocalTask, String> {
    let store = TaskStore::new().map_err(|e| e.to_string())?;
    store.load_task(task_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_task_progress(
    task_id: Uuid,
    item_index: usize,
    reviewed: bool,
) -> Result<(), String> {
    let store = TaskStore::new().map_err(|e| e.to_string())?;
    let mut task = store.load_task(task_id).map_err(|e| e.to_string())?;
    
    if item_index < task.items.len() {
        if task.items[item_index].reviewed != reviewed {
            task.items[item_index].reviewed = reviewed;
            if reviewed {
                task.completed_items +=1;
            } else {
                task.completed_items = task.completed_items.saturating_sub(1);
            }
            task.update_time = Utc::now();
            store.save_task(&task).map_err(|e| e.to_string())?;
        }
    }
    
    Ok(())
}

#[tauri::command]
pub async fn read_task_item_from_ref(
    repo_path: String,
    reference: String,
    file_path: String,
) -> Result<String, String> {
    RepoManager::read_file_from_ref(&repo_path, &reference, &file_path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_task(task_id: Uuid) -> Result<(), String> {
    let store = TaskStore::new().map_err(|e| e.to_string())?;
    store.delete_task(task_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn archive_task(task_id: Uuid) -> Result<(), String> {
    let store = TaskStore::new().map_err(|e| e.to_string())?;
    let mut task = store.load_task(task_id).map_err(|e| e.to_string())?;
    
    task.status = TaskStatus::Archived;
    task.update_time = Utc::now();
    store.save_task(&task).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn reimport_task_text(
    task_id: Uuid,
    items_text: String,
) -> Result<LocalTask, String> {
    let store = TaskStore::new().map_err(|e| e.to_string())?;
    let mut task = store.load_task(task_id).map_err(|e| e.to_string())?;
    
    let new_items = text_parser::parse_task_text(&items_text)
        .map_err(|e| e.to_string())?;
    
    let total_items = new_items.len() as u32;
    task.items = new_items;
    task.total_items = total_items;
    task.update_time = Utc::now();
    
    store.save_task(&task).map_err(|e| e.to_string())?;
    Ok(task)
}

#[tauri::command]
pub async fn update_task(
    task_id: Uuid,
    name: Option<String>,
    base_ref: Option<String>,
) -> Result<LocalTask, String> {
    let store = TaskStore::new().map_err(|e| e.to_string())?;
    let mut task = store.load_task(task_id).map_err(|e| e.to_string())?;
    
    if let Some(name) = name {
        task.name = name;
    }
    
    if let Some(base_ref) = base_ref {
        task.base_ref = base_ref;
    }
    
    task.update_time = Utc::now();
    store.save_task(&task).map_err(|e| e.to_string())?;
    Ok(task)
}

#[tauri::command]
pub async fn export_task(task_id: Uuid) -> Result<String, String> {
    let store = TaskStore::new().map_err(|e| e.to_string())?;
    let task = store.load_task(task_id).map_err(|e| e.to_string())?;
    
    let json = serde_json::to_string_pretty(&task)
        .map_err(|e| e.to_string())?;
    
    Ok(json)
}

#[tauri::command]
pub async fn export_all_tasks() -> Result<String, String> {
    let store = TaskStore::new().map_err(|e| e.to_string())?;
    let tasks = store.list_tasks().map_err(|e| e.to_string())?;
    
    let json = serde_json::to_string_pretty(&tasks)
        .map_err(|e| e.to_string())?;
    
    Ok(json)
}

#[tauri::command]
pub async fn submit_task_to_gerrit(
    _state: State<'_, crate::AppState>,
    _task_id: String,
    _gerrit_url: String,
    _username: String,
    _change_id: String,
    _revision_id: String,
    _score: Option<i32>,
) -> Result<crate::models::SubmitResult, String> {
    // Gerrit client temporarily disabled
    Err("Gerrit integration temporarily disabled".to_string())
}

#[tauri::command]
pub async fn submit_task_to_codearts(
    task_id: String,
    project_id: String,
    mr_id: u64,
    _approval: Option<String>,
) -> Result<crate::models::SubmitResult, String> {
    use crate::remote::codearts_client::CodeArtsClient;
    
    let store = TaskStore::new().map_err(|e| e.to_string())?;
    let task = store.load_task(uuid::Uuid::parse_str(&task_id).unwrap())
        .map_err(|e| e.to_string())?;

    let api_comments: Vec<crate::models::Comment> = task.items
        .into_iter()
        .filter_map(|item| {
            if item.comments.is_empty() {
                return None;
            }
            let task_comment = item.comments.into_iter().next()?;
            Some(convert_task_comment_to_api_comment(task_comment, item.file))
        })
        .collect();

    let codearts_client = CodeArtsClient::new("https://codearts.example.com");
    let result = codearts_client.submit_review(&project_id, mr_id, api_comments, None)
        .map_err(|e| e.to_string())?;

    Ok(result)
}

#[tauri::command]
pub async fn submit_task_to_custom_api(
    task_id: String,
    endpoint: String,
    method: String,
    api_url: String,
) -> Result<crate::models::SubmitResult, String> {
    use crate::remote::custom_client::CustomApiClient;
    use serde_json::json;
    
    let store = TaskStore::new().map_err(|e| e.to_string())?;
    let task = store.load_task(uuid::Uuid::parse_str(&task_id).unwrap())
        .map_err(|e| e.to_string())?;

    let payload = json!({
        "task_id": task.id.to_string(),
        "task_name": task.name,
        "repo_path": task.repo_path,
        "base_ref": task.base_ref,
        "status": format!("{:?}", task.status),
        "progress": {
            "total": task.total_items,
            "completed": task.completed_items,
            "percentage": if task.total_items > 0 {
                (task.completed_items as f32 / task.total_items as f32) * 100.0
            } else {
                0.0
            }
        },
        "items": task.items.iter().map(|item| json!({
            "file": item.file,
            "reviewed": item.reviewed,
            "comments": []
        })).collect::<Vec<_>>(),
        "submitted_at": task.update_time.to_rfc3339()
    });

    let custom_client = CustomApiClient::new(&api_url);
    let result = custom_client.submit_review(&endpoint, payload, &method)
        .map_err(|e| e.to_string())?;

    Ok(result)
}
