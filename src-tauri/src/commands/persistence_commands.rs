use serde::{Deserialize, Serialize};
use tauri::State;
use rusqlite::{Connection, params};
use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSettings {
    pub font_size: Option<i32>,
    pub enable_ligatures: Option<bool>,
    pub enable_vim: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoSelection {
    pub repo_path: Option<String>,
    pub branch_base: Option<String>,
    pub branch_head: Option<String>,
}

/// Get user settings
#[tauri::command]
pub async fn get_user_settings(
    state: State<'_, AppState>,
) -> Result<UserSettings, String> {
    let db_path = "hyper_review.db";
    let conn = Connection::open(db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    use crate::storage::settings::SettingsPersistence;
    let settings_persist = SettingsPersistence::new(conn);
    let settings = settings_persist.get_settings()
        .map_err(|e| format!("Failed to get settings: {}", e))?;

    Ok(UserSettings {
        font_size: settings.font_size,
        enable_ligatures: settings.enable_ligatures,
        enable_vim: settings.enable_vim,
    })
}

/// Save user setting
#[tauri::command]
pub async fn save_user_setting(
    state: State<'_, AppState>,
    key: String,
    value: String,
) -> Result<(), String> {
    let db_path = "hyper_review.db";
    let conn = Connection::open(db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    use crate::storage::settings::SettingsPersistence;
    let settings_persist = SettingsPersistence::new(conn);
    settings_persist.save_setting(&key, &value)
        .map_err(|e| format!("Failed to save setting: {}", e))?;

    Ok(())
}

/// Get repo selection
#[tauri::command]
pub async fn get_repo_selection(
    state: State<'_, AppState>,
) -> Result<RepoSelection, String> {
    let db_path = "hyper_review.db";
    let conn = Connection::open(db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS repo_selection (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            repo_path TEXT,
            branch_base TEXT,
            branch_head TEXT
        )",
        [],
    ).map_err(|e| format!("Failed to create table: {}", e))?;

    let mut stmt = conn.prepare(
        "SELECT repo_path, branch_base, branch_head FROM repo_selection WHERE id = 1"
    ).map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let mut selection = RepoSelection {
        repo_path: None,
        branch_base: None,
        branch_head: None,
    };

    if let Ok(rows) = stmt.query_map([], |row| {
        Ok((
            row.get::<_, Option<String>>(0)?,
            row.get::<_, Option<String>>(1)?,
            row.get::<_, Option<String>>(2)?,
        ))
    }) {
        for row in rows {
            if let Ok((repo_path, branch_base, branch_head)) = row {
                selection.repo_path = repo_path;
                selection.branch_base = branch_base;
                selection.branch_head = branch_head;
            }
        }
    }

    Ok(selection)
}

/// Save repo selection
#[tauri::command]
pub async fn save_repo_selection(
    state: State<'_, AppState>,
    repo_path: Option<String>,
    branch_base: Option<String>,
    branch_head: Option<String>,
) -> Result<(), String> {
    let db_path = "hyper_review.db";
    let conn = Connection::open(db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS repo_selection (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            repo_path TEXT,
            branch_base TEXT,
            branch_head TEXT
        )",
        [],
    ).map_err(|e| format!("Failed to create table: {}", e))?;

    conn.execute(
        "INSERT OR REPLACE INTO repo_selection (id, repo_path, branch_base, branch_head) 
         VALUES (1, ?1, ?2, ?3)",
        params![
            repo_path.as_deref(),
            branch_base.as_deref(),
            branch_head.as_deref(),
        ],
    ).map_err(|e| format!("Failed to save repo selection: {}", e))?;

    Ok(())
}

/// Clear repo selection
#[tauri::command]
pub async fn clear_repo_selection(
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db_path = "hyper_review.db";
    let conn = Connection::open(db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    conn.execute(
        "DELETE FROM repo_selection WHERE id = 1",
        [],
    ).map_err(|e| format!("Failed to clear repo selection: {}", e))?;

    Ok(())
}
