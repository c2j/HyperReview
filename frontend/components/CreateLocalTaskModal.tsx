import React, { useState } from 'react';
import Modal from '@components/Modal';
import { useTaskStore } from '@store/taskStore';
import type { LocalTask } from '../types/task';
import { useLocalTasks } from '@hooks/useLocalTasks';

interface CreateLocalTaskModalProps {
  isOpen: boolean;
  onClose: () => void;
  onTaskCreated?: (task: LocalTask) => void;
}

const CreateLocalTaskModal: React.FC<CreateLocalTaskModalProps> = ({
  isOpen,
  onClose,
  onTaskCreated,
}) => {
  const [name, setName] = useState('');
  const [repoPath, setRepoPath] = useState('');
  const [baseRef, setBaseRef] = useState('main');
  const [itemsText, setItemsText] = useState('');
  const [preview, setPreview] = useState<string[]>([]);
  const [isLoading, setIsLoading] = useState(false);

  const { fetchTasks } = useTaskStore();
  const { createTask } = useLocalTasks();

  const handleItemsTextChange = (text: string) => {
    setItemsText(text);
    const lines = text
      .trim()
      .split('\n')
      .filter((line) => line.trim());
    setPreview(lines);
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!name.trim() || !repoPath.trim() || !itemsText.trim()) {
      return;
    }

    setIsLoading(true);
    try {
      const task = await createTask(name, repoPath, baseRef, itemsText);

      if (onTaskCreated) {
        onTaskCreated(task);
      }

      await fetchTasks();

      onClose();
      setName('');
      setRepoPath('');
      setBaseRef('main');
      setItemsText('');
      setPreview([]);
    } catch (error) {
      console.error('Failed to create task:', error);
      alert(`Failed to create task: ${error}`);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose} title="Create Local Task">
      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label htmlFor="task-name" className="block text-sm font-medium mb-1 text-editor-fg">Task Name</label>
          <input
            id="task-name"
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
            className="w-full px-3 py-2 bg-editor-input border border-editor-line rounded text-editor-fg focus:outline-none focus:border-orange-500"
            placeholder="e.g., Code Review - Feature X"
            required
          />
        </div>

        <div>
          <label htmlFor="repo-path" className="block text-sm font-medium mb-1 text-editor-fg">Repository Path</label>
          <input
            id="repo-path"
            type="text"
            value={repoPath}
            onChange={(e) => setRepoPath(e.target.value)}
            className="w-full px-3 py-2 bg-editor-input border border-editor-line rounded text-editor-fg focus:outline-none focus:border-orange-500"
            placeholder="/path/to/repository"
            required
          />
        </div>

        <div>
          <label htmlFor="base-ref" className="block text-sm font-medium mb-1 text-editor-fg">Base Reference</label>
          <input
            id="base-ref"
            type="text"
            value={baseRef}
            onChange={(e) => setBaseRef(e.target.value)}
            className="w-full px-3 py-2 bg-editor-input border border-editor-line rounded text-editor-fg focus:outline-none focus:border-orange-500"
            placeholder="main or commit hash"
          />
        </div>

        <div>
          <label htmlFor="file-list" className="block text-sm font-medium mb-1 text-editor-fg">
            File List (one per line, tab or double-space for comments)
          </label>
          <textarea
            id="file-list"
            value={itemsText}
            onChange={(e) => handleItemsTextChange(e.target.value)}
            rows={8}
            className="w-full px-3 py-2 bg-editor-input border border-editor-line rounded text-editor-fg focus:outline-none focus:border-orange-500 font-mono text-sm"
            placeholder="src/main.rs\nsrc/lib.rs\tCheck this file\nsrc/utils.rs"
            required
          />
        </div>

        {preview.length > 0 && (
          <div>
            <label className="block text-sm font-medium mb-1 text-editor-fg">
              Preview ({preview.length} files)
            </label>
            <div className="border border-editor-line rounded bg-editor-input max-h-40 overflow-y-auto">
              {preview.map((line, idx) => (
                <div
                  key={idx}
                  className="px-3 py-1 text-sm border-b border-editor-line last:border-0 text-editor-fg"
                >
                  {line}
                </div>
              ))}
            </div>
          </div>
        )}

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
            disabled={isLoading || !name.trim() || !repoPath.trim() || !itemsText.trim()}
            className="px-4 py-2 rounded bg-orange-600 text-white hover:bg-orange-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            {isLoading ? 'Creating...' : 'Create Task'}
          </button>
        </div>
      </form>
    </Modal>
  );
};

export default CreateLocalTaskModal;
