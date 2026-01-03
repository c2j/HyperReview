import React, { useState, useEffect } from 'react';
import Modal from '@components/Modal';
import type { LocalTask } from '../types/task';
import { useLocalTasks } from '@hooks/useLocalTasks';

interface EditTaskModalProps {
  isOpen: boolean;
  onClose: () => void;
  task: LocalTask | null;
  onSave: (task: LocalTask) => void;
}

const EditTaskModal: React.FC<EditTaskModalProps> = ({ isOpen, onClose, task, onSave }) => {
  const [name, setName] = useState('');
  const [baseRef, setBaseRef] = useState('');
  const [itemsText, setItemsText] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const { updateTask } = useLocalTasks();

  useEffect(() => {
    if (task) {
      setName(task.name);
      setBaseRef(task.base_ref);
      const itemsStr = task.items
        .map((item: any) => {
          let line = item.file;
          if (item.preset_comment) {
            line += `\t${item.preset_comment}`;
          }
          if (item.tags && item.tags.length > 0) {
            line += `\t${item.tags.join(',')}`;
          }
          return line;
        })
        .join('\n');
      setItemsText(itemsStr);
    }
  }, [task]);

  const handleSave = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!task || !name.trim()) return;

    setIsLoading(true);
    try {
      const updated = await updateTask(task.id, name, baseRef);
      onSave(updated);
      onClose();
    } catch (error) {
      console.error('Failed to update task:', error);
      alert(`Failed to update task: ${error}`);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose} title="Edit Task">
      <form onSubmit={handleSave} className="space-y-4">
        <div>
          <label className="block text-sm font-medium mb-1 text-editor-fg">Task Name</label>
          <input
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
            className="w-full px-3 py-2 bg-editor-input border border-editor-line rounded text-editor-fg focus:outline-none focus:border-orange-500"
            required
          />
        </div>

        <div>
          <label className="block text-sm font-medium mb-1 text-editor-fg">Base Reference</label>
          <input
            type="text"
            value={baseRef}
            onChange={(e) => setBaseRef(e.target.value)}
            className="w-full px-3 py-2 bg-editor-input border border-editor-line rounded text-editor-fg focus:outline-none focus:border-orange-500"
            placeholder="main or commit hash"
          />
        </div>

        <div>
          <label className="block text-sm font-medium mb-1 text-editor-fg">Task Items</label>
          <p className="text-xs text-editor-fg/60 mb-2">
            Edit items in text format. Changes will be parsed and saved.
          </p>
          <textarea
            value={itemsText}
            onChange={(e) => setItemsText(e.target.value)}
            rows={10}
            className="w-full px-3 py-2 bg-editor-input border border-editor-line rounded text-editor-fg focus:outline-none focus:border-orange-500 font-mono text-sm"
          />
        </div>

        <div className="flex justify-end gap-2 pt-2">
          <button
            type="button"
            onClick={onClose}
            className="px-4 py-2 rounded border border-editor-line text-editor-fg hover:bg-editor-line transition-colors"
          >
            Cancel
          </button>
          <button
            type="submit"
            disabled={isLoading || !name.trim()}
            className="px-4 py-2 rounded bg-orange-600 text-white hover:bg-orange-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            {isLoading ? 'Saving...' : 'Save Changes'}
          </button>
        </div>
      </form>
    </Modal>
  );
};

export default EditTaskModal;
