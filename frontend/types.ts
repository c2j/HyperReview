export enum ReviewSeverity {
  ERROR = 'ERROR',
  WARNING = 'WARNING',
  INFO = 'INFO',
  SUCCESS = 'SUCCESS'
}

export interface DiffLine {
  oldLineNumber?: number;
  newLineNumber?: number;
  content: string;
  type: 'added' | 'removed' | 'context' | 'header';
  severity?: ReviewSeverity;
  message?: string;
}

export interface FileNode {
  id: string;
  name: string;
  path: string;
  type: 'file' | 'folder';
  status: 'modified' | 'added' | 'deleted';
  children?: FileNode[];
  stats?: {
    added: number;
    removed: number;
  };
}

export interface Task {
  id: string;
  title: string;
  status: 'active' | 'pending' | 'completed' | 'blocked';
  unreadCount?: number;
}