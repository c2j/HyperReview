import React, { useState, useEffect } from 'react';
import { GitBranch, ArrowRight, ArrowLeftRight, Check, Play, Loader2, ArrowLeft } from 'lucide-react';
import { useTranslation } from '../i18n';
import { getBranches } from '../api/client';
import type { Branch } from '../api/types';

interface BranchCompareModalProps {
  currentBase: string;
  currentHead: string;
  onClose: () => void;
  onApply: (base: string, head: string) => void;
  isInitialSetup?: boolean;
  onBack?: () => void;
}

const BranchCompareModal: React.FC<BranchCompareModalProps> = ({ currentBase, currentHead, onClose, onApply, isInitialSetup = false, onBack }) => {
  const { t } = useTranslation();
  const [base, setBase] = useState(currentBase);
  const [head, setHead] = useState(currentHead);
  const [branches, setBranches] = useState<Branch[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    setLoading(true);
    getBranches()
      .then(setBranches)
      .catch(console.error)
      .finally(() => setLoading(false));
  }, []);

  const handleSwap = () => {
    const temp = base;
    setBase(head);
    setHead(temp);
  };

  return (
    <div className="flex flex-col gap-6 p-2">
      {isInitialSetup && (
        <div className="text-xs text-gray-400 font-bold -mt-2 mb-2 pb-2 border-b border-editor-line">
            {t('modal.branch_compare.step2')}
        </div>
      )}

      {loading ? (
        <div className="flex flex-col items-center justify-center py-8 text-gray-500 gap-2">
           <Loader2 size={24} className="animate-spin text-editor-accent" />
           <span className="text-xs">Loading branches...</span>
        </div>
      ) : (
      <div className="flex items-center justify-between gap-4">
        
        {/* Base Branch Selection */}
        <div className="flex-1">
          <label className="text-xs text-editor-error font-bold mb-2 block flex items-center gap-1">
             <GitBranch size={12} /> {t('modal.branch_compare.base')}
          </label>
          <div className="space-y-1 max-h-[200px] overflow-y-auto border border-editor-line rounded p-1 bg-editor-sidebar">
             {branches.map(b => (
                <div 
                    key={`base-${b.name}`}
                    onClick={() => setBase(b.name)}
                    className={`px-3 py-2 rounded cursor-pointer text-xs font-mono flex items-center justify-between transition-colors
                        ${base === b.name ? 'bg-editor-error/20 text-white' : 'text-gray-400 hover:bg-editor-line hover:text-gray-300'}`}
                >
                    {b.name}
                    {base === b.name && <Check size={12} className="text-editor-error" />}
                </div>
             ))}
          </div>
        </div>

        {/* Direction Indicator */}
        <div className="flex flex-col items-center justify-center gap-2 pt-6">
            <ArrowRight size={24} className="text-gray-600" />
            <button 
                onClick={handleSwap}
                className="p-2 rounded-full hover:bg-editor-line text-gray-400 hover:text-white transition-colors"
                title={t('modal.branch_compare.swap')}
            >
                <ArrowLeftRight size={16} />
            </button>
        </div>

        {/* Head Branch Selection */}
        <div className="flex-1">
          <label className="text-xs text-editor-success font-bold mb-2 block flex items-center gap-1">
             <GitBranch size={12} /> {t('modal.branch_compare.compare')}
          </label>
           <div className="space-y-1 max-h-[200px] overflow-y-auto border border-editor-line rounded p-1 bg-editor-sidebar">
             {branches.map(b => (
                <div 
                    key={`head-${b.name}`}
                    onClick={() => setHead(b.name)}
                    className={`px-3 py-2 rounded cursor-pointer text-xs font-mono flex items-center justify-between transition-colors
                        ${head === b.name ? 'bg-editor-success/20 text-white' : 'text-gray-400 hover:bg-editor-line hover:text-gray-300'}`}
                >
                    {b.name}
                    {head === b.name && <Check size={12} className="text-editor-success" />}
                </div>
             ))}
          </div>
        </div>

      </div>
      )}

      <div className="flex justify-between items-center pt-4 border-t border-editor-line">
        <div>
            {isInitialSetup && onBack && (
                <button onClick={onBack} className="flex items-center gap-1 px-3 py-1.5 rounded text-xs hover:bg-editor-line text-gray-300 transition-colors">
                    <ArrowLeft size={12} /> {t('modal.branch_compare.back')}
                </button>
            )}
        </div>
        <div className="flex gap-2">
            <button onClick={onClose} className="px-4 py-1.5 rounded text-xs hover:bg-editor-line text-gray-300 transition-colors">
                {t('modal.open_repo.cancel')}
            </button>
            <button 
            onClick={() => onApply(base, head)}
            disabled={loading}
            className={`px-4 py-1.5 rounded text-xs text-white hover:bg-blue-600 transition-colors font-medium shadow-sm flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed
                ${isInitialSetup ? 'bg-editor-success hover:bg-green-600' : 'bg-editor-accent'}`}
            >
            {isInitialSetup ? (
                <>
                    <Play size={12} /> {t('modal.branch_compare.start')}
                </>
            ) : (
                t('modal.branch_compare.apply')
            )}
            </button>
        </div>
      </div>
    </div>
  );
};

export default BranchCompareModal;