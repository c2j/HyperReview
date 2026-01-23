import React from 'react';
import { useAppStore } from '../store/useAppStore';
import LocalToolBar from './toolbars/LocalToolBar';
import RemoteToolBar from './toolbars/RemoteToolBar';

interface ToolBarProps {
  onAction: (msg: string) => void;
  onOpenRepo?: () => void;
  onNewTask?: () => void;
  showLeft?: boolean;
  showRight?: boolean;
  onToggleLeft?: () => void;
  onToggleRight?: () => void;
  diffContext?: { base: string; head: string };
}

const ToolBar: React.FC<ToolBarProps> = (props) => {
  const { mode } = useAppStore();

  if (mode === 'local') {
    return (
      <LocalToolBar
        onAction={props.onAction}
        onOpenRepo={props.onOpenRepo || (() => {})}
        onNewTask={props.onNewTask || (() => {})}
        showLeft={props.showLeft ?? true}
        showRight={props.showRight ?? true}
        onToggleLeft={props.onToggleLeft || (() => {})}
        onToggleRight={props.onToggleRight || (() => {})}
        diffContext={props.diffContext || { base: 'master', head: 'feature/payment-retry' }}
      />
    );
  }

  return (
    <RemoteToolBar
      onAction={props.onAction}
      showLeft={props.showLeft ?? true}
      showRight={props.showRight ?? true}
      onToggleLeft={props.onToggleLeft || (() => {})}
      onToggleRight={props.onToggleRight || (() => {})}
      diffContext={props.diffContext || { base: 'master', head: 'feature/payment-retry' }}
    />
  );
};

export default ToolBar;
