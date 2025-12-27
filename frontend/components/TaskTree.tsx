
import React, { useState, useEffect, useMemo, useRef } from 'react';
import { ChevronDown, ChevronRight, CheckCircle2, Circle, AlertCircle, GitPullRequest, List, FolderTree, FileCode, Folder, Loader2, ArrowUpDown, Filter, File, CheckCircle, AlertTriangle, MessageSquare, XCircle, Download } from 'lucide-react';
import { useTranslation } from '../i18n';
import { useApiClient } from '../api/client';
import { save } from '@tauri-apps/api/dialog';
import { writeTextFile } from '@tauri-apps/api/fs';
import type { Task, FileNode, FileReviewStatus } from '../api/types';

interface TaskTreeProps {
  activeTaskId: string;
  onSelectTask: (id: string) => void;
  onAction: (msg: string) => void;
  repoRefreshKey?: number;
  onSelectFile?: (path: string) => void;
  diffContext?: { base: string; head: string };
}

enum LeftTab {
    GIT = 'git',
    LOCAL = 'local',
    FILES = 'files'
}

type LocalSortOption = 'status' | 'type' | 'name' | 'path';

// Recursive File Tree Item Component
const FileTreeItem: React.FC<{ node: FileNode; depth?: number; onSelect: (path: string) => void; expandAll?: boolean }> = ({ node, depth = 0, onSelect, expandAll = false }) => {
    // All items are collapsed by default for better UX
    // Only expand when expandAll is true or user manually clicks
    const [expanded, setExpanded] = useState(expandAll && node.type === 'folder');
    const hasChildren = node.children && node.children.length > 0;

    // Status colors
    const getStatusColor = (s: string) => {
        switch(s) {
            case 'modified': return 'text-editor-warning';
            case 'added': return 'text-editor-success';
            case 'deleted': return 'text-editor-error';
            default: return 'text-gray-500';
        }
    };

    return (
        <div>
            <div
                className="flex items-center gap-1.5 py-1 hover:bg-editor-line/50 cursor-pointer select-none transition-colors"
                style={{ paddingLeft: `${depth * 12 + 8}px` }}
                onClick={() => {
                    if (hasChildren) setExpanded(!expanded);
                    else onSelect(node.path);
                }}
            >
                {hasChildren && (
                    <span className="text-gray-500">
                        {expanded ? <ChevronDown size={12} /> : <ChevronRight size={12} />}
                    </span>
                )}
                {!hasChildren && <span className="w-3"></span>}

                {node.type === 'folder' ? (
                    <Folder size={14} className="text-editor-accent shrink-0" />
                ) : (
                    <FileCode size={14} className={`${getStatusColor(node.status)} shrink-0`} />
                )}

                <span className={`text-xs truncate ${node.type === 'folder' ? 'text-editor-fg font-medium' : 'text-gray-400'} ${!node.exists && node.type === 'file' ? 'opacity-60' : ''}`}>
                    {node.name}
                </span>

                {/* Mark for files that don't exist in working directory */}
                {!node.exists && node.type === 'file' && (
                    <span className="text-[10px] text-gray-500 italic ml-1" title="File doesn't exist in working directory">
                        ---
                    </span>
                )}

                {node.stats && (
                    <span className="ml-auto mr-2 text-[10px] text-gray-600 flex gap-1">
                        {node.stats.added > 0 && <span className="text-editor-success">+{node.stats.added}</span>}
                        {node.stats.removed > 0 && <span className="text-editor-error">-{node.stats.removed}</span>}
                    </span>
                )}
            </div>
            {hasChildren && expanded && (
                <div>
                    {node.children!.map(child => (
                        <FileTreeItem key={child.id} node={child} depth={depth + 1} onSelect={onSelect} expandAll={expandAll} />
                    ))}
                </div>
            )}
        </div>
    );
};

const TaskTree: React.FC<TaskTreeProps> = ({ activeTaskId, onSelectTask, onAction, repoRefreshKey, onSelectFile, diffContext }) => {
  const { t } = useTranslation();
  const { getTasks, getLocalTasks, getFileTree, markTaskCompleted, exportTaskReview } = useApiClient();
  const [activeTab, setActiveTab] = useState<LeftTab>(LeftTab.GIT);

  // Tab 1 Data (Git)
  const [sections, setSections] = useState({
    pending: true,
    watched: true,
    history: false
  });
  const [pendingTasks, setPendingTasks] = useState<Task[]>([]);
  const [watchedTasks, setWatchedTasks] = useState<Task[]>([]);

  // Tab 2 Data (Local)
  const [localTasks, setLocalTasks] = useState<Task[]>([]);
  const [localSort, setLocalSort] = useState<LocalSortOption>('status');
  const [expandedTaskIds, setExpandedTaskIds] = useState<Set<string>>(new Set());
  const [exportMenuOpen, setExportMenuOpen] = useState<string | null>(null);
  const exportMenuRef = useRef<HTMLDivElement>(null);

  // Tab 3 Data (Files)
  const [fileTree, setFileTree] = useState<FileNode[]>([]);
  const [expandAll, setExpandAll] = useState(false); // Control expand all for file tree

  const [loading, setLoading] = useState(false);

  // Close export menu when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (exportMenuRef.current && !exportMenuRef.current.contains(event.target as Node)) {
        setExportMenuOpen(null);
      }
    };
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, [exportMenuRef]);

  // Reset expandAll state when switching to Files tab
  useEffect(() => {
    if (activeTab === LeftTab.FILES) {
      setExpandAll(false);
    }
  }, [activeTab]);

  useEffect(() => {
    setLoading(true);
    const p1 = getTasks('pending').then(setPendingTasks);
    const p2 = getTasks('watched').then(setWatchedTasks);
    const p3 = getLocalTasks().then(setLocalTasks);
    // Pass diffContext branches to getFileTree
    const p4 = getFileTree(diffContext?.base, diffContext?.head).then((data) => {
      console.log('[TaskTree] File tree loaded:', data);
      setFileTree(data);
    });

    Promise.all([p1, p2, p3, p4])
        .catch(console.error)
        .finally(() => setLoading(false));
  }, [repoRefreshKey, diffContext]); // Include diffContext to refresh when branches change

  const toggleSection = (key: keyof typeof sections) => {
    setSections(prev => ({ ...prev, [key]: !prev[key] }));
  };

  const toggleTaskExpansion = (taskId: string) => {
      const newSet = new Set(expandedTaskIds);
      if (newSet.has(taskId)) {
          newSet.delete(taskId);
      } else {
          newSet.add(taskId);
      }
      setExpandedTaskIds(newSet);
  };

  // Handler for file review actions - triggers review modal
  const handleFileReviewAction = (
      taskId: string,
      fileId: string,
      filePath: string,
      type: 'approved' | 'concern' | 'must_change' | 'question'
  ) => {
      // Map must_change to reject for the modal (reject maps back to must_change in backend)
      const modalType = type === 'must_change' ? 'reject' : type;
      onAction(`FileReviewAction:${modalType}:${taskId}:${fileId}:${filePath}`);
  };

  // Handler for marking task as completed
  const handleMarkTaskCompleted = async (taskId: string) => {
      try {
          await markTaskCompleted(taskId);
          // Refresh local tasks to show updated status
          const refreshedTasks = await getLocalTasks();
          setLocalTasks(refreshedTasks);
          // Show success notification
          onAction('任务已标记为完成');
      } catch (error) {
          console.error('[TaskTree] Failed to mark task as completed:', error);
          onAction('标记任务完成失败，请重试');
      }
  };

  // Sort Logic for Local Tasks
  const sortedLocalTasks = useMemo(() => {
      const tasks = [...localTasks];
      tasks.sort((a, b) => {
          switch(localSort) {
              case 'status':
                  return a.status.localeCompare(b.status);
              case 'type':
                  return (a.type || '').localeCompare(b.type || '');
              case 'name':
                  return a.title.localeCompare(b.title);
              case 'path':
                  // Sort by first file path if available, else title
                  const pathA = a.files?.[0]?.path || a.title;
                  const pathB = b.files?.[0]?.path || b.title;
                  return pathA.localeCompare(pathB);
              default:
                  return 0;
          }
      });
      return tasks;
  }, [localTasks, localSort]);

  const getTypeBadgeColor = (type?: string) => {
      switch(type) {
          case 'sql': return 'bg-purple-500/20 text-purple-400 border-purple-500/30';
          case 'security': return 'bg-red-500/20 text-red-400 border-red-500/30';
          case 'code': return 'bg-blue-500/20 text-blue-400 border-blue-500/30';
          default: return 'bg-gray-500/20 text-gray-400 border-gray-500/30';
      }
  };

  // Export handlers
  const handleExportTask = async (taskId: string, format: 'csv' | 'excel') => {
      try {
          // Find the task to get its title for the default filename
          const task = localTasks.find(t => t.id === taskId);
          const taskTitle = task?.title || taskId;
          // Sanitize filename: remove special characters
          const safeTitle = taskTitle.replace(/[<>:"/\\|?*]/g, '_').replace(/\s+/g, '_');
          const extension = format === 'excel' ? 'xlsx' : 'csv';
          const defaultFileName = `${safeTitle}_review.${extension}`;

          // Open save dialog
          const filePath = await save({
              title: '导出任务评审',
              defaultPath: defaultFileName,
              filters: format === 'excel'
                  ? [{ name: 'Excel', extensions: ['xlsx'] }]
                  : [{ name: 'CSV', extensions: ['csv'] }]
          });

          if (!filePath) {
              // User cancelled the dialog
              setExportMenuOpen(null);
              return;
          }

          // Show loading message
          onAction('正在导出数据...');

          // Call the Tauri IPC export function to get CSV data
          const csvData = await exportTaskReview(taskId, format);

          // Write the file to the user-selected location
          await writeTextFile(filePath, csvData);

          setExportMenuOpen(null);
          onAction(`任务已导出到: ${filePath}`);
      } catch (error) {
          console.error('[TaskTree] Export failed:', error);
          onAction('导出失败，请重试');
      }
  };

  return (
    <div id="tour-task-tree" className="h-full bg-editor-sidebar border-r border-editor-line flex flex-col">

      {/* Tabs */}
      <div className="flex border-b border-editor-line bg-editor-bg shrink-0">
        <button
            onClick={() => setActiveTab(LeftTab.GIT)}
            className={`flex-1 py-2 flex justify-center items-center border-b-2 transition-colors ${activeTab === LeftTab.GIT ? 'border-editor-accent text-white' : 'border-transparent text-gray-500 hover:text-gray-300'}`}
            title="Git Tasks (PRs)"
        >
            <GitPullRequest size={16} />
        </button>
        <button
            onClick={() => setActiveTab(LeftTab.LOCAL)}
            className={`flex-1 py-2 flex justify-center items-center border-b-2 transition-colors ${activeTab === LeftTab.LOCAL ? 'border-editor-accent text-white' : 'border-transparent text-gray-500 hover:text-gray-300'}`}
            title="Local Tasks"
        >
            <List size={16} />
        </button>
        <button
            onClick={() => setActiveTab(LeftTab.FILES)}
            className={`flex-1 py-2 flex justify-center items-center border-b-2 transition-colors ${activeTab === LeftTab.FILES ? 'border-editor-accent text-white' : 'border-transparent text-gray-500 hover:text-gray-300'}`}
            title="File Explorer"
        >
            <FolderTree size={16} />
        </button>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto py-2">
        {loading ? (
             <div className="flex flex-col items-center justify-center h-full gap-2 text-gray-500">
                <Loader2 size={24} className="animate-spin text-editor-accent" />
                <span className="text-xs">Loading...</span>
             </div>
        ) : (
        <>
            {/* Panel 1: Git Tasks */}
            {activeTab === LeftTab.GIT && (
                <>
                <div className="mb-4">
                <div
                    className="flex items-center gap-1 px-2 text-xs font-bold text-gray-400 uppercase tracking-wider mb-1 cursor-pointer hover:text-white transition-colors select-none"
                    onClick={() => toggleSection('pending')}
                >
                    {sections.pending ? <ChevronDown size={12} /> : <ChevronRight size={12} />}
                    {t('tasktree.review_pending')} ({pendingTasks.length})
                </div>
                {sections.pending && (
                    <div className="flex flex-col">
                    {pendingTasks.map(task => (
                        <div
                        key={task.id}
                        onClick={() => onSelectTask(task.id)}
                        className={`flex items-center gap-2 px-4 py-1.5 cursor-pointer text-xs transition-colors ${task.id === activeTaskId ? 'bg-editor-selection text-white' : 'text-gray-400 hover:bg-editor-line'}`}
                        >
                        {task.id === activeTaskId ? (
                            <Circle size={10} className="text-white fill-white shrink-0" />
                        ) : task.unreadCount ? (
                            <Circle size={10} className="text-editor-error fill-editor-error shrink-0" />
                        ) : (
                            <Circle size={10} className="text-gray-600 shrink-0" />
                        )}
                        <span className="truncate">{task.title}</span>
                        </div>
                    ))}
                    </div>
                )}
                </div>

                <div className="mb-4">
                <div
                    className="flex items-center gap-1 px-2 text-xs font-bold text-gray-400 uppercase tracking-wider mb-1 cursor-pointer hover:text-white transition-colors select-none"
                    onClick={() => toggleSection('watched')}
                >
                    {sections.watched ? <ChevronDown size={12} /> : <ChevronRight size={12} />}
                    {t('tasktree.watching')} ({watchedTasks.length})
                </div>
                {sections.watched && (
                    <div className="flex flex-col">
                    {watchedTasks.map(task => (
                        <div
                        key={task.id}
                        onClick={() => onSelectTask(task.id)}
                        className={`flex items-center gap-2 px-4 py-1.5 cursor-pointer text-xs transition-colors ${task.id === activeTaskId ? 'bg-editor-selection text-white' : 'text-gray-400 hover:bg-editor-line'}`}
                        >
                        <AlertCircle size={10} className="text-editor-warning shrink-0" />
                        <span className="truncate">{task.title}</span>
                        {task.unreadCount && (
                            <span className="ml-auto text-[10px] bg-editor-error text-white px-1 rounded-full">{task.unreadCount}</span>
                        )}
                        </div>
                    ))}
                    </div>
                )}
                </div>

                <div className="mb-4">
                <div
                    className="flex items-center gap-1 px-2 text-xs font-bold text-gray-400 uppercase tracking-wider mb-1 cursor-pointer hover:text-white transition-colors select-none"
                    onClick={() => toggleSection('history')}
                >
                    {sections.history ? <ChevronDown size={12} /> : <ChevronRight size={12} />}
                    {t('tasktree.history')} (Today 127)
                </div>
                {sections.history && (
                    <div className="px-4 py-2 text-xs text-gray-500 italic">{t('tasktree.no_history')}</div>
                )}
                </div>
                </>
            )}

            {/* Panel 2: Local Tasks (Renamed & Enhanced) */}
            {activeTab === LeftTab.LOCAL && (
                 <div className="flex flex-col h-full">
                     {/* Local Toolbar */}
                     <div className="px-2 py-1 mb-2 flex items-center justify-between border-b border-editor-line/50 bg-editor-bg/30">
                        <span className="text-[10px] font-bold text-gray-400 uppercase tracking-wider">{t('tasktree.tab.local')}</span>
                        <div className="flex items-center gap-1">
                             <button
                                onClick={(e) => {
                                    e.stopPropagation();
                                    console.log('[TaskTree] Sort by status clicked, current sort:', localSort);
                                    setLocalSort('status');
                                }}
                                title={t('tasktree.sort.status')}
                                className={`p-1 rounded hover:bg-editor-line cursor-pointer ${localSort === 'status' ? 'text-editor-accent' : 'text-gray-500'}`}
                             >
                                 <ArrowUpDown size={12} />
                             </button>
                             <button
                                onClick={(e) => {
                                    e.stopPropagation();
                                    console.log('[TaskTree] Sort by type clicked, current sort:', localSort);
                                    setLocalSort('type');
                                }}
                                title={t('tasktree.sort.type')}
                                className={`p-1 rounded hover:bg-editor-line cursor-pointer ${localSort === 'type' ? 'text-editor-accent' : 'text-gray-500'}`}
                             >
                                 <Filter size={12} />
                             </button>
                             <button
                                onClick={(e) => {
                                    e.stopPropagation();
                                    console.log('[TaskTree] Sort by name clicked, current sort:', localSort);
                                    setLocalSort('name');
                                }}
                                title={t('tasktree.sort.name')}
                                className={`p-1 rounded hover:bg-editor-line cursor-pointer ${localSort === 'name' ? 'text-editor-accent' : 'text-gray-500'}`}
                             >
                                 <FileCode size={12} />
                             </button>
                             <button
                                onClick={(e) => {
                                    e.stopPropagation();
                                    console.log('[TaskTree] Sort by path clicked, current sort:', localSort);
                                    setLocalSort('path');
                                }}
                                title={t('tasktree.sort.path')}
                                className={`p-1 rounded hover:bg-editor-line cursor-pointer ${localSort === 'path' ? 'text-editor-accent' : 'text-gray-500'}`}
                             >
                                 <FolderTree size={12} />
                             </button>
                        </div>
                     </div>

                     {/* Task List */}
                     <div className="flex-1 overflow-y-auto">
                     {sortedLocalTasks.length === 0 ? (
                         <div className="px-4 py-2 text-xs text-gray-500 italic">No local tasks. Create one above.</div>
                     ) : (
                         sortedLocalTasks.map(task => {
                            const isExpanded = expandedTaskIds.has(task.id);
                            return (
                                <div key={task.id} className="flex flex-col">
                                    {/* Task Header */}
                                    <div
                                        onClick={() => {
                                            onSelectTask(task.id);
                                            toggleTaskExpansion(task.id);
                                        }}
                                        className={`flex items-center gap-2 px-3 py-2 cursor-pointer text-xs transition-colors border-l-2 group
                                            ${task.id === activeTaskId ? 'bg-editor-selection/50 border-editor-accent' : 'border-transparent hover:bg-editor-line'}
                                        `}
                                    >
                                        <div className="text-gray-500 hover:text-white transition-colors" onClick={(e) => { e.stopPropagation(); toggleTaskExpansion(task.id); }}>
                                            {isExpanded ? <ChevronDown size={12} /> : <ChevronRight size={12} />}
                                        </div>

                                        {task.status === 'completed' ? (
                                            <CheckCircle2 size={12} className="text-editor-success shrink-0" />
                                        ) : task.status === 'active' ? (
                                            <Circle size={12} className="text-editor-accent fill-editor-accent/20 shrink-0" />
                                        ) : (
                                            <Circle size={12} className="text-gray-600 shrink-0" />
                                        )}

                                        <div className="flex-1 min-w-0 flex items-center gap-2">
                                            <div className="flex flex-col gap-0.5">
                                                <span className={`truncate font-medium ${task.id === activeTaskId ? 'text-white' : 'text-gray-400 group-hover:text-gray-300'}`}>{task.title}</span>
                                                {task.type && (
                                                    <span className={`text-[9px] px-1.5 py-0 rounded border w-fit uppercase font-bold ${getTypeBadgeColor(task.type)}`}>
                                                        {task.type}
                                                    </span>
                                                )}
                                            </div>
                                            {/* Action buttons - show on hover */}
                                            <div className="ml-auto flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-all">
                                                {/* Task completion button - shows when all files are reviewed */}
                                                {task.files && task.files.length > 0 && task.files.every(f => f.reviewStatus && f.reviewStatus !== 'pending') && task.status !== 'completed' && (
                                                    <button
                                                        onClick={(e) => {
                                                            e.stopPropagation();
                                                            handleMarkTaskCompleted(task.id);
                                                        }}
                                                        className="p-1 rounded hover:bg-editor-success/20 text-gray-500 hover:text-editor-success transition-all"
                                                        title="标记任务为完成"
                                                    >
                                                        <CheckCircle2 size={14} />
                                                    </button>
                                                )}
                                                {/* Export button */}
                                                <div className="relative" ref={exportMenuOpen === task.id ? exportMenuRef : null}>
                                                    <button
                                                        onClick={(e) => {
                                                            e.stopPropagation();
                                                            setExportMenuOpen(exportMenuOpen === task.id ? null : task.id);
                                                        }}
                                                        className="p-1 rounded hover:bg-editor-accent/20 text-gray-500 hover:text-editor-accent transition-all"
                                                        title="导出任务评审"
                                                    >
                                                        <Download size={14} />
                                                    </button>
                                                    {/* Export dropdown menu */}
                                                    {exportMenuOpen === task.id && (
                                                        <div
                                                            onClick={(e) => e.stopPropagation()}
                                                            className="absolute right-0 top-full mt-1 bg-editor-bg border border-editor-line rounded shadow-lg py-1 min-w-[120px] z-50"
                                                        >
                                                            <button
                                                                onClick={() => handleExportTask(task.id, 'csv')}
                                                                className="w-full px-3 py-1.5 text-left text-xs text-gray-300 hover:bg-editor-line hover:text-white transition-colors flex items-center gap-2"
                                                            >
                                                                <File size={12} />
                                                                导出为CSV
                                                            </button>
                                                            <button
                                                                onClick={() => handleExportTask(task.id, 'excel')}
                                                                className="w-full px-3 py-1.5 text-left text-xs text-gray-300 hover:bg-editor-line hover:text-white transition-colors flex items-center gap-2"
                                                            >
                                                                <File size={12} />
                                                                导出为Excel
                                                            </button>
                                                        </div>
                                                    )}
                                                </div>
                                            </div>
                                        </div>
                                    </div>

                                    {/* Task Files (Expanded) */}
                                    {isExpanded && task.files && (
                                        <div className="flex flex-col bg-black/10 pb-1">
                                            {task.files.map(file => {
                                                const reviewStatus = file.reviewStatus || 'pending';
                                                const getReviewStatusIcon = (status: FileReviewStatus) => {
                                                    switch(status) {
                                                        case 'approved': return <CheckCircle size={12} className="text-editor-success" />;
                                                        case 'concern': return <AlertTriangle size={12} className="text-editor-warning" />;
                                                        case 'must_change': return <XCircle size={12} className="text-editor-error" />;
                                                        case 'question': return <MessageSquare size={12} className="text-editor-accent" />;
                                                        default: return <Circle size={12} className="text-gray-500" />;
                                                    }
                                                };
                                                const getReviewStatusColor = (status: FileReviewStatus) => {
                                                    switch(status) {
                                                        case 'approved': return 'bg-editor-success/20 text-editor-success';
                                                        case 'concern': return 'bg-editor-warning/20 text-editor-warning';
                                                        case 'must_change': return 'bg-editor-error/20 text-editor-error';
                                                        case 'question': return 'bg-editor-accent/20 text-editor-accent';
                                                        default: return 'bg-gray-500/10 text-gray-500';
                                                    }
                                                };

                                                return (
                                                    <div
                                                        key={file.id}
                                                        onClick={() => onAction(`Opening Diff: ${file.path}`)}
                                                        className="flex items-center gap-2 pl-9 pr-2 py-1 cursor-pointer hover:bg-editor-line/30 group/file relative"
                                                    >
                                                        <File size={12} className="text-gray-500 group-hover/file:text-editor-accent" />
                                                        <span className={`text-xs truncate text-gray-500 group-hover/file:text-gray-300 ${file.status === 'modified' ? 'text-editor-warning' : file.status === 'added' ? 'text-editor-success' : ''}`}>
                                                            {file.name}
                                                        </span>
                                                        <span className="ml-auto flex items-center gap-2">
                                                            {/* File status badge */}
                                                            <span className={`px-1.5 py-0 rounded text-[9px] font-medium border ${getReviewStatusColor(reviewStatus)}`}>
                                                                {file.status.substring(0,1).toUpperCase()}
                                                            </span>
                                                            {/* Review status icon */}
                                                            {getReviewStatusIcon(reviewStatus)}
                                                        </span>

                                                        {/* Hover action buttons */}
                                                        <div className="absolute right-2 top-1/2 -translate-y-1/2 hidden group-hover/file:flex items-center gap-1 bg-editor-bg/95 border border-editor-line rounded shadow-lg px-1.5 py-0.5 z-10">
                                                            <button
                                                                onClick={(e) => {
                                                                    e.stopPropagation();
                                                                    handleFileReviewAction(task.id, file.id, file.path, 'approved');
                                                                }}
                                                                className="p-1 rounded hover:bg-editor-success/20 text-gray-400 hover:text-editor-success transition-colors"
                                                                title="文件通过审核"
                                                            >
                                                                <CheckCircle size={12} />
                                                            </button>
                                                            <button
                                                                onClick={(e) => {
                                                                    e.stopPropagation();
                                                                    handleFileReviewAction(task.id, file.id, file.path, 'concern');
                                                                }}
                                                                className="p-1 rounded hover:bg-editor-warning/20 text-gray-400 hover:text-editor-warning transition-colors"
                                                                title="标记关注"
                                                            >
                                                                <AlertTriangle size={12} />
                                                            </button>
                                                            <button
                                                                onClick={(e) => {
                                                                    e.stopPropagation();
                                                                    handleFileReviewAction(task.id, file.id, file.path, 'must_change');
                                                                }}
                                                                className="p-1 rounded hover:bg-editor-error/20 text-gray-400 hover:text-editor-error transition-colors"
                                                                title="必须改"
                                                            >
                                                                <XCircle size={12} />
                                                            </button>
                                                            <button
                                                                onClick={(e) => {
                                                                    e.stopPropagation();
                                                                    handleFileReviewAction(task.id, file.id, file.path, 'question');
                                                                }}
                                                                className="p-1 rounded hover:bg-editor-accent/20 text-gray-400 hover:text-editor-accent transition-colors"
                                                                title="提问作者"
                                                            >
                                                                <MessageSquare size={12} />
                                                            </button>
                                                        </div>
                                                    </div>
                                                );
                                            })}
                                            {task.files.length === 0 && <div className="pl-9 text-[10px] text-gray-600 italic py-1">No files attached</div>}
                                        </div>
                                    )}
                                </div>
                            );
                         })
                     )}
                     </div>
                 </div>
            )}

            {/* Panel 3: File Tree */}
            {activeTab === LeftTab.FILES && (
                 <div className="flex flex-col">
                    <div className="px-2 text-xs font-bold text-gray-400 uppercase tracking-wider mb-2 mt-1 flex justify-between items-center">
                        <span>Explorer</span>
                        <div className="flex items-center gap-2">
                            <button
                                onClick={() => setExpandAll(!expandAll)}
                                className="text-[10px] px-2 py-0.5 rounded bg-editor-line/30 hover:bg-editor-line text-gray-400 hover:text-white transition-colors border border-editor-line/50"
                                title={expandAll ? "Collapse all" : "Expand all"}
                            >
                                {expandAll ? "Collapse All" : "Expand All"}
                            </button>
                            <span className="text-[10px] font-normal opacity-50">feature/retry</span>
                        </div>
                    </div>
                    {fileTree.length === 0 ? (
                        <div className="px-4 py-2 text-xs text-gray-500 italic">No files found</div>
                    ) : (
                        fileTree.map(node => (
                            <FileTreeItem key={node.id} node={node} onSelect={(path) => {
                                console.log('[TaskTree] File selected:', path);
                                onSelectFile?.(path);
                                onAction(`File selected: ${path}`);
                            }} expandAll={expandAll} />
                        ))
                    )}
                 </div>
            )}
        </>
        )}
      </div>
    </div>
  );
};

export default TaskTree;
