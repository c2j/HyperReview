import React, { useState } from 'react';
import { MessageSquare, X, Loader2, AlertCircle, CheckCircle2 } from 'lucide-react';
import { commentService, CreateCommentParams } from '../services/commentService';

interface CommentCreatorProps {
  changeId: string;
  filePath: string;
  patchSetNumber: number;
  line: number;
  onClose: () => void;
  onSubmit: (commentId: string) => void;
}

const CommentCreator: React.FC<CommentCreatorProps> = ({
  changeId,
  filePath,
  line,
  onClose,
  onSubmit,
}) => {
  const [message, setMessage] = useState('');
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [showSuccess, setShowSuccess] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!message.trim()) {
      setError('Comment message cannot be empty');
      return;
    }

    setSubmitting(true);
    setError(null);

    try {
      const params: CreateCommentParams = {
        changeId,
        filePath,
        line,
        message: message.trim(),
      };

      const result = await commentService.createComment(params);

      if (result.success) {
        setShowSuccess(true);
        onSubmit(result.id);

        setTimeout(() => {
          handleClose();
        }, 1500);
      } else {
        setError('Failed to create comment');
      }
    } catch (err) {
      console.error('Failed to create comment:', err);
      setError((err as Error).message);
    } finally {
      setSubmitting(false);
    }
  };

  const handleClose = () => {
    if (!submitting) {
      setMessage('');
      setError(null);
      setShowSuccess(false);
      onClose();
    }
  };

  return (
    <div className="relative">
      <button
        onClick={() => {
          if (!showSuccess && !error) {
            setShowSuccess(false);
          }
        }}
        className="absolute inset-0 z-0"
        aria-label="Close"
      />

      <div
        onClick={(e) => e.stopPropagation()}
        className="relative z-10 bg-editor-bg border border-editor-line rounded-lg shadow-xl max-w-md w-full"
      >
        <div className="flex items-start justify-between p-4 border-b border-editor-line">
          <div className="flex items-center gap-2">
            <MessageSquare size={20} className="text-editor-accent" />
            <div>
              <h3 className="text-sm font-medium text-editor-fg">Add Comment</h3>
              <div className="text-[10px] text-gray-500">
                {filePath}:{line}
              </div>
            </div>
          </div>
          <button
            onClick={handleClose}
            disabled={submitting}
            className="p-1 rounded hover:bg-editor-line text-gray-500 hover:text-white disabled:opacity-30 disabled:cursor-not-allowed transition-colors"
            aria-label="Close"
          >
            <X size={16} />
          </button>
        </div>

        <form onSubmit={handleSubmit} className="p-4">
          <div className="mb-4">
            <label htmlFor="comment" className="text-xs text-gray-400 mb-2 block font-medium">
              Your Comment
            </label>
            <textarea
              id="comment"
              value={message}
              onChange={(e) => setMessage(e.target.value)}
              placeholder="Share your feedback, suggestions, or questions..."
              disabled={submitting || showSuccess}
              rows={4}
              className={`w-full bg-editor-line/50 border rounded px-3 py-2 text-sm text-editor-fg placeholder-gray-600 focus:outline-none transition-colors resize-none ${
                error
                  ? 'border-red-500 focus:border-red-500'
                  : 'border-editor-line focus:border-editor-accent'
              }`}
            />
            <div className="text-[10px] text-gray-500 mt-1">
              {message.length} / 10000 characters
            </div>
          </div>

          {error && (
            <div className="mb-4 p-3 rounded bg-red-500/10 border border-red-500/30 flex items-start gap-2">
              <AlertCircle size={16} className="flex-shrink-0 mt-0.5 text-red-400" />
              <div className="flex-1">
                <div className="text-xs font-medium text-red-400">Error</div>
                <div className="text-[10px] text-red-400 mt-0.5">{error}</div>
              </div>
            </div>
          )}

          {showSuccess && (
            <div className="mb-4 p-3 rounded bg-green-500/10 border border-green-500/30 flex items-start gap-2">
              <CheckCircle2 size={16} className="flex-shrink-0 mt-0.5 text-green-400" />
              <div className="flex-1">
                <div className="text-xs font-medium text-green-400">Success</div>
                <div className="text-[10px] text-green-400 mt-0.5">
                  Comment created successfully!
                </div>
              </div>
            </div>
          )}

          <div className="flex justify-end gap-2">
            <button
              type="button"
              onClick={handleClose}
              disabled={submitting}
              className="px-4 py-2 rounded text-xs hover:bg-editor-line text-gray-300 hover:text-white disabled:opacity-30 disabled:cursor-not-allowed transition-colors"
            >
              Cancel
            </button>
            <button
              type="submit"
              disabled={submitting || !message.trim() || showSuccess}
              className="px-4 py-2 rounded text-xs bg-editor-accent text-white hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed transition-colors font-medium shadow-sm flex items-center gap-2"
            >
              {submitting ? (
                <>
                  <Loader2 size={14} className="animate-spin" />
                  Submitting...
                </>
              ) : showSuccess ? (
                <>
                  <CheckCircle2 size={14} />
                  Submitted
                </>
              ) : (
                <>
                  <MessageSquare size={14} />
                  Submit Comment
                </>
              )}
            </button>
          </div>
        </form>

        <div className="px-4 py-2 border-t border-editor-line bg-editor-line/20">
          <div className="text-[10px] text-gray-500">
            <span className="font-medium">Tips:</span> Use Markdown formatting. Comments can be edited until synced to Gerrit.
          </div>
        </div>
      </div>
    </div>
  );
};

export default CommentCreator;
