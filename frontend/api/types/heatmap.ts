export interface HeatmapItem {
  file_path: string;          // Relative path
  impact_score: number;       // 0-100 (higher = more important)
  churn_score: number;        // 0-100 (frequency of changes)
  complexity_score: number;   // 0-100 (code complexity)
  change_frequency: number;   // Number of recent changes
  lines_of_code: number;      // Total LOC
  category: 'High' | 'Medium' | 'Low';  // Impact category
}
