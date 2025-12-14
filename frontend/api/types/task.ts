export interface Task {
  id: string;                 // UUID v4
  title: string;              // Task description
  description?: string;       // Detailed description
  status: 'Active' | 'Pending' | 'Completed' | 'Blocked';
  priority: number;           // 1-5 (5 = highest)
  assignee?: string;          // User identifier
  created_at: string;         // ISO 8601 timestamp
  updated_at: string;         // ISO 8601 timestamp
  due_date?: string;          // ISO 8601 timestamp
  metadata: Record<string, string>;  // Flexible key-value store
}

// Alias for backward compatibility
export type TaskStatus = Task['status'];