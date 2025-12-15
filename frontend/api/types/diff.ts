export enum ReviewSeverity {
  ERROR = 'Error',
  WARNING = 'Warning',
  INFO = 'Info',
  SUCCESS = 'Success'
}

export interface DiffLine {
  old_line_number?: number;   // Line number in old version
  new_line_number?: number;   // Line number in new version
  content: string;            // Line content
  line_type: 'Added' | 'Removed' | 'Context' | 'Header';
  severity?: ReviewSeverity;
  message?: string;           // Static analysis message
  hunk_header?: string;       // Hunk context (e.g., @@ -1,3 +1,3 @@)
  isFoldPlaceholder?: boolean; // For folded sections
  onClick?: () => void;       // Click handler for folded sections
}

// Alias for backward compatibility
export type LineType = DiffLine['line_type'];