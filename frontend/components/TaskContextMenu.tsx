import React, { useState } from 'react';
import { Edit, Trash2, Archive, FileDown, Check, ExternalLink } from 'lucide-react';
import type { TaskSummary } from '../types/task';

interface TaskContextMenuProps {
  task: TaskSummary;
  onEdit: () => void;
  onDelete: () => void;
  onArchive: () => void;
  onExport: () => void;
  onComplete: () => void;
  onExternalSubmit: () => void;
  onClose: () => void;
  position: { x: number; y: number };
}

const TaskContextMenu: React.FC<TaskContextMenuProps> = ({
  task,
  onEdit,
  onDelete,
  onArchive,
  onExport,
  onComplete,
  onExternalSubmit,
  onClose,
  position,
}) => {
  const [showExternalMenu, setShowExternalMenu] = useState(false);

  const handleEdit = () => {
    onEdit();
    onClose();
  };

  const handleDelete = () => {
    if (window.confirm(`Are you sure you want to delete "${task.name}"?`)) {
      onDelete();
      onClose();
    }
  };

  const handleArchive = () => {
    onArchive();
    onClose();
  };

  const handleExport = () => {
    onExport();
    onClose();
  };

  const handleComplete = () => {
    if (window.confirm(`Mark "${task.name}" as completed?`)) {
      onComplete();
      onClose();
    }
  };

  const handleExternalSubmit = () => {
    onExternalSubmit();
    onClose();
  };

  const baseMenuItems = [
    {
      icon: <Edit size={14} />,
      label: 'Edit',
      onClick: handleEdit,
      disabled: task.status === 'archived',
    },
    {
      icon: <FileDown size={14} />,
      label: 'Export',
      onClick: handleExport,
    },
    {
      icon: <ExternalLink size={14} />,
      label: 'Submit to External',
      onClick: handleExternalSubmit,
      hasSubmenu: true,
    },
    {
      icon: <Archive size={14} />,
      label: 'Archive',
      onClick: handleArchive,
      disabled: task.status === 'archived',
    },
    {
      icon: <Check size={14} />,
      label: 'Mark Complete',
      onClick: handleComplete,
      disabled: task.status === 'completed' || task.status === 'archived',
    },
    {
      icon: <Trash2 size={14} />,
      label: 'Delete',
      onClick: handleDelete,
      danger: true,
    },
  ];

  return (
    <div
      className="fixed z-50 w-48 bg-editor-bg border border-editor-line rounded-lg shadow-xl overflow-hidden"
      style={{ left: position.x, top: position.y }}
      onClick={(e) => e.stopPropagation()}
    >
      {baseMenuItems.map((item, idx) => (
        <button
          key={idx}
          onClick={item.hasSubmenu ? () => setShowExternalMenu(!showExternalMenu) : item.onClick}
          disabled={item.disabled}
          className={`
            w-full px-3 py-2 text-sm flex items-center justify-between gap-2 transition-colors
            ${item.disabled ? 'opacity-50 cursor-not-allowed' : 'hover:bg-editor-line'}
            ${item.danger ? 'text-red-400 hover:text-red-300' : 'text-editor-fg'}
          `}
        >
          <div className="flex items-center gap-2">
            {item.icon}
            {item.label}
          </div>
          {item.hasSubmenu && <span className="text-editor-fg/60">›</span>}
        </button>
      ))}

      {showExternalMenu && (
        <div className="border-t border-editor-line">
          <button
            onClick={() => {
              // Would open ExternalSubmissionDialog with current task
              console.log('Submit to external for task:', task.id);
              setShowExternalMenu(false);
            }}
            className="w-full px-3 py-2 text-sm flex items-center gap-2 hover:bg-editor-line text-editor-fg"
          >
            <ExternalLink size={14} />
            Choose System...
          </button>
        </div>
      )}
    </div>
  );
};

export default TaskContextMenu;
