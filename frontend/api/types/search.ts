export interface SearchResult {
  result_type: 'File' | 'Symbol' | 'Commit' | 'Command';
  file_path?: string;         // For file/symbol results
  line_number?: number;       // For symbol/line results
  content: string;            // Matched content or description
  highlight?: string;         // Highlighted match
  score: number;              // Relevance score 0-100
}