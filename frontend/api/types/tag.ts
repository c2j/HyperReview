export interface Tag {
  id: string;                 // UUID v4
  label: string;              // Tag name
  color: string;              // Hex color code (#RRGGBB)
  description?: string;       // Optional description
  usage_count: number;        // How many items use this tag
  created_at: string;         // ISO 8601 timestamp
  updated_at: string;         // ISO 8601 timestamp
}