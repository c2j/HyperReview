/**
 * Data Validation Utilities
 * Validates API responses from Tauri backend
 */

import type {
  Repository,
  Branch,
  Task,
  DiffLine,
  HeatmapItem,
  BlameInfo,
  ReviewStats,
  ChecklistItem,
  Tag,
  SearchResult,
  ReviewTemplate,
  QualityGate,
  Comment
} from '../api/types';

// ============================================================================
// Type Guards
// ============================================================================

/**
 * Type guard for Repository
 */
export function isRepository(value: any): value is Repository {
  return (
    value &&
    typeof value === 'object' &&
    typeof value.path === 'string' &&
    typeof value.name === 'string' &&
    typeof value.current_branch === 'string' &&
    typeof value.head_commit === 'string' &&
    typeof value.is_active === 'boolean'
  );
}

/**
 * Type guard for Branch
 */
export function isBranch(value: any): value is Branch {
  return (
    value &&
    typeof value === 'object' &&
    typeof value.name === 'string' &&
    typeof value.is_current === 'boolean' &&
    typeof value.is_remote === 'boolean' &&
    typeof value.last_commit === 'string' &&
    typeof value.last_commit_message === 'string' &&
    typeof value.last_commit_date === 'string'
  );
}

/**
 * Type guard for Task
 */
export function isTask(value: any): value is Task {
  return (
    value &&
    typeof value === 'object' &&
    typeof value.id === 'string' &&
    typeof value.title === 'string' &&
    (value.status === 'Active' ||
      value.status === 'Pending' ||
      value.status === 'Completed' ||
      value.status === 'Blocked') &&
    (value.priority === 'Low' ||
      value.priority === 'Medium' ||
      value.priority === 'High' ||
      value.priority === 'Critical')
  );
}

/**
 * Type guard for DiffLine
 */
export function isDiffLine(value: any): value is DiffLine {
  return (
    value &&
    typeof value === 'object' &&
    typeof value.content === 'string' &&
    (value.line_type === 'added' ||
      value.line_type === 'removed' ||
      value.line_type === 'context' ||
      value.line_type === 'header') &&
    (value.old_line_number === undefined || typeof value.old_line_number === 'number') &&
    (value.new_line_number === undefined || typeof value.new_line_number === 'number')
  );
}

/**
 * Type guard for HeatmapItem
 */
export function isHeatmapItem(value: any): value is HeatmapItem {
  return (
    value &&
    typeof value === 'object' &&
    typeof value.path === 'string' &&
    typeof value.impact_score === 'number' &&
    typeof value.category === 'string'
  );
}

/**
 * Type guard for BlameInfo
 */
export function isBlameInfo(value: any): value is BlameInfo {
  return (
    value &&
    typeof value === 'object' &&
    typeof value.commit_oid === 'string' &&
    typeof value.author_name === 'string' &&
    typeof value.author_email === 'string' &&
    typeof value.timestamp === 'string' &&
    typeof value.message === 'string'
  );
}

/**
 * Type guard for ReviewStats
 */
export function isReviewStats(value: any): value is ReviewStats {
  return (
    value &&
    typeof value === 'object' &&
    typeof value.total_files === 'number' &&
    typeof value.files_reviewed === 'number' &&
    typeof value.completion_percentage === 'number' &&
    typeof value.files_per_hour === 'number'
  );
}

/**
 * Type guard for ChecklistItem
 */
export function isChecklistItem(value: any): value is ChecklistItem {
  return (
    value &&
    typeof value === 'object' &&
    typeof value.id === 'string' &&
    typeof value.description === 'string' &&
    typeof value.checked === 'boolean' &&
    typeof value.category === 'string' &&
    typeof value.severity === 'string'
  );
}

/**
 * Type guard for Tag
 */
export function isTag(value: any): value is Tag {
  return (
    value &&
    typeof value === 'object' &&
    typeof value.id === 'string' &&
    typeof value.label === 'string' &&
    typeof value.color === 'string'
  );
}

/**
 * Type guard for SearchResult
 */
export function isSearchResult(value: any): value is SearchResult {
  return (
    value &&
    typeof value === 'object' &&
    typeof value.type === 'string' &&
    typeof value.label === 'string' &&
    typeof value.path === 'string'
  );
}

/**
 * Type guard for ReviewTemplate
 */
export function isReviewTemplate(value: any): value is ReviewTemplate {
  return (
    value &&
    typeof value === 'object' &&
    typeof value.id === 'string' &&
    typeof value.name === 'string' &&
    typeof value.content === 'string' &&
    Array.isArray(value.placeholders) &&
    typeof value.created_at === 'string' &&
    typeof value.updated_at === 'string'
  );
}

/**
 * Type guard for QualityGate
 */
export function isQualityGate(value: any): value is QualityGate {
  return (
    value &&
    typeof value === 'object' &&
    typeof value.name === 'string' &&
    (value.status === 'Passing' ||
      value.status === 'Failing' ||
      value.status === 'Pending' ||
      value.status === 'Unknown') &&
    typeof value.last_checked === 'string'
  );
}

/**
 * Type guard for Comment
 */
export function isComment(value: any): value is Comment {
  return (
    value &&
    typeof value === 'object' &&
    typeof value.id === 'string' &&
    typeof value.author === 'string' &&
    typeof value.content === 'string' &&
    typeof value.created_at === 'string'
  );
}

// ============================================================================
// Array Validators
// ============================================================================

/**
 * Validate array of Repositories
 */
export function validateRepositoryArray(value: any): value is Repository[] {
  return Array.isArray(value) && value.every(isRepository);
}

/**
 * Validate array of Branches
 */
export function validateBranchArray(value: any): value is Branch[] {
  return Array.isArray(value) && value.every(isBranch);
}

/**
 * Validate array of Tasks
 */
export function validateTaskArray(value: any): value is Task[] {
  return Array.isArray(value) && value.every(isTask);
}

/**
 * Validate array of DiffLines
 */
export function validateDiffLineArray(value: any): value is DiffLine[] {
  return Array.isArray(value) && value.every(isDiffLine);
}

/**
 * Validate array of HeatmapItems
 */
export function validateHeatmapArray(value: any): value is HeatmapItem[] {
  return Array.isArray(value) && value.every(isHeatmapItem);
}

/**
 * Validate array of ChecklistItems
 */
export function validateChecklistArray(value: any): value is ChecklistItem[] {
  return Array.isArray(value) && value.every(isChecklistItem);
}

/**
 * Validate array of Tags
 */
export function validateTagArray(value: any): value is Tag[] {
  return Array.isArray(value) && value.every(isTag);
}

/**
 * Validate array of SearchResults
 */
export function validateSearchResultArray(value: any): value is SearchResult[] {
  return Array.isArray(value) && value.every(isSearchResult);
}

/**
 * Validate array of ReviewTemplates
 */
export function validateReviewTemplateArray(value: any): value is ReviewTemplate[] {
  return Array.isArray(value) && value.every(isReviewTemplate);
}

/**
 * Validate array of QualityGates
 */
export function validateQualityGateArray(value: any): value is QualityGate[] {
  return Array.isArray(value) && value.every(isQualityGate);
}

/**
 * Validate array of Comments
 */
export function validateCommentArray(value: any): value is Comment[] {
  return Array.isArray(value) && value.every(isComment);
}

// ============================================================================
// Validation Result Type
// ============================================================================

export interface ValidationResult<T = any> {
  valid: boolean;
  value?: T;
  error?: string;
}

// ============================================================================
// Validation Functions with Results
// ============================================================================

/**
 * Validate and return Repository
 */
export function validateRepository(value: any): ValidationResult<Repository> {
  if (isRepository(value)) {
    return { valid: true, value };
  }
  return {
    valid: false,
    error: 'Invalid Repository object'
  };
}

/**
 * Validate and return Branch
 */
export function validateBranch(value: any): ValidationResult<Branch> {
  if (isBranch(value)) {
    return { valid: true, value };
  }
  return {
    valid: false,
    error: 'Invalid Branch object'
  };
}

/**
 * Validate and return Task
 */
export function validateTask(value: any): ValidationResult<Task> {
  if (isTask(value)) {
    return { valid: true, value };
  }
  return {
    valid: false,
    error: 'Invalid Task object'
  };
}

/**
 * Validate and return DiffLine
 */
export function validateDiffLine(value: any): ValidationResult<DiffLine> {
  if (isDiffLine(value)) {
    return { valid: true, value };
  }
  return {
    valid: false,
    error: 'Invalid DiffLine object'
  };
}

/**
 * Validate and return array of DiffLines
 */
export function validateDiffLines(value: any): ValidationResult<DiffLine[]> {
  if (validateDiffLineArray(value)) {
    return { valid: true, value };
  }
  return {
    valid: false,
    error: 'Invalid DiffLine array'
  };
}

/**
 * Validate and return HeatmapItem
 */
export function validateHeatmapItem(value: any): ValidationResult<HeatmapItem> {
  if (isHeatmapItem(value)) {
    return { valid: true, value };
  }
  return {
    valid: false,
    error: 'Invalid HeatmapItem object'
  };
}

/**
 * Validate and return array of HeatmapItems
 */
export function validateHeatmap(value: any): ValidationResult<HeatmapItem[]> {
  if (validateHeatmapArray(value)) {
    return { valid: true, value };
  }
  return {
    valid: false,
    error: 'Invalid HeatmapItem array'
  };
}

/**
 * Validate and return BlameInfo
 */
export function validateBlameInfo(value: any): ValidationResult<BlameInfo> {
  if (isBlameInfo(value)) {
    return { valid: true, value };
  }
  return {
    valid: false,
    error: 'Invalid BlameInfo object'
  };
}

/**
 * Validate and return ReviewStats
 */
export function validateReviewStats(value: any): ValidationResult<ReviewStats> {
  if (isReviewStats(value)) {
    return { valid: true, value };
  }
  return {
    valid: false,
    error: 'Invalid ReviewStats object'
  };
}

/**
 * Validate and return ChecklistItem
 */
export function validateChecklistItem(value: any): ValidationResult<ChecklistItem> {
  if (isChecklistItem(value)) {
    return { valid: true, value };
  }
  return {
    valid: false,
    error: 'Invalid ChecklistItem object'
  };
}

/**
 * Validate and return array of ChecklistItems
 */
export function validateChecklist(value: any): ValidationResult<ChecklistItem[]> {
  if (validateChecklistArray(value)) {
    return { valid: true, value };
  }
  return {
    valid: false,
    error: 'Invalid ChecklistItem array'
  };
}

/**
 * Validate and return Tag
 */
export function validateTag(value: any): ValidationResult<Tag> {
  if (isTag(value)) {
    return { valid: true, value };
  }
  return {
    valid: false,
    error: 'Invalid Tag object'
  };
}

/**
 * Validate and return array of Tags
 */
export function validateTags(value: any): ValidationResult<Tag[]> {
  if (validateTagArray(value)) {
    return { valid: true, value };
  }
  return {
    valid: false,
    error: 'Invalid Tag array'
  };
}

/**
 * Validate and return SearchResult
 */
export function validateSearchResult(value: any): ValidationResult<SearchResult> {
  if (isSearchResult(value)) {
    return { valid: true, value };
  }
  return {
    valid: false,
    error: 'Invalid SearchResult object'
  };
}

/**
 * Validate and return array of SearchResults
 */
export function validateSearchResults(value: any): ValidationResult<SearchResult[]> {
  if (validateSearchResultArray(value)) {
    return { valid: true, value };
  }
  return {
    valid: false,
    error: 'Invalid SearchResult array'
  };
}

/**
 * Validate and return ReviewTemplate
 */
export function validateReviewTemplate(value: any): ValidationResult<ReviewTemplate> {
  if (isReviewTemplate(value)) {
    return { valid: true, value };
  }
  return {
    valid: false,
    error: 'Invalid ReviewTemplate object'
  };
}

/**
 * Validate and return array of ReviewTemplates
 */
export function validateReviewTemplates(value: any): ValidationResult<ReviewTemplate[]> {
  if (validateReviewTemplateArray(value)) {
    return { valid: true, value };
  }
  return {
    valid: false,
    error: 'Invalid ReviewTemplate array'
  };
}

/**
 * Validate and return QualityGate
 */
export function validateQualityGate(value: any): ValidationResult<QualityGate> {
  if (isQualityGate(value)) {
    return { valid: true, value };
  }
  return {
    valid: false,
    error: 'Invalid QualityGate object'
  };
}

/**
 * Validate and return array of QualityGates
 */
export function validateQualityGates(value: any): ValidationResult<QualityGate[]> {
  if (validateQualityGateArray(value)) {
    return { valid: true, value };
  }
  return {
    valid: false,
    error: 'Invalid QualityGate array'
  };
}

/**
 * Validate and return Comment
 */
export function validateComment(value: any): ValidationResult<Comment> {
  if (isComment(value)) {
    return { valid: true, value };
  }
  return {
    valid: false,
    error: 'Invalid Comment object'
  };
}

/**
 * Validate and return array of Comments
 */
export function validateComments(value: any): ValidationResult<Comment[]> {
  if (validateCommentArray(value)) {
    return { valid: true, value };
  }
  return {
    valid: false,
    error: 'Invalid Comment array'
  };
}

// ============================================================================
// Utility Functions
// ============================================================================

/**
 * Validate optional field
 */
export function validateOptional<T>(
  value: any,
  validator: (value: any) => ValidationResult<T>
): ValidationResult<T | undefined> {
  if (value === undefined || value === null) {
    return { valid: true, value: undefined };
  }
  return validator(value);
}

/**
 * Validate required field
 */
export function validateRequired<T>(
  value: any,
  validator: (value: any) => ValidationResult<T>,
  fieldName: string
): ValidationResult<T> {
  if (value === undefined || value === null) {
    return {
      valid: false,
      error: `Required field '${fieldName}' is missing`
    };
  }
  return validator(value);
}
