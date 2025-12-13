import React from 'react';
import { Search, RefreshCw, Settings, Circle, HelpCircle } from 'lucide-react';
import { useTranslation } from '../i18n';

interface TitleBarProps {
  onAction: (msg: string) => void;
}

const TitleBar: React.FC<TitleBarProps> = ({ onAction }) => {
  const { t } = useTranslation();
  return (
    <div className="h-[46px] bg-editor-bg border-b border-editor-line flex items-center justify-between px-4 select-none shrink-0 whitespace-nowrap overflow-hidden">
      {/* Left Section */}
      <div className="flex items-center gap-4 flex-1 min-w-0 mr-4">
        <span className="font-bold text-sm text-editor-fg tracking-wide shrink-0 hidden sm:block">{t('app.title')}</span>
        <span className="font-bold text-sm text-editor-fg tracking-wide shrink-0 sm:hidden">HR</span>
        
        <div className="h-4 w-[1px] bg-editor-line shrink-0 hidden sm:block"></div>
        
        <div 
             id="tour-global-search"
             className="flex items-center gap-2 text-xs text-gray-500 bg-editor-line/30 px-3 py-1.5 rounded w-full max-w-[300px] hover:bg-editor-line/50 cursor-text transition-colors shrink min-w-[120px]"
             onClick={() => onAction("Global Search Activated")}>
          <Search size={14} className="shrink-0" />
          <span className="opacity-60 truncate">{t('titlebar.search')}</span>
        </div>
      </div>

      {/* Center Section */}
      <div className="text-xs text-editor-accent font-medium tracking-wider uppercase shrink-0 hidden md:block">
        {t('titlebar.mode')}
      </div>

      {/* Right Section */}
      <div className="flex items-center gap-4 flex-1 justify-end min-w-0 ml-4">
        <div className="flex items-center gap-2 bg-editor-line/20 px-3 py-1.5 rounded border border-editor-line/50 cursor-help shrink min-w-0 max-w-[200px]"
             onClick={() => onAction("Semantic Context: Order Timeout")}>
          <div className="w-2 h-2 rounded-full bg-editor-info animate-pulse shrink-0"></div>
          <span className="truncate">{t('titlebar.semantic')}: "Order Timeout"</span>
        </div>
        
        <div className="flex items-center gap-2 text-gray-400 hover:text-white cursor-pointer transition-colors shrink-0 hidden lg:flex"
             onClick={() => onAction("Syncing with Remote...")}>
          <RefreshCw size={14} className="animate-spin-slow shrink-0" />
          <span>{t('titlebar.syncing')}</span>
        </div>

        <div className="flex items-center gap-3 shrink-0">
            <div title={t('titlebar.help')} onClick={() => onAction("Start Tour")} className="cursor-pointer text-gray-400 hover:text-white transition-colors">
                <HelpCircle size={14} />
            </div>
            <div title={t('titlebar.settings')} onClick={() => onAction("Settings Opened")} className="cursor-pointer text-gray-400 hover:text-white transition-colors">
                <Settings size={14} />
            </div>
            <div title={t('titlebar.sync_online')}>
                <Circle size={10} className="text-editor-success fill-editor-success" />
            </div>
        </div>
      </div>
    </div>
  );
};

export default TitleBar;