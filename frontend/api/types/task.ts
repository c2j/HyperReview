export interface Task {
  id: string;
  title: string;
  status: 'active' | 'pending' | 'completed' | 'blocked';
  unreadCount?: number;
}