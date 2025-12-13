import React, { useEffect, useState } from 'react';
import { RefreshCw, CheckCircle2, GitBranch, ArrowDown, ArrowUp, Terminal } from 'lucide-react';
import { useTranslation } from '../i18n';

interface SyncStatusModalProps {
  onClose: () => void;
}

const SyncStatusModal: React.FC<SyncStatusModalProps> = ({ onClose }) => {
  const { t } = useTranslation();
  const [logs, setLogs] = useState<string[]>([]);
  const [syncing, setSyncing] = useState(true);

  useEffect(() => {
    const sequence = [
        "Fetching origin...",
        "Pruning obsolete refs...",
        "remote: Enumerating objects: 42, done.",
        "remote: Compressing objects: 100% (28/28), done.",
        "remote: Total 42 (delta 14), reused 0 (delta 0), pack-reused 0",
        "Unpacking objects: 100% (42/42), 6.12 KiB | 124.00 KiB/s, done.",
        "From codearts.huawei.com:payment/service",
        "   3c9fa1b..a1b2c3d  feature/payment-retry -> origin/feature/payment-retry",
        "Updating 3c9fa1b..a1b2c3d",
        "Fast-forward",
        " src/main/java/PaymentService.java | 12 ++++",
        " 1 file changed, 12 insertions(+)",
        "Sync completed successfully."
    ];
    
    let i = 0;
    const interval = setInterval(() => {
        if (i < sequence.length) {
            setLogs(prev => [...prev, sequence[i]]);
            i++;
        } else {
            setSyncing(false);
            clearInterval(interval);
        }
    }, 400);

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="flex flex-col gap-0">
       <div className="flex items-center gap-4 py-4 px-2 border-b border-editor-line mb-2">
           <div className="flex flex-col items-center gap-1">
               <div className={`p-2 rounded-full ${syncing ? 'bg-editor-accent/20 text-editor-accent' : 'bg-editor-success/20 text-editor-success'} transition-colors duration-500`}>
                   {syncing ? <RefreshCw size={24} className="animate-spin" /> : <CheckCircle2 size={24} />}
               </div>
           </div>
           <div className="flex-1">
               <h3 className="text-sm font-bold text-white transition-all">{syncing ? t('modal.sync.syncing') : t('modal.sync.uptodate')}</h3>
               <p className="text-xs text-gray-500">{syncing ? t('modal.sync.pulling') : t('modal.sync.last_sync')}</p>
           </div>
           <div className="flex items-center gap-3 text-xs text-gray-400 bg-editor-line/30 px-3 py-1.5 rounded border border-editor-line/50">
               <span className="flex items-center gap-1 text-editor-info"><ArrowDown size={12} /> 12</span>
               <div className="w-[1px] h-3 bg-editor-line"></div>
               <span className="flex items-center gap-1 text-editor-success"><ArrowUp size={12} /> 0</span>
           </div>
       </div>

       <div className="bg-black/40 rounded p-3 font-mono text-[11px] text-gray-400 h-[200px] overflow-y-auto border border-editor-line/50 shadow-inner">
           {logs.map((log, i) => (
               <div key={i} className="mb-1 whitespace-pre-wrap break-all">
                   <span className="text-gray-600 mr-2 select-none">$</span>
                   <span className={log.includes("done") || log.includes("success") ? "text-editor-success" : ""}>{log}</span>
               </div>
           ))}
           {syncing && <div className="animate-pulse text-editor-accent">_</div>}
       </div>

        <div className="flex justify-end gap-2 pt-4">
            <button onClick={onClose} className="px-4 py-1.5 rounded text-xs bg-editor-line hover:bg-gray-600 text-white border border-editor-line transition-colors">{t('modal.sync.close')}</button>
            {!syncing && (
                <button onClick={onClose} className="px-4 py-1.5 rounded text-xs bg-editor-accent text-white hover:bg-blue-600 font-medium transition-colors shadow-sm">
                    OK
                </button>
            )}
        </div>
    </div>
  );
};

export default SyncStatusModal;