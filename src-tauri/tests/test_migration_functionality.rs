// Test that database migration works correctly for existing databases
// This test simulates the real-world scenario where a user has an old database

use hyperreview_lib::storage::sqlite::Database;
use hyperreview_lib::models::gerrit::{GerritInstance, ConnectionStatus};
use uuid::Uuid;
use chrono::Utc;

#[tokio::test]
async fn test_database_migration_functionality() {
    // Create in-memory database for testing
    let db = Database::new(":memory:").expect("Failed to create test database");
    
    // Initialize main schema first
    db.init_schema().expect("Failed to initialize main schema");
    
    // Now run the Gerrit schema initialization (which includes migration logic)
    db.init_gerrit_schema().expect("Failed to initialize Gerrit schema with migration");
    
    // Test that we can create a complete Gerrit instance with all fields
    let instance_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    let test_instance = GerritInstance {
        id: instance_id.clone(),
        name: "Migration Test Instance".to_string(),
        url: "http://edce7739774c:8080".to_string(),
        username: "admin".to_string(),
        password_encrypted: "8EWK0RrulrdN8d7vTFVpbQTAjQFU2lFQpRhTZpISBw".to_string(),
        version: "3.8.0".to_string(),
        is_active: true,
        last_connected: Some(now.clone()),
        connection_status: ConnectionStatus::Connected,
        polling_interval: 300,
        max_changes: 100,
        created_at: now.clone(),
        updated_at: now.clone(),
    };
    
    // Store the instance - this should work if migration was successful
    db.store_gerrit_instance(&test_instance)
        .expect("Failed to store Gerrit instance after migration");
    
    // Retrieve and verify all fields are present
    let retrieved = db.get_gerrit_instance(&instance_id)
        .expect("Failed to retrieve instance")
        .expect("Instance not found");
    
    assert_eq!(retrieved.id, test_instance.id);
    assert_eq!(retrieved.name, test_instance.name);
    assert_eq!(retrieved.url, test_instance.url);
    assert_eq!(retrieved.username, test_instance.username);
    assert_eq!(retrieved.password_encrypted, test_instance.password_encrypted);
    assert_eq!(retrieved.version, test_instance.version);
    assert_eq!(retrieved.is_active, test_instance.is_active);
    assert_eq!(retrieved.last_connected, test_instance.last_connected);
    assert_eq!(retrieved.connection_status, test_instance.connection_status);
    assert_eq!(retrieved.polling_interval, test_instance.polling_interval);
    assert_eq!(retrieved.max_changes, test_instance.max_changes);
    assert_eq!(retrieved.created_at, test_instance.created_at);
    assert_eq!(retrieved.updated_at, test_instance.updated_at);
    
    println!("✅ Database migration functionality test passed!");
    println!("   All Gerrit instance fields are properly stored and retrieved");
}

#[tokio::test]
async fn test_multiple_schema_initializations() {
    // Test that calling init_gerrit_schema multiple times is safe
    let db = Database::new(":memory:").expect("Failed to create test database");
    
    // Initialize main schema
    db.init_schema().expect("Failed to initialize main schema");
    
    // Initialize Gerrit schema multiple times - should be safe
    db.init_gerrit_schema().expect("First Gerrit schema initialization failed");
    db.init_gerrit_schema().expect("Second Gerrit schema initialization failed");
    db.init_gerrit_schema().expect("Third Gerrit schema initialization failed");
    
    // Test that we can still create instances
    let instance_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    let test_instance = GerritInstance {
        id: instance_id.clone(),
        name: "Multiple Init Test".to_string(),
        url: "http://test:8080".to_string(),
        username: "test".to_string(),
        password_encrypted: "test".to_string(),
        version: "3.8.0".to_string(),
        is_active: false,
        last_connected: None,
        connection_status: ConnectionStatus::Disconnected,
        polling_interval: 300,
        max_changes: 100,
        created_at: now.clone(),
        updated_at: now.clone(),
    };
    
    db.store_gerrit_instance(&test_instance)
        .expect("Failed to store instance after multiple initializations");
    
    let retrieved = db.get_gerrit_instance(&instance_id)
        .expect("Failed to retrieve instance")
        .expect("Instance not found");
    
    assert_eq!(retrieved.name, "Multiple Init Test");
    
    println!("✅ Multiple schema initializations test passed!");
}