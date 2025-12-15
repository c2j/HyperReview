import React, { useState } from 'react';
import { MessageSquare, Edit2, Trash2, Reply, Clock, User, Tag } from 'lucide-react';
import type { Comment } from '../api/types';
import { useTranslation } from '../i18n';
import CommentForm from './CommentForm';

interface CommentThreadProps {
  comments: Comment[];
  filePath: string;
  lineNumber: number;
  onCommentAdded: (comment: Comment) => void;
  onCommentUpdated?: (comment: Comment) => void;
  onCommentDeleted?: (commentId: string) => void;
  className?: string;
}

const CommentThread: React.FC<CommentThreadProps> = ({
  comments,
  filePath,
  lineNumber,
  onCommentAdded,
  onCommentUpdated,
  onCommentDeleted,
  className = ''
}) => {
  const { t } = useTranslation();
  const [showReplyForm, setShowReplyForm] = useState(false);
  const [editingCommentId, setEditingCommentId] = useState<string | null>(null);

  // Group comments by thread (parent_id)
  const threadComments = comments.filter(c => c.parent_id === null);
  const replies = comments.filter(c => c.parent_id !== null);

  const getReplies = (parentId: string) => {
    return replies.filter(r => r.parent_id === parentId);
  };

  // Simple time formatting
  const formatTimeAgo = (dateStr: string) => {
    const date = new Date(dateStr);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffMins = Math.floor(diffMs / 60000);
    const diffHours = Math.floor(diffMs / 3600000);
    const diffDays = Math.floor(diffMs / 86400000);

    if (diffMins < 60) return `${diffMins}m ago`;
    if (diffHours < 24) return `${diffHours}h ago`;
    return `${diffDays}d ago`;
  };

  const CommentItem: React.FC<{ comment: Comment; isReply?: boolean }> = ({ comment, isReply = false }) => {
    const [showEditForm, setShowEditForm] = useState(false);

    const handleEdit = () => {
      setEditingCommentId(comment.id);
      setShowEditForm(true);
    };

    const handleCancelEdit = () => {
      setEditingCommentId(null);
      setShowEditForm(false);
    };

    const handleUpdate = (updatedComment: Comment) => {
      onCommentUpdated?.(updatedComment);
      setShowEditForm(false);
      setEditingCommentId(null);
    };

    const handleDelete = () => {
      if (window.confirm(t('comment.delete_confirm'))) {
        onCommentDeleted?.(comment.id);
      }
    };

    const getStatusColor = (status: Comment['status']) => {
      switch (status) {
        case 'Draft':
          return 'bg-gray-500/20 text-gray-400';
        case 'Submitted':
          return 'bg-blue-500/20 text-blue-400';
        case 'Rejected':
          return 'bg-red-500/20 text-red-400';
        default:
          return 'bg-gray-500/20 text-gray-400';
      }
    };

    if (showEditForm && editingCommentId === comment.id) {
      return (
        <div className={`bg-editor-bg border border-editor-line rounded p-3 ${isReply ? 'ml-8' : ''}`}>
          <CommentForm
            filePath={filePath}
            lineNumber={lineNumber}
            initialContent={comment.content}
            onCommentAdded={handleUpdate}
            onCancel={handleCancelEdit}
            placeholder={t('comment.edit_placeholder')}
          />
        </div>
      );
    }

    return (
      <div className={`${isReply ? 'ml-8' : ''}`}>
        <div className="bg-editor-bg border border-editor-line rounded p-3 hover:border-editor-accent/50 transition-colors">
          <div className="flex items-start justify-between mb-2">
            <div className="flex items-center gap-2">
              <div className="w-6 h-6 bg-editor-accent/20 rounded-full flex items-center justify-center">
                <User size={12} className="text-editor-accent" />
              </div>
              <div>
                <div className="text-sm font-medium text-editor-fg">{comment.author}</div>
                <div className="flex items-center gap-2 text-xs text-editor-fg/60">
                  <Clock size={12} />
                  <span>{formatTimeAgo(comment.created_at)}</span>
                  {comment.updated_at !== comment.created_at && (
                    <span className="text-editor-fg/40">
                      (edited {formatTimeAgo(comment.updated_at)})
                    </span>
                  )}
                </div>
              </div>
            </div>

            <div className="flex items-center gap-2">
              <span className={`px-2 py-0.5 rounded text-xs ${getStatusColor(comment.status)}`}>
                {comment.status}
              </span>

              <div className="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
                <button
                  onClick={handleEdit}
                  className="p-1 hover:bg-editor-line rounded transition-colors"
                  title={t('comment.edit')}
                >
                  <Edit2 size={12} />
                </button>
                <button
                  onClick={handleDelete}
                  className="p-1 hover:bg-editor-line rounded transition-colors text-editor-error"
                  title={t('comment.delete')}
                >
                  <Trash2 size={12} />
                </button>
              </div>
            </div>
          </div>

          <div className="text-sm text-editor-fg whitespace-pre-wrap mb-2">
            {comment.content}
          </div>

          {comment.tags && comment.tags.length > 0 && (
            <div className="flex items-center gap-1 flex-wrap mb-2">
              <Tag size={12} className="text-editor-fg/40" />
              {comment.tags.map(tagId => (
                <span
                  key={tagId}
                  className="px-2 py-0.5 bg-editor-accent/10 text-editor-accent text-xs rounded"
                >
                  {tagId}
                </span>
              ))}
            </div>
          )}

          <div className="flex items-center gap-2">
            <button
              onClick={() => setShowReplyForm(!showReplyForm)}
              className="flex items-center gap-1 text-xs text-editor-fg/60 hover:text-editor-accent transition-colors"
            >
              <Reply size={12} />
              {t('comment.reply')}
            </button>
          </div>
        </div>

        {/* Replies */}
        {getReplies(comment.id).map(reply => (
          <div key={reply.id} className="mt-2">
            <CommentItem comment={reply} isReply={true} />
          </div>
        ))}

        {/* Reply form */}
        {showReplyForm && (
          <div className="mt-2 ml-8">
            <CommentForm
              filePath={filePath}
              lineNumber={lineNumber}
              parentCommentId={comment.id}
              onCommentAdded={(newComment) => {
                onCommentAdded(newComment);
                setShowReplyForm(false);
              }}
              onCancel={() => setShowReplyForm(false)}
              placeholder={`Reply to ${comment.author}...`}
            />
          </div>
        )}
      </div>
    );
  };

  return (
    <div className={`space-y-3 ${className}`}>
      {threadComments.length === 0 ? (
        <div className="text-center py-8 text-editor-fg/60">
          <MessageSquare size={24} className="mx-auto mb-2 opacity-50" />
          <p>{t('comment.no_comments')}</p>
        </div>
      ) : (
        threadComments.map(comment => (
          <CommentItem key={comment.id} comment={comment} />
        ))
      )}

      {/* New comment form */}
      <div className="mt-4">
        <CommentForm
          filePath={filePath}
          lineNumber={lineNumber}
          onCommentAdded={onCommentAdded}
          placeholder={t('comment.add_placeholder')}
        />
      </div>
    </div>
  );
};

export default CommentThread;