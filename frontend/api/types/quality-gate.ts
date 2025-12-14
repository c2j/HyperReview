export interface QualityGate {
  name: string;               // Gate name
  status: 'Passing' | 'Failing' | 'Pending' | 'Unknown';
  details?: string;           // Additional details
  last_checked: string;       // ISO 8601 timestamp
  url?: string;               // Link to external system
  metadata: Record<string, string>;  // Flexible metadata
}