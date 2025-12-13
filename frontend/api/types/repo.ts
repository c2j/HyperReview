export interface Repo {
  path: string;
  branch: string;
  lastOpened: string; // 暂时用 "xx mins ago" 格式
}