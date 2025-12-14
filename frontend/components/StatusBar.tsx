import React, { useState, useEffect } from 'react';
import { Cloud, Radio, Loader2 } from 'lucide-react';
import { useTranslation } from '../i18n';
import { useApiClient } from '../api/client';
import type { ReviewStats } from '../api/types';

const StatusBar: React.FC = () => {
  const { t } = useTranslation();
  const { getReviewStats } = useApiClient();
  const [stats, setStats] = useState<ReviewStats | null>(null);

  useEffect(() => {
      getReviewStats().then(setStats).catch(console.error);
  }, []);

  return (
    <div className="h-[28px] bg-editor-accent text-white flex items-center px-4 justify-between text-[11px] select-none shrink-0 z-50 overflow-hidden whitespace-nowrap">
      {/* Left Section: Flexible, truncates path first */}
      <div className="flex items-center gap-4 min-w-0 flex-1 mr-4">
        <span className="font-bold flex items-center gap-1 shrink-0">
            <Radio size={10} className="animate-pulse" />
            {t('statusbar.live')}
        </span>
        {/* Repo Path: Primary candidate for truncation */}
        <span className="opacity-80 truncate min-w-[50px]" title="/payment-service/feature/payment-retry">
            [/payment-service/feature/payment-retry]
        </span>
        
        {stats ? (
            <>
                <span className="shrink-0">{stats.total_files} {t('statusbar.files')}</span>
                <span className="shrink-0">→</span>
                <span className="shrink-0 truncate">
                    {t('statusbar.marked')} {stats.reviewed_files} (✗{stats.severe_issues} ⚠{stats.total_comments - stats.severe_issues} ❓{stats.pending_files})
                </span>
            </>
        ) : (
            <span className="shrink-0 opacity-50 flex items-center gap-1">
                <Loader2 size={10} className="animate-spin" /> Loading stats...
            </span>
        )}
      </div>

      {/* Right Section: Stays visible mostly */}
      <div className="flex items-center gap-4 shrink-0">
         <span className="hidden md:block truncate max-w-[200px] opacity-80">{t('statusbar.ready')}</span>
         <span className="flex items-center gap-1 opacity-80 shrink-0">
            <Cloud size={10} /> {t('statusbar.saved')}
         </span>
         <span className="font-bold shrink-0">23:59</span>
      </div>
    </div>
  );
};

export default StatusBar;