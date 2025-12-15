import React, { useState, useEffect } from 'react';
import { Layers, History, PieChart, ListChecks, GripVertical, CheckSquare, Square, Trash2, Loader2 } from 'lucide-react';
import { useTranslation } from '../i18n';
import { useApiClient } from '../api/client';
import { useRepositoryStatus } from '../hooks/useRepository';
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
  const { getHeatmap, getBlame, getReviewStats, getChecklist } = useApiClient();
  const { isRepoLoaded, getRepositoryInfo } = useRepositoryStatus();
  const [activeTab, setActiveTab] = useState<Tab>(Tab.HEATMAP);
  const [loading, setLoading] = useState(false);

  // Data States
  const [heatmapData, setHeatmapData] = useState<HeatmapItem[]>([]);
  const [blameData, setBlameData] = useState<BlameInfo | null>(null);
  const [statsData, setStatsData] = useState<ReviewStats | null>(null);
  const [listItems, setListItems] = useState<ChecklistItem[]>([]);

  // Clear all data when repository changes
  const repoInfo = getRepositoryInfo();
  const currentRepoPath = repoInfo?.path;

  useEffect(() => {
    if (currentRepoPath) {
      // Repository changed, clear all data
      setHeatmapData([]);
      setBlameData(null);
      setStatsData(null);
      setListItems([]);
    }
  }, [currentRepoPath]);

  // Initial load check
  useEffect(() => {
    if (isRepoLoaded && activeTab === Tab.HEATMAP) {
      // Only fetch if we don't have data yet for this repo
      if (heatmapData.length === 0) {
        setLoading(true);
        getHeatmap()
          .then(data => {
            setHeatmapData(data);
            setLoading(false);
          })
          .catch(err => {
            console.error('Initial heatmap fetch failed:', err);
            setLoading(false);
          });
      }
    }
  }, [isRepoLoaded, heatmapData.length, activeTab, currentRepoPath]);

  // Drag State
  const [draggedItemIndex, setDraggedItemIndex] = useState<number | null>(null);

  // Fetch data on tab change
  useEffect(() => {
    if (!isRepoLoaded || !currentRepoPath) {
      // No repository loaded, skip loading
      setLoading(false);
      return;
    }

    setLoading(true);
    let promise: Promise<any>;

    switch (activeTab) {
        case Tab.HEATMAP:
            promise = getHeatmap()
                .then(setHeatmapData)
                .catch(err => {
                    console.error('Failed to load heatmap:', err);
                    setHeatmapData([]);
                });
            break;
        case Tab.BLAME:
            promise = getBlame('current-file').then(setBlameData);
            break;
        case Tab.STATS:
            promise = getReviewStats().then(setStatsData);
            break;
        case Tab.LIST:
            // Always reload checklist when switching repositories to get fresh data
            promise = getChecklist('current-file')
                .then(setListItems)
                .catch(err => {
                    console.error('Failed to load checklist:', err);
                    setListItems([]);
                });
            break;
        default:
            promise = Promise.resolve();
    }

    promise.catch(console.error).finally(() => setLoading(false));
  }, [activeTab, isRepoLoaded, currentRepoPath]);


  // List Actions
  const toggleCheck = (id: string) => {
    setListItems(prev => prev.map(item => 
        item.id === id ? { ...item, is_checked: !item.is_checked } : item
    ));
  };

  const handleSelectAll = () => {
    const allChecked = listItems.every(i => i.is_checked);
    setListItems(prev => prev.map(item => ({ ...item, is_checked: !allChecked })));
    onAction(allChecked ? "Unselected All" : "Selected All");
  };

  const handleInvertSelection = () => {
    setListItems(prev => prev.map(item => ({ ...item, is_checked: !item.is_checked })));
    onAction("Inverted Selection");
  };

  const handleRemoveSelected = () => {
    const count = listItems.filter(i => i.is_checked).length;
    if (count === 0) return;
    setListItems(prev => prev.filter(item => !item.is_checked));
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
                    {!isRepoLoaded && (
                        <div className="text-yellow-500 text-xs mb-2">⚠️ No repository loaded</div>
                    )}

                    {heatmapData.length === 0 ? (
                        <div className="text-gray-500 text-xs italic text-center py-8">
                            {isRepoLoaded ? 'No heatmap data available' : 'Load a repository to view heatmap'}
                        </div>
                    ) : (
                        <div className="grid grid-cols-1 gap-2">
                            {heatmapData.map((item) => {
                                const categoryColors = {
                                    High: 'border-editor-error/50 bg-editor-error/10',
                                    Medium: 'border-editor-warning/50 bg-editor-warning/10',
                                    Low: 'border-editor-info/50 bg-editor-info/10'
                                };

                                const categoryTextColors = {
                                    High: 'text-editor-error',
                                    Medium: 'text-editor-warning',
                                    Low: 'text-editor-info'
                                };

                                return (
                                    <div
                                        key={item.file_path}
                                        className={`p-3 rounded border cursor-pointer transition-all hover:opacity-80 ${categoryColors[item.category]}`}
                                        onClick={() => onAction(`FILE_SELECTED:${item.file_path}`)}
                                    >
                                        <div className="flex justify-between items-start">
                                            <div className="flex-1 min-w-0">
                                                <div className="text-xs font-mono text-white truncate">
                                                    {item.file_path}
                                                </div>
                                                <div className="flex items-center gap-2 mt-1">
                                                    <span className={`text-[10px] uppercase font-bold ${categoryTextColors[item.category]}`}>
                                                        {item.category}
                                                    </span>
                                                    <span className="text-[10px] text-gray-500">
                                                        Changed {item.change_frequency} times
                                                    </span>
                                                </div>
                                            </div>
                                            <div className="text-right ml-2">
                                                <div className="text-[10px] text-gray-400">
                                                    Impact
                                                </div>
                                                <div className={`text-xs font-bold ${categoryTextColors[item.category]}`}>
                                                    {Math.round(item.impact_score * 100)}%
                                                </div>
                                            </div>
                                        </div>

                                        {/* Progress bar showing impact */}
                                        <div className="mt-2 h-1 bg-editor-line rounded-full overflow-hidden">
                                            <div
                                                className={`h-full ${categoryTextColors[item.category].replace('text-', 'bg-')}`}
                                                style={{ width: `${item.impact_score * 100}%` }}
                                            />
                                        </div>
                                    </div>
                                );
                            })}
                        </div>
                    )}
                </div>
            )}

            {activeTab === Tab.BLAME && blameData && (
                 <div className="space-y-4">
                    <div className="flex items-center gap-2 mb-2 hover:bg-editor-line p-1 rounded cursor-pointer transition-colors" onClick={() => onAction("Show Author Details")}>
                        <div className="w-6 h-6 rounded-full bg-blue-500 flex items-center justify-center text-[10px] font-bold text-white">{blameData.lines[0]?.author_name?.[0] || '?'}</div>
                        <div className="flex flex-col">
                            <span className="text-xs font-bold text-white">{blameData.lines[0]?.author_name || 'Unknown'}</span>
                            <span className="text-[10px] text-gray-400">{blameData.lines[0]?.commit_date || ''}</span>
                        </div>
                    </div>
                    <div className="bg-editor-line p-2 rounded text-xs text-gray-300 border-l-2 border-editor-accent cursor-pointer hover:bg-gray-700 transition-colors" onClick={() => onAction("View Linked PR")}>
                        {blameData.lines[0]?.commit_message?.substring(0, 50) || 'No commit message'}
                    </div>
                    <div className="text-[11px] text-gray-500">
                        Last changed in commit <span className="text-editor-accent">{blameData.lines[0]?.commit_oid?.substring(0, 7)}</span>
                    </div>
                    <div className="bg-gray-800 p-2 rounded text-[11px] text-gray-400 italic">
                        {blameData.lines[0]?.content?.substring(0, 100)}
                    </div>
                 </div>
            )}

            {activeTab === Tab.STATS && statsData && (
                 <div className="space-y-4">
                    <h3 className="text-xs font-bold text-gray-400 uppercase">{t('rightpanel.stats.title')}</h3>
                    <div className="flex flex-col gap-2">
                         <div className="flex justify-between text-xs">
                             <span className="text-gray-400">{t('rightpanel.stats.reviewed')}</span>
                             <span className="text-white">{statsData.reviewed_files} / {statsData.total_files}</span>
                         </div>
                         <div className="w-full bg-editor-line h-1.5 rounded-full overflow-hidden">
                             <div className="bg-editor-success h-full" style={{width: `${(statsData.reviewed_files / statsData.total_files) * 100}%`}}></div>
                         </div>
                    </div>

                    <div className="grid grid-cols-2 gap-2 mt-4">
                        <div className="bg-editor-line p-2 rounded text-center cursor-pointer hover:bg-editor-line/80 transition-colors" onClick={() => onAction("Filter Severe Issues")}>
                            <div className="text-xl font-bold text-editor-error">{statsData.severe_issues}</div>
                            <div className="text-[10px] text-gray-400 uppercase">{t('rightpanel.stats.severe')}</div>
                        </div>
                        <div className="bg-editor-line p-2 rounded text-center cursor-pointer hover:bg-editor-line/80 transition-colors" onClick={() => onAction("Filter Warnings")}>
                            <div className="text-xl font-bold text-editor-warning">{statsData.total_comments - statsData.severe_issues}</div>
                            <div className="text-[10px] text-gray-400 uppercase">{t('rightpanel.stats.warning')}</div>
                        </div>
                        <div className="bg-editor-line p-2 rounded text-center cursor-pointer hover:bg-editor-line/80 transition-colors" onClick={() => onAction("Filter Pending Replies")}>
                            <div className="text-xl font-bold text-editor-info">{statsData.pending_files}</div>
                            <div className="text-[10px] text-gray-400 uppercase">{t('rightpanel.stats.pending')}</div>
                        </div>
                        <div className="bg-editor-line p-2 rounded text-center">
                            <div className="text-xl font-bold text-white">{statsData.estimated_time_remaining || 0}m</div>
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
                                        ${item.is_checked ? 'bg-editor-selection/30 border border-editor-selection/50' : 'bg-editor-line/10 border border-transparent hover:bg-editor-line/50'}
                                        ${draggedItemIndex === index ? 'opacity-50 dashed border-gray-500' : ''}`}
                                >
                                    <GripVertical size={14} className="text-gray-600 group-hover:text-gray-400 shrink-0" />
                                    <div onClick={() => toggleCheck(item.id)} className="cursor-pointer text-gray-400 hover:text-white shrink-0">
                                        {item.is_checked ? <CheckSquare size={14} className="text-editor-accent" /> : <Square size={14} />}
                                    </div>
                                    <span className={`text-xs font-mono truncate cursor-text select-text flex-1 ${item.is_checked ? 'text-white' : 'text-gray-400'}`}>
                                        {item.description}
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
                         <button onClick={handleRemoveSelected} disabled={!listItems.some(i => i.is_checked)} className="px-2 py-1 bg-editor-error/10 hover:bg-editor-error/30 text-editor-error rounded text-[10px] transition-colors border border-editor-error/30 flex items-center gap-1 disabled:opacity-30 disabled:cursor-not-allowed">
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