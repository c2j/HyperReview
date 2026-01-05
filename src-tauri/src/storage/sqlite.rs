// SQLite database operations
// Local storage for review metadata

use crate::models::{Repo, Comment, CommentStatus};
use crate::models::gerrit::{GerritInstance, GerritChange, ConnectionStatus, ChangeStatus, ImportStatus, ConflictStatus};
use crate::errors::HyperReviewError;
use rusqlite::{Connection, Result, params};
use serde_json;
use uuid::Uuid;
use chrono::Utc;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self, rusqlite::Error> {
        let conn = Connection::open(path)?;
        Ok(Self { conn })
    }

    // TODO: Create database schema (Task T005)
    pub fn init_schema(&self) -> Result<(), rusqlite::Error> {
        // Create repos table
        self.conn.execute_batch("
            CREATE TABLE IF NOT EXISTS repos (
                path TEXT PRIMARY KEY,
                current_branch TEXT NOT NULL,
                last_opened TEXT NOT NULL,
                head_commit TEXT NOT NULL,
                remote_url TEXT,
                is_active INTEGER NOT NULL DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS branches (
                name TEXT NOT NULL,
                repo_path TEXT NOT NULL,
                is_current INTEGER NOT NULL DEFAULT 0,
                is_remote INTEGER NOT NULL DEFAULT 0,
                upstream TEXT,
                last_commit TEXT NOT NULL,
                last_commit_message TEXT,
                last_commit_author TEXT NOT NULL,
                last_commit_date TEXT NOT NULL,
                PRIMARY KEY (name, repo_path)
            );

            CREATE TABLE IF NOT EXISTS comments (
                id TEXT PRIMARY KEY,
                file_path TEXT NOT NULL,
                line_number INTEGER NOT NULL,
                content TEXT NOT NULL,
                author TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                status TEXT NOT NULL,
                parent_id TEXT,
                tags TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_comments_file_path ON comments(file_path);
            CREATE INDEX IF NOT EXISTS idx_comments_created_at ON comments(created_at);
            CREATE INDEX IF NOT EXISTS idx_comments_author ON comments(author);

            CREATE TABLE IF NOT EXISTS tags (
                id TEXT NOT NULL,
                repo_path TEXT NOT NULL,
                label TEXT NOT NULL,
                color TEXT NOT NULL,
                description TEXT,
                usage_count INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                PRIMARY KEY (id, repo_path)
            );

            CREATE INDEX IF NOT EXISTS idx_tags_label ON tags(label);

            CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL,
                priority INTEGER NOT NULL,
                assignee TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                due_date TEXT,
                metadata TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status);
            CREATE INDEX IF NOT EXISTS idx_tasks_assignee ON tasks(assignee);
            CREATE INDEX IF NOT EXISTS idx_tasks_due_date ON tasks(due_date);

            CREATE TABLE IF NOT EXISTS checklist_templates (
                id TEXT PRIMARY KEY,
                description TEXT NOT NULL,
                category TEXT NOT NULL,
                severity TEXT NOT NULL,
                applicable_file_types TEXT NOT NULL,
                applicable_patterns TEXT NOT NULL,
                is_auto_checkable INTEGER NOT NULL DEFAULT 0
            );

            CREATE INDEX IF NOT EXISTS idx_checklist_category ON checklist_templates(category);

            CREATE TABLE IF NOT EXISTS review_templates (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                content TEXT NOT NULL,
                placeholders TEXT NOT NULL,
                category TEXT,
                usage_count INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_review_templates_category ON review_templates(category);

            CREATE TABLE IF NOT EXISTS search_cache (
                id TEXT PRIMARY KEY,
                query TEXT NOT NULL,
                results TEXT NOT NULL,
                created_at TEXT NOT NULL,
                expires_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS review_guides (
                id TEXT PRIMARY KEY,
                category TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT NOT NULL,
                severity TEXT NOT NULL,
                reference_url TEXT,
                applicable_extensions TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_review_guides_category ON review_guides(category);
            CREATE INDEX IF NOT EXISTS idx_review_guides_severity ON review_guides(severity);

            -- Local Tasks table
            CREATE TABLE IF NOT EXISTS local_tasks (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL,
                priority INTEGER DEFAULT 1,
                assignee TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                due_date TEXT,
                task_type TEXT,
                metadata TEXT
            );

            CREATE INDEX IF NOT EXISTS idx_local_tasks_status ON local_tasks(status);
            CREATE INDEX IF NOT EXISTS idx_local_tasks_type ON local_tasks(task_type);

            -- Local Task Files table
            CREATE TABLE IF NOT EXISTS local_task_files (
                id TEXT PRIMARY KEY,
                task_id TEXT NOT NULL,
                path TEXT NOT NULL,
                name TEXT NOT NULL,
                status TEXT NOT NULL,
                review_status TEXT,
                review_comment TEXT,
                created_at TEXT NOT NULL,
                FOREIGN KEY (task_id) REFERENCES local_tasks(id) ON DELETE CASCADE
            );

            CREATE INDEX IF NOT EXISTS idx_local_task_files_task_id ON local_task_files(task_id);

            -- File Review Comments table - stores all review comments for each file
            CREATE TABLE IF NOT EXISTS file_review_comments (
                id TEXT PRIMARY KEY,
                task_id TEXT NOT NULL,
                file_id TEXT NOT NULL,
                review_status TEXT NOT NULL,
                review_comment TEXT NOT NULL,
                submitted_by TEXT,
                submitted_at TEXT NOT NULL,
                FOREIGN KEY (task_id) REFERENCES local_tasks(id) ON DELETE CASCADE
            );

            CREATE INDEX IF NOT EXISTS idx_file_review_comments_task_id ON file_review_comments(task_id);
            CREATE INDEX IF NOT EXISTS idx_file_review_comments_file_id ON file_review_comments(file_id);
        ")?;

        // Migration: Add missing columns to existing tables
        // For local_task_files table, add review_status and review_comment if they don't exist
        // Use a simpler check that queries the count directly
        let check_column = |col_name: &str| -> rusqlite::Result<bool> {
            let mut stmt = self.conn.prepare(&format!(
                "SELECT COUNT(*) FROM pragma_table_info('local_task_files') WHERE name='{}'",
                col_name
            ))?;
            let count: i64 = stmt.query_row([], |row| row.get(0))?;
            Ok(count > 0)
        };

        if !check_column("review_status")? {
            log::info!("Adding review_status column to local_task_files table");
            self.conn.execute(
                "ALTER TABLE local_task_files ADD COLUMN review_status TEXT",
                []
            )?;
        }

        if !check_column("review_comment")? {
            log::info!("Adding review_comment column to local_task_files table");
            self.conn.execute(
                "ALTER TABLE local_task_files ADD COLUMN review_comment TEXT",
                []
            )?;
        }

        // Create the index after ensuring the column exists
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_local_task_files_review_status ON local_task_files(review_status)",
            []
        )?;

        Ok(())
    }

    /// Store or update repository metadata
    pub fn store_repo_metadata(&self, repo: &Repo) -> Result<(), rusqlite::Error> {
        log::info!("Storing repository metadata for: {}", repo.path);

        self.conn.execute(
            "INSERT OR REPLACE INTO repos (path, current_branch, last_opened, head_commit, remote_url, is_active)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (
                &repo.path,
                &repo.current_branch,
                &repo.last_opened,
                &repo.head_commit,
                repo.remote_url.as_deref(),
                if repo.is_active { 1 } else { 0 },
            ),
        )?;

        Ok(())
    }

    /// Get recently opened repositories
    pub fn get_recent_repos(&self, limit: usize) -> Result<Vec<Repo>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT path, current_branch, last_opened, head_commit, remote_url, is_active
             FROM repos
             ORDER BY last_opened DESC
             LIMIT ?1"
        )?;

        let repo_iter = stmt.query_map([limit], |row| {
            Ok(Repo {
                path: row.get("path")?,
                current_branch: row.get("current_branch")?,
                last_opened: row.get("last_opened")?,
                head_commit: row.get("head_commit")?,
                remote_url: row.get("remote_url")?,
                is_active: row.get("is_active")?,
            })
        })?;

        let mut repos = Vec::new();
        for repo_result in repo_iter {
            repos.push(repo_result?);
        }

        Ok(repos)
    }

    /// Mark all repos as inactive
    pub fn mark_all_repos_inactive(&self) -> Result<(), rusqlite::Error> {
        self.conn.execute("UPDATE repos SET is_active = 0", ())?;
        Ok(())
    }

    /// Store a comment in the database
    pub fn store_comment(&self, comment: &Comment) -> Result<(), rusqlite::Error> {
        log::info!("Storing comment for file: {}", comment.file_path);

        self.conn.execute(
            "INSERT INTO comments (id, file_path, line_number, content, author, created_at, updated_at, status, parent_id, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            (
                &comment.id,
                &comment.file_path,
                comment.line_number,
                &comment.content,
                &comment.author,
                &comment.created_at,
                &comment.updated_at,
                &format!("{:?}", comment.status),
                comment.parent_id.as_deref(),
                &comment.tags.join(","),
            ),
        )?;

        Ok(())
    }

    /// Get comments for a specific file
    pub fn get_comments_for_file(&self, file_path: &str) -> Result<Vec<Comment>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT id, file_path, line_number, content, author, created_at, updated_at, status, parent_id, tags
             FROM comments
             WHERE file_path = ?1
             ORDER BY line_number ASC, created_at ASC"
        )?;

        let comment_iter = stmt.query_map([file_path], |row| {
            Ok(Comment {
                id: row.get("id")?,
                file_path: row.get("file_path")?,
                line_number: row.get("line_number")?,
                content: row.get("content")?,
                author: row.get("author")?,
                created_at: row.get("created_at")?,
                updated_at: row.get("updated_at")?,
                status: match row.get::<_, String>("status")?.as_str() {
                    "Draft" => CommentStatus::Draft,
                    "Submitted" => CommentStatus::Submitted,
                    "Rejected" => CommentStatus::Rejected,
                    _ => CommentStatus::Submitted,
                },
                parent_id: row.get("parent_id")?,
                tags: if let Ok(tags_str) = row.get::<_, String>("tags") {
                    if tags_str.is_empty() {
                        Vec::new()
                    } else {
                        tags_str.split(',').map(|s| s.to_string()).collect()
                    }
                } else {
                    Vec::new()
                },
            })
        })?;

        let mut comments = Vec::new();
        for comment_result in comment_iter {
            comments.push(comment_result?);
        }

        Ok(comments)
    }

    /// Get a single comment by ID
    pub fn get_comment(&self, comment_id: &str) -> Result<Option<Comment>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT id, file_path, line_number, content, author, created_at, updated_at, status, parent_id, tags
             FROM comments
             WHERE id = ?1"
        )?;

        let mut comment_iter = stmt.query_map([comment_id], |row| {
            Ok(Comment {
                id: row.get("id")?,
                file_path: row.get("file_path")?,
                line_number: row.get("line_number")?,
                content: row.get("content")?,
                author: row.get("author")?,
                created_at: row.get("created_at")?,
                updated_at: row.get("updated_at")?,
                status: match row.get::<_, String>("status")?.as_str() {
                    "Draft" => CommentStatus::Draft,
                    "Submitted" => CommentStatus::Submitted,
                    "Rejected" => CommentStatus::Rejected,
                    _ => CommentStatus::Draft,
                },
                parent_id: row.get("parent_id")?,
                tags: row.get::<_, String>("tags")?
                    .split(',')
                    .filter(|s| !s.is_empty())
                    .map(String::from)
                    .collect(),
            })
        })?;

        match comment_iter.next() {
            Some(comment_result) => comment_result.map(Some),
            None => Ok(None),
        }
    }

    /// Update an existing comment
    pub fn update_comment(&self, comment: &Comment) -> Result<(), rusqlite::Error> {
        log::info!("Updating comment: {}", comment.id);

        self.conn.execute(
            "UPDATE comments
             SET content = ?1, updated_at = ?2, status = ?3
             WHERE id = ?4",
            (
                &comment.content,
                &comment.updated_at,
                &format!("{:?}", comment.status),
                &comment.id,
            ),
        )?;

        Ok(())
    }

    /// Delete a comment by ID
    pub fn delete_comment(&self, comment_id: &str) -> Result<(), rusqlite::Error> {
        log::info!("Deleting comment: {}", comment_id);

        self.conn.execute("DELETE FROM comments WHERE id = ?1", [comment_id])?;

        Ok(())
    }

    // TODO: Implement task CRUD operations (Task T039)
    pub fn manage_tasks(&self) -> Result<(), rusqlite::Error> {
        unimplemented!()
    }

    // TODO: Implement review template management (Task T042)
    pub fn manage_templates(&self) -> Result<(), rusqlite::Error> {
        unimplemented!()
    }

    // ===== Task CRUD Operations (Task T039) =====

    /// Store a task in the database
    pub fn store_task(&self, task: &crate::models::Task) -> Result<(), rusqlite::Error> {
        log::info!("Storing task: {}", task.id);

        let metadata_json = serde_json::to_string(&task.metadata).unwrap_or_else(|_| "{}".to_string());
        let status_str = format!("{:?}", task.status);

        self.conn.execute(
            "INSERT OR REPLACE INTO tasks (id, title, description, status, priority, assignee, created_at, updated_at, due_date, metadata)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            (
                &task.id,
                &task.title,
                task.description.as_deref(),
                &status_str,
                task.priority,
                task.assignee.as_deref(),
                &task.created_at,
                &task.updated_at,
                task.due_date.as_deref(),
                &metadata_json,
            ),
        )?;

        Ok(())
    }

    /// Get all tasks for current repository
    pub fn get_tasks(&self) -> Result<Vec<crate::models::Task>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, status, priority, assignee, created_at, updated_at, due_date, metadata
             FROM tasks
             ORDER BY priority DESC, created_at DESC"
        )?;

        let task_iter = stmt.query_map([], |row| {
            let metadata_str: String = row.get("metadata")?;
            let metadata: std::collections::HashMap<String, String> = serde_json::from_str(&metadata_str).unwrap_or_default();

            Ok(crate::models::Task {
                id: row.get("id")?,
                title: row.get("title")?,
                description: row.get("description")?,
                status: match row.get::<_, String>("status")?.as_str() {
                    "Active" => crate::models::TaskStatus::Active,
                    "Pending" => crate::models::TaskStatus::Pending,
                    "Completed" => crate::models::TaskStatus::Completed,
                    "Blocked" => crate::models::TaskStatus::Blocked,
                    _ => crate::models::TaskStatus::Pending,
                },
                priority: row.get("priority")?,
                assignee: row.get("assignee")?,
                created_at: row.get("created_at")?,
                updated_at: row.get("updated_at")?,
                due_date: row.get("due_date")?,
                metadata,
                task_type: None,
                files: vec![],
            })
        })?;

        let mut tasks = Vec::new();
        for task_result in task_iter {
            tasks.push(task_result?);
        }

        Ok(tasks)
    }

    /// Update a task
    pub fn update_task(&self, task: &crate::models::Task) -> Result<(), rusqlite::Error> {
        log::info!("Updating task: {}", task.id);

        let metadata_json = serde_json::to_string(&task.metadata).unwrap_or_else(|_| "{}".to_string());
        let status_str = format!("{:?}", task.status);

        self.conn.execute(
            "UPDATE tasks SET title = ?1, description = ?2, status = ?3, priority = ?4, assignee = ?5, updated_at = ?6, due_date = ?7, metadata = ?8
             WHERE id = ?9",
            (
                &task.title,
                task.description.as_deref(),
                &status_str,
                task.priority,
                task.assignee.as_deref(),
                &task.updated_at,
                task.due_date.as_deref(),
                &metadata_json,
                &task.id,
            ),
        )?;

        Ok(())
    }

    /// Delete a task by ID
    pub fn delete_task(&self, task_id: &str) -> Result<(), rusqlite::Error> {
        log::info!("Deleting task: {}", task_id);
        self.conn.execute("DELETE FROM tasks WHERE id = ?1", [task_id])?;
        Ok(())
    }

    // ===== Review Template Management (Task T042) =====

    /// Store a review template
    pub fn store_review_template(&self, template: &crate::models::ReviewTemplate) -> Result<(), rusqlite::Error> {
        log::info!("Storing review template: {}", template.id);

        let placeholders_json = serde_json::to_string(&template.placeholders).unwrap_or_else(|_| "[]".to_string());

        self.conn.execute(
            "INSERT OR REPLACE INTO review_templates (id, name, content, placeholders, category, usage_count, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            (
                &template.id,
                &template.name,
                &template.content,
                &placeholders_json,
                template.category.as_deref(),
                template.usage_count,
                &template.created_at,
                &template.updated_at,
            ),
        )?;

        Ok(())
    }

    /// Get all review templates
    pub fn get_review_templates(&self) -> Result<Vec<crate::models::ReviewTemplate>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, content, placeholders, category, usage_count, created_at, updated_at
             FROM review_templates
             ORDER BY usage_count DESC, name ASC"
        )?;

        let template_iter = stmt.query_map([], |row| {
            let placeholders_str: String = row.get("placeholders")?;
            let placeholders: Vec<String> = serde_json::from_str(&placeholders_str).unwrap_or_default();

            Ok(crate::models::ReviewTemplate {
                id: row.get("id")?,
                name: row.get("name")?,
                content: row.get("content")?,
                placeholders,
                category: row.get("category")?,
                usage_count: row.get("usage_count")?,
                created_at: row.get("created_at")?,
                updated_at: row.get("updated_at")?,
            })
        })?;

        let mut templates = Vec::new();
        for template_result in template_iter {
            templates.push(template_result?);
        }

        Ok(templates)
    }

    /// Increment template usage count
    pub fn increment_template_usage(&self, template_id: &str) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            "UPDATE review_templates SET usage_count = usage_count + 1 WHERE id = ?1",
            [template_id],
        )?;
        Ok(())
    }

    // ===== Tag CRUD Operations (Task T080) =====

    /// Store a tag
    pub fn store_tag(&self, tag: &crate::models::Tag, repo_path: &str) -> Result<(), rusqlite::Error> {
        log::info!("Storing tag: {}", tag.id);

        self.conn.execute(
            "INSERT OR REPLACE INTO tags (id, repo_path, label, color, description, usage_count, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            (
                &tag.id,
                repo_path,
                &tag.label,
                &tag.color,
                tag.description.as_deref(),
                tag.usage_count,
                &tag.created_at,
                &tag.updated_at,
            ),
        )?;

        Ok(())
    }

    /// Get all tags for a repository
    pub fn get_tags(&self, repo_path: &str) -> Result<Vec<crate::models::Tag>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT id, label, color, description, usage_count, created_at, updated_at
             FROM tags
             WHERE repo_path = ?1
             ORDER BY usage_count DESC, label ASC"
        )?;

        let tag_iter = stmt.query_map([repo_path], |row| {
            Ok(crate::models::Tag {
                id: row.get("id")?,
                label: row.get("label")?,
                color: row.get("color")?,
                description: row.get("description")?,
                usage_count: row.get("usage_count")?,
                created_at: row.get("created_at")?,
                updated_at: row.get("updated_at")?,
            })
        })?;

        let mut tags = Vec::new();
        for tag_result in tag_iter {
            tags.push(tag_result?);
        }

        Ok(tags)
    }

    /// Delete a tag
    pub fn delete_tag(&self, tag_id: &str, repo_path: &str) -> Result<(), rusqlite::Error> {
        log::info!("Deleting tag: {}", tag_id);
        self.conn.execute(
            "DELETE FROM tags WHERE id = ?1 AND repo_path = ?2",
            [tag_id, repo_path],
        )?;
        Ok(())
    }

    /// Increment tag usage count
    pub fn increment_tag_usage(&self, tag_id: &str) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            "UPDATE tags SET usage_count = usage_count + 1 WHERE id = ?1",
            [tag_id],
        )?;
        Ok(())
    }

    // ===== Review Guide Operations =====

    /// Store a review guide
    pub fn store_review_guide(&self, guide: &crate::models::ReviewGuideItem) -> Result<(), rusqlite::Error> {
        log::info!("Storing review guide: {}", guide.id);

        let extensions_json = serde_json::to_string(&guide.applicable_extensions).unwrap_or_else(|_| "[]".to_string());
        let category_str = format!("{:?}", guide.category);
        let severity_str = format!("{:?}", guide.severity);

        self.conn.execute(
            "INSERT OR REPLACE INTO review_guides (id, category, title, description, severity, reference_url, applicable_extensions, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            (
                &guide.id,
                &category_str,
                &guide.title,
                &guide.description,
                &severity_str,
                guide.reference_url.as_deref(),
                &extensions_json,
                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            ),
        )?;

        Ok(())
    }

    /// Get all review guides
    pub fn get_review_guides(&self) -> Result<Vec<crate::models::ReviewGuideItem>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT id, category, title, description, severity, reference_url, applicable_extensions
             FROM review_guides
             ORDER BY category ASC, severity DESC, title ASC"
        )?;

        let guide_iter = stmt.query_map([], |row| {
            let extensions_str: String = row.get("applicable_extensions")?;
            let applicable_extensions: Vec<String> = serde_json::from_str(&extensions_str).unwrap_or_default();

            Ok(crate::models::ReviewGuideItem {
                id: row.get("id")?,
                // 直接使用字符串类别，支持中文
                category: row.get::<_, String>("category")?,
                title: row.get("title")?,
                description: row.get("description")?,
                severity: match row.get::<_, String>("severity")?.as_str() {
                    "High" => crate::models::ReviewGuideSeverity::High,
                    "Medium" => crate::models::ReviewGuideSeverity::Medium,
                    "Low" => crate::models::ReviewGuideSeverity::Low,
                    _ => crate::models::ReviewGuideSeverity::Low,
                },
                reference_url: row.get("reference_url")?,
                applicable_extensions,
            })
        })?;

        let mut guides = Vec::new();
        for guide_result in guide_iter {
            guides.push(guide_result?);
        }

        Ok(guides)
    }

    /// Delete a review guide
    pub fn delete_review_guide(&self, guide_id: &str) -> Result<(), rusqlite::Error> {
        log::info!("Deleting review guide: {}", guide_id);
        self.conn.execute("DELETE FROM review_guides WHERE id = ?1", [guide_id])?;
        Ok(())
    }

    // ===== Local Task Operations =====

    /// Create a new local task
    pub fn create_local_task(
        &self,
        title: &str,
        task_type: &str,
        file_paths: &[String],
    ) -> Result<crate::models::Task, rusqlite::Error> {
        log::info!("Creating local task: {} with {} files", title, file_paths.len());

        let task_id = uuid::Uuid::new_v4().to_string();
        let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        // Insert task
        self.conn.execute(
            "INSERT INTO local_tasks (id, title, status, priority, created_at, updated_at, task_type)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (
                &task_id,
                title,
                "active",
                1i32,
                &timestamp,
                &timestamp,
                task_type,
            ),
        )?;

        // Insert files
        for file_path in file_paths {
            let file_id = uuid::Uuid::new_v4().to_string();
            let file_name = std::path::Path::new(file_path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown");

            self.conn.execute(
                "INSERT INTO local_task_files (id, task_id, path, name, status, review_status, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                (
                    &file_id,
                    &task_id,
                    file_path,
                    file_name,
                    "modified",
                    "pending",  // Default review status
                    &timestamp,
                ),
            )?;
        }

        // Return the created task with files
        Ok(crate::models::Task {
            id: task_id.clone(),
            title: title.to_string(),
            description: None,
            status: crate::models::TaskStatus::Active,
            priority: 1,
            assignee: None,
            created_at: timestamp.clone(),
            updated_at: timestamp,
            due_date: None,
            metadata: std::collections::HashMap::new(),
            task_type: match task_type {
                "code" => Some(crate::models::TaskType::Code),
                "sql" => Some(crate::models::TaskType::Sql),
                "security" => Some(crate::models::TaskType::Security),
                _ => None,
            },
            files: file_paths
                .iter()
                .enumerate()
                .map(|(i, path)| {
                    let name = std::path::Path::new(path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("Unknown")
                        .to_string();
                    crate::models::TaskFile {
                        id: format!("{}-{}", task_id, i),
                        path: path.clone(),
                        name,
                        status: crate::models::FileStatus::Modified,
                        review_status: Some(crate::models::FileReviewStatus::Pending),
                        review_comment: None,
                    }
                })
                .collect(),
        })
    }

    /// Get all local tasks
    pub fn get_local_tasks(&self) -> Result<Vec<crate::models::Task>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, status, task_type FROM local_tasks
             ORDER BY created_at DESC"
        )?;

        let task_iter = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>("id")?,
                row.get::<_, String>("title")?,
                row.get::<_, String>("status")?,
                row.get::<_, Option<String>>("task_type")?,
            ))
        })?;

        let mut tasks = Vec::new();
        for task_result in task_iter {
            let (task_id, title, status, task_type) = task_result?;

            // Get files for this task
            let mut file_stmt = self.conn.prepare(
                "SELECT id, path, name, status, review_status, review_comment FROM local_task_files WHERE task_id = ?1"
            )?;

            let files: Vec<crate::models::TaskFile> = file_stmt
                .query_map([&task_id], |row| {
                    let review_status_str: Option<String> = row.get("review_status")?;
                    Ok(crate::models::TaskFile {
                        id: row.get("id")?,
                        path: row.get("path")?,
                        name: row.get("name")?,
                        status: match row.get::<_, String>("status")?.as_str() {
                            "modified" => crate::models::FileStatus::Modified,
                            "added" => crate::models::FileStatus::Added,
                            "deleted" => crate::models::FileStatus::Deleted,
                            _ => crate::models::FileStatus::Modified,
                        },
                        review_status: review_status_str.and_then(|s| match s.as_str() {
                            "pending" => Some(crate::models::FileReviewStatus::Pending),
                            "approved" => Some(crate::models::FileReviewStatus::Approved),
                            "concern" => Some(crate::models::FileReviewStatus::Concern),
                            "must_change" => Some(crate::models::FileReviewStatus::MustChange),
                            "question" => Some(crate::models::FileReviewStatus::Question),
                            _ => Some(crate::models::FileReviewStatus::Pending),
                        }),
                        review_comment: row.get("review_comment")?,
                    })
                })?
                .collect::<Result<Vec<_>, _>>()?;

            tasks.push(crate::models::Task {
                id: task_id.clone(),
                title,
                description: None,
                status: match status.as_str() {
                    "active" => crate::models::TaskStatus::Active,
                    "pending" => crate::models::TaskStatus::Pending,
                    "completed" => crate::models::TaskStatus::Completed,
                    "blocked" => crate::models::TaskStatus::Blocked,
                    _ => crate::models::TaskStatus::Pending,
                },
                priority: 1,
                assignee: None,
                created_at: String::new(),
                updated_at: String::new(),
                due_date: None,
                metadata: std::collections::HashMap::new(),
                task_type: match task_type.as_deref() {
                    Some("code") => Some(crate::models::TaskType::Code),
                    Some("sql") => Some(crate::models::TaskType::Sql),
                    Some("security") => Some(crate::models::TaskType::Security),
                    _ => None,
                },
                files,
            });
        }

        Ok(tasks)
    }

    /// Delete a local task
    pub fn delete_local_task(&self, task_id: &str) -> Result<(), rusqlite::Error> {
        log::info!("Deleting local task: {}", task_id);
        self.conn.execute("DELETE FROM local_tasks WHERE id = ?1", [task_id])?;
        // Files will be deleted automatically due to FOREIGN KEY
        Ok(())
    }

    /// Update file review status
    pub fn update_file_review_status(
        &self,
        task_id: &str,
        file_id: &str,
        review_status: &str,
        review_comment: Option<&str>,
        submitted_by: Option<&str>,
    ) -> Result<(), rusqlite::Error> {
        log::info!("Updating review status for file {} in task {} to {}", file_id, task_id, review_status);

        // Update the current status in local_task_files
        self.conn.execute(
            "UPDATE local_task_files
             SET review_status = ?1, review_comment = ?2
             WHERE id = ?3 AND task_id = ?4",
            (review_status, review_comment.unwrap_or(""), file_id, task_id),
        )?;

        // Also insert into comment history
        let comment_id = uuid::Uuid::new_v4().to_string();
        let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        self.conn.execute(
            "INSERT INTO file_review_comments (id, task_id, file_id, review_status, review_comment, submitted_by, submitted_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (
                &comment_id,
                task_id,
                file_id,
                review_status,
                review_comment.unwrap_or(""),
                submitted_by.unwrap_or("Anonymous"),
                &timestamp,
            ),
        )?;

        Ok(())
    }

    /// Get all review comments for a file
    pub fn get_file_review_comments(
        &self,
        task_id: &str,
        file_id: &str,
    ) -> Result<Vec<crate::models::FileReviewComment>, rusqlite::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT id, task_id, file_id, review_status, review_comment, submitted_by, submitted_at
             FROM file_review_comments
             WHERE task_id = ?1 AND file_id = ?2
             ORDER BY submitted_at ASC"
        )?;

        let comment_iter = stmt.query_map([task_id, file_id], |row| {
            Ok(crate::models::FileReviewComment {
                id: row.get("id")?,
                task_id: row.get("task_id")?,
                file_id: row.get("file_id")?,
                review_status: row.get("review_status")?,
                review_comment: row.get("review_comment")?,
                submitted_by: row.get("submitted_by")?,
                submitted_at: row.get("submitted_at")?,
            })
        })?;

        let mut comments = Vec::new();
        for comment_result in comment_iter {
            comments.push(comment_result?);
        }

        Ok(comments)
    }

    /// Mark task as completed
    pub fn mark_task_completed(&self, task_id: &str) -> Result<(), rusqlite::Error> {
        log::info!("Marking task {} as completed", task_id);
        self.conn.execute(
            "UPDATE local_tasks SET status = 'completed' WHERE id = ?1",
            [task_id],
        )?;
        Ok(())
    }

    /// Export task review data as CSV
    /// Returns CSV string with task info, file review status, and all review comments
    pub fn export_task_review_csv(&self, task_id: &str) -> Result<String, rusqlite::Error> {
        log::info!("Exporting task review data as CSV: {}", task_id);

        // Get task info
        let mut stmt = self.conn.prepare(
            "SELECT title, status, task_type, created_at FROM local_tasks WHERE id = ?1"
        )?;
        let task_result = stmt.query_row([&task_id], |row| {
            Ok((
                row.get::<_, String>("title")?,
                row.get::<_, String>("status")?,
                row.get::<_, Option<String>>("task_type")?,
                row.get::<_, String>("created_at")?,
            ))
        });

        let (task_title, task_status, task_type, task_created) = match task_result {
            Ok(t) => t,
            Err(_) => return Err(rusqlite::Error::QueryReturnedNoRows),
        };

        // Build CSV header
        let mut csv = String::from("\u{FEFF}"); // UTF-8 BOM for Excel compatibility
        csv.push_str("Task ID,Task Title,Task Type,Task Status,Created At,File ID,File Path,File Name,File Status,Review Status,Review Comment,Comment ID,Comment Status,Comment Author,Comment Date\n");

        // Get files for this task
        let mut file_stmt = self.conn.prepare(
            "SELECT id, path, name, status, review_status, review_comment FROM local_task_files WHERE task_id = ?1"
        )?;

        let file_iter = file_stmt.query_map([&task_id], |row| {
            Ok((
                row.get::<_, String>("id")?,
                row.get::<_, String>("path")?,
                row.get::<_, String>("name")?,
                row.get::<_, String>("status")?,
                row.get::<_, Option<String>>("review_status")?,
                row.get::<_, Option<String>>("review_comment")?,
            ))
        })?;

        // CSV writer helper - escapes fields with commas or quotes
        let escape_csv = |s: &str| -> String {
            if s.contains(',') || s.contains('"') || s.contains('\n') {
                format!("\"{}\"", s.replace("\"", "\"\""))
            } else {
                s.to_string()
            }
        };

        for file_result in file_iter {
            let (file_id, file_path, file_name, file_status, review_status, review_comment) = file_result?;

            // Row with current review status
            let row = format!(
                "{},{},{},{},{},{},{},{},{},{},{},,,,,\n",
                escape_csv(&task_id),
                escape_csv(&task_title),
                escape_csv(&task_type.as_deref().unwrap_or("")),
                escape_csv(&task_status),
                escape_csv(&task_created),
                escape_csv(&file_id),
                escape_csv(&file_path),
                escape_csv(&file_name),
                escape_csv(&file_status),
                escape_csv(&review_status.as_deref().unwrap_or("pending")),
                escape_csv(&review_comment.as_deref().unwrap_or("")),
            );
            csv.push_str(&row);

            // Get all historical review comments for this file
            let mut comment_stmt = self.conn.prepare(
                "SELECT id, review_status, review_comment, submitted_by, submitted_at
                 FROM file_review_comments
                 WHERE task_id = ?1 AND file_id = ?2
                 ORDER BY submitted_at ASC"
            )?;

            let comment_iter = comment_stmt.query_map([&task_id, file_id.as_str()], |row| {
                Ok((
                    row.get::<_, String>("id")?,
                    row.get::<_, String>("review_status")?,
                    row.get::<_, String>("review_comment")?,
                    row.get::<_, String>("submitted_by")?,
                    row.get::<_, String>("submitted_at")?,
                ))
            })?;

            for comment_result in comment_iter {
                let (comment_id, comment_status, comment_text, comment_author, comment_date) = comment_result?;
                let comment_row = format!(
                    "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},\n",
                    escape_csv(&task_id),
                    escape_csv(&task_title),
                    escape_csv(&task_type.as_deref().unwrap_or("")),
                    escape_csv(&task_status),
                    escape_csv(&task_created),
                    escape_csv(&file_id),
                    escape_csv(&file_path),
                    escape_csv(&file_name),
                    escape_csv(&file_status),
                    "", // current review status - empty for comment rows
                    "", // current review comment - empty for comment rows
                    escape_csv(&comment_id),
                    escape_csv(&comment_status),
                    escape_csv(&comment_author),
                    escape_csv(&comment_date),
                );
                csv.push_str(&comment_row);
            }
        }

        Ok(csv)
    }

    // ============================================================================
    // Gerrit Integration Methods
    // ============================================================================

    /// Create Gerrit tables if they don't exist
    pub fn init_gerrit_schema(&self) -> Result<(), HyperReviewError> {
        self.conn.execute_batch("
            CREATE TABLE IF NOT EXISTS gerrit_instances (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                url TEXT NOT NULL UNIQUE,
                username TEXT NOT NULL,
                password_encrypted TEXT NOT NULL,
                version TEXT DEFAULT '',
                last_connected TEXT,
                is_active INTEGER NOT NULL DEFAULT 0,
                connection_status TEXT NOT NULL DEFAULT 'disconnected',
                polling_interval INTEGER NOT NULL DEFAULT 300,
                max_changes INTEGER NOT NULL DEFAULT 100,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS gerrit_changes (
                id TEXT PRIMARY KEY,
                change_id TEXT NOT NULL,
                instance_id TEXT NOT NULL,
                project TEXT NOT NULL,
                branch TEXT NOT NULL,
                subject TEXT NOT NULL,
                status TEXT NOT NULL,
                owner_name TEXT NOT NULL,
                owner_email TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                insertions INTEGER NOT NULL DEFAULT 0,
                deletions INTEGER NOT NULL DEFAULT 0,
                current_revision TEXT NOT NULL,
                current_patch_set_num INTEGER NOT NULL,
                total_files INTEGER NOT NULL DEFAULT 0,
                reviewed_files INTEGER NOT NULL DEFAULT 0,
                local_comments INTEGER NOT NULL DEFAULT 0,
                remote_comments INTEGER NOT NULL DEFAULT 0,
                import_status TEXT NOT NULL DEFAULT 'pending',
                last_sync_at TEXT,
                conflict_status TEXT NOT NULL DEFAULT 'none',
                metadata TEXT NOT NULL DEFAULT '{}',
                
                FOREIGN KEY (instance_id) REFERENCES gerrit_instances(id) ON DELETE CASCADE,
                UNIQUE(instance_id, change_id)
            );

            CREATE INDEX IF NOT EXISTS idx_gerrit_changes_instance ON gerrit_changes(instance_id);
            CREATE INDEX IF NOT EXISTS idx_gerrit_changes_status ON gerrit_changes(status);
            CREATE INDEX IF NOT EXISTS idx_gerrit_changes_import_status ON gerrit_changes(import_status);
        ").map_err(HyperReviewError::Database)?;

        // Migration: Add missing columns to existing tables if they don't exist
        // Check if username column exists in gerrit_instances table
        let check_column = |table_name: &str, col_name: &str| -> rusqlite::Result<bool> {
            let mut stmt = self.conn.prepare(&format!(
                "SELECT COUNT(*) FROM pragma_table_info('{}') WHERE name='{}'",
                table_name, col_name
            ))?;
            let count: i64 = stmt.query_row([], |row| row.get(0))?;
            Ok(count > 0)
        };

        // Check if gerrit_instances table exists and has the username column
        let table_exists = self.conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='gerrit_instances'")
            .and_then(|mut stmt| stmt.query_row([], |_| Ok(())))
            .is_ok();

        if table_exists {
            // Add missing columns if they don't exist
            if !check_column("gerrit_instances", "username").unwrap_or(false) {
                log::info!("Adding username column to gerrit_instances table");
                self.conn.execute(
                    "ALTER TABLE gerrit_instances ADD COLUMN username TEXT DEFAULT 'admin'",
                    []
                ).map_err(HyperReviewError::Database)?;
            }

            if !check_column("gerrit_instances", "password_encrypted").unwrap_or(false) {
                log::info!("Adding password_encrypted column to gerrit_instances table");
                self.conn.execute(
                    "ALTER TABLE gerrit_instances ADD COLUMN password_encrypted TEXT DEFAULT ''",
                    []
                ).map_err(HyperReviewError::Database)?;
            }

            if !check_column("gerrit_instances", "version").unwrap_or(false) {
                log::info!("Adding version column to gerrit_instances table");
                self.conn.execute(
                    "ALTER TABLE gerrit_instances ADD COLUMN version TEXT DEFAULT ''",
                    []
                ).map_err(HyperReviewError::Database)?;
            }

            if !check_column("gerrit_instances", "last_connected").unwrap_or(false) {
                log::info!("Adding last_connected column to gerrit_instances table");
                self.conn.execute(
                    "ALTER TABLE gerrit_instances ADD COLUMN last_connected TEXT",
                    []
                ).map_err(HyperReviewError::Database)?;
            }

            if !check_column("gerrit_instances", "connection_status").unwrap_or(false) {
                log::info!("Adding connection_status column to gerrit_instances table");
                self.conn.execute(
                    "ALTER TABLE gerrit_instances ADD COLUMN connection_status TEXT DEFAULT 'disconnected'",
                    []
                ).map_err(HyperReviewError::Database)?;
            }

            if !check_column("gerrit_instances", "polling_interval").unwrap_or(false) {
                log::info!("Adding polling_interval column to gerrit_instances table");
                self.conn.execute(
                    "ALTER TABLE gerrit_instances ADD COLUMN polling_interval INTEGER DEFAULT 300",
                    []
                ).map_err(HyperReviewError::Database)?;
            }

            if !check_column("gerrit_instances", "max_changes").unwrap_or(false) {
                log::info!("Adding max_changes column to gerrit_instances table");
                self.conn.execute(
                    "ALTER TABLE gerrit_instances ADD COLUMN max_changes INTEGER DEFAULT 100",
                    []
                ).map_err(HyperReviewError::Database)?;
            }

            if !check_column("gerrit_instances", "created_at").unwrap_or(false) {
                log::info!("Adding created_at column to gerrit_instances table");
                self.conn.execute(
                    "ALTER TABLE gerrit_instances ADD COLUMN created_at TEXT DEFAULT CURRENT_TIMESTAMP",
                    []
                ).map_err(HyperReviewError::Database)?;
            }

            if !check_column("gerrit_instances", "updated_at").unwrap_or(false) {
                log::info!("Adding updated_at column to gerrit_instances table");
                self.conn.execute(
                    "ALTER TABLE gerrit_instances ADD COLUMN updated_at TEXT DEFAULT CURRENT_TIMESTAMP",
                    []
                ).map_err(HyperReviewError::Database)?;
            }
        }

        // Similar migration for gerrit_changes table
        let changes_table_exists = self.conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='gerrit_changes'")
            .and_then(|mut stmt| stmt.query_row([], |_| Ok(())))
            .is_ok();

        if changes_table_exists {
            if !check_column("gerrit_changes", "insertions").unwrap_or(false) {
                log::info!("Adding insertions column to gerrit_changes table");
                self.conn.execute(
                    "ALTER TABLE gerrit_changes ADD COLUMN insertions INTEGER DEFAULT 0",
                    []
                ).map_err(HyperReviewError::Database)?;
            }

            if !check_column("gerrit_changes", "deletions").unwrap_or(false) {
                log::info!("Adding deletions column to gerrit_changes table");
                self.conn.execute(
                    "ALTER TABLE gerrit_changes ADD COLUMN deletions INTEGER DEFAULT 0",
                    []
                ).map_err(HyperReviewError::Database)?;
            }
        }

        Ok(())
    }

    /// Store a Gerrit instance
    pub fn store_gerrit_instance(&self, instance: &GerritInstance) -> Result<(), HyperReviewError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO gerrit_instances 
             (id, name, url, username, password_encrypted, version, last_connected, 
              is_active, connection_status, polling_interval, max_changes, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                instance.id,
                instance.name,
                instance.url,
                instance.username,
                instance.password_encrypted,
                instance.version,
                instance.last_connected,
                instance.is_active,
                instance.connection_status.to_string(),
                instance.polling_interval,
                instance.max_changes,
                instance.created_at,
                instance.updated_at
            ],
        ).map_err(HyperReviewError::Database)?;

        Ok(())
    }

    /// Get a Gerrit instance by ID
    pub fn get_gerrit_instance(&self, instance_id: &str) -> Result<Option<GerritInstance>, HyperReviewError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, url, username, password_encrypted, version, last_connected,
                    is_active, connection_status, polling_interval, max_changes, created_at, updated_at
             FROM gerrit_instances WHERE id = ?1"
        ).map_err(HyperReviewError::Database)?;

        let result = stmt.query_row(params![instance_id], |row| {
            Ok(GerritInstance {
                id: row.get(0)?,
                name: row.get(1)?,
                url: row.get(2)?,
                username: row.get(3)?,
                password_encrypted: row.get(4)?,
                version: row.get(5)?,
                is_active: row.get::<_, i32>(7)? != 0,
                last_connected: row.get(6)?,
                connection_status: ConnectionStatus::from_string(&row.get::<_, String>(8)?),
                polling_interval: row.get(9)?,
                max_changes: row.get(10)?,
                created_at: row.get(11)?,
                updated_at: row.get(12)?,
            })
        });

        match result {
            Ok(instance) => Ok(Some(instance)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(HyperReviewError::Database(e)),
        }
    }

    /// Get all Gerrit instances
    pub fn get_all_gerrit_instances(&self) -> Result<Vec<GerritInstance>, HyperReviewError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, url, username, password_encrypted, version, last_connected,
                    is_active, connection_status, polling_interval, max_changes, created_at, updated_at
             FROM gerrit_instances ORDER BY name"
        ).map_err(HyperReviewError::Database)?;

        let rows = stmt.query_map([], |row| {
            Ok(GerritInstance {
                id: row.get(0)?,
                name: row.get(1)?,
                url: row.get(2)?,
                username: row.get(3)?,
                password_encrypted: row.get(4)?,
                version: row.get(5)?,
                is_active: row.get::<_, i32>(7)? != 0,
                last_connected: row.get(6)?,
                connection_status: ConnectionStatus::from_string(&row.get::<_, String>(8)?),
                polling_interval: row.get(9)?,
                max_changes: row.get(10)?,
                created_at: row.get(11)?,
                updated_at: row.get(12)?,
            })
        }).map_err(HyperReviewError::Database)?;

        let mut instances = Vec::new();
        for row in rows {
            instances.push(row.map_err(HyperReviewError::Database)?);
        }

        Ok(instances)
    }

    /// Delete a Gerrit instance
    pub fn delete_gerrit_instance(&self, instance_id: &str) -> Result<bool, HyperReviewError> {
        let rows_affected = self.conn.execute(
            "DELETE FROM gerrit_instances WHERE id = ?1",
            params![instance_id],
        ).map_err(HyperReviewError::Database)?;

        Ok(rows_affected > 0)
    }

    /// Store a Gerrit change
    pub fn store_gerrit_change(&self, change: &GerritChange) -> Result<(), HyperReviewError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO gerrit_changes 
             (id, change_id, instance_id, project, branch, subject, status, owner_name, owner_email,
              created_at, updated_at, insertions, deletions, current_revision, current_patch_set_num, 
              total_files, reviewed_files, local_comments, remote_comments, import_status, last_sync_at, 
              conflict_status, metadata)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23)",
            params![
                change.id,
                change.change_id,
                change.instance_id,
                change.project,
                change.branch,
                change.subject,
                change.status.to_string(),
                change.owner.name,
                change.owner.email,
                change.created,
                change.updated,
                change.insertions,
                change.deletions,
                change.current_revision,
                change.current_patch_set_num,
                change.total_files,
                change.reviewed_files,
                change.local_comments,
                change.remote_comments,
                change.import_status.to_string(),
                change.last_sync,
                change.conflict_status.to_string(),
                serde_json::to_string(&change.metadata).unwrap_or_default()
            ],
        ).map_err(HyperReviewError::Database)?;

        Ok(())
    }

    /// Get a Gerrit change by ID
    pub fn get_gerrit_change(&self, change_id: &str) -> Result<Option<GerritChange>, HyperReviewError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, change_id, instance_id, project, branch, subject, status, owner_name, owner_email,
                    created_at, updated_at, insertions, deletions, current_revision, current_patch_set_num, 
                    total_files, reviewed_files, local_comments, remote_comments, import_status, last_sync_at,
                    conflict_status, metadata
             FROM gerrit_changes WHERE id = ?1 OR change_id = ?1"
        ).map_err(HyperReviewError::Database)?;

        let result = stmt.query_row(params![change_id], |row| {
            let metadata_str: String = row.get(22)?;
            let metadata = serde_json::from_str(&metadata_str).unwrap_or_default();

            Ok(GerritChange {
                id: row.get(0)?,
                change_id: row.get(1)?,
                instance_id: row.get(2)?,
                project: row.get(3)?,
                branch: row.get(4)?,
                subject: row.get(5)?,
                status: ChangeStatus::from_string(&row.get::<_, String>(6)?),
                owner: crate::models::gerrit::GerritUser {
                    account_id: 0, // TODO: Add account_id to schema
                    name: row.get(7)?,
                    email: row.get(8)?,
                    username: None,
                    avatar_url: None,
                },
                created: row.get(9)?,
                updated: row.get(10)?,
                insertions: row.get(11)?,
                deletions: row.get(12)?,
                current_revision: row.get(13)?,
                current_patch_set_num: row.get(14)?,
                patch_sets: Vec::new(), // TODO: Load from patch_sets table
                files: Vec::new(),      // TODO: Load from gerrit_files table
                total_files: row.get(15)?,
                reviewed_files: row.get(16)?,
                local_comments: row.get(17)?,
                remote_comments: row.get(18)?,
                import_status: ImportStatus::from_string(&row.get::<_, String>(19)?),
                last_sync: row.get(20)?,
                conflict_status: ConflictStatus::from_string(&row.get::<_, String>(21)?),
                metadata,
            })
        });

        match result {
            Ok(change) => Ok(Some(change)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(HyperReviewError::Database(e)),
        }
    }

    /// Update a Gerrit change
    pub fn update_gerrit_change(&self, change: &GerritChange) -> Result<(), HyperReviewError> {
        self.conn.execute(
            "UPDATE gerrit_changes SET 
             subject = ?2, status = ?3, updated_at = ?4, insertions = ?5, deletions = ?6,
             current_revision = ?7, current_patch_set_num = ?8, total_files = ?9, reviewed_files = ?10,
             local_comments = ?11, remote_comments = ?12, import_status = ?13,
             last_sync_at = ?14, conflict_status = ?15, metadata = ?16
             WHERE id = ?1",
            params![
                change.id,
                change.subject,
                change.status.to_string(),
                change.updated,
                change.insertions,
                change.deletions,
                change.current_revision,
                change.current_patch_set_num,
                change.total_files,
                change.reviewed_files,
                change.local_comments,
                change.remote_comments,
                change.import_status.to_string(),
                change.last_sync,
                change.conflict_status.to_string(),
                serde_json::to_string(&change.metadata).unwrap_or_default()
            ],
        ).map_err(HyperReviewError::Database)?;

        Ok(())
    }

    /// Get all Gerrit changes for an instance
    pub fn get_gerrit_changes_for_instance(&self, instance_id: &str) -> Result<Vec<GerritChange>, HyperReviewError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, change_id, instance_id, project, branch, subject, status, owner_name, owner_email,
                    created_at, updated_at, insertions, deletions, current_revision, current_patch_set_num, 
                    total_files, reviewed_files, local_comments, remote_comments, import_status, last_sync_at,
                    conflict_status, metadata
             FROM gerrit_changes WHERE instance_id = ?1 ORDER BY updated_at DESC"
        ).map_err(HyperReviewError::Database)?;

        let rows = stmt.query_map(params![instance_id], |row| {
            let metadata_str: String = row.get(22)?;
            let metadata = serde_json::from_str(&metadata_str).unwrap_or_default();

            Ok(GerritChange {
                id: row.get(0)?,
                change_id: row.get(1)?,
                instance_id: row.get(2)?,
                project: row.get(3)?,
                branch: row.get(4)?,
                subject: row.get(5)?,
                status: ChangeStatus::from_string(&row.get::<_, String>(6)?),
                owner: crate::models::gerrit::GerritUser {
                    account_id: 0,
                    name: row.get(7)?,
                    email: row.get(8)?,
                    username: None,
                    avatar_url: None,
                },
                created: row.get(9)?,
                updated: row.get(10)?,
                insertions: row.get(11)?,
                deletions: row.get(12)?,
                current_revision: row.get(13)?,
                current_patch_set_num: row.get(14)?,
                patch_sets: Vec::new(),
                files: Vec::new(),
                total_files: row.get(15)?,
                reviewed_files: row.get(16)?,
                local_comments: row.get(17)?,
                remote_comments: row.get(18)?,
                import_status: ImportStatus::from_string(&row.get::<_, String>(19)?),
                last_sync: row.get(20)?,
                conflict_status: ConflictStatus::from_string(&row.get::<_, String>(21)?),
                metadata,
            })
        }).map_err(HyperReviewError::Database)?;

        let mut changes = Vec::new();
        for row in rows {
            changes.push(row.map_err(HyperReviewError::Database)?);
        }

        Ok(changes)
    }

    /// Delete a Gerrit change
    pub fn delete_gerrit_change(&self, change_id: &str) -> Result<bool, HyperReviewError> {
        let rows_affected = self.conn.execute(
            "DELETE FROM gerrit_changes WHERE id = ?1 OR change_id = ?1",
            params![change_id],
        ).map_err(HyperReviewError::Database)?;

        Ok(rows_affected > 0)
    }

    /// Clear all Gerrit data (for debugging/testing purposes)
    pub fn clear_all_gerrit_data(&self) -> Result<usize, HyperReviewError> {
        let rows_affected = self.conn.execute("DELETE FROM gerrit_instances", [])
            .map_err(HyperReviewError::Database)?;
        Ok(rows_affected)
    }
}