// Credential storage using OS Keychain
// Secure credential management

use std::collections::HashMap;

pub struct CredentialStore {
    // In production, would use keyring crate for OS keychain integration
    // For now, using in-memory storage (NOT for production)
    credentials: HashMap<String, Credential>,
}

#[derive(Debug, Clone)]
pub struct Credential {
    pub service: String,
    pub username: String,
    pub password: String,
}

impl CredentialStore {
    pub fn new() -> Self {
        Self {
            credentials: HashMap::new(),
        }
    }

    /// Store a credential securely
    pub fn store(&mut self, service: &str, username: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Storing credential for service: {}", service);

        // In production, would use OS keychain:
        // keyring::Entry::new(service, username)?.set_password(password)?;

        let key = format!("{}:{}", service, username);
        self.credentials.insert(key, Credential {
            service: service.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        });

        Ok(())
    }

    /// Retrieve a credential
    pub fn retrieve(&self, service: &str, username: &str) -> Result<Option<Credential>, Box<dyn std::error::Error>> {
        log::info!("Retrieving credential for service: {}", service);

        // In production, would use OS keychain:
        // let entry = keyring::Entry::new(service, username)?;
        // let password = entry.get_password()?;

        let key = format!("{}:{}", service, username);
        Ok(self.credentials.get(&key).cloned())
    }

    /// Delete a credential
    pub fn delete(&mut self, service: &str, username: &str) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Deleting credential for service: {}", service);

        let key = format!("{}:{}", service, username);
        self.credentials.remove(&key);

        Ok(())
    }

    /// List all stored services
    pub fn list_services(&self) -> Vec<String> {
        self.credentials.values()
            .map(|c| c.service.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect()
    }

    /// Check if credential exists
    pub fn has_credential(&self, service: &str, username: &str) -> bool {
        let key = format!("{}:{}", service, username);
        self.credentials.contains_key(&key)
    }
}

impl Default for CredentialStore {
    fn default() -> Self {
        Self::new()
    }
}
