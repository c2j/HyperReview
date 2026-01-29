
import React, { useState, useEffect } from 'react';
import {
  ChevronDown,
  ChevronRight,
  Globe,
  RefreshCw,
  FileCode,
  Folder,
  Loader2,
  List,
  LayoutGrid,
  MoreHorizontal,
  CheckCircle2
} from 'lucide-react';
import { useTranslation } from '../../i18n';
import type { GerritChange, GerritFile } from '../../api/types/gerrit';
import type { FileNode } from '../../api/types';
import { useGerritPolling } from '../../hooks/useGerritPolling';
import { useGerritChangesStore } from '../../store/gerritStore';

interface RemoteTaskTreeProps {
  activeTaskId: string;
  onSelectTask: (id: string) => void;
  onAction: (msg: string) => void;
}

type FileViewMode = 'flat' | 'tree';

const FileTreeItem: React.FC<{ 
  node: FileNode; 
  depth?: number; 
  onSelect: (path: string) => void;
}> = ({ node, depth = 0, onSelect }) => {
  const [expanded, setExpanded] = useState(true);
  const hasChildren = node.children && node.children.length > 0;

  return (
    <div>
      <div 
        className="flex items-center gap-1.5 py-1 hover:bg-editor-line/40 cursor-pointer select-none group"
        style={{ paddingLeft: `${depth * 12 + 8}px` }}
        onClick={() => {
          if (hasChildren) setExpanded(!expanded);
          else onSelect(node.path);
        }}
      >
        <span className="w-3 flex shrink-0 justify-center">
          {hasChildren ? (
            expanded ? <ChevronDown size={10} className="text-gray-500" /> : <ChevronRight size={10} className="text-gray-500" />
          ) : null}
        </span>
        {node.type === 'folder' ? (
          <Folder size={12} className="text-purple-400/80 shrink-0" />
        ) : (
          <FileCode size={12} className="text-gray-500 shrink-0 group-hover:text-purple-300" />
        )}
        <span className={`text-[11px] truncate flex-1 font-mono ${node.type === 'folder' ? 'text-gray-400' : 'text-gray-300'}`}>
          {node.name}
        </span>
      </div>
      {hasChildren && expanded && (
        <div>
          {node.children!.map(child => (
            <FileTreeItem key={child.id} node={child} depth={depth + 1} onSelect={onSelect} />
          ))}
        </div>
      )}
    </div>
  );
};

const RemoteTaskTree: React.FC<RemoteTaskTreeProps> = ({ activeTaskId, onSelectTask, onAction }) => {
  const { t } = useTranslation();
  const { fetchChanges, gerritChanges, loading: storeLoading } = useGerritChangesStore();
  const [expandedChanges, setExpandedChanges] = useState<Set<string>>(new Set());
  const [fileViewModes, setFileViewModes] = useState<Record<string, FileViewMode>>({});

  // 从共享存储获取所有Gerrit变更（只在初始加载和刷新时）
  const loadGerritChanges = async () => {
    console.log('[RemoteTaskTree] Loading Gerrit changes from shared store');
    await fetchChanges(0, 25);
  };

  useEffect(() => {
    console.log('[RemoteTaskTree] Component mounted, fetching initial changes');
    loadGerritChanges();

    // Cleanup function to prevent memory leaks
    return () => {
      console.log('[RemoteTaskTree] Component unmounted');
    };
  }, []); // Empty dependency array - should only run once

  const { isPolling, lastPolled, manualPoll } = useGerritPolling({
    enabled: true,
    interval: 30000,
    onPoll: async () => {
      await new Promise(resolve => setTimeout(resolve, 1000));
      await fetchChanges(0, 25);
    },
    onError: (error) => console.error('Polling error:', error)
  });

  const toggleChange = (id: string) => {
    setExpandedChanges(prev => {
      const next = new Set(prev);
      if (next.has(id)) next.delete(id);
      else next.add(id);
      return next;
    });
  };

  const setViewMode = (id: string, mode: FileViewMode) => {
    setFileViewModes(prev => ({ ...prev, [id]: mode }));
  };

  const buildFileTree = (files: GerritFile[]): FileNode[] => {
    const root: FileNode[] = [];
    const map: Record<string, FileNode> = {};

    files.forEach(file => {
      const parts = file.filePath.split('/');
      let currentPath = '';
      let parentNodes = root;

      parts.forEach((part: string, index: number) => {
        currentPath = currentPath ? `${currentPath}/${part}` : part;
        const isFile = index === parts.length - 1;

        if (!map[currentPath]) {
          const newNode: FileNode = {
            id: currentPath,
            name: part,
            path: currentPath,
            type: isFile ? 'file' : 'folder',
            status: isFile ? (file.status === 'unreviewed' ? 'modified' : 'none') : 'none',
            children: isFile ? undefined : [],
            exists: true
          };
          map[currentPath] = newNode;
          parentNodes.push(newNode);
        }
        parentNodes = (map[currentPath] as FileNode).children || [];
      });
    });

    return root;
  };

  return (
    <div className="h-full bg-editor-sidebar border-r border-editor-line flex flex-col">
      <div className="px-4 py-3 border-b border-editor-line flex items-center justify-between bg-editor-bg/50">
        <div className="flex items-center gap-2">
          <Globe size={14} className="text-purple-400" />
          <span className="text-[10px] font-black text-gray-500 uppercase tracking-widest">
            {t('tasktree.gerrit_changes')}
          </span>
          {lastPolled && (
            <span className="text-[9px] text-gray-600">
              Updated: {lastPolled.toLocaleTimeString()}
            </span>
          )}
        </div>
        <button
          onClick={manualPoll}
          disabled={storeLoading || isPolling}
          className={`p-1 rounded hover:bg-editor-line text-gray-500 transition-all ${(storeLoading || isPolling) ? 'animate-spin text-purple-400' : ''}`}
          title={t('tasktree.refresh_changes')}
        >
          <RefreshCw size={14} />
        </button>
      </div>

      <div className="flex-1 overflow-y-auto custom-scrollbar">
         {storeLoading && gerritChanges.length === 0 ? (
            <div className="flex flex-col items-center justify-center py-12 gap-3 text-gray-600">
              <Loader2 size={24} className="animate-spin text-purple-500" />
              <span className="text-xs">{t('tasktree.connecting_gerrit')}</span>
            </div>
         ) : (
           <div className="divide-y divide-editor-line/30">
             {gerritChanges.map((change: GerritChange) => {
               const isExpanded = expandedChanges.has(change.id);
               const isActive = activeTaskId === `gerrit-${change.id}`;
               const viewMode = fileViewModes[change.id] || 'tree';
               const fileTree = isExpanded ? buildFileTree(change.files || []) : [];

               return (
                 <div key={change.id} className="flex flex-col">
                   {/* Change Header */}
                   <div
                     onClick={() => {
                       onSelectTask(`gerrit-${change.id}`);
                       toggleChange(change.id);
                     }}
                     className={`px-4 py-3 cursor-pointer transition-all border-l-2 relative group ${isActive ? 'bg-purple-900/10 border-purple-500' : 'hover:bg-editor-line border-transparent'}`}
                   >
                     <div className="flex items-start gap-2 mb-1.5">
                       <div className="mt-0.5">
                         {isExpanded ? <ChevronDown size={14} className="text-gray-400" /> : <ChevronRight size={14} className="text-gray-500" />}
                       </div>
                       <div className="flex-1 min-w-0">
                          <div className={`text-xs font-bold truncate leading-tight ${isActive ? 'text-white' : 'text-gray-300'}`}>
                            <span className="text-purple-400 mr-1">#{change.id}</span>
                            {change.subject}
                          </div>
                          <div className="flex items-center gap-2 mt-1 text-[9px] font-mono text-gray-500">
                            <span className="bg-purple-600/20 text-purple-300 px-1 rounded font-bold">PS{change.currentPatchSetNum}</span>
                            <span className="truncate">{change.owner?.name || t('tasktree.unknown')}</span>
                           <span className="ml-auto opacity-60">{change.updated}</span>
                         </div>
                       </div>
                     </div>

                     {/* Tiny Progress Bar */}
                     <div className="h-0.5 w-full bg-gray-800/50 rounded-full overflow-hidden mt-1">
                       <div className="h-full bg-purple-500/60" style={{ width: `${(change.reviewedFiles/change.totalFiles)*100}%` }} />
                     </div>
                   </div>

                   {/* File List (when expanded) */}
                   {isExpanded && (
                     <div className="bg-black/10 border-b border-editor-line/20 pb-2">
                        <div className="flex items-center justify-between px-4 py-1.5 border-b border-editor-line/10 mb-1">
                          <span className="text-[9px] text-gray-500 font-bold uppercase tracking-tighter">
                            {change.totalFiles} {t('tasktree.files')}
                          </span>
                          <div className="flex items-center gap-2">
                            <button
                              onClick={(e) => { e.stopPropagation(); setViewMode(change.id, 'flat'); }}
                              className={`p-1 rounded transition-colors ${viewMode === 'flat' ? 'bg-purple-500/20 text-purple-400' : 'text-gray-600 hover:text-gray-400'}`}
                              title={t('tasktree.flat_view')}
                            >
                             <List size={10} />
                           </button>
                            <button
                              onClick={(e) => { e.stopPropagation(); setViewMode(change.id, 'tree'); }}
                              className={`p-1 rounded transition-colors ${viewMode === 'tree' ? 'bg-purple-500/20 text-purple-400' : 'text-gray-600 hover:text-gray-400'}`}
                              title={t('tasktree.tree_view')}
                            >
                             <LayoutGrid size={10} />
                           </button>
                         </div>
                       </div>

                       <div className="max-h-[300px] overflow-y-auto custom-scrollbar">
                         {viewMode === 'flat' ? (
                           <div className="space-y-0.5">
                             {change.files?.map((file: GerritFile) => (
                               <div
                                 key={file.filePath}
                                 onClick={() => onAction(`File selected: ${file.filePath}`)}
                                 className="flex items-center gap-2 pl-8 pr-4 py-1 hover:bg-editor-line/40 cursor-pointer group"
                               >
                                 <FileCode size={11} className="text-gray-600 group-hover:text-purple-300" />
                                 <span className="text-[11px] font-mono text-gray-400 group-hover:text-gray-200 truncate flex-1">
                                   {file.filePath}
                                 </span>
                                 {file.lastReviewed && <CheckCircle2 size={10} className="text-editor-success opacity-50" />}
                               </div>
                             ))}
                           </div>
                         ) : (
                           <div className="pl-4 pr-2">
                             {fileTree.map((node: FileNode) => (
                               <FileTreeItem
                                 key={node.id}
                                 node={node}
                                 onSelect={(p) => onAction(`File selected: ${p}`)}
                               />
                             ))}
                           </div>
                         )}
                       </div>
                     </div>
                   )}
                 </div>
               );
             })}

             {/* Load More Trigger */}
             <button
               onClick={() => fetchChanges()}
               disabled={storeLoading}
               className="w-full py-4 text-[10px] font-bold text-gray-500 hover:text-purple-400 hover:bg-editor-line/30 transition-all flex items-center justify-center gap-2 group"
             >
               {storeLoading ? (
                 <Loader2 size={12} className="animate-spin" />
               ) : (
                 <>
                   <MoreHorizontal size={14} className="group-hover:scale-110 transition-transform" />
                   <span>{t('tasktree.load_more')}</span>
                 </>
               )}
             </button>
           </div>
         )}
       </div>
     </div>
   );
 };

export default RemoteTaskTree;