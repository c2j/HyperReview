export interface ReviewStats {
  total_files?: number;        // Total files in review
  reviewed_files?: number;     // Files with comments/changes reviewed
  pending_files?: number;      // Files not yet reviewed
  total_comments?: number;     // Total comments added
  severe_issues?: number;      // Comments marked as severe
  completion_percentage?: number;  // 0-100
  estimated_time_remaining?: number;  // Minutes
  files_per_hour?: number;     // Review velocity
  reviewedCount: number;
  totalCount: number;
  severeCount: number;
  warningCount: number;
  pendingCount: number;
  estimatedTime: string;
}
