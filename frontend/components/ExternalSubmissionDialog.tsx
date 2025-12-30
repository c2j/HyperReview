import React, { useState } from 'react';
import Modal from '@components/Modal';
import { ExternalLink, CheckCircle, AlertCircle, Settings } from 'lucide-react';

type SubmissionSystem = 'gerrit' | 'codearts' | 'custom';

interface ExternalSubmissionDialogProps {
  isOpen: boolean;
  onClose: () => void;
  taskId: string;
  taskName: string;
  onSubmitted: (result: any) => void;
}

const ExternalSubmissionDialog: React.FC<ExternalSubmissionDialogProps> = ({
  isOpen,
  onClose,
  taskId,
  taskName,
  onSubmitted,
}) => {
  const [system, setSystem] = useState<SubmissionSystem>('gerrit');
  const [isSubmitting, setIsSubmitting] = useState(false);

  const [gerritConfig, setGerritConfig] = useState({
    gerritUrl: 'https://gerrit-review.example.com',
    username: '',
    password: '',
    changeId: '',
    revisionId: '',
    score: 0,
  });

  const [codeartsConfig, setCodeartsConfig] = useState({
    projectId: '',
    mrId: '',
    approval: '',
  });

  const [customConfig, setCustomConfig] = useState({
    endpoint: '',
    method: 'POST',
    apiUrl: '',
  });

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsSubmitting(true);

    try {
      const {
        submitToGerrit: fnGerrit,
        submitToCodeArts: fnCodeArts,
        submitToCustomApi: fnCustom,
      } = await import('@hooks/useLocalTasks');

      let result;
      switch (system) {
        case 'gerrit':
          result = await fnGerrit(
            taskId,
            gerritConfig.gerritUrl,
            gerritConfig.username,
            gerritConfig.changeId,
            gerritConfig.revisionId,
            gerritConfig.score,
          );
          break;
        case 'codearts':
          result = await fnCodeArts(
            taskId,
            codeartsConfig.projectId,
            Number(codeartsConfig.mrId),
            codeartsConfig.approval || undefined,
          );
          break;
        case 'custom':
          result = await fnCustom(
            taskId,
            customConfig.endpoint,
            customConfig.method,
            customConfig.apiUrl,
          );
          break;
      }

      onSubmitted(result);
      onClose();
    } catch (error) {
      console.error('Failed to submit review:', error);
      alert(`Failed to submit review: ${error}`);
    } finally {
      setIsSubmitting(false);
    }
  };

  const renderGerritForm = () => (
    <div className="space-y-3">
      <h3 className="text-sm font-semibold text-editor-fg mb-3">Gerrit Configuration</h3>
      <div>
        <label className="block text-xs font-medium mb-1 text-editor-fg">Gerrit URL</label>
        <input
          type="text"
          value={gerritConfig.gerritUrl}
          onChange={(e) => setGerritConfig({ ...gerritConfig, gerritUrl: e.target.value })}
          className="w-full px-3 py-2 bg-editor-input border border-editor-line rounded text-editor-fg focus:outline-none focus:border-orange-500 text-sm"
          placeholder="https://gerrit.example.com"
          required
        />
      </div>
      <div>
        <label className="block text-xs font-medium mb-1 text-editor-fg">Username</label>
        <input
          type="text"
          value={gerritConfig.username}
          onChange={(e) => setGerritConfig({ ...gerritConfig, username: e.target.value })}
          className="w-full px-3 py-2 bg-editor-input border border-editor-line rounded text-editor-fg focus:outline-none focus:border-orange-500 text-sm"
          placeholder="your-username"
          required
        />
      </div>
      <div>
        <label className="block text-xs font-medium mb-1 text-editor-fg">
          Password / HTTP Password
        </label>
        <input
          type="password"
          value={gerritConfig.password}
          onChange={(e) => setGerritConfig({ ...gerritConfig, password: e.target.value })}
          className="w-full px-3 py-2 bg-editor-input border border-editor-line rounded text-editor-fg focus:outline-none focus:border-orange-500 text-sm"
          placeholder="your-password"
          required
        />
      </div>
      <div>
        <label className="block text-xs font-medium mb-1 text-editor-fg">Change ID</label>
        <input
          type="text"
          value={gerritConfig.changeId}
          onChange={(e) => setGerritConfig({ ...gerritConfig, changeId: e.target.value })}
          className="w-full px-3 py-2 bg-editor-input border border-editor-line rounded text-editor-fg focus:outline-none focus:border-orange-500 text-sm"
          placeholder="12345"
          required
        />
      </div>
      <div>
        <label className="block text-xs font-medium mb-1 text-editor-fg">Revision ID</label>
        <input
          type="text"
          value={gerritConfig.revisionId}
          onChange={(e) => setGerritConfig({ ...gerritConfig, revisionId: e.target.value })}
          className="w-full px-3 py-2 bg-editor-input border border-editor-line rounded text-editor-fg focus:outline-none focus:border-orange-500 text-sm"
          placeholder="1a2b3c..."
          required
        />
      </div>
      <div>
        <label className="block text-xs font-medium mb-1 text-editor-fg">Score (optional)</label>
        <input
          type="number"
          min={-2}
          max={2}
          value={gerritConfig.score}
          onChange={(e) => setGerritConfig({ ...gerritConfig, score: Number(e.target.value) })}
          className="w-full px-3 py-2 bg-editor-input border border-editor-line rounded text-editor-fg focus:outline-none focus:border-orange-500 text-sm"
          placeholder="+1, 0, -1, -2"
        />
        <p className="text-xs text-editor-fg/60 mt-1">Values: +2 to -2, leave empty for no vote</p>
      </div>
    </div>
  );

  const renderCodeartsForm = () => (
    <div className="space-y-3">
      <h3 className="text-sm font-semibold text-editor-fg mb-3">CodeArts Configuration</h3>
      <div>
        <label className="block text-xs font-medium mb-1 text-editor-fg">Project ID</label>
        <input
          type="text"
          value={codeartsConfig.projectId}
          onChange={(e) => setCodeartsConfig({ ...codeartsConfig, projectId: e.target.value })}
          className="w-full px-3 py-2 bg-editor-input border border-editor-line rounded text-editor-fg focus:outline-none focus:border-orange-500 text-sm"
          placeholder="project-123"
          required
        />
      </div>
      <div>
        <label className="block text-xs font-medium mb-1 text-editor-fg">Merge Request ID</label>
        <input
          type="number"
          value={codeartsConfig.mrId}
          onChange={(e) => setCodeartsConfig({ ...codeartsConfig, mrId: e.target.value })}
          className="w-full px-3 py-2 bg-editor-input border border-editor-line rounded text-editor-fg focus:outline-none focus:border-orange-500 text-sm"
          placeholder="123"
          required
        />
      </div>
      <div>
        <label className="block text-xs font-medium mb-1 text-editor-fg">
          Approval Status (optional)
        </label>
        <select
          value={codeartsConfig.approval}
          onChange={(e) => setCodeartsConfig({ ...codeartsConfig, approval: e.target.value })}
          className="w-full px-3 py-2 bg-editor-input border border-editor-line rounded text-editor-fg focus:outline-none focus:border-orange-500 text-sm"
        >
          <option value="">No vote</option>
          <option value="+1">+1 (Looks Good)</option>
          <option value="-1">-1 (Needs Work)</option>
          <option value="0">0 (No Opinion)</option>
        </select>
      </div>
    </div>
  );

  const renderCustomForm = () => (
    <div className="space-y-3">
      <h3 className="text-sm font-semibold text-editor-fg mb-3">Custom API Configuration</h3>
      <div>
        <label className="block text-xs font-medium mb-1 text-editor-fg">API URL</label>
        <input
          type="text"
          value={customConfig.apiUrl}
          onChange={(e) => setCustomConfig({ ...customConfig, apiUrl: e.target.value })}
          className="w-full px-3 py-2 bg-editor-input border border-editor-line rounded text-editor-fg focus:outline-none focus:border-orange-500 text-sm"
          placeholder="https://api.example.com"
          required
        />
      </div>
      <div>
        <label className="block text-xs font-medium mb-1 text-editor-fg">Endpoint Path</label>
        <input
          type="text"
          value={customConfig.endpoint}
          onChange={(e) => setCustomConfig({ ...customConfig, endpoint: e.target.value })}
          className="w-full px-3 py-2 bg-editor-input border border-editor-line rounded text-editor-fg focus:outline-none focus:border-orange-500 text-sm"
          placeholder="/api/reviews"
          required
        />
      </div>
      <div>
        <label className="block text-xs font-medium mb-1 text-editor-fg">HTTP Method</label>
        <select
          value={customConfig.method}
          onChange={(e) => setCustomConfig({ ...customConfig, method: e.target.value })}
          className="w-full px-3 py-2 bg-editor-input border border-editor-line rounded text-editor-fg focus:outline-none focus:border-orange-500 text-sm"
        >
          <option value="POST">POST</option>
          <option value="PUT">PUT</option>
          <option value="PATCH">PATCH</option>
        </select>
      </div>
    </div>
  );

  return (
    <Modal isOpen={isOpen} onClose={onClose} title="Submit to External System">
      <div className="space-y-4">
        <div>
          <label className="block text-xs font-medium mb-2 text-editor-fg">
            Select External System
          </label>
          <div className="grid grid-cols-3 gap-3">
            <button
              onClick={() => setSystem('gerrit')}
              className={`
                p-3 rounded border transition-all flex flex-col items-center gap-2
                ${
                  system === 'gerrit'
                    ? 'bg-orange-500/20 border-orange-500'
                    : 'border-editor-line text-editor-fg hover:bg-editor-line'
                }
              `}
            >
              <ExternalLink size={24} />
              <span className="text-sm font-medium">Gerrit</span>
              <span className="text-xs text-editor-fg/60">Code review system</span>
            </button>
            <button
              onClick={() => setSystem('codearts')}
              className={`
                p-3 rounded border transition-all flex flex-col items-center gap-2
                ${
                  system === 'codearts'
                    ? 'bg-orange-500/20 border-orange-500'
                    : 'border-editor-line text-editor-fg hover:bg-editor-line'
                }
              `}
            >
              <CheckCircle size={24} />
              <span className="text-sm font-medium">CodeArts</span>
              <span className="text-xs text-editor-fg/60">DevCloud platform</span>
            </button>
            <button
              onClick={() => setSystem('custom')}
              className={`
                p-3 rounded border transition-all flex flex-col items-center gap-2
                ${
                  system === 'custom'
                    ? 'bg-orange-500/20 border-orange-500'
                    : 'border-editor-line text-editor-fg hover:bg-editor-line'
                }
              `}
            >
              <Settings size={24} />
              <span className="text-sm font-medium">Custom API</span>
              <span className="text-xs text-editor-fg/60">Webhook/REST</span>
            </button>
          </div>
        </div>

        <div className="border-t border-editor-line pt-4">
          {system === 'gerrit' && renderGerritForm()}
          {system === 'codearts' && renderCodeartsForm()}
          {system === 'custom' && renderCustomForm()}
        </div>

        <div className="p-3 bg-editor-line/30 rounded text-xs text-editor-fg/70">
          <p className="mb-1">
            Task: <span className="font-medium text-editor-fg">{taskName}</span>
          </p>
          <p>
            Task ID: <span className="font-mono">{taskId}</span>
          </p>
        </div>

        <div className="flex justify-between items-center gap-4 pt-2">
          <button
            type="button"
            onClick={onClose}
            className="px-4 py-2 rounded border border-editor-line text-editor-fg hover:bg-editor-line transition-colors"
          >
            Cancel
          </button>
          <div className="flex items-center gap-2 text-xs text-editor-fg/60">
            <AlertCircle size={14} />
            <span>External credentials required in production</span>
          </div>
          <button
            type="submit"
            onClick={handleSubmit}
            disabled={isSubmitting}
            className="px-4 py-2 rounded bg-orange-600 text-white hover:bg-orange-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors flex items-center gap-2"
          >
            <ExternalLink size={16} />
            {isSubmitting ? 'Submitting...' : 'Submit Review'}
          </button>
        </div>
      </div>
    </Modal>
  );
};

export default ExternalSubmissionDialog;
