use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use log::{info, warn};

use crate::errors::HyperReviewError;
use crate::services::encryption::EncryptionService;

/// Secure credential storage service for Gerrit instances
pub struct CredentialStore {
    encryption: Arc<EncryptionService>,
    cache: Arc<Mutex<HashMap<String, CachedCredential>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedCredential {
    pub username_encrypted: Vec<u8>,
    pub password_encrypted: Vec<u8>,
    pub cached_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct PlainCredential {
    pub username: String,
    pub password: String,
}

impl CredentialStore {
    /// Create a new credential store
    pub fn new(encryption: Arc<EncryptionService>) -> Self {
        Self {
            encryption,
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Store encrypted credentials
    pub fn store_credentials(
        &self,
        instance_id: &str,
        username: &str,
        password: &str,
    ) -> Result<(), HyperReviewError> {
        info!("Storing credentials for instance: {}", instance_id);
        
        // Validate credentials before encryption
        Self::validate_credentials(username, password)?;
        
        // Encrypt username
        let username_encrypted = self.encryption
            .encrypt(username.as_bytes())
            .map_err(|e| HyperReviewError::encryption(format!("Failed to encrypt username: {}", e)))?;
        
        // Encrypt password
        let password_encrypted = self.encryption
            .encrypt(password.as_bytes())
            .map_err(|e| HyperReviewError::encryption(format!("Failed to encrypt password: {}", e)))?;
        
        let cached_credential = CachedCredential {
            username_encrypted,
            password_encrypted,
            cached_at: chrono::Utc::now(),
            expires_at: chrono::Utc::now() + chrono::Duration::hours(24), // 24 hour cache
        };
        
        // Store in cache
        let mut cache = self.cache.lock().unwrap();
        cache.insert(instance_id.to_string(), cached_credential);
        
        info!("Credentials stored successfully for instance: {}", instance_id);
        Ok(())
    }
    
    /// Retrieve decrypted credentials
    pub fn get_credentials(
        &self,
        instance_id: &str,
    ) -> Result<Option<PlainCredential>, HyperReviewError> {
        let mut cache = self.cache.lock().unwrap();
        
        if let Some(cached) = cache.get(instance_id) {
            // Check if cache has expired
            if cached.expires_at < chrono::Utc::now() {
                cache.remove(instance_id);
                warn!("Credentials expired for instance: {}", instance_id);
                return Ok(None);
            }
            
            // Decrypt username
            let username_bytes = self.encryption
                .decrypt(&cached.username_encrypted)
                .map_err(|e| HyperReviewError::decryption(format!("Failed to decrypt username: {}", e)))?;
            
            let username = String::from_utf8(username_bytes)
                .map_err(|e| HyperReviewError::decryption(format!("Invalid UTF-8 in username: {}", e)))?;
            
            // Decrypt password
            let password_bytes = self.encryption
                .decrypt(&cached.password_encrypted)
                .map_err(|e| HyperReviewError::decryption(format!("Failed to decrypt password: {}", e)))?;
            
            let password = String::from_utf8(password_bytes)
                .map_err(|e| HyperReviewError::decryption(format!("Invalid UTF-8 in password: {}", e)))?;
            
            info!("Credentials retrieved successfully for instance: {}", instance_id);
            
            Ok(Some(PlainCredential {
                username,
                password,
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Remove credentials from cache
    pub fn remove_credentials(
        &self,
        instance_id: &str,
    ) -> Result<(), HyperReviewError> {
        let mut cache = self.cache.lock().unwrap();
        
        if cache.remove(instance_id).is_some() {
            info!("Credentials removed from cache for instance: {}", instance_id);
        }
        
        Ok(())
    }
    
    /// Clear all cached credentials
    pub fn clear_cache(
        &self,
    ) -> Result<(), HyperReviewError> {
        let mut cache = self.cache.lock().unwrap();
        let removed_count = cache.len();
        cache.clear();
        
        info!("Cleared {} cached credentials", removed_count);
        Ok(())
    }
    
    /// Validate credential format and content
    fn validate_credentials(
        username: &str,
        password: &str,
    ) -> Result<(), HyperReviewError> {
        if username.trim().is_empty() {
            return Err(HyperReviewError::validation("Username cannot be empty".to_string(), None));
        }
        
        if username.len() > 255 {
            return Err(HyperReviewError::validation("Username too long (max 255 characters)".to_string(), None));
        }
        
        if password.trim().is_empty() {
            return Err(HyperReviewError::validation("Password cannot be empty".to_string(), None));
        }
        
        if password.len() > 2048 {
            return Err(HyperReviewError::validation("Password too long (max 2048 characters)".to_string(), None));
        }
        
        // Basic password complexity check
        if password.len() < 8 {
            warn!("Password for instance is shorter than recommended 8 characters");
        }
        
        Ok(())
    }
    
    /// Get cache statistics
    pub fn get_cache_stats(
        &self,
    ) -> CredentialCacheStats {
        let cache = self.cache.lock().unwrap();
        
        CredentialCacheStats {
            total_entries: cache.len(),
            expired_entries: cache.values()
                .filter(|cred| cred.expires_at < chrono::Utc::now())
                .count(),
            valid_entries: cache.values()
                .filter(|cred| cred.expires_at >= chrono::Utc::now())
                .count(),
        }
    }
    
    /// Rotate encryption keys and re-encrypt all stored credentials
    pub fn rotate_keys(
        &self,
        new_encryption: Arc<EncryptionService>,
    ) -> Result<(), HyperReviewError> {
        info!("Starting credential key rotation");
        
        let mut cache = self.cache.lock().unwrap();
        let mut rotated_count = 0;
        
        for (instance_id, cached_credential) in cache.iter_mut() {
            // Decrypt with old key
            let username_plain = self.encryption
                .decrypt(&cached_credential.username_encrypted)
                .map_err(|e| HyperReviewError::decryption(format!("Failed to decrypt username during rotation: {}", e)))?;
            
            let password_plain = self.encryption
                .decrypt(&cached_credential.password_encrypted)
                .map_err(|e| HyperReviewError::decryption(format!("Failed to decrypt password during rotation: {}", e)))?;
            
            // Re-encrypt with new key
            cached_credential.username_encrypted = new_encryption
                .encrypt(&username_plain)
                .map_err(|e| HyperReviewError::encryption(format!("Failed to re-encrypt username: {}", e)))?;
            
            cached_credential.password_encrypted = new_encryption
                .encrypt(&password_plain)
                .map_err(|e| HyperReviewError::encryption(format!("Failed to re-encrypt password: {}", e)))?;
            
            rotated_count += 1;
        }
        
        info!("Successfully rotated keys for {} credentials", rotated_count);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CredentialCacheStats {
    pub total_entries: usize,
    pub expired_entries: usize,
    pub valid_entries: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_credential_store() {
        let encryption = Arc::new(EncryptionService::new(b"test_master_key").unwrap());
        let store = CredentialStore::new(encryption);
        
        // Test store and retrieve
        store.store_credentials("instance-1", "testuser", "testpass123").unwrap();
        
        let retrieved = store.get_credentials("instance-1").unwrap();
        assert!(retrieved.is_some());
        
        let creds = retrieved.unwrap();
        assert_eq!(creds.username, "testuser");
        assert_eq!(creds.password, "testpass123");
        
        // Test removal
        store.remove_credentials("instance-1").unwrap();
        let after_remove = store.get_credentials("instance-1").unwrap();
        assert!(after_remove.is_none());
    }
    
    #[test]
    fn test_credential_validation() {
        // Valid credentials
        assert!(CredentialStore::validate_credentials("validuser", "validpass123").is_ok());
        
        // Empty username
        assert!(CredentialStore::validate_credentials("", "password").is_err());
        
        // Empty password
        assert!(CredentialStore::validate_credentials("user", "").is_err());
        
        // Too long username
        let long_username = "a".repeat(300);
        assert!(CredentialStore::validate_credentials(&long_username, "password").is_err());
        
        // Too long password
        let long_password = "a".repeat(3000);
        assert!(CredentialStore::validate_credentials("user", &long_password).is_err());
    }
}