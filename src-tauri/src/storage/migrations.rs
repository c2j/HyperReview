use std::path::Path;
use rusqlite::{Connection, params};
use log::{info, warn, error};

use crate::errors::HyperReviewError;

/// Database migration system for Gerrit integration schema
pub struct MigrationRunner {
    conn: Connection,
}

impl MigrationRunner {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }
    
    /// Run all migrations to bring database to latest schema
    pub fn run_migrations(&mut self,
    ) -> Result<(), HyperReviewError> {
        // Create migrations table if it doesn't exist
        self.create_migrations_table()?;
        
        // Get current migration version
        let current_version = self.get_current_version()?;
        info!("Current database version: {}", current_version);
        
        // Run migrations in order
        let migrations = self.get_migrations();
        
        for (version, migration) in migrations {
            if version > current_version {
                info!("Running migration V{}: {}", version, migration.name());
                
                // Start transaction
                let tx = self.conn.unchecked_transaction()
                    .map_err(|e| HyperReviewError::Other { message: e.to_string() })?;
                
                // Run migration
                migration.run(&tx)?;
                
                // Record migration
                self.record_migration(version, migration.name())?;
                
                // Commit transaction
                tx.commit()
                    .map_err(|e| HyperReviewError::Other { message: e.to_string() })?;
                
                info!("Migration V{} completed successfully", version);
            }
        }
        
        Ok(())
    }
    
    /// Create migrations tracking table
    fn create_migrations_table(&self,
    ) -> Result<(), HyperReviewError> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS schema_migrations (
                version INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        ).map_err(|e| HyperReviewError::Other { message: e.to_string() })?;
        
        Ok(())
    }
    
    /// Get current database version
    fn get_current_version(&self,
    ) -> Result<i32, HyperReviewError> {
        let version: rusqlite::Result<i32> = self.conn.query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
            [],
            |row| row.get(0),
        );
        
        match version {
            Ok(v) => Ok(v),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(0),
            Err(e) => Err(HyperReviewError::Other { message: e.to_string() }),
        }
    }
    
    /// Record that a migration was applied
    fn record_migration(
        &self,
        version: i32,
        name: &str,
    ) -> Result<(), HyperReviewError> {
        self.conn.execute(
            "INSERT INTO schema_migrations (version, name) VALUES (?1, ?2)",
            params![version, name],
        ).map_err(|e| HyperReviewError::Other { message: e.to_string() })?;
        
        Ok(())
    }
    
    /// Get all available migrations in order
    fn get_migrations(&self,
    ) -> Vec<(i32, Box<dyn Migration>)> {
        vec![
            (1, Box::new(V001CreateGerritTables)),
            (2, Box::new(V002AddPerformanceIndexes)),
            (3, Box::new(V003AddSyncTracking)),
            (4, Box::new(V004AddEncryptionSupport)),
            (5, Box::new(V005AddOfflineCache)),
        ]
    }
}

/// Trait for database migrations
trait Migration {
    fn name(&self,
    ) -> &str;
    
    fn run(
        &self,
        conn: &Connection,
    ) -> Result<(), HyperReviewError>;
}

// ============================================================================
// Migration Implementations
// ============================================================================

/// V001: Initial Gerrit integration tables
struct V001CreateGerritTables;

impl Migration for V001CreateGerritTables {
    fn name(&self,
    ) -> &str {
        "create_gerrit_tables"
    }
    
    fn run(
        &self,
        conn: &Connection,
    ) -> Result<(), HyperReviewError> {
        conn.execute_batch(
            "-- Gerrit instances configuration
            CREATE TABLE gerrit_instances (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                url TEXT NOT NULL UNIQUE,
                username_encrypted BLOB NOT NULL,
                password_encrypted BLOB NOT NULL,
                version TEXT DEFAULT '',
                last_connected TEXT,
                is_active INTEGER NOT NULL DEFAULT 0,
                connection_status TEXT NOT NULL DEFAULT 'disconnected',
                polling_interval INTEGER NOT NULL DEFAULT 300,
                max_changes INTEGER NOT NULL DEFAULT 100,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                
                CONSTRAINT chk_url_https CHECK (url LIKE 'https://%'),
                CONSTRAINT chk_polling_interval CHECK (polling_interval BETWEEN 60 AND 3600),
                CONSTRAINT chk_max_changes CHECK (max_changes BETWEEN 10 AND 1000),
                CONSTRAINT chk_connection_status CHECK (connection_status IN ('connected', 'disconnected', 'authentication_failed', 'version_incompatible', 'network_error'))
            );
            
            -- Gerrit changes
            CREATE TABLE gerrit_changes (
                id TEXT PRIMARY KEY,
                change_id TEXT NOT NULL,
                instance_id TEXT NOT NULL,
                project TEXT NOT NULL,
                branch TEXT NOT NULL,
                subject TEXT NOT NULL,
                status TEXT NOT NULL,
                owner_account_id TEXT NOT NULL,
                owner_name TEXT NOT NULL,
                owner_email TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
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
                CONSTRAINT chk_change_status CHECK (status IN ('new', 'draft', 'merged', 'abandoned')),
                CONSTRAINT chk_import_status CHECK (import_status IN ('pending', 'importing', 'imported', 'failed', 'outdated')),
                CONSTRAINT chk_conflict_status CHECK (conflict_status IN ('none', 'comments_pending', 'patch_set_updated', 'manual_resolution_required')),
                CONSTRAINT chk_revision_length CHECK (length(current_revision) = 40),
                CONSTRAINT chk_patch_set_num CHECK (current_patch_set_num >= 1),
                CONSTRAINT chk_file_counts CHECK (reviewed_files <= total_files),
                UNIQUE(instance_id, change_id)
            );
            
            -- Patch sets (revisions)
            CREATE TABLE patch_sets (
                id TEXT PRIMARY KEY,
                change_id TEXT NOT NULL,
                revision_id TEXT NOT NULL,
                patch_set_number INTEGER NOT NULL,
                ref_name TEXT NOT NULL,
                commit_sha TEXT NOT NULL,
                commit_message TEXT NOT NULL,
                author_name TEXT NOT NULL,
                author_email TEXT NOT NULL,
                author_date TEXT NOT NULL,
                committer_name TEXT NOT NULL,
                committer_email TEXT NOT NULL,
                committer_date TEXT NOT NULL,
                is_current INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                
                FOREIGN KEY (change_id) REFERENCES gerrit_changes(id) ON DELETE CASCADE,
                CONSTRAINT chk_revision_length CHECK (length(commit_sha) = 40),
                CONSTRAINT chk_patch_set_number CHECK (patch_set_number >= 1),
                UNIQUE(change_id, patch_set_number),
                UNIQUE(change_id, is_current)
            );
            
            -- Files within changes
            CREATE TABLE gerrit_files (
                id TEXT PRIMARY KEY,
                change_id TEXT NOT NULL,
                patch_set_id TEXT NOT NULL,
                file_path TEXT NOT NULL,
                old_file_path TEXT,
                change_type TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'unreviewed',
                lines_inserted INTEGER NOT NULL DEFAULT 0,
                lines_deleted INTEGER NOT NULL DEFAULT 0,
                size_delta INTEGER NOT NULL DEFAULT 0,
                size_new INTEGER NOT NULL DEFAULT 0,
                is_binary INTEGER NOT NULL DEFAULT 0,
                content_type TEXT NOT NULL DEFAULT 'text/plain',
                diff_content_encrypted BLOB,
                is_reviewed INTEGER NOT NULL DEFAULT 0,
                review_progress TEXT NOT NULL DEFAULT '{\"total_lines\":0,\"reviewed_lines\":0,\"comment_count\":0,\"severity_score\":0}',
                last_reviewed_at TEXT,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                
                FOREIGN KEY (change_id) REFERENCES gerrit_changes(id) ON DELETE CASCADE,
                FOREIGN KEY (patch_set_id) REFERENCES patch_sets(id) ON DELETE CASCADE,
                CONSTRAINT chk_change_type CHECK (change_type IN ('added', 'modified', 'deleted', 'renamed', 'copied', 'rewritten')),
                CONSTRAINT chk_file_status CHECK (status IN ('unreviewed', 'pending', 'reviewed', 'approved', 'needs_work', 'question')),
                CONSTRAINT chk_size_constraints CHECK (size_new >= 0 AND lines_inserted >= 0 AND lines_deleted >= 0),
                UNIQUE(change_id, patch_set_id, file_path)
            );
            
            -- Comments (both local and remote)
            CREATE TABLE gerrit_comments (
                id TEXT PRIMARY KEY,
                change_id TEXT NOT NULL,
                patch_set_id TEXT NOT NULL,
                file_id TEXT NOT NULL,
                external_comment_id TEXT,
                parent_comment_id TEXT,
                line_number INTEGER NOT NULL,
                line_range_start INTEGER,
                line_range_end INTEGER,
                content_encrypted BLOB NOT NULL,
                author_name TEXT,
                author_email TEXT,
                author_account_id TEXT,
                comment_type TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'draft',
                severity TEXT,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                synced_at TEXT,
                
                FOREIGN KEY (change_id) REFERENCES gerrit_changes(id) ON DELETE CASCADE,
                FOREIGN KEY (patch_set_id) REFERENCES patch_sets(id) ON DELETE CASCADE,
                FOREIGN KEY (file_id) REFERENCES gerrit_files(id) ON DELETE CASCADE,
                FOREIGN KEY (parent_comment_id) REFERENCES gerrit_comments(id) ON DELETE SET NULL,
                CONSTRAINT chk_comment_type CHECK (comment_type IN ('local', 'remote')),
                CONSTRAINT chk_comment_status CHECK (status IN ('draft', 'published', 'pending_sync', 'synced')),
                CONSTRAINT chk_line_number CHECK (line_number >= 1),
                CONSTRAINT chk_message_length CHECK (length(content_encrypted) > 0),
                CHECK (line_range_end IS NULL OR line_range_start IS NOT NULL),
                CHECK (line_range_end IS NULL OR line_range_end >= line_range_start)
            );
            
            -- Review operations
            CREATE TABLE gerrit_reviews (
                id TEXT PRIMARY KEY,
                change_id TEXT NOT NULL,
                patch_set_id TEXT NOT NULL,
                external_review_id TEXT,
                score_code_review INTEGER,
                score_verified INTEGER,
                message_encrypted BLOB,
                reviewer_name TEXT,
                reviewer_email TEXT,
                reviewer_account_id TEXT,
                status TEXT NOT NULL DEFAULT 'draft',
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                synced_at TEXT,
                
                FOREIGN KEY (change_id) REFERENCES gerrit_changes(id) ON DELETE CASCADE,
                FOREIGN KEY (patch_set_id) REFERENCES patch_sets(id) ON DELETE CASCADE,
                CONSTRAINT chk_review_status CHECK (status IN ('draft', 'pending_sync', 'synced')),
                CONSTRAINT chk_score_range CHECK (score_code_review IS NULL OR score_code_review IN (-2, -1, 0, 1, 2)),
                CONSTRAINT chk_verified_range CHECK (score_verified IS NULL OR score_verified IN (-1, 0, 1))
            );
            
            -- Sync status tracking
            CREATE TABLE sync_status (
                id TEXT PRIMARY KEY,
                entity_type TEXT NOT NULL,
                entity_id TEXT NOT NULL,
                sync_status TEXT NOT NULL DEFAULT 'pending',
                last_attempt_at TEXT,
                next_retry_at TEXT,
                retry_count INTEGER NOT NULL DEFAULT 0,
                error_message TEXT,
                conflict_details TEXT,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                
                FOREIGN KEY (entity_id) REFERENCES gerrit_changes(id) ON DELETE CASCADE,
                CONSTRAINT chk_entity_type CHECK (entity_type IN ('change', 'comment', 'review')),
                CONSTRAINT chk_sync_status CHECK (sync_status IN ('pending', 'synced', 'conflict', 'failed')),
                CONSTRAINT chk_retry_count CHECK (retry_count >= 0),
                UNIQUE(entity_type, entity_id)
            );
            
            -- Operation queue for offline sync
            CREATE TABLE operation_queue (
                id TEXT PRIMARY KEY,
                instance_id TEXT NOT NULL,
                change_id TEXT NOT NULL,
                operation_type TEXT NOT NULL,
                payload_encrypted BLOB NOT NULL,
                priority TEXT NOT NULL DEFAULT 'normal',
                status TEXT NOT NULL DEFAULT 'queued',
                retry_count INTEGER NOT NULL DEFAULT 0,
                max_retries INTEGER NOT NULL DEFAULT 3,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                last_attempt TEXT,
                next_retry TEXT,
                error_message TEXT,
                
                FOREIGN KEY (instance_id) REFERENCES gerrit_instances(id) ON DELETE CASCADE,
                FOREIGN KEY (change_id) REFERENCES gerrit_changes(id) ON DELETE CASCADE,
                CONSTRAINT chk_operation_type CHECK (operation_type IN ('submit_comment', 'update_review', 'push_patchset', 'sync_change')),
                CONSTRAINT chk_priority CHECK (priority IN ('low', 'normal', 'high', 'critical')),
                CONSTRAINT chk_operation_status CHECK (status IN ('queued', 'processing', 'completed', 'failed', 'cancelled')),
                CONSTRAINT chk_retry_count CHECK (retry_count <= max_retries),
                CONSTRAINT chk_max_retries CHECK (max_retries BETWEEN 0 AND 10)
            );"
        ).map_err(|e| HyperReviewError::Other { message: e.to_string() })?;
        
        Ok(())
    }
}

/// V002: Add performance indexes
struct V002AddPerformanceIndexes;

impl Migration for V002AddPerformanceIndexes {
    fn name(&self,
    ) -> &str {
        "add_performance_indexes"
    }
    
    fn run(
        &self,
        conn: &Connection,
    ) -> Result<(), HyperReviewError> {
        conn.execute_batch(
            "-- Performance indexes for common queries
            CREATE INDEX idx_gerrit_changes_instance ON gerrit_changes(instance_id);
            CREATE INDEX idx_gerrit_changes_status ON gerrit_changes(status);
            CREATE INDEX idx_gerrit_changes_project ON gerrit_changes(project);
            CREATE INDEX idx_gerrit_changes_updated ON gerrit_changes(updated_at);
            CREATE INDEX idx_gerrit_changes_import_status ON gerrit_changes(import_status);
            
            CREATE INDEX idx_patch_sets_change ON patch_sets(change_id);
            CREATE INDEX idx_patch_sets_current ON patch_sets(is_current);
            
            CREATE INDEX idx_gerrit_files_change ON gerrit_files(change_id);
            CREATE INDEX idx_gerrit_files_patch_set ON gerrit_files(patch_set_id);
            CREATE INDEX idx_gerrit_files_path ON gerrit_files(file_path);
            CREATE INDEX idx_gerrit_files_status ON gerrit_files(status);
            CREATE INDEX idx_gerrit_files_reviewed ON gerrit_files(is_reviewed);
            
            CREATE INDEX idx_gerrit_comments_change ON gerrit_comments(change_id);
            CREATE INDEX idx_gerrit_comments_patch_set ON gerrit_comments(patch_set_id);
            CREATE INDEX idx_gerrit_comments_file ON gerrit_comments(file_id);
            CREATE INDEX idx_gerrit_comments_status ON gerrit_comments(status);
            CREATE INDEX idx_gerrit_comments_line ON gerrit_comments(line_number);
            
            CREATE INDEX idx_gerrit_reviews_change ON gerrit_reviews(change_id);
            CREATE INDEX idx_gerrit_reviews_patch_set ON gerrit_reviews(patch_set_id);
            CREATE INDEX idx_gerrit_reviews_status ON gerrit_reviews(status);
            
            CREATE INDEX idx_sync_status_entity ON sync_status(entity_type, entity_id);
            CREATE INDEX idx_sync_status_status ON sync_status(sync_status);
            CREATE INDEX idx_sync_status_retry ON sync_status(next_retry_at);
            
            CREATE INDEX idx_operation_queue_instance ON operation_queue(instance_id);
            CREATE INDEX idx_operation_queue_change ON operation_queue(change_id);
            CREATE INDEX idx_operation_queue_status ON operation_queue(status);
            CREATE INDEX idx_operation_queue_priority ON operation_queue(priority);
            CREATE INDEX idx_operation_queue_next_retry ON operation_queue(next_retry);"
        ).map_err(|e| HyperReviewError::Other { message: e.to_string() })?;
        
        Ok(())
    }
}

/// V003: Add sync tracking improvements
struct V003AddSyncTracking;

impl Migration for V003AddSyncTracking {
    fn name(&self,
    ) -> &str {
        "add_sync_tracking"
    }
    
    fn run(
        &self,
        conn: &Connection,
    ) -> Result<(), HyperReviewError> {
        conn.execute_batch(
            "-- Add sync statistics tracking
            ALTER TABLE gerrit_instances ADD COLUMN sync_stats TEXT DEFAULT '{}';
            
            -- Add last sync timestamp to changes
            ALTER TABLE gerrit_changes ADD COLUMN last_successful_sync TEXT;
            
            -- Add conflict resolution tracking
            CREATE TABLE conflict_resolution (
                id TEXT PRIMARY KEY,
                entity_type TEXT NOT NULL,
                entity_id TEXT NOT NULL,
                conflict_type TEXT NOT NULL,
                local_version TEXT NOT NULL,
                remote_version TEXT NOT NULL,
                resolution_type TEXT NOT NULL,
                resolution_data TEXT,
                resolved_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                resolved_by TEXT,
                
                CONSTRAINT chk_conflict_resolution_type CHECK (resolution_type IN ('merge', 'overwrite', 'discard', 'manual')),
                UNIQUE(entity_type, entity_id, conflict_type)
            );"
        ).map_err(|e| HyperReviewError::Other { message: e.to_string() })?;
        
        Ok(())
    }
}

/// V004: Add encryption support (placeholder)
struct V004AddEncryptionSupport;

impl Migration for V004AddEncryptionSupport {
    fn name(&self,
    ) -> &str {
        "add_encryption_support"
    }
    
    fn run(
        &self,
        conn: &Connection,
    ) -> Result<(), HyperReviewError> {
        // Encryption support is handled at the application level
        // This migration is a placeholder for future database-level encryption
        info!("Encryption support is handled at application level");
        Ok(())
    }
}

/// V005: Add offline cache (placeholder)
struct V005AddOfflineCache;

impl Migration for V005AddOfflineCache {
    fn name(&self,
    ) -> &str {
        "add_offline_cache"
    }
    
    fn run(
        &self,
        conn: &Connection,
    ) -> Result<(), HyperReviewError> {
        // Offline cache is handled by the application
        // This migration is a placeholder for future database-level caching
        info!("Offline cache is handled at application level");
        Ok(())
    }
}