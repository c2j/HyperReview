import React, { useState, useEffect } from 'react';
import {
  FolderOpen,
  Upload,
  GitBranch,
  Tag,
  PanelLeft,
  PanelRight,
  ArrowLeft,
  Loader2,
} from 'lucide-react';
import { useTranslation } from '../i18n';
import { useApiClient } from '../api/client';
import { useReviewStore } from '../store/reviewStore';
import type { Tag as TagType } from '../api/types';

interface ToolBarProps {
  onAction: (msg: string) => void;
  onOpenRepo?: () => void;
  onNewTask?: () => void;
  onImportTask?: () => void;
  showLeft?: boolean;
  showRight?: boolean;
  onToggleLeft?: () => void;
  onToggleRight?: () => void;
  diffContext?: { base: string; head: string };
}

const ToolBar: React.FC<ToolBarProps> = ({
  onAction,
  onOpenRepo,
  onNewTask: _onNewTask,
  onImportTask,
  showLeft = true,
  showRight = true,
  onToggleLeft,
  onToggleRight,
  diffContext,
}) => {
  const { t } = useTranslation();
  const { getTags } = useApiClient();
  const { currentRepo, branches } = useReviewStore();
  const [tags, setTags] = useState<TagType[]>([]);
  const [loadingTags, setLoadingTags] = useState(false);

  useEffect(() => {
    setLoadingTags(true);
    getTags()
      .then(setTags)
      .catch(console.error)
      .finally(() => setLoadingTags(false));
  }, []);

  // Helper to map generic color class to specific toolbar style
  // Assuming color is like 'bg-editor-error'
  const getTagStyle = (bgClass: string) => {
    if (bgClass.includes('error'))
      return 'bg-editor-error/20 text-editor-error border-editor-error/30 hover:bg-editor-error/30';
    if (bgClass.includes('warning'))
      return 'bg-editor-warning/20 text-editor-warning border-editor-warning/30 hover:bg-editor-warning/30';
    if (bgClass.includes('info'))
      return 'bg-editor-info/20 text-editor-info border-editor-info/30 hover:bg-editor-info/30';
    if (bgClass.includes('success'))
      return 'bg-editor-success/20 text-editor-success border-editor-success/30 hover:bg-editor-success/30';
    return 'bg-editor-line/50 text-gray-300 border-editor-line hover:bg-editor-line';
  };

  return (
    <div
      id="tour-toolbar"
      className="h-[52px] bg-editor-bg border-b border-editor-line flex flex-col shrink-0 z-40"
    >
      {/* Top Action Row */}
      <div className="flex-1 flex items-center px-2 gap-3 text-xs">
        {/* Left Toggle */}
        <button
          onClick={onToggleLeft}
          className={`p-1.5 rounded transition-colors ${showLeft ? 'bg-editor-line text-white' : 'text-gray-500 hover:bg-editor-line hover:text-gray-300'}`}
          title="Toggle Left Panel"
        >
          <PanelLeft size={16} />
        </button>

        <div className="h-4 w-[1px] bg-editor-line"></div>

        <div className="flex items-center gap-2">
          <button
            onClick={onOpenRepo ? onOpenRepo : () => onAction('Opening Repository Picker...')}
            className="flex items-center gap-1.5 px-3 py-1 bg-editor-accent/10 hover:bg-editor-accent/20 text-editor-accent rounded transition-colors active:scale-95"
          >
            <FolderOpen size={14} /> {t('toolbar.open_repo')}
          </button>
          <button
            onClick={onImportTask ? onImportTask : () => onAction('Importing Task...')}
            className="flex items-center gap-1.5 px-3 py-1 bg-editor-line hover:bg-editor-line/80 text-editor-fg rounded transition-colors active:scale-95"
          >
            <Upload size={14} /> {t('toolbar.import_task')}
          </button>
        </div>

        <div className="h-4 w-[1px] bg-editor-line"></div>

        {/* Hidden: CodeArts submission - feature not ready */}
        {/* <div className="flex items-center gap-2">
           <span className="text-gray-500 hidden xl:inline">{t('toolbar.submit_to')}</span>
           <button
             onClick={() => onAction('Selected Target: CodeArts')}
             className="flex items-center gap-1 px-2 py-0.5 bg-editor-line rounded text-white hover:bg-gray-700 transition-colors"
           >
             CodeArts <ChevronDown size={12} />
           </button>
         </div> */}

        <div className="flex-1"></div>

        <div className="flex items-center gap-2 overflow-hidden">
          <span className="text-gray-500 hidden lg:inline shrink-0">{t('toolbar.quick_tags')}</span>

          {loadingTags && <Loader2 size={12} className="animate-spin text-gray-500" />}

          <div className="flex gap-2 overflow-x-auto [&::-webkit-scrollbar]:hidden mask-linear-fade">
            {tags.slice(0, 3).map((tag) => (
              <span
                key={tag.id}
                onClick={() => onAction(`Filter: ${tag.label}`)}
                className={`flex items-center gap-1 px-2 py-0.5 rounded border cursor-pointer text-[11px] transition-colors whitespace-nowrap ${getTagStyle(tag.color)}`}
              >
                <Tag size={10} /> {tag.label}
              </span>
            ))}
          </div>

          <span
            onClick={() => onAction('Opening Tag Manager...')}
            className="flex items-center gap-1 px-2 py-0.5 bg-editor-line text-gray-400 rounded cursor-pointer text-[11px] hover:text-white transition-colors shrink-0"
          >
            <Tag size={10} /> {t('toolbar.manage')}
          </span>
        </div>

        <div className="h-4 w-[1px] bg-editor-line ml-1"></div>

        {/* Right Toggle */}
        <button
          onClick={onToggleRight}
          className={`p-1.5 rounded transition-colors ${showRight ? 'bg-editor-line text-white' : 'text-gray-500 hover:bg-editor-line hover:text-gray-300'}`}
          title="Toggle Right Panel"
        >
          <PanelRight size={16} />
        </button>
      </div>

      {/* Bottom Info Row */}
      <div className="h-[24px] bg-editor-sidebar flex items-center px-4 gap-4 text-[11px] text-gray-400 border-t border-editor-line">
        <div
          className="flex items-center gap-2 cursor-pointer hover:text-white transition-colors"
          onClick={() =>
            onAction(currentRepo ? `Navigating to ${currentRepo.path}` : 'No repository loaded')
          }
        >
          <span className="text-gray-500">{t('toolbar.current_repo')}</span>
          <span className="text-editor-fg font-medium">{currentRepo?.path || 'No repository'}</span>
        </div>

        {/* Dynamic Branch Display */}
        {diffContext && (
          <div
            className="flex items-center gap-1 text-editor-accent cursor-pointer hover:text-blue-300 transition-colors bg-editor-line/30 px-2 rounded"
            onClick={() => {
              console.log('[ToolBar] Branch area clicked, diffContext:', diffContext);
              onAction('Switching Branch...');
            }}
          >
            <GitBranch size={10} />
            <span className="text-editor-error/80 truncate max-w-[100px]">{diffContext.base}</span>
            <ArrowLeft size={10} className="text-gray-500" />
            <span className="text-editor-success/80 truncate max-w-[100px]">
              {diffContext.head}
            </span>
          </div>
        )}
        {!diffContext && (
          <div
            className="flex items-center gap-1 text-editor-accent cursor-pointer hover:text-blue-300 transition-colors"
            onClick={() => {
              console.log('[ToolBar] Branch area clicked (no diffContext)');
              onAction('Switching Branch...');
            }}
          >
            <GitBranch size={10} />
            <span>
              {currentRepo?.current_branch || branches.find((b) => b.is_current)?.name || 'main'}
            </span>
          </div>
        )}

        {/* Show current commit hash if available */}
        {currentRepo?.head_commit && (
          <div
            className="flex items-center gap-1 px-1.5 bg-editor-line rounded text-gray-300 cursor-pointer hover:bg-gray-600 transition-colors"
            onClick={() => onAction(`Current commit: ${currentRepo.head_commit}`)}
            title="Current HEAD commit"
          >
            <span className="font-mono">{currentRepo.head_commit.substring(0, 7)}</span>
          </div>
        )}

        {/* Hidden: Diff stats and pending files - needs backend implementation */}
        {/* <div className="flex-1"></div>
        <div
          className="cursor-pointer hover:text-white transition-colors"
          onClick={() => onAction('Listing pending files')}
        >
          <span className="text-white font-bold">127</span> {t('toolbar.files_pending')}
        </div> */}
      </div>
    </div>
  );
};

export default ToolBar;
