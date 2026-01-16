import React, { useState } from 'react';
import { RefreshCw, Upload, AlertCircle, CheckCircle2, X, Loader2 } from 'lucide-react';

interface GerritPushControlsProps {
  changeId: string;
  localCommentCount: number;
  pendingSyncCount: number;
  isOnline: boolean;
  onSync: () => void;
  onPush: () => void;
  onRefresh: () => void;
}

const GerritPushControls: React.FC<GerritPushControlsProps> = ({
  localCommentCount,
  pendingSyncCount,
  isOnline,
  onSync,
  onPush,
  onRefresh,
}) => {
  const [isSyncing, setIsSyncing] = useState(false);
  const [isPushing, setIsPushing] = useState(false);
  const [showSuccess, setShowSuccess] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSync = async () => {
    if (!isOnline || isSyncing) return;

    setIsSyncing(true);
    setError(null);

    try {
      await onSync();
      setShowSuccess(true);
      setTimeout(() => setShowSuccess(false), 3000);
    } catch (err) {
      console.error('Sync failed:', err);
      setError('Sync failed: ' + (err as Error).message);
    } finally {
      setIsSyncing(false);
    }
  };

  const handlePush = async () => {
    if (!isOnline || localCommentCount === 0 || isPushing) return;

    setIsPushing(true);
    setError(null);

    try {
      await onPush();
      setShowSuccess(true);
      setTimeout(() => setShowSuccess(false), 3000);
    } catch (err) {
      console.error('Push failed:', err);
      setError('Push failed: ' + (err as Error).message);
    } finally {
      setIsPushing(false);
    }
  };

  const handleCloseSuccess = () => {
    setShowSuccess(false);
    setError(null);
  };

  return (
    <div className="flex items-center gap-3">
      <div className="flex-1">
        {pendingSyncCount > 0 && (
          <button
            onClick={handleSync}
            disabled={isSyncing || !isOnline}
            className="w-full px-4 py-2 rounded bg-editor-line/50 border border-editor-line hover:bg-editor-line/80 transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
            title="Sync pending changes with Gerrit"
          >
            {isSyncing ? (
              <Loader2 size={16} className="animate-spin" />
            ) : (
              <RefreshCw size={16} />
            )}
            <span className="text-sm text-editor-fg">
              Sync {pendingSyncCount} pending
            </span>
          </button>
        )}

        {localCommentCount > 0 && pendingSyncCount === 0 && (
          <button
            onClick={handlePush}
            disabled={isPushing || !isOnline}
            className="w-full px-4 py-2 rounded bg-editor-accent/10 border border-editor-accent/50 hover:bg-editor-accent/20 transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
            title={`Push ${localCommentCount} comment${localCommentCount !== 1 ? 's' : ''} to Gerrit`}
          >
            {isPushing ? (
              <>
                <Loader2 size={16} className="animate-spin" />
                <span className="text-sm text-white">Pushing...</span>
              </>
            ) : (
              <>
                <Upload size={16} />
                <span className="text-sm text-editor-fg">
                  Push {localCommentCount} comment{localCommentCount !== 1 ? 's' : ''}
                </span>
              </>
            )}
          </button>
        )}
      </div>

      <button
        onClick={onRefresh}
        disabled={isSyncing || isPushing}
        className="px-3 py-2 rounded bg-editor-line/50 border border-editor-line hover:bg-editor-line/80 transition-all disabled:opacity-30 disabled:cursor-not-allowed"
        title="Refresh changes"
      >
        <RefreshCw size={16} className="text-gray-400" />
      </button>

      {!isOnline && (
        <div className="px-3 py-2 rounded bg-yellow-500/10 border border-yellow-500/30 flex items-center gap-2">
          <AlertCircle size={16} className="text-yellow-400 flex-shrink-0" />
          <span className="text-xs text-yellow-400">
            Offline
          </span>
        </div>
      )}

      {showSuccess && (
        <div className="fixed top-4 right-4 z-50">
          <div className="bg-green-500/95 backdrop-blur-sm border border-green-500/30 rounded-lg shadow-xl p-4 max-w-sm">
            <div className="flex items-start gap-3">
              <CheckCircle2 size={20} className="text-green-400 flex-shrink-0 mt-0.5" />
              <div className="flex-1">
                <div className="text-sm font-medium text-editor-fg mb-1">Success</div>
                <div className="text-xs text-gray-300">
                  {pendingSyncCount > 0 && localCommentCount === 0
                    ? `Synced ${pendingSyncCount} pending changes`
                    : localCommentCount > 0
                    ? `Pushed ${localCommentCount} comment${localCommentCount !== 1 ? 's' : ''}`
                    : 'Operation completed'
                  }
                </div>
              </div>
              <button
                onClick={handleCloseSuccess}
                className="absolute top-2 right-2 p-1 rounded hover:bg-editor-line/20 text-gray-400 hover:text-white transition-colors"
              >
                <X size={16} />
              </button>
            </div>
          </div>
        </div>
      )}

      {error && (
        <div className="fixed top-4 right-4 z-50">
          <div className="bg-red-500/95 backdrop-blur-sm border border-red-500/30 rounded-lg shadow-xl p-4 max-w-sm">
            <div className="flex items-start gap-3">
              <AlertCircle size={20} className="text-red-400 flex-shrink-0 mt-0.5" />
              <div className="flex-1">
                <div className="text-sm font-medium text-editor-fg mb-1">Error</div>
                <div className="text-xs text-gray-300">{error}</div>
              </div>
              <button
                onClick={handleCloseSuccess}
                className="absolute top-2 right-2 p-1 rounded hover:bg-editor-line/20 text-gray-400 hover:text-white transition-colors"
              >
                <X size={16} />
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default GerritPushControls;
