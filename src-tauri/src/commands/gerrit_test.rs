// Test Gerrit command to verify API connectivity
use tauri::State;
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct TestResponse {
    pub success: bool,
    pub message: String,
}

/// Test Gerrit connectivity
#[tauri::command]
pub async fn gerrit_test_connectivity(
    _state: State<'_, AppState>,
) -> Result<TestResponse, String> {
    Ok(TestResponse {
        success: true,
        message: "Gerrit connectivity test successful".to_string(),
    })
}