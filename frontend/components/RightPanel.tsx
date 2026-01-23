import React from 'react';
import LocalRightPanel from './panels/LocalRightPanel';
import RemoteRightPanel from './panels/RemoteRightPanel';
import { useAppStore } from '../store/useAppStore';

interface RightPanelProps {
  onAction: (msg: string) => void;
  activeFileExtension: string;
  repoRefreshKey: number;
  onSelectFile: (file: string | null) => void;
  selectedFile: string | null;
  diffContext: { base: string; head: string };
}

export const RightPanel: React.FC<RightPanelProps> = ({ onAction }) => {
  const { mode } = useAppStore();

  return (
    <div className="h-full">
      {mode === 'local' ? <LocalRightPanel onAction={onAction} /> : <RemoteRightPanel onAction={onAction} />}
    </div>
  );
};

export default RightPanel;
