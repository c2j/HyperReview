import React, { useEffect, useState } from 'react';
import { useTaskStore } from '@store/taskStore';
import { Plus, FolderOpen, CheckCircle2, Clock, Filter, X } from 'lucide-react';
import type { TaskSummary, TaskStatus } from '@types/task';

interface TaskPanelProps {
  onTaskSelect: (taskId: string) => void;
  onCreateTask: () => void;
}

const TaskPanel: React.FC<TaskPanelProps> = ({ onTaskSelect, onCreateTask }) => {
  const { tasks, fetchTasks, isLoading, error } = useTaskStore();
  const [filter, setFilter] = useState<TaskStatus | 'all'>('all');

  useEffect(() => {
    fetchTasks();
  }, [fetchTasks]);

  const filteredTasks = tasks.filter((task) => {
    if (filter === 'all') return true;
    return task.status === filter;
  });

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'completed':
        return <CheckCircle2 size={16} className="text-green-500" />;
      case 'in_progress':
        return <Clock size={16} className="text-orange-500" />;
      default:
        return <FolderOpen size={16} className="text-gray-500" />;
    }
  };

  const clearFilter = () => setFilter('all');

  if (isLoading) {
    return (
      <div className="flex items-center justify-center p-4">
        <div className="text-editor-fg text-sm">Loading tasks...</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex items-center justify-center p-4">
        <div className="text-red-400 text-sm">Error: {error}</div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full">
      <div className="p-3 border-b border-editor-line flex items-center justify-between">
        <div className="flex items-center gap-2">
          <span className="text-orange-500 font-semibold text-sm">Local Tasks</span>
          <span className="text-editor-fg text-xs bg-orange-500/20 px-2 py-0.5 rounded">
            {filteredTasks.length}
          </span>
        </div>
        <button
          onClick={onCreateTask}
          className="p-1.5 rounded hover:bg-orange-500/20 text-orange-500 transition-colors"
          title="Create New Task"
        >
          <Plus size={18} />
        </button>
      </div>

      <div className="p-2 border-b border-editor-line flex gap-2">
        <Filter size={14} className="text-editor-fg/70 shrink-0" />
        <div className="flex gap-1 flex-wrap">
          {(['all', 'in_progress', 'completed', 'archived'] as const).map((status) => (
            <button
              key={status}
              onClick={() => setFilter(status)}
              className={`
                px-2 py-1 text-xs rounded transition-colors
                ${
                  filter === status
                    ? 'bg-orange-500/20 text-orange-500'
                    : 'text-editor-fg/70 hover:bg-editor-line hover:text-editor-fg'
                }
              `}
            >
              {status === 'all' ? 'All' : status.replace('_', ' ')}
              {filter === status && status !== 'all' && <X size={10} className="ml-1 inline" />}
            </button>
          ))}
        </div>
      </div>

      <div className="flex-1 overflow-y-auto">
        {filteredTasks.length === 0 ? (
          <div className="flex flex-col items-center justify-center p-8 text-center">
            <FolderOpen size={48} className="text-gray-600 mb-4" />
            <p className="text-editor-fg text-sm mb-2">No tasks found</p>
            <p className="text-editor-fg/60 text-xs mb-4">
              {filter !== 'all'
                ? `No ${filter.replace('_', ' ')} tasks`
                : 'Create a task from a file list to review code offline'}
            </p>
            {filter !== 'all' && (
              <button onClick={clearFilter} className="text-xs text-orange-500 hover:underline">
                Clear filter
              </button>
            )}
          </div>
        ) : (
          <div className="p-2 space-y-1">
            {filteredTasks.map((task) => (
              <div
                key={task.id}
                onClick={() => onTaskSelect(task.id)}
                className="p-3 rounded border border-editor-line hover:border-orange-500/50 hover:bg-orange-500/10 cursor-pointer transition-all"
              >
                <div className="flex items-start justify-between gap-2 mb-2">
                  <h3 className="text-sm font-medium text-editor-fg line-clamp-2 flex-1">
                    {task.name}
                  </h3>
                  <div className="flex items-center gap-1 shrink-0">
                    {getStatusIcon(task.status)}
                  </div>
                </div>

                <div className="flex items-center gap-2">
                  <div className="flex-1 h-1.5 bg-editor-line rounded-full overflow-hidden">
                    <div
                      className="h-full bg-orange-500 transition-all duration-300"
                      style={{ width: `${task.progress}%` }}
                    />
                  </div>
                  <span className="text-xs text-editor-fg/70 whitespace-nowrap">
                    {Math.round(task.progress)}%
                  </span>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};

export default TaskPanel;
