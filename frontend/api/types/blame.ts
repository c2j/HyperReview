export interface BlameLine {
  line_number: number;        // Line number in file
  content: string;            // Line content
  commit_oid: string;         // Commit hash
  commit_message: string;
  author_name: string;
  author_email: string;
  committer_name: string;
  committer_email: string;
  commit_date: string;        // ISO 8601 timestamp
}

export interface BlameInfo {
  file_path?: string;
  lines?: BlameLine[];
  author: string;
  avatar: string;
  time: string;
  prName: string;
  reviewer: string;
  reviewerStatus: string;
  comment: string;
}
