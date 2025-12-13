import type { 
  Repo, Branch, Task, DiffLine, HeatmapItem, BlameInfo, ReviewStats, 
  ChecklistItem, Tag, SearchResult, ReviewTemplate, QualityGate 
} from './types';
import { ReviewSeverity } from './types';

// 在当前演示环境中默认为 true，实际项目中使用 import.meta.env.VITE_MOCK === 'true'
const MOCK = true; 

// Tauri IPC window interface extension
declare global {
  interface Window {
    __TAURI_IPC__?: any;
    __TAURI__?: {
        invoke: (cmd: string, args?: any) => Promise<any>;
    };
    showDirectoryPicker?: () => Promise<FileSystemDirectoryHandle>;
  }
}

// --- Exports ---

export const getRecentRepos = (): Promise<Repo[]> =>
  MOCK ? mockGetRecentRepos() : isTauri() ? tauriGetRecentRepos() : httpGetRecentRepos();

export const getBranches = (): Promise<Branch[]> => 
  MOCK ? mockGetBranches() : isTauri() ? invoke('get_branches') : httpGetBranches();

export const getTasks = (type: 'pending' | 'watched'): Promise<Task[]> => 
  MOCK ? mockGetTasks(type) : isTauri() ? invoke('get_tasks', { type }) : httpGetTasks(type);

export const getFileDiff = (fileId: string): Promise<DiffLine[]> => 
  MOCK ? mockGetFileDiff(fileId) : isTauri() ? invoke('get_file_diff', { fileId }) : httpGetFileDiff(fileId);

export const getHeatmap = (): Promise<HeatmapItem[]> =>
  MOCK ? mockGetHeatmap() : isTauri() ? invoke('get_heatmap') : httpGetHeatmap();

export const getBlame = (fileId: string): Promise<BlameInfo> =>
  MOCK ? mockGetBlame(fileId) : isTauri() ? invoke('get_blame', { fileId }) : httpGetBlame(fileId);

export const getReviewStats = (): Promise<ReviewStats> =>
  MOCK ? mockGetReviewStats() : isTauri() ? invoke('get_review_stats') : httpGetReviewStats();

export const getChecklist = (): Promise<ChecklistItem[]> =>
  MOCK ? mockGetChecklist() : isTauri() ? invoke('get_checklist') : httpGetChecklist();

export const getTags = (): Promise<Tag[]> =>
  MOCK ? mockGetTags() : isTauri() ? invoke('get_tags') : httpGetTags();

export const getCommands = (): Promise<SearchResult[]> =>
  MOCK ? mockGetCommands() : isTauri() ? invoke('get_commands') : httpGetCommands();

export const getReviewTemplates = (): Promise<ReviewTemplate[]> =>
  MOCK ? mockGetReviewTemplates() : isTauri() ? invoke('get_review_templates') : httpGetReviewTemplates();

export const getQualityGates = (): Promise<QualityGate[]> =>
  MOCK ? mockGetQualityGates() : isTauri() ? invoke('get_quality_gates') : httpGetQualityGates();

export const openLocalRepoDialog = (): Promise<string | null> =>
  MOCK ? mockOpenLocalRepoDialog() : isTauri() ? invoke('open_repo_dialog') : httpOpenLocalRepoDialog();


/* --------- Mock Implementations --------- */

async function mockGetRecentRepos(): Promise<Repo[]> {
  await new Promise(r => setTimeout(r, 200)); 
  // 为了演示空状态效果，您可以暂时返回空数组 []，或者保持原样
  return [
    { path: '~/work/payment-service', branch: 'feature/payment-retry', lastOpened: '2 mins ago' },
    { path: '~/work/auth-center', branch: 'master', lastOpened: '1 hour ago' },
    { path: '~/work/legacy-monolith', branch: 'hotfix/npe-fix', lastOpened: '2 days ago' },
    { path: '~/personal/dotfiles', branch: 'main', lastOpened: '1 week ago' },
    { path: '~/infrastructure/k8s-configs', branch: 'staging', lastOpened: '2 weeks ago' },
  ];
}

async function mockGetBranches(): Promise<Branch[]> {
  await new Promise(r => setTimeout(r, 150));
  return [
    { name: 'master' },
    { name: 'main' },
    { name: 'develop' },
    { name: 'feature/payment-retry' },
    { name: 'feature/auth-refactor' },
    { name: 'hotfix/login-issue' },
    { name: 'release/v3.1.0' }
  ];
}

async function mockGetTasks(type: 'pending' | 'watched'): Promise<Task[]> {
  await new Promise(r => setTimeout(r, 100));
  if (type === 'pending') {
    return [
      { id: '1', title: 'PR#2877 Pay Retry Refactor', status: 'active' },
      { id: '2', title: 'PR#2871 Auth Center Update', status: 'pending', unreadCount: 1 },
      { id: '3', title: 'PR#2869 Oracle Proc Opt', status: 'pending' },
      { id: '4', title: 'Local#11 SQL Review', status: 'pending' },
    ];
  } else {
    return [
      { id: '5', title: 'PR#2847 Async Task Refactor', status: 'pending', unreadCount: 3 },
    ];
  }
}

async function mockGetFileDiff(fileId: string): Promise<DiffLine[]> {
  await new Promise(r => setTimeout(r, 300));
  return [
    { oldLineNumber: 1, newLineNumber: 1, content: 'import java.util.List;', type: 'context' },
    { oldLineNumber: 2, newLineNumber: 2, content: 'import java.util.Map;', type: 'context' },
    { oldLineNumber: 3, newLineNumber: 3, content: 'import lombok.extern.slf4j.Slf4j;', type: 'context' },
    { oldLineNumber: 4, newLineNumber: 4, content: 'import org.springframework.stereotype.Service;', type: 'context' },
    { oldLineNumber: 5, newLineNumber: 5, content: 'import com.alipay.payment.mapper.OrderMapper;', type: 'context' },
    { oldLineNumber: 6, newLineNumber: 6, content: '', type: 'context' },
    { oldLineNumber: 7, newLineNumber: 7, content: '@Slf4j', type: 'context' },
    { oldLineNumber: 8, newLineNumber: 8, content: '@Service', type: 'context' },
    { oldLineNumber: 9, newLineNumber: 9, content: '@RequiredArgsConstructor', type: 'context' },
    { oldLineNumber: 10, newLineNumber: 10, content: 'public class RetryServiceImpl implements RetryService {', type: 'context' },
    { oldLineNumber: 11, newLineNumber: 11, content: '', type: 'context' },
    { oldLineNumber: 101, content: '    public void retry(String orderId) {', type: 'context' },
    { oldLineNumber: 102, content: '        // Old logic was here', type: 'removed' },
    { newLineNumber: 125, content: '        log.info("Starting retry process for order: {}", orderId);', type: 'added' },
    { newLineNumber: 126, content: '        Order order = orderMapper.selectById(orderId);', type: 'added' },
    { newLineNumber: 127, content: '        orderMapper.updateStatus(orderId, OrderStatus.RETRYING);', type: 'added', severity: ReviewSeverity.WARNING, message: 'Potential N+1 Query Risk' },
    { newLineNumber: 128, content: '        // TODO: Add distributed lock', type: 'added', severity: ReviewSeverity.ERROR, message: 'Missing @Transactional annotation' },
    { newLineNumber: 129, content: '    }', type: 'context' },
    { oldLineNumber: 105, newLineNumber: 130, content: '', type: 'context' },
    { oldLineNumber: 106, newLineNumber: 131, content: '    private void validate(Order order) {', type: 'context' },
    { oldLineNumber: 107, newLineNumber: 132, content: '        if (order == null) throw new IllegalArgumentException();', type: 'context' },
  ];
}

async function mockGetHeatmap(): Promise<HeatmapItem[]> {
  await new Promise(r => setTimeout(r, 200));
  return [
    { id: 'payment', name: 'payment', impact: 'high' },
    { id: 'auth', name: 'auth', impact: 'high' },
    { id: 'db', name: 'db', impact: 'medium' },
    { id: 'api', name: 'api', impact: 'low' },
  ];
}

async function mockGetBlame(fileId: string): Promise<BlameInfo> {
  await new Promise(r => setTimeout(r, 150));
  return {
    author: 'alice',
    avatar: 'A',
    time: '2025-11-20 18:33',
    prName: 'PR#2711 "Introduce Retry State Machine"',
    reviewer: 'ferris',
    reviewerStatus: 'LGTM with nit',
    comment: '"This part might cause a deadlock, added lock optimization."'
  };
}

async function mockGetReviewStats(): Promise<ReviewStats> {
  await new Promise(r => setTimeout(r, 200));
  return {
    reviewedCount: 73,
    totalCount: 127,
    severeCount: 12,
    warningCount: 28,
    pendingCount: 7,
    estimatedTime: '11m'
  };
}

async function mockGetChecklist(): Promise<ChecklistItem[]> {
  await new Promise(r => setTimeout(r, 100));
  return [
    { id: '1', text: 'src/main/java/com/alipay/**/*.java', checked: false },
    { id: '2', text: 'src/main/resources/mapper/*Payment*.xml', checked: false },
    { id: '3', text: 'db/procedure/pkg_payment.pkb L200-500', checked: false },
  ];
}

async function mockGetTags(): Promise<Tag[]> {
  await new Promise(r => setTimeout(r, 100));
  return [
    { id: 1, label: 'Severe N+1', color: 'bg-editor-error' },
    { id: 2, label: 'No Tx', color: 'bg-editor-error' },
    { id: 3, label: 'Hardcoded', color: 'bg-editor-warning' },
    { id: 4, label: 'Typo', color: 'bg-editor-info' },
  ];
}

async function mockGetCommands(): Promise<SearchResult[]> {
  await new Promise(r => setTimeout(r, 150));
  return [
    { type: 'file', label: 'RetryServiceImpl.java', desc: 'src/main/java/.../impl' },
    { type: 'file', label: 'PaymentController.java', desc: 'src/main/java/.../web' },
    { type: 'symbol', label: 'method: updateStatus', desc: 'RetryServiceImpl.java' },
    { type: 'symbol', label: 'const: MAX_RETRY_COUNT', desc: 'RetryConfig.java' },
    { type: 'cmd', label: 'Toggle Vim Mode', desc: 'Settings' },
    { type: 'cmd', label: 'Fold All Regions', desc: 'View' },
  ];
}

async function mockGetReviewTemplates(): Promise<ReviewTemplate[]> {
  await new Promise(r => setTimeout(r, 100));
  return [
    { id: 'tx', label: 'Missing Transaction', content: 'Transaction boundary is missing for this composite operation.' },
    { id: 'n1', label: 'Potential N+1', content: 'This loop triggers a query for each iteration. Consider fetching in batch.' },
    { id: 'sql', label: 'Hardcoded SQL', content: 'SQL should be moved to XML mapper or repository class.' },
    { id: 'log', label: 'Missing Log', content: 'Please add log for this critical path.' },
    { id: 'sec', label: 'XSS Vulnerability', content: 'Unsanitized input detected here.' },
  ];
}

async function mockGetQualityGates(): Promise<QualityGate[]> {
  await new Promise(r => setTimeout(r, 600));
  return [
    { id: 'ci', name: 'CI Pipeline', status: 'passed', message: 'Build #8821 success' },
    { id: 'test', name: 'Unit Tests', status: 'passed', message: '100% (42/42)' },
    { id: 'cov', name: 'Code Coverage', status: 'warning', message: '82% (Target 85%)' },
    { id: 'sec', name: 'Security Scan', status: 'passed', message: 'No high risks found' },
  ];
}

async function mockOpenLocalRepoDialog(): Promise<string | null> {
  // Try to use native API if available for better realism
  if (typeof window !== 'undefined' && window.showDirectoryPicker) {
     try {
       const handle = await window.showDirectoryPicker();
       // In a real app we'd keep the handle, but here we just return a simulated path using the directory name
       return `/local/${handle.name}`;
     } catch (err) {
       // User cancelled
       return null;
     }
  }

  await new Promise(r => setTimeout(r, 600));
  return "/home/user/workspace/new-feature-repo";
}

/* --------- IPC / HTTP Placeholders --------- */

function isTauri() {
  return !!window.__TAURI_IPC__ || !!window.__TAURI__;
}

// Helper to bridge calls
const invoke = window.__TAURI__?.invoke || ((() => Promise.resolve([])) as any);

async function tauriGetRecentRepos(): Promise<Repo[]> { return invoke('get_recent_repos'); }

async function httpGetRecentRepos(): Promise<Repo[]> {
  const response = await fetch('/api/recent-repos');
  if (!response.ok) throw new Error('Network error');
  return response.json();
}

async function httpGetBranches(): Promise<Branch[]> { return []; }
async function httpGetTasks(type: string): Promise<Task[]> { return []; }
async function httpGetFileDiff(fileId: string): Promise<DiffLine[]> { return []; }
async function httpGetHeatmap(): Promise<HeatmapItem[]> { return []; }
async function httpGetBlame(fileId: string): Promise<BlameInfo> { return {} as any; }
async function httpGetReviewStats(): Promise<ReviewStats> { return {} as any; }
async function httpGetChecklist(): Promise<ChecklistItem[]> { return []; }
async function httpGetTags(): Promise<Tag[]> { return []; }
async function httpGetCommands(): Promise<SearchResult[]> { return []; }
async function httpGetReviewTemplates(): Promise<ReviewTemplate[]> { return []; }
async function httpGetQualityGates(): Promise<QualityGate[]> { return []; }
async function httpOpenLocalRepoDialog(): Promise<string | null> { return null; }