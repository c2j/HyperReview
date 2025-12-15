import React, { useState, useEffect, useRef } from 'react';
import { Search, FileCode, Hash, Command, ArrowRight } from 'lucide-react';
import { useTranslation } from '../i18n';
import { useApiClient } from '../api/client';
import type { SearchResult } from '../api/types';

interface CommandPaletteProps {
  onClose: () => void;
  onNavigate: (target: string) => void;
}

const CommandPalette: React.FC<CommandPaletteProps> = ({ onNavigate }) => {
  const { t } = useTranslation();
  const { getCommands } = useApiClient();
  const [query, setQuery] = useState('');
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [commands, setCommands] = useState<SearchResult[]>([]);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    inputRef.current?.focus();
    getCommands().then(setCommands).catch(console.error);
  }, []);

  const filtered = commands.filter(c => c.content.toLowerCase().includes(query.toLowerCase()));

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'ArrowDown') {
      setSelectedIndex(prev => (prev + 1) % filtered.length);
    } else if (e.key === 'ArrowUp') {
      setSelectedIndex(prev => (prev - 1 + filtered.length) % filtered.length);
    } else if (e.key === 'Enter') {
      if (filtered[selectedIndex]) {
        onNavigate(filtered[selectedIndex].content);
      }
    }
  };

  const getIcon = (type: string) => {
      switch(type) {
          case 'File': return FileCode;
          case 'Symbol': return Hash;
          case 'Command': return Command;
          default: return FileCode;
      }
  };

  return (
    <div className="flex flex-col h-[400px]">
      <div className="flex items-center px-4 py-3 border-b border-editor-line">
        <Search className="text-editor-accent mr-3" size={18} />
        <input 
          ref={inputRef}
          className="bg-transparent border-none outline-none text-white w-full placeholder-gray-500 font-mono text-sm"
          placeholder={t('command.placeholder')}
          value={query}
          onChange={e => { setQuery(e.target.value); setSelectedIndex(0); }}
          onKeyDown={handleKeyDown}
        />
        <span className="text-[10px] text-gray-500 border border-editor-line px-1.5 rounded">{t('command.esc')}</span>
      </div>
      <div className="flex-1 overflow-y-auto py-2">
        {filtered.map((item, idx) => {
          const Icon = getIcon(item.result_type);
          return (
          <div
            key={idx}
            className={`px-4 py-2 flex items-center gap-3 cursor-pointer ${idx === selectedIndex ? 'bg-editor-selection' : 'hover:bg-editor-line'}`}
            onClick={() => onNavigate(item.content)}
            onMouseEnter={() => setSelectedIndex(idx)}
          >
            <Icon size={14} className={idx === selectedIndex ? 'text-white' : 'text-gray-500'} />
            <div className="flex-1 min-w-0">
              <div className={`text-sm font-mono truncate ${idx === selectedIndex ? 'text-white' : 'text-editor-fg'}`}>
                {item.content}
              </div>
              <div className={`text-[10px] truncate ${idx === selectedIndex ? 'text-gray-300' : 'text-gray-500'}`}>
                {item.highlight || ''}
              </div>
            </div>
            {idx === selectedIndex && <ArrowRight size={12} className="text-white" />}
          </div>
          );
        })}
        {filtered.length === 0 && (
          <div className="text-center py-8 text-gray-500 text-xs">No results found</div>
        )}
      </div>
      <div className="bg-editor-line/30 px-4 py-1 border-t border-editor-line text-[10px] text-gray-500 flex justify-between">
        <span>{t('command.tips')}</span>
        <span>HyperReview Search Service</span>
      </div>
    </div>
  );
};

export default CommandPalette;