import React, { useEffect, useState } from 'react';
import { RefreshCw, CheckCircle2, AlertTriangle, ArrowDown, ArrowUp } from 'lucide-react';
import { useTranslation } from '../i18n';
import { useApiClient } from '../api/client';

interface SyncStatusModalProps {
  onClose: () => void;
}

const SyncStatusModal: React.FC<SyncStatusModalProps> = ({ onClose }) => {
  const { t } = useTranslation();
  const { syncRepo } = useApiClient();
  const [logs, setLogs] = useState<string[]>([]);
  const [syncing, setSyncing] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [syncResult, setSyncResult] = useState<any>(null);

  useEffect(() => {
    const performSync = async () => {
      try {
        setLogs(prev => [...prev, "Starting repository sync..."]);

        const result = await syncRepo();
        setSyncResult(result);

        if (result.success) {
          setLogs(prev => [...prev,
            "✓ Fetching origin...",
            "✓ Pruning obsolete refs...",
            "✓ Unpacking objects...",
            "✓ Sync completed successfully."
          ]);
        } else {
          setError(result.message || "Sync failed");
          setLogs(prev => [...prev,
            "✗ Sync failed: " + (result.message || "Unknown error"),
            "Please check your network connection and credentials."
          ]);
        }
      } catch (err) {
        const errorMessage = err instanceof Error ? err.message : "Unknown error";
        setError(errorMessage);
        setLogs(prev => [...prev,
          "✗ Sync error: " + errorMessage,
          "Operation failed. Please try again."
        ]);
      } finally {
        setSyncing(false);
      }
    };

    performSync();
  }, [syncRepo]);

  const getStatusIcon = () => {
    if (syncing) return <RefreshCw size={24} className="animate-spin" />;
    if (error) return <AlertTriangle size={24} className="text-editor-warning" />;
    return <CheckCircle2 size={24} className="text-editor-success" />;
  };

  const getStatusTitle = () => {
    if (syncing) return t('modal.sync.syncing') || 'Syncing Repository';
    if (error) return 'Sync Failed';
    return t('modal.sync.uptodate') || 'Sync Complete';
  };

  const getStatusMessage = () => {
    if (syncing) return t('modal.sync.pulling') || 'Synchronizing with remote repository...';
    if (error) return 'Please check the logs below for details';
    return t('modal.sync.last_sync') || 'Repository is up to date';
  };

  const handleRetry = () => {
    setLogs([]);
    setError(null);
    setSyncing(true);
    setSyncResult(null);

    // Trigger sync again
    performSync();
  };

  // Re-expose performSync for retry functionality
  const performSync = async () => {
    try {
      setLogs(prev => [...prev, "Starting repository sync..."]);

      const result = await syncRepo();
      setSyncResult(result);

      if (result.success) {
        setLogs(prev => [...prev,
          "✓ Fetching origin...",
          "✓ Pruning obsolete refs...",
          "✓ Unpacking objects...",
          "✓ Sync completed successfully."
        ]);
      } else {
        setError(result.message || "Sync failed");
        setLogs(prev => [...prev,
          "✗ Sync failed: " + (result.message || "Unknown error"),
          "Please check your network connection and credentials."
        ]);
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : "Unknown error";
      setError(errorMessage);
      setLogs(prev => [...prev,
        "✗ Sync error: " + errorMessage,
        "Operation failed. Please try again."
      ]);
    } finally {
      setSyncing(false);
    }
  };

  // Make performSync available in useEffect
  useEffect(() => {
    (window as any).performSync = performSync;
  }, [syncRepo]);

  return (
    <div className="flex flex-col gap-0">
       <div className="flex items-center gap-4 py-4 px-2 border-b border-editor-line mb-2">
           <div className="flex flex-col items-center gap-1">
               <div className={`p-2 rounded-full transition-colors duration-500 ${
                   syncing ? 'bg-editor-accent/20 text-editor-accent' :
                   error ? 'bg-editor-warning/20 text-editor-warning' :
                   'bg-editor-success/20 text-editor-success'
               }`}>
                   {getStatusIcon()}
               </div>
           </div>
           <div className="flex-1">
               <h3 className="text-sm font-bold text-white transition-all">{getStatusTitle()}</h3>
               <p className="text-xs text-gray-500">{getStatusMessage()}</p>
           </div>
           <div className="flex items-center gap-3 text-xs text-gray-400 bg-editor-line/30 px-3 py-1.5 rounded border border-editor-line/50">
               <ArrowDown size={12} />
               <span>{syncResult?.objects?.received || 0}</span>
               <ArrowUp size={12} />
               <span>{syncResult?.objects?.sent || 0}</span>
           </div>
       </div>

       <div className="flex flex-col gap-2">
           <div className="bg-editor-line/30 p-2 rounded border border-editor-line">
               <div className="text-xs text-gray-400 mb-1">{syncing ? 'Sync Progress' : 'Sync Log'}</div>
               <div className="max-h-[200px] overflow-y-auto font-mono text-[11px] space-y-1">
                   {logs.map((log, index) => (
                       <div key={index} className={`${
                           log.startsWith('✓') ? 'text-green-400' :
                           log.startsWith('✗') ? 'text-red-400' :
                           'text-gray-300'
                       }`}>
                           {log}
                       </div>
                   ))}
               </div>
           </div>

           {!syncing && error && (
               <div className="bg-editor-error/10 border border-editor-error/30 rounded p-3">
                   <div className="flex items-start gap-2">
                       <AlertTriangle size={14} className="text-editor-error shrink-0 mt-0.5" />
                       <div className="text-xs text-editor-error">
                           <div className="font-semibold mb-1">Sync Failed</div>
                           <div>{error}</div>
                       </div>
                   </div>
                   <div className="flex gap-2 mt-3">
                       <button
                           onClick={handleRetry}
                           className="px-3 py-1.5 bg-editor-accent text-white rounded text-xs hover:bg-editor-accent/80 transition-colors"
                       >
                           Retry
                       </button>
                       <button
                           onClick={onClose}
                           className="px-3 py-1.5 bg-editor-line text-editor-fg rounded text-xs hover:bg-editor-line/80 transition-colors"
                       >
                           Close
                       </button>
                   </div>
               </div>
           )}
       </div>

       {!error && !syncing && (
           <div className="flex justify-end gap-2 mt-4">
               <button
                   onClick={onClose}
                   className="px-3 py-1.5 bg-editor-accent text-white rounded text-sm hover:bg-editor-accent/80 transition-colors"
               >
                   Close
               </button>
           </div>
       )}
    </div>
  );
};

export default SyncStatusModal;