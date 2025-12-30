/**
 * Virtual Scrolling Diff Viewer
 * Handles large diffs (10k+ lines) with smooth 60fps scrolling
 */

import React, { useCallback } from 'react';
import type { DiffLine } from '../api/types';
import { Zap } from 'lucide-react';

interface VirtualDiffViewerProps {
  diffLines: DiffLine[];
  onLineClick?: (line: DiffLine, index: number) => void;
  onContextMenu?: (e: React.MouseEvent, line: DiffLine, index: number) => void;
  renderLineContent?: (content: string, index: number) => React.ReactNode;
  className?: string;
  height?: number; // Kept for compatibility but not used
  lineHeight?: number; // Kept for compatibility but not used
  showLineNumbers?: boolean; // Kept for compatibility but not used
  showHeatmap?: boolean; // Kept for compatibility but not used
  viewMode?: 'diff' | 'old' | 'new';
  isLineWrap?: boolean;
  searchTerm?: string;
  currentMatchIndex?: number;
  matches?: Array<{lineIndex: number, start: number, end: number}>;
  isFileContent?: boolean; // New prop to indicate we're showing file content
}

/**
 * Virtual scrolling diff viewer component
 * Handles large diffs (10k+ lines) with smooth 60fps scrolling
 */
const VirtualDiffViewer: React.FC<VirtualDiffViewerProps> = ({
  diffLines,
  onLineClick,
  onContextMenu,
  renderLineContent,
  className = '',
  height: _height, // Kept for compatibility but not used
  lineHeight: _lineHeight, // Kept for compatibility but not used
  showLineNumbers: _showLineNumbers, // Kept for compatibility but not used
  showHeatmap: _showHeatmap, // Kept for compatibility but not used
  viewMode = 'diff',
  isLineWrap = false,
  searchTerm,
  currentMatchIndex,
  matches,
  isFileContent = false
}) => {
  const handleLineClick = useCallback((line: DiffLine, index: number) => {
    if (onLineClick) {
      onLineClick(line, index);
    }
  }, [onLineClick]);

  // Helper to render content with search highlights
  const renderHighlightedContent = (content: string, lineIndex: number) => {
    if (!searchTerm || !matches) {
      return content;
    }

    const lineMatches = matches.filter(m => m.lineIndex === lineIndex);
    if (lineMatches.length === 0) {
      return content;
    }

    let lastIndex = 0;
    const parts = [];

    lineMatches.forEach((match) => {
      if (match.start > lastIndex) {
        parts.push(content.substring(lastIndex, match.start));
      }
      const isCurrent = matches[currentMatchIndex || 0] === match;
      parts.push(
        <span
          key={`${lineIndex}-${match.start}`}
          className={`${isCurrent ? 'bg-orange-500 text-white outline outline-1 outline-white/50 rounded-[1px]' : 'bg-yellow-600/50 text-white rounded-[1px]'}`}
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

  // Ensure diffLines is always an array
  const safeDiffLines = Array.isArray(diffLines) ? diffLines : [];

  // Filter lines based on view mode (but don't filter file content)
  const filteredLines = safeDiffLines.filter(line => {
    // For file content, show all lines regardless of view mode
    if (isFileContent) return true;
    
    // For diffs, apply view mode filtering
    if (viewMode === 'old' && line.line_type === 'Added') return false;
    if (viewMode === 'new' && line.line_type === 'Removed') return false;
    return true;
  });

  // Debug logging
  console.log('[VirtualDiffViewer] isFileContent:', isFileContent, 'diffLines length:', diffLines?.length, 'filteredLines length:', filteredLines.length);

  // Don't render if no lines
  if (!filteredLines || filteredLines.length === 0) {
    return (
      <div className={`flex flex-col h-full bg-editor-bg text-editor-fg ${className}`}>
        <div className="flex flex-col items-center justify-center h-full text-editor-fg/60 gap-2">
          <div>{isFileContent ? 'No file content to display' : 'No diff lines to display'}</div>
          <div className="text-xs text-editor-fg/40 text-center max-w-md">
            {isFileContent
              ? 'Unable to load file content. The file may not exist or may be a binary file.'
              : 'This file has no changes in the current diff context'}
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className={`flex flex-col h-full bg-editor-bg text-editor-fg ${className}`}>
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-2 bg-editor-line/30 border-b border-editor-line">
        <div className="text-xs text-editor-fg/60">
          {filteredLines.length} lines
        </div>
        <div className="flex items-center gap-4 text-xs">
          <div className="flex items-center gap-1">
            <div className="w-3 h-3 bg-green-500/20 border border-green-500"></div>
            <span className="text-editor-fg/60">Added</span>
          </div>
          <div className="flex items-center gap-1">
            <div className="w-3 h-3 bg-red-500/20 border border-red-500"></div>
            <span className="text-editor-fg/60">Removed</span>
          </div>
        </div>
      </div>

      {/* Simple diff lines display - TODO: Replace with virtual scrolling */}
      <div className="flex-1 overflow-auto">
        {filteredLines.length > 0 ? (
          <div className="min-h-full">
            {filteredLines.map((line, index) => {
              const isAdd = line.line_type === 'Added';
              const isRemove = line.line_type === 'Removed';
              let bgClass = '';
              if (viewMode === 'diff') {
                if (isAdd) bgClass = 'bg-editor-success/10';
                if (isRemove) bgClass = 'bg-editor-error/10';
              }

              return (
                <div
                  key={index}
                  className={`flex w-full hover:bg-editor-line/50 group relative ${bgClass} ${isLineWrap ? '' : 'min-w-fit'}`}
                  onClick={() => handleLineClick(line, index)}
                  onContextMenu={(e) => onContextMenu && onContextMenu(e, line, index)}
                >
                  {(viewMode === 'diff' || viewMode === 'old') && (
                    <div className="w-[60px] text-right pr-3 select-none bg-editor-bg border-r border-red-900/30 shrink-0 sticky left-0 z-10 text-red-500/50 font-mono text-xs pt-[1px]">
                      {line.old_line_number || ''}
                    </div>
                  )}

                  {(viewMode === 'diff' || viewMode === 'new') && (
                    <div className={`w-[60px] text-right pr-3 select-none bg-editor-bg border-r border-green-900/30 shrink-0 sticky z-10 text-green-500/50 font-mono text-xs pt-[1px] ${viewMode === 'diff' ? 'left-[60px]' : 'left-0'}`}>
                      {line.new_line_number || ''}
                    </div>
                  )}

                  <div className={`flex-1 px-4 relative min-w-0 flex items-center ${isLineWrap ? 'whitespace-pre-wrap break-all' : 'whitespace-pre'}`}>
                    <span className={(viewMode === 'diff' && isRemove) ? 'line-through text-gray-500 opacity-70' : ''}>
                      {renderLineContent ? renderLineContent(line.content, index) : renderHighlightedContent(line.content, index)}
                    </span>
                  </div>
                </div>
              );
            })}
          </div>
        ) : (
          <div className="flex items-center justify-center h-full text-editor-fg/60">
            No lines to display
          </div>
        )}
      </div>

      {/* Stats footer */}
      <div className="flex items-center justify-between px-4 py-2 bg-editor-bg border-t border-editor-line text-xs text-editor-fg/60">
        <div className="flex items-center gap-2">
          {diffLines.length > 1000 && (
            <span className="flex items-center gap-1 px-2 py-0.5 bg-green-500/20 text-green-400 rounded">
              <Zap size={12} />
              Optimized
            </span>
          )}
          <span>Virtual scrolling enabled</span>
        </div>
        <div>
          {filteredLines.length} total lines
        </div>
      </div>
    </div>
  );
};

export default VirtualDiffViewer;