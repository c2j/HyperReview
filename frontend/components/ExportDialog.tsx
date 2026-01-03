import React, { useState } from 'react';
import Modal from '@components/Modal';
import { FileDown, Download } from 'lucide-react';
import { useLocalTasks } from '@hooks/useLocalTasks';

interface ExportDialogProps {
  isOpen: boolean;
  onClose: () => void;
  taskId: string | null;
  taskName: string;
}

type ExportFormat = 'json' | 'csv';

const ExportDialog: React.FC<ExportDialogProps> = ({ isOpen, onClose, taskId, taskName }) => {
  const [format, setFormat] = useState<ExportFormat>('json');
  const [includeComments, setIncludeComments] = useState(true);
  const [includeProgress, setIncludeProgress] = useState(true);
  const [isExporting, setIsExporting] = useState(false);
  const { exportTask } = useLocalTasks();

  const handleExport = async () => {
    if (!taskId) return;

    setIsExporting(true);
    try {
      let content: string;
      if (taskId) {
        content = await exportTask(taskId);
      } else {
        content = '';
      }

      const fileName = taskId ? `${taskName}_export` : 'all_tasks_export';
      const extension = format === 'json' ? 'json' : 'csv';

      const blob = new Blob([content], { type: `application/${extension}` });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `${fileName}.${extension}`;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);

      onClose();
    } catch (error) {
      console.error('Failed to export:', error);
      alert(`Failed to export task: ${error}`);
    } finally {
      setIsExporting(false);
    }
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose} title="Export Task">
      <div className="space-y-4">
        <div>
          <label className="block text-sm font-medium mb-2 text-editor-fg">Export Format</label>
          <div className="grid grid-cols-2 gap-3">
            <button
              onClick={() => setFormat('json')}
              className={`
                p-4 rounded border transition-all flex flex-col items-center gap-2
                ${
                  format === 'json'
                    ? 'bg-orange-500/20 border-orange-500 text-orange-500'
                    : 'border-editor-line text-editor-fg hover:bg-editor-line'
                }
              `}
            >
              <FileDown size={24} />
              <span className="text-sm font-medium">JSON</span>
              <span className="text-xs text-editor-fg/60">Standard format, full data</span>
            </button>
            <button
              onClick={() => setFormat('csv')}
              className={`
                p-4 rounded border transition-all flex flex-col items-center gap-2
                ${
                  format === 'csv'
                    ? 'bg-orange-500/20 border-orange-500 text-orange-500'
                    : 'border-editor-line text-editor-fg hover:bg-editor-line'
                }
              `}
            >
              <Download size={24} />
              <span className="text-sm font-medium">CSV</span>
              <span className="text-xs text-editor-fg/60">Spreadsheet compatible</span>
            </button>
          </div>
        </div>

        {format === 'json' && (
          <div className="space-y-2">
            <label className="block text-sm font-medium mb-2 text-editor-fg">Export Options</label>
            <label className="flex items-center gap-2 cursor-pointer">
              <input
                type="checkbox"
                checked={includeComments}
                onChange={(e) => setIncludeComments(e.target.checked)}
                className="rounded border-editor-line"
              />
              <span className="text-sm text-editor-fg">Include review comments</span>
            </label>
            <label className="flex items-center gap-2 cursor-pointer">
              <input
                type="checkbox"
                checked={includeProgress}
                onChange={(e) => setIncludeProgress(e.target.checked)}
                className="rounded border-editor-line"
              />
              <span className="text-sm text-editor-fg">Include progress data</span>
            </label>
          </div>
        )}

        <div className="p-3 bg-editor-line/30 rounded text-sm text-editor-fg/70">
          <p className="mb-1">
            Task: <span className="font-medium text-editor-fg">{taskName}</span>
          </p>
          {taskId && (
            <p>
              Task ID: <span className="font-mono text-xs">{taskId}</span>
            </p>
          )}
        </div>

        <div className="flex justify-end gap-2 pt-2">
          <button
            onClick={onClose}
            className="px-4 py-2 rounded border border-editor-line text-editor-fg hover:bg-editor-line transition-colors"
          >
            Cancel
          </button>
          <button
            onClick={handleExport}
            disabled={isExporting}
            className="px-4 py-2 rounded bg-orange-600 text-white hover:bg-orange-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors flex items-center gap-2"
          >
            <Download size={16} />
            {isExporting ? 'Exporting...' : 'Export'}
          </button>
        </div>
      </div>
    </Modal>
  );
};

export default ExportDialog;
