export interface ReviewTemplate {
  id: string;                 // UUID v4
  name: string;               // Template name
  content: string;            // Template content with placeholders
  placeholders: string[];     // List of placeholders
  category?: string;          // Optional category
  usage_count: number;        // Usage statistics
  created_at: string;         // ISO 8601 timestamp
  updated_at: string;         // ISO 8601 timestamp
}