import React, { useState, useEffect, useRef, useMemo } from 'react';
import { ReviewSeverity } from '../api/types';
import type { DiffLine, ReviewTemplate } from '../api/types';
import { getFileDiff, getReviewTemplates } from '../api/client';
import { AlertTriangle, XCircle, ChevronDown, Maximize2, Minimize2, Search, X, ArrowUp, ArrowDown, HelpCircle, Check, Wand, ChevronRight, Eye, Package, Box, MoreHorizontal, WrapText, Loader2 } from 'lucide-react';
import { useTranslation } from '../i18n';

interface DiffViewProps {
  isMaximized: boolean;
  toggleMaximize: () => void;
  onAction?: (msg: string) => void;
  diffContext?: { base: string; head: string };
}

type ViewMode = 'diff' | 'old' | 'new';

const DiffView: React.FC<DiffViewProps> = ({ isMaximized, toggleMaximize, onAction, diffContext }) => {
  const { t } = useTranslation();
  
  // Data State
  const [diffLines, setDiffLines] = useState<DiffLine[]>([]);
  const [templates, setTemplates] = useState<ReviewTemplate[]>([]);
  const [loading, setLoading] = useState(true);

  // View State
  const [viewMode, setViewMode] = useState<ViewMode>('diff');
  const [isLineWrap, setIsLineWrap] = useState(false);
  
  // Folding State
  const [foldImports, setFoldImports] = useState(false);
  const [foldLombok, setFoldLombok] = useState(false);

  // Search State
  const [searchOpen, setSearchOpen] = useState(false);
  const [searchTerm, setSearchTerm] = useState('');
  const [matches, setMatches] = useState<{lineIndex: number, start: number, end: number}[]>([]);
  const [currentMatchIdx, setCurrentMatchIdx] = useState(0);
  const searchInputRef = useRef<HTMLInputElement>(null);

  // Context Menu State
  const [contextMenu, setContextMenu] = useState<{ x: number; y: number; visible: boolean; lineIndex: number | null }>({
    x: 0,
    y: 0,
    visible: false,
    lineIndex: null,
  });

  // Load Diff Data & Templates
  useEffect(() => {
    setLoading(true);
    // Fetch diff and templates in parallel
    Promise.all([
        getFileDiff('current-file'),
        getReviewTemplates()
    ]).then(([diffData, templateData]) => {
        setDiffLines(diffData);
        setTemplates(templateData);
    })
    .catch(console.error)
    .finally(() => setLoading(false));
  }, []);

  // --- Logic for Folding Lines ---
  const displayLines = useMemo(() => {
    const lines: DiffLine[] = [];
    let skippingImports = false;
    let skippingLombok = false;

    for (let i = 0; i < diffLines.length; i++) {
        const line = diffLines[i];
        const content = line.content.trim();
        
        const isImport = content.startsWith('import ') || (skippingImports && content === '');
        const isLombokAnnotation = content.startsWith('@') || (skippingLombok && content === '');

        // Handle Imports Folding
        if (foldImports && isImport) {
            if (!skippingImports) {
                skippingImports = true;
                // Insert a placeholder line for folded imports
                lines.push({ 
                    ...line, 
                    type: 'header', 
                    content: `import ... (${diffLines.filter(l => l.content.trim().startsWith('import ')).length} lines hidden)`,
                    isFoldPlaceholder: true,
                    onClick: () => setFoldImports(false)
                });
            }
            continue; // Skip the actual line
        } else {
            skippingImports = false;
        }

        // Handle Lombok Folding
        if (foldLombok && isLombokAnnotation) {
            if (!skippingLombok) {
                skippingLombok = true;
                 // Insert a placeholder line for folded lombok
                 lines.push({ 
                    ...line, 
                    type: 'header', 
                    content: `@Annotations ...`,
                    isFoldPlaceholder: true,
                    onClick: () => setFoldLombok(false)
                });
            }
            continue; // Skip the actual line
        } else {
            skippingLombok = false;
        }

        lines.push(line);
    }
    return lines;
  }, [foldImports, foldLombok, viewMode, diffLines]);

  const toggleSearch = () => {
    setSearchOpen(prev => !prev);
    if (!searchOpen) {
        setSearchTerm('');
        setTimeout(() => searchInputRef.current?.focus(), 100);
    }
  };

  const handleContextMenu = (e: React.MouseEvent, lineIndex: number) => {
    e.preventDefault();
    const x = e.clientX;
    const y = e.clientY;
    setContextMenu({ x, y, visible: true, lineIndex });
  };

  const closeContextMenu = () => {
    setContextMenu(prev => ({ ...prev, visible: false }));
  };

  useEffect(() => {
    const handleClick = () => closeContextMenu();
    document.addEventListener('click', handleClick);
    return () => document.removeEventListener('click', handleClick);
  }, []);

  const handleMenuAction = (action: string) => {
    if (onAction) {
        onAction(`Context Action: ${action} on line ${contextMenu.lineIndex !== null ? diffLines[contextMenu.lineIndex]?.newLineNumber || 'context' : 'unknown'}`);
    }
    closeContextMenu();
  };

  // Search Logic
  useEffect(() => {
    if (!searchTerm) {
        setMatches([]);
        setCurrentMatchIdx(0);
        return;
    }

    const newMatches: typeof matches = [];
    const regex = new RegExp(searchTerm.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'gi');

    // Search within currently displayed lines to match WYSIWYG
    displayLines.forEach((line: any, displayIndex) => {
        // Skip placeholders
        if (line.isFoldPlaceholder) return;
        
        // Filter lines based on view mode before searching
        if (viewMode === 'old' && line.type === 'added') return;
        if (viewMode === 'new' && line.type === 'removed') return;
        
        if (!line.content) return;
        let match;
        regex.lastIndex = 0;
        while ((match = regex.exec(line.content)) !== null) {
            newMatches.push({
                lineIndex: displayIndex, // Use display index for scroll target
                start: match.index,
                end: match.index + searchTerm.length
            });
        }
    });

    setMatches(newMatches);
    setCurrentMatchIdx(0);
  }, [searchTerm, viewMode, displayLines]);

  // Scroll to current match
  useEffect(() => {
    if (matches.length > 0 && matches[currentMatchIdx]) {
        const lineIdx = matches[currentMatchIdx].lineIndex;
        // Use a slight timeout to allow render
        setTimeout(() => {
            const el = document.getElementById(`diff-line-${lineIdx}`);
            if (el) {
                el.scrollIntoView({ behavior: 'smooth', block: 'center' });
            }
        }, 50);
    }
  }, [currentMatchIdx, matches]);

  const handleNextMatch = () => {
    setCurrentMatchIdx(prev => (prev + 1) % matches.length);
  };

  const handlePrevMatch = () => {
    setCurrentMatchIdx(prev => (prev - 1 + matches.length) % matches.length);
  };

  // Helper to render content with highlights
  const renderLineContent = (content: string, displayIndex: number) => {
      const lineMatches = matches.filter(m => m.lineIndex === displayIndex);
      
      if (!searchTerm || lineMatches.length === 0) {
          return <span className="text-editor-fg">{content}</span>;
      }

      let lastIndex = 0;
      const parts = [];

      lineMatches.forEach((match, i) => {
          if (match.start > lastIndex) {
              parts.push(content.substring(lastIndex, match.start));
          }
          const isCurrent = matches[currentMatchIdx] === match;
          parts.push(
              <span 
                key={`${displayIndex}-${match.start}`} 
                className={`${isCurrent ? 'bg-orange-500 text-white outline outline-1 outline-white/50 z-10 rounded-[1px]' : 'bg-yellow-600/50 text-white rounded-[1px]'}`}
              >
                  {content.substring(match.start, match.end)}
              </span>
          );
          lastIndex = match.end;
      });

      if (lastIndex < content.length) {
          parts.push(content.substring(lastIndex));
      }

      return <span>{parts}</span>;
  };

  return (
    <div id="tour-diff-view" className="h-full bg-editor-bg flex flex-col min-w-0 relative">
      
      {/* File Header - Responsive Layout */}
      <div className="h-[36px] bg-editor-bg border-b border-editor-line flex items-center px-4 justify-between shrink-0 relative z-20">
        <div className="flex items-center gap-2 text-xs truncate mr-4">
            <span className="text-gray-500 hidden sm:inline">{t('diffview.file')}:</span>
            <span className="text-editor-fg font-medium truncate">src/main/java/.../impl/RetryServiceImpl.java</span>
            <span className="text-editor-success ml-2 hidden sm:inline">+342</span>
            <span className="text-editor-error hidden sm:inline">-108</span>
        </div>
        
        <div className="flex items-center gap-2 shrink-0">
             
             {/* View Mode Switcher */}
             <div className="relative group mr-1">
                <button className="flex items-center gap-1.5 px-2 py-0.5 rounded hover:bg-editor-line text-xs text-gray-400 hover:text-white transition-colors" title={t('diffview.view.diff')}>
                    <Eye size={14} />
                    <span className="hidden lg:inline">
                        {viewMode === 'diff' ? t('diffview.view.diff') : 
                         viewMode === 'old' ? t('diffview.view.old') : t('diffview.view.new')}
                    </span>
                    <ChevronDown size={10} />
                </button>
                <div className="absolute right-0 top-full w-32 pt-2 hidden group-hover:block z-50">
                    <div className="bg-editor-sidebar border border-editor-line rounded shadow-xl py-1">
                        <div onClick={() => setViewMode('diff')} className={`px-3 py-1.5 text-xs cursor-pointer hover:bg-editor-line ${viewMode === 'diff' ? 'text-white' : 'text-gray-400'}`}>{t('diffview.view.diff')}</div>
                        <div onClick={() => setViewMode('old')} className={`px-3 py-1.5 text-xs cursor-pointer hover:bg-editor-line ${viewMode === 'old' ? 'text-white' : 'text-gray-400'}`}>{t('diffview.view.old')}</div>
                        <div onClick={() => setViewMode('new')} className={`px-3 py-1.5 text-xs cursor-pointer hover:bg-editor-line ${viewMode === 'new' ? 'text-white' : 'text-gray-400'}`}>{t('diffview.view.new')}</div>
                    </div>
                </div>
             </div>

             <div className="w-[1px] h-3 bg-editor-line mr-1 hidden sm:block"></div>

             {/* Search */}
             <button 
                onClick={toggleSearch}
                className={`p-1 rounded transition-colors mr-1 ${searchOpen ? 'bg-editor-line text-white' : 'text-gray-400 hover:bg-editor-line hover:text-white'}`}
                title="Find (Ctrl+F)"
             >
                <Search size={14} />
             </button>

             {/* Maximize */}
             <button 
                onClick={toggleMaximize}
                className="p-1 hover:bg-editor-line rounded text-gray-400 hover:text-white transition-colors mr-1"
                title={isMaximized ? t('diffview.restore') : t('diffview.maximize')}
             >
                {isMaximized ? <Minimize2 size={14} /> : <Maximize2 size={14} />}
             </button>

             {/* Wrap Toggle - New Feature */}
             <button 
                onClick={() => setIsLineWrap(!isLineWrap)}
                className={`p-1 rounded transition-colors mr-1 ${isLineWrap ? 'bg-editor-line text-white' : 'text-gray-400 hover:bg-editor-line hover:text-white'}`}
                title={t('diffview.line_wrap')}
             >
                <WrapText size={14} />
             </button>
             
             {/* Fold Imports - Responsive */}
             <button 
                onClick={() => setFoldImports(!foldImports)}
                className={`flex items-center gap-1.5 text-[10px] px-2 py-0.5 rounded transition-colors ${foldImports ? 'bg-editor-accent text-white' : 'bg-editor-line text-gray-400 hover:text-white'}`}
                title={t('diffview.fold_imports')}
             >
                <Package size={12} />
                <span className="hidden lg:inline">{t('diffview.fold_imports')}</span>
             </button>
             
             {/* Fold Lombok - Responsive */}
             <button 
                onClick={() => setFoldLombok(!foldLombok)}
                className={`flex items-center gap-1.5 text-[10px] px-2 py-0.5 rounded transition-colors ${foldLombok ? 'bg-editor-accent text-white' : 'bg-editor-line text-gray-400 hover:text-white'}`}
                title={t('diffview.fold_lombok')}
             >
                <Box size={12} />
                <span className="hidden lg:inline">{t('diffview.fold_lombok')}</span>
             </button>

             <span className="text-[10px] text-editor-accent uppercase font-bold ml-1 hidden xl:inline">Java</span>
        </div>
      </div>

      {/* Floating Search Widget */}
      {searchOpen && (
          <div className="absolute top-[40px] right-6 z-50 bg-editor-sidebar border border-editor-line shadow-xl rounded flex items-center p-1 gap-1 animate-fade-in-down">
              <input 
                  ref={searchInputRef}
                  value={searchTerm}
                  onChange={e => setSearchTerm(e.target.value)}
                  placeholder={t('diffview.search.placeholder')}
                  className="bg-editor-bg border border-editor-line rounded px-2 py-1 text-xs text-white focus:outline-none focus:border-editor-accent w-[180px]"
                  onKeyDown={e => {
                      if(e.key === 'Enter') {
                          if (e.shiftKey) handlePrevMatch();
                          else handleNextMatch();
                      }
                      if(e.key === 'Escape') toggleSearch();
                  }}
              />
              <div className="text-[10px] text-gray-500 min-w-[50px] text-center font-mono">
                  {matches.length > 0 ? `${currentMatchIdx + 1}/${matches.length}` : t('diffview.search.no_results')}
              </div>
              <button onClick={handlePrevMatch} disabled={matches.length === 0} className="p-1 hover:bg-editor-line rounded text-gray-400 hover:text-white disabled:opacity-30">
                  <ArrowUp size={14} />
              </button>
              <button onClick={handleNextMatch} disabled={matches.length === 0} className="p-1 hover:bg-editor-line rounded text-gray-400 hover:text-white disabled:opacity-30">
                  <ArrowDown size={14} />
              </button>
              <button onClick={toggleSearch} className="p-1 hover:bg-editor-line rounded text-gray-400 hover:text-white ml-1">
                  <X size={14} />
              </button>
          </div>
      )}

      {/* Context Menu */}
      {contextMenu.visible && (
        <div 
            className="fixed z-[100] bg-editor-sidebar border border-editor-line shadow-[0_4px_12px_rgba(0,0,0,0.5)] rounded py-1 w-[220px] animate-scale-in"
            style={{ top: contextMenu.y, left: contextMenu.x }}
            onClick={(e) => e.stopPropagation()} 
        >
            <div onClick={() => handleMenuAction('Reject')} className="flex items-center gap-3 px-3 py-1.5 text-xs text-editor-fg hover:bg-editor-line hover:text-white cursor-pointer transition-colors">
                <XCircle size={14} className="text-editor-error" />
                <span>{t('contextmenu.must_change')}</span>
            </div>
            <div onClick={() => handleMenuAction('Concern')} className="flex items-center gap-3 px-3 py-1.5 text-xs text-editor-fg hover:bg-editor-line hover:text-white cursor-pointer transition-colors">
                <AlertTriangle size={14} className="text-editor-warning" />
                <span>{t('contextmenu.concern')}</span>
            </div>
             <div onClick={() => handleMenuAction('Question')} className="flex items-center gap-3 px-3 py-1.5 text-xs text-editor-fg hover:bg-editor-line hover:text-white cursor-pointer transition-colors">
                <HelpCircle size={14} className="text-editor-info" />
                <span>{t('contextmenu.question')}</span>
            </div>
            
            <div className="h-[1px] bg-editor-line my-1"></div>

             <div onClick={() => handleMenuAction('ApproveHunk')} className="flex items-center gap-3 px-3 py-1.5 text-xs text-editor-fg hover:bg-editor-line hover:text-white cursor-pointer transition-colors">
                <Check size={14} className="text-editor-success" />
                <span>{t('contextmenu.approve_hunk')}</span>
            </div>

            <div className="h-[1px] bg-editor-line my-1"></div>

            {/* Submenu for Template */}
            <div className="group relative flex items-center justify-between px-3 py-1.5 text-xs text-editor-fg hover:bg-editor-line hover:text-white cursor-pointer transition-colors">
                 <span>{t('contextmenu.insert_template')}</span>
                 <ChevronRight size={12} />
                 
                 {/* Submenu Content - Dynamic from API */}
                 <div className="absolute left-full top-0 ml-[-2px] bg-editor-sidebar border border-editor-line shadow-[0_4px_12px_rgba(0,0,0,0.5)] rounded py-1 w-[200px] hidden group-hover:block max-h-[200px] overflow-y-auto">
                    {templates.length > 0 ? (
                        templates.map(tpl => (
                            <div 
                                key={tpl.id} 
                                onClick={() => handleMenuAction(`Template: ${tpl.label}`)} 
                                className="px-3 py-1.5 text-xs text-editor-fg hover:bg-editor-line hover:text-white cursor-pointer whitespace-nowrap"
                                title={tpl.content}
                            >
                                {tpl.label}
                            </div>
                        ))
                    ) : (
                         <div className="px-3 py-1.5 text-xs text-gray-500 italic">No templates</div>
                    )}
                 </div>
            </div>

            <div className="h-[1px] bg-editor-line my-1"></div>

             <div onClick={() => handleMenuAction('Generate Patch')} className="flex items-center gap-3 px-3 py-1.5 text-xs text-editor-fg hover:bg-editor-line hover:text-white cursor-pointer transition-colors">
                <Wand size={14} className="text-editor-accent" />
                <span>{t('contextmenu.generate_patch')}</span>
            </div>
        </div>
      )}

      {/* Column Headers Row */}
      <div className="flex items-center bg-editor-sidebar border-b border-editor-line text-[10px] text-gray-500 font-bold uppercase tracking-wider select-none shrink-0 z-10 sticky top-0">
         {(viewMode === 'diff' || viewMode === 'old') && (
            <div className={`w-[60px] text-right pr-3 py-1 border-r border-editor-line/30 bg-editor-bg/50 text-red-400/70 truncate ${viewMode === 'old' ? 'w-[60px]' : ''}`}
                 title={diffContext?.base || t('diffview.header.old')}>
               {diffContext?.base ? diffContext.base.split('/').pop() : t('diffview.header.old')}
            </div>
         )}
         
         {viewMode === 'diff' && (
            <div className="w-[12px] flex justify-center py-1 bg-editor-bg/50" title="Changes Heatmap">
               <div className="w-[2px] h-3 bg-gray-600/50"></div>
            </div>
         )}

         {(viewMode === 'diff' || viewMode === 'new') && (
             <div className="w-[60px] text-right pr-3 py-1 border-r border-editor-line/30 bg-editor-bg/50 text-green-400/70 truncate"
                  title={diffContext?.head || t('diffview.header.new')}>
                 {diffContext?.head ? diffContext.head.split('/').pop() : t('diffview.header.new')}
             </div>
         )}

         <div className="flex-1 px-4 py-1 bg-editor-bg/50">
             {t('diffview.header.main')}
         </div>
      </div>

      {/* Diff Content - Update overflow to auto for horizontal scroll support */}
      <div className="flex-1 overflow-y-auto overflow-x-auto font-mono text-[14px] leading-[22px] relative">
        
        {loading && (
          <div className="absolute inset-0 flex items-center justify-center bg-editor-bg/80 z-30">
            <div className="flex flex-col items-center gap-3">
              <Loader2 size={32} className="animate-spin text-editor-accent" />
              <span className="text-xs text-gray-400">Loading diff...</span>
            </div>
          </div>
        )}

        {displayLines.map((line: any, displayIndex: number) => {
             // Logic for View Modes
             if (viewMode === 'old' && line.type === 'added' && !line.isFoldPlaceholder) return null;
             if (viewMode === 'new' && line.type === 'removed' && !line.isFoldPlaceholder) return null;

             const isFoldedPlaceholder = line.isFoldPlaceholder;
             
             // Logic for diff colors (only in diff mode)
             const isModified = line.type !== 'context';
             const isAdd = line.type === 'added';
             const isRemove = line.type === 'removed';
             
             let bgClass = '';
             if (viewMode === 'diff' && !isFoldedPlaceholder) {
                 if (isAdd) bgClass = 'bg-editor-success/10';
                 if (isRemove) bgClass = 'bg-editor-error/10';
             }

             if (isFoldedPlaceholder) {
                 bgClass = 'bg-editor-line/30 hover:bg-editor-line/50 cursor-pointer';
             }

             return (
                <div 
                    key={displayIndex} 
                    id={`diff-line-${displayIndex}`} 
                    className={`flex w-full hover:bg-editor-line/50 group relative ${bgClass} ${isLineWrap ? '' : 'min-w-fit'}`}
                    onContextMenu={(e) => handleContextMenu(e, displayIndex)}
                    onClick={line.onClick}
                >
                    
                    {/* Old Line Number - Red tint for Old */}
                    {(viewMode === 'diff' || viewMode === 'old') && (
                        <div className={`w-[60px] text-right pr-3 select-none bg-editor-bg border-r border-red-900/30 shrink-0 sticky left-0 z-10 text-red-500/50 ${isFoldedPlaceholder ? '' : 'font-mono text-xs pt-[1px]'}`}>
                            {isFoldedPlaceholder ? <MoreHorizontal size={12} className="inline opacity-50 text-gray-600" /> : (line.oldLineNumber || '')}
                        </div>
                    )}

                    {/* Heatmap Bar - Only in Diff Mode */}
                    {viewMode === 'diff' && (
                        <div className="w-[12px] flex items-center justify-center bg-editor-bg shrink-0 sticky left-[60px] z-10">
                            {!isFoldedPlaceholder && isAdd && <div className="w-1 h-full bg-editor-success opacity-40"></div>}
                            {!isFoldedPlaceholder && isRemove && <div className="w-1 h-full bg-editor-error opacity-40"></div>}
                        </div>
                    )}

                    {/* New Line Number - Green tint for New */}
                    {(viewMode === 'diff' || viewMode === 'new') && (
                        <div className={`w-[60px] text-right pr-3 select-none bg-editor-bg border-r border-green-900/30 shrink-0 sticky z-10 text-green-500/50 ${viewMode === 'diff' ? 'left-[72px]' : 'left-0'} ${isFoldedPlaceholder ? '' : 'font-mono text-xs pt-[1px]'}`}>
                            {isFoldedPlaceholder ? <MoreHorizontal size={12} className="inline opacity-50 text-gray-600" /> : (line.newLineNumber || '')}
                        </div>
                    )}

                    {/* Code Content */}
                    <div className={`flex-1 px-4 relative min-w-0 flex items-center ${isLineWrap ? 'whitespace-pre-wrap break-all' : 'whitespace-pre'}`}>
                        {isFoldedPlaceholder ? (
                            <span className="text-gray-500 italic select-none flex items-center gap-2 text-xs">
                                <ChevronRight size={12} /> {line.content}
                            </span>
                        ) : (
                            <>
                                <span className={(viewMode === 'diff' && isRemove) ? 'line-through text-gray-500 opacity-70' : ''}>
                                    {renderLineContent(line.content, displayIndex)}
                                </span>

                                {/* Inline Warning/Error Markers - Keep these visible in New & Diff modes */}
                                {(viewMode === 'diff' || viewMode === 'new') && line.severity === ReviewSeverity.WARNING && (
                                    <div className="absolute left-0 bottom-0 w-full h-[2px] bg-orange-500/50" style={{backgroundImage: 'url("data:image/svg+xml;base64,PHN2ZyB4bWxucz0naHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmcnIHdpZHRoPSc0JyBoZWlnaHQ9JzQnPjxwYXRoIGQ9J00wIDQgTDIgMiBMNCA0JyBzdHJva2U9JyNmZjg4MDAnIGZpbGw9J25vbmUnLz48L3N2Zz4=")'}}></div>
                                )}
                                {(viewMode === 'diff' || viewMode === 'new') && line.severity === ReviewSeverity.ERROR && (
                                    <div className="absolute inset-0 border border-editor-error/50 bg-editor-error/5 pointer-events-none"></div>
                                )}
                            </>
                        )}
                    </div>
                     
                    {/* Right Gutter Icons for Issues */}
                    {(viewMode === 'diff' || viewMode === 'new') && line.severity && !isFoldedPlaceholder && (
                        <div className="absolute right-4 top-1/2 -translate-y-1/2 flex items-center gap-2">
                             {line.severity === ReviewSeverity.WARNING && (
                                <div className="flex items-center gap-1 bg-editor-bg border border-editor-warning text-editor-warning text-[10px] px-2 py-0.5 rounded shadow-lg z-10 cursor-help">
                                    <AlertTriangle size={12} />
                                    <span>{line.message}</span>
                                </div>
                             )}
                             {line.severity === ReviewSeverity.ERROR && (
                                 <div className="flex items-center gap-1 bg-editor-bg border border-editor-error text-editor-error text-[10px] px-2 py-0.5 rounded shadow-lg z-10 cursor-help">
                                    <XCircle size={12} />
                                    <span>{line.message}</span>
                                </div>
                             )}
                        </div>
                    )}

                </div>
             );
        })}
        {/* Fill empty space */}
        <div className="flex-1 bg-editor-bg"></div>
      </div>
    </div>
  );
};

export default DiffView;