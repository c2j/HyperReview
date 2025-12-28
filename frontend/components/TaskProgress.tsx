import React from 'react';
import { CheckCircle2, Clock, Archive } from 'lucide-react';

interface TaskProgressProps {
  completed: number;
  total: number;
  status: 'in_progress' | 'completed' | 'archived';
  size?: 'sm' | 'md' | 'lg';
}

const TaskProgress: React.FC<TaskProgressProps> = ({ completed, total, status, size = 'md' }) => {
  const percentage = total > 0 ? (completed / total) * 100 : 0;

  const sizeClasses = {
    sm: 'h-1',
    md: 'h-2',
    lg: 'h-3',
  };

  const getStatusIcon = () => {
    switch (status) {
      case 'completed':
        return <CheckCircle2 size={16} className="text-green-500" />;
      case 'archived':
        return <Archive size={16} className="text-gray-500" />;
      default:
        return <Clock size={16} className="text-orange-500" />;
    }
  };

  const getStatusColor = () => {
    switch (status) {
      case 'completed':
        return 'bg-green-500';
      case 'archived':
        return 'bg-gray-500';
      default:
        return 'bg-orange-500';
    }
  };

  return (
    <div className="flex items-center gap-2">
      {getStatusIcon()}
      <div className="flex-1">
        <div className={`w-full ${sizeClasses[size]} bg-editor-line rounded-full overflow-hidden`}>
          <div
            className={`${sizeClasses[size]} ${getStatusColor()} transition-all duration-300`}
            style={{ width: `${percentage}%` }}
          />
        </div>
      </div>
      <span className="text-xs text-editor-fg/70 min-w-[3rem] text-right">
        {completed}/{total} ({Math.round(percentage)}%)
      </span>
    </div>
  );
};

export default TaskProgress;
