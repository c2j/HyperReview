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
  isFoldPlaceholder?: boolean; // UI state helper
  onClick?: () => void; // UI handler helper
}