// HyperReview Tauri Commands
// IPC handler functions exposed to the frontend

use crate::models::*;
use crate::AppState;
use crate::storage;
use tauri::State;

// Import modules
use crate::git;
use crate::analysis;
use crate::remote;

/// Opens a repository selection dialog
/// NOTE: This is deprecated. Dialogs should be opened from the frontend using @tauri-apps/plugin-dialog
#[tauri::command]
pub async fn open_repo_dialog() -> Result<Option<String>, String> {
    log::warn!("open_repo_dialog command is deprecated - frontend should use dialog API directly");
    Err("Use frontend dialog API instead".to_string())
}

/// Opens a repository selection dialog from frontend request
#[tauri::command]
pub async fn open_repo_dialog_frontend(_app: tauri::AppHandle) -> Result<Option<String>, String> {
    log::info!("Opening repository selection dialog from frontend");

    // Note: In Tauri v1, folder dialogs should be opened from the frontend
    // using the @tauri-apps/api dialog module
    // This backend command is kept for compatibility but frontend should handle dialogs
    log::warn!("Folder selection should be done from frontend in Tauri v1");
    Err("Use frontend dialog API to select folder".to_string())
}

/// Gets list of recently opened repositories
#[tauri::command]
pub async fn get_recent_repos(state: State<'_, AppState>) -> Result<Vec<Repo>, String> {
    log::info!("Getting list of recent repositories");

    let database = state.database.lock().unwrap();
    database.get_recent_repos(50)
        .map_err(|e| e.to_string())
}

/// Gets list of branches for the current repository
#[tauri::command]
pub async fn get_branches(state: State<'_, AppState>) -> Result<Vec<Branch>, String> {
    log::info!("Getting list of branches");

    let git_service = state.git_service.lock().unwrap();
    git_service.get_branches()
        .map_err(|e| e.to_string())
}

/// Loads a repository into memory
#[tauri::command]
pub async fn load_repo(path: String, state: State<'_, AppState>) -> Result<Repo, String> {
    log::info!("Loading repository: {}", path);

    let git_service = state.git_service.lock().unwrap();
    let repo = git_service.open_repo(&path)
        .map_err(|e| e.to_string())?;

    // Store repository metadata in database
    let database = state.database.lock().unwrap();
    database.mark_all_repos_inactive()
        .map_err(|e| e.to_string())?;
    database.store_repo_metadata(&repo)
        .map_err(|e| e.to_string())?;

    Ok(repo)
}

/// Gets file diff with analysis
#[tauri::command]
pub async fn get_file_diff(
    params: DiffParams,
    state: State<'_, AppState>,
) -> Result<Vec<DiffLine>, String> {
    log::info!("Getting file diff for: {}", params.file_path);

    // Check if repository is loaded
    let git_service = state.git_service.lock().unwrap();
    if !git_service.is_repo_loaded() {
        return Err("No repository loaded".to_string());
    }

    // Get repository from GitService
    let repository = match git_service.get_repository() {
        Some(repo) => repo,
        None => return Err("Repository not available".to_string()),
    };

    // Create diff engine
    let diff_engine = git::diff::DiffEngine::new(repository);

    // Compute diff
    let mut diff_lines = diff_engine.compute_file_diff(
        &params.file_path,
        params.old_commit.as_deref(),
        params.new_commit.as_deref(),
    )
    .map_err(|e| e.to_string())?;

    // Perform static analysis
    let analysis_engine = analysis::engine::AnalysisEngine::new();
    analysis_engine.analyze_diff_lines(&mut diff_lines, &params.file_path)
        .map_err(|e| e.to_string())?;

    // Cache the computed diff
    let cache_key = storage::cache::CacheManager::generate_diff_key(
        &params.file_path,
        params.old_commit.as_deref(),
        params.new_commit.as_deref(),
    );
    state.cache_manager.put_diff(cache_key, diff_lines.clone());

    log::info!("Successfully computed and analyzed diff for {} ({} lines)", params.file_path, diff_lines.len());
    Ok(diff_lines)
}

/// Adds a comment to a file
#[tauri::command]
pub async fn add_comment(
    params: CommentParams,
    state: State<'_, AppState>,
) -> Result<Comment, String> {
    log::info!("Adding comment to {} at line {}", params.file_path, params.line_number);

    // Create comment ID using UUID
    let comment_id = uuid::Uuid::new_v4().to_string();

    // Get current timestamp
    let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Create comment object
    let comment = Comment {
        id: comment_id,
        file_path: params.file_path.clone(),
        line_number: params.line_number,
        content: params.content.clone(),
        author: "current_user".to_string(), // TODO: Get from auth
        created_at: timestamp.clone(),
        updated_at: timestamp,
        status: CommentStatus::Draft,
        parent_id: None,
        tags: Vec::new(),
    };

    // Store comment in database
    let database = state.database.lock().unwrap();
    database.store_comment(&comment)
        .map_err(|e| e.to_string())?;

    log::info!("Successfully added comment {} to {} at line {}", comment.id, params.file_path, params.line_number);
    Ok(comment)
}

/// Updates an existing comment
#[tauri::command]
pub async fn update_comment(
    params: UpdateCommentParams,
    state: State<'_, AppState>,
) -> Result<Comment, String> {
    log::info!("Updating comment {}", params.comment_id);

    if params.content.trim().is_empty() {
        return Err("Comment content cannot be empty".to_string());
    }

    let database = state.database.lock().unwrap();

    // Get existing comment
    let existing_comment = database.get_comment(&params.comment_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Comment not found".to_string())?;

    // Update comment
    let updated_comment = Comment {
        content: params.content.trim().to_string(),
        updated_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        ..existing_comment
    };

    database.update_comment(&updated_comment)
        .map_err(|e| e.to_string())?;

    log::info!("Successfully updated comment {}", params.comment_id);
    Ok(updated_comment)
}

/// Deletes a comment
#[tauri::command]
pub async fn delete_comment(
    comment_id: String,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    log::info!("Deleting comment {}", comment_id);

    let database = state.database.lock().unwrap();
    database.delete_comment(&comment_id)
        .map_err(|e| e.to_string())?;

    log::info!("Successfully deleted comment {}", comment_id);
    Ok(true)
}

/// Gets comments for a specific file
#[tauri::command]
pub async fn get_comments(
    file_path: String,
    state: State<'_, AppState>,
) -> Result<Vec<Comment>, String> {
    log::info!("Getting comments for file: {}", file_path);

    let database = state.database.lock().unwrap();
    database.get_comments_for_file(&file_path)
        .map_err(|e| e.to_string())
}

/// Gets tasks for current review
#[tauri::command]
pub async fn get_tasks(state: State<'_, AppState>) -> Result<Vec<Task>, String> {
    log::info!("Getting tasks for current review");

    let database = state.database.lock().unwrap();
    database.get_tasks()
        .map_err(|e| e.to_string())
}

/// Gets review statistics
#[tauri::command]
pub async fn get_review_stats(state: State<'_, AppState>) -> Result<ReviewStats, String> {
    log::info!("Getting review statistics");

    let database = state.database.lock().unwrap();

    // Get tasks and calculate stats
    let tasks = database.get_tasks().map_err(|e| e.to_string())?;

    // For now, use placeholder values for file counts
    // In production, this would track actual files being reviewed
    let stats_aggregator = analysis::stats::StatsAggregator::new();
    let stats = stats_aggregator.calculate_from_db(
        tasks,
        Vec::new(), // TODO: Get comments from database
        10, // Placeholder total files
        5,  // Placeholder reviewed files
    );

    Ok(stats)
}

/// Gets quality gates status
#[tauri::command]
pub async fn get_quality_gates(state: State<'_, AppState>) -> Result<Vec<QualityGate>, String> {
    log::info!("Getting quality gates status");

    let git_service = state.git_service.lock().unwrap();
    let repo_path = git_service.get_current_path()
        .unwrap_or_else(|| ".".to_string());

    let remote_client = remote::client::RemoteClient::new();
    remote_client.check_quality_gates(&repo_path)
        .map_err(|e| e.to_string())
}

/// Gets review templates
#[tauri::command]
pub async fn get_review_templates(state: State<'_, AppState>) -> Result<Vec<ReviewTemplate>, String> {
    log::info!("Getting review templates");

    let database = state.database.lock().unwrap();
    database.get_review_templates()
        .map_err(|e| e.to_string())
}

/// Creates a new review template
#[tauri::command]
pub async fn create_template(
    name: String,
    content: String,
    state: State<'_, AppState>,
) -> Result<ReviewTemplate, String> {
    log::info!("Creating new review template: {}", name);

    let template_id = uuid::Uuid::new_v4().to_string();
    let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Extract placeholders from content (e.g., {{placeholder}})
    let placeholders: Vec<String> = regex::Regex::new(r"\{\{(\w+)\}\}")
        .map(|re| {
            re.captures_iter(&content)
                .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
                .collect()
        })
        .unwrap_or_default();

    let template = ReviewTemplate {
        id: template_id,
        name,
        content,
        placeholders,
        category: None,
        usage_count: 0,
        created_at: timestamp.clone(),
        updated_at: timestamp,
    };

    let database = state.database.lock().unwrap();
    database.store_review_template(&template)
        .map_err(|e| e.to_string())?;

    Ok(template)
}

/// Gets heatmap data
#[tauri::command]
pub async fn get_heatmap(state: State<'_, AppState>) -> Result<Vec<HeatmapItem>, String> {
    log::info!("Getting heatmap data");

    let git_service = state.git_service.lock().unwrap();
    let repo_path = git_service.get_current_path()
        .ok_or_else(|| "No repository loaded".to_string())?;

    log::info!("Generating heatmap for repo: {}", repo_path);
    let heatmap_generator = analysis::heatmap::HeatmapGenerator::new();
    let result = heatmap_generator.generate_from_git(&repo_path)
        .map_err(|e| e.to_string())?;

    log::info!("Heatmap generated with {} items", result.len());
    Ok(result)
}

/// Gets smart checklist
#[tauri::command]
pub async fn get_checklist(file_path: String) -> Result<Vec<ChecklistItem>, String> {
    log::info!("Getting checklist for file: {}", file_path);

    let checklist_engine = analysis::checklist::ChecklistEngine::new();
    let items = checklist_engine.generate_checklist(&file_path, None);

    Ok(items)
}

/// Gets git blame for a file
#[tauri::command]
pub async fn get_blame(
    file_path: String,
    commit: Option<String>,
    state: State<'_, AppState>,
) -> Result<BlameInfo, String> {
    log::info!("Getting blame for file: {}", file_path);

    let git_service = state.git_service.lock().unwrap();
    git_service.get_blame(&file_path, commit.as_deref())
        .map_err(|e| e.to_string())
}

/// Reads file content
#[tauri::command]
pub async fn read_file_content(
    params: FilePathParams,
    state: State<'_, AppState>,
) -> Result<String, String> {
    log::info!("Reading file content: {}", params.file_path);

    // Get repository path
    let git_service = state.git_service.lock().unwrap();
    let repo_path = git_service.get_current_path()
        .ok_or_else(|| "No repository loaded".to_string())?;

    log::info!("Repository path: {:?}", repo_path);

    // Construct full file path
    let full_path = std::path::Path::new(&repo_path).join(&params.file_path);

    log::info!("Full file path: {:?}", full_path);
    log::info!("File exists: {}", full_path.exists());
    log::info!("Is file: {}", full_path.is_file());

    // Read file content
    match std::fs::read_to_string(&full_path) {
        Ok(content) => {
            log::info!("Successfully read file, content length: {} bytes", content.len());
            Ok(content)
        },
        Err(e) => {
            log::error!("Failed to read file {}: {}", params.file_path, e);
            Err(format!("Failed to read file {}: {}", params.file_path, e))
        },
    }
}

/// Analyzes code complexity
#[tauri::command]
pub async fn analyze_complexity(file_path: String) -> Result<ComplexityMetrics, String> {
    log::info!("Analyzing complexity for file: {}", file_path);

    // Read file content
    let content = std::fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Simple complexity analysis based on code patterns
    let lines_of_code = content.lines().count() as u32;

    // Count functions/methods
    let function_count = regex::Regex::new(r"(pub\s+)?(async\s+)?fn\s+\w+")
        .map(|re| re.find_iter(&content).count())
        .unwrap_or(0) as u32;

    // Count classes/structs
    let class_count = regex::Regex::new(r"(pub\s+)?(struct|enum|impl)\s+\w+")
        .map(|re| re.find_iter(&content).count())
        .unwrap_or(0) as u32;

    // Estimate cyclomatic complexity (simplified: count branches)
    let cyclomatic_complexity = regex::Regex::new(r"\b(if|else|match|for|while|loop)\b")
        .map(|re| re.find_iter(&content).count())
        .unwrap_or(0) as u32 + 1;

    // Estimate cognitive complexity (simplified: nested structures)
    let cognitive_complexity = regex::Regex::new(r"(\{[^}]*\{)")
        .map(|re| re.find_iter(&content).count())
        .unwrap_or(0) as u32 + cyclomatic_complexity;

    Ok(ComplexityMetrics {
        file_path,
        cyclomatic_complexity,
        cognitive_complexity,
        lines_of_code,
        function_count,
        class_count,
    })
}

/// Scans for security issues
#[tauri::command]
pub async fn scan_security(file_path: String) -> Result<Vec<SecurityIssue>, String> {
    log::info!("Scanning security issues for file: {}", file_path);

    let content = std::fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    let mut issues = Vec::new();

    // Check for common security issues
    let patterns = vec![
        (r#"(?i)(password|passwd|pwd)\s*=\s*["'][^"']+["']"#, "Hardcoded password detected", Severity::Error),
        (r#"(?i)(api_key|apikey|api-key)\s*=\s*["'][^"']+["']"#, "Hardcoded API key detected", Severity::Error),
        (r#"(?i)(secret|token)\s*=\s*["'][^"']+["']"#, "Hardcoded secret/token detected", Severity::Error),
        (r"unwrap\(\)", "Use of unwrap() - consider proper error handling", Severity::Warning),
        (r"panic!\(", "Direct panic! call - consider error handling", Severity::Warning),
        (r"unsafe\s*\{", "Unsafe block detected - review carefully", Severity::Warning),
        (r"exec\s*\(|system\s*\(", "Potential command injection vulnerability", Severity::Error),
        (r"(?i)eval\s*\(", "Potential code injection via eval", Severity::Error),
    ];

    for (pattern, message, severity) in patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            for mat in re.find_iter(&content) {
                let line_number = content[..mat.start()].lines().count() as u32;
                issues.push(SecurityIssue {
                    severity: severity.clone(),
                    message: message.to_string(),
                    line_number: Some(line_number),
                    file_path: file_path.clone(),
                    rule_id: format!("SEC{:03}", issues.len() + 1),
                });
            }
        }
    }

    Ok(issues)
}

/// Submits review to external system
#[tauri::command]
pub async fn submit_review(
    system: String,
    review_data: serde_json::Value,
    state: State<'_, AppState>,
) -> Result<SubmitResult, String> {
    log::info!("Submitting review to system: {}", system);

    // Get comments from database for this review
    let database = state.database.lock().unwrap();

    // Parse review data
    let project_id = review_data.get("project_id")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let mr_id = review_data.get("merge_request_id")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    match system.as_str() {
        "gitlab" => {
            let client = remote::gitlab_client::GitLabClient::new("https://gitlab.com");
            client.submit_review(project_id, mr_id, Vec::new())
                .map_err(|e| e.to_string())
        }
        "gerrit" => {
            let client = remote::gerrit_client::GerritClient::new("https://gerrit.example.com");
            client.submit_review(project_id, "current", Vec::new(), None)
                .map_err(|e| e.to_string())
        }
        _ => Err(format!("Unsupported review system: {}", system)),
    }
}

/// Syncs repository with remote
#[tauri::command]
pub async fn sync_repo(state: State<'_, AppState>) -> Result<SyncResult, String> {
    log::info!("Syncing repository with remote");

    let git_service = state.git_service.lock().unwrap();
    let repo_path = git_service.get_current_path()
        .ok_or_else(|| "No repository loaded".to_string())?;

    // In production, would run git fetch/pull
    // For now, return mock result
    Ok(SyncResult {
        success: true,
        message: "Repository synced successfully".to_string(),
        commits_ahead: Some(0),
        commits_behind: Some(0),
    })
}

/// Searches repository
#[tauri::command]
pub async fn search(query: String, state: State<'_, AppState>) -> Result<Vec<SearchResult>, String> {
    log::info!("Searching for: {}", query);

    let git_service = state.git_service.lock().unwrap();
    let repo_path = git_service.get_current_path()
        .ok_or_else(|| "No repository loaded".to_string())?;

    let search_service = crate::search::service::SearchService::new()
        .with_repo(repo_path);

    search_service.search(&query, None)
        .map_err(|e| e.to_string())
}

/// Gets available commands
#[tauri::command]
pub async fn get_commands() -> Result<Vec<CommandInfo>, String> {
    log::info!("Getting available commands");

    // Return list of available commands for command palette
    Ok(vec![
        CommandInfo {
            id: "open_repo".to_string(),
            name: "Open Repository".to_string(),
            description: "Open a Git repository".to_string(),
            category: "Repository".to_string(),
        },
        CommandInfo {
            id: "get_branches".to_string(),
            name: "List Branches".to_string(),
            description: "List all branches in the repository".to_string(),
            category: "Repository".to_string(),
        },
        CommandInfo {
            id: "get_file_diff".to_string(),
            name: "View Diff".to_string(),
            description: "View file diff between commits".to_string(),
            category: "Review".to_string(),
        },
        CommandInfo {
            id: "add_comment".to_string(),
            name: "Add Comment".to_string(),
            description: "Add a review comment".to_string(),
            category: "Review".to_string(),
        },
        CommandInfo {
            id: "get_heatmap".to_string(),
            name: "View Heatmap".to_string(),
            description: "View file impact heatmap".to_string(),
            category: "Analysis".to_string(),
        },
        CommandInfo {
            id: "analyze_complexity".to_string(),
            name: "Analyze Complexity".to_string(),
            description: "Analyze code complexity".to_string(),
            category: "Analysis".to_string(),
        },
        CommandInfo {
            id: "scan_security".to_string(),
            name: "Security Scan".to_string(),
            description: "Scan for security issues".to_string(),
            category: "Analysis".to_string(),
        },
        CommandInfo {
            id: "search".to_string(),
            name: "Search".to_string(),
            description: "Search repository".to_string(),
            category: "Navigation".to_string(),
        },
        CommandInfo {
            id: "submit_review".to_string(),
            name: "Submit Review".to_string(),
            description: "Submit review to external system".to_string(),
            category: "External".to_string(),
        },
    ])
}

/// Gets tags
#[tauri::command]
pub async fn get_tags(state: State<'_, AppState>) -> Result<Vec<Tag>, String> {
    log::info!("Getting tags");

    let git_service = state.git_service.lock().unwrap();
    let repo_path = git_service.get_current_path()
        .unwrap_or_else(|| ".".to_string());

    let database = state.database.lock().unwrap();
    database.get_tags(&repo_path)
        .map_err(|e| e.to_string())
}

/// Creates a new tag
#[tauri::command]
pub async fn create_tag(
    label: String,
    color: String,
    state: State<'_, AppState>,
) -> Result<Tag, String> {
    log::info!("Creating new tag: {}", label);

    let tag_id = uuid::Uuid::new_v4().to_string();
    let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let tag = Tag {
        id: tag_id,
        label,
        color,
        description: None,
        usage_count: 0,
        created_at: timestamp.clone(),
        updated_at: timestamp,
    };

    let git_service = state.git_service.lock().unwrap();
    let repo_path = git_service.get_current_path()
        .unwrap_or_else(|| ".".to_string());

    let database = state.database.lock().unwrap();
    database.store_tag(&tag, &repo_path)
        .map_err(|e| e.to_string())?;

    Ok(tag)
}
