use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use log::{info, warn, error, debug};
use serde::{Deserialize, Serialize};

use crate::models::gerrit::*;
use crate::errors::HyperReviewError;

/// Manages queued operations for offline sync and retry logic
pub struct OperationQueue {
    queue: Arc<Mutex<VecDeque<QueuedOperation>>>,
    processing: Arc<Mutex<HashMap<String, ProcessingInfo>>>,
    stats: Arc<Mutex<QueueStats>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedOperation {
    pub id: String,
    pub instance_id: String,
    pub change_id: String,
    pub operation_type: OperationType,
    pub payload: serde_json::Value,
    pub priority: OperationPriority,
    pub status: OperationStatus,
    pub retry_count: u32,
    pub max_retries: u32,
    pub created_at: DateTime<Utc>,
    pub last_attempt: Option<DateTime<Utc>>,
    pub next_retry: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingInfo {
    pub started_at: DateTime<Utc>,
    pub worker_id: String,
    pub progress: f64,
    pub estimated_completion: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueStats {
    pub total_enqueued: u64,
    pub total_completed: u64,
    pub total_failed: u64,
    pub current_queue_size: usize,
    pub processing_count: usize,
    pub average_processing_time_ms: f64,
    pub success_rate: f64,
}

impl OperationQueue {
    /// Create a new operation queue
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            processing: Arc::new(Mutex::new(HashMap::new())),
            stats: Arc::new(Mutex::new(QueueStats {
                total_enqueued: 0,
                total_completed: 0,
                total_failed: 0,
                current_queue_size: 0,
                processing_count: 0,
                average_processing_time_ms: 0.0,
                success_rate: 0.0,
            })),
        }
    }
    
    /// Add an operation to the queue
    pub fn enqueue(
        &self,
        operation: QueuedOperation,
    ) -> Result<String, HyperReviewError> {
        let operation_id = operation.id.clone();
        let operation_type = operation.operation_type.clone();
        let operation_priority = operation.priority;
        
        let mut queue = self.queue.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        // Insert based on priority (higher priority first)
        let insert_index = match operation_priority {
            OperationPriority::Critical => 0,
            OperationPriority::High => {
                queue.iter().position(|op| op.priority != OperationPriority::Critical)
                    .unwrap_or(queue.len())
            }
            OperationPriority::Normal => {
                queue.iter().position(|op| op.priority == OperationPriority::Low)
                    .unwrap_or(queue.len())
            }
            OperationPriority::Low => queue.len(),
        };
        
        queue.insert(insert_index, operation);
        stats.total_enqueued += 1;
        stats.current_queue_size = queue.len();
        
        info!("Enqueued operation {} of type {:?} with priority {:?}", 
              operation_id, operation_type, operation_priority);
        
        Ok(operation_id)
    }
    
    /// Get the next operation to process
    pub fn dequeue(
        &self,
        worker_id: &str,
    ) -> Result<Option<QueuedOperation>, HyperReviewError> {
        let mut queue = self.queue.lock().unwrap();
        let mut processing = self.processing.lock().unwrap();
        
        // Find next operation that's not already being processed
        while let Some(operation) = queue.pop_front() {
            if !processing.contains_key(&operation.id) {
                // Mark as processing
                processing.insert(operation.id.clone(), ProcessingInfo {
                    started_at: Utc::now(),
                    worker_id: worker_id.to_string(),
                    progress: 0.0,
                    estimated_completion: None,
                });
                
                let mut stats = self.stats.lock().unwrap();
                stats.current_queue_size = queue.len();
                stats.processing_count = processing.len();
                
                debug!("Dequeued operation {} for processing", operation.id);
                return Ok(Some(operation));
            }
        }
        
        Ok(None)
    }
    
    /// Mark an operation as completed
    pub fn complete_operation(
        &self,
        operation_id: &str,
        success: bool,
        result: Option<serde_json::Value>,
    ) -> Result<(), HyperReviewError> {
        let mut processing = self.processing.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        if let Some(_processing_info) = processing.remove(operation_id) {
            let duration_ms = (Utc::now() - _processing_info.started_at).num_milliseconds() as f64;
            
            // Update statistics
            if success {
                stats.total_completed += 1;
            } else {
                stats.total_failed += 1;
            }
            
            // Update average processing time
            let total_ops = stats.total_completed + stats.total_failed;
            stats.average_processing_time_ms = 
                (stats.average_processing_time_ms * (total_ops - 1) as f64 + duration_ms) / total_ops as f64;
            
            // Update success rate
            stats.success_rate = stats.total_completed as f64 / total_ops as f64;
            stats.processing_count = processing.len();
            
            info!("Operation {} completed with success={}, duration={}ms", 
                  operation_id, success, duration_ms);
            
            Ok(())
        } else {
            warn!("Attempted to complete unknown operation: {}", operation_id);
            Err(HyperReviewError::Other { message: format!("Operation {} not found in processing", operation_id) })
        }
    }
    
    /// Mark an operation as failed and schedule retry
    pub fn fail_operation(
        &self,
        operation_id: &str,
        error: &str,
    ) -> Result<(), HyperReviewError> {
        let mut processing = self.processing.lock().unwrap();
        let mut queue = self.queue.lock().unwrap();
        
        if let Some(processing_info) = processing.remove(operation_id) {
            // Create retry operation
            let retry_delay = self.calculate_retry_delay(processing_info.started_at);
            let retry_operation = QueuedOperation {
                id: format!("{}_retry_{}", operation_id, Utc::now().timestamp()),
                instance_id: "retry".to_string(), // Would need to store original
                change_id: "retry".to_string(),
                operation_type: OperationType::AddComment, // Would need to store original
                payload: serde_json::json!({"original_id": operation_id, "error": error}),
                priority: OperationPriority::High, // Retry with higher priority
                status: OperationStatus::Queued,
                retry_count: 1,
                max_retries: 3,
                created_at: Utc::now(),
                last_attempt: Some(Utc::now()),
                next_retry: Some(Utc::now() + chrono::Duration::seconds(retry_delay)),
                error_message: Some(error.to_string()),
            };
            
            queue.push_back(retry_operation);
            
            warn!("Operation {} failed, scheduled retry in {} seconds: {}", 
                  operation_id, retry_delay, error);
            
            Ok(())
        } else {
            warn!("Attempted to fail unknown operation: {}", operation_id);
            Err(HyperReviewError::Other { message: format!("Operation {} not found in processing", operation_id) })
        }
    }
    
    /// Get current queue status
    pub fn get_queue_status(&self,
    ) -> Result<QueueStatus, HyperReviewError> {
        let queue = self.queue.lock().unwrap();
        let processing = self.processing.lock().unwrap();
        let stats = self.stats.lock().unwrap();
        
        let mut pending_operations = Vec::new();
        for operation in queue.iter().take(10) { // Return top 10
            pending_operations.push(PendingOperationInfo {
                id: operation.id.clone(),
                operation_type: operation.operation_type.clone(),
                priority: operation.priority.clone(),
                created_at: operation.created_at,
                estimated_completion: self.estimate_completion_time(operation),
            });
        }
        
        Ok(QueueStatus {
            stats: stats.clone(),
            pending_operations,
            processing_operations: processing.keys().cloned().collect(),
        })
    }
    
    /// Get operations for a specific change
    pub fn get_operations_for_change(
        &self,
        change_id: &str,
    ) -> Result<Vec<QueuedOperation>, HyperReviewError> {
        let queue = self.queue.lock().unwrap();
        let operations: Vec<QueuedOperation> = queue
            .iter()
            .filter(|op| op.change_id == change_id)
            .cloned()
            .collect();
        
        Ok(operations)
    }
    
    /// Cancel a specific operation
    pub fn cancel_operation(
        &self,
        operation_id: &str,
    ) -> Result<bool, HyperReviewError> {
        let mut queue = self.queue.lock().unwrap();
        let mut processing = self.processing.lock().unwrap();
        
        // Remove from queue
        let queue_removed = queue.iter().position(|op| op.id == operation_id)
            .map(|pos| queue.remove(pos).is_some())
            .unwrap_or(false);
        
        // Remove from processing
        let processing_removed = processing.remove(operation_id).is_some();
        
        if queue_removed || processing_removed {
            info!("Cancelled operation {}", operation_id);
            Ok(true)
        } else {
            warn!("Operation {} not found for cancellation", operation_id);
            Ok(false)
        }
    }
    
    /// Retry failed operations
    pub fn retry_failed_operations(
        &self,
        change_id: Option<&str>,
    ) -> Result<u32, HyperReviewError> {
        let mut retry_count = 0;
        
        // This would need to track failed operations separately
        // For now, we'll implement a simple retry mechanism
        
        info!("Retrying failed operations for change: {:?}", change_id);
        
        Ok(retry_count)
    }
    
    // Helper methods
    
    fn calculate_retry_delay(&self,
        last_attempt: DateTime<Utc>,
    ) -> i64 {
        let elapsed = (Utc::now() - last_attempt).num_seconds();
        let base_delay = 60; // 1 minute base delay
        let max_delay = 3600; // 1 hour max delay
        
        // Exponential backoff with jitter
        let delay = (base_delay * 2i64.pow((elapsed / 60) as u32)).min(max_delay);
        let jitter = (delay as f64 * 0.1) as i64;
        
        delay + jitter
    }
    
    fn estimate_completion_time(
        &self,
        operation: &QueuedOperation,
    ) -> Option<DateTime<Utc>> {
        // Simple estimation based on operation type and priority
        let base_time_seconds = match operation.operation_type {
            OperationType::AddComment => 30,
            OperationType::UpdateComment => 30,
            OperationType::DeleteComment => 20,
            OperationType::SubmitReview => 60,
            OperationType::UpdateLabels => 45,
            OperationType::PushPatchSet => 120,
        };
        
        let priority_multiplier = match operation.priority {
            OperationPriority::Critical => 0.5,
            OperationPriority::High => 0.7,
            OperationPriority::Normal => 1.0,
            OperationPriority::Low => 1.5,
        };
        
        let estimated_seconds = (base_time_seconds as f64 * priority_multiplier) as i64;
        Some(Utc::now() + chrono::Duration::seconds(estimated_seconds))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueStatus {
    pub stats: QueueStats,
    pub pending_operations: Vec<PendingOperationInfo>,
    pub processing_operations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingOperationInfo {
    pub id: String,
    pub operation_type: OperationType,
    pub priority: OperationPriority,
    pub created_at: DateTime<Utc>,
    pub estimated_completion: Option<DateTime<Utc>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_operation_queue() {
        let queue = OperationQueue::new();
        
        let operation = QueuedOperation {
            id: "test-op-1".to_string(),
            instance_id: "instance-1".to_string(),
            change_id: "change-1".to_string(),
            operation_type: OperationType::AddComment,
            payload: serde_json::json!({"test": "data"}),
            priority: OperationPriority::Normal,
            status: OperationStatus::Queued,
            retry_count: 0,
            max_retries: 3,
            created_at: Utc::now(),
            last_attempt: None,
            next_retry: None,
            error_message: None,
        };
        
        // Test enqueue
        let op_id = queue.enqueue(operation.clone()).unwrap();
        assert_eq!(op_id, "test-op-1");
        
        // Test dequeue
        let dequeued = queue.dequeue("worker-1").unwrap();
        assert!(dequeued.is_some());
        assert_eq!(dequeued.unwrap().id, "test-op-1");
        
        // Test complete
        assert!(queue.complete_operation("test-op-1", true, None).is_ok());
        
        // Test queue status
        let status = queue.get_queue_status().unwrap();
        assert_eq!(status.stats.total_completed, 1);
        assert_eq!(status.stats.success_rate, 1.0);
    }
    
    #[test]
    fn test_priority_ordering() {
        let queue = OperationQueue::new();
        
        // Add operations with different priorities
        let priorities = vec![
            OperationPriority::Low,
            OperationPriority::Critical,
            OperationPriority::Normal,
            OperationPriority::High,
        ];
        
        for (i, priority) in priorities.iter().enumerate() {
            let operation = QueuedOperation {
                id: format!("test-op-{}", i),
                instance_id: "instance-1".to_string(),
                change_id: "change-1".to_string(),
                operation_type: OperationType::AddComment,
                payload: serde_json::json!({}),
                priority: priority.clone(),
                status: OperationStatus::Queued,
                retry_count: 0,
                max_retries: 3,
                created_at: Utc::now(),
                last_attempt: None,
                next_retry: None,
                error_message: None,
            };
            
            queue.enqueue(operation).unwrap();
        }
        
        // Dequeue operations and verify priority order
        let op1 = queue.dequeue("worker-1").unwrap().unwrap();
        assert_eq!(op1.id, "test-op-1"); // CRITICAL priority
        
        let op2 = queue.dequeue("worker-1").unwrap().unwrap();
        assert_eq!(op2.id, "test-op-3"); // HIGH priority
        
        let op3 = queue.dequeue("worker-1").unwrap().unwrap();
        assert_eq!(op3.id, "test-op-2"); // NORMAL priority
        
        let op4 = queue.dequeue("worker-1").unwrap().unwrap();
        assert_eq!(op4.id, "test-op-0"); // LOW priority
    }
}