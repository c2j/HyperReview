import React, { useState } from 'react';
import { Upload, Link, GitPullRequest, FileCode } from 'lucide-react';
import { useTranslation } from '../i18n';

interface ImportTaskModalProps {
  onClose: () => void;
  onImport: (id: string) => void;
}

const ImportTaskModal: React.FC<ImportTaskModalProps> = ({ onClose, onImport }) => {
  const { t } = useTranslation();
  const [inputValue, setInputValue] = useState('');
  const [isDragging, setIsDragging] = useState(false);

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    setIsDragging(true);
  };

  const handleDragLeave = () => {
    setIsDragging(false);
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    setIsDragging(false);
    // Simulation of drop
    setInputValue("patch-from-file-v2.diff");
  };

  return (
    <div className="flex flex-col gap-4">
      
      <div>
        <label className="text-xs text-gray-400 mb-2 block font-medium">{t('modal.import_task.label')}</label>
        <div className="relative">
            <GitPullRequest className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500" size={16} />
            <input 
            type="text" 
            value={inputValue}
            onChange={(e) => setInputValue(e.target.value)}
            placeholder="e.g. PR#2899, GH-123, or https://gitlab.com/..." 
            className="w-full bg-editor-line/50 border border-editor-line rounded pl-9 pr-3 py-2 text-sm text-white placeholder-gray-500 focus:outline-none focus:border-editor-accent transition-colors"
            autoFocus
            />
        </div>
      </div>

      <div className="flex items-center gap-2 my-1">
        <div className="h-[1px] bg-editor-line flex-1"></div>
        <span className="text-[10px] text-gray-500 uppercase font-bold">{t('modal.import_task.or')}</span>
        <div className="h-[1px] bg-editor-line flex-1"></div>
      </div>

      <div 
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
        onDrop={handleDrop}
        className={`border-2 border-dashed rounded-lg p-6 flex flex-col items-center justify-center gap-3 transition-all cursor-pointer group
            ${isDragging ? 'border-editor-accent bg-editor-accent/10' : 'border-editor-line hover:border-editor-accent/50 hover:bg-editor-line/30'}`}
      >
        <div className={`p-3 rounded-full ${isDragging ? 'bg-editor-accent text-white' : 'bg-editor-line text-gray-400 group-hover:text-editor-accent'}`}>
             <Upload size={20} />
        </div>
        <div className="text-center">
            <span className="text-xs text-editor-fg block font-medium">{t('modal.import_task.drag')}</span>
            <span className="text-[10px] text-gray-500 block mt-1">{t('modal.import_task.supports')}</span>
        </div>
      </div>

      <div className="flex justify-end gap-2 pt-3 border-t border-editor-line mt-1">
        <button onClick={onClose} className="px-4 py-1.5 rounded text-xs hover:bg-editor-line text-gray-300 transition-colors">{t('modal.import_task.cancel')}</button>
        <button 
          onClick={() => inputValue && onImport(inputValue)}
          disabled={!inputValue}
          className="px-4 py-1.5 rounded text-xs bg-editor-accent text-white hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed transition-colors font-medium shadow-sm"
        >
          {t('modal.import_task.import')}
        </button>
      </div>
    </div>
  );
};

export default ImportTaskModal;