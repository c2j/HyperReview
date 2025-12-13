import React, { useState } from 'react';
import { AlertTriangle, XCircle, HelpCircle, MessageSquare, Send } from 'lucide-react';
import { useTranslation } from '../i18n';

export type ReviewType = 'concern' | 'reject' | 'question' | 'comment';

interface ReviewActionModalProps {
  type: ReviewType;
  onClose: () => void;
  onSubmit: (text: string, type: ReviewType) => void;
}

const ReviewActionModal: React.FC<ReviewActionModalProps> = ({ type, onClose, onSubmit }) => {
  const { t } = useTranslation();
  const [text, setText] = useState('');

  const config = {
    concern: { icon: AlertTriangle, color: 'text-editor-warning', title: t('modal.review_action.concern'), placeholder: 'Describe the potential issue (e.g., Performance, Security)...' },
    reject: { icon: XCircle, color: 'text-editor-error', title: t('modal.review_action.reject'), placeholder: 'Explain why this code must be changed...' },
    question: { icon: HelpCircle, color: 'text-editor-info', title: t('modal.review_action.question'), placeholder: 'What needs clarification?' },
    comment: { icon: MessageSquare, color: 'text-gray-400', title: t('modal.review_action.comment'), placeholder: 'Add a neutral observation or note...' },
  }[type];

  const Icon = config.icon;

  return (
    <div className="flex flex-col gap-4">
      <div className="flex items-center gap-2 mb-1">
        <Icon className={config.color} size={18} />
        <span className={`font-bold text-sm ${config.color.replace('text-', 'text-')}`}>{config.title}</span>
      </div>
      
      <div className="bg-editor-line/30 rounded p-2 border border-editor-line/50 text-xs font-mono text-gray-400">
        src/main/java/.../RetryServiceImpl.java <span className="text-editor-accent">:127</span>
      </div>

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
            onClick={() => text && onSubmit(text, type)}
            disabled={!text}
            className={`px-4 py-1.5 rounded text-xs text-white flex items-center gap-2 font-medium shadow-sm transition-colors disabled:opacity-50 disabled:cursor-not-allowed
              ${type === 'reject' ? 'bg-editor-error hover:bg-red-600' : 
                type === 'concern' ? 'bg-editor-warning hover:bg-orange-600 text-black' : 
                'bg-editor-accent hover:bg-blue-600'}`}
          >
            <Send size={12} />
            {t('modal.review_action.post')}
          </button>
        </div>
      </div>
    </div>
  );
};

export default ReviewActionModal;