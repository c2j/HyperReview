export interface QualityGate {
  id: string;
  name: string;
  status: 'passed' | 'failed' | 'warning';
  message: string;
}