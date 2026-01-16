import React, { useState } from 'react';
import { Send, CheckCircle2, X, Loader2, AlertCircle } from 'lucide-react';
import { reviewService, SubmitReviewParams, Label } from '../services/reviewService';

interface GerritReviewSubmitProps {
  changeId: string;
  patchSetNumber: number;
  availableComments: string[];
  onClose: () => void;
  onSubmit: (reviewId: string) => void;
}

const GerritReviewSubmit: React.FC<GerritReviewSubmitProps> = ({
  changeId,
  patchSetNumber,
  availableComments,
  onClose,
  onSubmit,
}) => {
  const [selectedCodeReview, setSelectedCodeReview] = useState<Label | null>(null);
  const [selectedVerified, setSelectedVerified] = useState<Label | null>(null);
  const [reviewMessage, setReviewMessage] = useState('');
  const [selectedComments, setSelectedComments] = useState<string[]>([]);
  const [isDraft, setIsDraft] = useState(true);
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [showSuccess, setShowSuccess] = useState(false);

  const codeReviewLabels = reviewService.getCodeReviewLabels();
  const verifiedLabels = reviewService.getVerifiedLabels();

  const handleToggleComment = (commentId: string) => {
    if (selectedComments.includes(commentId)) {
      setSelectedComments(prev => prev.filter(id => id !== commentId));
    } else {
      setSelectedComments(prev => [...prev, commentId]);
    }
  };

  const handleSelectAll = () => {
    setSelectedComments([...availableComments]);
  };

  const handleClearAll = () => {
    setSelectedComments([]);
  };

  const handleSubmit = async () => {
    setError(null);

    const params: SubmitReviewParams = {
      changeId,
      patchSetNumber,
      message: reviewMessage.trim(),
      labels: [
        ...(selectedCodeReview ? [selectedCodeReview] : []),
        ...(selectedVerified ? [selectedVerified] : []),
      ],
      commentIds: selectedComments,
      draft: isDraft,
    };

    const validation = reviewService.validateReview(params);

    if (!validation.valid) {
      setError(validation.errors.join('; '));
      return;
    }

    setSubmitting(true);

    try {
      const result = await reviewService.submitReview(params);

      if (result.success) {
        setShowSuccess(true);
        onSubmit(result.reviewId);

        setTimeout(() => {
          handleClose();
        }, 2000);
      } else {
        setError('Failed to submit review: ' + result.message);
      }
    } catch (err) {
      console.error('Failed to submit review:', err);
      setError('Failed to submit review: ' + (err as Error).message);
    } finally {
      setSubmitting(false);
    }
  };

  const handleClose = () => {
    if (!submitting) {
      onClose();
      setShowSuccess(false);
      setReviewMessage('');
      setSelectedCodeReview(null);
      setSelectedVerified(null);
      setSelectedComments([]);
      setError(null);
    }
  };

  if (showSuccess) {
    return (
      <div className="flex flex-col items-center justify-center h-full">
        <div className="text-center p-8 bg-green-500/10 border border-green-500/30 rounded-lg">
          <CheckCircle2 size={64} className="text-green-400 mb-4 mx-auto" />
          <h3 className="text-lg font-medium text-editor-fg mb-2">Review Submitted!</h3>
          <p className="text-sm text-gray-300 mb-4">
            {selectedComments.length} comment{selectedComments.length !== 1 ? 's' : ''} submitted successfully
          </p>
          <button
            onClick={handleClose}
            className="px-6 py-2 rounded bg-editor-accent text-white hover:bg-blue-600 transition-colors font-medium"
          >
            Close
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full">
      <div className="flex items-start justify-between px-4 py-3 border-b border-editor-line bg-editor-line/20">
        <div>
          <h2 className="text-base font-medium text-editor-fg">Submit Review</h2>
          <p className="text-[10px] text-gray-500">
            Change #{changeId} • Patch Set {patchSetNumber}
          </p>
        </div>
        <button
          onClick={handleClose}
          disabled={submitting}
          className="p-1.5 rounded hover:bg-editor-line text-gray-500 hover:text-white transition-colors disabled:opacity-30 disabled:cursor-not-allowed"
          aria-label="Close"
        >
          <X size={18} />
        </button>
      </div>

      <div className="flex-1 overflow-y-auto px-4 py-4">
        {error && (
          <div className="mb-4 p-3 rounded bg-red-500/10 border border-red-500/30 flex items-start gap-2">
            <AlertCircle size={16} className="flex-shrink-0 mt-0.5 text-red-400" />
            <div className="flex-1">
              <div className="text-xs font-medium text-red-400 mb-1">Error</div>
              <div className="text-[10px] text-red-400">{error}</div>
            </div>
          </div>
        )}

        <div className="mb-6">
          <label className="text-xs text-gray-400 mb-3 block font-medium">
            Code Review Score
          </label>
          <div className="flex gap-2 flex-wrap">
            {codeReviewLabels.map(label => (
              <button
                key={`${label.name}-${label.value}`}
                onClick={() => setSelectedCodeReview(label)}
                className={`px-3 py-2 rounded text-sm font-medium transition-colors ${
                  selectedCodeReview?.name === label.name && selectedCodeReview.value === label.value
                    ? `${label.value < 0 ? 'bg-red-500/20 border-red-500' : 'bg-green-500/20 border-green-500'} text-white`
                    : 'bg-editor-line/50 border-editor-line text-gray-400 hover:text-white hover:border-editor-accent'
                }`}
              >
                {label.value > 0 ? '+' : ''}{label.value}
                {label.name === 'Code-Review' ? '' : ' Review'}
              </button>
            ))}
          </div>
        </div>

        <div className="mb-6">
          <label className="text-xs text-gray-400 mb-3 block font-medium">
            Verified Score
          </label>
          <div className="flex gap-2 flex-wrap">
            {verifiedLabels.map(label => (
              <button
                key={`${label.name}-${label.value}`}
                onClick={() => setSelectedVerified(label)}
                className={`px-3 py-2 rounded text-sm font-medium transition-colors ${
                  selectedVerified?.name === label.name && selectedVerified.value === label.value
                    ? 'bg-green-500/20 border-green-500 text-white'
                    : 'bg-editor-line/50 border-editor-line text-gray-400 hover:text-white hover:border-editor-accent'
                }`}
              >
                {label.value > 0 ? '+' : ''}{label.value}
              </button>
            ))}
          </div>
        </div>

        {availableComments.length > 0 && (
          <div className="mb-6">
            <div className="flex items-center justify-between mb-3">
              <label className="text-xs text-gray-400 font-medium">
                Comments to Include ({selectedComments.length} / {availableComments.length})
              </label>
              <div className="flex gap-2">
                <button
                  onClick={handleSelectAll}
                  disabled={selectedComments.length === availableComments.length}
                  className="px-2 py-1 text-xs rounded hover:bg-editor-line text-gray-500 hover:text-white transition-colors disabled:opacity-30 disabled:cursor-not-allowed"
                >
                  Select All
                </button>
                <button
                  onClick={handleClearAll}
                  disabled={selectedComments.length === 0}
                  className="px-2 py-1 text-xs rounded hover:bg-editor-line text-gray-500 hover:text-white transition-colors disabled:opacity-30 disabled:cursor-not-allowed"
                >
                  Clear
                </button>
              </div>
            </div>
            <div className="flex flex-wrap gap-2">
              {availableComments.map(commentId => (
                <button
                  key={commentId}
                  onClick={() => handleToggleComment(commentId)}
                  className={`px-3 py-1.5 rounded text-xs transition-colors ${
                    selectedComments.includes(commentId)
                      ? 'bg-editor-accent text-white'
                      : 'bg-editor-line/50 border-editor-line text-gray-400 hover:text-white hover:border-editor-accent'
                  }`}
                >
                  {selectedComments.includes(commentId) ? (
                    <CheckCircle2 size={12} className="inline" />
                  ) : null}
                  {' '}
                  Comment #{commentId.substring(0, 8)}
                </button>
              ))}
            </div>
          </div>
        )}

        <div className="mb-6">
          <label htmlFor="message" className="text-xs text-gray-400 mb-3 block font-medium">
            Review Message
          </label>
          <textarea
            id="message"
            value={reviewMessage}
            onChange={(e) => setReviewMessage(e.target.value)}
            placeholder="Add a summary of your review (optional)"
            disabled={submitting}
            rows={3}
            className="w-full bg-editor-line/50 border rounded px-3 py-2 text-sm text-editor-fg placeholder-gray-600 focus:outline-none focus:border-editor-accent resize-none transition-colors"
          />
          <div className="text-[10px] text-gray-500 mt-1">
            {reviewMessage.length} / 10000 characters
          </div>
        </div>

        <div className="flex items-center gap-3 mb-6">
          <label className="flex items-center gap-2 text-xs text-gray-400">
            <input
              type="checkbox"
              checked={!isDraft}
              onChange={(e) => setIsDraft(!e.target.checked)}
              disabled={submitting}
              className="rounded border-editor-line bg-editor-line/30 focus:ring-editor-accent"
            />
            <span>Submit directly (not as draft)</span>
          </label>
          <span className="text-[10px] text-gray-500">
            Drafts can be edited later
          </span>
        </div>
      </div>

      <div className="flex justify-end gap-3 px-4 py-3 border-t border-editor-line bg-editor-line/20">
        <button
          onClick={handleClose}
          disabled={submitting}
          className="px-4 py-2 rounded text-xs hover:bg-editor-line text-gray-300 hover:text-white transition-colors disabled:opacity-30 disabled:cursor-not-allowed"
        >
          Cancel
        </button>
        <button
          onClick={handleSubmit}
          disabled={submitting || (!selectedCodeReview && !selectedVerified && selectedComments.length === 0)}
          className="px-4 py-2 rounded text-xs bg-editor-accent text-white hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed font-medium shadow-sm flex items-center gap-2"
        >
          {submitting ? (
            <>
              <Loader2 size={14} className="animate-spin" />
              Submitting...
            </>
          ) : (
            <>
              <Send size={14} />
              Submit Review
            </>
          )}
        </button>
      </div>
    </div>
  );
};

export default GerritReviewSubmit;
