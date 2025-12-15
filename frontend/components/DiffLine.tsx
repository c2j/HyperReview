import React, { useState } from 'react';
import type { DiffLine as DiffLineType, Comment } from '../api/types';
import { MessageSquare, AlertTriangle, XCircle, Info, Check } from 'lucide-react';
import CommentThread from './CommentThread';
import CommentForm from './CommentForm';

interface DiffLineProps {
  line: DiffLineType;
  index: number;
  isSelected: boolean;
  hasComment: boolean;
  comments: Comment[];
  onLineClick: (line: DiffLineType, index: number) => void;
  onCommentAdded: (comment: Comment) => void;
  onCommentUpdated?: (comment: Comment) => void;
  onCommentDeleted?: (commentId: string) => void;
  showComments: boolean;
  className?: string;
}

const DiffLine: React.FC<DiffLineProps> = ({
  line,
  index,
  isSelected,
  hasComment,
  comments,
  onLineClick,
  onCommentAdded,
  onCommentUpdated,
  onCommentDeleted,
  showComments,
  className = ''
}) => {
  const [showCommentForm, setShowCommentForm] = useState(false);

  const handleLineClick = () => {
    onLineClick(line, index);
  };

  const handleAddComment = () => {
    setShowCommentForm(true);
  };

  const getLineStyle = () => {
    const baseStyle = 'flex items-start px-4 py-1 cursor-pointer transition-colors ';

    switch (line.line_type) {
      case 'Added':
        return baseStyle + 'bg-green-500/10 border-l-4 border-green-500 hover:bg-green-500/20';
      case 'Removed':
        return baseStyle + 'bg-red-500/10 border-l-4 border-red-500 hover:bg-red-500/20';
      case 'Header':
        return baseStyle + 'bg-blue-500/10 border-l-4 border-blue-500 font-bold';
      case 'Context':
      default:
        return baseStyle + 'hover:bg-editor-selection/20';
    }
  };

  const getSeverityIcon = () => {
    if (!line.severity) return null;

    const iconProps = { size: 14, className: 'inline mr-1' };

    switch (line.severity) {
      case 'Error':
        return <XCircle {...iconProps} className={`${iconProps.className} text-red-400`} />;
      case 'Warning':
        return <AlertTriangle {...iconProps} className={`${iconProps.className} text-yellow-400`} />;
      case 'Info':
        return <Info {...iconProps} className={`${iconProps.className} text-blue-400`} />;
      case 'Success':
        return <Check {...iconProps} className={`${iconProps.className} text-green-400`} />;
      default:
        return null;
    }
  };

  const getSeverityBadge = () => {
    if (!line.severity) return null;

    const severityStyles = {
      Error: 'bg-red-500/20 text-red-400 border-red-500/30',
      Warning: 'bg-yellow-500/20 text-yellow-400 border-yellow-500/30',
      Info: 'bg-blue-500/20 text-blue-400 border-blue-500/30',
      Success: 'bg-green-500/20 text-green-400 border-green-500/30'
    };

    return (
      <span className={`px-2 py-0.5 rounded text-xs font-mono border ${severityStyles[line.severity]}`}>
        {line.severity}
      </span>
    );
  };

  return (
    <div className={className}>
      {/* Main diff line */}
      <div
        className={`${getLineStyle()} ${isSelected ? 'ring-2 ring-editor-accent' : ''}`}
        onClick={handleLineClick}
      >
        {/* Line numbers */}
        <div className="flex items-center gap-4 w-32 flex-shrink-0 text-xs text-editor-fg/60 font-mono">
          <span className="w-12 text-right">
            {line.old_line_number || ''}
          </span>
          <span className="w-12 text-right">
            {line.new_line_number || ''}
          </span>
        </div>

        {/* Content */}
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2">
            {getSeverityIcon()}
            <pre className="text-sm text-editor-fg whitespace-pre-wrap font-mono leading-relaxed">
              {line.content}
            </pre>
          </div>

          {line.message && (
            <div className="mt-1 text-xs text-editor-fg/70 bg-editor-line/30 rounded px-2 py-1">
              {getSeverityBadge()}
              <span className="ml-2">{line.message}</span>
            </div>
          )}

          {line.hunk_header && (
            <div className="mt-1 text-xs text-editor-fg/60 font-mono">
              {line.hunk_header}
            </div>
          )}
        </div>

        {/* Comment indicator */}
        {hasComment && (
          <div className="flex items-center gap-2 ml-4">
            <MessageSquare
              size={14}
              className="text-editor-accent cursor-pointer"
              onClick={(e) => {
                e.stopPropagation();
                handleAddComment();
              }}
            />
            <span className="text-xs text-editor-fg/60">
              {comments.length}
            </span>
          </div>
        )}

        {/* Add comment button (visible on hover) */}
        {!hasComment && (
          <div
            className="ml-4 opacity-0 hover:opacity-100 transition-opacity"
            onClick={(e) => {
              e.stopPropagation();
              handleAddComment();
            }}
          >
            <MessageSquare
              size={14}
              className="text-editor-fg/40 hover:text-editor-accent cursor-pointer"
            />
          </div>
        )}
      </div>

      {/* Comment section (shown when selected or when comments exist) */}
      {(showComments || isSelected) && (hasComment || showCommentForm) && (
        <div className="ml-16 mt-2 p-3 bg-editor-line/20 border-l-4 border-editor-accent rounded-r">
          {hasComment && (
            <CommentThread
              comments={comments}
              filePath={comments[0]?.file_path || ''}
              lineNumber={comments[0]?.line_number || 0}
              onCommentAdded={onCommentAdded}
              onCommentUpdated={onCommentUpdated}
              onCommentDeleted={onCommentDeleted}
            />
          )}

          {showCommentForm && !hasComment && (
            <CommentForm
              filePath={comments[0]?.file_path || ''}
              lineNumber={comments[0]?.line_number || 0}
              onCommentAdded={(comment: Comment) => {
                onCommentAdded(comment);
                setShowCommentForm(false);
              }}
              onCancel={() => setShowCommentForm(false)}
            />
          )}
        </div>
      )}
    </div>
  );
};

export default DiffLine;