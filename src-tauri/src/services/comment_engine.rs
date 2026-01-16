// Comment Engine Service
// Handles comment creation, editing, threading, and status management

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use log::{info, warn, error, debug};

use crate::errors::HyperReviewError;
use crate::models::gerrit::{ReviewComment, CommentType, CommentStatus};
use crate::storage::sqlite::Database;

/// Comment engine for managing review comments
pub struct CommentEngine {
    config: CommentEngineConfig,
}

/// Configuration for comment engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentEngineConfig {
    pub auto_save_drafts: bool,
    pub max_comment_length: u32,
    pub enable_threading: bool,
    pub enable_markdown: bool,
    pub auto_publish_threshold: u32, // Auto-publish after N characters
}

impl Default for CommentEngineConfig {
    fn default() -> Self {
        Self {
            auto_save_drafts: true,
            max_comment_length: 10000,
            enable_threading: true,
            enable_markdown: true,
            auto_publish_threshold: 0, // Disabled by default
        }
    }
}

/// Comment thread structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentThread {
    pub id: String,
    pub session_id: String,
    pub file_path: String,
    pub line_number: Option<u32>,
    pub root_comment: ReviewComment,
    pub replies: Vec<ReviewComment>,
    pub is_resolved: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// Comment creation parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCommentParams {
    pub session_id: String,
    pub file_path: String,
    pub line_number: Option<u32>,
    pub content: String,
    pub comment_type: CommentType,
    pub parent_comment_id: Option<String>,
}

/// Comment update parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCommentParams {
    pub comment_id: String,
    pub content: Option<String>,
    pub status: Option<CommentStatus>,
    pub comment_type: Option<CommentType>,
}

/// Comment search criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentSearchCriteria {
    pub session_id: Option<String>,
    pub file_path: Option<String>,
    pub line_number: Option<u32>,
    pub comment_types: Vec<CommentType>,
    pub statuses: Vec<CommentStatus>,
    pub content_pattern: Option<String>,
    pub author_filter: Option<String>,
    pub date_range: Option<(String, String)>, // (start, end) ISO dates
}

/// Comment statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentStats {
    pub total_comments: u32,
    pub draft_comments: u32,
    pub published_comments: u32,
    pub resolved_comments: u32,
    pub comments_by_type: HashMap<CommentType, u32>,
    pub comments_by_file: HashMap<String, u32>,
    pub threads_count: u32,
}

/// Inline comment positioning information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineCommentPosition {
    pub file_path: String,
    pub line_number: u32,
    pub column_start: Option<u32>,
    pub column_end: Option<u32>,
    pub side: DiffSide, // Which side of the diff (old/new)
    pub context_lines: Vec<String>, // Surrounding lines for context
}

/// Which side of a diff the comment applies to
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DiffSide {
    Left,  // Old version (deletions)
    Right, // New version (additions)
    Both,  // Applies to both sides
}

/// Inline comment with positioning and display information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineComment {
    pub comment: ReviewComment,
    pub position: InlineCommentPosition,
    pub display_info: InlineCommentDisplay,
}

/// Display information for inline comments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineCommentDisplay {
    pub is_visible: bool,
    pub is_highlighted: bool,
    pub highlight_color: String,
    pub anchor_id: String, // For navigation
    pub z_index: u32, // For layering multiple comments
}

/// Parameters for creating inline comments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInlineCommentParams {
    pub session_id: String,
    pub file_path: String,
    pub line_number: u32,
    pub column_start: Option<u32>,
    pub column_end: Option<u32>,
    pub side: DiffSide,
    pub content: String,
    pub comment_type: CommentType,
    pub parent_comment_id: Option<String>,
}

/// Line-level comment aggregation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineComments {
    pub file_path: String,
    pub line_number: u32,
    pub side: DiffSide,
    pub comments: Vec<InlineComment>,
    pub total_count: u32,
    pub unresolved_count: u32,
    pub has_drafts: bool,
}

/// Comment operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentOperationResult {
    pub success: bool,
    pub comment_id: Option<String>,
    pub thread_id: Option<String>,
    pub message: String,
    pub warnings: Vec<String>,
}

impl CommentEngine {
    /// Create a new comment engine
    pub fn new(config: CommentEngineConfig) -> Self {
        Self { config }
    }

    /// Create a new comment
    pub fn create_comment(
        &self,
        params: CreateCommentParams,
        database: &Database,
    ) -> Result<CommentOperationResult, HyperReviewError> {
        info!("Creating comment for session: {}, file: {}", params.session_id, params.file_path);

        // Validate comment content
        self.validate_comment_content(&params.content)?;

        // Generate comment ID
        let comment_id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        // Create comment object
        let comment = ReviewComment {
            id: comment_id.clone(),
            session_id: params.session_id.clone(),
            file_path: params.file_path.clone(),
            line_number: params.line_number,
            content: params.content.clone(),
            comment_type: params.comment_type.clone(),
            status: CommentStatus::Draft, // Always start as draft
            parent_comment_id: params.parent_comment_id.clone(),
            created_at: now.clone(),
            updated_at: now,
        };

        // Store comment in database
        database.store_review_comment(&comment)?;

        // Check if this creates a new thread or adds to existing
        let thread_id = if params.parent_comment_id.is_none() {
            // This is a root comment, create new thread
            self.create_comment_thread(&comment, database)?
        } else {
            // This is a reply, find the thread
            self.find_thread_for_comment(&comment_id, database)?
        };

        let mut warnings = Vec::new();

        // Auto-publish if configured
        if self.config.auto_publish_threshold > 0 && 
           params.content.len() >= self.config.auto_publish_threshold as usize {
            match self.update_comment_status(&comment_id, CommentStatus::Published, database) {
                Ok(_) => info!("Auto-published comment due to length threshold"),
                Err(e) => {
                    warn!("Failed to auto-publish comment: {}", e);
                    warnings.push("Failed to auto-publish comment".to_string());
                }
            }
        }

        info!("Successfully created comment: {}", comment_id);

        Ok(CommentOperationResult {
            success: true,
            comment_id: Some(comment_id),
            thread_id: Some(thread_id),
            message: "Comment created successfully".to_string(),
            warnings,
        })
    }

    /// Update an existing comment
    pub fn update_comment(
        &self,
        params: UpdateCommentParams,
        database: &Database,
    ) -> Result<CommentOperationResult, HyperReviewError> {
        info!("Updating comment: {}", params.comment_id);

        // Get existing comment
        let mut comment = database.get_review_comment(&params.comment_id)?
            .ok_or_else(|| HyperReviewError::other(format!("Comment not found: {}", params.comment_id)))?;

        let mut updated = false;

        // Update content if provided
        if let Some(content) = &params.content {
            self.validate_comment_content(content)?;
            comment.content = content.clone();
            updated = true;
        }

        // Update status if provided
        if let Some(status) = &params.status {
            comment.status = status.clone();
            updated = true;
        }

        // Update type if provided
        if let Some(comment_type) = &params.comment_type {
            comment.comment_type = comment_type.clone();
            updated = true;
        }

        if updated {
            comment.updated_at = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
            database.update_review_comment(&comment)?;
            info!("Successfully updated comment: {}", params.comment_id);
        }

        Ok(CommentOperationResult {
            success: true,
            comment_id: Some(params.comment_id),
            thread_id: None,
            message: if updated { "Comment updated successfully" } else { "No changes made" }.to_string(),
            warnings: Vec::new(),
        })
    }

    /// Delete a comment
    pub fn delete_comment(
        &self,
        comment_id: &str,
        database: &Database,
    ) -> Result<CommentOperationResult, HyperReviewError> {
        info!("Deleting comment: {}", comment_id);

        // Check if comment exists and get its details
        let comment = database.get_review_comment(comment_id)?
            .ok_or_else(|| HyperReviewError::other(format!("Comment not found: {}", comment_id)))?;

        // Check if this comment has replies
        let replies = self.get_comment_replies(comment_id, database)?;
        if !replies.is_empty() {
            return Err(HyperReviewError::other(
                "Cannot delete comment with replies. Delete replies first.".to_string()
            ));
        }

        // Delete the comment
        database.delete_review_comment(comment_id)?;

        info!("Successfully deleted comment: {}", comment_id);

        Ok(CommentOperationResult {
            success: true,
            comment_id: Some(comment_id.to_string()),
            thread_id: None,
            message: "Comment deleted successfully".to_string(),
            warnings: Vec::new(),
        })
    }

    /// Get comment by ID
    pub fn get_comment(
        &self,
        comment_id: &str,
        database: &Database,
    ) -> Result<Option<ReviewComment>, HyperReviewError> {
        debug!("Getting comment: {}", comment_id);
        database.get_review_comment(comment_id)
    }

    /// Get comments for a session
    pub fn get_session_comments(
        &self,
        session_id: &str,
        database: &Database,
    ) -> Result<Vec<ReviewComment>, HyperReviewError> {
        debug!("Getting comments for session: {}", session_id);
        database.get_session_comments(session_id)
    }

    /// Get comments for a file
    pub fn get_file_comments(
        &self,
        session_id: &str,
        file_path: &str,
        database: &Database,
    ) -> Result<Vec<ReviewComment>, HyperReviewError> {
        debug!("Getting comments for file: {} in session: {}", file_path, session_id);
        database.get_file_comments(session_id, file_path)
    }

    /// Get comment thread
    pub fn get_comment_thread(
        &self,
        root_comment_id: &str,
        database: &Database,
    ) -> Result<Option<CommentThread>, HyperReviewError> {
        debug!("Getting comment thread for root: {}", root_comment_id);

        // Get root comment
        let root_comment = match database.get_review_comment(root_comment_id)? {
            Some(comment) if comment.parent_comment_id.is_none() => comment,
            Some(_) => return Err(HyperReviewError::other("Comment is not a root comment".to_string())),
            None => return Ok(None),
        };

        // Get all replies
        let replies = self.get_comment_replies(root_comment_id, database)?;

        // Check if thread is resolved (all comments are resolved or acknowledged)
        let is_resolved = std::iter::once(&root_comment)
            .chain(replies.iter())
            .all(|c| matches!(c.status, CommentStatus::Resolved | CommentStatus::Acknowledged));

        let thread = CommentThread {
            id: format!("thread_{}", root_comment_id),
            session_id: root_comment.session_id.clone(),
            file_path: root_comment.file_path.clone(),
            line_number: root_comment.line_number,
            root_comment: root_comment.clone(),
            replies,
            is_resolved,
            created_at: root_comment.created_at.clone(),
            updated_at: root_comment.updated_at.clone(),
        };

        Ok(Some(thread))
    }

    /// Search comments
    pub fn search_comments(
        &self,
        criteria: &CommentSearchCriteria,
        database: &Database,
    ) -> Result<Vec<ReviewComment>, HyperReviewError> {
        debug!("Searching comments with criteria: {:?}", criteria);

        // Get all comments for the session (if specified)
        let mut comments = if let Some(session_id) = &criteria.session_id {
            database.get_session_comments(session_id)?
        } else {
            // If no session specified, this would require a different database method
            // For now, return empty as we typically search within a session
            Vec::new()
        };

        // Apply filters
        comments.retain(|comment| self.matches_search_criteria(comment, criteria));

        info!("Found {} comments matching search criteria", comments.len());
        Ok(comments)
    }

    /// Get comment statistics
    pub fn get_comment_stats(
        &self,
        session_id: &str,
        database: &Database,
    ) -> Result<CommentStats, HyperReviewError> {
        debug!("Calculating comment statistics for session: {}", session_id);

        let comments = database.get_session_comments(session_id)?;

        let mut stats = CommentStats {
            total_comments: comments.len() as u32,
            draft_comments: 0,
            published_comments: 0,
            resolved_comments: 0,
            comments_by_type: HashMap::new(),
            comments_by_file: HashMap::new(),
            threads_count: 0,
        };

        for comment in &comments {
            // Count by status
            match comment.status {
                CommentStatus::Draft => stats.draft_comments += 1,
                CommentStatus::Published => stats.published_comments += 1,
                CommentStatus::Resolved | CommentStatus::Acknowledged => stats.resolved_comments += 1,
            }

            // Count by type
            *stats.comments_by_type.entry(comment.comment_type.clone()).or_insert(0) += 1;

            // Count by file
            *stats.comments_by_file.entry(comment.file_path.clone()).or_insert(0) += 1;

            // Count threads (root comments only)
            if comment.parent_comment_id.is_none() {
                stats.threads_count += 1;
            }
        }

        info!("Comment statistics calculated: {} total, {} threads", stats.total_comments, stats.threads_count);
        Ok(stats)
    }

    /// Publish all draft comments
    pub fn publish_all_drafts(
        &self,
        session_id: &str,
        database: &Database,
    ) -> Result<u32, HyperReviewError> {
        info!("Publishing all draft comments for session: {}", session_id);

        let comments = database.get_session_comments(session_id)?;
        let mut published_count = 0;

        for comment in comments {
            if comment.status == CommentStatus::Draft {
                match self.update_comment_status(&comment.id, CommentStatus::Published, database) {
                    Ok(_) => published_count += 1,
                    Err(e) => warn!("Failed to publish comment {}: {}", comment.id, e),
                }
            }
        }

        info!("Published {} draft comments", published_count);
        Ok(published_count)
    }

    /// Create an inline comment with positioning information
    pub fn create_inline_comment(
        &self,
        params: CreateInlineCommentParams,
        database: &Database,
    ) -> Result<CommentOperationResult, HyperReviewError> {
        info!("Creating inline comment for {}:{} in session: {}", 
              params.file_path, params.line_number, params.session_id);

        // Validate positioning
        self.validate_inline_position(&params)?;

        // Create the base comment
        let comment_params = CreateCommentParams {
            session_id: params.session_id.clone(),
            file_path: params.file_path.clone(),
            line_number: Some(params.line_number),
            content: params.content.clone(),
            comment_type: params.comment_type.clone(),
            parent_comment_id: params.parent_comment_id.clone(),
        };

        let result = self.create_comment(comment_params, database)?;

        // Store additional inline positioning data if needed
        // For now, we store this in the comment's line_number field
        // In a more complex implementation, we might have a separate table

        info!("Successfully created inline comment at {}:{}", params.file_path, params.line_number);
        Ok(result)
    }

    /// Get inline comments for a specific line
    pub fn get_line_comments(
        &self,
        session_id: &str,
        file_path: &str,
        line_number: u32,
        side: Option<DiffSide>,
        database: &Database,
    ) -> Result<LineComments, HyperReviewError> {
        debug!("Getting line comments for {}:{} in session: {}", file_path, line_number, session_id);

        // Get all comments for the file
        let file_comments = database.get_file_comments(session_id, file_path)?;

        // Filter to specific line
        let line_comments: Vec<ReviewComment> = file_comments.into_iter()
            .filter(|c| c.line_number == Some(line_number))
            .collect();

        // Convert to inline comments with positioning
        let mut inline_comments = Vec::new();
        for comment in &line_comments {
            let inline_comment = self.create_inline_comment_from_review_comment(
                comment, 
                side.clone().unwrap_or(DiffSide::Right)
            )?;
            inline_comments.push(inline_comment);
        }

        // Calculate aggregation stats
        let total_count = inline_comments.len() as u32;
        let unresolved_count = inline_comments.iter()
            .filter(|c| !matches!(c.comment.status, CommentStatus::Resolved | CommentStatus::Acknowledged))
            .count() as u32;
        let has_drafts = inline_comments.iter()
            .any(|c| c.comment.status == CommentStatus::Draft);

        let line_comments = LineComments {
            file_path: file_path.to_string(),
            line_number,
            side: side.unwrap_or(DiffSide::Right),
            comments: inline_comments,
            total_count,
            unresolved_count,
            has_drafts,
        };

        debug!("Found {} comments for line {}:{}", total_count, file_path, line_number);
        Ok(line_comments)
    }

    /// Get all inline comments for a file with positioning
    pub fn get_file_inline_comments(
        &self,
        session_id: &str,
        file_path: &str,
        database: &Database,
    ) -> Result<Vec<InlineComment>, HyperReviewError> {
        debug!("Getting inline comments for file: {} in session: {}", file_path, session_id);

        let file_comments = database.get_file_comments(session_id, file_path)?;
        let mut inline_comments = Vec::new();

        for comment in file_comments {
            if comment.line_number.is_some() {
                let inline_comment = self.create_inline_comment_from_review_comment(
                    &comment, 
                    DiffSide::Right // Default to right side
                )?;
                inline_comments.push(inline_comment);
            }
        }

        info!("Found {} inline comments for file: {}", inline_comments.len(), file_path);
        Ok(inline_comments)
    }

    /// Get comments grouped by line for diff display
    pub fn get_comments_by_line(
        &self,
        session_id: &str,
        file_path: &str,
        database: &Database,
    ) -> Result<HashMap<u32, LineComments>, HyperReviewError> {
        debug!("Getting comments grouped by line for file: {} in session: {}", file_path, session_id);

        let file_comments = database.get_file_comments(session_id, file_path)?;
        let mut comments_by_line: HashMap<u32, Vec<ReviewComment>> = HashMap::new();

        // Group comments by line number
        for comment in file_comments {
            if let Some(line_num) = comment.line_number {
                comments_by_line.entry(line_num).or_insert_with(Vec::new).push(comment);
            }
        }

        // Convert to LineComments structure
        let mut result = HashMap::new();
        for (line_num, comments) in comments_by_line {
            let mut inline_comments = Vec::new();
            for comment in &comments {
                let inline_comment = self.create_inline_comment_from_review_comment(
                    comment, 
                    DiffSide::Right
                )?;
                inline_comments.push(inline_comment);
            }

            let total_count = inline_comments.len() as u32;
            let unresolved_count = inline_comments.iter()
                .filter(|c| !matches!(c.comment.status, CommentStatus::Resolved | CommentStatus::Acknowledged))
                .count() as u32;
            let has_drafts = inline_comments.iter()
                .any(|c| c.comment.status == CommentStatus::Draft);

            let line_comments = LineComments {
                file_path: file_path.to_string(),
                line_number: line_num,
                side: DiffSide::Right,
                comments: inline_comments,
                total_count,
                unresolved_count,
                has_drafts,
            };

            result.insert(line_num, line_comments);
        }

        info!("Grouped comments into {} lines for file: {}", result.len(), file_path);
        Ok(result)
    }

    /// Update inline comment positioning
    pub fn update_inline_comment_position(
        &self,
        comment_id: &str,
        new_line_number: u32,
        new_column_start: Option<u32>,
        new_column_end: Option<u32>,
        database: &Database,
    ) -> Result<CommentOperationResult, HyperReviewError> {
        info!("Updating inline comment position: {} to line {}", comment_id, new_line_number);

        // Update the comment's line number
        let params = UpdateCommentParams {
            comment_id: comment_id.to_string(),
            content: None,
            status: None,
            comment_type: None,
        };

        // Get the comment first to update line number
        let mut comment = database.get_review_comment(comment_id)?
            .ok_or_else(|| HyperReviewError::other("Comment not found".to_string()))?;

        comment.line_number = Some(new_line_number);
        comment.updated_at = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        database.update_review_comment(&comment)?;

        info!("Successfully updated inline comment position");
        Ok(CommentOperationResult {
            success: true,
            comment_id: Some(comment_id.to_string()),
            thread_id: None,
            message: "Comment position updated successfully".to_string(),
            warnings: Vec::new(),
        })
    }

    /// Highlight comments in a line range
    pub fn highlight_comments_in_range(
        &self,
        session_id: &str,
        file_path: &str,
        start_line: u32,
        end_line: u32,
        database: &Database,
    ) -> Result<Vec<InlineComment>, HyperReviewError> {
        debug!("Highlighting comments in range {}:{}-{} for session: {}", 
               file_path, start_line, end_line, session_id);

        let file_comments = database.get_file_comments(session_id, file_path)?;
        let mut highlighted_comments = Vec::new();

        for comment in file_comments {
            if let Some(line_num) = comment.line_number {
                if line_num >= start_line && line_num <= end_line {
                    let mut inline_comment = self.create_inline_comment_from_review_comment(
                        &comment, 
                        DiffSide::Right
                    )?;
                    
                    // Set highlight
                    inline_comment.display_info.is_highlighted = true;
                    inline_comment.display_info.highlight_color = self.get_highlight_color(&comment.comment_type);
                    
                    highlighted_comments.push(inline_comment);
                }
            }
        }

        info!("Highlighted {} comments in range {}:{}-{}", 
              highlighted_comments.len(), file_path, start_line, end_line);
        Ok(highlighted_comments)
    }

    // Private helper methods

    fn create_comment_thread(
        &self,
        root_comment: &ReviewComment,
        _database: &Database,
    ) -> Result<String, HyperReviewError> {
        // For now, thread ID is just based on the root comment ID
        // In a more complex implementation, we might store threads separately
        let thread_id = format!("thread_{}", root_comment.id);
        debug!("Created comment thread: {}", thread_id);
        Ok(thread_id)
    }

    fn find_thread_for_comment(
        &self,
        comment_id: &str,
        database: &Database,
    ) -> Result<String, HyperReviewError> {
        // Find the root comment for this reply
        let comment = database.get_review_comment(comment_id)?
            .ok_or_else(|| HyperReviewError::other("Comment not found".to_string()))?;

        if let Some(parent_id) = &comment.parent_comment_id {
            // Find the root by traversing up the parent chain
            let mut current_id = parent_id.clone();
            loop {
                let parent = database.get_review_comment(&current_id)?
                    .ok_or_else(|| HyperReviewError::other("Parent comment not found".to_string()))?;
                
                if parent.parent_comment_id.is_none() {
                    // Found the root
                    return Ok(format!("thread_{}", parent.id));
                }
                current_id = parent.parent_comment_id.unwrap();
            }
        } else {
            // This is a root comment
            Ok(format!("thread_{}", comment.id))
        }
    }

    fn get_comment_replies(
        &self,
        parent_comment_id: &str,
        database: &Database,
    ) -> Result<Vec<ReviewComment>, HyperReviewError> {
        // This would need a database method to get replies by parent ID
        // For now, we'll get all session comments and filter
        let comment = database.get_review_comment(parent_comment_id)?
            .ok_or_else(|| HyperReviewError::other("Parent comment not found".to_string()))?;

        let all_comments = database.get_session_comments(&comment.session_id)?;
        
        let replies: Vec<ReviewComment> = all_comments.into_iter()
            .filter(|c| c.parent_comment_id.as_deref() == Some(parent_comment_id))
            .collect();

        Ok(replies)
    }

    fn update_comment_status(
        &self,
        comment_id: &str,
        status: CommentStatus,
        database: &Database,
    ) -> Result<(), HyperReviewError> {
        let params = UpdateCommentParams {
            comment_id: comment_id.to_string(),
            content: None,
            status: Some(status),
            comment_type: None,
        };

        self.update_comment(params, database)?;
        Ok(())
    }

    fn validate_comment_content(&self, content: &str) -> Result<(), HyperReviewError> {
        if content.trim().is_empty() {
            return Err(HyperReviewError::other("Comment content cannot be empty".to_string()));
        }

        if content.len() > self.config.max_comment_length as usize {
            return Err(HyperReviewError::other(format!(
                "Comment content exceeds maximum length of {} characters",
                self.config.max_comment_length
            )));
        }

        Ok(())
    }

    fn matches_search_criteria(&self, comment: &ReviewComment, criteria: &CommentSearchCriteria) -> bool {
        // Check file path
        if let Some(file_path) = &criteria.file_path {
            if comment.file_path != *file_path {
                return false;
            }
        }

        // Check line number
        if let Some(line_number) = criteria.line_number {
            if comment.line_number != Some(line_number) {
                return false;
            }
        }

        // Check comment types
        if !criteria.comment_types.is_empty() && !criteria.comment_types.contains(&comment.comment_type) {
            return false;
        }

        // Check statuses
        if !criteria.statuses.is_empty() && !criteria.statuses.contains(&comment.status) {
            return false;
        }

        // Check content pattern
        if let Some(pattern) = &criteria.content_pattern {
            if !comment.content.contains(pattern) {
                return false;
            }
        }

        // TODO: Add date range filtering when needed

        true
    }

    /// Validate inline comment positioning
    fn validate_inline_position(&self, params: &CreateInlineCommentParams) -> Result<(), HyperReviewError> {
        if params.line_number == 0 {
            return Err(HyperReviewError::other("Line number must be greater than 0".to_string()));
        }

        if let (Some(start), Some(end)) = (params.column_start, params.column_end) {
            if start > end {
                return Err(HyperReviewError::other("Column start must be less than or equal to column end".to_string()));
            }
        }

        Ok(())
    }

    /// Create an InlineComment from a ReviewComment
    fn create_inline_comment_from_review_comment(
        &self,
        comment: &ReviewComment,
        side: DiffSide,
    ) -> Result<InlineComment, HyperReviewError> {
        let line_number = comment.line_number.unwrap_or(1);
        
        let position = InlineCommentPosition {
            file_path: comment.file_path.clone(),
            line_number,
            column_start: None, // TODO: Extract from metadata if stored
            column_end: None,   // TODO: Extract from metadata if stored
            side,
            context_lines: Vec::new(), // TODO: Fetch surrounding lines if needed
        };

        let display_info = InlineCommentDisplay {
            is_visible: true,
            is_highlighted: false,
            highlight_color: self.get_highlight_color(&comment.comment_type),
            anchor_id: format!("comment-{}", comment.id),
            z_index: self.get_z_index_for_comment(&comment.comment_type),
        };

        Ok(InlineComment {
            comment: comment.clone(),
            position,
            display_info,
        })
    }

    /// Get highlight color for comment type
    fn get_highlight_color(&self, comment_type: &CommentType) -> String {
        match comment_type {
            CommentType::Issue => "#ff6b6b".to_string(),      // Red for issues
            CommentType::Suggestion => "#4ecdc4".to_string(), // Teal for suggestions
            CommentType::Question => "#45b7d1".to_string(),   // Blue for questions
            CommentType::General => "#a8a8a8".to_string(),    // Gray for general
            CommentType::Inline => "#feca57".to_string(),     // Yellow for inline
            CommentType::FileLevel => "#96ceb4".to_string(),  // Green for file-level
        }
    }

    /// Get z-index for comment layering
    fn get_z_index_for_comment(&self, comment_type: &CommentType) -> u32 {
        match comment_type {
            CommentType::Issue => 100,      // Highest priority
            CommentType::Question => 90,
            CommentType::Suggestion => 80,
            CommentType::Inline => 70,
            CommentType::General => 60,
            CommentType::FileLevel => 50,   // Lowest priority
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_database() -> (Database, TempDir) {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let db_path = temp_dir.path().join("test.db");
        let database = Database::new(db_path.to_str().unwrap()).expect("Failed to create database");
        database.init_schema().expect("Failed to init schema");
        database.init_gerrit_schema().expect("Failed to init gerrit schema");
        
        // Create a test gerrit instance first
        let instance = crate::models::gerrit::GerritInstance {
            id: "test-instance".to_string(),
            name: "Test Instance".to_string(),
            url: "https://test.gerrit.com".to_string(),
            username: "test-user".to_string(),
            password_encrypted: "encrypted-password".to_string(),
            version: "3.8.0".to_string(),
            is_active: true,
            last_connected: Some("2025-01-05 12:00:00".to_string()),
            connection_status: crate::models::gerrit::ConnectionStatus::Connected,
            polling_interval: 300,
            max_changes: 100,
            created_at: "2025-01-05 12:00:00".to_string(),
            updated_at: "2025-01-05 12:00:00".to_string(),
        };
        database.store_gerrit_instance(&instance).expect("Failed to create test instance");
        
        // Create a test gerrit change
        let change = crate::models::gerrit::GerritChange {
            id: "test-change".to_string(),
            change_id: "I1234567890abcdef".to_string(),
            instance_id: "test-instance".to_string(),
            project: "test-project".to_string(),
            branch: "main".to_string(),
            subject: "Test change".to_string(),
            status: crate::models::gerrit::ChangeStatus::New,
            owner: crate::models::gerrit::GerritUser {
                account_id: 1000,
                name: "Test User".to_string(),
                email: "test@example.com".to_string(),
                username: Some("testuser".to_string()),
                avatar_url: None,
            },
            created: "2025-01-05 12:00:00".to_string(),
            updated: "2025-01-05 12:00:00".to_string(),
            insertions: 10,
            deletions: 5,
            current_revision: "abc123".to_string(),
            current_patch_set_num: 1,
            patch_sets: Vec::new(),
            files: Vec::new(),
            total_files: 1,
            reviewed_files: 0,
            local_comments: 0,
            remote_comments: 0,
            import_status: crate::models::gerrit::ImportStatus::Pending,
            last_sync: None,
            conflict_status: crate::models::gerrit::ConflictStatus::None,
            metadata: std::collections::HashMap::new(),
        };
        database.store_gerrit_change(&change).expect("Failed to create test change");
        
        // Create a test review session to satisfy foreign key constraints
        let session = crate::models::gerrit::ReviewSession {
            id: "test-session".to_string(),
            change_id: "test-change".to_string(),
            patch_set_number: 1,
            reviewer_id: "test-reviewer".to_string(),
            mode: crate::models::gerrit::ReviewMode::Offline,
            status: crate::models::gerrit::ReviewStatus::InProgress,
            progress: crate::models::gerrit::ReviewProgress {
                total_files: 1,
                reviewed_files: 0,
                files_with_comments: 0,
                pending_files: Vec::new(),
            },
            created_at: "2025-01-05 12:00:00".to_string(),
            updated_at: "2025-01-05 12:00:00".to_string(),
        };
        database.store_review_session(&session).expect("Failed to create test session");
        
        (database, temp_dir)
    }

    #[tokio::test]
    async fn test_comment_engine_creation() {
        let config = CommentEngineConfig::default();
        let engine = CommentEngine::new(config);
        assert!(engine.config.auto_save_drafts);
        assert_eq!(engine.config.max_comment_length, 10000);
    }

    #[tokio::test]
    async fn test_create_comment() {
        let (database, _temp_dir) = create_test_database();
        let engine = CommentEngine::new(CommentEngineConfig::default());

        let params = CreateCommentParams {
            session_id: "test-session".to_string(),
            file_path: "src/main.rs".to_string(),
            line_number: Some(42),
            content: "This needs improvement".to_string(),
            comment_type: CommentType::Issue,
            parent_comment_id: None,
        };

        let result = engine.create_comment(params, &database).unwrap();
        assert!(result.success);
        assert!(result.comment_id.is_some());
        assert!(result.thread_id.is_some());
    }

    #[tokio::test]
    async fn test_update_comment() {
        let (database, _temp_dir) = create_test_database();
        let engine = CommentEngine::new(CommentEngineConfig::default());

        // Create a comment first
        let create_params = CreateCommentParams {
            session_id: "test-session".to_string(),
            file_path: "src/main.rs".to_string(),
            line_number: Some(42),
            content: "Original content".to_string(),
            comment_type: CommentType::Issue,
            parent_comment_id: None,
        };

        let create_result = engine.create_comment(create_params, &database).unwrap();
        let comment_id = create_result.comment_id.unwrap();

        // Update the comment
        let update_params = UpdateCommentParams {
            comment_id: comment_id.clone(),
            content: Some("Updated content".to_string()),
            status: Some(CommentStatus::Published),
            comment_type: None,
        };

        let update_result = engine.update_comment(update_params, &database).unwrap();
        assert!(update_result.success);

        // Verify the update
        let updated_comment = engine.get_comment(&comment_id, &database).unwrap().unwrap();
        assert_eq!(updated_comment.content, "Updated content");
        assert_eq!(updated_comment.status, CommentStatus::Published);
    }

    #[tokio::test]
    async fn test_comment_validation() {
        let engine = CommentEngine::new(CommentEngineConfig::default());

        // Test empty content
        assert!(engine.validate_comment_content("").is_err());
        assert!(engine.validate_comment_content("   ").is_err());

        // Test valid content
        assert!(engine.validate_comment_content("Valid comment").is_ok());

        // Test max length
        let long_content = "a".repeat(10001);
        assert!(engine.validate_comment_content(&long_content).is_err());
    }

    #[tokio::test]
    async fn test_get_comment_stats() {
        let (database, _temp_dir) = create_test_database();
        let engine = CommentEngine::new(CommentEngineConfig::default());

        // Create some test comments
        for i in 0..3 {
            let params = CreateCommentParams {
                session_id: "test-session".to_string(),
                file_path: format!("src/file{}.rs", i),
                line_number: Some(i * 10),
                content: format!("Comment {}", i),
                comment_type: CommentType::Issue,
                parent_comment_id: None,
            };
            engine.create_comment(params, &database).unwrap();
        }

        let stats = engine.get_comment_stats("test-session", &database).unwrap();
        assert_eq!(stats.total_comments, 3);
        assert_eq!(stats.draft_comments, 3); // All start as drafts
        assert_eq!(stats.threads_count, 3); // All are root comments
    }

    #[tokio::test]
    async fn test_create_inline_comment() {
        let (database, _temp_dir) = create_test_database();
        let engine = CommentEngine::new(CommentEngineConfig::default());

        let params = CreateInlineCommentParams {
            session_id: "test-session".to_string(),
            file_path: "src/main.rs".to_string(),
            line_number: 42,
            column_start: Some(10),
            column_end: Some(20),
            side: DiffSide::Right,
            content: "This line needs improvement".to_string(),
            comment_type: CommentType::Issue,
            parent_comment_id: None,
        };

        let result = engine.create_inline_comment(params, &database).unwrap();
        assert!(result.success);
        assert!(result.comment_id.is_some());
    }

    #[tokio::test]
    async fn test_get_line_comments() {
        let (database, _temp_dir) = create_test_database();
        let engine = CommentEngine::new(CommentEngineConfig::default());

        // Create multiple comments on the same line
        for i in 0..3 {
            let params = CreateInlineCommentParams {
                session_id: "test-session".to_string(),
                file_path: "src/main.rs".to_string(),
                line_number: 42,
                column_start: Some(i * 10),
                column_end: Some(i * 10 + 5),
                side: DiffSide::Right,
                content: format!("Comment {} on line 42", i),
                comment_type: CommentType::Issue,
                parent_comment_id: None,
            };
            engine.create_inline_comment(params, &database).unwrap();
        }

        let line_comments = engine.get_line_comments(
            "test-session",
            "src/main.rs",
            42,
            Some(DiffSide::Right),
            &database,
        ).unwrap();

        assert_eq!(line_comments.line_number, 42);
        assert_eq!(line_comments.total_count, 3);
        assert_eq!(line_comments.comments.len(), 3);
        assert!(line_comments.has_drafts); // All start as drafts
    }

    #[tokio::test]
    async fn test_get_comments_by_line() {
        let (database, _temp_dir) = create_test_database();
        let engine = CommentEngine::new(CommentEngineConfig::default());

        // Create comments on different lines
        let lines = vec![10, 20, 30];
        for line in &lines {
            let params = CreateInlineCommentParams {
                session_id: "test-session".to_string(),
                file_path: "src/main.rs".to_string(),
                line_number: *line,
                column_start: None,
                column_end: None,
                side: DiffSide::Right,
                content: format!("Comment on line {}", line),
                comment_type: CommentType::Issue,
                parent_comment_id: None,
            };
            engine.create_inline_comment(params, &database).unwrap();
        }

        let comments_by_line = engine.get_comments_by_line(
            "test-session",
            "src/main.rs",
            &database,
        ).unwrap();

        assert_eq!(comments_by_line.len(), 3);
        for line in lines {
            assert!(comments_by_line.contains_key(&line));
            assert_eq!(comments_by_line[&line].total_count, 1);
        }
    }

    #[tokio::test]
    async fn test_highlight_comments_in_range() {
        let (database, _temp_dir) = create_test_database();
        let engine = CommentEngine::new(CommentEngineConfig::default());

        // Create comments on different lines
        for line in 10..=15 {
            let params = CreateInlineCommentParams {
                session_id: "test-session".to_string(),
                file_path: "src/main.rs".to_string(),
                line_number: line,
                column_start: None,
                column_end: None,
                side: DiffSide::Right,
                content: format!("Comment on line {}", line),
                comment_type: CommentType::Issue,
                parent_comment_id: None,
            };
            engine.create_inline_comment(params, &database).unwrap();
        }

        let highlighted = engine.highlight_comments_in_range(
            "test-session",
            "src/main.rs",
            12,
            14,
            &database,
        ).unwrap();

        assert_eq!(highlighted.len(), 3); // Lines 12, 13, 14
        for comment in highlighted {
            assert!(comment.display_info.is_highlighted);
            assert_eq!(comment.display_info.highlight_color, "#ff6b6b"); // Issue color
        }
    }

    #[tokio::test]
    async fn test_inline_position_validation() {
        let engine = CommentEngine::new(CommentEngineConfig::default());

        // Test invalid line number
        let invalid_params = CreateInlineCommentParams {
            session_id: "test-session".to_string(),
            file_path: "src/main.rs".to_string(),
            line_number: 0, // Invalid
            column_start: None,
            column_end: None,
            side: DiffSide::Right,
            content: "Test comment".to_string(),
            comment_type: CommentType::Issue,
            parent_comment_id: None,
        };

        assert!(engine.validate_inline_position(&invalid_params).is_err());

        // Test invalid column range
        let invalid_columns = CreateInlineCommentParams {
            session_id: "test-session".to_string(),
            file_path: "src/main.rs".to_string(),
            line_number: 42,
            column_start: Some(20),
            column_end: Some(10), // End before start
            side: DiffSide::Right,
            content: "Test comment".to_string(),
            comment_type: CommentType::Issue,
            parent_comment_id: None,
        };

        assert!(engine.validate_inline_position(&invalid_columns).is_err());

        // Test valid params
        let valid_params = CreateInlineCommentParams {
            session_id: "test-session".to_string(),
            file_path: "src/main.rs".to_string(),
            line_number: 42,
            column_start: Some(10),
            column_end: Some(20),
            side: DiffSide::Right,
            content: "Test comment".to_string(),
            comment_type: CommentType::Issue,
            parent_comment_id: None,
        };

        assert!(engine.validate_inline_position(&valid_params).is_ok());
    }
}