import React, { useState, useEffect } from 'react';
import { CheckCircle, AlertTriangle, XCircle, HelpCircle, MessageSquare, Send, Clock, User } from 'lucide-react';
import { useTranslation } from '../i18n';
import { useApiClient } from '../api/client';
import type { FileReviewComment } from '../api/types';

export type ReviewType = 'approved' | 'concern' | 'reject' | 'question' | 'comment';

interface ReviewActionModalProps {
  type: ReviewType;
  taskId?: string;
  fileId?: string;
  filePath?: string;
  lineNumber?: number;
  onClose: () => void;
  onSubmit: (text: string, type: ReviewType, taskId?: string, fileId?: string) => void;
}

const ReviewActionModal: React.FC<ReviewActionModalProps> = ({ type, taskId, fileId, filePath, lineNumber, onClose, onSubmit }) => {
  const { t } = useTranslation();
  const { getFileReviewComments } = useApiClient();
  const [text, setText] = useState('');
  const [historyComments, setHistoryComments] = useState<FileReviewComment[]>([]);

  // Fetch comment history when taskId and fileId are provided
  useEffect(() => {
    if (taskId && fileId) {
      getFileReviewComments(taskId, fileId)
        .then(setHistoryComments)
        .catch(console.error);
    }
  }, [taskId, fileId, getFileReviewComments]);

  const config = {
    approved: { icon: CheckCircle, color: 'text-editor-success', title: '文件通过审核', placeholder: '添加审核通过的意见（可选）...' },
    concern: { icon: AlertTriangle, color: 'text-editor-warning', title: t('modal.review_action.concern'), placeholder: 'Describe the potential issue (e.g., Performance, Security)...' },
    reject: { icon: XCircle, color: 'text-editor-error', title: t('modal.review_action.reject'), placeholder: 'Explain why this code must be changed...' },
    question: { icon: HelpCircle, color: 'text-editor-info', title: t('modal.review_action.question'), placeholder: 'What needs clarification?' },
    comment: { icon: MessageSquare, color: 'text-gray-400', title: t('modal.review_action.comment'), placeholder: 'Add a neutral observation or note...' },
  }[type];

  const Icon = config.icon;

  // Get status config for history comments
  const getStatusConfig = (status: string) => {
    switch (status) {
      case 'approved': return { icon: CheckCircle, color: 'text-editor-success', label: '通过' };
      case 'concern': return { icon: AlertTriangle, color: 'text-editor-warning', label: '关注' };
      case 'must_change': return { icon: XCircle, color: 'text-editor-error', label: '必须改' };
      case 'question': return { icon: HelpCircle, color: 'text-editor-info', label: '提问' };
      default: return { icon: MessageSquare, color: 'text-gray-400', label: status };
    }
  };

  // Format timestamp
  const formatTime = (timestamp: string) => {
    try {
      const date = new Date(timestamp);
      return date.toLocaleString('zh-CN', { month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit' });
    } catch {
      return timestamp;
    }
  };

  return (
    <div className="flex flex-col gap-4">
      <div className="flex items-center gap-2 mb-1">
        <Icon className={config.color} size={18} />
        <span className={`font-bold text-sm ${config.color.replace('text-', 'text-')}`}>{config.title}</span>
      </div>

      <div className="bg-editor-line/30 rounded p-2 border border-editor-line/50 text-xs font-mono text-gray-400">
        {filePath || '未指定文件'}{lineNumber !== undefined && <span className="text-editor-accent">:{lineNumber}</span>}
      </div>

      {/* Comment History */}
      {historyComments.length > 0 && (
        <div className="flex flex-col gap-2">
          <div className="text-xs font-medium text-gray-400">历史评论</div>
          <div className="max-h-40 overflow-y-auto flex flex-col gap-2">
            {historyComments.map((comment) => {
              const statusConfig = getStatusConfig(comment.reviewStatus);
              const StatusIcon = statusConfig.icon;
              return (
                <div key={comment.id} className="bg-editor-line/20 rounded border border-editor-line/30 p-2 text-xs">
                  <div className="flex items-center gap-2 mb-1">
                    <StatusIcon className={statusConfig.color} size={12} />
                    <span className={`font-medium ${statusConfig.color}`}>{statusConfig.label}</span>
                    <span className="ml-auto text-[10px] text-gray-500 flex items-center gap-1">
                      <Clock size={10} />
                      {formatTime(comment.submittedAt)}
                    </span>
                  </div>
                  {comment.reviewComment && (
                    <div className="text-gray-300 mt-1 pl-4 border-l-2 border-editor-line/30">
                      {comment.reviewComment}
                    </div>
                  )}
                  <div className="text-[10px] text-gray-500 mt-1 flex items-center gap-1">
                    <User size={10} />
                    {comment.submittedBy || 'Anonymous'}
                  </div>
                </div>
              );
            })}
          </div>
        </div>
      )}

      <textarea
        className="w-full h-32 bg-editor-bg border border-editor-line rounded p-3 text-sm text-white placeholder-gray-600 focus:outline-none focus:border-editor-accent resize-none font-mono leading-relaxed"
        placeholder={config.placeholder}
        value={text}
        onChange={e => setText(e.target.value)}
        autoFocus
      />

      <div className="flex justify-between items-center pt-2">
        <div className="text-[10px] text-gray-500">{t('modal.review_action.markdown')}</div>
        <div className="flex gap-2">
          <button onClick={onClose} className="px-4 py-1.5 rounded text-xs hover:bg-editor-line text-gray-300 transition-colors">{t('modal.review_action.cancel')}</button>
          <button
            onClick={() => onSubmit(text, type, taskId, fileId)}
            disabled={type !== 'approved' && !text}
            className={`px-4 py-1.5 rounded text-xs text-white flex items-center gap-2 font-medium shadow-sm transition-colors disabled:opacity-50 disabled:cursor-not-allowed
              ${type === 'reject' ? 'bg-editor-error hover:bg-red-600' :
                type === 'concern' ? 'bg-editor-warning hover:bg-orange-600 text-black' :
                type === 'approved' ? 'bg-editor-success hover:bg-green-600' :
                'bg-editor-accent hover:bg-blue-600'}`}
          >
            <Send size={12} />
            {type === 'approved' ? '确认' : t('modal.review_action.post')}
          </button>
        </div>
      </div>
    </div>
  );
};

export default ReviewActionModal;