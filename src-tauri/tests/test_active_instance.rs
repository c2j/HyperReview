// Test active instance persistence
// Tests that active instance state is correctly saved and loaded from database

use hyperreview_lib::storage::sqlite::Database;
use hyperreview_lib::models::gerrit::{GerritInstance, ConnectionStatus};
use uuid::Uuid;
use chrono::Utc;

#[tokio::test]
async fn test_active_instance_persistence() {
    // Create in-memory database for testing
    let db = Database::new(":memory:").expect("Failed to create test database");
    
    // Initialize schemas
    db.init_schema().expect("Failed to initialize schema");
    db.init_gerrit_schema().expect("Failed to initialize Gerrit schema");
    
    let now = Utc::now().to_rfc3339();
    
    // Create multiple test instances
    let instance1_id = Uuid::new_v4().to_string();
    let instance2_id = Uuid::new_v4().to_string();
    
    let instance1 = GerritInstance {
        id: instance1_id.clone(),
        name: "Instance 1".to_string(),
        url: "http://gerrit1:8080".to_string(),
        username: "user1".to_string(),
        password_encrypted: "pass1".to_string(),
        version: "3.8.0".to_string(),
        is_active: false, // Initially not active
        last_connected: Some(now.clone()),
        connection_status: ConnectionStatus::Connected,
        polling_interval: 300,
        max_changes: 100,
        created_at: now.clone(),
        updated_at: now.clone(),
    };
    
    let instance2 = GerritInstance {
        id: instance2_id.clone(),
        name: "Instance 2".to_string(),
        url: "http://gerrit2:8080".to_string(),
        username: "user2".to_string(),
        password_encrypted: "pass2".to_string(),
        version: "3.9.0".to_string(),
        is_active: false, // Initially not active
        last_connected: None,
        connection_status: ConnectionStatus::Disconnected,
        polling_interval: 600,
        max_changes: 50,
        created_at: now.clone(),
        updated_at: now.clone(),
    };
    
    // Store both instances
    db.store_gerrit_instance(&instance1).expect("Failed to store instance1");
    db.store_gerrit_instance(&instance2).expect("Failed to store instance2");
    
    // Verify both are initially inactive
    let instances = db.get_all_gerrit_instances().expect("Failed to get instances");
    assert_eq!(instances.len(), 2);
    assert!(!instances.iter().any(|i| i.is_active));
    
    // Set instance1 as active
    let mut active_instance1 = instance1.clone();
    active_instance1.is_active = true;
    db.store_gerrit_instance(&active_instance1).expect("Failed to update instance1");
    
    // Verify instance1 is now active
    let retrieved_instance1 = db.get_gerrit_instance(&instance1_id)
        .expect("Failed to get instance1")
        .expect("Instance1 not found");
    assert!(retrieved_instance1.is_active);
    
    // Verify instance2 is still inactive
    let retrieved_instance2 = db.get_gerrit_instance(&instance2_id)
        .expect("Failed to get instance2")
        .expect("Instance2 not found");
    assert!(!retrieved_instance2.is_active);
    
    // Now set instance2 as active and instance1 as inactive (simulating the set_active logic)
    let mut updated_instance1 = active_instance1.clone();
    updated_instance1.is_active = false;
    let mut updated_instance2 = instance2.clone();
    updated_instance2.is_active = true;
    
    db.store_gerrit_instance(&updated_instance1).expect("Failed to update instance1");
    db.store_gerrit_instance(&updated_instance2).expect("Failed to update instance2");
    
    // Verify the switch worked
    let final_instance1 = db.get_gerrit_instance(&instance1_id)
        .expect("Failed to get final instance1")
        .expect("Final instance1 not found");
    assert!(!final_instance1.is_active);
    
    let final_instance2 = db.get_gerrit_instance(&instance2_id)
        .expect("Failed to get final instance2")
        .expect("Final instance2 not found");
    assert!(final_instance2.is_active);
    
    // Verify only one instance is active
    let final_instances = db.get_all_gerrit_instances().expect("Failed to get final instances");
    let active_count = final_instances.iter().filter(|i| i.is_active).count();
    assert_eq!(active_count, 1);
    
    println!("✅ Active instance persistence test passed!");
}

#[tokio::test]
async fn test_active_instance_retrieval() {
    // Test finding the active instance from a list
    let db = Database::new(":memory:").expect("Failed to create test database");
    
    db.init_schema().expect("Failed to initialize schema");
    db.init_gerrit_schema().expect("Failed to initialize Gerrit schema");
    
    let now = Utc::now().to_rfc3339();
    
    // Create test instances with one active
    let active_id = Uuid::new_v4().to_string();
    let inactive_id = Uuid::new_v4().to_string();
    
    let active_instance = GerritInstance {
        id: active_id.clone(),
        name: "Active Instance".to_string(),
        url: "http://active:8080".to_string(),
        username: "active_user".to_string(),
        password_encrypted: "active_pass".to_string(),
        version: "3.8.0".to_string(),
        is_active: true, // This one is active
        last_connected: Some(now.clone()),
        connection_status: ConnectionStatus::Connected,
        polling_interval: 300,
        max_changes: 100,
        created_at: now.clone(),
        updated_at: now.clone(),
    };
    
    let inactive_instance = GerritInstance {
        id: inactive_id.clone(),
        name: "Inactive Instance".to_string(),
        url: "http://inactive:8080".to_string(),
        username: "inactive_user".to_string(),
        password_encrypted: "inactive_pass".to_string(),
        version: "3.9.0".to_string(),
        is_active: false, // This one is inactive
        last_connected: None,
        connection_status: ConnectionStatus::Disconnected,
        polling_interval: 600,
        max_changes: 50,
        created_at: now.clone(),
        updated_at: now.clone(),
    };
    
    db.store_gerrit_instance(&active_instance).expect("Failed to store active instance");
    db.store_gerrit_instance(&inactive_instance).expect("Failed to store inactive instance");
    
    // Get all instances and find the active one
    let instances = db.get_all_gerrit_instances().expect("Failed to get instances");
    let active_found = instances.iter().find(|i| i.is_active);
    
    assert!(active_found.is_some());
    let active = active_found.unwrap();
    assert_eq!(active.id, active_id);
    assert_eq!(active.name, "Active Instance");
    assert!(active.is_active);
    
    println!("✅ Active instance retrieval test passed!");
}