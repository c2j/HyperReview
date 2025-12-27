import React, { useState, useEffect, useRef, useMemo } from 'react';
import type { DiffLine, ReviewTemplate } from '../api/types';
import { useApiClient } from '../api/client';
import { useRepositoryStatus } from '../hooks/useRepository';
import VirtualDiffViewer from './VirtualDiffViewer';
import { DiffOptimizer, PerformanceMonitor, DiffCache } from '../utils/diffOptimization';
import { AlertTriangle, XCircle, ChevronDown, Maximize2, Minimize2, Search, X, ArrowUp, ArrowDown, HelpCircle, Check, Wand, ChevronRight, Eye, Package, Box, WrapText, Loader2 } from 'lucide-react';
import { useTranslation } from '../i18n';

interface DiffViewProps {
  isMaximized: boolean;
  toggleMaximize: () => void;
  onAction?: (msg: string) => void;
  diffContext?: { base: string; head: string };
  selectedFile?: string | null;
  activeFilePath?: string;
}

type ViewMode = 'diff' | 'old' | 'new';

const DiffView: React.FC<DiffViewProps> = ({ isMaximized, toggleMaximize, onAction, diffContext, selectedFile, activeFilePath: _activeFilePath }) => {
  const { t } = useTranslation();
  const { getFileDiff, getReviewTemplates, readFileContent } = useApiClient();
  const { isRepoLoaded } = useRepositoryStatus();

  // Performance optimization instances
  const diffOptimizer = useMemo(() => new DiffOptimizer({
    maxVisibleLines: 1000,
    chunkSize: 500,
    enableFolding: true,
    enableSyntaxHighlighting: false // Disabled for performance
  }), []);

  const performanceMonitor = useMemo(() => new PerformanceMonitor(), []);
  const diffCache = useMemo(() => new DiffCache(5), []); // 5 minute cache

  // Data State
  const [diffLines, setDiffLines] = useState<DiffLine[]>([]);
  const [optimizedChunks, setOptimizedChunks] = useState<any[]>([]);
  const [templates, setTemplates] = useState<ReviewTemplate[]>([]);
  const [loading, setLoading] = useState(true);
  const [filePath, setFilePath] = useState('current-file'); // Default file path
  const [showFileContent, setShowFileContent] = useState(false); // Show file content when no diff
  const [fileNotFoundInfo, setFileNotFoundInfo] = useState<{
    exists: boolean;
    message: string;
    details?: string;
  } | null>(null); // Friendly info for files that don't exist

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

  // Update file path when selectedFile changes
  useEffect(() => {
    console.log('[DiffView] selectedFile changed to:', selectedFile);
    console.log('[DiffView] Current filePath before update:', filePath);
    if (selectedFile) {
      console.log('[DiffView] Setting filePath to selectedFile:', selectedFile);
      setFilePath(selectedFile);
      // Clear any previous file not found info when file changes
      setFileNotFoundInfo(null);
    } else {
      console.log('[DiffView] No selectedFile, resetting to current-file');
      // If no file is selected, reset to default
      setFilePath('current-file');
      setFileNotFoundInfo(null);
    }
  }, [selectedFile]);

  // Load Diff Data & Templates
  useEffect(() => {
    console.log('[DiffView] useEffect triggered with filePath:', filePath, 'isRepoLoaded:', isRepoLoaded);
    if (!isRepoLoaded) {
      // No repository loaded, skip loading
      console.log('[DiffView] No repo loaded, skipping diff load');
      return;
    }
    console.log('[DiffView] Starting to load diff for file:', filePath);
    setLoading(true);
    console.log('[DiffView] Loading state set to true');
    performanceMonitor.startTimer('diff_load');

    // Check cache first - include branch info in cache key
    const baseBranch = diffContext?.base || 'none';
    const headBranch = diffContext?.head || 'none';
    const cacheKey = `diff-${filePath || 'current-file'}-${baseBranch}-${headBranch}`;
    console.log('Cache key:', cacheKey);
    const cachedDiff = diffCache.get<DiffLine[]>(cacheKey);

    if (cachedDiff) {
      console.log('Using cached diff data');
      setDiffLines(cachedDiff);
      const chunks = diffOptimizer.processDiff(cachedDiff);
      setOptimizedChunks(chunks);
      setLoading(false);
      return;
    }

    // Fetch diff and templates in parallel
    // Use diffContext to determine which branches/commits to compare
    const fetchDiff = async () => {
      try {
        console.log('fetchDiff called with filePath:', filePath, 'diffContext:', diffContext);

        // Determine commits based on diffContext
        // If we have a diff context with base and head, use those branches
        // Otherwise fall back to HEAD vs working directory
        let oldCommit: string | undefined;
        let newCommit: string | undefined;

        if (diffContext && diffContext.base && diffContext.head) {
          // Compare two branches/commits
          oldCommit = diffContext.base;
          newCommit = diffContext.head;
          console.log('Using branch comparison: base=', oldCommit, 'head=', newCommit);
        } else if (filePath && filePath !== 'current-file') {
          // No branch context, show HEAD vs working directory for specific file
          oldCommit = 'HEAD';
          newCommit = undefined;
          console.log('Using HEAD vs working directory');
        } else {
          // No context at all
          oldCommit = undefined;
          newCommit = undefined;
          console.log('No commit context specified');
        }

        console.log('Requesting diff with oldCommit:', oldCommit, 'newCommit:', newCommit);

        const [diffData, templateData] = await Promise.all([
          getFileDiff(filePath || 'current-file', oldCommit, newCommit),
          getReviewTemplates()
        ]);

        console.log('Diff data received:', diffData.length, 'lines');
        console.log('Diff data content:', diffData); // Log the actual diff data

        if (diffData.length === 0) {
          console.log('No diff lines returned for file:', filePath);
          console.log('This means the file has no changes between HEAD and working directory');

          // 如果是查看特定文件，尝试显示文件内容
          if (filePath && filePath !== 'current-file') {
            console.log('Attempting to load file content for:', filePath);
            setShowFileContent(true);

            try {
              const content = await readFileContent(filePath);
              console.log('File content loaded, length:', content.length, 'characters');
              console.log('First 200 chars of content:', content.substring(0, 200));

              const lines = content.split('\n').map((line, index) => ({
                line_number: index + 1,
                content: line || '', // Ensure content is never null/undefined
                line_type: 'Context' as const,
                file_path: filePath,
                severity: undefined,
                comments: [],
                change_type: 'None' as const,
                old_line_number: index + 1,
                new_line_number: index + 1
              }));
              console.log('Created', lines.length, 'lines from file content');
              setDiffLines(lines);
              setOptimizedChunks([]);
            } catch (err) {
              console.error('Failed to load file content:', err as Error);
              const errorMessage = (err as Error).message || '';

              // 检查是否是"文件不存在"错误（Git历史文件或已删除文件）
              if (errorMessage.includes('No such file or directory') ||
                  errorMessage.includes('os error 2') ||
                  errorMessage.includes('The system cannot find the file')) {
                // 分析文件不存在的具体原因
                let message = '';
                let details = '';

                // 根据是否有diffContext判断是否是基线对比场景
                if (diffContext) {
                  // 基线对比场景：文件在某个基线中被删除
                  message = `File deleted in target branch`;
                  details = `This file exists in "${diffContext.base}" but has been removed in "${diffContext.head}".`;
                } else {
                  // 默认场景：工作目录文件被删除
                  message = `File not found in working directory`;
                  details = `This file exists in Git history but has been removed from the current working directory. It may have been deleted or moved.`;
                }

                console.log('File not found:', message, details);

                // 设置友好提示信息，而不是用DiffLine显示错误
                setFileNotFoundInfo({
                  exists: false,
                  message,
                  details
                });
                setDiffLines([]);
                setOptimizedChunks([]);
              } else {
                console.log('Other error (not file not found):', errorMessage);
                // 其他错误显示友好提示，但使用不同的消息
                setFileNotFoundInfo({
                  exists: false,
                  message: 'Failed to load file',
                  details: `An error occurred while loading the file: ${errorMessage}`
                });
                setDiffLines([]);
                setOptimizedChunks([]);
              }
            }
            return;
          }
        } else {
          setShowFileContent(false);
        }

        // Cache the diff data
        diffCache.set(cacheKey, diffData);

        // Process with optimizations
        const chunks = diffOptimizer.processDiff(diffData);

        setDiffLines(diffData);
        setOptimizedChunks(chunks);
        setTemplates(templateData);

        const loadTime = performanceMonitor.endTimer('diff_load');
        console.log(`Diff loaded with ${diffData.length} lines in ${loadTime}ms`);

        // Log memory stats for large files
        if (diffData.length > 1000) {
          const stats = diffOptimizer.getMemoryStats();
          console.log('Memory usage:', stats);
        }
      } catch (err) {
        console.error('Failed to load diff:', err as Error);
        console.error('Error details:', (err as Error).message || err);
        console.error('File path that failed:', filePath);
        // Clear any previous diff data on error
        setDiffLines([]);
        setOptimizedChunks([]);
      } finally {
        console.log('[DiffView] Setting loading state to false');
        setLoading(false);
      }
    };

    fetchDiff();
  }, [isRepoLoaded, filePath, diffContext, diffOptimizer, diffCache, performanceMonitor, getFileDiff, getReviewTemplates]);

  // --- Logic for Folding Lines with Optimization ---
  const displayLines = useMemo(() => {
    // Ensure diffLines is always an array
    if (!diffLines || !Array.isArray(diffLines)) {
      return [];
    }

    // Use optimized chunks for large files
    if (diffLines.length > 1000 && optimizedChunks.length > 0) {
      return diffOptimizer.getVisibleLines();
    }

    // Original folding logic for smaller files
    const lines: DiffLine[] = [];
    let skippingImports = false;
    let skippingLombok = false;

    for (let i = 0; i < diffLines.length; i++) {
        const line = diffLines[i];
        if (!line) continue; // Skip null/undefined lines

        const content = line.content?.trim() || '';

        const isImport = content.startsWith('import ') || (skippingImports && content === '');
        const isLombokAnnotation = content.startsWith('@') || (skippingLombok && content === '');

        // Handle Imports Folding
        if (foldImports && isImport) {
            if (!skippingImports) {
                skippingImports = true;
                // Insert a placeholder line for folded imports
                lines.push({
                    ...line,
                    line_type: 'Header',
                    content: `import ... (${diffLines.filter(l => l?.content?.trim().startsWith('import ')).length} lines hidden)`,
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
                    line_type: 'Header',
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
  }, [foldImports, foldLombok, viewMode, diffLines, diffOptimizer, optimizedChunks]);

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
        onAction(`Context Action: ${action} on line ${contextMenu.lineIndex !== null ? diffLines[contextMenu.lineIndex]?.new_line_number || 'context' : 'unknown'}`);
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
        if (viewMode === 'old' && line.line_type === 'Added') return;
        if (viewMode === 'new' && line.line_type === 'Removed') return;
        
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

      lineMatches.forEach((match) => {
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
            <span className="text-editor-fg font-medium truncate">{filePath === 'current-file' ? 'src/main/java/.../impl/RetryServiceImpl.java' : filePath}</span>
            {showFileContent ? (
              <span className="text-editor-info text-[10px] ml-2 border border-editor-info/30 px-1 py-0.5 rounded">FILE</span>
            ) : (
              <>
                <span className="text-editor-success ml-2 hidden sm:inline">+342</span>
                <span className="text-editor-error hidden sm:inline">-108</span>
              </>
            )}
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
                                onClick={() => handleMenuAction(`Template: ${tpl.name}`)} 
                                className="px-3 py-1.5 text-xs text-editor-fg hover:bg-editor-line hover:text-white cursor-pointer whitespace-nowrap"
                                title={tpl.content}
                            >
                                {tpl.name}
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

        {/* Friendly File Not Found Message */}
        {fileNotFoundInfo && !loading && (
          <div className="flex-1 overflow-y-auto p-8">
            <div className="max-w-2xl mx-auto">
              <div className="bg-editor-sidebar border border-editor-line/50 rounded-lg p-6 shadow-xl">
                <div className="flex items-start gap-4">
                  <div className="flex-shrink-0 w-12 h-12 rounded-full bg-editor-warning/10 flex items-center justify-center">
                    <AlertTriangle size={24} className="text-editor-warning" />
                  </div>
                  <div className="flex-1">
                    <h3 className="text-lg font-semibold text-editor-fg mb-2">
                      {fileNotFoundInfo.message}
                    </h3>
                    <p className="text-sm text-gray-400 mb-4 leading-relaxed">
                      {fileNotFoundInfo.details}
                    </p>
                    <div className="bg-editor-bg/50 border border-editor-line/30 rounded p-3 mb-4">
                      <div className="flex items-center gap-2 mb-1">
                        <span className="text-[10px] font-bold text-gray-500 uppercase tracking-wider">File Path</span>
                      </div>
                      <code className="text-xs text-editor-accent font-mono break-all">{filePath}</code>
                    </div>
                    {diffContext && (
                      <div className="grid grid-cols-2 gap-2 mb-4">
                        <div className="bg-editor-bg/30 border border-editor-line/30 rounded p-2">
                          <div className="flex items-center gap-1.5 mb-1">
                            <div className="w-2 h-2 rounded-full bg-editor-success"></div>
                            <span className="text-[10px] font-bold text-gray-500 uppercase tracking-wider">Source Branch</span>
                          </div>
                          <code className="text-xs text-gray-300 font-mono">{diffContext.base}</code>
                        </div>
                        <div className="bg-editor-bg/30 border border-editor-line/30 rounded p-2">
                          <div className="flex items-center gap-1.5 mb-1">
                            <div className="w-2 h-2 rounded-full bg-editor-error"></div>
                            <span className="text-[10px] font-bold text-gray-500 uppercase tracking-wider">Target Branch</span>
                          </div>
                          <code className="text-xs text-gray-300 font-mono">{diffContext.head}</code>
                        </div>
                      </div>
                    )}
                    <div className="flex items-center gap-2 text-xs text-gray-500">
                      <HelpCircle size={14} />
                      <span>This file was likely deleted in a recent commit or branch merge.</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* Virtual Diff Viewer for large files with smooth scrolling */}
        {!fileNotFoundInfo && (
        <VirtualDiffViewer
          diffLines={displayLines}
          viewMode={viewMode}
          isLineWrap={isLineWrap}
          height={600}
          lineHeight={24}
          showLineNumbers={true}
          showHeatmap={true}
          searchTerm={searchTerm}
          currentMatchIndex={currentMatchIdx}
          matches={matches}
          onLineClick={(line, index) => {
            // Handle line click - add comment or other actions
            console.log('Line clicked:', line, index);
          }}
          onContextMenu={(e, _line, index) => handleContextMenu(e, index)}
          renderLineContent={renderLineContent}
          isFileContent={showFileContent}
        />
        )}
        {/* Fill empty space */}
        {!fileNotFoundInfo && <div className="flex-1 bg-editor-bg"></div>}
      </div>
    </div>
  );
};

export default DiffView;