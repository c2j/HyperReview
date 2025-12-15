import React from 'react';
import { useTranslation } from '../i18n';

const TestTranslation: React.FC = () => {
  const { t } = useTranslation();

  return (
    <div className="p-4 bg-editor-bg text-white">
      <h3 className="text-lg mb-4">Translation Test</h3>
      <div className="space-y-2">
        <div>Original: rightpanel.tab.tasks</div>
        <div>Translated: {t('rightpanel.tab.tasks')}</div>
        <div>Original: rightpanel.tab.templates</div>
        <div>Translated: {t('rightpanel.tab.templates')}</div>
        <div>Original: comment.placeholder</div>
        <div>Translated: {t('comment.placeholder')}</div>
      </div>
    </div>
  );
};

export default TestTranslation;