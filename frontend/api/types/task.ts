// File review status for task files
export type FileReviewStatus = 'pending' | 'approved' | 'concern' | 'must_change' | 'question';

export interface TaskFile {
  id: string;
  path: string;
  name: string;
  status: 'modified' | 'added' | 'deleted';
  reviewStatus?: FileReviewStatus;  // 审核状态
  reviewComment?: string;  // 审核备注
}

// File review comment - historical record
export interface FileReviewComment {
  id: string;
  taskId: string;
  fileId: string;
  reviewStatus: string;
  reviewComment: string;
  submittedBy: string;
  submittedAt: string;
}

export interface Task {
  id: string;                 // UUID v4
  title: string;              // Task description
  description?: string;       // Detailed description
  status: 'Active' | 'Pending' | 'Completed' | 'Blocked' | 'active' | 'pending' | 'completed';
  priority?: number;          // 1-5 (5 = highest)
  assignee?: string;          // User identifier
  created_at?: string;        // ISO 8601 timestamp
  updated_at?: string;        // ISO 8601 timestamp
  due_date?: string;          // ISO 8601 timestamp
  metadata?: Record<string, string>;  // Flexible key-value store
  type?: string;              // Task type (e.g., 'code', 'sql', 'security')
  unreadCount?: number;       // For watched tasks
  files?: TaskFile[];         // Files associated with task with review status
}

// Alias for backward compatibility
export type TaskStatus = Task['status'];