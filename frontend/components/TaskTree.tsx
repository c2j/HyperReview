import React from 'react';
import LocalTaskTree from './task-trees/LocalTaskTree';
import RemoteTaskTree from './task-trees/RemoteTaskTree';
import { useAppStore } from '../store/useAppStore';

interface TaskTreeProps {
  activeTaskId: string;
  onSelectTask: (id: string) => void;
  onAction: (msg: string) => void;
  repoRefreshKey: number;
  onSelectFile: (file: string | null) => void;
  selectedFile: string | null;
  gerritRefreshKey: number;
}

export const TaskTree: React.FC<TaskTreeProps> = (props) => {
  const { mode } = useAppStore();

  if (mode === 'local') {
    return <LocalTaskTree {...props} />;
  }

  return <RemoteTaskTree {...props} />;
};

export default TaskTree;
