
import React, { useState, useEffect } from 'react';
import { ChevronDown, ChevronRight, FolderTree, FileCode, Folder, GitPullRequest, Search } from 'lucide-react';
import { useTranslation } from '../i18n';
import { getFileTree, getTasks } from '../api/client';
import type { Task, FileNode } from '../api/types';

interface LocalTaskTreeProps {
  activeTaskId: string;
  onSelectTask: (id: string) => void;
  onAction: (msg: string) => void;
}

const LocalTaskTree: React.FC<LocalTaskTreeProps> = ({ activeTaskId, onSelectTask, onAction }) => {
  const { t } = useTranslation();
  const [fileTree, setFileTree] = useState<FileNode[]>([]);
  const [tasks, setTasks] = useState<Task[]>([]);

  useEffect(() => {
    getFileTree().then(setFileTree);
    getTasks('pending').then(setTasks);
  }, []);

  return (
    <div className="h-full bg-editor-sidebar border-r border-editor-line flex flex-col">
      <div className="p-3 border-b border-editor-line">
          <div className="relative">
              <Search className="absolute left-2 top-1/2 -translate-y-1/2 text-gray-600" size={12} />
              <input placeholder="Search files..." className="w-full bg-editor-bg border border-editor-line rounded pl-7 pr-2 py-1 text-[11px] focus:outline-none focus:border-editor-accent" />
          </div>
      </div>

      <div className="flex-1 overflow-y-auto py-2 custom-scrollbar">
          <div className="mb-4">
              <div className="px-3 text-[10px] font-bold text-gray-500 uppercase tracking-widest mb-2 flex items-center justify-between">
                <span>{t('tasktree.review_pending')}</span>
                <GitPullRequest size={12} />
              </div>
              {tasks.map(task => (
                  <div key={task.id} onClick={() => onSelectTask(task.id)} className={`px-4 py-1.5 cursor-pointer text-xs font-medium truncate border-l-2 transition-all ${activeTaskId === task.id ? 'bg-editor-accent/10 border-editor-accent text-white' : 'border-transparent text-gray-400 hover:bg-editor-line'}`}>
                      {task.title}
                  </div>
              ))}
          </div>

          <div>
              <div className="px-3 text-[10px] font-bold text-gray-500 uppercase tracking-widest mb-2">{t('tasktree.workspace_explorer')}</div>
              <div className="px-2">
                  {fileTree.map(node => (
                      <div key={node.id} className="flex items-center gap-2 py-1 px-2 hover:bg-editor-line cursor-pointer rounded">
                          <Folder size={14} className="text-editor-accent shrink-0" />
                          <span className="text-xs text-gray-300 font-mono truncate">{node.name}</span>
                      </div>
                  ))}
              </div>
          </div>
      </div>
    </div>
  );
};

export default LocalTaskTree;
