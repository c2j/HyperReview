export interface HeatmapItem {
  id: string;
  name: string;
  path: string;
  impact: 'High' | 'Medium' | 'Low';
  score: number;
  exists: boolean; // Whether the file exists in the working directory
}
