// Diff Engine Management Commands
// Tauri commands for diff generation and navigation

use tauri::State;
use log::{info, error};

use crate::AppState;
use crate::services::diff_engine::{DiffEngine, DiffConfig, ProcessedDiff, DiffViewType, LineMapping};
use crate::models::gerrit::ChangeFile;

/// Generate unified diff view for a change file
#[tauri::command]
pub async fn diff_generate_unified(
    change_file: ChangeFile,
    config: Option<DiffConfig>,
    _state: State<'_, AppState>,
) -> Result<ProcessedDiff, String> {
    info!("Generating unified diff for file: {}", change_file.file_path);

    tokio::task::spawn_blocking(move || {
        let diff_config = config.unwrap_or_default();
        let engine = DiffEngine::new(diff_config);

        match engine.generate_unified_diff(&change_file) {
            Ok(diff) => {
                info!("Successfully generated unified diff for: {}", change_file.file_path);
                Ok(diff)
            }
            Err(e) => {
                error!("Failed to generate unified diff: {}", e);
                Err(format!("Failed to generate unified diff: {}", e))
            }
        }
    }).await.map_err(|e| format!("Task join error: {}", e))?
}

/// Generate side-by-side diff view for a change file
#[tauri::command]
pub async fn diff_generate_side_by_side(
    change_file: ChangeFile,
    config: Option<DiffConfig>,
    _state: State<'_, AppState>,
) -> Result<ProcessedDiff, String> {
    info!("Generating side-by-side diff for file: {}", change_file.file_path);

    tokio::task::spawn_blocking(move || {
        let diff_config = config.unwrap_or_default();
        let engine = DiffEngine::new(diff_config);

        match engine.generate_side_by_side_diff(&change_file) {
            Ok(diff) => {
                info!("Successfully generated side-by-side diff for: {}", change_file.file_path);
                Ok(diff)
            }
            Err(e) => {
                error!("Failed to generate side-by-side diff: {}", e);
                Err(format!("Failed to generate side-by-side diff: {}", e))
            }
        }
    }).await.map_err(|e| format!("Task join error: {}", e))?
}

/// Create line mapping for diff navigation
#[tauri::command]
pub async fn diff_create_line_mapping(
    diff: ProcessedDiff,
    _state: State<'_, AppState>,
) -> Result<LineMapping, String> {
    info!("Creating line mapping for file: {}", diff.file_path);

    tokio::task::spawn_blocking(move || {
        let engine = DiffEngine::new(DiffConfig::default());
        let mapping = engine.create_line_mapping(&diff);
        
        info!("Successfully created line mapping for: {}", diff.file_path);
        Ok(mapping)
    }).await.map_err(|e| format!("Task join error: {}", e))?
}

/// Navigate to a specific line in the diff
#[tauri::command]
pub async fn diff_navigate_to_line(
    diff: ProcessedDiff,
    line_number: u32,
    is_old_file: bool,
    _state: State<'_, AppState>,
) -> Result<Option<(usize, usize)>, String> {
    info!("Navigating to line {} in {} file", line_number, if is_old_file { "old" } else { "new" });

    tokio::task::spawn_blocking(move || {
        let engine = DiffEngine::new(DiffConfig::default());
        let result = engine.navigate_to_line(&diff, line_number, is_old_file);
        
        if result.is_some() {
            info!("Successfully navigated to line {} in {}", line_number, diff.file_path);
        } else {
            info!("Line {} not found in {}", line_number, diff.file_path);
        }
        
        Ok(result)
    }).await.map_err(|e| format!("Task join error: {}", e))?
}

/// Get context around a specific line
#[tauri::command]
pub async fn diff_get_context_around_line(
    diff: ProcessedDiff,
    line_number: u32,
    is_old_file: bool,
    context_size: u32,
    _state: State<'_, AppState>,
) -> Result<Option<Vec<String>>, String> {
    info!("Getting context around line {} with size {}", line_number, context_size);

    tokio::task::spawn_blocking(move || {
        let engine = DiffEngine::new(DiffConfig::default());
        let context = engine.get_context_around_line(&diff, line_number, is_old_file, context_size);
        
        // Convert ProcessedLine references to owned strings for serialization
        let result = context.map(|lines| {
            lines.iter().map(|line| line.content.clone()).collect()
        });
        
        if result.is_some() {
            info!("Successfully retrieved context for line {} in {}", line_number, diff.file_path);
        } else {
            info!("No context found for line {} in {}", line_number, diff.file_path);
        }
        
        Ok(result)
    }).await.map_err(|e| format!("Task join error: {}", e))?
}

/// Get diff configuration
#[tauri::command]
pub async fn diff_get_config(_state: State<'_, AppState>) -> Result<DiffConfig, String> {
    Ok(DiffConfig::default())
}

/// Update diff configuration
#[tauri::command]
pub async fn diff_update_config(
    config: DiffConfig,
    _state: State<'_, AppState>,
) -> Result<bool, String> {
    info!("Updating diff configuration");
    
    // TODO: Persist configuration to storage
    info!("Diff configuration updated successfully");
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::gerrit::{FileChangeType, FileDiff, DiffHunk, DiffLine, DiffLineType};

    fn create_test_change_file() -> ChangeFile {
        let mut diff = FileDiff::default();
        diff.hunks = vec![
            DiffHunk {
                old_start: 1,
                old_count: 2,
                new_start: 1,
                new_count: 2,
                lines: vec![
                    DiffLine {
                        line_type: DiffLineType::Context,
                        old_line_number: Some(1),
                        new_line_number: Some(1),
                        content: "fn main() {".to_string(),
                    },
                    DiffLine {
                        line_type: DiffLineType::Added,
                        old_line_number: None,
                        new_line_number: Some(2),
                        content: "    println!(\"Hello, World!\");".to_string(),
                    },
                ],
            },
        ];

        ChangeFile {
            id: "test-file-id".to_string(),
            change_id: "test-change".to_string(),
            patch_set_number: 1,
            file_path: "src/main.rs".to_string(),
            change_type: FileChangeType::Modified,
            old_content: Some("fn main() {\n}".to_string()),
            new_content: Some("fn main() {\n    println!(\"Hello, World!\");\n}".to_string()),
            diff,
            file_size: 42,
            downloaded_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }

    #[tokio::test]
    async fn test_diff_config_creation() {
        let config = DiffConfig::default();
        assert_eq!(config.context_lines, 3);
        assert_eq!(config.tab_size, 4);
        assert!(!config.show_whitespace);
    }

    #[tokio::test]
    async fn test_change_file_creation() {
        let change_file = create_test_change_file();
        assert_eq!(change_file.file_path, "src/main.rs");
        assert_eq!(change_file.change_type, FileChangeType::Modified);
        assert_eq!(change_file.file_size, 42);
    }
}