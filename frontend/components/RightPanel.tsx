import React, { useState, useEffect, useMemo } from 'react';
import {
  Layers,
  History,
  PieChart,
  ListChecks,
  GripVertical,
  CheckSquare,
  Square,
  Trash2,
  Loader2,
  BookOpen,
  ExternalLink,
  Shield,
  Zap,
  Palette,
  Code,
  Filter,
} from 'lucide-react';
import { useTranslation } from '../i18n';
import { useApiClient } from '../api/client';
import type {
  HeatmapItem,
  BlameInfo,
  ReviewStats,
  ChecklistItem,
  ReviewGuideItem,
} from '../api/types';

enum Tab {
  HEATMAP = 'heatmap',
  BLAME = 'blame',
  STATS = 'stats',
  LIST = 'list',
  GUIDE = 'guide',
}

interface RightPanelProps {
  onAction: (msg: string) => void;
  activeFileExtension?: string;
  repoRefreshKey?: number;
  onSelectFile?: (path: string) => void;
  selectedFile?: string | null;
  diffContext?: { base: string; head: string };
}

const RightPanel: React.FC<RightPanelProps> = ({
  onAction,
  activeFileExtension = '',
  repoRefreshKey,
  onSelectFile,
  selectedFile,
  diffContext,
}) => {
  const { t } = useTranslation();
  const { getHeatmap, getBlame, getReviewStats, getChecklist, getReviewGuide } = useApiClient();
  const [activeTab, setActiveTab] = useState<Tab>(Tab.HEATMAP);
  const [loading, setLoading] = useState(false);

  // Data States
  const [heatmapData, setHeatmapData] = useState<HeatmapItem[]>([]);
  const [blameData, setBlameData] = useState<BlameInfo | null>(null);
  const [statsData, setStatsData] = useState<ReviewStats | null>(null);
  const [listItems, setListItems] = useState<ChecklistItem[]>([]);
  const [guideItems, setGuideItems] = useState<ReviewGuideItem[]>([]);

  // Filter State for Guide
  const [categoryFilter, setCategoryFilter] = useState<string>('all');

  // Drag State for List
  const [draggedItemIndex, setDraggedItemIndex] = useState<number | null>(null);

  // Fetch data on tab change
  useEffect(() => {
    setLoading(true);
    let promise: Promise<any>;

    switch (activeTab) {
      case Tab.HEATMAP:
        // Use diff context branches if available for diff-based heatmap
        promise = getHeatmap().then(setHeatmapData);
        break;
      case Tab.BLAME:
        promise = getBlame('current-file').then(setBlameData);
        break;
      case Tab.STATS:
        promise = getReviewStats().then(setStatsData);
        break;
      case Tab.LIST:
        if (listItems.length === 0) {
          promise = getChecklist().then(setListItems);
        } else {
          promise = Promise.resolve();
        }
        break;
      case Tab.GUIDE:
        promise = getReviewGuide().then(setGuideItems);
        break;
      default:
        promise = Promise.resolve();
    }

    promise.catch(console.error).finally(() => setLoading(false));
  }, [activeTab, repoRefreshKey, diffContext]); // 依赖 activeTab、repoRefreshKey 和 diffContext

  // Filtered Guide Items Logic
  const filteredGuideItems = useMemo(() => {
    return guideItems.filter((item) => {
      const matchesCategory = categoryFilter === 'all' || item.category === categoryFilter;
      const matchesExtension =
        activeFileExtension === '' || item.applicableExtensions.includes(activeFileExtension);
      return matchesCategory && matchesExtension;
    });
  }, [guideItems, categoryFilter, activeFileExtension]);

  // List Actions
  const toggleCheck = (id: string) => {
    setListItems((prev) =>
      prev.map((item) => (item.id === id ? { ...item, checked: !item.checked } : item)),
    );
  };

  const handleSelectAll = () => {
    const allChecked = listItems.every((i) => i.checked);
    setListItems((prev) => prev.map((item) => ({ ...item, checked: !allChecked })));
  };

  const handleInvertSelection = () => {
    setListItems((prev) => prev.map((item) => ({ ...item, checked: !item.checked })));
  };

  const handleRemoveSelected = () => {
    const count = listItems.filter((i) => i.checked).length;
    if (count === 0) return;
    setListItems((prev) => prev.filter((item) => !item.checked));
    onAction(`Removed ${count} items from pending list`);
  };

  // Drag and Drop Logic
  const handleDragStart = (index: number) => setDraggedItemIndex(index);
  const handleDragEnter = (index: number) => {
    if (draggedItemIndex === null || draggedItemIndex === index) return;
    const newItems = [...listItems];
    const draggedItem = newItems[draggedItemIndex];
    newItems.splice(draggedItemIndex, 1);
    newItems.splice(index, 0, draggedItem);
    setListItems(newItems);
    setDraggedItemIndex(index);
  };
  const handleDragEnd = () => setDraggedItemIndex(null);

  // Heatmap Statistics
  const heatmapStats = useMemo(() => {
    const stats = { high: 0, medium: 0, low: 0, total: 0, avgScore: 0 };
    if (!heatmapData.length) return stats;
    let totalScore = 0;
    heatmapData.forEach((item) => {
      stats.total++;
      // Ensure score is a valid number, default to 0 if not
      const score = typeof item.score === 'number' && !isNaN(item.score) ? item.score : 0;
      totalScore += score;
      if (item.impact === 'High') stats.high++;
      else if (item.impact === 'Medium') stats.medium++;
      else stats.low++;
    });
    stats.avgScore = stats.total > 0 ? Math.round(totalScore / stats.total) : 0;
    return stats;
  }, [heatmapData]);

  // Guide Helper - 根据中文类别返回图标
  const getCategoryIcon = (cat: string) => {
    switch (cat) {
      case '安全性':
      case 'Security':
        return <Shield size={14} className="text-editor-error" />;
      case '性能优化':
      case '性能':
      case 'Performance':
        return <Zap size={14} className="text-editor-warning" />;
      case '代码规范':
      case 'Code Style':
      case 'Style':
        return <Palette size={14} className="text-editor-info" />;
      case '通用原则':
      case '可扩展性':
      case '可维护性/可复用性':
      case '并发与防重':
      case '异常处理':
      case '多线程':
      case 'Logic':
        return <Code size={14} className="text-editor-accent" />;
      default:
        return <BookOpen size={14} />;
    }
  };

  return (
    <div
      id="tour-right-panel"
      className="h-full bg-editor-sidebar border-l border-editor-line flex flex-col"
    >
      {/* Tabs */}
      <div className="flex border-b border-editor-line bg-editor-bg shrink-0">
        {[Tab.HEATMAP, Tab.BLAME, Tab.STATS, Tab.LIST, Tab.GUIDE].map((tab) => (
          <button
            key={tab}
            onClick={() => setActiveTab(tab)}
            className={`flex-1 py-2 flex justify-center items-center border-b-2 transition-colors ${activeTab === tab ? 'border-editor-accent text-white' : 'border-transparent text-gray-500 hover:text-gray-300'}`}
            title={t(`rightpanel.tab.${tab}`)}
          >
            {tab === Tab.HEATMAP && <Layers size={16} />}
            {tab === Tab.BLAME && <History size={16} />}
            {tab === Tab.STATS && <PieChart size={16} />}
            {tab === Tab.LIST && <ListChecks size={16} />}
            {tab === Tab.GUIDE && <BookOpen size={16} />}
          </button>
        ))}
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto p-4">
        {loading && (activeTab !== Tab.LIST || listItems.length === 0) ? (
          <div className="flex flex-col items-center justify-center h-full gap-2 text-gray-500">
            <Loader2 size={24} className="animate-spin text-editor-accent" />
            <span className="text-xs">Loading data...</span>
          </div>
        ) : (
          <>
            {activeTab === Tab.HEATMAP && (
              <div className="flex flex-col h-full">
                <h3 className="text-xs font-bold text-gray-400 uppercase mb-2 shrink-0">
                  {t('rightpanel.heatmap.title')}
                </h3>
                <div className="bg-editor-line/30 p-3 rounded mb-4 shrink-0 border border-editor-line">
                  <div className="text-xs text-gray-400 mb-2 font-medium">Global Impact Score</div>
                  <div className="flex items-end gap-2 mb-3">
                    <span className="text-2xl font-bold text-white">{heatmapStats.avgScore}</span>
                    <span className="text-[10px] text-gray-500 mb-1">/ 100</span>
                  </div>
                  <div className="flex gap-1 h-2 rounded-full overflow-hidden bg-editor-bg">
                    <div
                      className="bg-editor-error"
                      style={{ width: `${(heatmapStats.high / heatmapStats.total) * 100}%` }}
                    ></div>
                    <div
                      className="bg-editor-warning"
                      style={{ width: `${(heatmapStats.medium / heatmapStats.total) * 100}%` }}
                    ></div>
                    <div
                      className="bg-editor-info"
                      style={{ width: `${(heatmapStats.low / heatmapStats.total) * 100}%` }}
                    ></div>
                  </div>
                </div>
                <div className="flex-1 overflow-y-auto space-y-1 pr-1">
                  {heatmapData.map((item) => (
                    <div
                      key={item.id}
                      onClick={() => {
                        console.log('[RightPanel] Heatmap item clicked:', item.path);
                        onSelectFile?.(item.path);
                        onAction(`Opening Diff: ${item.path}`);
                      }}
                      className={`flex items-center gap-3 p-2 rounded cursor-pointer group border-l-4 transition-colors ${
                        selectedFile === item.path
                          ? 'bg-editor-accent/30 border-editor-accent shadow-[0_0_16px_rgba(59,130,246,0.5)]'
                          : 'border-transparent hover:bg-editor-line/50 hover:border-editor-accent/50'
                      }`}
                    >
                      <div
                        className={`w-1 h-8 rounded-full shrink-0 ${item.impact === 'High' ? 'bg-editor-error shadow-[0_0_8px_rgba(241,76,76,0.6)]' : item.impact === 'Medium' ? 'bg-editor-warning' : 'bg-editor-info'}`}
                      ></div>
                      <div className="flex-1 min-w-0">
                        <div className="flex justify-between items-center mb-0.5">
                          <div className="flex items-center gap-1.5 min-w-0">
                            <span
                              className={`text-xs font-medium text-gray-300 group-hover:text-white truncate ${!item.exists ? 'opacity-60' : ''} ${
                                selectedFile === item.path ? 'text-white font-bold' : ''
                              }`}
                            >
                              {item.name}
                            </span>
                            {/* Selected indicator */}
                            {selectedFile === item.path && (
                              <span className="ml-auto mr-2 flex items-center gap-2 bg-editor-accent/20 px-2 py-0.5 rounded">
                                <span className="w-2 h-2 bg-editor-accent rounded-full animate-pulse shadow-[0_0_8px_rgba(59,130,246,1)]"></span>
                                <span className="text-[11px] text-editor-accent font-bold">
                                  OPEN
                                </span>
                              </span>
                            )}
                            {/* Mark for files that don't exist in working directory */}
                            {!item.exists && (
                              <span
                                className="text-[10px] text-gray-500 italic"
                                title="File doesn't exist in working directory"
                              >
                                ---
                              </span>
                            )}
                          </div>
                          <span
                            className={`text-[10px] font-bold ${item.impact === 'High' ? 'text-editor-error' : item.impact === 'Medium' ? 'text-editor-warning' : 'text-gray-500'}`}
                          >
                            {item.score}
                          </span>
                        </div>
                        <div
                          className={`text-[10px] text-gray-600 truncate font-mono ${!item.exists ? 'opacity-60' : ''}`}
                        >
                          {item.path}
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {activeTab === Tab.BLAME && blameData && (
              <div className="space-y-4">
                <div
                  className="flex items-center gap-2 mb-2 hover:bg-editor-line p-1 rounded cursor-pointer transition-colors"
                  onClick={() => onAction('Show Author Details')}
                >
                  <div className="w-6 h-6 rounded-full bg-blue-500 flex items-center justify-center text-[10px] font-bold text-white">
                    {blameData.avatar}
                  </div>
                  <div className="flex flex-col">
                    <span className="text-xs font-bold text-white">{blameData.author}</span>
                    <span className="text-[10px] text-gray-400">{blameData.time}</span>
                  </div>
                </div>
                <div
                  className="bg-editor-line p-2 rounded text-xs text-gray-300 border-l-2 border-editor-accent cursor-pointer hover:bg-gray-700 transition-colors"
                  onClick={() => onAction('View Linked PR')}
                >
                  {blameData.prName}
                </div>
                <div className="bg-gray-800 p-2 rounded text-[11px] text-gray-400 italic">
                  {blameData.comment}
                </div>
              </div>
            )}

            {activeTab === Tab.STATS && statsData && (
              <div className="space-y-4">
                <h3 className="text-xs font-bold text-gray-400 uppercase">
                  {t('rightpanel.stats.title')}
                </h3>
                <div className="flex flex-col gap-2">
                  <div className="flex justify-between text-xs">
                    <span className="text-gray-400">{t('rightpanel.stats.reviewed')}</span>
                    <span className="text-white">
                      {statsData.reviewedCount} / {statsData.totalCount}
                    </span>
                  </div>
                  <div className="w-full bg-editor-line h-1.5 rounded-full overflow-hidden">
                    <div
                      className="bg-editor-success h-full"
                      style={{
                        width: `${(statsData.reviewedCount / statsData.totalCount) * 100}%`,
                      }}
                    ></div>
                  </div>
                </div>
                <div className="grid grid-cols-2 gap-2 mt-4">
                  <div className="bg-editor-line p-2 rounded text-center cursor-pointer hover:bg-editor-line/80 transition-colors">
                    <div className="text-xl font-bold text-editor-error">
                      {statsData.severeCount}
                    </div>
                    <div className="text-[10px] text-gray-400 uppercase">
                      {t('rightpanel.stats.severe')}
                    </div>
                  </div>
                  <div className="bg-editor-line p-2 rounded text-center cursor-pointer hover:bg-editor-line/80 transition-colors">
                    <div className="text-xl font-bold text-editor-warning">
                      {statsData.warningCount}
                    </div>
                    <div className="text-[10px] text-gray-400 uppercase">
                      {t('rightpanel.stats.warning')}
                    </div>
                  </div>
                </div>
              </div>
            )}

            {activeTab === Tab.LIST && (
              <div className="flex flex-col h-full">
                <h3 className="text-xs font-bold text-gray-400 uppercase mb-3 shrink-0">
                  {t('rightpanel.list.title')}
                </h3>
                <div className="flex-1 overflow-y-auto space-y-1 mb-4 pr-1">
                  {listItems.length === 0 ? (
                    <div className="text-xs text-gray-500 italic text-center py-4">
                      {t('rightpanel.list.no_items')}
                    </div>
                  ) : (
                    listItems.map((item, index) => (
                      <div
                        key={item.id}
                        draggable
                        onDragStart={() => handleDragStart(index)}
                        onDragEnter={() => handleDragEnter(index)}
                        onDragEnd={handleDragEnd}
                        onDragOver={(e) => e.preventDefault()}
                        className={`flex items-center gap-2 px-2 py-1.5 rounded group transition-colors cursor-move ${item.checked ? 'bg-editor-selection/30 border border-editor-selection/50' : 'bg-editor-line/10 border border-transparent hover:bg-editor-line/50'} ${draggedItemIndex === index ? 'opacity-50 dashed border-gray-500' : ''}`}
                      >
                        <GripVertical
                          size={14}
                          className="text-gray-600 group-hover:text-gray-400 shrink-0"
                        />
                        <div
                          onClick={() => toggleCheck(item.id)}
                          className="cursor-pointer text-gray-400 hover:text-white shrink-0"
                        >
                          {item.checked ? (
                            <CheckSquare size={14} className="text-editor-accent" />
                          ) : (
                            <Square size={14} />
                          )}
                        </div>
                        <span
                          className={`text-xs font-mono truncate flex-1 ${item.checked ? 'text-white' : 'text-gray-400'}`}
                        >
                          {item.text}
                        </span>
                      </div>
                    ))
                  )}
                </div>
                <div className="shrink-0 border-t border-editor-line pt-3 flex items-center justify-between gap-2">
                  <div className="flex gap-2">
                    <button
                      onClick={handleSelectAll}
                      className="px-2 py-1 bg-editor-line hover:bg-gray-600 rounded text-[10px] text-gray-300 border border-gray-600"
                    >
                      {t('rightpanel.list.select_all')}
                    </button>
                    <button
                      onClick={handleInvertSelection}
                      className="px-2 py-1 bg-editor-line hover:bg-gray-600 rounded text-[10px] text-gray-300 border border-gray-600"
                    >
                      {t('rightpanel.list.invert')}
                    </button>
                  </div>
                  <button
                    onClick={handleRemoveSelected}
                    disabled={!listItems.some((i) => i.checked)}
                    className="px-2 py-1 bg-editor-error/10 hover:bg-editor-error/30 text-editor-error rounded text-[10px] border border-editor-error/30 flex items-center gap-1 disabled:opacity-30 disabled:cursor-not-allowed"
                  >
                    <Trash2 size={10} /> {t('rightpanel.list.remove')}
                  </button>
                </div>
              </div>
            )}

            {activeTab === Tab.GUIDE && (
              <div className="flex flex-col h-full">
                {/* Header with Filter */}
                <div className="shrink-0 mb-4">
                  <h3 className="text-xs font-bold text-gray-400 uppercase mb-3">
                    {t('rightpanel.guide.title')}
                  </h3>

                  <div className="flex items-center gap-2">
                    <div className="relative flex-1">
                      <select
                        value={categoryFilter}
                        onChange={(e) => setCategoryFilter(e.target.value)}
                        className="w-full bg-editor-line/50 border border-editor-line rounded pl-8 pr-2 py-1.5 text-[11px] text-white focus:outline-none focus:border-editor-accent appearance-none"
                      >
                        <option value="all">全部分类</option>
                        {/* 动态生成分类选项 */}
                        {Array.from(new Set(guideItems.map((item) => item.category)))
                          .sort()
                          .map((cat) => (
                            <option key={cat} value={cat}>
                              {cat}
                            </option>
                          ))}
                      </select>
                      <Filter
                        size={12}
                        className="absolute left-2.5 top-1/2 -translate-y-1/2 text-gray-500"
                      />
                    </div>

                    {/* Extension Badge Indicator */}
                    {activeFileExtension && (
                      <div className="px-2 py-1 bg-editor-accent/20 border border-editor-accent/30 rounded text-[10px] text-editor-accent font-bold">
                        {activeFileExtension}
                      </div>
                    )}
                  </div>
                </div>

                {/* Scrollable List */}
                <div className="flex-1 overflow-y-auto space-y-3 pr-1">
                  {filteredGuideItems.length === 0 ? (
                    <div className="flex flex-col items-center justify-center py-8 text-center px-4">
                      <div className="w-12 h-12 rounded-full bg-editor-line/30 flex items-center justify-center mb-3">
                        <Filter size={20} className="text-gray-600" />
                      </div>
                      <div className="text-xs text-gray-500 italic">
                        {t('rightpanel.guide.no_items')}
                      </div>
                      <button
                        onClick={() => {
                          setCategoryFilter('all');
                          onAction('Reset Filters');
                        }}
                        className="mt-3 text-[10px] text-editor-accent hover:underline"
                      >
                        Show all categories for {activeFileExtension}
                      </button>
                    </div>
                  ) : (
                    filteredGuideItems.map((guide) => (
                      <div
                        key={guide.id}
                        onClick={() => onAction(`Guide Point: ${guide.title}`)}
                        className={`p-3 rounded border bg-editor-line/10 hover:bg-editor-line/20 cursor-pointer transition-all group relative
                                        ${
                                          guide.severity === 'high'
                                            ? 'border-editor-error/30 hover:border-editor-error/50 shadow-[inset_0_0_8px_rgba(241,76,76,0.05)]'
                                            : guide.severity === 'medium'
                                              ? 'border-editor-warning/30 hover:border-editor-warning/50'
                                              : 'border-editor-line hover:border-editor-info/50'
                                        }`}
                      >
                        <div className="flex items-center justify-between mb-1.5">
                          <div className="flex items-center gap-2">
                            {getCategoryIcon(guide.category)}
                            <span
                              className={`text-[9px] uppercase font-black px-1.5 py-0.5 rounded-sm ${
                                guide.severity === 'high'
                                  ? 'bg-editor-error text-white'
                                  : guide.severity === 'medium'
                                    ? 'bg-editor-warning text-black'
                                    : 'bg-editor-info text-white'
                              }`}
                            >
                              {guide.category}
                            </span>
                          </div>
                          <ExternalLink
                            size={10}
                            className="text-gray-600 opacity-0 group-hover:opacity-100 transition-opacity"
                          />
                        </div>
                        <h4 className="text-xs font-bold text-white mb-1 group-hover:text-editor-accent transition-colors">
                          {guide.title}
                        </h4>
                        <p className="text-[11px] text-gray-400 leading-snug">
                          {guide.description}
                        </p>

                        {/* High Severity Pulse */}
                        {guide.severity === 'high' && (
                          <div className="absolute -top-1 -right-1">
                            <span className="flex h-3 w-3">
                              <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-editor-error opacity-75"></span>
                              <span className="relative inline-flex rounded-full h-3 w-3 bg-editor-error/50"></span>
                            </span>
                          </div>
                        )}
                      </div>
                    ))
                  )}

                  {/* KB Footer */}
                  <div className="mt-4 p-4 bg-black/10 border border-dashed border-editor-line rounded text-center">
                    <BookOpen size={20} className="mx-auto text-gray-600 mb-2" />
                    <h5 className="text-[11px] font-medium text-gray-400 mb-1">Knowledge Base</h5>
                    <button className="text-[10px] text-editor-accent hover:underline">
                      Full Documentation →
                    </button>
                  </div>
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
