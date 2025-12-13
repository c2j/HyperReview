import React, { useState, useEffect } from 'react';
import { Search, Folder, GitBranch, Clock, ArrowRight, FolderOpen } from 'lucide-react';
import { useTranslation } from '../i18n';
import { getRecentRepos, openLocalRepoDialog } from '../api/client';
import type { Repo } from '../api/types';

interface OpenRepoModalProps {
  onClose: () => void;
  onOpen: (path: string) => void;
}

const OpenRepoModal: React.FC<OpenRepoModalProps> = ({ onClose, onOpen }) => {
  const { t } = useTranslation();
  const [selected, setSelected] = useState<string | null>(null);
  const [search, setSearch] = useState('');
  const [repos, setRepos] = useState<Repo[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    setLoading(true);
    getRecentRepos()
      .then(data => {
        setRepos(data);
      })
      .catch(console.error)
      .finally(() => setLoading(false));
  }, []);

  const filteredRepos = repos.filter(r => r.path.toLowerCase().includes(search.toLowerCase()));

  const handleBrowseLocal = async () => {
    try {
        const path = await openLocalRepoDialog();
        if (path) {
            // Check if already in list to avoid duplicates
            const existing = repos.find(r => r.path === path);
            if (!existing) {
                const newRepo: Repo = {
                    path: path,
                    branch: 'HEAD', // Default for new repo
                    lastOpened: 'Just now'
                };
                setRepos([newRepo, ...repos]);
            }
            // Auto-advance to Step 2 (Branch Selection)
            onOpen(path);
        }
    } catch (e) {
        console.error(e);
    }
  };

  return (
    <div className="flex flex-col gap-4">
      <div className="text-xs text-gray-400 font-bold mb-1">
          {t('modal.open_repo.step1')}
      </div>

      <div className="relative">
        <Search className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500" size={16} />
        <input 
          type="text" 
          value={search}
          onChange={e => setSearch(e.target.value)}
          placeholder={t('modal.open_repo.search')}
          className="w-full bg-editor-line/50 border border-editor-line rounded pl-9 pr-3 py-2 text-sm text-white placeholder-gray-500 focus:outline-none focus:border-editor-accent transition-colors"
          autoFocus
        />
      </div>

      <div className="flex flex-col gap-1 max-h-[300px] overflow-y-auto min-h-[150px]">
        <div className="flex items-center justify-between mb-1 sticky top-0 bg-editor-bg z-10 py-1">
            <span className="text-[10px] uppercase text-gray-500 font-bold">{t('modal.open_repo.recent')}</span>
            <button onClick={handleBrowseLocal} className="text-[10px] text-editor-accent hover:underline flex items-center gap-1 cursor-pointer">
                <FolderOpen size={10} /> {t('modal.open_repo.open_local')}
            </button>
        </div>
        
        {loading ? (
          <div className="flex flex-col items-center justify-center py-8 text-gray-500 gap-2">
             <div className="w-4 h-4 border-2 border-editor-line border-t-editor-accent rounded-full animate-spin"></div>
             <span className="text-xs">Loading repositories...</span>
          </div>
        ) : (
          <>
            {filteredRepos.map((repo) => (
              <div 
                key={repo.path}
                onClick={() => setSelected(repo.path)}
                onDoubleClick={() => onOpen(repo.path)}
                className={`flex items-center gap-3 p-2 rounded cursor-pointer border border-transparent transition-all ${selected === repo.path ? 'bg-editor-selection text-white border-editor-accent/50' : 'hover:bg-editor-line text-gray-300'}`}
              >
                <Folder size={16} className={selected === repo.path ? 'text-white' : 'text-editor-accent'} />
                <div className="flex flex-col flex-1 min-w-0">
                  <span className="text-xs font-medium truncate font-mono">{repo.path}</span>
                  <div className="flex items-center gap-2 text-[10px] opacity-70">
                    <span className="flex items-center gap-1"><Clock size={10} /> {repo.lastOpened}</span>
                    <span className="flex items-center gap-1 text-gray-500">|</span>
                    <span className="flex items-center gap-1"><GitBranch size={10} /> {repo.branch}</span>
                  </div>
                </div>
                {selected === repo.path && <ArrowRight size={14} className="text-editor-accent" />}
              </div>
            ))}
            {filteredRepos.length === 0 && (
                 <div className="flex flex-col items-center justify-center py-8 gap-3 text-gray-500">
                    <div className="text-xs italic">{t('modal.open_repo.empty_desc')}</div>
                    <button 
                        onClick={handleBrowseLocal}
                        className="flex items-center gap-2 px-4 py-2 bg-editor-line hover:bg-editor-line/80 text-white rounded text-xs transition-colors border border-editor-line/50"
                    >
                        <FolderOpen size={14} />
                        {t('modal.open_repo.open_local')}
                    </button>
                </div>
            )}
          </>
        )}
      </div>

      <div className="flex justify-end gap-2 pt-3 border-t border-editor-line mt-1">
        <button onClick={onClose} className="px-4 py-1.5 rounded text-xs hover:bg-editor-line text-gray-300 transition-colors">{t('modal.open_repo.cancel')}</button>
        <button 
          onClick={() => selected && onOpen(selected)}
          disabled={!selected}
          className="px-4 py-1.5 rounded text-xs bg-editor-accent text-white hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed transition-colors font-medium shadow-sm flex items-center gap-1"
        >
          {t('modal.open_repo.next')} <ArrowRight size={12} />
        </button>
      </div>
    </div>
  );
};

export default OpenRepoModal;