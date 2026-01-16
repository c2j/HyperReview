// Integration tests for Gerrit database operations
// Tests the complete CRUD operations for Gerrit instances and changes

use hyperreview_lib::storage::sqlite::Database;
use hyperreview_lib::models::gerrit::{
    GerritInstance, GerritChange, ConnectionStatus, ChangeStatus, 
    ImportStatus, ConflictStatus, GerritUser
};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::Utc;

#[tokio::test]
async fn test_gerrit_instance_crud_operations() {
    // Create in-memory database for testing
    let db = Database::new(":memory:").expect("Failed to create test database");
    
    // Initialize schemas
    db.init_schema().expect("Failed to initialize schema");
    db.init_gerrit_schema().expect("Failed to initialize Gerrit schema");
    
    // Test data
    let instance_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    let test_instance = GerritInstance {
        id: instance_id.clone(),
        name: "Test Gerrit Server".to_string(),
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
    
    // Test CREATE
    println!("Testing Gerrit instance creation...");
    db.store_gerrit_instance(&test_instance)
        .expect("Failed to store Gerrit instance");
    
    // Test READ (single)
    println!("Testing Gerrit instance retrieval...");
    let retrieved_instance = db.get_gerrit_instance(&instance_id)
        .expect("Failed to get Gerrit instance")
        .expect("Instance not found");
    
    assert_eq!(retrieved_instance.id, test_instance.id);
    assert_eq!(retrieved_instance.name, test_instance.name);
    assert_eq!(retrieved_instance.url, test_instance.url);
    assert_eq!(retrieved_instance.username, test_instance.username);
    assert_eq!(retrieved_instance.is_active, test_instance.is_active);
    assert_eq!(retrieved_instance.connection_status, test_instance.connection_status);
    
    // Test READ (all)
    println!("Testing all Gerrit instances retrieval...");
    let all_instances = db.get_all_gerrit_instances()
        .expect("Failed to get all Gerrit instances");
    
    assert_eq!(all_instances.len(), 1);
    assert_eq!(all_instances[0].id, test_instance.id);
    
    // Test UPDATE (via store_gerrit_instance with same ID)
    println!("Testing Gerrit instance update...");
    let mut updated_instance = test_instance.clone();
    updated_instance.name = "Updated Test Server".to_string();
    updated_instance.connection_status = ConnectionStatus::Disconnected;
    updated_instance.is_active = false;
    
    db.store_gerrit_instance(&updated_instance)
        .expect("Failed to update Gerrit instance");
    
    let retrieved_updated = db.get_gerrit_instance(&instance_id)
        .expect("Failed to get updated instance")
        .expect("Updated instance not found");
    
    assert_eq!(retrieved_updated.name, "Updated Test Server");
    assert_eq!(retrieved_updated.connection_status, ConnectionStatus::Disconnected);
    assert_eq!(retrieved_updated.is_active, false);
    
    // Test DELETE
    println!("Testing Gerrit instance deletion...");
    let deleted = db.delete_gerrit_instance(&instance_id)
        .expect("Failed to delete Gerrit instance");
    
    assert!(deleted, "Instance should have been deleted");
    
    let not_found = db.get_gerrit_instance(&instance_id)
        .expect("Failed to query for deleted instance");
    
    assert!(not_found.is_none(), "Instance should not exist after deletion");
    
    println!("✅ Gerrit instance CRUD operations test passed!");
}

#[tokio::test]
async fn test_gerrit_change_crud_operations() {
    // Create in-memory database for testing
    let db = Database::new(":memory:").expect("Failed to create test database");
    
    // Initialize schemas
    db.init_schema().expect("Failed to initialize schema");
    db.init_gerrit_schema().expect("Failed to initialize Gerrit schema");
    
    // Create a test instance first
    let instance_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    let test_instance = GerritInstance {
        id: instance_id.clone(),
        name: "Test Server".to_string(),
        url: "http://test:8080".to_string(),
        username: "testuser".to_string(),
        password_encrypted: "testpass".to_string(),
        version: "3.8.0".to_string(),
        is_active: true,
        last_connected: Some(now.clone()),
        connection_status: ConnectionStatus::Connected,
        polling_interval: 300,
        max_changes: 100,
        created_at: now.clone(),
        updated_at: now.clone(),
    };
    
    db.store_gerrit_instance(&test_instance)
        .expect("Failed to store test instance");
    
    // Test data for change
    let change_id = Uuid::new_v4().to_string();
    let gerrit_change_id = "I1234567890abcdef".to_string();
    
    let test_change = GerritChange {
        id: change_id.clone(),
        change_id: gerrit_change_id.clone(),
        instance_id: instance_id.clone(),
        project: "test-project".to_string(),
        branch: "main".to_string(),
        subject: "Test change for database operations".to_string(),
        status: ChangeStatus::New,
        owner: GerritUser {
            account_id: 1001,
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            username: Some("testuser".to_string()),
            avatar_url: None,
        },
        created: now.clone(),
        updated: now.clone(),
        insertions: 42,
        deletions: 13,
        current_revision: "abc123def456".to_string(),
        current_patch_set_num: 1,
        patch_sets: Vec::new(),
        files: Vec::new(),
        total_files: 3,
        reviewed_files: 1,
        local_comments: 2,
        remote_comments: 5,
        import_status: ImportStatus::Imported,
        last_sync: Some(now.clone()),
        conflict_status: ConflictStatus::None,
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("test_key".to_string(), "test_value".to_string());
            meta
        },
    };
    
    // Test CREATE
    println!("Testing Gerrit change creation...");
    db.store_gerrit_change(&test_change)
        .expect("Failed to store Gerrit change");
    
    // Test READ (by ID)
    println!("Testing Gerrit change retrieval by ID...");
    let retrieved_change = db.get_gerrit_change(&change_id)
        .expect("Failed to get Gerrit change")
        .expect("Change not found");
    
    assert_eq!(retrieved_change.id, test_change.id);
    assert_eq!(retrieved_change.change_id, test_change.change_id);
    assert_eq!(retrieved_change.instance_id, test_change.instance_id);
    assert_eq!(retrieved_change.project, test_change.project);
    assert_eq!(retrieved_change.subject, test_change.subject);
    assert_eq!(retrieved_change.status, test_change.status);
    assert_eq!(retrieved_change.owner.name, test_change.owner.name);
    assert_eq!(retrieved_change.insertions, test_change.insertions);
    assert_eq!(retrieved_change.deletions, test_change.deletions);
    assert_eq!(retrieved_change.import_status, test_change.import_status);
    
    // Test READ (by Gerrit Change-ID)
    println!("Testing Gerrit change retrieval by Change-ID...");
    let retrieved_by_change_id = db.get_gerrit_change(&gerrit_change_id)
        .expect("Failed to get change by Change-ID")
        .expect("Change not found by Change-ID");
    
    assert_eq!(retrieved_by_change_id.id, test_change.id);
    assert_eq!(retrieved_by_change_id.change_id, test_change.change_id);
    
    // Test READ (all changes for instance)
    println!("Testing all changes retrieval for instance...");
    let instance_changes = db.get_gerrit_changes_for_instance(&instance_id)
        .expect("Failed to get changes for instance");
    
    assert_eq!(instance_changes.len(), 1);
    assert_eq!(instance_changes[0].id, test_change.id);
    
    // Test UPDATE
    println!("Testing Gerrit change update...");
    let mut updated_change = test_change.clone();
    updated_change.subject = "Updated test change subject".to_string();
    updated_change.status = ChangeStatus::Merged;
    updated_change.reviewed_files = 3;
    updated_change.import_status = ImportStatus::Outdated;
    updated_change.conflict_status = ConflictStatus::CommentsPending;
    
    db.update_gerrit_change(&updated_change)
        .expect("Failed to update Gerrit change");
    
    let retrieved_updated = db.get_gerrit_change(&change_id)
        .expect("Failed to get updated change")
        .expect("Updated change not found");
    
    assert_eq!(retrieved_updated.subject, "Updated test change subject");
    assert_eq!(retrieved_updated.status, ChangeStatus::Merged);
    assert_eq!(retrieved_updated.reviewed_files, 3);
    assert_eq!(retrieved_updated.import_status, ImportStatus::Outdated);
    assert_eq!(retrieved_updated.conflict_status, ConflictStatus::CommentsPending);
    
    // Test DELETE
    println!("Testing Gerrit change deletion...");
    let deleted = db.delete_gerrit_change(&change_id)
        .expect("Failed to delete Gerrit change");
    
    assert!(deleted, "Change should have been deleted");
    
    let not_found = db.get_gerrit_change(&change_id)
        .expect("Failed to query for deleted change");
    
    assert!(not_found.is_none(), "Change should not exist after deletion");
    
    // Verify cascade deletion doesn't affect instance
    let instance_still_exists = db.get_gerrit_instance(&instance_id)
        .expect("Failed to check instance after change deletion");
    
    assert!(instance_still_exists.is_some(), "Instance should still exist after change deletion");
    
    println!("✅ Gerrit change CRUD operations test passed!");
}

#[tokio::test]
async fn test_gerrit_enum_serialization() {
    // Test that our enum serialization/deserialization works correctly
    println!("Testing Gerrit enum serialization...");
    
    // Test ConnectionStatus
    assert_eq!(ConnectionStatus::Connected.to_string(), "Connected");
    assert_eq!(ConnectionStatus::from_string("Connected"), ConnectionStatus::Connected);
    assert_eq!(ConnectionStatus::from_string("invalid"), ConnectionStatus::Disconnected);
    
    // Test ChangeStatus
    assert_eq!(ChangeStatus::New.to_string(), "new");
    assert_eq!(ChangeStatus::from_string("NEW"), ChangeStatus::New);
    assert_eq!(ChangeStatus::from_string("merged"), ChangeStatus::Merged);
    assert_eq!(ChangeStatus::from_string("invalid"), ChangeStatus::New);
    
    // Test ImportStatus
    assert_eq!(ImportStatus::Imported.to_string(), "imported");
    assert_eq!(ImportStatus::from_string("IMPORTED"), ImportStatus::Imported);
    assert_eq!(ImportStatus::from_string("failed"), ImportStatus::Failed);
    assert_eq!(ImportStatus::from_string("invalid"), ImportStatus::Pending);
    
    // Test ConflictStatus
    assert_eq!(ConflictStatus::None.to_string(), "none");
    assert_eq!(ConflictStatus::from_string("NONE"), ConflictStatus::None);
    assert_eq!(ConflictStatus::from_string("comments_pending"), ConflictStatus::CommentsPending);
    assert_eq!(ConflictStatus::from_string("invalid"), ConflictStatus::None);
    
    println!("✅ Gerrit enum serialization test passed!");
}

#[tokio::test]
async fn test_gerrit_database_schema_initialization() {
    // Test that schema initialization works correctly
    println!("Testing Gerrit database schema initialization...");
    
    let db = Database::new(":memory:").expect("Failed to create test database");
    
    // Initialize main schema first
    db.init_schema().expect("Failed to initialize main schema");
    
    // Initialize Gerrit schema
    db.init_gerrit_schema().expect("Failed to initialize Gerrit schema");
    
    // Test that we can create instances and changes (schema exists)
    let instance_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    let test_instance = GerritInstance {
        id: instance_id.clone(),
        name: "Schema Test".to_string(),
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
    
    // This should work if schema was created correctly
    db.store_gerrit_instance(&test_instance)
        .expect("Failed to store instance - schema may not be initialized correctly");
    
    let retrieved = db.get_gerrit_instance(&instance_id)
        .expect("Failed to retrieve instance")
        .expect("Instance not found");
    
    assert_eq!(retrieved.name, "Schema Test");
    
    println!("✅ Gerrit database schema initialization test passed!");
}

#[tokio::test]
async fn test_gerrit_multiple_instances_and_changes() {
    // Test handling multiple instances and changes
    println!("Testing multiple Gerrit instances and changes...");
    
    let db = Database::new(":memory:").expect("Failed to create test database");
    db.init_schema().expect("Failed to initialize schema");
    db.init_gerrit_schema().expect("Failed to initialize Gerrit schema");
    
    let now = Utc::now().to_rfc3339();
    
    // Create multiple instances
    let instance1_id = Uuid::new_v4().to_string();
    let instance2_id = Uuid::new_v4().to_string();
    
    let instance1 = GerritInstance {
        id: instance1_id.clone(),
        name: "Instance 1".to_string(),
        url: "http://gerrit1:8080".to_string(),
        username: "user1".to_string(),
        password_encrypted: "pass1".to_string(),
        version: "3.8.0".to_string(),
        is_active: true,
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
        is_active: false,
        last_connected: None,
        connection_status: ConnectionStatus::Disconnected,
        polling_interval: 600,
        max_changes: 50,
        created_at: now.clone(),
        updated_at: now.clone(),
    };
    
    db.store_gerrit_instance(&instance1).expect("Failed to store instance1");
    db.store_gerrit_instance(&instance2).expect("Failed to store instance2");
    
    // Create changes for each instance
    let change1 = GerritChange {
        id: Uuid::new_v4().to_string(),
        change_id: "I111111".to_string(),
        instance_id: instance1_id.clone(),
        project: "project1".to_string(),
        branch: "main".to_string(),
        subject: "Change for instance 1".to_string(),
        status: ChangeStatus::New,
        owner: GerritUser {
            account_id: 1001,
            name: "User 1".to_string(),
            email: "user1@example.com".to_string(),
            username: Some("user1".to_string()),
            avatar_url: None,
        },
        created: now.clone(),
        updated: now.clone(),
        insertions: 10,
        deletions: 5,
        current_revision: "rev1".to_string(),
        current_patch_set_num: 1,
        patch_sets: Vec::new(),
        files: Vec::new(),
        total_files: 1,
        reviewed_files: 0,
        local_comments: 0,
        remote_comments: 0,
        import_status: ImportStatus::Imported,
        last_sync: Some(now.clone()),
        conflict_status: ConflictStatus::None,
        metadata: HashMap::new(),
    };
    
    let change2 = GerritChange {
        id: Uuid::new_v4().to_string(),
        change_id: "I222222".to_string(),
        instance_id: instance2_id.clone(),
        project: "project2".to_string(),
        branch: "develop".to_string(),
        subject: "Change for instance 2".to_string(),
        status: ChangeStatus::Merged,
        owner: GerritUser {
            account_id: 2002,
            name: "User 2".to_string(),
            email: "user2@example.com".to_string(),
            username: Some("user2".to_string()),
            avatar_url: None,
        },
        created: now.clone(),
        updated: now.clone(),
        insertions: 20,
        deletions: 10,
        current_revision: "rev2".to_string(),
        current_patch_set_num: 2,
        patch_sets: Vec::new(),
        files: Vec::new(),
        total_files: 2,
        reviewed_files: 2,
        local_comments: 1,
        remote_comments: 3,
        import_status: ImportStatus::Imported,
        last_sync: Some(now.clone()),
        conflict_status: ConflictStatus::None,
        metadata: HashMap::new(),
    };
    
    db.store_gerrit_change(&change1).expect("Failed to store change1");
    db.store_gerrit_change(&change2).expect("Failed to store change2");
    
    // Test retrieving all instances
    let all_instances = db.get_all_gerrit_instances().expect("Failed to get all instances");
    assert_eq!(all_instances.len(), 2);
    
    // Test retrieving changes for each instance
    let instance1_changes = db.get_gerrit_changes_for_instance(&instance1_id)
        .expect("Failed to get changes for instance1");
    assert_eq!(instance1_changes.len(), 1);
    assert_eq!(instance1_changes[0].project, "project1");
    
    let instance2_changes = db.get_gerrit_changes_for_instance(&instance2_id)
        .expect("Failed to get changes for instance2");
    assert_eq!(instance2_changes.len(), 1);
    assert_eq!(instance2_changes[0].project, "project2");
    
    // Test deleting an instance (should cascade delete its changes)
    let deleted = db.delete_gerrit_instance(&instance1_id)
        .expect("Failed to delete instance1");
    assert!(deleted);
    
    // Verify instance1 changes are gone
    let instance1_changes_after_delete = db.get_gerrit_changes_for_instance(&instance1_id)
        .expect("Failed to query changes after instance deletion");
    assert_eq!(instance1_changes_after_delete.len(), 0);
    
    // Verify instance2 and its changes still exist
    let remaining_instances = db.get_all_gerrit_instances().expect("Failed to get remaining instances");
    assert_eq!(remaining_instances.len(), 1);
    assert_eq!(remaining_instances[0].id, instance2_id);
    
    let instance2_changes_after_delete = db.get_gerrit_changes_for_instance(&instance2_id)
        .expect("Failed to get instance2 changes after deletion");
    assert_eq!(instance2_changes_after_delete.len(), 1);
    
    println!("✅ Multiple instances and changes test passed!");
}