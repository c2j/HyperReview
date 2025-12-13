import React, { useState, useEffect } from 'react';
import { Layers, History, PieChart, ListChecks, GripVertical, CheckSquare, Square, Trash2, Loader2 } from 'lucide-react';
import { useTranslation } from '../i18n';
import { getHeatmap, getBlame, getReviewStats, getChecklist } from '../api/client';
import type { HeatmapItem, BlameInfo, ReviewStats, ChecklistItem } from '../api/types';

enum Tab {
  HEATMAP = 'heatmap',
  BLAME = 'blame',
  STATS = 'stats',
  LIST = 'list'
}

interface RightPanelProps {
  onAction: (msg: string) => void;
}

const RightPanel: React.FC<RightPanelProps> = ({ onAction }) => {
  const { t } = useTranslation();
  const [activeTab, setActiveTab] = useState<Tab>(Tab.HEATMAP);
  const [loading, setLoading] = useState(false);

  // Data States
  const [heatmapData, setHeatmapData] = useState<HeatmapItem[]>([]);
  const [blameData, setBlameData] = useState<BlameInfo | null>(null);
  const [statsData, setStatsData] = useState<ReviewStats | null>(null);
  const [listItems, setListItems] = useState<ChecklistItem[]>([]);

  // Drag State
  const [draggedItemIndex, setDraggedItemIndex] = useState<number | null>(null);

  // Fetch data on tab change
  useEffect(() => {
    setLoading(true);
    let promise: Promise<any>;

    switch (activeTab) {
        case Tab.HEATMAP:
            promise = getHeatmap().then(setHeatmapData);
            break;
        case Tab.BLAME:
            promise = getBlame('current-file').then(setBlameData);
            break;
        case Tab.STATS:
            promise = getReviewStats().then(setStatsData);
            break;
        case Tab.LIST:
            // Only load checklist if empty to preserve local edits
            if (listItems.length === 0) {
                promise = getChecklist().then(setListItems);
            } else {
                promise = Promise.resolve();
            }
            break;
        default:
            promise = Promise.resolve();
    }

    promise.catch(console.error).finally(() => setLoading(false));
  }, [activeTab]);


  // List Actions
  const toggleCheck = (id: string) => {
    setListItems(prev => prev.map(item => 
        item.id === id ? { ...item, checked: !item.checked } : item
    ));
  };

  const handleSelectAll = () => {
    const allChecked = listItems.every(i => i.checked);
    setListItems(prev => prev.map(item => ({ ...item, checked: !allChecked })));
    onAction(allChecked ? "Unselected All" : "Selected All");
  };

  const handleInvertSelection = () => {
    setListItems(prev => prev.map(item => ({ ...item, checked: !item.checked })));
    onAction("Inverted Selection");
  };

  const handleRemoveSelected = () => {
    const count = listItems.filter(i => i.checked).length;
    if (count === 0) return;
    setListItems(prev => prev.filter(item => !item.checked));
    onAction(`Removed ${count} items from pending list`);
  };

  // Drag and Drop Logic
  const handleDragStart = (index: number) => {
    setDraggedItemIndex(index);
  };

  const handleDragEnter = (index: number) => {
    if (draggedItemIndex === null) return;
    if (draggedItemIndex === index) return;

    const newItems = [...listItems];
    const draggedItem = newItems[draggedItemIndex];
    newItems.splice(draggedItemIndex, 1);
    newItems.splice(index, 0, draggedItem);
    
    setListItems(newItems);
    setDraggedItemIndex(index);
  };

  const handleDragEnd = () => {
    setDraggedItemIndex(null);
    onAction("Reordered Pending List");
  };

  return (
    <div id="tour-right-panel" className="h-full bg-editor-sidebar border-l border-editor-line flex flex-col">
        {/* Tabs */}
        <div className="flex border-b border-editor-line bg-editor-bg shrink-0">
            <button 
                onClick={() => setActiveTab(Tab.HEATMAP)}
                className={`flex-1 py-2 flex justify-center items-center border-b-2 transition-colors ${activeTab === Tab.HEATMAP ? 'border-editor-accent text-white' : 'border-transparent text-gray-500 hover:text-gray-300'}`}
                title={t('rightpanel.tab.heatmap')}
            >
                <Layers size={16} />
            </button>
            <button 
                onClick={() => setActiveTab(Tab.BLAME)}
                className={`flex-1 py-2 flex justify-center items-center border-b-2 transition-colors ${activeTab === Tab.BLAME ? 'border-editor-accent text-white' : 'border-transparent text-gray-500 hover:text-gray-300'}`}
                title={t('rightpanel.tab.blame')}
            >
                <History size={16} />
            </button>
            <button 
                onClick={() => setActiveTab(Tab.STATS)}
                className={`flex-1 py-2 flex justify-center items-center border-b-2 transition-colors ${activeTab === Tab.STATS ? 'border-editor-accent text-white' : 'border-transparent text-gray-500 hover:text-gray-300'}`}
                title={t('rightpanel.tab.stats')}
            >
                <PieChart size={16} />
            </button>
             <button 
                onClick={() => setActiveTab(Tab.LIST)}
                className={`flex-1 py-2 flex justify-center items-center border-b-2 transition-colors ${activeTab === Tab.LIST ? 'border-editor-accent text-white' : 'border-transparent text-gray-500 hover:text-gray-300'}`}
                title={t('rightpanel.tab.list')}
            >
                <ListChecks size={16} />
            </button>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto p-4">
            {loading && listItems.length === 0 ? (
                 <div className="flex flex-col items-center justify-center h-full gap-2 text-gray-500">
                    <Loader2 size={24} className="animate-spin text-editor-accent" />
                    <span className="text-xs">Loading data...</span>
                 </div>
            ) : (
            <>
            {activeTab === Tab.HEATMAP && (
                <div className="space-y-4">
                    <h3 className="text-xs font-bold text-gray-400 uppercase">{t('rightpanel.heatmap.title')}</h3>
                    <div className="text-[11px] text-gray-500 mb-2">{t('rightpanel.heatmap.desc')}</div>
                    
                    <div className="grid grid-cols-2 gap-2">
                        {heatmapData.map(item => {
                            const borderColor = item.impact === 'high' ? 'border-editor-error/40' : item.impact === 'medium' ? 'border-editor-error/20' : 'border-gray-600';
                            const opacity = item.impact === 'high' ? 'opacity-20' : item.impact === 'medium' ? 'opacity-10' : 'opacity-0';
                            const textColor = item.impact === 'low' ? 'text-gray-300' : 'text-white';
                            
                            return (
                                <div key={item.id} className={`bg-editor-line p-3 rounded border ${borderColor} relative overflow-hidden group cursor-pointer hover:border-white transition-colors`}
                                     onClick={() => onAction(`Focus: ${item.name} Module`)}>
                                    {item.impact !== 'low' && <div className={`absolute inset-0 bg-editor-error ${opacity}`}></div>}
                                    <span className={`font-bold relative ${textColor}`}>{item.name}</span>
                                    <div className="mt-2 text-[10px] text-gray-400 relative">
                                        {item.impact === 'high' ? t('rightpanel.heatmap.impact.high') : 
                                         item.impact === 'medium' ? t('rightpanel.heatmap.impact.medium') : 
                                         t('rightpanel.heatmap.impact.low')}
                                    </div>
                                </div>
                            );
                        })}
                    </div>
                </div>
            )}

            {activeTab === Tab.BLAME && blameData && (
                 <div className="space-y-4">
                    <div className="flex items-center gap-2 mb-2 hover:bg-editor-line p-1 rounded cursor-pointer transition-colors" onClick={() => onAction("Show Author Details")}>
                        <div className="w-6 h-6 rounded-full bg-blue-500 flex items-center justify-center text-[10px] font-bold text-white">{blameData.avatar}</div>
                        <div className="flex flex-col">
                            <span className="text-xs font-bold text-white">{blameData.author}</span>
                            <span className="text-[10px] text-gray-400">{blameData.time}</span>
                        </div>
                    </div>
                    <div className="bg-editor-line p-2 rounded text-xs text-gray-300 border-l-2 border-editor-accent cursor-pointer hover:bg-gray-700 transition-colors" onClick={() => onAction("View Linked PR")}>
                        {blameData.prName}
                    </div>
                    <div className="text-[11px] text-gray-500">
                        {t('rightpanel.blame.reviewer')}: <span className="text-editor-accent cursor-pointer hover:underline">{blameData.reviewer}</span> ({blameData.reviewerStatus})
                    </div>
                    <div className="bg-gray-800 p-2 rounded text-[11px] text-gray-400 italic">
                        {blameData.comment}
                    </div>
                 </div>
            )}

            {activeTab === Tab.STATS && statsData && (
                 <div className="space-y-4">
                    <h3 className="text-xs font-bold text-gray-400 uppercase">{t('rightpanel.stats.title')}</h3>
                    <div className="flex flex-col gap-2">
                         <div className="flex justify-between text-xs">
                             <span className="text-gray-400">{t('rightpanel.stats.reviewed')}</span>
                             <span className="text-white">{statsData.reviewedCount} / {statsData.totalCount}</span>
                         </div>
                         <div className="w-full bg-editor-line h-1.5 rounded-full overflow-hidden">
                             <div className="bg-editor-success h-full" style={{width: `${(statsData.reviewedCount / statsData.totalCount) * 100}%`}}></div>
                         </div>
                    </div>

                    <div className="grid grid-cols-2 gap-2 mt-4">
                        <div className="bg-editor-line p-2 rounded text-center cursor-pointer hover:bg-editor-line/80 transition-colors" onClick={() => onAction("Filter Severe Issues")}>
                            <div className="text-xl font-bold text-editor-error">{statsData.severeCount}</div>
                            <div className="text-[10px] text-gray-400 uppercase">{t('rightpanel.stats.severe')}</div>
                        </div>
                        <div className="bg-editor-line p-2 rounded text-center cursor-pointer hover:bg-editor-line/80 transition-colors" onClick={() => onAction("Filter Warnings")}>
                            <div className="text-xl font-bold text-editor-warning">{statsData.warningCount}</div>
                            <div className="text-[10px] text-gray-400 uppercase">{t('rightpanel.stats.warning')}</div>
                        </div>
                        <div className="bg-editor-line p-2 rounded text-center cursor-pointer hover:bg-editor-line/80 transition-colors" onClick={() => onAction("Filter Pending Replies")}>
                            <div className="text-xl font-bold text-editor-info">{statsData.pendingCount}</div>
                            <div className="text-[10px] text-gray-400 uppercase">{t('rightpanel.stats.pending')}</div>
                        </div>
                        <div className="bg-editor-line p-2 rounded text-center">
                            <div className="text-xl font-bold text-white">{statsData.estimatedTime}</div>
                            <div className="text-[10px] text-gray-400 uppercase">{t('rightpanel.stats.time')}</div>
                        </div>
                    </div>
                 </div>
            )}
             
            {activeTab === Tab.LIST && (
                 <div className="flex flex-col h-full">
                     <h3 className="text-xs font-bold text-gray-400 uppercase mb-3 shrink-0">{t('rightpanel.list.title')}</h3>
                     
                     {/* Draggable List */}
                     <div className="flex-1 overflow-y-auto space-y-1 mb-4 pr-1">
                         {listItems.length === 0 ? (
                             <div className="text-xs text-gray-500 italic text-center py-4">{t('rightpanel.list.no_items')}</div>
                         ) : (
                             listItems.map((item, index) => (
                                <div 
                                    key={item.id}
                                    draggable
                                    onDragStart={() => handleDragStart(index)}
                                    onDragEnter={() => handleDragEnter(index)}
                                    onDragEnd={handleDragEnd}
                                    onDragOver={(e) => e.preventDefault()}
                                    className={`flex items-center gap-2 px-2 py-1.5 rounded group transition-colors cursor-move 
                                        ${item.checked ? 'bg-editor-selection/30 border border-editor-selection/50' : 'bg-editor-line/10 border border-transparent hover:bg-editor-line/50'}
                                        ${draggedItemIndex === index ? 'opacity-50 dashed border-gray-500' : ''}`}
                                >
                                    <GripVertical size={14} className="text-gray-600 group-hover:text-gray-400 shrink-0" />
                                    <div onClick={() => toggleCheck(item.id)} className="cursor-pointer text-gray-400 hover:text-white shrink-0">
                                        {item.checked ? <CheckSquare size={14} className="text-editor-accent" /> : <Square size={14} />}
                                    </div>
                                    <span className={`text-xs font-mono truncate cursor-text select-text flex-1 ${item.checked ? 'text-white' : 'text-gray-400'}`}>
                                        {item.text}
                                    </span>
                                </div>
                             ))
                         )}
                     </div>

                     {/* Action Buttons */}
                     <div className="shrink-0 border-t border-editor-line pt-3 flex items-center justify-between gap-2">
                         <div className="flex gap-2">
                             <button onClick={handleSelectAll} className="px-2 py-1 bg-editor-line hover:bg-gray-600 rounded text-[10px] text-gray-300 transition-colors border border-gray-600">
                                 {t('rightpanel.list.select_all')}
                             </button>
                             <button onClick={handleInvertSelection} className="px-2 py-1 bg-editor-line hover:bg-gray-600 rounded text-[10px] text-gray-300 transition-colors border border-gray-600">
                                 {t('rightpanel.list.invert')}
                             </button>
                         </div>
                         <button onClick={handleRemoveSelected} disabled={!listItems.some(i => i.checked)} className="px-2 py-1 bg-editor-error/10 hover:bg-editor-error/30 text-editor-error rounded text-[10px] transition-colors border border-editor-error/30 flex items-center gap-1 disabled:opacity-30 disabled:cursor-not-allowed">
                             <Trash2 size={10} /> {t('rightpanel.list.remove')}
                         </button>
                     </div>
                 </div>
            )}
            </>
            )}
        </div>
    </div>
  );
};

export default RightPanel;