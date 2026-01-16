import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { ChevronDown, ChevronUp, FileText, Loader2, AlertCircle } from 'lucide-react';

interface DiffChunk {
  chunk_type: string;
  old_start: number;
  old_lines: number;
  new_start: number;
  new_lines: number;
  lines: DiffLine[];
}

interface DiffLine {
  line_type: string;
  old_line_number: number | null;
  new_line_number: number | null;
  content: string;
}

interface GetDiffResult {
  file_path: string;
  change_type: string;
  total_lines: number;
  diff_chunks: DiffChunk[];
  load_time_ms: number;
  success: boolean;
}

interface GerritDiffViewerProps {
  changeId: string;
  filePath: string;
  patchSetNumber?: number;
  onClose: () => void;
}

const GerritDiffViewer: React.FC<GerritDiffViewerProps> = ({
  changeId,
  filePath,
  patchSetNumber,
  onClose
}) => {
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [diff, setDiff] = useState<GetDiffResult | null>(null);
  const [currentLine, setCurrentLine] = useState(0);

  const loadDiff = async () => {
    setLoading(true);
    setError(null);

    try {
      const result = await invoke<GetDiffResult>('gerrit_get_diff_simple', {
        changeId,
        filePath,
        patchSetNumber,
        startLine: null,
        endLine: null,
      });

      setDiff(result);
    } catch (err) {
      console.error('Failed to load diff:', err);
      setError('Failed to load diff: ' + (err as Error).message);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadDiff();
  }, [changeId, filePath, patchSetNumber]);

  const getLineClass = (lineType: string) => {
    switch (lineType) {
      case 'Addition':
        return 'bg-green-500/10 text-green-400';
      case 'Deletion':
        return 'bg-red-500/10 text-red-400';
      default:
        return 'bg-editor-line/30 text-gray-300';
    }
  };

  const getLinePrefix = (lineType: string) => {
    switch (lineType) {
      case 'Addition':
        return '+';
      case 'Deletion':
        return '-';
      default:
        return ' ';
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (!diff) return;

    if (e.key === 'ArrowDown' && currentLine < diff.total_lines - 1) {
      setCurrentLine(prev => prev + 1);
    } else if (e.key === 'ArrowUp' && currentLine > 0) {
      setCurrentLine(prev => prev - 1);
    } else if (e.key === 'Home') {
      setCurrentLine(0);
    } else if (e.key === 'End') {
      setCurrentLine(diff.total_lines - 1);
    } else if (e.key === 'PageDown') {
      setCurrentLine(prev => Math.min(prev + 20, diff.total_lines - 1));
    } else if (e.key === 'PageUp') {
      setCurrentLine(prev => Math.max(prev - 20, 0));
    }
  };

  if (loading) {
    return (
      <div className="flex flex-col items-center justify-center h-[500px]">
        <Loader2 size={48} className="animate-spin text-editor-accent" />
        <div className="text-sm text-gray-400 mt-4">Loading diff...</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex flex-col items-center justify-center h-[500px]">
        <AlertCircle size={48} className="text-red-400 mb-3" />
        <div className="text-sm text-red-400 mb-1">Error loading diff</div>
        <div className="text-xs text-gray-500">{error}</div>
      </div>
    );
  }

  if (!diff) {
    return null;
  }

  return (
    <div className="flex flex-col h-full" onKeyDown={handleKeyDown} tabIndex={0}>
      <div className="flex items-center justify-between px-4 py-3 border-b border-editor-line bg-editor-line/20">
        <div className="flex items-center gap-3">
          <div className="flex items-center gap-2">
            <FileText size={16} className="text-gray-500" />
            <span className="text-sm text-gray-300 font-mono">{filePath}</span>
          </div>
          <div className={`px-2 py-1 rounded text-xs font-medium ${
            diff.change_type === 'ADDED' ? 'bg-green-500/20 text-green-400' :
            diff.change_type === 'DELETED' ? 'bg-red-500/20 text-red-400' :
            'bg-blue-500/20 text-blue-400'
          }`}>
            {diff.change_type}
          </div>
        </div>
        <div className="flex items-center gap-3 text-xs text-gray-500">
          <span>{diff.total_lines} lines</span>
          <span>Loaded in {diff.load_time_ms}ms</span>
          <button
            onClick={onClose}
            className="px-3 py-1 rounded hover:bg-editor-line text-gray-400 hover:text-white transition-colors"
          >
            Close
          </button>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto font-mono text-xs">
        {diff.diff_chunks.map((chunk, chunkIndex) => (
          <div key={chunkIndex} className="border-b border-editor-line/50">
            <div className={`px-4 py-2 text-[10px] uppercase font-semibold tracking-wider ${
              chunk.chunk_type === 'Addition' ? 'bg-green-500/5 text-green-400' :
              chunk.chunk_type === 'Deletion' ? 'bg-red-500/5 text-red-400' :
              'bg-gray-500/5 text-gray-500'
            }`}>
              {chunk.chunk_type}
            </div>
            <div>
              {chunk.lines.map((line, lineIndex) => {
                const lineNum = chunk.new_start + lineIndex;
                const isCurrentLine = lineNum === currentLine;

                return (
                  <div
                    key={`${chunkIndex}-${lineIndex}`}
                    className={`flex ${isCurrentLine ? 'bg-editor-accent/20' : ''}`}
                  >
                    <div className="flex-shrink-0 w-12 text-right pr-2 text-gray-600 select-none">
                      {line.new_line_number ?? line.old_line_number ?? ''}
                    </div>
                    <div className={`flex-1 px-3 py-0.5 ${getLineClass(line.line_type)}`}>
                      <span className="select-none mr-2 text-gray-500">
                        {getLinePrefix(line.line_type)}
                      </span>
                      <span className="whitespace-pre">
                        {line.content.trim()}
                      </span>
                    </div>
                  </div>
                );
              })}
            </div>
          </div>
        ))}
      </div>

      <div className="flex items-center justify-between px-4 py-2 border-t border-editor-line bg-editor-line/20">
        <div className="text-xs text-gray-500">
          Line {currentLine + 1} of {diff.total_lines}
        </div>
        <div className="flex items-center gap-2">
          <button
            onClick={() => setCurrentLine(0)}
            disabled={currentLine === 0}
            className="p-1 rounded hover:bg-editor-line text-gray-500 hover:text-white disabled:opacity-30 disabled:cursor-not-allowed transition-colors"
            title="Go to top"
          >
            <ChevronUp size={14} />
          </button>
          <button
            onClick={() => setCurrentLine(diff.total_lines - 1)}
            disabled={currentLine === diff.total_lines - 1}
            className="p-1 rounded hover:bg-editor-line text-gray-500 hover:text-white disabled:opacity-30 disabled:cursor-not-allowed transition-colors"
            title="Go to bottom"
          >
            <ChevronDown size={14} />
          </button>
        </div>
      </div>
    </div>
  );
};

export default GerritDiffViewer;
