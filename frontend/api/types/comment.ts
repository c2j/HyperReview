export interface Comment {
  id: string;                 // UUID v4
  file_path: string;          // Relative path within repository
  line_number: number;        // Line number in diff
  content: string;            // Comment text
  author: string;             // User identifier from local config
  created_at: string;         // ISO 8601 timestamp
  updated_at: string;         // ISO 8601 timestamp
  status: 'Draft' | 'Submitted' | 'Rejected';
  parent_id?: string;         // For threaded comments
  tags: string[];             // Associated tag IDs
}
