import React, { useState, useEffect, useRef } from 'react';
import { GitBranch, ArrowRight, ArrowLeftRight, Check, Play, Loader2, ArrowLeft, AlertTriangle } from 'lucide-react';
import { useTranslation } from '../i18n';
import { useApiClient } from '../api/client';
import { useCurrentRepository } from '../hooks/useRepository';
import { useBranchSelectionPersistence } from '../hooks/useBranchSelectionPersistence';
import type { Branch } from '../api/types';

interface BranchCompareModalProps {
  currentBase: string;
  currentHead: string;
  onClose: () => void;
  onApply: (base: string, head: string) => void;
  isInitialSetup?: boolean;
  onBack?: () => void;
  selectedRepoPath?: string;
}

const BranchCompareModal: React.FC<BranchCompareModalProps> = ({ currentBase, currentHead, onClose, onApply, isInitialSetup = false, onBack, selectedRepoPath }) => {
  const { t } = useTranslation();
  const { getBranches } = useApiClient();
  const { loadRepository } = useCurrentRepository();
  const { baseBranch: storedBase, headBranch: storedHead } = useBranchSelectionPersistence();

  // Use props if provided, otherwise use stored values
  const [base, setBase] = useState(currentBase || storedBase || '');
  const [head, setHead] = useState(currentHead || storedHead || '');
  const [branches, setBranches] = useState<Branch[]>([]);
  const [loading, setLoading] = useState(true);
  const [repoLoading, setRepoLoading] = useState(false);

  // Track loaded repo path to prevent duplicate loads
  const loadedPathRef = useRef<string | null>(null);
  const hasInitializedRef = useRef(false);

  // Auto-select branches on mount (only once)
  useEffect(() => {
    if (hasInitializedRef.current) {
      return;
    }

    console.log('[BranchCompareModal] Initial props - currentBase:', currentBase, 'currentHead:', currentHead);
    console.log('[BranchCompareModal] Stored branches - base:', storedBase, 'head:', storedHead);

    // Use props if provided, otherwise use stored values
    const initialBase = currentBase || storedBase || '';
    const initialHead = currentHead || storedHead || '';

    if (initialBase && initialHead) {
      console.log('[BranchCompareModal] Initializing with branches:', initialBase, initialHead);
      setBase(initialBase);
      setHead(initialHead);
      hasInitializedRef.current = true;
    }
  }, [currentBase, currentHead, storedBase, storedHead]);

  useEffect(() => {
    const loadRepoAndBranches = async () => {
      if (!selectedRepoPath) {
        console.log('[BranchCompareModal] No repository path provided, using mock data');
        setLoading(false);
        return;
      }

      // Skip if we already loaded this repo
      if (loadedPathRef.current === selectedRepoPath) {
        console.log('[BranchCompareModal] Repo already loaded, skipping');
        return;
      }

      try {
        setRepoLoading(true);
        console.log('[BranchCompareModal] Loading repository:', selectedRepoPath);
        // Use loadRepository which updates store state
        await loadRepository(selectedRepoPath);
        console.log('[BranchCompareModal] ✅ Repository loaded successfully');
        loadedPathRef.current = selectedRepoPath;
      } catch (error) {
        console.error('[BranchCompareModal] ❌ Failed to load repository:', error);
        console.warn('[BranchCompareModal] Will try to get branches anyway...');
      } finally {
        setRepoLoading(false);
      }

      setLoading(true);
      console.log('[BranchCompareModal] Fetching branches...');
      getBranches()
        .then((branchList) => {
          console.log('[BranchCompareModal] ✅ Got branches:', branchList.length);
          setBranches(branchList);
        })
        .catch((error) => {
          console.error('[BranchCompareModal] ❌ Failed to get branches:', error);
          setBranches([]);
        })
        .finally(() => setLoading(false));
    };

    loadRepoAndBranches();
  }, [selectedRepoPath]);

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

      {loading || repoLoading ? (
        <div className="flex flex-col items-center justify-center py-8 text-gray-500 gap-2">
           <Loader2 size={24} className="animate-spin text-editor-accent" />
           <span className="text-xs">
             {repoLoading ? 'Loading repository...' : 'Loading branches...'}
           </span>
        </div>
      ) : (
      <>
        {/* Repository Status Info */}
        <div className="bg-editor-line/10 border border-editor-line/30 rounded p-3 text-xs">
          <div className="flex items-center gap-2 text-gray-400 font-bold mb-1">
            <GitBranch size={14} />
            Repository Loaded: {selectedRepoPath || 'N/A'}
          </div>
          <div className="text-gray-500">
            {branches.length > 0 ? (
              <div className="text-editor-success">
                ✅ Found {branches.length} branch(es) from repository
              </div>
            ) : (
              <div className="text-gray-400">
                Repository loaded but no branches found. This might be a new/empty repository.
              </div>
            )}
          </div>
        </div>

        {/* Show warning only if we have mock data (no repo path) */}
        {!selectedRepoPath && (
          <div className="bg-editor-warning/10 border border-editor-warning/30 rounded p-3 text-xs">
            <div className="flex items-center gap-2 text-editor-warning font-bold mb-1">
              <AlertTriangle size={14} />
              Demo Mode: Mock Branch Data
            </div>
            <div className="text-gray-400 space-y-1">
              <div>No repository path provided.</div>
              <div className="mt-2 text-gray-500">
                💡 The mock data below is for development/demo purposes only.
              </div>
            </div>
          </div>
        )}
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
                    <div className="flex items-center gap-2">
                        <span>{b.name}</span>
                        {b.is_current && <span className="text-[10px] text-editor-success">(current)</span>}
                    </div>
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
                    <div className="flex items-center gap-2">
                        <span>{b.name}</span>
                        {b.is_current && <span className="text-[10px] text-editor-success">(current)</span>}
                    </div>
                    {head === b.name && <Check size={12} className="text-editor-success" />}
                </div>
             ))}
          </div>
        </div>

      </div>
      </>
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
            onClick={() => {
              console.log('[BranchCompareModal] Applying comparison - base:', base, 'head:', head);
              onApply(base, head);
            }}
            disabled={loading || repoLoading}
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