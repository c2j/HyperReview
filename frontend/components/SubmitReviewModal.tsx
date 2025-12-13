import React, { useState, useEffect } from 'react';
import { ShieldCheck, AlertTriangle, CheckCircle2, XCircle, ArrowRight, Loader2 } from 'lucide-react';
import { useTranslation } from '../i18n';
import { getQualityGates } from '../api/client';
import type { QualityGate } from '../api/types';

interface SubmitReviewModalProps {
  onClose: () => void;
  onSubmit: () => void;
}

const SubmitReviewModal: React.FC<SubmitReviewModalProps> = ({ onClose, onSubmit }) => {
  const { t } = useTranslation();
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [gates, setGates] = useState<QualityGate[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
      setLoading(true);
      getQualityGates()
        .then(setGates)
        .catch(console.error)
        .finally(() => setLoading(false));
  }, []);

  const handleSubmit = () => {
    setIsSubmitting(true);
    setTimeout(() => {
        onSubmit();
    }, 1500);
  };

  return (
    <div className="flex flex-col gap-4">
      <div className="bg-editor-line/30 p-4 rounded border border-editor-line flex items-start gap-3">
        <ShieldCheck className="text-editor-success shrink-0" size={20} />
        <div>
            <h3 className="text-sm font-bold text-white mb-1">{t('modal.submit.ready')}</h3>
            <p className="text-xs text-gray-400">You are approving <span className="text-white">PR#2877</span>. This will trigger the merge pipeline.</p>
        </div>
      </div>

      <div>
        <h4 className="text-[10px] uppercase font-bold text-gray-500 mb-2">{t('modal.submit.gates')}</h4>
        <div className="space-y-2 max-h-[150px] overflow-y-auto pr-1">
             {loading ? (
                <div className="flex flex-col items-center justify-center py-4 text-gray-500 gap-2">
                    <Loader2 size={16} className="animate-spin text-editor-accent" />
                    <span className="text-xs">Checking gates...</span>
                </div>
             ) : (
                gates.map(gate => (
                    <div key={gate.id} className="flex items-center justify-between p-2 bg-editor-line/20 rounded border border-editor-line/50">
                        <div className="flex items-center gap-2">
                            {gate.status === 'passed' && <CheckCircle2 size={14} className="text-editor-success" />}
                            {gate.status === 'warning' && <AlertTriangle size={14} className="text-editor-warning" />}
                            {gate.status === 'failed' && <XCircle size={14} className="text-editor-error" />}
                            <span className="text-xs text-editor-fg">{gate.name}</span>
                        </div>
                        <span className={`text-xs font-mono ${gate.status === 'passed' ? 'text-editor-success' : gate.status === 'warning' ? 'text-editor-warning' : 'text-editor-error'}`}>
                            {gate.message}
                        </span>
                    </div>
                ))
             )}
        </div>
      </div>

      <div>
          <h4 className="text-[10px] uppercase font-bold text-gray-500 mb-2">{t('modal.submit.summary')}</h4>
          <div className="flex gap-2 text-xs">
              <span className="bg-editor-error/10 text-editor-error px-2 py-1 rounded border border-editor-error/20">0 Blockers</span>
              <span className="bg-editor-warning/10 text-editor-warning px-2 py-1 rounded border border-editor-warning/20">2 Concerns</span>
              <span className="bg-editor-info/10 text-editor-info px-2 py-1 rounded border border-editor-info/20">1 Question</span>
          </div>
      </div>

      <textarea 
        className="w-full h-20 bg-editor-bg border border-editor-line rounded p-3 text-sm text-white placeholder-gray-600 focus:outline-none focus:border-editor-accent resize-none font-mono mt-2"
        placeholder="Optional: Add a final summary message..."
      />

      <div className="flex justify-end gap-2 pt-3 border-t border-editor-line mt-1">
        <button onClick={onClose} className="px-4 py-1.5 rounded text-xs hover:bg-editor-line text-gray-300 transition-colors">{t('modal.submit.cancel')}</button>
        <button 
          onClick={handleSubmit}
          disabled={isSubmitting || loading}
          className="px-4 py-1.5 rounded text-xs bg-editor-success text-white hover:bg-green-600 transition-colors font-medium shadow-sm flex items-center gap-2 min-w-[120px] justify-center disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {isSubmitting ? t('modal.submit.submitting') : (
             <>
               <CheckCircle2 size={14} /> {t('modal.submit.approve_merge')}
             </>
          )}
        </button>
      </div>
    </div>
  );
};

export default SubmitReviewModal;