// SQLite database operations
// Local storage for review metadata

use crate::models::{Repo, Comment, CommentStatus};
use rusqlite::{Connection, Result};
use serde_json;

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
        ")?;

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
}
