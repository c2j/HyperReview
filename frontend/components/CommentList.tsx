import React, { useState, useEffect } from 'react';
import { MessageSquare, Clock, User, AlertCircle, Loader2, RefreshCw, Filter } from 'lucide-react';
import { commentService, SimpleComment } from '../services/commentService';

interface CommentListProps {
  changeId: string;
  filePath?: string;
  showResolved?: boolean;
  onReplyToComment?: (comment: SimpleComment) => void;
}

const CommentList: React.FC<CommentListProps> = ({
  changeId,
  filePath,
  showResolved = false,
  onReplyToComment,
}) => {
  const [comments, setComments] = useState<SimpleComment[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [filter, setFilter] = useState<'all' | 'unresolved'>('all');
  const [sortBy, setSortBy] = useState<'newest' | 'oldest'>('newest');

  const loadComments = async () => {
    setLoading(true);
    setError(null);

    try {
      const result = await commentService.getComments(changeId);
      setComments(result.comments);
    } catch (err) {
      console.error('Failed to load comments:', err);
      setError('Failed to load comments: ' + (err as Error).message);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadComments();
  }, [changeId, filePath, showResolved]);

  const filteredComments = comments.filter(comment => {
    if (filter === 'unresolved' && !comment.unresolved) {
      return false;
    }
    if (filePath && comment.filePath !== filePath) {
      return false;
    }
    return true;
  });

  const sortedComments = [...filteredComments].sort((a, b) => {
    const dateA = new Date(a.created).getTime();
    const dateB = new Date(b.created).getTime();
    return sortBy === 'newest' ? dateB - dateA : dateA - dateB;
  });

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffMins = Math.floor(diffMs / (1000 * 60));
    const diffHours = Math.floor(diffMins / 60);
    const diffDays = Math.floor(diffHours / 24);

    if (diffMins < 1) {
      return 'Just now';
    } else if (diffMins < 60) {
      return `${diffMins}m ago`;
    } else if (diffHours < 24) {
      return `${diffHours}h ago`;
    } else if (diffDays < 7) {
      return `${diffDays}d ago`;
    } else {
      return date.toLocaleDateString();
    }
  };

  const groupedComments = sortedComments.reduce((groups, comment) => {
    const key = comment.filePath;
    if (!groups[key]) {
      groups[key] = [];
    }
    groups[key].push(comment);
    return groups;
  }, {} as Record<string, SimpleComment[]>);

  return (
    <div className="flex flex-col h-full">
      <div className="flex items-center justify-between px-4 py-3 border-b border-editor-line bg-editor-line/20">
        <div className="flex items-center gap-3">
          <div className="flex items-center gap-2">
            <MessageSquare size={18} className="text-editor-accent" />
            <h3 className="text-sm font-medium text-editor-fg">Comments</h3>
            <span className="text-xs text-gray-500">
              {sortedComments.length} comment{sortedComments.length !== 1 ? 's' : ''}
            </span>
          </div>
          {filteredComments.length < sortedComments.length && (
            <span className="text-[10px] text-gray-500">
              Showing {filteredComments.length} of {sortedComments.length}
            </span>
          )}
        </div>
        <div className="flex items-center gap-2">
          <button
            onClick={loadComments}
            disabled={loading}
            className="p-1.5 rounded hover:bg-editor-line text-gray-500 hover:text-white transition-colors disabled:opacity-30 disabled:cursor-not-allowed"
            title="Refresh comments"
          >
            {loading ? (
              <Loader2 size={16} className="animate-spin" />
            ) : (
              <RefreshCw size={16} />
            )}
          </button>
        </div>
      </div>

      <div className="flex gap-2 px-4 py-2 border-b border-editor-line bg-editor-line/10">
        <button
          onClick={() => setFilter('all')}
          className={`px-3 py-1.5 rounded text-xs font-medium transition-colors ${
            filter === 'all'
              ? 'bg-editor-accent text-white'
              : 'bg-editor-line text-gray-400 hover:text-white'
          }`}
        >
          All
        </button>
        <button
          onClick={() => setFilter('unresolved')}
          className={`px-3 py-1.5 rounded text-xs font-medium transition-colors ${
            filter === 'unresolved'
              ? 'bg-editor-accent text-white'
              : 'bg-editor-line text-gray-400 hover:text-white'
          }`}
        >
          Unresolved
        </button>
        <div className="flex-1" />
        <button
          onClick={() => setSortBy(sortBy === 'newest' ? 'oldest' : 'newest')}
          className="p-1.5 rounded hover:bg-editor-line text-gray-500 hover:text-white transition-colors flex items-center gap-1"
          title={`Sort by ${sortBy === 'newest' ? 'oldest' : 'newest'}`}
        >
          <Clock size={14} />
          <span className="text-xs">{sortBy === 'newest' ? 'Newest' : 'Oldest'}</span>
        </button>
      </div>

      {error && (
        <div className="mx-4 mt-4 p-3 rounded bg-red-500/10 border border-red-500/30 flex items-start gap-2">
          <AlertCircle size={16} className="flex-shrink-0 mt-0.5 text-red-400" />
          <div className="flex-1">
            <div className="text-xs font-medium text-red-400">Error</div>
            <div className="text-[10px] text-red-400 mt-0.5">{error}</div>
          </div>
        </div>
      )}

      {loading && sortedComments.length === 0 && (
        <div className="flex flex-col items-center justify-center py-12">
          <Loader2 size={32} className="animate-spin text-editor-accent" />
          <div className="text-sm text-gray-400 mt-3">Loading comments...</div>
        </div>
      )}

      {!loading && sortedComments.length === 0 && (
        <div className="flex flex-col items-center justify-center py-12">
          <MessageSquare size={32} className="text-gray-500 mb-3" />
          <div className="text-sm text-gray-400 mb-1">No comments yet</div>
          <div className="text-xs text-gray-500">
            Be the first to review this change!
          </div>
        </div>
      )}

      {!loading && sortedComments.length > 0 && (
        <div className="flex-1 overflow-y-auto">
          {Object.entries(groupedComments).map(([filePath, fileComments]) => (
            <div key={filePath} className="mb-6">
              <div className="px-4 py-2 bg-editor-line/30 border-b border-editor-line/50">
                <div className="flex items-center gap-2">
                  <Filter size={14} className="text-gray-500" />
                  <span className="text-xs font-medium text-gray-300 font-mono">{filePath}</span>
                  <span className="text-[10px] text-gray-500">
                    {fileComments.length} comment{fileComments.length !== 1 ? 's' : ''}
                  </span>
                </div>
              </div>

              <div className="flex flex-col gap-2 px-4 py-3">
                {fileComments.map(comment => (
                  <div
                    key={comment.id}
                    className={`p-3 rounded border transition-all ${
                      comment.unresolved
                        ? 'bg-yellow-500/5 border-yellow-500/20'
                        : 'bg-editor-line/30 border-editor-line/50'
                    }`}
                  >
                    <div className="flex items-start gap-3 mb-2">
                      <div className="flex-shrink-0 w-8 h-8 rounded-full bg-editor-accent/20 flex items-center justify-center">
                        <User size={16} className="text-editor-accent" />
                      </div>
                      <div className="flex-1">
                        <div className="text-[10px] text-gray-500 mb-1">
                          {comment.author}
                          {' • '}
                          {formatDate(comment.created)}
                        </div>
                        <div className="text-sm text-editor-fg whitespace-pre-wrap">
                          {comment.message}
                        </div>
                      </div>
                    </div>

                    <div className="flex items-center gap-3 text-[10px] text-gray-500">
                      <div>
                        Line {comment.line}
                        {comment.filePath !== filePath && (
                          <span className="ml-1 text-gray-400">in {comment.filePath}</span>
                        )}
                      </div>
                      <div className="flex-1" />
                      {onReplyToComment && (
                        <button
                          onClick={() => onReplyToComment(comment)}
                          className="px-2 py-1 rounded hover:bg-editor-line text-gray-500 hover:text-white transition-colors text-xs font-medium"
                        >
                          Reply
                        </button>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default CommentList;
