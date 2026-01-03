import React from 'react';
import { FileText, CheckCircle, Circle, AlertTriangle, HelpCircle } from 'lucide-react';
import type { TaskItem } from '../types/task';

interface TaskItemComponentProps {
  item: TaskItem;
  isActive: boolean;
  onClick: () => void;
}

const TaskItemComponent: React.FC<TaskItemComponentProps> = ({ item, isActive, onClick }) => {
  const getSeverityIcon = () => {
    switch (item.severity) {
      case 'error':
        return <AlertTriangle size={14} className="text-red-500 shrink-0" />;
      case 'warning':
        return <AlertTriangle size={14} className="text-yellow-500 shrink-0" />;
      case 'question':
        return <HelpCircle size={14} className="text-blue-500 shrink-0" />;
      case 'ok':
        return <CheckCircle size={14} className="text-green-500 shrink-0" />;
      default:
        return null;
    }
  };

  const getLineRangeDisplay = () => {
    if (!item.line_range) return '';
    const { start, end } = item.line_range;
    if (start !== undefined && end !== undefined) {
      return `:${start}-${end}`;
    } else if (start !== undefined) {
      return `:${start}+`;
    }
    return '';
  };

  return (
    <div
      onClick={onClick}
      className={`
        p-3 rounded border transition-all cursor-pointer
        ${
          isActive
            ? 'border-orange-500 bg-orange-500/10'
            : 'border-editor-line hover:border-editor-line/50 bg-editor-input/30'
        }
        ${item.reviewed ? 'opacity-60' : ''}
      `}
    >
      <div className="flex items-start gap-2">
        <div className="mt-0.5 shrink-0">
          {item.reviewed ? (
            <CheckCircle size={16} className="text-green-500" />
          ) : (
            <Circle size={16} className="text-gray-500" />
          )}
        </div>

        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2 mb-1">
            <FileText size={14} className="text-gray-500 shrink-0" />
            <span className="text-sm text-editor-fg font-mono truncate">
              {item.file}
              {getLineRangeDisplay()}
            </span>
            {getSeverityIcon()}
          </div>

          {item.preset_comment && (
            <p className="text-xs text-editor-fg/70 line-clamp-2 mt-1">{item.preset_comment}</p>
          )}

          {item.tags && item.tags.length > 0 && (
            <div className="flex flex-wrap gap-1 mt-2">
              {item.tags.map((tag: string, idx: number) => (
                <span
                  key={idx}
                  className="text-xs px-2 py-0.5 rounded bg-editor-line text-editor-fg/70"
                >
                  {tag}
                </span>
              ))}
            </div>
          )}

          {item.comments && item.comments.length > 0 && (
            <div className="text-xs text-orange-500/80 mt-2">
              {item.comments.length} comment{item.comments.length !== 1 ? 's' : ''}
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default TaskItemComponent;
