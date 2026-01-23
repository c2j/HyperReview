
import React, { useState, useEffect } from 'react';
import { Folder, GitPullRequest, Search } from 'lucide-react';
import { useTranslation } from '../../i18n';
import { useApiClient } from '../../api/client';
import type { Task, FileNode } from '../../api/types';

interface LocalTaskTreeProps {
  activeTaskId: string;
  onSelectTask: (id: string) => void;
  onAction: (msg: string) => void;
  onSelectFile: (file: string | null) => void;
  selectedFile: string | null;
}

const LocalTaskTree: React.FC<LocalTaskTreeProps> = ({ activeTaskId, onSelectTask, onAction, onSelectFile, selectedFile }) => {
  const { t } = useTranslation();
  const apiClient = useApiClient();
  const [fileTree, setFileTree] = useState<FileNode[]>([]);
  const [tasks, setTasks] = useState<Task[]>([]);

  useEffect(() => {
    apiClient.getFileTree().then(setFileTree);
    apiClient.getTasks('pending').then(setTasks);
  }, [apiClient]);

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
                      <div
                        key={node.id}
                        onClick={() => {
                          if (node.type === 'file') {
                            onSelectFile(node.path);
                            onAction(`File selected: ${node.path}`);
                          }
                        }}
                        className={`flex items-center gap-2 py-1 px-2 rounded cursor-pointer transition-all ${
                          selectedFile === node.path ? 'bg-editor-accent/20 border-editor-accent text-white' : 'hover:bg-editor-line border-transparent text-gray-300'
                        }`}
                      >
                          <Folder size={14} className="text-editor-accent shrink-0" />
                          <span className="text-xs font-mono truncate">{node.name}</span>
                      </div>
                  ))}
              </div>
          </div>
      </div>
    </div>
  );
};

export default LocalTaskTree;
