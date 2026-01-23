
import React from 'react';
import { Search, Settings, Circle, HelpCircle, Monitor, Globe } from 'lucide-react';
import { useTranslation } from '../i18n';

interface TitleBarProps {
  onAction: (msg: string) => void;
  mode: 'local' | 'remote';
  onModeChange: (mode: 'local' | 'remote') => void;
}

const TitleBar: React.FC<TitleBarProps> = ({ onAction, mode, onModeChange }) => {
  const { t } = useTranslation();

  return (
    <div className="h-[48px] bg-editor-bg border-b border-editor-line flex items-center justify-between px-4 select-none shrink-0 whitespace-nowrap overflow-hidden">
      {/* Left Section: App Logo & Mode Selector */}
      <div className="flex items-center gap-4 flex-1 min-w-0 mr-4">
        <div className="flex items-center gap-2">
            <span className="font-black text-lg text-white tracking-tighter shrink-0">{t('app.title')}</span>
            <div className={`px-2 py-0.5 rounded text-[10px] font-black tracking-widest ${mode === 'remote' ? 'bg-purple-600 text-white' : 'bg-editor-accent text-white'}`}>
                {mode.toUpperCase()}
            </div>
        </div>

        <div className="h-4 w-[1px] bg-editor-line shrink-0 hidden sm:block"></div>

        {/* Quick Mode Switcher */}
        <div className="flex bg-editor-line/30 p-0.5 rounded-md shrink-0">
            <button
                onClick={() => onModeChange('local')}
                className={`flex items-center gap-1.5 px-3 py-1 rounded text-[10px] font-bold transition-all ${mode === 'local' ? 'bg-editor-accent text-white shadow-sm' : 'text-gray-500 hover:text-gray-300'}`}
                title="Switch to Local Git Mode"
            >
                <Monitor size={12} />
            </button>
            <button
                onClick={() => onModeChange('remote')}
                className={`flex items-center gap-1.5 px-3 py-1 rounded text-[10px] font-bold transition-all ${mode === 'remote' ? 'bg-purple-600 text-white shadow-sm' : 'text-gray-500 hover:text-gray-300'}`}
                title="Switch to Gerrit Remote Mode"
            >
                <Globe size={12} />
            </button>
        </div>

        <div
             className="flex items-center gap-2 text-xs text-gray-500 bg-editor-line/30 px-3 py-1.5 rounded w-full max-w-[200px] hover:bg-editor-line/50 cursor-text transition-colors shrink min-w-[120px]"
             onClick={() => onAction("Global Search Activated")}>
          <Search size={14} className="shrink-0" />
          <span className="opacity-60 truncate">{t('titlebar.search')}</span>
        </div>
      </div>

      {/* Center Section: Main Context Display */}
      <div className={`text-lg font-black tracking-[0.2em] uppercase shrink-0 hidden md:flex items-center gap-3 px-8 py-1 rounded-full border transition-all duration-500 ${mode === 'remote' ? 'text-purple-400 border-purple-500/30 bg-purple-900/10' : 'text-editor-accent border-editor-accent/30 bg-editor-accent/5'}`}>
        <span>{t('titlebar.mode')}</span>
      </div>

      {/* Right Section: System Actions */}
      <div className="flex text-xs items-center gap-4 flex-1 justify-end min-w-0 ml-4">
        <div className="flex items-center gap-2 bg-editor-line/20 px-3 py-1.5 rounded border border-editor-line/50 cursor-help shrink min-w-0 max-w-[200px]"
             onClick={() => onAction("Semantic Context: Active")}>
          <div className={`w-2 h-2 rounded-full animate-pulse shrink-0 ${mode === 'remote' ? 'bg-purple-500' : 'bg-editor-info'}`}></div>
          <span className="truncate text-gray-400">{t('titlebar.semantic')}: <span className="text-gray-200">Order Timeout</span></span>
        </div>

        <div className="flex items-center gap-3 shrink-0">
            <div title={t('titlebar.help')} onClick={() => onAction("Start Tour")} className="cursor-pointer text-gray-500 hover:text-white transition-colors">
                <HelpCircle size={16} />
            </div>
            <div title={t('titlebar.settings')} onClick={() => onAction("Settings Opened")} className="cursor-pointer text-gray-500 hover:text-white transition-colors">
                <Settings size={16} />
            </div>
            <div title={t('titlebar.sync_online')}>
                <Circle size={10} className={`${mode === 'remote' ? 'text-purple-500 fill-purple-500' : 'text-editor-success fill-editor-success'}`} />
            </div>
        </div>
      </div>
    </div>
  );
};

export default TitleBar;
