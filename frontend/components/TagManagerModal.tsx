import React, { useState, useEffect } from 'react';
import { X, Plus, Loader2 } from 'lucide-react';
import { useTranslation } from '../i18n';
import { useApiClient } from '../api/client';
import type { Tag as TagType } from '../api/types';

interface TagManagerModalProps {
  onClose: () => void;
}

const COLORS = [
    { name: 'red', class: 'bg-editor-error' },
    { name: 'orange', class: 'bg-editor-warning' },
    { name: 'blue', class: 'bg-editor-info' },
    { name: 'green', class: 'bg-editor-success' },
    { name: 'gray', class: 'bg-gray-500' },
];

const TagManagerModal: React.FC<TagManagerModalProps> = ({ onClose }) => {
  const { t } = useTranslation();
  const { getTags } = useApiClient();
  const [tags, setTags] = useState<TagType[]>([]);
  const [loading, setLoading] = useState(true);
  const [newTag, setNewTag] = useState('');
  const [selectedColor, setSelectedColor] = useState(COLORS[0].class);

  useEffect(() => {
    setLoading(true);
    getTags()
        .then(setTags)
        .catch(console.error)
        .finally(() => setLoading(false));
  }, []);

  const removeTag = (id: string) => {
    setTags(tags.filter(t => t.id !== id));
  };

  const addTag = () => {
    if(!newTag) return;
    setTags([...tags, { id: crypto.randomUUID(), label: newTag, color: selectedColor, usage_count: 0, created_at: new Date().toISOString(), updated_at: new Date().toISOString() }]);
    setNewTag('');
  };

  return (
    <div className="flex flex-col gap-4">
        <div className="space-y-2 max-h-[250px] overflow-y-auto pr-1 min-h-[100px]">
            {loading ? (
                <div className="flex flex-col items-center justify-center h-[100px] gap-2 text-gray-500">
                    <Loader2 size={24} className="animate-spin text-editor-accent" />
                    <span className="text-xs">Loading tags...</span>
                </div>
            ) : (
                <>
                {tags.map(tag => (
                    <div key={tag.id} className="flex items-center justify-between bg-editor-line/30 p-2 rounded border border-editor-line/50 group hover:border-editor-line transition-colors">
                        <div className="flex items-center gap-2">
                            <div className={`w-3 h-3 rounded-full ${tag.color} shadow-sm`}></div>
                            <span className="text-sm text-editor-fg font-medium">{tag.label}</span>
                        </div>
                        <button onClick={() => removeTag(tag.id)} className="text-gray-500 hover:text-white opacity-0 group-hover:opacity-100 transition-opacity p-1 hover:bg-editor-line rounded">
                            <X size={14} />
                        </button>
                    </div>
                ))}
                {tags.length === 0 && <div className="text-center text-xs text-gray-500 py-4 italic">{t('modal.tag_manager.no_tags')}</div>}
                </>
            )}
        </div>

        <div className="border-t border-editor-line pt-4">
            <label className="text-[10px] text-gray-500 font-bold uppercase mb-2 block">{t('modal.tag_manager.add')}</label>
            <div className="flex gap-2 mb-2">
                <input 
                    value={newTag}
                    onChange={e => setNewTag(e.target.value)}
                    placeholder={t('modal.tag_manager.placeholder')}
                    className="flex-1 bg-editor-line/50 border border-editor-line rounded px-3 py-1.5 text-sm text-white focus:outline-none focus:border-editor-accent transition-colors placeholder-gray-600"
                    onKeyDown={e => e.key === 'Enter' && addTag()}
                />
                <button 
                    onClick={addTag} 
                    disabled={!newTag}
                    className="bg-editor-line hover:bg-editor-accent hover:text-white text-gray-300 px-3 rounded border border-editor-line transition-colors disabled:opacity-50 disabled:hover:bg-editor-line">
                    <Plus size={16} />
                </button>
            </div>
            <div className="flex gap-2">
                {COLORS.map(c => (
                    <div 
                        key={c.name}
                        onClick={() => setSelectedColor(c.class)}
                        className={`w-5 h-5 rounded-full cursor-pointer ${c.class} ${selectedColor === c.class ? 'ring-2 ring-white ring-offset-1 ring-offset-editor-bg scale-110' : 'opacity-70 hover:opacity-100'} transition-all`}
                    ></div>
                ))}
            </div>
        </div>

        <div className="flex justify-end pt-2">
            <button onClick={onClose} className="px-4 py-1.5 rounded text-xs bg-editor-accent text-white hover:bg-blue-600 font-medium transition-colors">{t('modal.tag_manager.done')}</button>
        </div>
    </div>
  );
};

export default TagManagerModal;