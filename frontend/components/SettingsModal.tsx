import React, { useState, useEffect } from 'react';
import { Type, Keyboard, Eye, Globe, Settings, Zap, Cpu } from 'lucide-react';
import { useTranslation } from '../i18n';

type SettingsTab = 'general' | 'editor' | 'shortcuts' | 'ai';

interface SettingsModalProps {
  onClose: () => void;
}

const SettingsModal: React.FC<SettingsModalProps> = ({ onClose }) => {
  const { language, setLanguage, t } = useTranslation();
  const [activeTab, setActiveTab] = useState<SettingsTab>('general');
  const [fontSize, setFontSize] = useState('14');
  const [enableLigatures, setEnableLigatures] = useState(false);
  const [enableVim, setEnableVim] = useState(false);

  // Load settings from localStorage on mount
  useEffect(() => {
    const savedFontSize = localStorage.getItem('settings.fontSize');
    const savedLigatures = localStorage.getItem('settings.enableLigatures');
    const savedVim = localStorage.getItem('settings.enableVim');

    if (savedFontSize) setFontSize(savedFontSize);
    if (savedLigatures) setEnableLigatures(savedLigatures === 'true');
    if (savedVim) setEnableVim(savedVim === 'true');
  }, []);

  // Apply font size to document root
  useEffect(() => {
    document.documentElement.style.fontSize = `${fontSize}px`;
    localStorage.setItem('settings.fontSize', fontSize);
  }, [fontSize]);

  // Save other settings
  useEffect(() => {
    localStorage.setItem('settings.enableLigatures', enableLigatures.toString());
  }, [enableLigatures]);

  useEffect(() => {
    localStorage.setItem('settings.enableVim', enableVim.toString());
  }, [enableVim]);

  const tabs = [
    { id: 'general' as SettingsTab, label: t('modal.settings.general'), icon: Settings },
    { id: 'editor' as SettingsTab, label: t('modal.settings.editor'), icon: Type },
    { id: 'shortcuts' as SettingsTab, label: t('modal.settings.shortcuts'), icon: Keyboard },
    { id: 'ai' as SettingsTab, label: t('modal.settings.ai'), icon: Zap },
  ];

  return (
    <div className="flex flex-col gap-1">
      <div className="grid grid-cols-[120px_1fr] gap-8 h-[300px]">
        {/* Sidebar */}
        <div className="border-r border-editor-line py-2 flex flex-col gap-1">
          {tabs.map((tab) => {
            const Icon = tab.icon;
            return (
              <div
                key={tab.id}
                onClick={() => setActiveTab(tab.id)}
                className={`px-3 py-1.5 text-xs font-medium rounded cursor-pointer flex items-center gap-2 transition-colors ${
                  activeTab === tab.id
                    ? 'bg-editor-line/50 text-white'
                    : 'text-gray-400 hover:text-white hover:bg-editor-line/30'
                }`}
              >
                <Icon size={14} />
                <span>{tab.label}</span>
              </div>
            );
          })}
        </div>

        {/* Content */}
        <div className="py-2 pr-2 overflow-y-auto">
          {activeTab === 'general' && (
            <>
              <h3 className="text-xs font-bold text-gray-400 uppercase mb-4 border-b border-editor-line pb-1">
                {t('modal.settings.appearance')}
              </h3>

              <div className="flex items-center justify-between mb-4">
                <div className="flex items-center gap-2">
                  <Globe size={14} className="text-editor-accent" />
                  <div className="flex flex-col">
                    <span className="text-sm text-editor-fg">{t('modal.settings.language')}</span>
                  </div>
                </div>
                <select
                  value={language}
                  onChange={(e) => setLanguage(e.target.value as any)}
                  className="bg-editor-line border border-editor-line rounded px-2 py-1 text-xs text-white focus:outline-none min-w-[100px]"
                >
                  <option value="zh">中文</option>
                  <option value="en">English</option>
                </select>
              </div>

              <div className="flex items-center justify-between mb-4">
                <div className="flex items-center gap-2">
                  <Type size={14} className="text-editor-accent" />
                  <div className="flex flex-col">
                    <span className="text-sm text-editor-fg">{t('modal.settings.font_size')}</span>
                    <span className="text-[10px] text-gray-500">
                      {t('modal.settings.font_size_desc')}
                    </span>
                  </div>
                </div>
                <select
                  value={fontSize}
                  onChange={(e) => setFontSize(e.target.value)}
                  className="bg-editor-line border border-editor-line rounded px-2 py-1 text-xs text-white focus:outline-none min-w-[100px]"
                >
                  <option value="12">12px</option>
                  <option value="14">14px</option>
                  <option value="16">16px</option>
                  <option value="18">18px</option>
                </select>
              </div>
            </>
          )}

          {activeTab === 'editor' && (
            <>
              <h3 className="text-xs font-bold text-gray-400 uppercase mb-4 border-b border-editor-line pb-1">
                {t('modal.settings.editor_settings')}
              </h3>

              <div className="flex items-center justify-between mb-4">
                <div className="flex items-center gap-2">
                  <Eye size={14} className="text-editor-accent" />
                  <div className="flex flex-col">
                    <span className="text-sm text-editor-fg">{t('modal.settings.ligatures')}</span>
                    <span className="text-[10px] text-gray-500">
                      {t('modal.settings.ligatures_desc')}
                    </span>
                  </div>
                </div>
                <input
                  type="checkbox"
                  checked={enableLigatures}
                  onChange={(e) => setEnableLigatures(e.target.checked)}
                  className="accent-editor-accent"
                />
              </div>

              <div className="text-xs text-gray-500 mt-6 italic">
                More editor settings coming soon...
              </div>
            </>
          )}

          {activeTab === 'shortcuts' && (
            <>
              <h3 className="text-xs font-bold text-gray-400 uppercase mb-4 border-b border-editor-line pb-1">
                {t('modal.settings.shortcuts')}
              </h3>

              <div className="flex items-center justify-between mb-4">
                <div className="flex items-center gap-2">
                  <Keyboard size={14} className="text-editor-accent" />
                  <div className="flex flex-col">
                    <span className="text-sm text-editor-fg">{t('modal.settings.vim')}</span>
                    <span className="text-[10px] text-gray-500">
                      {t('modal.settings.vim_desc')}
                    </span>
                  </div>
                </div>
                <input
                  type="checkbox"
                  checked={enableVim}
                  onChange={(e) => setEnableVim(e.target.checked)}
                  className="accent-editor-accent"
                />
              </div>

              <div className="text-xs text-gray-500 mt-6 italic">
                More keyboard shortcuts coming soon...
              </div>
            </>
          )}

          {activeTab === 'ai' && (
            <>
              <h3 className="text-xs font-bold text-gray-400 uppercase mb-4 border-b border-editor-line pb-1">
                {t('modal.settings.ai')}
              </h3>

              <div className="flex flex-col gap-4">
                <div className="flex items-center gap-2 text-editor-fg">
                  <Cpu size={14} className="text-editor-accent" />
                  <span className="text-sm">AI-Assisted Code Review</span>
                </div>
                <div className="text-xs text-gray-500">
                  Configure AI settings for automated code review and suggestions.
                </div>

                <div className="text-xs text-gray-500 mt-6 italic">AI features coming soon...</div>
              </div>
            </>
          )}
        </div>
      </div>

      <div className="flex justify-end pt-3 border-t border-editor-line">
        <button
          onClick={onClose}
          className="px-4 py-1.5 rounded text-xs bg-editor-accent text-white hover:bg-blue-600 transition-colors"
        >
          {t('modal.settings.done')}
        </button>
      </div>
    </div>
  );
};

export default SettingsModal;
