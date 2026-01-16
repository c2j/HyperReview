// SQLite connection manager for Gerrit operations
// Manages database connections and operations for Gerrit integration

use std::sync::{Arc, Mutex};
use rusqlite::{Connection, Result as SqliteResult, params};
use log::{info, warn, error, debug};

use crate::errors::HyperReviewError;
use crate::models::gerrit::{GerritInstance, GerritChange, GerritComment, ConnectionStatus, ChangeStatus};

/// SQLite connection manager for Gerrit operations
/// Provides thread-safe database access and connection pooling
pub struct GerritConnectionManager {
    /// Primary database connection
    primary_conn: Arc<Mutex<Connection>>,
    /// Connection string for creating new connections
    database_path: String,
    /// Connection pool for concurrent operations
    connection_pool: Vec<Arc<Mutex<Connection>>>,
    /// Maximum pool size
    max_pool_size: usize,
}

impl GerritConnectionManager {
    /// Create a new Gerrit connection manager
    pub fn new(database_path: &str) -> Result<Self, HyperReviewError> {
        info!("Creating Gerrit connection manager for database: {}", database_path);
        
        // Create primary connection
        let primary_conn = Connection::open(database_path)
            .map_err(HyperReviewError::Database)?;
        
        // Configure connection for better performance
        Self::configure_connection(&primary_conn)?;
        
        let manager = Self {
            primary_conn: Arc::new(Mutex::new(primary_conn)),
            database_path: database_path.to_string(),
            connection_pool: Vec::new(),
            max_pool_size: 5, // Default pool size
        };
        
        info!("Gerrit connection manager created successfully");
        Ok(manager)
    }
    
    /// Configure database connection for optimal performance
    fn configure_connection(conn: &Connection) -> Result<(), HyperReviewError> {
        // Enable foreign key constraints
        conn.execute("PRAGMA foreign_keys = ON", [])
            .map_err(|e| HyperReviewError::Other { message: format!("Failed to enable foreign keys: {}", e) })?;
        
        // Set journal mode for better concurrency
        conn.execute("PRAGMA journal_mode = WAL", [])
            .map_err(|e| HyperReviewError::Other { message: format!("Failed to set journal mode: {}", e) })?;
        
        // Set synchronous mode for balanced performance/safety
        conn.execute("PRAGMA synchronous = NORMAL", [])
            .map_err(|e| HyperReviewError::Other { message: format!("Failed to set synchronous mode: {}", e) })?;
        
        // Set cache size for better performance
        conn.execute("PRAGMA cache_size = -10000", [])
            .map_err(|e| HyperReviewError::Other { message: format!("Failed to set cache size: {}", e) })?;
        
        // Enable automatic indexing
        conn.execute("PRAGMA automatic_index = ON", [])
            .map_err(|e| HyperReviewError::Other { message: format!("Failed to enable automatic indexing: {}", e) })?;
        
        Ok(())
    }
    
    /// Get a connection from the pool (or create a new one if needed)
    pub fn get_connection(&mut self,
    ) -> Result<Arc<Mutex<Connection>>, HyperReviewError> {
        // Try to get an existing connection from the pool
        if let Some(conn) = self.connection_pool.pop() {
            debug!("Reusing existing connection from pool");
            return Ok(conn);
        }
        
        // Create a new connection if pool is empty and we're under the limit
        if self.connection_pool.len() < self.max_pool_size {
            debug!("Creating new database connection");
            let conn = Connection::open(&self.database_path)
                .map_err(|e| HyperReviewError::Other { message: format!("Failed to create pooled connection: {}", e) })?;
            
            Self::configure_connection(&conn)?;
            
            return Ok(Arc::new(Mutex::new(conn)));
        }
        
        // If pool is at capacity, wait and return the primary connection
        warn!("Connection pool at capacity, using primary connection");
        Ok(self.primary_conn.clone())
    }
    
    /// Return a connection to the pool
    pub fn return_connection(&mut self,
        conn: Arc<Mutex<Connection>>,
    ) {
        if self.connection_pool.len() < self.max_pool_size {
            self.connection_pool.push(conn);
            debug!("Connection returned to pool");
        } else {
            debug!("Pool at capacity, dropping connection");
        }
    }
    
    /// Get the primary connection (for critical operations)
    pub fn get_primary_connection(&self,
    ) -> Arc<Mutex<Connection>> {
        self.primary_conn.clone()
    }
    
    /// Test database connection
    pub fn test_connection(&self,
    ) -> Result<(), HyperReviewError> {
        let conn = self.primary_conn.lock().unwrap();
        
        // Simple query to test connection
        conn.execute("SELECT 1", [])
            .map_err(|e| HyperReviewError::Other { message: format!("Connection test failed: {}", e) })?;
        
        info!("Database connection test successful");
        Ok(())
    }
    
    /// Get connection statistics
    pub fn get_connection_stats(&self,
    ) -> ConnectionStats {
        ConnectionStats {
            pool_size: self.connection_pool.len(),
            max_pool_size: self.max_pool_size,
            database_path: self.database_path.clone(),
        }
    }
    
    /// Execute a transaction with automatic retry
    pub fn execute_transaction<F, T>(
        &mut self,
        mut transaction_fn: F,
    ) -> Result<T, HyperReviewError>
    where
        F: FnMut(&Connection,
        ) -> Result<T, HyperReviewError>,
    {
        let conn = self.get_primary_connection();
        let conn_guard = conn.lock().unwrap();
        
        // Start transaction
        let tx = conn_guard.unchecked_transaction()
            .map_err(|e| HyperReviewError::Other { message: format!("Failed to start transaction: {}", e) })?;
        
        // Execute transaction function
        match transaction_fn(&tx) {
            Ok(result) => {
                // Commit transaction
                tx.commit()
                    .map_err(|e| HyperReviewError::Other { message: format!("Failed to commit transaction: {}", e) })?;
                Ok(result)
            }
            Err(error) => {
                // Rollback transaction
                drop(tx); // This will rollback the transaction
                Err(error)
            }
        }
    }
    
    /// Close all connections and clean up resources
    pub fn close(&mut self,
    ) -> Result<(), HyperReviewError> {
        info!("Closing Gerrit connection manager");
        
        // Clear connection pool
        self.connection_pool.clear();
        
        // Close primary connection
        let conn = self.primary_conn.lock().unwrap();
        // Connection is automatically closed when dropped
        drop(conn);
        
        info!("Gerrit connection manager closed successfully");
        Ok(())
    }
}

/// Connection statistics
#[derive(Debug, Clone)]
pub struct ConnectionStats {
    pub pool_size: usize,
    pub max_pool_size: usize,
    pub database_path: String,
}

/// Repository pattern for GerritInstance operations
pub struct GerritInstanceRepository {
    conn_manager: Arc<Mutex<GerritConnectionManager>>,
}

impl GerritInstanceRepository {
    pub fn new(conn_manager: Arc<Mutex<GerritConnectionManager>>) -> Self {
        Self { conn_manager }
    }
    
    /// Create a new Gerrit instance
    pub fn create_instance(
        &self,
        instance: &GerritInstance,
    ) -> Result<String, HyperReviewError> {
        let mut manager = self.conn_manager.lock().unwrap();
        
        manager.execute_transaction(|conn| {
            conn.execute(
                "INSERT INTO gerrit_instances (
                    id, name, url, username, password_encrypted,
                    version, is_active, connection_status, polling_interval, max_changes,
                    created_at, updated_at, last_connected
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
                params![
                    instance.id, instance.name, instance.url,
                    instance.username, instance.password_encrypted,
                    instance.version, instance.is_active as i32,
                    instance.connection_status.to_string(), instance.polling_interval,
                    instance.max_changes, instance.created_at, instance.updated_at,
                    instance.last_connected
                ],)
            .map_err(|e| HyperReviewError::Other { message: format!("Failed to create Gerrit instance: {}", e) })?;
            
            Ok(instance.id.clone())
        })
    }
    
    /// Get a Gerrit instance by ID
    pub fn get_instance(
        &self,
        instance_id: &str,
    ) -> Result<Option<GerritInstance>, HyperReviewError> {
        let mut manager = self.conn_manager.lock().unwrap();
        let conn = manager.get_primary_connection();
        let conn_guard = conn.lock().unwrap();
        
        let mut stmt = conn_guard.prepare(
            "SELECT * FROM gerrit_instances WHERE id = ?1"
        ).map_err(|e| HyperReviewError::Other { message: format!("Failed to prepare statement: {}", e) })?;
        
        let mut instances = stmt.query_map([instance_id], |row| {
            let status: String = row.get(7)?;
            Ok(GerritInstance {
                id: row.get(0)?,
                name: row.get(1)?,
                url: row.get(2)?,
                username: row.get(3)?,
                password_encrypted: row.get(4)?,
                version: row.get(5)?,
                is_active: row.get(6)?,
                connection_status: ConnectionStatus::from_string(&status),
                polling_interval: row.get(8)?,
                max_changes: row.get(9)?,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
                last_connected: row.get(12)?,
            })
        }).map_err(|e| HyperReviewError::Other { message: format!("Failed to query instance: {}", e) })?;
        
        match instances.next() {
            Some(result) => result.map_err(HyperReviewError::Database).map(Some),
            None => Ok(None),
        }
    }
    
    /// Get all Gerrit instances
    pub fn get_all_instances(
        &self,
    ) -> Result<Vec<GerritInstance>, HyperReviewError> {
        let mut manager = self.conn_manager.lock().unwrap();
        let conn = manager.get_primary_connection();
        let conn_guard = conn.lock().unwrap();
        
        let mut stmt = conn_guard.prepare(
            "SELECT * FROM gerrit_instances ORDER BY name"
        ).map_err(|e| HyperReviewError::Other { message: format!("Failed to prepare statement: {}", e) })?;
        
        let instances = stmt.query_map([], |row| {
            let status: String = row.get(7)?;
            Ok(GerritInstance {
                id: row.get(0)?,
                name: row.get(1)?,
                url: row.get(2)?,
                username: row.get(3)?,
                password_encrypted: row.get(4)?,
                version: row.get(5)?,
                is_active: row.get(6)?,
                connection_status: ConnectionStatus::from_string(&status),
                polling_interval: row.get(8)?,
                max_changes: row.get(9)?,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
                last_connected: row.get(12)?,
            })
        }).map_err(|e| HyperReviewError::Other { message: format!("Failed to query instances: {}", e) })?;
        
        instances.collect::<Result<Vec<_>, _>>()
            .map_err(|e| HyperReviewError::Other { message: format!("Failed to collect instances: {}", e) })
    }
    
    /// Update a Gerrit instance
    pub fn update_instance(
        &self,
        instance: &GerritInstance,
    ) -> Result<(), HyperReviewError> {
        let mut manager = self.conn_manager.lock().unwrap();
        
        manager.execute_transaction(|conn| {
            conn.execute(
                "UPDATE gerrit_instances SET
                    name = ?2, url = ?3, username = ?4, password_encrypted = ?5,
                    version = ?6, is_active = ?7, connection_status = ?8, polling_interval = ?9,
                    max_changes = ?10, updated_at = ?11, last_connected = ?12
                WHERE id = ?1",
                params![
                    instance.id, instance.name, instance.url,
                    instance.username, instance.password_encrypted,
                    instance.version, instance.is_active as i32,
                    instance.connection_status.to_string(), instance.polling_interval,
                    instance.max_changes, instance.updated_at, instance.last_connected
                ],)
            .map_err(|e| HyperReviewError::Other { message: format!("Failed to update Gerrit instance: {}", e) })?;
            
            Ok(())
        })
    }
    
    /// Delete a Gerrit instance
    pub fn delete_instance(
        &self,
        instance_id: &str,
    ) -> Result<(), HyperReviewError> {
        let mut manager = self.conn_manager.lock().unwrap();
        
        manager.execute_transaction(|conn| {
            conn.execute(
                "DELETE FROM gerrit_instances WHERE id = ?1",
                [instance_id],)
            .map_err(|e| HyperReviewError::Other { message: format!("Failed to delete Gerrit instance: {}", e) })?;
            
            Ok(())
        })
    }
    
    /// Set the active instance (ensuring only one is active)
    pub fn set_active_instance(
        &self,
        instance_id: &str,
    ) -> Result<(), HyperReviewError> {
        let mut manager = self.conn_manager.lock().unwrap();
        
        manager.execute_transaction(|conn| {
            // Deactivate all instances
            conn.execute(
                "UPDATE gerrit_instances SET is_active = 0",
                [],)
            .map_err(|e| HyperReviewError::Other { message: format!("Failed to deactivate instances: {}", e) })?;
            
            // Activate the specified instance
            conn.execute(
                "UPDATE gerrit_instances SET is_active = 1 WHERE id = ?1",
                [instance_id],)
            .map_err(|e| HyperReviewError::Other { message: format!("Failed to activate instance: {}", e) })?;
            
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    
    fn create_test_manager() -> GerritConnectionManager {
        let temp_file = NamedTempFile::new().unwrap();
        let manager = GerritConnectionManager::new(temp_file.path().to_str().unwrap()).unwrap();
        
        // Initialize schema (this would normally be done by migration runner)
        let conn = manager.get_primary_connection();
        let conn_guard = conn.lock().unwrap();
        
        conn_guard.execute_batch(
            "CREATE TABLE IF NOT EXISTS gerrit_instances (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                url TEXT NOT NULL UNIQUE,
                username BLOB NOT NULL,
                password_encrypted BLOB NOT NULL,
                version TEXT DEFAULT '',
                is_active INTEGER NOT NULL DEFAULT 0,
                connection_status TEXT NOT NULL DEFAULT 'disconnected',
                polling_interval INTEGER NOT NULL DEFAULT 300,
                max_changes INTEGER NOT NULL DEFAULT 100,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                last_connected TEXT
            )"
        ).unwrap();
        
        manager
    }
    
    #[test]
    fn test_connection_manager_creation() {
        let manager = create_test_manager();
        assert!(manager.test_connection().is_ok());
    }
    
    #[test]
    fn test_connection_pool() {
        let mut manager = create_test_manager();
        
        // Get a connection
        let conn1 = manager.get_connection().unwrap();
        assert_eq!(manager.get_connection_stats().pool_size, 0);
        
        // Return the connection
        manager.return_connection(conn1);
        assert_eq!(manager.get_connection_stats().pool_size, 1);
    }
}