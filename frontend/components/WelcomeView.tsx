
import React, { useState, useEffect } from 'react';
import { Monitor, Globe, Clock, FolderOpen, PlugZap, ArrowRight, BookOpen, Trash2, AlertCircle, Sparkles } from 'lucide-react';
import { useTranslation } from '../i18n';
import { getRecentRepos } from '../api/client';
import type { Repo } from '../api/types';

interface WelcomeViewProps {
  onOpenLocal: () => void;
  onImportRemote: () => void;
  onSelectHistory: (path: string, mode: 'local' | 'remote') => void;
  isGerritConfigured: boolean;
}

const WelcomeView: React.FC<WelcomeViewProps> = ({ onOpenLocal, onImportRemote, onSelectHistory, isGerritConfigured }) => {
  const { t } = useTranslation();
  const [recent, setRecent] = useState<Repo[]>([]);

  useEffect(() => {
    getRecentRepos().then(setRecent).catch(console.error);
  }, []);

  return (
    <div className="flex-1 flex flex-col items-center justify-start bg-editor-bg overflow-y-auto p-8 select-none custom-scrollbar">
      <div className="max-w-4xl w-full">
        
        {/* Priority Setup Banner for First-time users */}
        {!isGerritConfigured && (
            <div className="mb-12 animate-fade-in">
                <div className="bg-purple-900/20 border border-purple-500/30 rounded-2xl p-6 flex flex-col md:flex-row items-center gap-6 backdrop-blur-md shadow-2xl">
                    <div className="w-16 h-16 rounded-2xl bg-purple-500/20 text-purple-400 flex items-center justify-center shrink-0">
                        <PlugZap size={32} className="animate-pulse" />
                    </div>
                    <div className="flex-1 text-center md:text-left">
                        <div className="flex items-center justify-center md:justify-start gap-2 mb-1">
                            <Sparkles size={14} className="text-purple-300" />
                            <h3 className="text-lg font-black text-white tracking-tight">{t('welcome.guide.title')}</h3>
                        </div>
                        <p className="text-xs text-gray-400 leading-relaxed max-w-2xl">
                            {t('welcome.guide.desc')}
                        </p>
                    </div>
                    <button 
                        onClick={onImportRemote}
                        className="px-6 py-2.5 bg-purple-600 hover:bg-purple-500 text-white rounded-xl text-sm font-black transition-all flex items-center gap-2 shadow-lg shadow-purple-900/40 whitespace-nowrap"
                    >
                        {t('welcome.guide.btn')} <ArrowRight size={18} />
                    </button>
                </div>
            </div>
        )}

        {/* Main Header */}
        <div className="text-center mb-12 animate-fade-in-down" style={{ animationDelay: '100ms' }}>
          <div className="inline-flex items-center gap-3 px-4 py-1.5 rounded-full bg-editor-accent/10 border border-editor-accent/20 text-editor-accent text-xs font-bold tracking-widest uppercase mb-4">
             <span className="relative flex h-2 w-2">
                <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-editor-accent opacity-75"></span>
                <span className="relative inline-flex rounded-full h-2 w-2 bg-editor-accent"></span>
             </span>
             {t('welcome.tag')}
          </div>
          <h1 className="text-5xl font-black text-white mb-4 tracking-tighter italic uppercase">{t('welcome.title')}</h1>
          <p className="text-gray-500 text-lg font-medium">{t('welcome.subtitle')}</p>
        </div>

        {/* Path Selection */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-12 animate-fade-in" style={{ animationDelay: '200ms' }}>
            <div 
                onClick={onOpenLocal}
                className="group relative bg-editor-sidebar border border-editor-line rounded-2xl p-8 cursor-pointer hover:border-editor-accent transition-all hover:shadow-[0_0_30px_rgba(0,122,204,0.15)] overflow-hidden"
            >
                <div className="absolute top-0 right-0 p-4 opacity-5 group-hover:opacity-10 transition-opacity">
                    <Monitor size={120} />
                </div>
                <div className="relative z-10">
                    <div className="w-12 h-12 rounded-xl bg-editor-accent/20 text-editor-accent flex items-center justify-center mb-6">
                        <Monitor size={24} />
                    </div>
                    <h3 className="text-xl font-bold text-white mb-2">{t('welcome.local_title')}</h3>
                    <p className="text-gray-500 text-sm mb-6 leading-relaxed">{t('welcome.local_desc')}</p>
                    <div className="flex items-center gap-2 text-editor-accent font-bold text-sm uppercase tracking-widest">
                        {t('welcome.local_btn')} <ArrowRight size={16} />
                    </div>
                </div>
            </div>

            <div 
                onClick={onImportRemote}
                className={`group relative bg-editor-sidebar border rounded-2xl p-8 cursor-pointer transition-all overflow-hidden
                    ${!isGerritConfigured ? 'border-purple-500/50 shadow-[0_0_20px_rgba(147,51,234,0.1)]' : 'border-editor-line hover:border-purple-500 hover:shadow-[0_0_30px_rgba(147,51,234,0.15)]'}`}
            >
                <div className="absolute top-0 right-0 p-4 opacity-5 group-hover:opacity-10 transition-opacity text-purple-400">
                    <Globe size={120} />
                </div>
                {!isGerritConfigured && (
                    <div className="absolute top-4 right-4 z-20">
                         <div className="flex items-center gap-1.5 px-2 py-0.5 rounded bg-purple-500 text-white text-[10px] font-black uppercase tracking-tighter">
                            <AlertCircle size={10} /> {t('welcome.guide.setup_needed')}
                         </div>
                    </div>
                )}
                <div className="relative z-10">
                    <div className="w-12 h-12 rounded-xl bg-purple-500/20 text-purple-400 flex items-center justify-center mb-6">
                        <Globe size={24} />
                    </div>
                    <h3 className="text-xl font-bold text-white mb-2">{t('welcome.remote_title')}</h3>
                    <p className="text-gray-500 text-sm mb-6 leading-relaxed">{t('welcome.remote_desc')}</p>
                    <div className="flex items-center gap-2 text-purple-400 font-bold text-sm uppercase tracking-widest">
                        {t('welcome.remote_btn')} <ArrowRight size={16} />
                    </div>
                </div>
            </div>
        </div>

        {/* History Section */}
        {recent.length > 0 && (
            <div className="animate-fade-in" style={{ animationDelay: '300ms' }}>
                <div className="flex items-center justify-between mb-4 px-2">
                    <h4 className="text-xs font-bold text-gray-500 uppercase tracking-widest flex items-center gap-2">
                        <Clock size={14} /> {t('welcome.history_title')}
                    </h4>
                    <button className="text-[10px] text-gray-600 hover:text-red-400 flex items-center gap-1 transition-colors">
                        <Trash2 size={12} /> {t('welcome.history_clear')}
                    </button>
                </div>
                <div className="bg-editor-sidebar border border-editor-line rounded-xl divide-y divide-editor-line overflow-hidden shadow-xl">
                    {recent.map((repo, idx) => (
                        <div 
                            key={repo.path}
                            onClick={() => onSelectHistory(repo.path, idx === 0 ? 'remote' : 'local')}
                            className="flex items-center justify-between p-4 hover:bg-editor-line/40 cursor-pointer transition-all group"
                        >
                            <div className="flex items-center gap-4 min-w-0">
                                <div className={`shrink-0 p-2 rounded-lg transition-transform group-hover:scale-110 ${idx === 0 ? 'bg-purple-500/10 text-purple-400' : 'bg-editor-accent/10 text-editor-accent'}`}>
                                    {idx === 0 ? <PlugZap size={16} /> : <FolderOpen size={16} />}
                                </div>
                                <div className="truncate">
                                    <div className="text-sm font-bold text-gray-200 group-hover:text-white truncate font-mono tracking-tight">{repo.path}</div>
                                    <div className="text-[10px] text-gray-600 flex items-center gap-2 mt-0.5">
                                        <span className="font-medium">{t('welcome.history_last_opened', { time: repo.lastOpened })}</span>
                                        <span className="opacity-30">•</span>
                                        <span className="text-gray-500 font-mono italic">{repo.branch}</span>
                                    </div>
                                </div>
                            </div>
                            <ArrowRight size={14} className="text-gray-700 group-hover:text-gray-400 group-hover:translate-x-1 transition-all" />
                        </div>
                    ))}
                </div>
            </div>
        )}
      </div>
    </div>
  );
};

export default WelcomeView;
