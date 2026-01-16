export type TaskStatus = 'in_progress' | 'completed' | 'archived';
export type TaskSeverity = 'error' | 'warning' | 'question' | 'ok';
export type ExportFormat = 'json' | 'csv';

export interface LineRange {
  start?: number;
  end?: number;
}

export interface Comment {
  id: string;
  author: string;
  content: string;
  created_at: string;
  line_number?: number;
}

export interface TaskItem {
  file: string;
  line_range?: LineRange;
  preset_comment?: string;
  severity?: TaskSeverity;
  tags: string[];
  reviewed: boolean;
  comments: Comment[];
}

export interface LocalTask {
  id: string;
  name: string;
  repo_path: string;
  base_ref: string;
  create_time: string;
  update_time: string;
  status: TaskStatus;
  total_items: number;
  completed_items: number;
  items: TaskItem[];
}

export interface TaskSummary {
  id: string;
  name: string;
  status: TaskStatus;
  progress: number;
}
