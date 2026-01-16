// Comment Engine Management Commands
// Tauri commands for comment creation, editing, threading, and management

use tauri::State;
use log::{info, error};

use crate::AppState;
use crate::services::comment_engine::{
    CommentEngine, CommentEngineConfig, CreateCommentParams, UpdateCommentParams,
    CommentSearchCriteria, CommentStats, CommentOperationResult, CommentThread,
    CreateInlineCommentParams, InlineComment, LineComments, DiffSide
};
use crate::models::gerrit::{ReviewComment, CommentStatus};

/// Create a new comment
#[tauri::command]
pub async fn comment_create(
    params: CreateCommentParams,
    state: State<'_, AppState>,
) -> Result<CommentOperationResult, String> {
    info!("Creating comment for session: {}, file: {}", params.session_id, params.file_path);

    let database = state.database.clone();
    let engine = CommentEngine::new(CommentEngineConfig::default());

    tokio::task::spawn_blocking(move || {
        let database = database.lock().unwrap();
        engine.create_comment(params, &*database)
    }).await.map_err(|e| format!("Task join error: {}", e))?.map_err(|e| {
        error!("Failed to create comment: {}", e);
        format!("Failed to create comment: {}", e)
    }).map(|result| {
        info!("Successfully created comment");
        result
    })
}

/// Update an existing comment
#[tauri::command]
pub async fn comment_update(
    params: UpdateCommentParams,
    state: State<'_, AppState>,
) -> Result<CommentOperationResult, String> {
    info!("Updating comment: {}", params.comment_id);

    let database = state.database.clone();
    let engine = CommentEngine::new(CommentEngineConfig::default());

    tokio::task::spawn_blocking(move || {
        let database = database.lock().unwrap();
        engine.update_comment(params, &*database)
    }).await.map_err(|e| format!("Task join error: {}", e))?.map_err(|e| {
        error!("Failed to update comment: {}", e);
        format!("Failed to update comment: {}", e)
    }).map(|result| {
        info!("Successfully updated comment");
        result
    })
}

/// Delete a comment
#[tauri::command]
pub async fn comment_delete(
    comment_id: String,
    state: State<'_, AppState>,
) -> Result<CommentOperationResult, String> {
    info!("Deleting comment: {}", comment_id);

    let database = state.database.clone();
    let engine = CommentEngine::new(CommentEngineConfig::default());

    tokio::task::spawn_blocking(move || {
        let database = database.lock().unwrap();
        engine.delete_comment(&comment_id, &*database)
    }).await.map_err(|e| format!("Task join error: {}", e))?.map_err(|e| {
        error!("Failed to delete comment: {}", e);
        format!("Failed to delete comment: {}", e)
    }).map(|result| {
        info!("Successfully deleted comment");
        result
    })
}

/// Get a comment by ID
#[tauri::command]
pub async fn comment_get(
    comment_id: String,
    state: State<'_, AppState>,
) -> Result<Option<ReviewComment>, String> {
    info!("Getting comment: {}", comment_id);

    let database = state.database.clone();
    let engine = CommentEngine::new(CommentEngineConfig::default());

    tokio::task::spawn_blocking(move || {
        let database = database.lock().unwrap();
        engine.get_comment(&comment_id, &*database)
    }).await.map_err(|e| format!("Task join error: {}", e))?.map_err(|e| {
        error!("Failed to get comment: {}", e);
        format!("Failed to get comment: {}", e)
    }).map(|comment| {
        info!("Successfully retrieved comment");
        comment
    })
}

/// Get comments for a session
#[tauri::command]
pub async fn comment_get_session_comments(
    session_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<ReviewComment>, String> {
    info!("Getting comments for session: {}", session_id);

    let database = state.database.clone();
    let engine = CommentEngine::new(CommentEngineConfig::default());

    tokio::task::spawn_blocking(move || {
        let database = database.lock().unwrap();
        engine.get_session_comments(&session_id, &*database)
    }).await.map_err(|e| format!("Task join error: {}", e))?.map_err(|e| {
        error!("Failed to get session comments: {}", e);
        format!("Failed to get session comments: {}", e)
    }).map(|comments| {
        info!("Successfully retrieved {} comments for session", comments.len());
        comments
    })
}

/// Get comments for a file
#[tauri::command]
pub async fn comment_get_file_comments(
    session_id: String,
    file_path: String,
    state: State<'_, AppState>,
) -> Result<Vec<ReviewComment>, String> {
    info!("Getting comments for file: {} in session: {}", file_path, session_id);

    let database = state.database.clone();
    let engine = CommentEngine::new(CommentEngineConfig::default());

    tokio::task::spawn_blocking(move || {
        let database = database.lock().unwrap();
        engine.get_file_comments(&session_id, &file_path, &*database)
    }).await.map_err(|e| format!("Task join error: {}", e))?.map_err(|e| {
        error!("Failed to get file comments: {}", e);
        format!("Failed to get file comments: {}", e)
    }).map(|comments| {
        info!("Successfully retrieved {} comments for file", comments.len());
        comments
    })
}

/// Get comment thread
#[tauri::command]
pub async fn comment_get_thread(
    root_comment_id: String,
    state: State<'_, AppState>,
) -> Result<Option<CommentThread>, String> {
    info!("Getting comment thread for root: {}", root_comment_id);

    let database = state.database.clone();
    let engine = CommentEngine::new(CommentEngineConfig::default());

    tokio::task::spawn_blocking(move || {
        let database = database.lock().unwrap();
        engine.get_comment_thread(&root_comment_id, &*database)
    }).await.map_err(|e| format!("Task join error: {}", e))?.map_err(|e| {
        error!("Failed to get comment thread: {}", e);
        format!("Failed to get comment thread: {}", e)
    }).map(|thread| {
        info!("Successfully retrieved comment thread");
        thread
    })
}

/// Search comments
#[tauri::command]
pub async fn comment_search(
    criteria: CommentSearchCriteria,
    state: State<'_, AppState>,
) -> Result<Vec<ReviewComment>, String> {
    info!("Searching comments");

    let database = state.database.clone();
    let engine = CommentEngine::new(CommentEngineConfig::default());

    tokio::task::spawn_blocking(move || {
        let database = database.lock().unwrap();
        engine.search_comments(&criteria, &*database)
    }).await.map_err(|e| format!("Task join error: {}", e))?.map_err(|e| {
        error!("Failed to search comments: {}", e);
        format!("Failed to search comments: {}", e)
    }).map(|comments| {
        info!("Successfully found {} comments matching criteria", comments.len());
        comments
    })
}

/// Get comment statistics
#[tauri::command]
pub async fn comment_get_stats(
    session_id: String,
    state: State<'_, AppState>,
) -> Result<CommentStats, String> {
    info!("Getting comment statistics for session: {}", session_id);

    let database = state.database.clone();
    let engine = CommentEngine::new(CommentEngineConfig::default());

    tokio::task::spawn_blocking(move || {
        let database = database.lock().unwrap();
        engine.get_comment_stats(&session_id, &*database)
    }).await.map_err(|e| format!("Task join error: {}", e))?.map_err(|e| {
        error!("Failed to get comment stats: {}", e);
        format!("Failed to get comment stats: {}", e)
    }).map(|stats| {
        info!("Successfully calculated comment statistics");
        stats
    })
}

/// Publish all draft comments
#[tauri::command]
pub async fn comment_publish_all_drafts(
    session_id: String,
    state: State<'_, AppState>,
) -> Result<u32, String> {
    info!("Publishing all draft comments for session: {}", session_id);

    let database = state.database.clone();
    let engine = CommentEngine::new(CommentEngineConfig::default());

    tokio::task::spawn_blocking(move || {
        let database = database.lock().unwrap();
        engine.publish_all_drafts(&session_id, &*database)
    }).await.map_err(|e| format!("Task join error: {}", e))?.map_err(|e| {
        error!("Failed to publish draft comments: {}", e);
        format!("Failed to publish draft comments: {}", e)
    }).map(|count| {
        info!("Successfully published {} draft comments", count);
        count
    })
}

/// Update comment status
#[tauri::command]
pub async fn comment_update_status(
    comment_id: String,
    status: String,
    state: State<'_, AppState>,
) -> Result<CommentOperationResult, String> {
    info!("Updating comment status: {} to {}", comment_id, status);

    let comment_status = CommentStatus::from_string(&status);
    let params = UpdateCommentParams {
        comment_id: comment_id.clone(),
        content: None,
        status: Some(comment_status),
        comment_type: None,
    };

    let database = state.database.clone();
    let engine = CommentEngine::new(CommentEngineConfig::default());

    tokio::task::spawn_blocking(move || {
        let database = database.lock().unwrap();
        engine.update_comment(params, &*database)
    }).await.map_err(|e| format!("Task join error: {}", e))?.map_err(|e| {
        error!("Failed to update comment status: {}", e);
        format!("Failed to update comment status: {}", e)
    }).map(|result| {
        info!("Successfully updated comment status");
        result
    })
}

/// Get comment engine configuration
#[tauri::command]
pub async fn comment_get_config(_state: State<'_, AppState>) -> Result<CommentEngineConfig, String> {
    Ok(CommentEngineConfig::default())
}

/// Update comment engine configuration
#[tauri::command]
pub async fn comment_update_config(
    config: CommentEngineConfig,
    _state: State<'_, AppState>,
) -> Result<bool, String> {
    info!("Updating comment engine configuration");
    
    // TODO: Persist configuration to storage
    info!("Comment engine configuration updated successfully");
    Ok(true)
}

/// Create an inline comment with positioning
#[tauri::command]
pub async fn comment_create_inline(
    params: CreateInlineCommentParams,
    state: State<'_, AppState>,
) -> Result<CommentOperationResult, String> {
    info!("Creating inline comment for {}:{} in session: {}", 
          params.file_path, params.line_number, params.session_id);

    let database = state.database.clone();
    let engine = CommentEngine::new(CommentEngineConfig::default());

    tokio::task::spawn_blocking(move || {
        let database = database.lock().unwrap();
        engine.create_inline_comment(params, &*database)
    }).await.map_err(|e| format!("Task join error: {}", e))?.map_err(|e| {
        error!("Failed to create inline comment: {}", e);
        format!("Failed to create inline comment: {}", e)
    }).map(|result| {
        info!("Successfully created inline comment");
        result
    })
}

/// Get comments for a specific line
#[tauri::command]
pub async fn comment_get_line_comments(
    session_id: String,
    file_path: String,
    line_number: u32,
    side: Option<String>,
    state: State<'_, AppState>,
) -> Result<LineComments, String> {
    info!("Getting line comments for {}:{} in session: {}", file_path, line_number, session_id);

    let diff_side = side.as_deref().map(|s| match s {
        "left" => DiffSide::Left,
        "right" => DiffSide::Right,
        "both" => DiffSide::Both,
        _ => DiffSide::Right,
    });

    let database = state.database.clone();
    let engine = CommentEngine::new(CommentEngineConfig::default());

    tokio::task::spawn_blocking(move || {
        let database = database.lock().unwrap();
        engine.get_line_comments(&session_id, &file_path, line_number, diff_side, &*database)
    }).await.map_err(|e| format!("Task join error: {}", e))?.map_err(|e| {
        error!("Failed to get line comments: {}", e);
        format!("Failed to get line comments: {}", e)
    }).map(|line_comments| {
        info!("Successfully retrieved {} comments for line", line_comments.total_count);
        line_comments
    })
}

/// Get all inline comments for a file
#[tauri::command]
pub async fn comment_get_file_inline_comments(
    session_id: String,
    file_path: String,
    state: State<'_, AppState>,
) -> Result<Vec<InlineComment>, String> {
    info!("Getting inline comments for file: {} in session: {}", file_path, session_id);

    let database = state.database.clone();
    let engine = CommentEngine::new(CommentEngineConfig::default());

    tokio::task::spawn_blocking(move || {
        let database = database.lock().unwrap();
        engine.get_file_inline_comments(&session_id, &file_path, &*database)
    }).await.map_err(|e| format!("Task join error: {}", e))?.map_err(|e| {
        error!("Failed to get file inline comments: {}", e);
        format!("Failed to get file inline comments: {}", e)
    }).map(|comments| {
        info!("Successfully retrieved {} inline comments for file", comments.len());
        comments
    })
}

/// Get comments grouped by line for diff display
#[tauri::command]
pub async fn comment_get_comments_by_line(
    session_id: String,
    file_path: String,
    state: State<'_, AppState>,
) -> Result<std::collections::HashMap<u32, LineComments>, String> {
    info!("Getting comments grouped by line for file: {} in session: {}", file_path, session_id);

    let database = state.database.clone();
    let engine = CommentEngine::new(CommentEngineConfig::default());

    tokio::task::spawn_blocking(move || {
        let database = database.lock().unwrap();
        engine.get_comments_by_line(&session_id, &file_path, &*database)
    }).await.map_err(|e| format!("Task join error: {}", e))?.map_err(|e| {
        error!("Failed to get comments by line: {}", e);
        format!("Failed to get comments by line: {}", e)
    }).map(|comments_by_line| {
        info!("Successfully grouped comments into {} lines", comments_by_line.len());
        comments_by_line
    })
}

/// Update inline comment position
#[tauri::command]
pub async fn comment_update_inline_position(
    comment_id: String,
    new_line_number: u32,
    new_column_start: Option<u32>,
    new_column_end: Option<u32>,
    state: State<'_, AppState>,
) -> Result<CommentOperationResult, String> {
    info!("Updating inline comment position: {} to line {}", comment_id, new_line_number);

    let database = state.database.clone();
    let engine = CommentEngine::new(CommentEngineConfig::default());

    tokio::task::spawn_blocking(move || {
        let database = database.lock().unwrap();
        engine.update_inline_comment_position(
            &comment_id, 
            new_line_number, 
            new_column_start, 
            new_column_end, 
            &*database
        )
    }).await.map_err(|e| format!("Task join error: {}", e))?.map_err(|e| {
        error!("Failed to update inline comment position: {}", e);
        format!("Failed to update inline comment position: {}", e)
    }).map(|result| {
        info!("Successfully updated inline comment position");
        result
    })
}

/// Highlight comments in a line range
#[tauri::command]
pub async fn comment_highlight_range(
    session_id: String,
    file_path: String,
    start_line: u32,
    end_line: u32,
    state: State<'_, AppState>,
) -> Result<Vec<InlineComment>, String> {
    info!("Highlighting comments in range {}:{}-{} for session: {}", 
          file_path, start_line, end_line, session_id);

    let database = state.database.clone();
    let engine = CommentEngine::new(CommentEngineConfig::default());

    tokio::task::spawn_blocking(move || {
        let database = database.lock().unwrap();
        engine.highlight_comments_in_range(&session_id, &file_path, start_line, end_line, &*database)
    }).await.map_err(|e| format!("Task join error: {}", e))?.map_err(|e| {
        error!("Failed to highlight comments in range: {}", e);
        format!("Failed to highlight comments in range: {}", e)
    }).map(|highlighted_comments| {
        info!("Successfully highlighted {} comments in range", highlighted_comments.len());
        highlighted_comments
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::gerrit::CommentType;

    #[tokio::test]
    async fn test_comment_config_creation() {
        let config = CommentEngineConfig::default();
        assert!(config.auto_save_drafts);
        assert_eq!(config.max_comment_length, 10000);
        assert!(config.enable_threading);
    }

    #[tokio::test]
    async fn test_create_comment_params() {
        let params = CreateCommentParams {
            session_id: "test-session".to_string(),
            file_path: "src/main.rs".to_string(),
            line_number: Some(42),
            content: "This needs improvement".to_string(),
            comment_type: CommentType::Issue,
            parent_comment_id: None,
        };

        assert_eq!(params.session_id, "test-session");
        assert_eq!(params.file_path, "src/main.rs");
        assert_eq!(params.line_number, Some(42));
        assert_eq!(params.comment_type, CommentType::Issue);
    }

    #[tokio::test]
    async fn test_update_comment_params() {
        let params = UpdateCommentParams {
            comment_id: "comment-123".to_string(),
            content: Some("Updated content".to_string()),
            status: Some(CommentStatus::Published),
            comment_type: None,
        };

        assert_eq!(params.comment_id, "comment-123");
        assert_eq!(params.content, Some("Updated content".to_string()));
        assert_eq!(params.status, Some(CommentStatus::Published));
    }

    #[tokio::test]
    async fn test_create_inline_comment_params() {
        let params = CreateInlineCommentParams {
            session_id: "test-session".to_string(),
            file_path: "src/main.rs".to_string(),
            line_number: 42,
            column_start: Some(10),
            column_end: Some(20),
            side: DiffSide::Right,
            content: "This line needs improvement".to_string(),
            comment_type: crate::models::gerrit::CommentType::Issue,
            parent_comment_id: None,
        };

        assert_eq!(params.session_id, "test-session");
        assert_eq!(params.file_path, "src/main.rs");
        assert_eq!(params.line_number, 42);
        assert_eq!(params.column_start, Some(10));
        assert_eq!(params.column_end, Some(20));
        assert_eq!(params.side, DiffSide::Right);
    }

    #[tokio::test]
    async fn test_diff_side_enum() {
        assert_eq!(DiffSide::Left, DiffSide::Left);
        assert_eq!(DiffSide::Right, DiffSide::Right);
        assert_eq!(DiffSide::Both, DiffSide::Both);
        assert_ne!(DiffSide::Left, DiffSide::Right);
    }
}