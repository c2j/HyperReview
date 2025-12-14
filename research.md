# HyperReview Backend Research

## Part A: Tauri IPC Security Best Practices

### 1. Secure IPC Communication Patterns in Tauri v2

#### Overview
Tauri v2 introduces a capability-based security model that replaces the legacy allowlist approach. This model provides granular control over what operations each window/tab can perform.

#### Core Security Principles

**A. Capability-Based Permissions (Recommended)**
Instead of global allowlist, use `capabilities` to define permissions per window:

```json
// src-tauri/capabilities/review-window.json
{
  "identifier": "review-window-capability",
  "description": "Permissions for the main code review window",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "path:default",
    "fs:default",
    {
      "identifier": "fs:scope",
      "allow": [
        {
          "path": "$HOME/.hyperreview/repos/**",
          "description": "Allow access to opened repositories"
        }
      ],
      "deny": [
        {
          "path": "$HOME/.hyperreview/repos/**/node_modules/**",
          "description": "Deny access to dependency directories"
        }
      ]
    },
    {
      "identifier": "custom:restricted-commands",
      "commands": [
        "open_repo_dialog",
        "get_recent_repos",
        "get_file_diff"
      ],
      "scope": {
        "accept_files": ["**/*.rs", "**/*.ts", "**/*.js", "**/*.java", "**/*.sql", "**/*.xml"]
      }
    }
  ]
}
```

**B. Command Handler Security Pattern**
```rust
// src-tauri/src/commands/security.rs

use tauri::{command, State, AppHandle};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use once_cell::sync::Lazy;

// Validation state tracking
static RATE_LIMITER: Lazy<Mutex<RateLimiter>> = Lazy::new(|| {
    Mutex::new(RateLimiter::new())
});

#[derive(Debug)]
struct RateLimiter {
    requests: std::collections::HashMap<String, Vec<std::time::Instant>>,
    window_duration: std::time::Duration,
    max_requests: usize,
}

impl RateLimiter {
    fn new() -> Self {
        Self {
            requests: std::collections::HashMap::new(),
            window_duration: std::time::Duration::from_secs(60),
            max_requests: 100,
        }
    }

    fn is_allowed(&mut self, client_id: &str) -> bool {
        let now = std::time::Instant::now();
        let client_requests = self.requests.entry(client_id.to_string()).or_insert_with(Vec::new);

        // Remove old requests outside the window
        client_requests.retain(|&time| now.duration_since(time) < self.window_duration);

        if client_requests.len() >= self.max_requests {
            return false;
        }

        client_requests.push(now);
        true
    }
}

// Input validation using validator crate
#[derive(Debug, Deserialize)]
pub struct GetFileDiffCommand {
    pub file_id: String,
    #[validate(length(min = 1, max = 1000))]
    pub repo_path: String,
}

#[command]
pub async fn get_file_diff(
    app: AppHandle,
    command: GetFileDiffCommand,
) -> Result<Vec<DiffLine>, String> {
    // Rate limiting
    let client_id = "default"; // TODO: derive from window context
    let mut limiter = RATE_LIMITER.lock().map_err(|_| "Rate limiter poisoned")?;
    if !limiter.is_allowed(client_id) {
        return Err("Rate limit exceeded".to_string());
    }

    // Input sanitization
    let sanitized_path = sanitize_path(&command.repo_path)?;
    if !is_path_allowed(&sanitized_path) {
        return Err("Path not in allowed scope".to_string());
    }

    // Additional validation
    if !is_safe_file_id(&command.file_id) {
        return Err("Invalid file identifier".to_string());
    }

    // Execute with proper error handling
    execute_file_diff(&app, &sanitized_path, &command.file_id).await
        .map_err(|e| format!("Failed to get diff: {}", e))
}

// Path sanitization
fn sanitize_path(path: &str) -> Result<String, String> {
    // Remove null bytes
    if path.contains('\0') {
        return Err("Null bytes not allowed".to_string());
    }

    // Normalize path
    let normalized = std::path::Path::new(path)
        .canonicalize()
        .map_err(|_| "Invalid path".to_string())?;

    // Convert to string and check for traversal
    let path_str = normalized.to_string_lossy();

    if path_str.contains("..") {
        return Err("Path traversal detected".to_string());
    }

    Ok(path_str.to_string())
}

// Safe file ID validation
fn is_safe_file_id(file_id: &str) -> bool {
    // Only allow alphanumeric, hyphens, underscores, and forward slashes
    file_id.chars().all(|c| {
        c.is_alphanumeric() || c == '-' || c == '_' || c == '/'
    }) && file_id.len() <= 256
}

// Scope validation
fn is_path_allowed(path: &str) -> bool {
    // In production, check against configured allowed paths
    // This is a simplified example
    path.starts_with("/home") || path.starts_with("/Users") || path.starts_with("C:\\Users")
}
```

**C. Command Registration with Security**
```rust
// src-tauri/src/lib.rs

use tauri::{Builder, Manager};

fn main() {
    Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            greet, // Only non-sensitive commands should be in global handler
            commands::security::get_file_diff,
            commands::repo::open_repo_dialog,
            commands::analysis::get_checklist
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 2. Minimizing Attack Surface in tauri.conf.json

#### Allowlist Configuration (Legacy but still used)
```json
{
  "app": {
    "windows": [
      {
        "title": "hyperreview",
        "width": 800,
        "height": 600,
        "security": {
          "csp": "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; connect-src 'self' https:; font-src 'self' data:;"
        }
      }
    ],
    "security": {
      "csp": "default-src 'self';",
      "disableDynamicLoading": false,
      "dangerousRemoteDomainIpcAccess": []
    }
  },
  "plugins": {
    "shell": {
      "all": false,
      "execute": true,
      "sidecar": true,
      "open": true
    },
    "fs": {
      "all": false,
      "readFile": true,
      "writeFile": false,
      "readDir": true,
      "copyFile": false,
      "createDir": false,
      "removeDir": false,
      "removeFile": false,
      "renameFile": false,
      "exists": true
    },
    "path": {
      "all": true
    },
    "protocol": {
      "asset": true,
      "assetScope": [
        "**",
        "!**/node_modules/**",
        "!**/.git/**"
      ]
    }
  }
}
```

#### Modern Capability-Based Configuration
```json
// src-tauri/capabilities/main-capability.json
{
  "identifier": "main-capability",
  "description": "Main application window with minimal permissions",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "window:default",
    "window:close",
    "window:hide",
    "window:show",
    "window:maximize",
    "window:minimize",
    "window:unmaximize",
    "window:unminimize",
    "window:start-dragging",
    "path:default",
    {
      "identifier": "fs:read-only",
      "allow": [
        { "path": "$APPDATA/hyperreview/repos/**" },
        { "path": "$HOME/.hyperreview/**" }
      ],
      "deny": [
        { "path": "**/node_modules/**" },
        { "path": "**/.git/objects/**" },
        { "path": "**/.git/config" }
      ]
    },
    {
      "identifier": "custom:secure-commands",
      "commands": {
        "allow": ["get_recent_repos", "get_file_diff", "get_checklist"],
        "deny": ["system_exec", "write_config", "delete_repo"]
      },
      "scope": {
        "max_file_size": "10MB",
        "allowed_extensions": [".rs", ".ts", ".js", ".java", ".sql", ".xml", ".json", ".md"],
        "denied_paths": ["**/node_modules/**", "**/target/**", "**/.git/**"]
      }
    }
  ]
}
```

### 3. Input Validation and Sanitization in Rust Commands

#### Comprehensive Validation Strategy

```rust
// src-tauri/src/validation/mod.rs

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use std::collections::HashSet;
use regex::Regex;

/// Validated repository path
#[derive(Debug, Validate, Deserialize)]
pub struct RepoPath {
    #[validate(custom = "validate_repo_path")]
    pub path: String,
}

fn validate_repo_path(path: &str) -> Result<(), ValidationError> {
    // Check for null bytes
    if path.contains('\0') {
        return Err(ValidationError::new("null_bytes_not_allowed"));
    }

    // Check length (prevent DoS)
    if path.len() > 4096 {
        return Err(ValidationError::new("path_too_long"));
    }

    // Check for path traversal
    if path.contains("..") {
        return Err(ValidationError::new("path_traversal_detected"));
    }

    // Check for absolute paths only
    if !std::path::Path::new(path).is_absolute() {
        return Err(ValidationError::new("must_be_absolute_path"));
    }

    // Whitelist of allowed characters
    let allowed_pattern = Regex::new(r"^[a-zA-Z0-9._\-/]+$").unwrap();
    if !allowed_pattern.is_match(path) {
        return Err(ValidationError::new("invalid_characters"));
    }

    Ok(())
}

/// Validated file ID
#[derive(Debug, Validate, Deserialize)]
pub struct FileId {
    #[validate(length(min = 1, max = 256))]
    #[validate(regex = "^[a-zA-Z0-9/_-]+$")]
    pub id: String,
}

/// Branch name validation
#[derive(Debug, Validate, Deserialize)]
pub struct BranchName {
    #[validate(length(min = 1, max = 255))]
    #[validate(custom = "validate_branch_name")]
    pub name: String,
}

fn validate_branch_name(name: &str) -> Result<(), ValidationError> {
    // Git reference naming restrictions
    let invalid_patterns = [
        "~", "^", ":", "?", "*", "[", "\\", "..", "@{", "~", "^"
    ];

    for pattern in &invalid_patterns {
        if name.contains(pattern) {
            return Err(ValidationError::new("invalid_branch_characters"));
        }
    }

    Ok(())
}

/// Command with validation wrapper
#[derive(Debug)]
pub struct ValidatedCommand<T> {
    pub data: T,
    pub validated_at: std::time::SystemTime,
}

impl<T: Validate> ValidatedCommand<T> {
    pub fn new(data: T) -> Result<Self, Vec<ValidationError>> {
        data.validate()?;
        Ok(Self {
            data,
            validated_at: std::time::SystemTime::now(),
        })
    }
}

/// Secure command handler with validation
#[command]
pub async fn get_file_diff_secure(
    app: AppHandle,
    raw_input: serde_json::Value,
) -> Result<Vec<DiffLine>, String> {
    // Parse with validation
    let command: ValidatedCommand<GetFileDiffCommand> =
        ValidatedCommand::new(raw_input)
            .map_err(|errors| format!("Validation failed: {:?}", errors))?;

    // Additional runtime checks
    check_rate_limit(&command.data.repo_path)?;
    check_file_size_limit(&command.data.file_id)?;

    // Execute with validated input
    execute_file_diff_secure(&app, &command.data).await
}

/// Additional runtime validations
fn check_rate_limit(path: &str) -> Result<(), String> {
    // Implement sliding window rate limiting
    // Track requests per client/operation
    Ok(())
}

fn check_file_size_limit(file_id: &str) -> Result<(), String> {
    // Prevent processing of extremely large files
    // This is a simplified check
    if file_id.len() > 10000 {
        return Err("File identifier too large".to_string());
    }
    Ok(())
}
```

### 4. Secure Credential Storage

#### A. Keychain/Keyring Integration (Recommended for macOS/Linux)

```rust
// src-tauri/src/storage/keychain.rs

use tauri_plugin_store::PluginBuilder;
use keyring::Entry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SecureCredential {
    pub service: String,
    pub username: String,
    // Never store password directly in plugin store
}

pub struct SecureStorage {
    keychain_entry: Entry,
    store: tauri_plugin_store::Store,
}

impl SecureStorage {
    pub fn new(app: &AppHandle) -> Result<Self> {
        let store = app.plugin::<tauri_plugin_store::Plugin>(tauri_plugin_store::PLUGIN_NAME)
            .expect("Store plugin not loaded")
            .store();

        let entry = Entry::new("hyperreview", "credentials")
            .map_err(|e| format!("Keychain error: {}", e))?;

        Ok(Self {
            keychain_entry: entry,
            store,
        })
    }

    /// Store credentials in OS keychain
    pub async fn store_credential(&self, credential: &SecureCredential, password: &str) -> Result<(), String> {
        // Store password in keychain (OS-level security)
        self.keychain_entry
            .set_password(password)
            .map_err(|e| format!("Failed to store password: {}", e))?;

        // Store non-sensitive metadata in plugin store
        let serialized = serde_json::to_string(credential)
            .map_err(|e| format!("Serialization error: {}", e))?;

        self.store.set("credential_metadata", serialized);
        self.store.save().await
            .map_err(|e| format!("Failed to save store: {}", e))?;

        Ok(())
    }

    /// Retrieve password from keychain
    pub async fn get_password(&self) -> Result<String, String> {
        self.keychain_entry.get_password()
            .map_err(|e| format!("Failed to retrieve password: {}", e))
    }

    /// Retrieve metadata from plugin store
    pub async fn get_credential_metadata(&self) -> Result<Option<SecureCredential>, String> {
        match self.store.get("credential_metadata") {
            Some(data) => {
                let credential: SecureCredential = serde_json::from_str(&data)
                    .map_err(|e| format!("Deserialization error: {}", e))?;
                Ok(Some(credential))
            }
            None => Ok(None),
        }
    }
}
```

#### B. Plugin Store for Non-Sensitive Data

```rust
// src-tauri/src/storage/config.rs

use tauri_plugin_store::Store;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use once_cell::sync::Lazy;

static CONFIG_STORE: Lazy<Mutex<Option<Store>>> = Lazy::new(|| Mutex::new(None));

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub recent_repos: Vec<Repo>,
    pub user_preferences: UserPreferences,
    pub ui_state: UiState,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPreferences {
    pub theme: String,
    pub auto_refresh: bool,
    pub notifications_enabled: bool,
}

pub fn init_store(store: Store) {
    let mut store_guard = CONFIG_STORE.lock().unwrap();
    *store_guard = Some(store);
}

pub fn get_config() -> Option<AppConfig> {
    let store_guard = CONFIG_STORE.lock().unwrap();
    let store = store_guard.as_ref()?;

    store.get("app_config")
        .and_then(|data| serde_json::from_str(&data).ok())
}

pub fn save_config(config: &AppConfig) -> Result<(), String> {
    let store_guard = CONFIG_STORE.lock().unwrap();
    let store = store_guard.as_ref()
        .ok_or("Store not initialized")?;

    let serialized = serde_json::to_string(config)
        .map_err(|e| format!("Serialization error: {}", e))?;

    store.set("app_config", serialized);
    Ok(())
}

/// Do NOT store in plugin store:
/// - Passwords
/// - API keys
/// - Private keys
/// - OAuth tokens
/// - Session tokens
/// - Any PII
```

### 5. Command Injection Prevention Patterns

#### Shell Command Prevention

```rust
// src-tauri/src/security/command_guard.rs

use std::process::Command;
use regex::Regex;

/// Safe command execution without shell
pub fn execute_git_command(
    repo_path: &str,
    args: &[&str],
) -> Result<String, String> {
    // Validate repository path
    if !is_safe_path(repo_path) {
        return Err("Invalid repository path".to_string());
    }

    // Validate all arguments
    for arg in args {
        if !is_safe_git_argument(arg) {
            return Err(format!("Invalid argument: {}", arg));
        }
    }

    // Execute without shell to prevent injection
    let output = Command::new("git")
        .args(args)
        .current_dir(repo_path)
        .output()
        .map_err(|e| format!("Failed to execute git: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Validate path is safe
fn is_safe_path(path: &str) -> bool {
    // Check for path traversal
    if path.contains("..") {
        return false;
    }

    // Check for null bytes
    if path.contains('\0') {
        return false;
    }

    // Check for absolute paths
    if !std::path::Path::new(path).is_absolute() {
        return false;
    }

    // Check for allowed characters
    let pattern = Regex::new(r"^[a-zA-Z0-9._\-/]+$").unwrap();
    pattern.is_match(path)
}

/// Validate git argument
fn is_safe_git_argument(arg: &str) -> bool {
    // Reject arguments containing shell metacharacters
    let dangerous_chars = ['|', '&', ';', '$', '`', '\\', '"', '\'', '<', '>', '(', ')', '{', '}', '[', ']', '*', '?'];

    for &c in &dangerous_chars {
        if arg.contains(c) {
            return false;
        }
    }

    // Length check
    if arg.len() > 1000 {
        return false;
    }

    true
}

/// SQL injection prevention for file queries
fn build_safe_file_query(table: &str, file_id: &str) -> Result<String, String> {
    // Whitelist table names
    match table {
        "files" | "diffs" | "analysis" => {},
        _ => return Err("Invalid table name".to_string()),
    }

    // Use parameterized queries (conceptual - actual implementation depends on database)
    Ok(format!("SELECT * FROM {} WHERE id = ?", table))
}
```

### 6. Sandboxing and Capability-Based Security

#### File System Sandboxing

```rust
// src-tauri/src/security/sandbox.rs

use std::path::{Path, PathBuf};
use tauri::AppHandle;

pub struct FileSystemSandbox {
    allowed_paths: Vec<PathBuf>,
    denied_paths: Vec<PathBuf>,
    max_file_size: u64,
}

impl FileSystemSandbox {
    pub fn new() -> Self {
        Self {
            allowed_paths: vec![
                PathBuf::from("$APPDATA/hyperreview/repos"),
                PathBuf::from("$HOME/.hyperreview/repos"),
            ],
            denied_paths: vec![
                PathBuf::from("**/node_modules"),
                PathBuf::from("**/.git/objects"),
                PathBuf::from("**/.git/config"),
                PathBuf::from("**/.git/hooks"),
                PathBuf::from("**/.git/refs/heads"),
            ],
            max_file_size: 10 * 1024 * 1024, // 10MB
        }
    }

    /// Check if access to a path is allowed
    pub fn is_path_allowed(&self, path: &Path) -> Result<(), String> {
        let path = path.canonicalize()
            .map_err(|_| "Invalid path".to_string())?;

        // Check denied patterns first
        for denied in &self.denied_paths {
            if self.matches_pattern(&path, denied) {
                return Err(format!("Access denied: {:?}", denied));
            }
        }

        // Check allowed patterns
        let mut allowed = false;
        for allowed_pattern in &self.allowed_paths {
            if self.matches_pattern(&path, allowed_pattern) {
                allowed = true;
                break;
            }
        }

        if !allowed {
            return Err("Path not in allowed scope".to_string());
        }

        Ok(())
    }

    /// Check if a path matches a pattern
    fn matches_pattern(&self, path: &Path, pattern: &PathBuf) -> bool {
        // Simple pattern matching - in production, use glob or similar
        let path_str = path.to_string_lossy();
        let pattern_str = pattern.to_string_lossy();

        if pattern_str.starts_with("**/") {
            // Match anywhere in path
            let suffix = &pattern_str[3..];
            path_str.contains(suffix)
        } else {
            // Match prefix
            path_str.starts_with(&pattern_str)
        }
    }

    /// Check file size limit
    pub fn check_file_size(&self, path: &Path) -> Result<u64, String> {
        let metadata = std::fs::metadata(path)
            .map_err(|_| "Failed to read file metadata".to_string())?;

        let size = metadata.len();
        if size > self.max_file_size {
            return Err(format!(
                "File size {} exceeds limit {}",
                size,
                self.max_file_size
            ));
        }

        Ok(size)
    }
}
```

#### Process Sandboxing

```rust
// src-tauri/src/security/process_guard.rs

use std::process::{Command, Stdio};
use crate::security::command_guard;

pub struct ProcessSandbox {
    allowed_commands: Vec<String>,
    max_memory_mb: u64,
    max_cpu_time_sec: u64,
}

impl ProcessSandbox {
    pub fn new() -> Self {
        Self {
            allowed_commands: vec![
                "git".to_string(),
                "rg".to_string(), // ripgrep
                "fd".to_string(),
            ],
            max_memory_mb: 500,
            max_cpu_time_sec: 30,
        }
    }

    /// Execute command with resource limits
    pub fn execute_safe(
        &self,
        command: &str,
        args: &[&str],
    ) -> Result<String, String> {
        // Check if command is allowed
        if !self.allowed_commands.contains(&command.to_string()) {
            return Err(format!("Command not allowed: {}", command));
        }

        // Use command guard for path safety
        if command == "git" {
            return command_guard::execute_git_command(args[0], &args[1..]);
        }

        // For other commands, similar validation...
        Err("Not implemented".to_string())
    }
}
```

### 7. Frontend-Backend Trust Boundaries

#### Frontend Validation Layer

```typescript
// frontend/src/utils/validation.ts

export interface ValidatedInput<T> {
  data: T;
  validationErrors: string[];
  timestamp: number;
}

export class FrontendValidator {
  private static readonly MAX_PATH_LENGTH = 4096;
  private static readonly MAX_FILE_SIZE = 10 * 1024 * 1024; // 10MB

  static validateRepoPath(path: string): ValidatedInput<string> {
    const errors: string[] = [];

    if (!path) {
      errors.push("Path is required");
    }

    if (path.length > this.MAX_PATH_LENGTH) {
      errors.push("Path too long");
    }

    if (path.includes('..')) {
      errors.push("Path traversal not allowed");
    }

    if (path.includes('\0')) {
      errors.push("Null bytes not allowed");
    }

    // Check if absolute path
    if (!path.startsWith('/') && !path.match(/^[a-zA-Z]:\\/)) {
      errors.push("Must be absolute path");
    }

    return {
      data: path,
      validationErrors: errors,
      timestamp: Date.now(),
    };
  }

  static validateFileId(fileId: string): ValidatedInput<string> {
    const errors: string[] = [];

    if (!fileId) {
      errors.push("File ID is required");
    }

    if (fileId.length > 256) {
      errors.push("File ID too long");
    }

    if (!/^[a-zA-Z0-9/_-]+$/.test(fileId)) {
      errors.push("Invalid characters in file ID");
    }

    return {
      data: fileId,
      validationErrors: errors,
      timestamp: Date.now(),
    };
  }

  static sanitizeDiffInput(input: any): any {
    // Remove potentially dangerous fields
    const sanitized = { ...input };
    delete sanitized.__proto__;
    delete sanitized.constructor;
    delete sanitized.prototype;

    // Validate and sanitize each field
    if (sanitized.fileId) {
      const validated = this.validateFileId(sanitized.fileId);
      if (validated.validationErrors.length > 0) {
        throw new Error(`Invalid file ID: ${validated.validationErrors.join(', ')}`);
      }
      sanitized.fileId = validated.data;
    }

    return sanitized;
  }
}

// frontend/src/api/secure-client.ts

export class SecureApiClient {
  private static readonly RATE_LIMIT = 100; // requests per minute
  private requestCount = 0;
  private requestWindow: number[] = [];

  private checkRateLimit(): boolean {
    const now = Date.now();
    this.requestWindow = this.requestWindow.filter(
      time => now - time < 60000
    );

    if (this.requestWindow.length >= this.RATE_LIMIT) {
      return false;
    }

    this.requestWindow.push(now);
    return true;
  }

  async invokeSecure<T>(
    command: string,
    args: any
  ): Promise<T> {
    // Client-side validation
    const validated = FrontendValidator.sanitizeDiffInput(args);

    // Check rate limit
    if (!this.checkRateLimit()) {
      throw new Error("Rate limit exceeded. Please wait before making more requests.");
    }

    try {
      const result = await window.__TAURI__.invoke<T>(command, validated);
      return result;
    } catch (error) {
      // Log error but don't expose sensitive details
      console.error(`Command ${command} failed:`, error);
      throw new Error(`Operation failed: ${error}`);
    }
  }
}
```

---

## Part B: Static Analysis Integration with tree-sitter

### 1. Integrating tree-sitter for Multi-Language Support

#### Language Parser Integration

```rust
// src-tauri/src/analysis/mod.rs

use tree_sitter::{Parser, Language, Node};
use tree_sitter_java as java;
use tree_sitter_sql as sql;
use tree_sitter_xml as xml;
use tree_sitter_rust as rust;
use tree_sitter_typescript as typescript;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub language: String,
    pub file_path: String,
    pub complexity: ComplexityMetrics,
    pub issues: Vec<Issue>,
    pub suggestions: Vec<Suggestion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityMetrics {
    pub cyclomatic: u32,
    pub cognitive: u32,
    pub halstead: HalsteadMetrics,
    pub lines_of_code: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub severity: Severity,
    pub message: String,
    pub line: u32,
    pub column: u32,
    pub rule_id: String,
    pub snippet: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggestion {
    pub message: String,
    pub line: u32,
    pub replacement: Option<String>,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HalsteadMetrics {
    pub vocabulary: u32,
    pub length: u32,
    pub calculated_length: u32,
    pub volume: f64,
    pub difficulty: f64,
    pub effort: f64,
}

/// Language registry with dynamic loading
pub struct LanguageRegistry {
    parsers: HashMap<String, Parser>,
    grammars: HashMap<String, Language>,
}

impl LanguageRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            parsers: HashMap::new(),
            grammars: HashMap::new(),
        };

        // Register grammars
        registry.register_language("rust", tree_sitter_rust());
        registry.register_language("java", tree_sitter_java());
        registry.register_language("sql", tree_sitter_sql());
        registry.register_language("xml", tree_sitter_xml());
        registry.register_language("typescript", tree_sitter_typescript());
        registry.register_language("javascript", tree_sitter_javascript());

        registry
    }

    fn register_language(&mut self, name: &str, language: Language) {
        let mut parser = Parser::new();
        parser.set_language(language)
            .expect(&format!("Failed to set language: {}", name));

        self.parsers.insert(name.to_string(), parser);
        self.grammars.insert(name.to_string(), language);
    }

    pub fn get_parser(&self, language: &str) -> Option<&Parser> {
        self.parsers.get(language)
    }

    pub fn detect_language(&self, file_path: &str) -> Option<String> {
        let extension = std::path::Path::new(file_path)
            .extension()
            .and_then(|ext| ext.to_str())?
            .to_lowercase();

        let language_map = HashMap::from([
            ("rs", "rust"),
            ("java", "java"),
            ("sql", "sql"),
            ("xml", "xml"),
            ("ts", "typescript"),
            ("js", "javascript"),
            ("tsx", "typescript"),
            ("jsx", "javascript"),
        ]);

        language_map.get(&extension as &str).map(|s| s.to_string())
    }
}
```

#### Multi-Language Analysis Engine

```rust
// src-tauri/src/analysis/engine.rs

use super::*;
use tree_sitter::Tree;

pub struct AnalysisEngine {
    language_registry: Arc<LanguageRegistry>,
    rule_engine: RuleEngine,
}

impl AnalysisEngine {
    pub fn new(language_registry: Arc<LanguageRegistry>) -> Self {
        Self {
            language_registry,
            rule_engine: RuleEngine::new(),
        }
    }

    /// Analyze source code with multi-language support
    pub fn analyze_source(
        &self,
        file_path: &str,
        source_code: &str,
    ) -> Result<AnalysisResult, String> {
        // Detect language from file extension
        let language = self.language_registry.detect_language(file_path)
            .ok_or_else(|| format!("Unsupported file type: {}", file_path))?;

        // Get parser for detected language
        let parser = self.language_registry.get_parser(&language)
            .ok_or_else(|| format!("No parser available for: {}", language))?;

        // Parse source code
        let tree = parser.parse(source_code, None)
            .ok_or_else(|| "Failed to parse source code".to_string())?;

        // Create analysis context
        let context = AnalysisContext {
            language: language.clone(),
            file_path: file_path.to_string(),
            source_code: source_code.to_string(),
            tree: tree.clone(),
        };

        // Run complexity analysis
        let complexity = self.calculate_complexity(&context);

        // Run security analysis
        let security_issues = self.rule_engine.run_security_rules(&context);

        // Run quality rules
        let quality_issues = self.rule_engine.run_quality_rules(&context);

        // Generate suggestions
        let suggestions = self.generate_suggestions(&context);

        // Combine all issues
        let mut issues = security_issues;
        issues.extend(quality_issues);

        Ok(AnalysisResult {
            language,
            file_path: file_path.to_string(),
            complexity,
            issues,
            suggestions,
        })
    }

    /// Calculate cyclomatic complexity
    fn calculate_complexity(&self, context: &AnalysisContext) -> ComplexityMetrics {
        let root_node = context.tree.root_node();
        let mut cyclomatic = 1; // Base complexity

        // Count decision points
        cyclomatic += self.count_decision_points(root_node);

        // Count cognitive complexity (simplified)
        let cognitive = self.calculate_cognitive_complexity(root_node);

        // Calculate Halstead metrics
        let halstead = self.calculate_halstead_metrics(&context.source_code);

        // Count lines of code
        let loc = context.source_code.lines().count() as u32;

        ComplexityMetrics {
            cyclomatic,
            cognitive,
            halstead,
            lines_of_code: loc,
        }
    }

    fn count_decision_points(&self, node: Node) -> u32 {
        let mut count = 0;

        // Check if current node is a decision point
        if node.kind() == "if_statement" ||
           node.kind() == "for_statement" ||
           node.kind() == "while_statement" ||
           node.kind() == "case_statement" ||
           node.kind() == "catch" ||
           node.kind() == "binary_expression" {

            count += 1;
        }

        // Recursively check children
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                count += self.count_decision_points(cursor.node());
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }

        count
    }

    fn calculate_cognitive_complexity(&self, node: Node) -> u32 {
        // Simplified cognitive complexity calculation
        // Adds penalty for nesting and complexity
        let mut complexity = 0;
        let mut depth = 0;

        let mut cursor = node.walk();
        self.walk_cognitive(&mut cursor, &mut complexity, &mut depth);

        complexity
    }

    fn walk_cognitive(
        &self,
        cursor: &mut tree_sitter::TreeCursor,
        complexity: &mut u32,
        depth: &mut u32,
    ) {
        let node_kind = cursor.node().kind();

        // Increase complexity for nesting
        if node_kind == "if_statement" ||
           node_kind == "for_statement" ||
           node_kind == "while_statement" ||
           node_kind == "function_definition" ||
           node_kind == "class_definition" {

            if *depth > 0 {
                *complexity += *depth;
            }
            *complexity += 1;
            *depth += 1;
        }

        // Process children
        if cursor.goto_first_child() {
            loop {
                self.walk_cognitive(cursor, complexity, depth);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();
            if *depth > 0 {
                *depth -= 1;
            }
        }
    }

    fn calculate_halstead_metrics(&self, source: &str) -> HalsteadMetrics {
        use std::collections::HashSet;

        let mut operators = HashSet::new();
        let mut operands = HashSet::new();
        let mut operator_count = 0;
        let mut operand_count = 0;

        // Simplified Halstead calculation
        // In production, use a proper lexer

        for (is_operator, _) in source.chars().scan(true, |state, c| {
            *state = !*state;
            Some((*state, c))
        }) {
            if is_operator {
                operators.insert(c);
                operator_count += 1;
            } else {
                operands.insert(c);
                operand_count += 1;
            }
        }

        let vocabulary = (operators.len() + operands.len()) as u32;
        let length = (operator_count + operand_count) as u32;

        // Simplified calculations
        let volume = (length as f64) * (vocabulary as f64).log2();
        let difficulty = (operators.len() as f64 / 2.0) * (operands.len() as f64 / operands.len().max(1) as f64);
        let effort = volume * difficulty;

        HalsteadMetrics {
            vocabulary,
            length,
            calculated_length: (vocabulary as f64 * (vocabulary as f64).log2()).round() as u32,
            volume,
            difficulty,
            effort,
        }
    }

    fn generate_suggestions(&self, context: &AnalysisContext) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();

        // Generate suggestions based on analysis
        if context.language == "rust" {
            suggestions.extend(self.generate_rust_suggestions(context));
        }

        suggestions
    }

    fn generate_rust_suggestions(&self, context: &AnalysisContext) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();

        // Example: Suggest using Result instead of unwrap
        if context.source_code.contains(".unwrap()") {
            suggestions.push(Suggestion {
                message: "Consider using pattern matching instead of unwrap() for better error handling".to_string(),
                line: 0,
                replacement: None,
                confidence: 0.8,
            });
        }

        suggestions
    }
}
```

### 2. Cyclomatic Complexity Calculation Patterns

#### Advanced Complexity Analysis

```rust
// src-tauri/src/analysis/complexity.rs

use tree_sitter::{Node, TreeCursor};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ComplexityAnalyzer {
    language_patterns: HashMap<&'static str, Vec<ComplexityPattern>>,
}

#[derive(Debug)]
struct ComplexityPattern {
    node_kind: &'static str,
    weight: u32,
    child_weights: HashMap<&'static str, u32>,
}

impl ComplexityAnalyzer {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();

        // Rust complexity patterns
        patterns.insert("rust", vec![
            ComplexityPattern {
                node_kind: "if_expression",
                weight: 1,
                child_weights: [
                    ("block", 1),
                    ("if_expression", 1),
                ].into_iter().collect(),
            },
            ComplexityPattern {
                node_kind: "match_expression",
                weight: 2, // Match expressions are more complex
                child_weights: [
                    ("match_arm", 1),
                ].into_iter().collect(),
            },
            ComplexityPattern {
                node_kind: "for_expression",
                weight: 1,
                child_weights: HashMap::new(),
            },
            ComplexityPattern {
                node_kind: "while_expression",
                weight: 1,
                child_weights: HashMap::new(),
            },
            ComplexityPattern {
                node_kind: "binary_expression",
                weight: 1,
                child_weights: HashMap::new(),
            },
        ]);

        // Java complexity patterns
        patterns.insert("java", vec![
            ComplexityPattern {
                node_kind: "if_statement",
                weight: 1,
                child_weights: HashMap::new(),
            },
            ComplexityPattern {
                node_kind: "for_statement",
                weight: 1,
                child_weights: HashMap::new(),
            },
            ComplexityPattern {
                node_kind: "while_statement",
                weight: 1,
                child_weights: HashMap::new(),
            },
            ComplexityPattern {
                node_kind: "do_statement",
                weight: 1,
                child_weights: HashMap::new(),
            },
            ComplexityPattern {
                node_kind: "switch_statement",
                weight: 2, // Each case adds complexity
                child_weights: [
                    ("switch_label", 1),
                ].into_iter().collect(),
            },
            ComplexityPattern {
                node_kind: "catch_clause",
                weight: 1,
                child_weights: HashMap::new(),
            },
        ]);

        // SQL complexity patterns
        patterns.insert("sql", vec![
            ComplexityPattern {
                node_kind: "where_clause",
                weight: 1,
                child_weights: [
                    ("and_expression", 1),
                    ("or_expression", 1),
                ].into_iter().collect(),
            },
            ComplexityPattern {
                node_kind: "case_expression",
                weight: 2,
                child_weights: HashMap::new(),
            },
        ]);

        Self {
            language_patterns: patterns,
        }
    }

    /// Calculate weighted cyclomatic complexity
    pub fn calculate_weighted_complexity(
        &self,
        language: &str,
        tree: &tree_sitter::Tree,
    ) -> (u32, HashMap<String, u32>) {
        let patterns = self.language_patterns
            .get(language)
            .unwrap_or(&Vec::new());

        let root = tree.root_node();
        let mut total_complexity = 0;
        let mut complexity_breakdown = HashMap::new();

        self.analyze_node_weighted(root, patterns, &mut total_complexity, &mut complexity_breakdown);

        (total_complexity, complexity_breakdown)
    }

    fn analyze_node_weighted(
        &self,
        node: Node,
        patterns: &[ComplexityPattern],
        total: &mut u32,
        breakdown: &mut HashMap<String, u32>,
    ) {
        // Find matching pattern
        for pattern in patterns {
            if node.kind() == pattern.node_kind {
                // Add base weight
                *total += pattern.weight;

                // Update breakdown
                *breakdown.entry(pattern.node_kind.to_string())
                    .or_insert(0) += pattern.weight;

                // Check children for weighted complexity
                let mut cursor = node.walk();
                if cursor.goto_first_child() {
                    loop {
                        self.analyze_node_weighted(
                            cursor.node(),
                            patterns,
                            total,
                            breakdown,
                        );

                        if !cursor.goto_next_sibling() {
                            break;
                        }
                    }
                }

                return;
            }
        }

        // If no match, recurse on children
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                self.analyze_node_weighted(cursor.node(), patterns, total, breakdown);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
    }
}
```

### 3. Dynamic Grammar Loading to Reduce Bundle Size

#### Lazy Loading System

```rust
// src-tauri/src/analysis/grammar_loader.rs

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use tree_sitter::Language;

pub struct GrammarLoader {
    loaded_grammars: Arc<Mutex<HashMap<String, Language>>>,
    grammar_registry: GrammarRegistry,
}

struct GrammarRegistry {
    grammars: HashMap<&'static str, fn() -> Language>,
}

impl GrammarRegistry {
    const fn new() -> Self {
        Self {
            grammars: HashMap::new(),
        }
    }

    fn register(&mut self, name: &'static str, loader: fn() -> Language) {
        self.grammars.insert(name, loader);
    }

    fn get_loader(&self, name: &str) -> Option<&fn() -> Language> {
        self.grammars.get(name)
    }
}

static REGISTRY: Lazy<Mutex<GrammarRegistry>> = Lazy::new(|| {
    let mut registry = GrammarRegistry::new();

    // Register all available grammars
    registry.register("rust", tree_sitter_rust);
    registry.register("java", tree_sitter_java);
    registry.register("sql", tree_sitter_sql);
    registry.register("xml", tree_sitter_xml);
    registry.register("typescript", tree_sitter_typescript);
    registry.register("javascript", tree_sitter_javascript);

    Mutex::new(registry)
});

impl GrammarLoader {
    pub fn new() -> Self {
        Self {
            loaded_grammars: Arc::new(Mutex::new(HashMap::new())),
            grammar_registry: GrammarRegistry::new(),
        }
    }

    /// Load grammar on-demand (lazy loading)
    pub fn load_grammar(&self, language: &str) -> Result<tree_sitter::Language, String> {
        // Check if already loaded
        {
            let loaded = self.loaded_grammars.lock().unwrap();
            if let Some(lang) = loaded.get(language) {
                return Ok(*lang);
            }
        }

        // Get loader from registry
        let loader = REGISTRY.lock().unwrap()
            .get_loader(language)
            .ok_or_else(|| format!("Grammar not registered: {}", language))?;

        // Load grammar
        let language = loader();

        // Cache it
        {
            let mut loaded = self.loaded_grammars.lock().unwrap();
            loaded.insert(language.to_string(), language);
        }

        Ok(loader())
    }

    /// Preload grammars for specific languages (for performance)
    pub fn preload_grammars(&self, languages: &[&str]) -> Result<(), String> {
        for lang in languages {
            self.load_grammar(lang)?;
        }
        Ok(())
    }

    /// Unload unused grammars to save memory
    pub fn unload_grammar(&self, language: &str) -> Result<(), String> {
        let mut loaded = self.loaded_grammars.lock().unwrap();
        loaded.remove(language);
        Ok(())
    }

    /// Get list of loaded grammars
    pub fn get_loaded_grammars(&self) -> Vec<String> {
        let loaded = self.loaded_grammars.lock().unwrap();
        loaded.keys().cloned().collect()
    }
}

/// Performance monitoring
pub struct GrammarPerformanceMetrics {
    load_times: HashMap<String, std::time::Duration>,
    usage_count: HashMap<String, u64>,
    memory_usage: HashMap<String, usize>,
}

impl GrammarPerformanceMetrics {
    pub fn new() -> Self {
        Self {
            load_times: HashMap::new(),
            usage_count: HashMap::new(),
            memory_usage: HashMap::new(),
        }
    }

    pub fn record_load_time(&mut self, language: &str, duration: std::time::Duration) {
        self.load_times.insert(language.to_string(), duration);
    }

    pub fn record_usage(&mut self, language: &str) {
        *self.usage_count.entry(language.to_string()).or_insert(0) += 1;
    }

    pub fn get_optimal_preload_set(&self, max_grammars: usize) -> Vec<String> {
        // Sort by usage count and select top N
        let mut languages: Vec<_> = self.usage_count.iter()
            .map(|(lang, count)| (lang.clone(), *count))
            .collect();

        languages.sort_by(|a, b| b.1.cmp(&a.1));

        languages.into_iter()
            .take(max_grammars)
            .map(|(lang, _)| lang)
            .collect()
    }
}
```

#### Conditional Compilation for Bundle Size

```toml
# Cargo.toml features

[features]
# Enable specific language support to reduce binary size
rust-support = ["tree-sitter-rust"]
java-support = ["tree-sitter-java"]
sql-support = ["tree-sitter-sql"]
xml-support = ["tree-sitter-xml"]
ts-support = ["tree-sitter-typescript"]
js-support = ["tree-sitter-javascript"]

# Default features
default = ["rust-support", "java-support"]

[dependencies]
tree-sitter = "0.22"

# Optional dependencies
tree-sitter-rust = { version = "0.20", optional = true }
tree-sitter-java = { version = "0.20", optional = true }
tree-sitter-sql = { version = "0.20", optional = true }
tree-sitter-xml = { version = "0.20", optional = true }
tree-sitter-typescript = { version = "0.20", optional = true }
tree-sitter-javascript = { version = "0.20", optional = true }
```

### 4. Rule Engine Architecture for Smart Checklists

#### Rule Engine Implementation

```rust
// src-tauri/src/analysis/rule_engine.rs

use tree_sitter::{Node, Tree};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub severity: Severity,
    pub category: RuleCategory,
    pub enabled: bool,
    pub language_patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleCategory {
    Security,
    Performance,
    CodeQuality,
    BestPractice,
    Maintainability,
    Documentation,
}

pub type RuleCheck = fn(&AnalysisContext) -> Vec<Issue>;

#[derive(Debug)]
pub struct RuleContext {
    pub rule: Rule,
    pub check: RuleCheck,
}

#[derive(Debug)]
pub struct AnalysisContext<'a> {
    pub language: String,
    pub file_path: String,
    pub source_code: &'a str,
    pub tree: &'a Tree,
}

pub struct RuleEngine {
    rules: HashMap<String, RuleContext>,
    enabled_rules: HashMap<String, bool>,
}

impl RuleEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            rules: HashMap::new(),
            enabled_rules: HashMap::new(),
        };

        // Register built-in rules
        engine.register_security_rules();
        engine.register_quality_rules();
        engine.register_performance_rules();

        engine
    }

    fn register_rule<F>(&mut self, rule: Rule, check: F)
    where
        F: Fn(&AnalysisContext) -> Vec<Issue> + 'static,
    {
        self.rules.insert(
            rule.id.clone(),
            RuleContext {
                rule: rule.clone(),
                check: Box::new(check),
            }
        );

        self.enabled_rules.insert(rule.id, rule.enabled);
    }

    fn register_security_rules(&mut self) {
        // Hardcoded secrets detection
        self.register_rule(
            Rule {
                id: "SEC001".to_string(),
                name: "Hardcoded Secrets".to_string(),
                description: "Detects potential hardcoded passwords, API keys, or tokens",
                severity: Severity::Error,
                category: RuleCategory::Security,
                enabled: true,
                language_patterns: vec!["rust".to_string(), "java".to_string(), "javascript".to_string()],
            },
            |ctx| self.check_hardcoded_secrets(ctx),
        );

        // SQL injection prevention
        self.register_rule(
            Rule {
                id: "SEC002".to_string(),
                name: "SQL Injection Risk".to_string(),
                description: "Detects potential SQL injection vulnerabilities",
                severity: Severity::Error,
                category: RuleCategory::Security,
                enabled: true,
                language_patterns: vec!["java".to_string(), "rust".to_string()],
            },
            |ctx| self.check_sql_injection(ctx),
        );

        // Command injection prevention
        self.register_rule(
            Rule {
                id: "SEC003".to_string(),
                name: "Command Injection Risk".to_string(),
                description: "Detects potential command injection vulnerabilities",
                severity: Severity::Error,
                category: RuleCategory::Security,
                enabled: true,
                language_patterns: vec!["rust".to_string(), "java".to_string()],
            },
            |ctx| self.check_command_injection(ctx),
        );
    }

    fn register_quality_rules(&mut self) {
        // TODO comments
        self.register_rule(
            Rule {
                id: "QUAL001".to_string(),
                name: "TODO/FIXME Comments".to_string(),
                description: "Finds TODO and FIXME comments",
                severity: Severity::Info,
                category: RuleCategory::CodeQuality,
                enabled: true,
                language_patterns: vec!["*".to_string()],
            },
            |ctx| self.check_todo_comments(ctx),
        );

        // Long function detection
        self.register_rule(
            Rule {
                id: "QUAL002".to_string(),
                name: "Long Function".to_string(),
                description: "Functions should not exceed 50 lines",
                severity: Severity::Warning,
                category: RuleCategory::Maintainability,
                enabled: true,
                language_patterns: vec!["rust".to_string(), "java".to_string(), "javascript".to_string()],
            },
            |ctx| self.check_long_function(ctx),
        );
    }

    fn register_performance_rules(&mut self) {
        // N+1 query detection
        self.register_rule(
            Rule {
                id: "PERF001".to_string(),
                name: "Potential N+1 Query".to_string(),
                description: "Detects patterns that may cause N+1 query problems",
                severity: Severity::Warning,
                category: RuleCategory::Performance,
                enabled: true,
                language_patterns: vec!["java".to_string(), "rust".to_string()],
            },
            |ctx| self.check_n_plus_one(ctx),
        );
    }

    /// Run all enabled rules for a language
    pub fn run_rules(&self, context: &AnalysisContext) -> Vec<Issue> {
        let mut issues = Vec::new();

        for (rule_id, rule_context) in &self.rules {
            // Check if rule is enabled
            if !*self.enabled_rules.get(rule_id).unwrap_or(&false) {
                continue;
            }

            // Check if rule applies to this language
            if !rule_context.rule.language_patterns.contains(&"*".to_string()) &&
               !rule_context.rule.language_patterns.contains(&context.language) {
                continue;
            }

            // Run rule check
            let rule_issues = (rule_context.check)(context);
            issues.extend(rule_issues);
        }

        issues
    }

    /// Run security rules only
    pub fn run_security_rules(&self, context: &AnalysisContext) -> Vec<Issue> {
        let mut issues = Vec::new();

        for (_, rule_context) in &self.rules {
            if rule_context.rule.category == RuleCategory::Security {
                let rule_issues = (rule_context.check)(context);
                issues.extend(rule_issues);
            }
        }

        issues
    }

    /// Run quality rules only
    pub fn run_quality_rules(&self, context: &AnalysisContext) -> Vec<Issue> {
        let mut issues = Vec::new();

        for (_, rule_context) in &self.rules {
            if rule_context.rule.category == RuleCategory::CodeQuality ||
               rule_context.rule.category == RuleCategory::Maintainability {
                let rule_issues = (rule_context.check)(context);
                issues.extend(rule_issues);
            }
        }

        issues
    }

    // Rule implementations
    fn check_hardcoded_secrets(&self, ctx: &AnalysisContext) -> Vec<Issue> {
        let mut issues = Vec::new();
        let source = ctx.source_code;
        let lines: Vec<&str> = source.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            let line_lower = line.to_lowercase();

            // Check for common secret patterns
            let patterns = [
                (r"(?i)(password\s*=\s*['\"][^'\"]+['\"])", "Hardcoded password"),
                (r"(?i)(api[_-]?key\s*=\s*['\"][^'\"]+['\"])", "Hardcoded API key"),
                (r"(?i)(secret\s*=\s*['\"][^'\"]+['\"])", "Hardcoded secret"),
                (r"(?i)(token\s*=\s*['\"][^'\"]+['\"])", "Hardcoded token"),
                (r"(?i)(private[_-]?key\s*=\s*['\"][^'\"]+['\"])", "Hardcoded private key"),
            ];

            for (pattern, message) in &patterns {
                if let Ok(regex) = regex::Regex::new(pattern) {
                    if regex.is_match(line) {
                        issues.push(Issue {
                            severity: Severity::Error,
                            message: message.to_string(),
                            line: (i + 1) as u32,
                            column: 0,
                            rule_id: "SEC001".to_string(),
                            snippet: Some(line.to_string()),
                        });
                    }
                }
            }
        }

        issues
    }

    fn check_sql_injection(&self, ctx: &AnalysisContext) -> Vec<Issue> {
        let mut issues = Vec::new();
        let source = ctx.source_code;
        let lines: Vec<&str> = source.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            // Check for string concatenation in SQL queries
            if line.contains("SELECT") && (line.contains("+") || line.contains("format!")) {
                issues.push(Issue {
                    severity: Severity::Error,
                    message: "Potential SQL injection: string concatenation in query".to_string(),
                    line: (i + 1) as u32,
                    column: 0,
                    rule_id: "SEC002".to_string(),
                    snippet: Some(line.to_string()),
                });
            }
        }

        issues
    }

    fn check_command_injection(&self, ctx: &AnalysisContext) -> Vec<Issue> {
        let mut issues = Vec::new();
        let source = ctx.source_code;

        // Check for shell command execution without proper sanitization
        let patterns = [
            (r"system\s*\(", "shell command execution"),
            (r"exec\s*\(", "process execution"),
            (r"ProcessBuilder", "process building"),
        ];

        for (pattern, message) in &patterns {
            if let Ok(regex) = regex::Regex::new(pattern) {
                for (i, line) in source.lines().enumerate() {
                    if regex.is_match(line) {
                        // Check if input is sanitized
                        if !line.contains("sanitize") && !line.contains("escape") {
                            issues.push(Issue {
                                severity: Severity::Error,
                                message: format!("Potential command injection: {}", message),
                                line: (i + 1) as u32,
                                column: 0,
                                rule_id: "SEC003".to_string(),
                                snippet: Some(line.to_string()),
                            });
                        }
                    }
                }
            }
        }

        issues
    }

    fn check_todo_comments(&self, ctx: &AnalysisContext) -> Vec<Issue> {
        let mut issues = Vec::new();
        let source = ctx.source_code;
        let lines: Vec<&str> = source.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            if line.contains("TODO") || line.contains("FIXME") || line.contains("XXX") {
                issues.push(Issue {
                    severity: Severity::Info,
                    message: "TODO/FIXME comment found".to_string(),
                    line: (i + 1) as u32,
                    column: 0,
                    rule_id: "QUAL001".to_string(),
                    snippet: Some(line.to_string()),
                });
            }
        }

        issues
    }

    fn check_long_function(&self, ctx: &AnalysisContext) -> Vec<Issue> {
        let mut issues = Vec::new();
        let tree = ctx.tree;

        // Find function definitions
        let mut cursor = tree.walk();
        if cursor.goto_first_child() {
            loop {
                let node = cursor.node();

                if self.is_function_node(ctx.language.as_str(), node.kind()) {
                    let line_count = self.get_node_line_count(node);
                    if line_count > 50 {
                        issues.push(Issue {
                            severity: Severity::Warning,
                            message: format!("Function too long: {} lines (max: 50)", line_count),
                            line: node.start_position().row + 1,
                            column: node.start_position().column,
                            rule_id: "QUAL002".to_string(),
                            snippet: None,
                        });
                    }
                }

                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }

        issues
    }

    fn check_n_plus_one(&self, ctx: &AnalysisContext) -> Vec<Issue> {
        // Simplified N+1 detection
        let mut issues = Vec::new();
        let source = ctx.source_code;

        // Look for loop patterns with database queries
        let has_loop = source.contains("for ") || source.contains("while ");
        let has_query = source.contains("query") || source.contains("fetch") || source.contains("find");

        if has_loop && has_query {
            issues.push(Issue {
                severity: Severity::Warning,
                message: "Potential N+1 query: loop contains database operation".to_string(),
                line: 1,
                column: 0,
                rule_id: "PERF001".to_string(),
                snippet: None,
            });
        }

        issues
    }

    fn is_function_node(&self, language: &str, kind: &str) -> bool {
        match language {
            "rust" => kind == "function_item",
            "java" => kind == "method_declaration",
            "javascript" | "typescript" => kind == "function_declaration" || kind == "arrow_function",
            _ => kind.contains("function"),
        }
    }

    fn get_node_line_count(&self, node: Node) -> u32 {
        node.end_position().row - node.start_position().row + 1
    }
}
```

### 5. Performance Considerations for Real-Time Analysis

#### Incremental Analysis Engine

```rust
// src-tauri/src/analysis/incremental.rs

use tree_sitter::Parser;
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct IncrementalAnalyzer {
    parsers: HashMap<String, Parser>,
    cache: AnalysisCache,
    performance_tracker: PerformanceTracker,
}

struct AnalysisCache {
    /// File path -> (Tree, hash, timestamp)
    tree_cache: HashMap<String, CachedTree>,
    /// Maximum cache size
    max_cache_size: usize,
    /// Time to live for cache entries
    ttl: Duration,
}

#[derive(Debug)]
struct CachedTree {
    tree: tree_sitter::Tree,
    hash: u64,
    timestamp: Instant,
}

impl AnalysisCache {
    pub fn new(max_cache_size: usize, ttl: Duration) -> Self {
        Self {
            tree_cache: HashMap::new(),
            max_cache_size,
            ttl,
        }
    }

    /// Get cached tree if valid
    pub fn get(&self, file_path: &str) -> Option<&tree_sitter::Tree> {
        let cached = self.tree_cache.get(file_path)?;
        let age = cached.timestamp.elapsed();

        if age > self.ttl {
            None
        } else {
            Some(&cached.tree)
        }
    }

    /// Cache a tree
    pub fn put(&mut self, file_path: String, tree: tree_sitter::Tree, hash: u64) {
        // Evict old entries if cache is full
        if self.tree_cache.len() >= self.max_cache_size {
            self.evict_oldest();
        }

        self.tree_cache.insert(
            file_path,
            CachedTree {
                tree,
                hash,
                timestamp: Instant::now(),
            },
        );
    }

    fn evict_oldest(&mut self) {
        // Find and remove oldest entry
        let mut oldest_key = None;
        let mut oldest_time = Instant::now();

        for (key, cached) in &self.tree_cache {
            if cached.timestamp < oldest_time {
                oldest_time = cached.timestamp;
                oldest_key = Some(key.clone());
            }
        }

        if let Some(key) = oldest_key {
            self.tree_cache.remove(&key);
        }
    }

    /// Calculate file hash for change detection
    pub fn calculate_hash(&self, content: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        hasher.finish()
    }
}

struct PerformanceTracker {
    /// Operation -> (count, total_time)
    metrics: HashMap<String, (usize, Duration)>,
}

impl PerformanceTracker {
    pub fn new() -> Self {
        Self {
            metrics: HashMap::new(),
        }
    }

    pub fn record(&mut self, operation: &str, duration: Duration) {
        let entry = self.metrics.entry(operation.to_string()).or_insert((0, Duration::from_secs(0)));
        entry.0 += 1;
        entry.1 += duration;
    }

    pub fn get_stats(&self, operation: &str) -> Option<(usize, Duration, Duration)> {
        self.metrics.get(operation).map(|(count, total)| {
            let avg = Duration::from_nanos(total.as_nanos() as u64 / *count as u64);
            (*count, *total, avg)
        })
    }

    pub fn print_report(&self) {
        println!("Performance Report:");
        for (op, (count, total, _)) in self.metrics.iter().map(|(k, v)| (k, (v.0, v.1, Duration::from_nanos(v.1.as_nanos() as u64 / v.0 as u64)))) {
            println!("  {}: {} calls, total: {:?}, avg: {:?}", op, count, total, _);
        }
    }
}

impl IncrementalAnalyzer {
    pub fn new() -> Self {
        Self {
            parsers: HashMap::new(),
            cache: AnalysisCache::new(1000, Duration::from_secs(300)), // 5 minute TTL
            performance_tracker: PerformanceTracker::new(),
        }
    }

    /// Analyze file with incremental updates
    pub fn analyze_file(
        &mut self,
        file_path: &str,
        language: &str,
        content: &str,
    ) -> Result<AnalysisResult, String> {
        let start_time = Instant::now();

        // Calculate file hash
        let new_hash = self.cache.calculate_hash(content);

        // Check if we need to re-parse
        let should_reparse = if let Some(cached_tree) = self.cache.get(file_path) {
            // Check if content has changed
            // In production, compare actual content or use diff
            true // Simplified - always reparse for now
        } else {
            true
        };

        let parse_time = if should_reparse {
            // Get or create parser
            let parser = self.parsers.entry(language.to_string())
                .or_insert_with(|| {
                    let mut p = Parser::new();
                    // Set language based on language string
                    // This is simplified - actual implementation would map language to grammar
                    p
                });

            // Parse content
            let tree = parser.parse(content, None)
                .ok_or_else(|| "Failed to parse content".to_string())?;

            // Cache the tree
            self.cache.put(file_path.to_string(), tree.clone(), new_hash);

            let parse_duration = start_time.elapsed();
            self.performance_tracker.record("parse", parse_duration);
            parse_duration
        } else {
            Duration::from_nanos(0)
        };

        // Get tree from cache
        let tree = self.cache.get(file_path)
            .ok_or_else(|| "Tree not in cache".to_string())?;

        // Run analysis
        let analysis_start = Instant::now();
        let result = self.run_analysis(tree, content, language)?;
        let analysis_duration = analysis_start.elapsed();

        self.performance_tracker.record("analysis", analysis_duration);

        let total_duration = start_time.elapsed();
        self.performance_tracker.record("total", total_duration);

        Ok(result)
    }

    /// Batch analyze multiple files
    pub fn analyze_batch(
        &mut self,
        files: &[(&str, &str, &str)], // (file_path, language, content)
    ) -> Vec<Result<AnalysisResult, String>> {
        files.iter().map(|(path, lang, content)| {
            self.analyze_file(path, lang, content)
        }).collect()
    }

    /// Get performance statistics
    pub fn get_performance_stats(&self) -> HashMap<String, (usize, Duration, Duration)> {
        let mut stats = HashMap::new();
        for (op, (count, total)) in &self.performance_tracker.metrics {
            let avg = Duration::from_nanos(total.as_nanos() as u64 / *count as u64);
            stats.insert(op.clone(), (*count, *total, avg));
        }
        stats
    }
}
```

### 6. Incremental Parsing for Diff Views

#### Diff-Aware Parsing

```rust
// src-tauri/src/analysis/diff_parser.rs

use tree_sitter::{Tree, Parser, Node};
use std::collections::HashMap;

pub struct DiffParser {
    parsers: HashMap<String, Parser>,
}

impl DiffParser {
    pub fn new() -> Self {
        Self {
            parsers: HashMap::new(),
        }
    }

    /// Parse only changed regions
    pub fn parse_incremental(
        &mut self,
        language: &str,
        old_tree: Option<&Tree>,
        old_content: &str,
        new_content: &str,
    ) -> Result<IncrementalParseResult, String> {
        // Get parser
        let parser = self.parsers.entry(language.to_string())
            .or_insert_with(|| Parser::new());

        // Parse new content
        let new_tree = parser.parse(new_content, old_tree)
            .ok_or_else(|| "Failed to parse new content".to_string())?;

        // Identify changed regions
        let changes = self.detect_changes(old_content, new_content);

        // Analyze only changed nodes
        let changed_nodes = self.find_changed_nodes(old_tree, &new_tree, &changes);

        Ok(IncrementalParseResult {
            tree: new_tree,
            changes,
            changed_nodes,
        })
    }

    /// Detect what changed between old and new content
    fn detect_changes(&self, old_content: &str, new_content: &str) -> Vec<TextChange> {
        use diff::Diff;

        let changes = diff::chars(old_content, new_content);

        let mut text_changes = Vec::new();

        for change in changes {
            match change {
                diff::Result::Left(line) => {
                    text_changes.push(TextChange {
                        range: 0..line.len(),
                        old_text: Some(line.to_string()),
                        new_text: None,
                        change_type: ChangeType::Deletion,
                    });
                }
                diff::Result::Both(line, _) => {
                    // Unchanged, no action needed
                }
                diff::Result::Right(line) => {
                    text_changes.push(TextChange {
                        range: 0..line.len(),
                        old_text: None,
                        new_text: Some(line.to_string()),
                        change_type: ChangeType::Insertion,
                    });
                }
            }
        }

        text_changes
    }

    /// Find nodes that correspond to changed regions
    fn find_changed_nodes(
        &self,
        old_tree: Option<&Tree>,
        new_tree: &Tree,
        changes: &[TextChange],
    ) -> Vec<ChangedNode> {
        let mut changed_nodes = Vec::new();

        // Walk the new tree and check if nodes contain changes
        let mut cursor = new_tree.walk();
        if cursor.goto_first_child() {
            loop {
                let node = cursor.node();
                let node_range = node.start_position()..node.end_position();

                // Check if this node intersects with any changes
                let has_changes = changes.iter().any(|change| {
                    // Simplified overlap check
                    // In production, do proper range intersection
                    true
                });

                if has_changes {
                    changed_nodes.push(ChangedNode {
                        node: node.clone(),
                        change_type: ChangeType::Modification,
                        line_number: node.start_position().row as u32 + 1,
                    });
                }

                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }

        changed_nodes
    }

    /// Re-analyze only changed nodes
    pub fn analyze_changes(
        &self,
        changed_nodes: &[ChangedNode],
        context: &AnalysisContext,
    ) -> Vec<Issue> {
        let mut issues = Vec::new();

        for changed_node in changed_nodes {
            // Run focused analysis on this node
            let node_issues = self.analyze_node(&changed_node.node, context);
            issues.extend(node_issues);
        }

        issues
    }

    fn analyze_node(&self, node: &Node, context: &AnalysisContext) -> Vec<Issue> {
        // Focused analysis on a specific node
        // This is more efficient than analyzing the entire tree
        let mut issues = Vec::new();

        // Example: Check for complexity issues in this function
        if node.kind() == "function_item" || node.kind() == "method_declaration" {
            let complexity = self.calculate_node_complexity(node);
            if complexity > 10 {
                issues.push(Issue {
                    severity: Severity::Warning,
                    message: format!("High cyclomatic complexity: {}", complexity),
                    line: node.start_position().row as u32 + 1,
                    column: node.start_position().column,
                    rule_id: "COMPLEX001".to_string(),
                    snippet: None,
                });
            }
        }

        issues
    }

    fn calculate_node_complexity(&self, node: &Node) -> u32 {
        let mut complexity = 1;
        let mut cursor = node.walk();

        // Walk the subtree and count decision points
        if cursor.goto_first_child() {
            loop {
                let kind = cursor.node().kind();
                if kind == "if_statement" ||
                   kind == "for_statement" ||
                   kind == "while_statement" ||
                   kind == "case_statement" ||
                   kind == "binary_expression" {
                    complexity += 1;
                }

                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }

        complexity
    }
}

#[derive(Debug, Clone)]
struct TextChange {
    range: std::ops::Range<usize>,
    old_text: Option<String>,
    new_text: Option<String>,
    change_type: ChangeType,
}

#[derive(Debug, Clone, PartialEq)]
enum ChangeType {
    Insertion,
    Deletion,
    Modification,
}

#[derive(Debug, Clone)]
struct ChangedNode {
    node: Node,
    change_type: ChangeType,
    line_number: u32,
}

#[derive(Debug)]
struct IncrementalParseResult {
    tree: Tree,
    changes: Vec<TextChange>,
    changed_nodes: Vec<ChangedNode>,
}
```

### 7. Security-Focused Analysis

#### Security Rule Implementation

```rust
// src-tauri/src/analysis/security_rules.rs

use tree_sitter::{Node, Tree, Parser};
use regex::Regex;
use std::collections::HashMap;

pub struct SecurityAnalyzer {
    /// Known vulnerable patterns
    vulnerable_patterns: HashMap<&'static str, Vec<SecurityPattern>>,
    /// Credential detection patterns
    secret_patterns: Vec<Regex>,
}

#[derive(Debug)]
struct SecurityPattern {
    name: &'static str,
    description: &'static str,
    severity: Severity,
    pattern: Regex,
    remediation: &'static str,
}

impl SecurityAnalyzer {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();

        // Rust-specific security patterns
        patterns.insert("rust", vec![
            SecurityPattern {
                name: "unsafe_block",
                description: "Usage of unsafe code",
                severity: Severity::Warning,
                pattern: Regex::new(r"\bunsafe\s*\{").unwrap(),
                remediation: "Review unsafe block and ensure proper bounds checking",
            },
            SecurityPattern {
                name: "unwrap_usage",
                description: "Panic-prone unwrap() usage",
                severity: Severity::Warning,
                pattern: Regex::new(r"\.unwrap\(\)").unwrap(),
                remediation: "Use pattern matching or expect() with proper error handling",
            },
            SecurityPattern {
                name: "expect_usage",
                description: "Panic-prone expect() usage",
                severity: Severity::Warning,
                pattern: Regex::new(r"\.expect\(").unwrap(),
                remediation: "Use proper error handling instead of expect()",
            },
        ]);

        // Java-specific security patterns
        patterns.insert("java", vec![
            SecurityPattern {
                name: "sql_injection",
                description: "Potential SQL injection vulnerability",
                severity: Severity::Error,
                pattern: Regex::new(r"Statement\s*\.\s*execute\s*\(\s*['\"].*\+.*['\"]").unwrap(),
                remediation: "Use PreparedStatement with parameterized queries",
            },
            SecurityPattern {
                name: "command_injection",
                description: "Potential command injection vulnerability",
                severity: Severity::Error,
                pattern: Regex::new(r"Runtime\.getRuntime\(\)\.exec\s*\(\s*['\"].*\+").unwrap(),
                remediation: "Avoid string concatenation, use ProcessBuilder",
            },
            SecurityPattern {
                name: "weak_hash",
                description: "Weak hashing algorithm (MD5/SHA1)",
                severity: Severity::Warning,
                pattern: Regex::new(r"MessageDigest\.getInstance\s*\(\s*['\"]MD5['\"]\s*\)").unwrap(),
                remediation: "Use SHA-256 or stronger hashing algorithms",
            },
        ]);

        // Common credential patterns
        let secret_patterns = vec![
            Regex::new(r"(?i)(password\s*[=:]\s*['\"][^'\"]{8,}['\"])").unwrap(),
            Regex::new(r"(?i)(api[_-]?key\s*[=:]\s*['\"][^'\"]{8,}['\"])").unwrap(),
            Regex::new(r"(?i)(secret\s*[=:]\s*['\"][^'\"]{8,}['\"])").unwrap(),
            Regex::new(r"(?i)(token\s*[=:]\s*['\"][^'\"]{8,}['\"])").unwrap(),
            Regex::new(r"(?i)(private[_-]?key\s*[=:]\s*['\"][^'\"]{8,}['\"])").unwrap(),
            Regex::new(r"(?i)(access[_-]?key\s*[=:]\s*['\"][^'\"]{8,}['\"])").unwrap(),
        ];

        Self {
            vulnerable_patterns: patterns,
            secret_patterns,
        }
    }

    /// Perform comprehensive security analysis
    pub fn analyze_security(
        &self,
        language: &str,
        tree: &Tree,
        source: &str,
    ) -> Vec<SecurityIssue> {
        let mut issues = Vec::new();

        // Run language-specific security patterns
        if let Some(patterns) = self.vulnerable_patterns.get(language) {
            for pattern in patterns {
                issues.extend(self.check_pattern(pattern, source));
            }
        }

        // Check for hardcoded secrets
        issues.extend(self.check_hardcoded_secrets(source));

        // Check for hardcoded IPs
        issues.extend(self.check_hardcoded_ips(source));

        // Check for TODO/FIXME in security-critical code
        issues.extend(self.check_security_todos(source));

        // Check for debugging code in production
        issues.extend(self.check_debug_code(source));

        issues
    }

    /// Check for hardcoded secrets
    fn check_hardcoded_secrets(&self, source: &str) -> Vec<SecurityIssue> {
        let mut issues = Vec::new();
        let lines: Vec<&str> = source.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            for (idx, pattern) in self.secret_patterns.iter().enumerate() {
                if pattern.is_match(line) {
                    let secret_types = ["password", "api key", "secret", "token", "private key", "access key"];
                    if idx < secret_types.len() {
                        issues.push(SecurityIssue {
                            severity: Severity::Error,
                            message: format!("Hardcoded {} detected", secret_types[idx]),
                            line: (i + 1) as u32,
                            column: 0,
                            pattern_id: format!("SECRET_{}", idx),
                            remediation: "Move secrets to environment variables or secure vault".to_string(),
                            evidence: Some(line.to_string()),
                        });
                    }
                }
            }
        }

        issues
    }

    /// Check for hardcoded IP addresses
    fn check_hardcoded_ips(&self, source: &str) -> Vec<SecurityIssue> {
        let mut issues = Vec::new();
        let ip_pattern = Regex::new(r"\b(?:[0-9]{1,3}\.){3}[0-9]{1,3}\b").unwrap();
        let lines: Vec<&str> = source.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            if ip_pattern.is_match(line) && !line.contains("127.0.0.1") && !line.contains("localhost") {
                issues.push(SecurityIssue {
                    severity: Severity::Warning,
                    message: "Hardcoded IP address detected".to_string(),
                    line: (i + 1) as u32,
                    column: 0,
                    pattern_id: "IP001".to_string(),
                    remediation: "Use configuration files or environment variables for IPs".to_string(),
                    evidence: Some(line.to_string()),
                });
            }
        }

        issues
    }

    /// Check for TODO/FIXME comments in security contexts
    fn check_security_todos(&self, source: &str) -> Vec<SecurityIssue> {
        let mut issues = Vec::new();
        let lines: Vec<&str> = source.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            let lower = line.to_lowercase();
            if (lower.contains("todo") || lower.contains("fixme")) &&
               (lower.contains("security") || lower.contains("auth") || lower.contains("encrypt")) {
                issues.push(SecurityIssue {
                    severity: Severity::Warning,
                    message: "Security-related TODO/FIXME found".to_string(),
                    line: (i + 1) as u32,
                    column: 0,
                    pattern_id: "TODO_SEC".to_string(),
                    remediation: "Address security TODO items before production".to_string(),
                    evidence: Some(line.to_string()),
                });
            }
        }

        issues
    }

    /// Check for debug code in production
    fn check_debug_code(&self, source: &str) -> Vec<SecurityIssue> {
        let mut issues = Vec::new();
        let debug_patterns = vec![
            (Regex::new(r"console\.log\s*\(").unwrap(), "console.log"),
            (Regex::new(r"print\s*\(").unwrap(), "print statement"),
            (Regex::new(r"System\.out\.print").unwrap(), "System.out.print"),
        ];

        let lines: Vec<&str> = source.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            for (pattern, name) in &debug_patterns {
                if pattern.is_match(line) {
                    issues.push(SecurityIssue {
                        severity: Severity::Info,
                        message: format!("Debug code ({}) in production", name),
                        line: (i + 1) as u32,
                        column: 0,
                        pattern_id: "DEBUG001".to_string(),
                        remediation: "Remove debug code before production deployment".to_string(),
                        evidence: Some(line.to_string()),
                    });
                }
            }
        }

        issues
    }

    fn check_pattern(&self, pattern: &SecurityPattern, source: &str) -> Vec<SecurityIssue> {
        let mut issues = Vec::new();
        let lines: Vec<&str> = source.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            if pattern.pattern.is_match(line) {
                issues.push(SecurityIssue {
                    severity: pattern.severity.clone(),
                    message: pattern.description.to_string(),
                    line: (i + 1) as u32,
                    column: 0,
                    pattern_id: pattern.name.to_string(),
                    remediation: pattern.remediation.to_string(),
                    evidence: Some(line.to_string()),
                });
            }
        }

        issues
    }
}

#[derive(Debug, Clone)]
pub struct SecurityIssue {
    pub severity: Severity,
    pub message: String,
    pub line: u32,
    pub column: u32,
    pub pattern_id: String,
    pub remediation: String,
    pub evidence: Option<String>,
}
```

#### Security Checklist Generation

```rust
// src-tauri/src/analysis/checklist_generator.rs

use super::*;
use std::collections::HashSet;

pub struct ChecklistGenerator {
    security_analyzer: SecurityAnalyzer,
}

impl ChecklistGenerator {
    pub fn new() -> Self {
        Self {
            security_analyzer: SecurityAnalyzer::new(),
        }
    }

    /// Generate security checklist based on modified files
    pub fn generate_security_checklist(
        &self,
        files: &[&str], // Modified files
        language: &str,
    ) -> Vec<ChecklistItem> {
        let mut items = Vec::new();

        // Language-specific checklist items
        match language {
            "rust" => {
                items.extend(self.generate_rust_security_checklist(files));
            }
            "java" => {
                items.extend(self.generate_java_security_checklist(files));
            }
            "javascript" | "typescript" => {
                items.extend(self.generate_js_security_checklist(files));
            }
            _ => {}
        }

        // Universal security checks
        items.extend(self.generate_universal_security_checklist());

        // Remove duplicates
        self.deduplicate_items(&mut items);

        items
    }

    fn generate_rust_security_checklist(&self, files: &[&str]) -> Vec<ChecklistItem> {
        vec![
            ChecklistItem {
                id: "RUST_SEC_001".to_string(),
                title: "Review unsafe blocks".to_string(),
                description: "Ensure all unsafe blocks have proper justification and safety comments".to_string(),
                category: ChecklistCategory::Security,
                severity: Severity::Warning,
                applicable: files.iter().any(|f| f.ends_with(".rs")),
            },
            ChecklistItem {
                id: "RUST_SEC_002".to_string(),
                title: "Validate error handling".to_string(),
                description: "Ensure unwrap/expect are not used in production code".to_string(),
                category: ChecklistCategory::Security,
                severity: Severity::Warning,
                applicable: files.iter().any(|f| f.ends_with(".rs")),
            },
            ChecklistItem {
                id: "RUST_SEC_003".to_string(),
                title: "Check memory safety".to_string(),
                description: "Verify no memory leaks, buffer overflows, or use-after-free vulnerabilities".to_string(),
                category: ChecklistCategory::Security,
                severity: Severity::Error,
                applicable: true,
            },
        ]
    }

    fn generate_java_security_checklist(&self, files: &[&str]) -> Vec<ChecklistItem> {
        vec![
            ChecklistItem {
                id: "JAVA_SEC_001".to_string(),
                title: "SQL Injection prevention".to_string(),
                description: "Verify all database queries use PreparedStatement with parameterized queries".to_string(),
                category: ChecklistCategory::Security,
                severity: Severity::Error,
                applicable: files.iter().any(|f| f.ends_with(".java")),
            },
            ChecklistItem {
                id: "JAVA_SEC_002".to_string(),
                title: "Command Injection prevention".to_string(),
                description: "Ensure Runtime.exec or ProcessBuilder calls are sanitized".to_string(),
                category: ChecklistCategory::Security,
                severity: Severity::Error,
                applicable: files.iter().any(|f| f.ends_with(".java")),
            },
            ChecklistItem {
                id: "JAVA_SEC_003".to_string(),
                title: "Weak cryptography check".to_string(),
                description: "Verify no MD5 or SHA1 usage for security-sensitive operations".to_string(),
                category: ChecklistCategory::Security,
                severity: Severity::Warning,
                applicable: files.iter().any(|f| f.ends_with(".java")),
            },
            ChecklistItem {
                id: "JAVA_SEC_004".to_string(),
                title: "Input validation".to_string(),
                description: "Ensure all user inputs are properly validated and sanitized".to_string(),
                category: ChecklistCategory::Security,
                severity: Severity::Error,
                applicable: true,
            },
        ]
    }

    fn generate_js_security_checklist(&self, files: &[&str]) -> Vec<ChecklistItem> {
        vec![
            ChecklistItem {
                id: "JS_SEC_001".to_string(),
                title: "XSS Prevention".to_string(),
                description: "Verify all user input is properly escaped before DOM insertion".to_string(),
                category: ChecklistCategory::Security,
                severity: Severity::Error,
                applicable: files.iter().any(|f| f.ends_with(".js") || f.ends_with(".ts")),
            },
            ChecklistItem {
                id: "JS_SEC_002".to_string(),
                title: "CSRF Protection".to_string(),
                description: "Ensure CSRF tokens are used for state-changing operations".to_string(),
                category: ChecklistCategory::Security,
                severity: Severity::Warning,
                applicable: files.iter().any(|f| f.ends_with(".js") || f.ends_with(".ts")),
            },
            ChecklistItem {
                id: "JS_SEC_003".to_string(),
                title: "Dependency vulnerabilities".to_string(),
                description: "Run npm audit or yarn audit to check for known vulnerabilities".to_string(),
                category: ChecklistCategory::Security,
                severity: Severity::Error,
                applicable: files.iter().any(|f| f.contains("package.json")),
            },
        ]
    }

    fn generate_universal_security_checklist(&self) -> Vec<ChecklistItem> {
        vec![
            ChecklistItem {
                id: "SEC_001".to_string(),
                title: "Hardcoded secrets check".to_string(),
                description: "Verify no passwords, API keys, or tokens are hardcoded".to_string(),
                category: ChecklistCategory::Security,
                severity: Severity::Error,
                applicable: true,
            },
            ChecklistItem {
                id: "SEC_002".to_string(),
                title: "Authentication review".to_string(),
                description: "Review authentication and authorization logic".to_string(),
                category: ChecklistCategory::Security,
                severity: Severity::Error,
                applicable: true,
            },
            ChecklistItem {
                id: "SEC_003".to_string(),
                title: "Logging and monitoring".to_string(),
                description: "Ensure security events are properly logged and monitored".to_string(),
                category: ChecklistCategory::Security,
                severity: Severity::Info,
                applicable: true,
            },
            ChecklistItem {
                id: "SEC_004".to_string(),
                title: "HTTPS enforcement".to_string(),
                description: "Verify all network communications use HTTPS".to_string(),
                category: ChecklistCategory::Security,
                severity: Severity::Error,
                applicable: true,
            },
        ]
    }

    fn deduplicate_items(&self, items: &mut Vec<ChecklistItem>) {
        let mut seen = HashSet::new();
        items.retain(|item| {
            if seen.contains(&item.id) {
                false
            } else {
                seen.insert(item.id.clone());
                true
            }
        });
    }
}

#[derive(Debug, Clone)]
pub struct ChecklistItem {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: ChecklistCategory,
    pub severity: Severity,
    pub applicable: bool,
}

#[derive(Debug, Clone)]
pub enum ChecklistCategory {
    Security,
    Performance,
    CodeQuality,
    Testing,
    Documentation,
}
```

---

## Summary

This research document covers critical security and analysis patterns for the HyperReview backend:

### Part A: Tauri IPC Security
- **Capability-based permissions** replace allowlist for granular control
- **Input validation** using validator crate with custom rules
- **Secure credential storage** with OS keychain for sensitive data
- **Command injection prevention** through safe execution patterns
- **Sandboxing** with file system and process restrictions
- **Trust boundary** validation on both frontend and backend

### Part B: Static Analysis with tree-sitter
- **Multi-language support** with dynamic grammar loading
- **Cyclomatic complexity** calculation with weighted patterns
- **Rule engine** architecture for extensible security analysis
- **Performance optimization** with incremental parsing and caching
- **Security-focused rules** for secrets, injection, and vulnerabilities
- **Smart checklists** generated based on language and file context

### Key Security Considerations
1. Never trust frontend input - always validate on backend
2. Use capability-based permissions instead of global allowlist
3. Store credentials in OS keychain, not plugin store
4. Validate all file paths to prevent traversal attacks
5. Run security analysis on all code changes
6. Implement rate limiting for IPC commands
7. Use parameterized queries to prevent SQL injection

### Performance Considerations
1. Cache parsed trees for incremental analysis
2. Load grammars on-demand to reduce bundle size
3. Use incremental parsing for diff views
4. Track performance metrics for optimization
5. Implement batch analysis for multiple files
