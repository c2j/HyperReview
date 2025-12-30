export interface ReviewGuideItem {
  id: string;
  category: string; // 支持中文类别，如 "安全性", "性能优化", "代码规范", "通用原则" 等
  title: string;
  description: string;
  severity: 'high' | 'medium' | 'low';
  referenceUrl?: string;
  applicableExtensions: string[];
}
