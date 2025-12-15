export interface Repository {
  path: string;              // Absolute path to repository
  current_branch: string;     // Active branch name
  last_opened: string;        // ISO 8601 timestamp
  head_commit: string;        // Current HEAD commit hash
  remote_url?: string;        // Remote repository URL
  is_active: boolean;         // Is this the currently loaded repo?
}

// Alias for backward compatibility
export type Repo = Repository;