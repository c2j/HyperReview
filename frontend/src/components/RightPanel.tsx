import React from 'react';
import LocalRightPanel from './panels/local/LocalRightPanel';
import RemoteRightPanel from './panels/remote/RemoteRightPanel';
import { useAppStore } from '../store/useAppStore';

export const RightPanel: React.FC = () => {
  const { mode } = useAppStore();

  if (mode === 'local') {
    return <LocalRightPanel />;
  }

  return <RemoteRightPanel />;
};
