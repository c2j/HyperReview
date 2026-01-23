
import React, { useState, useEffect } from 'react';
import { FolderOpen, GitBranch, ChevronDown, Tag, PanelLeft, PanelRight, ArrowLeft, Plus, Cpu } from 'lucide-react';
import { useTranslation } from '../i18n';
import { getTags } from '../api/client';
import type { Tag as TagType } from '../api/types';

interface LocalToolBarProps {
  onAction: (msg: string) => void;
  onOpenRepo: () => void;
  onNewTask: () => void;
  showLeft: boolean;
  showRight: boolean;
  onToggleLeft: () => void;
  onToggleRight: () => void;
  diffContext: { base: string; head: string };
}

const LocalToolBar: React.FC<LocalToolBarProps> = ({ 
  onAction, onOpenRepo, onNewTask, showLeft, showRight, onToggleLeft, onToggleRight, diffContext 
}) => {
  const { t } = useTranslation();
  const [tags, setTags] = useState<TagType[]>([]);

  useEffect(() => {
    getTags().then(setTags).catch(console.error);
  }, []);

  return (
    <div className="h-[56px] bg-editor-bg border-b border-editor-line flex flex-col shrink-0 z-40">
      <div className="flex-1 flex items-center px-3 gap-4 text-xs">
        <button onClick={onToggleLeft} className={`p-1.5 rounded transition-all ${showLeft ? 'bg-editor-line text-white shadow-inner' : 'text-gray-500 hover:bg-editor-line'}`}>
            <PanelLeft size={18} />
        </button>

        <div className="flex items-center gap-2">
            <button onClick={onOpenRepo} className="flex items-center gap-1.5 px-4 h-8 bg-editor-accent text-white rounded-lg transition-all hover:bg-blue-600 active:scale-95 font-bold shadow-md shadow-editor-accent/20">
                <FolderOpen size={14} /> {t('toolbar.open_repo')}
            </button>
            <button onClick={onNewTask} className="flex items-center gap-1.5 px-4 h-8 bg-editor-line hover:bg-editor-line/80 text-editor-fg rounded-lg transition-all font-bold">
                <Plus size={14} /> {t('toolbar.new_task')}
            </button>
        </div>

        <div className="h-4 w-[1px] bg-editor-line"></div>

        <div className="flex items-center gap-2">
            <span className="text-gray-500 hidden xl:inline font-medium uppercase text-[10px] tracking-widest">{t('toolbar.submit_to')}</span>
            <div className="flex rounded-lg overflow-hidden border border-editor-line">
                <button className="flex items-center gap-1.5 px-3 h-8 text-white transition-all font-black text-[10px] bg-editor-line hover:bg-gray-700">CODEARTS</button>
                <button className="bg-black/20 px-2 hover:bg-black/40 transition-colors border-l border-editor-line">
                    <ChevronDown size={12} className="text-gray-500" />
                </button>
            </div>
        </div>

        <div className="flex-1"></div>

        <div className="flex items-center gap-3">
             <div className="flex items-center gap-2 bg-editor-success/5 px-2.5 py-1 rounded-full border border-editor-success/20 cursor-pointer hover:bg-editor-success/10" onClick={() => onAction("AI Insights")}>
                 <Cpu size={14} className="text-editor-success" />
                 <span className="text-[10px] font-black text-editor-success uppercase">Local AI Analyzer</span>
             </div>
             <div className="flex gap-2">
                {tags.map(tag => (
                     <span key={tag.id} className="px-3 h-7 rounded-full border border-editor-line bg-editor-line/50 text-gray-300 text-[11px] font-bold flex items-center">{tag.label}</span>
                ))}
             </div>
        </div>

        <button onClick={onToggleRight} className={`p-1.5 rounded transition-all ${showRight ? 'bg-editor-line text-white shadow-inner' : 'text-gray-500 hover:bg-editor-line'}`}>
            <PanelRight size={18} />
        </button>
      </div>

      <div className="h-[24px] flex items-center px-4 gap-4 text-[10px] border-t border-editor-line bg-editor-sidebar text-gray-500">
        <div className="flex items-center gap-2">
            <span className="opacity-60 font-bold uppercase tracking-tighter">{t('toolbar.current_repo')}</span>
            <span className="font-mono text-gray-300">/home/lead/payment-service</span>
        </div>
        <div className="flex items-center gap-2 font-black px-2 py-0.5 rounded bg-editor-line/50 text-editor-accent">
            <GitBranch size={10} />
            <span>{diffContext.base}</span>
            <ArrowLeft size={10} className="opacity-50" />
            <span>{diffContext.head}</span>
        </div>
      </div>
    </div>
  );
};

export default LocalToolBar;
