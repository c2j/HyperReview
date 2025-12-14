export interface Branch {
  name: string;               // Branch name
  is_current: boolean;        // Is this the active branch?
  is_remote: boolean;         // Is this a remote branch?
  upstream?: string;          // Upstream branch name
  last_commit: string;        // Latest commit hash
  last_commit_message: string;
  last_commit_author: string;
  last_commit_date: string;   // ISO 8601 timestamp
}