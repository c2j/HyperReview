import React, { useState } from 'react';
import { useApiClient } from '../api/client';
import type { Comment } from '../api/types';
import { Send, Loader2 } from 'lucide-react';
import { useTranslation } from '../i18n';

interface CommentFormProps {
  filePath: string;
  lineNumber: number;
  onCommentAdded: (comment: Comment) => void;
  onCancel?: () => void;
  placeholder?: string;
  initialContent?: string;
  parentCommentId?: string;
}

const CommentForm: React.FC<CommentFormProps> = ({
  filePath,
  lineNumber,
  onCommentAdded,
  onCancel,
  placeholder,
  initialContent = ''
}) => {
  const { t } = useTranslation();
  const { addComment } = useApiClient();

  const [content, setContent] = useState(initialContent);
  const [submitting, setSubmitting] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!content.trim()) {
      return;
    }

    try {
      setSubmitting(true);

      const result = await addComment(filePath, lineNumber, content);

      if (result) {
        onCommentAdded(result);
        setContent('');

        // Call onCancel if provided to close the form
        if (onCancel) {
          onCancel();
        }
      }
    } catch (error) {
      console.error('Failed to add comment:', error);
      // Handle error - you might want to show a toast here
    } finally {
      setSubmitting(false);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    // Submit on Ctrl+Enter or Cmd+Enter
    if ((e.ctrlKey || e.metaKey) && e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSubmit(e as any);
    }

    // Cancel on Escape
    if (e.key === 'Escape' && onCancel) {
      e.preventDefault();
      onCancel();
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-3">
      <div className="relative">
        <textarea
          value={content}
          onChange={(e) => setContent(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder={placeholder || t('comment.placeholder')}
          className="w-full p-3 bg-editor-bg border border-editor-line rounded text-sm text-editor-fg
                     focus:outline-none focus:border-editor-accent resize-none"
          rows={3}
          disabled={submitting}
          autoFocus
        />
        <div className="absolute bottom-2 right-2 text-xs text-editor-fg/40">
          {content.length}/5000
        </div>
      </div>

      <div className="flex items-center justify-between">
        <div className="text-xs text-editor-fg/60">
          {t('comment.shortcuts')}
        </div>

        <div className="flex items-center gap-2">
          {onCancel && (
            <button
              type="button"
              onClick={onCancel}
              disabled={submitting}
              className="px-3 py-1.5 text-sm text-editor-fg hover:bg-editor-line rounded transition-colors"
            >
              {t('common.cancel')}
            </button>
          )}

          <button
            type="submit"
            disabled={submitting || !content.trim() || content.length > 5000}
            className="flex items-center gap-2 px-3 py-1.5 text-sm bg-editor-accent text-white rounded
                       hover:bg-editor-accent/80 disabled:opacity-50 disabled:cursor-not-allowed
                       transition-colors"
          >
            {submitting ? (
              <>
                <Loader2 size={14} className="animate-spin" />
                {t('comment.adding')}
              </>
            ) : (
              <>
                <Send size={14} />
                {t('comment.add')}
              </>
            )}
          </button>
        </div>
      </div>
    </form>
  );
};

export default CommentForm;