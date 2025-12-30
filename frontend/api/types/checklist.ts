import { ReviewSeverity } from './diff';

export interface ChecklistItem {
  id: string;                 // UUID v4
  description?: string;       // Checklist item text
  category?: 'Security' | 'Performance' | 'Style' | 'Architecture' | 'Testing' | 'Documentation';
  severity?: ReviewSeverity;
  applicable_file_types?: string[];  // e.g., ['.rs', '.ts', '.js']
  applicable_patterns?: string[];    // Regex patterns
  is_checked?: boolean;       // User checked this item?
  is_auto_checkable?: boolean; // Can be auto-verified?
  related_file?: string;      // Optional file association
  text: string;               // Simplified text field
  checked: boolean;           // Simplified checked field
}
