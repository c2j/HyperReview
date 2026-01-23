
import React, { useState, useEffect } from 'react';
import { Layers, History, BookOpen, ExternalLink, Shield, Loader2, AlertCircle } from 'lucide-react';
import { useTranslation } from '../i18n';
import { getHeatmap, getBlame } from '../api/client';
import type { HeatmapItem, BlameInfo } from '../api/types';
import KnowledgeBaseContent from './KnowledgeBaseContent';

interface LocalRightPanelProps {
  onAction: (msg: string) => void;
}

enum LocalTab {
  RISK = 'risk',
  HISTORY = 'history',
  KB = 'kb'
}

const LocalRightPanel: React.FC<LocalRightPanelProps> = ({ onAction }) => {
  const { t } = useTranslation();
  const [activeTab, setActiveTab] = useState<LocalTab>(LocalTab.RISK);
  const [heatmap, setHeatmap] = useState<HeatmapItem[]>([]);
  const [blame, setBlame] = useState<BlameInfo | null>(null);

  useEffect(() => {
    getHeatmap().then(setHeatmap);
    getBlame('current').then(setBlame);
  }, []);

  return (
    <div className="h-full bg-editor-sidebar border-l border-editor-line flex flex-col overflow-hidden">
      {/* Tab Header */}
      <div className="flex border-b border-editor-line bg-editor-bg shrink-0">
          <button 
              onClick={() => setActiveTab(LocalTab.RISK)}
              className={`flex-1 py-2 flex justify-center items-center border-b-2 transition-colors ${activeTab === LocalTab.RISK ? 'border-editor-accent text-white' : 'border-transparent text-gray-500 hover:text-gray-300'}`}
              title={t('rightpanel.tab.heatmap')}
          >
              <Layers size={16} />
          </button>
          <button 
              onClick={() => setActiveTab(LocalTab.HISTORY)}
              className={`flex-1 py-2 flex justify-center items-center border-b-2 transition-colors ${activeTab === LocalTab.HISTORY ? 'border-editor-accent text-white' : 'border-transparent text-gray-500 hover:text-gray-300'}`}
              title={t('rightpanel.tab.blame')}
          >
              <History size={16} />
          </button>
          <button 
              onClick={() => setActiveTab(LocalTab.KB)}
              className={`flex-1 py-2 flex justify-center items-center border-b-2 transition-colors ${activeTab === LocalTab.KB ? 'border-editor-accent text-white' : 'border-transparent text-gray-500 hover:text-gray-300'}`}
              title={t('rightpanel.tab.kb')}
          >
              <BookOpen size={16} />
          </button>
      </div>

      <div className="flex-1 overflow-y-auto p-4 custom-scrollbar">
        {activeTab === LocalTab.RISK && (
            <section className="animate-fade-in">
                <h3 className="text-[10px] font-bold text-gray-500 uppercase tracking-widest mb-4 flex items-center gap-2">
                    <Layers size={14} className="text-editor-accent" />
                    {t('rightpanel.risk_map')}
                </h3>
                <div className="space-y-3">
                    {heatmap.map(item => (
                        <div key={item.id} className="flex items-center gap-3 p-2 bg-editor-line/20 rounded border border-transparent hover:border-editor-line cursor-pointer transition-all group">
                            <div className={`w-1 h-8 rounded-full ${item.riskLevel === 'critical' ? 'bg-red-500 shadow-[0_0_8px_rgba(239,68,68,0.4)]' : 'bg-blue-500'}`} />
                            <div className="flex-1 min-w-0">
                                <div className="text-xs font-bold text-gray-200 group-hover:text-white truncate">{item.name}</div>
                                <div className="text-[10px] text-gray-600 font-mono truncate">{item.path}</div>
                            </div>
                            <span className="text-[9px] font-black text-gray-500">I:{item.score}</span>
                        </div>
                    ))}
                </div>
            </section>
        )}

        {activeTab === LocalTab.HISTORY && (
            <section className="animate-fade-in">
                <h3 className="text-[10px] font-bold text-gray-500 uppercase tracking-widest mb-4 flex items-center gap-2">
                    <History size={14} className="text-editor-accent" />
                    {t('rightpanel.line_provenance')}
                </h3>
                {blame && (
                    <div className="bg-editor-bg border border-editor-line rounded-lg p-4 shadow-xl shadow-black/20">
                        <div className="flex items-center gap-3 mb-4">
                            <div className="w-8 h-8 rounded-full bg-editor-accent/20 flex items-center justify-center text-editor-accent font-bold">{blame.avatar}</div>
                            <div>
                                <div className="text-xs font-bold text-white leading-none mb-1">{blame.author}</div>
                                <div className="text-[10px] text-gray-500">{blame.time}</div>
                            </div>
                        </div>
                        <div className="p-2.5 bg-editor-line/40 rounded border border-editor-line text-[11px] text-gray-400 italic mb-4">
                            "{blame.comment}"
                        </div>
                        <div className="flex items-center justify-between text-[10px]">
                            <span className="text-gray-500 uppercase">{t('rightpanel.history.task')}</span>
                            <span className="text-editor-accent font-bold hover:underline cursor-pointer">{blame.prName}</span>
                        </div>
                    </div>
                )}
            </section>
        )}

        {activeTab === LocalTab.KB && (
            <KnowledgeBaseContent />
        )}
      </div>
    </div>
  );
};

export default LocalRightPanel;
