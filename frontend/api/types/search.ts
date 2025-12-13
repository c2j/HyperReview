export interface SearchResult {
  type: 'file' | 'symbol' | 'cmd';
  label: string;
  desc: string;
  icon?: string; // string identifier for icon component
}