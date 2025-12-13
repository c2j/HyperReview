import React, { useState } from 'react';
import { FileText, Database, Shield, Plus } from 'lucide-react';
import { useTranslation } from '../i18n';

interface CreateTaskModalProps {
  onClose: () => void;
  onCreate: (task: { title: string; type: string }) => void;
}

const CreateTaskModal: React.FC<CreateTaskModalProps> = ({ onClose, onCreate }) => {
  const { t } = useTranslation();
  const [title, setTitle] = useState('');
  const [type, setType] = useState('code');

  return (
    <div className="flex flex-col gap-4">
      <div>
        <label className="text-xs text-gray-400 mb-1 block font-medium">{t('modal.create_task.label_title')}</label>
        <input 
          type="text" 
          value={title}
          onChange={e => setTitle(e.target.value)}
          placeholder="e.g. Q3 Performance Review"
          className="w-full bg-editor-line/50 border border-editor-line rounded px-3 py-2 text-sm text-white focus:outline-none focus:border-editor-accent transition-colors placeholder-gray-600"
          autoFocus
        />
      </div>

      <div>
        <label className="text-xs text-gray-400 mb-2 block font-medium">{t('modal.create_task.label_type')}</label>
        <div className="grid grid-cols-3 gap-3">
            <div 
                onClick={() => setType('code')}
                className={`cursor-pointer p-3 rounded border flex flex-col items-center gap-2 transition-all ${type === 'code' ? 'bg-editor-accent/20 border-editor-accent text-white shadow-[0_0_10px_rgba(0,122,204,0.2)]' : 'bg-editor-line/30 border-transparent text-gray-500 hover:bg-editor-line/50 hover:text-gray-300'}`}>
                <FileText size={20} />
                <span className="text-xs font-medium">{t('modal.create_task.type_code')}</span>
            </div>
            <div 
                onClick={() => setType('sql')}
                className={`cursor-pointer p-3 rounded border flex flex-col items-center gap-2 transition-all ${type === 'sql' ? 'bg-editor-accent/20 border-editor-accent text-white shadow-[0_0_10px_rgba(0,122,204,0.2)]' : 'bg-editor-line/30 border-transparent text-gray-500 hover:bg-editor-line/50 hover:text-gray-300'}`}>
                <Database size={20} />
                <span className="text-xs font-medium">{t('modal.create_task.type_sql')}</span>
            </div>
            <div 
                onClick={() => setType('security')}
                className={`cursor-pointer p-3 rounded border flex flex-col items-center gap-2 transition-all ${type === 'security' ? 'bg-editor-accent/20 border-editor-accent text-white shadow-[0_0_10px_rgba(0,122,204,0.2)]' : 'bg-editor-line/30 border-transparent text-gray-500 hover:bg-editor-line/50 hover:text-gray-300'}`}>
                <Shield size={20} />
                <span className="text-xs font-medium">{t('modal.create_task.type_security')}</span>
            </div>
        </div>
      </div>
      
      <div className="flex justify-end gap-2 pt-3 border-t border-editor-line mt-2">
        <button onClick={onClose} className="px-4 py-1.5 rounded text-xs hover:bg-editor-line text-gray-300 transition-colors">{t('modal.create_task.cancel')}</button>
        <button 
            onClick={() => {
                if (title) onCreate({ title, type });
            }}
            disabled={!title}
            className="px-4 py-1.5 rounded text-xs bg-editor-accent text-white hover:bg-blue-600 font-medium transition-colors flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed">
            <Plus size={14} /> {t('modal.create_task.create')}
        </button>
      </div>
    </div>
  );
};

export default CreateTaskModal;