use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use password_hash::{rand_core::RngCore, SaltString};
use std::error::Error;

use crate::errors::HyperReviewError;

/// Secure encryption service for Gerrit credentials and sensitive data
pub struct EncryptionService {
    cipher: Aes256Gcm,
}

impl EncryptionService {
    /// Create a new encryption service with a derived key
    pub fn new(master_key: &[u8]) -> Result<Self, Box<dyn Error>> {
        let argon2 = Argon2::default();
        let salt = b"hyperreview_gerrit_salt_2024";
        
        // Derive a 32-byte key using Argon2id
        let mut derived_key = vec![0u8; 32];
        argon2.hash_password_into(master_key, salt, &mut derived_key)
            .map_err(|e| HyperReviewError::hashing(format!("Argon2 key derivation failed: {}", e)))?;
        
        let key = Key::<Aes256Gcm>::from_slice(&derived_key);
        let cipher = Aes256Gcm::new(key);
        
        Ok(Self { cipher })
    }
    
    /// Encrypt sensitive data
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        // Generate a random nonce
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        // Encrypt the data
        let ciphertext = self.cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| HyperReviewError::encryption(format!("AES encryption failed: {}", e)))?;
        
        // Combine nonce and ciphertext for storage
        let mut result = Vec::new();
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);
        
        Ok(result)
    }
    
    /// Decrypt sensitive data
    pub fn decrypt(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        if encrypted_data.len() < 12 {
            return Err("Invalid encrypted data: too short".into());
        }
        
        // Extract nonce (first 12 bytes)
        let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        // Decrypt the data
        let plaintext = self.cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| HyperReviewError::decryption(format!("AES decryption failed: {}", e)))?;
        
        Ok(plaintext)
    }
    
    /// Generate a secure master key
    pub fn generate_master_key() -> Vec<u8> {
        let mut key = vec![0u8; 32];
        OsRng.fill_bytes(&mut key);
        key
    }
    
    /// Hash a password for secure storage (not encryption)
    pub fn hash_password(password: &str) -> Result<String, Box<dyn Error>> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| HyperReviewError::hashing(format!("Password hashing failed: {}", e)))?;
        
        Ok(password_hash.to_string())
    }
    
    /// Verify a password against a hash
    pub fn verify_password(password: &str, hash: &str) -> Result<bool, Box<dyn Error>> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| HyperReviewError::hashing(format!("Invalid password hash: {}", e)))?;
        
        let argon2 = Argon2::default();
        Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encryption_decryption() {
        let service = EncryptionService::new(b"test_master_key").unwrap();
        let plaintext = b"sensitive credential data";
        
        let encrypted = service.encrypt(plaintext).unwrap();
        assert_ne!(encrypted, plaintext);
        assert!(encrypted.len() > 12); // Should include nonce
        
        let decrypted = service.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }
    
    #[test]
    fn test_password_hashing() {
        let password = "test_password_123";
        let hash = EncryptionService::hash_password(password).unwrap();
        
        assert!(EncryptionService::verify_password(password, &hash).unwrap());
        assert!(!EncryptionService::verify_password("wrong_password", &hash).unwrap());
    }
    
    #[test]
    fn test_master_key_generation() {
        let key1 = EncryptionService::generate_master_key();
        let key2 = EncryptionService::generate_master_key();
        
        assert_eq!(key1.len(), 32);
        assert_eq!(key2.len(), 32);
        assert_ne!(key1, key2); // Should be random
    }
}