
import React, { useState } from 'react';
import { MessageSquare, GitPullRequest, CheckCircle } from 'lucide-react';
import { useTranslation } from '../../i18n';

interface RemoteRightPanelProps {
  onAction: (msg: string) => void;
}

enum RemoteTab {
  COMMENTS = 'comments',
  REVIEWS = 'reviews',
  INFO = 'info'
}

const RemoteRightPanel: React.FC<RemoteRightPanelProps> = () => {
  const { t } = useTranslation();
  const [activeTab, setActiveTab] = useState<RemoteTab>(RemoteTab.COMMENTS);

  return (
    <div className="h-full bg-editor-sidebar border-l border-editor-line flex flex-col overflow-hidden">
      <div className="flex border-b border-editor-line bg-editor-bg shrink-0">
        <button
          onClick={() => setActiveTab(RemoteTab.COMMENTS)}
          className={`flex-1 py-2 flex justify-center items-center border-b-2 transition-colors ${activeTab === RemoteTab.COMMENTS ? 'border-purple-500 text-white' : 'border-transparent text-gray-500 hover:text-gray-300'}`}
          title={t('rightpanel.tab.comments')}
        >
          <MessageSquare size={16} />
        </button>
        <button
          onClick={() => setActiveTab(RemoteTab.REVIEWS)}
          className={`flex-1 py-2 flex justify-center items-center border-b-2 transition-colors ${activeTab === RemoteTab.REVIEWS ? 'border-purple-500 text-white' : 'border-transparent text-gray-500 hover:text-gray-300'}`}
          title={t('rightpanel.tab.reviews')}
        >
          <CheckCircle size={16} />
        </button>
        <button
          onClick={() => setActiveTab(RemoteTab.INFO)}
          className={`flex-1 py-2 flex justify-center items-center border-b-2 transition-colors ${activeTab === RemoteTab.INFO ? 'border-purple-500 text-white' : 'border-transparent text-gray-500 hover:text-gray-300'}`}
          title={t('rightpanel.tab.info')}
        >
          <GitPullRequest size={16} />
        </button>
      </div>

      <div className="flex-1 overflow-y-auto p-4 custom-scrollbar">
        {activeTab === RemoteTab.COMMENTS && (
          <section className="animate-fade-in">
            <h3 className="text-[10px] font-bold text-gray-500 uppercase tracking-widest mb-4 flex items-center gap-2">
              <MessageSquare size={14} className="text-purple-400" />
              {t('rightpanel.comments')}
            </h3>
            <div className="text-center text-gray-500 text-sm py-8">
              Comments will be displayed here
            </div>
          </section>
        )}

        {activeTab === RemoteTab.REVIEWS && (
          <section className="animate-fade-in">
            <h3 className="text-[10px] font-bold text-gray-500 uppercase tracking-widest mb-4 flex items-center gap-2">
              <CheckCircle size={14} className="text-purple-400" />
              {t('rightpanel.reviews')}
            </h3>
            <div className="text-center text-gray-500 text-sm py-8">
              Reviews will be displayed here
            </div>
          </section>
        )}

        {activeTab === RemoteTab.INFO && (
          <section className="animate-fade-in">
            <h3 className="text-[10px] font-bold text-gray-500 uppercase tracking-widest mb-4 flex items-center gap-2">
              <GitPullRequest size={14} className="text-purple-400" />
              {t('rightpanel.change_info')}
            </h3>
            <div className="text-center text-gray-500 text-sm py-8">
              Change information will be displayed here
            </div>
          </section>
        )}
      </div>
    </div>
  );
};

export default RemoteRightPanel;
