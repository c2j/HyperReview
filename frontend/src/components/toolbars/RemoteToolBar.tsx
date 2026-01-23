
import React, { useState, useEffect } from 'react';
import { Globe, Plug, GitBranch, ChevronDown, Tag, PanelLeft, PanelRight, ArrowLeft, Cpu, Activity } from 'lucide-react';
import { useTranslation } from '../i18n';
import { getTags } from '../api/client';
import { simpleGerritService, SimpleGerritInstance } from '../../services/gerrit-simple-service';
import type { Tag as TagType } from '../api/types';

interface RemoteToolBarProps {
  onAction: (msg: string) => void;
  showLeft: boolean;
  showRight: boolean;
  onToggleLeft: () => void;
  onToggleRight: () => void;
  diffContext: { base: string; head: string };
}

const RemoteToolBar: React.FC<RemoteToolBarProps> = ({ 
  onAction, showLeft, showRight, onToggleLeft, onToggleRight, diffContext 
}) => {
  const { t } = useTranslation();
  const [tags, setTags] = useState<TagType[]>([]);
  const [activeInstance, setActiveInstance] = useState<SimpleGerritInstance | null>(null);

  useEffect(() => {
    getTags().then(setTags).catch(console.error);
  }, []);

  useEffect(() => {
    const loadActiveInstance = async () => {
      try {
        const instances = await simpleGerritService.getInstances();
        const active = instances.find(inst => inst.is_active) || null;
        setActiveInstance(active);
      } catch (error) {
        console.error('Failed to load active Gerrit instance:', error);
      }
    };
    loadActiveInstance();
  }, []);

  return (
    <div className="h-[56px] bg-editor-bg border-b border-editor-line flex flex-col shrink-0 z-40 shadow-lg shadow-purple-900/5">
      <div className="flex-1 flex items-center px-3 gap-4 text-xs">
        <button onClick={onToggleLeft} className={`p-1.5 rounded transition-all ${showLeft ? 'bg-purple-900/40 text-purple-200' : 'text-gray-500 hover:bg-editor-line'}`}>
            <PanelLeft size={18} />
        </button>

        <div className="flex items-center gap-2">
            <button onClick={() => onAction("Gerrit Import Dialog")} className="flex items-center gap-1.5 px-4 h-8 bg-purple-600 text-white rounded-lg transition-all hover:bg-purple-500 active:scale-95 font-bold shadow-md shadow-purple-900/30">
                <Plug size={14} /> {t('toolbar.gerrit_import')}
            </button>
            <button onClick={() => onAction("Gerrit Server Settings")} className="flex items-center gap-1.5 px-3 h-8 bg-editor-line hover:bg-editor-line/80 text-gray-300 rounded-lg transition-all">
                <Globe size={14} />
            </button>
        </div>

        <div className="h-4 w-[1px] bg-editor-line"></div>

        <div className="flex items-center gap-2">
            <span className="text-gray-500 hidden xl:inline font-medium uppercase text-[10px] tracking-widest">{t('toolbar.submit_to')}</span>
            <div className="flex rounded-lg overflow-hidden border border-purple-500/20">
                <button className="flex items-center gap-1.5 px-3 h-8 text-white font-black text-[10px] bg-purple-900/60 hover:bg-purple-800 transition-all uppercase tracking-tighter">Gerrit Server</button>
                <button className="bg-purple-950/40 px-2 hover:bg-purple-900/60 transition-colors border-l border-purple-500/20">
                    <ChevronDown size={12} className="text-purple-400" />
                </button>
            </div>
        </div>

        <div className="flex-1"></div>

        <div className="flex items-center gap-3">
             <div className="flex items-center gap-2 bg-purple-500/10 px-2.5 py-1 rounded-full border border-purple-500/30 cursor-pointer animate-pulse" onClick={() => onAction("AI Insights")}>
                 <Activity size={14} className="text-purple-400" />
                 <span className="text-[10px] font-black text-purple-300 uppercase tracking-widest">Remote AI Sync</span>
             </div>
        </div>

        <button onClick={onToggleRight} className={`p-1.5 rounded transition-all ${showRight ? 'bg-purple-900/40 text-purple-200' : 'text-gray-500 hover:bg-editor-line'}`}>
            <PanelRight size={18} />
        </button>
      </div>

      <div className="h-[24px] flex items-center px-4 gap-4 text-[10px] border-t border-editor-line bg-purple-950/20 text-purple-400/80">
        <div className="flex items-center gap-2">
            <span className="opacity-60 font-bold uppercase">{t('gerrit.toolbar.gerrit_host')}:</span>
            <span className="font-mono text-purple-200">{activeInstance?.url || 'No instance configured'}</span>
        </div>
        {activeInstance && (
          <div className="flex items-center gap-3 ml-4">
             <div className="flex items-center gap-1.5 bg-purple-600/30 px-2 py-0.5 rounded font-black text-white">
                <span>{activeInstance.name}</span>
             </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default RemoteToolBar;
