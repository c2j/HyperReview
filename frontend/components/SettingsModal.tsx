import React from 'react';
import { Type, Keyboard, Eye, Globe } from 'lucide-react';
import { useTranslation } from '../i18n';

interface SettingsModalProps {
  onClose: () => void;
}

const SettingsModal: React.FC<SettingsModalProps> = ({ onClose }) => {
  const { language, setLanguage, t } = useTranslation();

  return (
    <div className="flex flex-col gap-1">
      <div className="grid grid-cols-[120px_1fr] gap-8 h-[300px]">
        {/* Sidebar */}
        <div className="border-r border-editor-line py-2 flex flex-col gap-1">
          <div className="px-3 py-1.5 bg-editor-line/50 text-white text-xs font-medium rounded cursor-pointer">{t('modal.settings.general')}</div>
          <div className="px-3 py-1.5 text-gray-400 hover:text-white hover:bg-editor-line/30 text-xs rounded cursor-pointer">{t('modal.settings.editor')}</div>
          <div className="px-3 py-1.5 text-gray-400 hover:text-white hover:bg-editor-line/30 text-xs rounded cursor-pointer">{t('modal.settings.shortcuts')}</div>
          <div className="px-3 py-1.5 text-gray-400 hover:text-white hover:bg-editor-line/30 text-xs rounded cursor-pointer">{t('modal.settings.ai')}</div>
        </div>

        {/* Content */}
        <div className="py-2 pr-2 overflow-y-auto">
          <h3 className="text-xs font-bold text-gray-400 uppercase mb-4 border-b border-editor-line pb-1">{t('modal.settings.appearance')}</h3>
          
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
                <span className="text-[10px] text-gray-500">{t('modal.settings.font_size_desc')}</span>
              </div>
            </div>
            <select className="bg-editor-line border border-editor-line rounded px-2 py-1 text-xs text-white focus:outline-none">
              <option>12px</option>
              <option selected>14px</option>
              <option>16px</option>
            </select>
          </div>

          <div className="flex items-center justify-between mb-4">
            <div className="flex items-center gap-2">
              <Eye size={14} className="text-editor-accent" />
              <div className="flex flex-col">
                <span className="text-sm text-editor-fg">{t('modal.settings.ligatures')}</span>
                <span className="text-[10px] text-gray-500">{t('modal.settings.ligatures_desc')}</span>
              </div>
            </div>
             <input type="checkbox" checked className="accent-editor-accent" />
          </div>

           <h3 className="text-xs font-bold text-gray-400 uppercase mb-4 border-b border-editor-line pb-1 mt-6">{t('modal.settings.behavior')}</h3>

           <div className="flex items-center justify-between mb-4">
            <div className="flex items-center gap-2">
              <Keyboard size={14} className="text-editor-accent" />
              <div className="flex flex-col">
                <span className="text-sm text-editor-fg">{t('modal.settings.vim')}</span>
                <span className="text-[10px] text-gray-500">{t('modal.settings.vim_desc')}</span>
              </div>
            </div>
             <input type="checkbox" className="accent-editor-accent" />
          </div>

        </div>
      </div>
      
      <div className="flex justify-end pt-3 border-t border-editor-line">
         <button onClick={onClose} className="px-4 py-1.5 rounded text-xs bg-editor-accent text-white hover:bg-blue-600 transition-colors">{t('modal.settings.done')}</button>
      </div>
    </div>
  );
};

export default SettingsModal;