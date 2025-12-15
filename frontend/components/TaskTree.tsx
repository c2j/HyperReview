import React, { useState, useEffect } from 'react';
import { ChevronDown, ChevronRight, Circle, AlertCircle, Plus } from 'lucide-react';
import { useTranslation } from '../i18n';
import { useApiClient } from '../api/client';
import type { Task } from '../api/types';

interface TaskTreeProps {
  activeTaskId: string;
  onSelectTask: (id: string) => void;
  onAction: (msg: string) => void;
}

const TaskTree: React.FC<TaskTreeProps> = ({ activeTaskId, onSelectTask, onAction }) => {
  const { t } = useTranslation();
  const { getTasks } = useApiClient();
  const [sections, setSections] = useState({
    pending: true,
    watched: true,
    history: false
  });

  const [pendingTasks, setPendingTasks] = useState<Task[]>([]);
  const [watchedTasks, setWatchedTasks] = useState<Task[]>([]);

  useEffect(() => {
    getTasks().then((tasks) => {
      setPendingTasks(tasks.filter(t => t.status === 'Pending'));
      setWatchedTasks(tasks.filter(t => t.status === 'Active'));
    }).catch(console.error);
  }, []);

  const toggleSection = (key: keyof typeof sections) => {
    setSections(prev => ({ ...prev, [key]: !prev[key] }));
  };

  return (
    <div id="tour-task-tree" className="h-full bg-editor-sidebar border-r border-editor-line flex flex-col">
      <div className="flex-1 overflow-y-auto py-2">
        {/* Section 1 */}
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
                     <Circle size={10} className="text-white fill-white" />
                  ) : (
                     <Circle size={10} className="text-editor-error fill-editor-error" />
                  )}
                  <span className="truncate">{task.title}</span>
                </div>
              ))}
            </div>
          )}
        </div>

        {/* Section 2 */}
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
                   <AlertCircle size={10} className="text-editor-warning" />
                   <span className="truncate">{task.title}</span>
                </div>
              ))}
            </div>
           )}
        </div>

        {/* Section 3 */}
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
      </div>

      <div className="p-2 border-t border-editor-line">
         <button 
           onClick={() => onAction("Creating Local Task...")}
           className="w-full flex items-center justify-center gap-2 bg-editor-line hover:bg-gray-700 text-xs text-gray-300 py-2 rounded transition-colors border border-gray-700 active:bg-gray-600">
             <Plus size={14} /> {t('tasktree.create_task')}
         </button>
      </div>
    </div>
  );
};

export default TaskTree;