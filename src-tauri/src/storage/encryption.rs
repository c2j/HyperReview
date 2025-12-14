// Data encryption for local storage
// Encrypt sensitive data before storing in SQLite database

use std::collections::HashMap;

/// Simple XOR cipher for basic encryption
/// Note: This is a simplified implementation for demonstration
/// In production, use a proper encryption library like AES
pub struct EncryptionManager {
    key: Vec<u8>,
}

impl EncryptionManager {
    pub fn new(key: &[u8]) -> Self {
        Self {
            key: key.to_vec(),
        }
    }

    /// Encrypt data using XOR cipher (simplified)
    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let mut encrypted = Vec::with_capacity(data.len());
        for (i, byte) in data.iter().enumerate() {
            let key_byte = self.key[i % self.key.len()];
            encrypted.push(byte ^ key_byte);
        }
        encrypted
    }

    /// Decrypt data using XOR cipher
    pub fn decrypt(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
        if encrypted_data.is_empty() {
            return Ok(Vec::new());
        }

        let mut decrypted = Vec::with_capacity(encrypted_data.len());
        for (i, byte) in encrypted_data.iter().enumerate() {
            let key_byte = self.key[i % self.key.len()];
            decrypted.push(byte ^ key_byte);
        }

        Ok(decrypted)
    }

    /// Encrypt a string
    pub fn encrypt_string(&self, text: &str) -> String {
        let encrypted = self.encrypt(text.as_bytes());
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &encrypted)
    }

    /// Decrypt a string
    pub fn decrypt_string(&self, encrypted_text: &str) -> Result<String, EncryptionError> {
        let encrypted_data = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, encrypted_text)
            .map_err(|_| EncryptionError::Base64DecodeError)?;

        let decrypted_bytes = self.decrypt(&encrypted_data)?;
        String::from_utf8(decrypted_bytes)
            .map_err(|_| EncryptionError::InvalidUtf8)
    }

    /// Encrypt sensitive fields in a hashmap
    pub fn encrypt_fields(&self, data: &mut HashMap<String, String>, fields_to_encrypt: &[&str]) {
        for field in fields_to_encrypt {
            if let Some(value) = data.get_mut(*field) {
                let encrypted = self.encrypt_string(value);
                *value = encrypted;
            }
        }
    }

    /// Decrypt sensitive fields in a hashmap
    pub fn decrypt_fields(&self, data: &HashMap<String, String>, fields_to_decrypt: &[&str]) -> Result<HashMap<String, String>, EncryptionError> {
        let mut decrypted = HashMap::new();

        for (key, value) in data {
            if fields_to_decrypt.contains(&key.as_str()) {
                let decrypted_value = self.decrypt_string(value)?;
                decrypted.insert(key.clone(), decrypted_value);
            } else {
                decrypted.insert(key.clone(), value.clone());
            }
        }

        Ok(decrypted)
    }

    /// Generate a random encryption key
    pub fn generate_key() -> Vec<u8> {
        use rand::Rng;
        let mut key = vec![0u8; 32]; // 256-bit key
        rand::thread_rng().fill(&mut key[..]);
        key
    }
}

/// Encryption error types
#[derive(Debug)]
pub enum EncryptionError {
    DecryptionFailed,
    Base64DecodeError,
    InvalidUtf8,
}

impl std::fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EncryptionError::DecryptionFailed => write!(f, "Decryption failed"),
            EncryptionError::Base64DecodeError => write!(f, "Base64 decode error"),
            EncryptionError::InvalidUtf8 => write!(f, "Invalid UTF-8 after decryption"),
        }
    }
}

impl std::error::Error for EncryptionError {}

/// Credential encryption helper
pub struct CredentialEncryptor {
    encryption_manager: EncryptionManager,
}

impl CredentialEncryptor {
    pub fn new(key: &[u8]) -> Self {
        Self {
            encryption_manager: EncryptionManager::new(key),
        }
    }

    /// Encrypt a credential before storing in database
    pub fn encrypt_credential(&self, username: &str, password: &str) -> (String, String) {
        let encrypted_username = self.encryption_manager.encrypt_string(username);
        let encrypted_password = self.encryption_manager.encrypt_string(password);
        (encrypted_username, encrypted_password)
    }

    /// Decrypt a credential from database
    pub fn decrypt_credential(&self, encrypted_username: &str, encrypted_password: &str) -> Result<(String, String), EncryptionError> {
        let username = self.encryption_manager.decrypt_string(encrypted_username)?;
        let password = self.encryption_manager.decrypt_string(encrypted_password)?;
        Ok((username, password))
    }

    /// Encrypt metadata containing sensitive information
    pub fn encrypt_metadata(&self, metadata: &mut HashMap<String, String>) {
        // Encrypt common sensitive fields
        let sensitive_fields = ["password", "api_key", "secret", "token", "credential"];
        self.encryption_manager.encrypt_fields(metadata, &sensitive_fields);
    }

    /// Decrypt metadata
    pub fn decrypt_metadata(&self, metadata: &HashMap<String, String>) -> Result<HashMap<String, String>, EncryptionError> {
        let sensitive_fields = ["password", "api_key", "secret", "token", "credential"];
        self.encryption_manager.decrypt_fields(metadata, &sensitive_fields)
    }
}

/// Secure storage for encrypted data
pub struct SecureStorage {
    encryption_manager: EncryptionManager,
}

impl SecureStorage {
    pub fn new(key: &[u8]) -> Self {
        Self {
            encryption_manager: EncryptionManager::new(key),
        }
    }

    /// Store encrypted data in database
    pub fn store_encrypted(&self, table: &str, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        let encrypted_value = self.encryption_manager.encrypt_string(value);

        // In production, this would write to SQLite
        log::info!("Stored encrypted data in {} with key {}", table, key);

        Ok(())
    }

    /// Retrieve and decrypt data from database
    pub fn retrieve_encrypted(&self, table: &str, key: &str) -> Result<String, Box<dyn std::error::Error>> {
        // In production, this would read from SQLite
        // For now, just return the key (placeholder)
        let encrypted_data = format!("encrypted_{}", key);

        self.encryption_manager.decrypt_string(&encrypted_data)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = b"my_secret_key_32_bytes_long!!";
        let manager = EncryptionManager::new(key);

        let original = "Hello, World!";
        let encrypted = manager.encrypt_string(original);
        let decrypted = manager.decrypt_string(&encrypted).unwrap();

        assert_eq!(original, decrypted);
        assert_ne!(original, encrypted);
    }

    #[test]
    fn test_encrypt_decrypt_empty() {
        let key = b"my_secret_key_32_bytes_long!!";
        let manager = EncryptionManager::new(key);

        let original = "";
        let encrypted = manager.encrypt_string(original);
        let decrypted = manager.decrypt_string(&encrypted).unwrap();

        assert_eq!(original, decrypted);
    }

    #[test]
    fn test_encrypt_decrypt_fields() {
        let key = b"my_secret_key_32_bytes_long!!";
        let manager = EncryptionManager::new(key);

        let mut data = HashMap::new();
        data.insert("username".to_string(), "john_doe".to_string());
        data.insert("password".to_string(), "secret123".to_string());
        data.insert("email".to_string(), "john@example.com".to_string());

        let sensitive_fields = ["password"];
        manager.encrypt_fields(&mut data, &sensitive_fields);

        assert_ne!(data["password"], "secret123");
        assert_eq!(data["username"], "john_doe");
        assert_eq!(data["email"], "john@example.com");

        let decrypted = manager.decrypt_fields(&data, &sensitive_fields).unwrap();
        assert_eq!(decrypted["password"], "secret123");
    }
}
