import React, { useState, useEffect, useRef } from 'react';
import {
  Search,
  FileCode,
  Command,
  ArrowRight,
  Settings,
  Database,
  Shield,
  Navigation,
  ExternalLink,
  File,
  Hash,
  GitCommit,
  Loader2,
} from 'lucide-react';
import { useTranslation } from '../i18n';
import { useApiClient } from '../api/client';
import type { SearchResult } from '../api/types/search';

interface CommandInfo {
  id: string;
  name: string;
  description: string;
  category: string;
}

type SearchType = 'All' | 'Commands' | 'Files' | 'Symbols' | 'Commits';

interface CommandPaletteProps {
  onClose: () => void;
  onNavigate: (target: string, type?: 'file' | 'command') => void;
}

const CommandPalette: React.FC<CommandPaletteProps> = ({ onNavigate }) => {
  const { t } = useTranslation();
  const { getCommands, search } = useApiClient();
  const [query, setQuery] = useState('');
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [commands, setCommands] = useState<CommandInfo[]>([]);
  const [searchResults, setSearchResults] = useState<SearchResult[]>([]);
  const [loading, setLoading] = useState(false);
  const [searchType, setSearchType] = useState<SearchType>('All');
  const inputRef = useRef<HTMLInputElement>(null);
  const searchTimeoutRef = useRef<NodeJS.Timeout | undefined>(undefined);

  // Load commands on mount
  useEffect(() => {
    inputRef.current?.focus();
    getCommands()
      .then((data: any[]) => {
        if (data.length > 0 && 'content' in data[0]) {
          setCommands(
            data.map((item: any) => ({
              id: item.id || item.content,
              name: item.content,
              description: item.highlight || item.description || '',
              category: item.result_type || 'Command',
            })),
          );
        } else {
          setCommands(data as CommandInfo[]);
        }
      })
      .catch((error) => {
        console.error('Failed to load commands:', error);
        setCommands([
          {
            id: 'open_repo',
            name: 'Open Repository',
            description: 'Open a Git repository',
            category: 'Repository',
          },
          {
            id: 'get_branches',
            name: 'List Branches',
            description: 'List all branches in repository',
            category: 'Repository',
          },
          {
            id: 'get_file_diff',
            name: 'View Diff',
            description: 'View file diff between commits',
            category: 'Review',
          },
        ]);
      });
  }, [getCommands]);

  // Debounced search for files
  useEffect(() => {
    if (searchTimeoutRef.current) {
      clearTimeout(searchTimeoutRef.current);
    }

    if (query.trim().length < 2) {
      setSearchResults([]);
      setLoading(false);
      return;
    }

    // Only search if we're looking for files/symbols/commits
    if (
      searchType === 'All' ||
      searchType === 'Files' ||
      searchType === 'Symbols' ||
      searchType === 'Commits'
    ) {
      searchTimeoutRef.current = setTimeout(async () => {
        setLoading(true);
        try {
          const results = await search(query);
          setSearchResults(results);
          setSelectedIndex(results.length > 0 ? 0 : -1);
        } catch (error) {
          console.error('Search error:', error);
          setSearchResults([]);
        } finally {
          setLoading(false);
        }
      }, 300);
    }

    return () => {
      if (searchTimeoutRef.current) {
        clearTimeout(searchTimeoutRef.current);
      }
    };
  }, [query, searchType, search]);

  // Filter commands
  const filteredCommands = commands.filter((c) => {
    const searchText = `${c.name} ${c.description} ${c.category}`.toLowerCase();
    return searchText.includes(query.toLowerCase());
  });

  // Filter search results by type
  const filteredSearchResults =
    searchType === 'All' || searchType === 'Commands'
      ? []
      : searchResults.filter((r) => {
          // Handle plural to singular mapping
          if (searchType === 'Files') return r.result_type === 'File';
          if (searchType === 'Symbols') return r.result_type === 'Symbol';
          if (searchType === 'Commits') return r.result_type === 'Commit';
          return false;
        });

  // Combine results based on search type
  const allResults =
    searchType === 'All' || searchType === 'Commands'
      ? filteredCommands.map((c) => ({
          type: 'command' as const,
          data: c,
        }))
      : filteredSearchResults.map((r) => ({
          type: 'search' as const,
          data: r,
        }));

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'ArrowDown') {
      e.preventDefault();
      setSelectedIndex((prev) => (prev + 1) % Math.max(allResults.length, 1));
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      setSelectedIndex(
        (prev) => (prev - 1 + Math.max(allResults.length, 1)) % Math.max(allResults.length, 1),
      );
    } else if (e.key === 'Enter') {
      e.preventDefault();
      if (allResults[selectedIndex]) {
        const result = allResults[selectedIndex];
        if (result.type === 'command') {
          onNavigate(result.data.name, 'command');
        } else {
          onNavigate(result.data.file_path || result.data.content, 'file');
        }
      }
    } else if (e.key === 'Escape') {
      onNavigate('close');
    }
  };

  const getIcon = (result: { type: 'command' | 'search'; data: CommandInfo | SearchResult }) => {
    if (result.type === 'command') {
      const cmd = result.data as CommandInfo;
      switch (cmd.category) {
        case 'Repository':
          return Database;
        case 'Review':
          return FileCode;
        case 'Analysis':
          return Shield;
        case 'Navigation':
          return Navigation;
        case 'External':
          return ExternalLink;
        case 'Command':
          return Command;
        default:
          return Settings;
      }
    } else {
      const searchResult = result.data as SearchResult;
      switch (searchResult.result_type) {
        case 'File':
          return File;
        case 'Symbol':
          return Hash;
        case 'Commit':
          return GitCommit;
        default:
          return Search;
      }
    }
  };

  return (
    <div className="flex flex-col h-[500px]">
      <div className="flex items-center px-4 py-3 border-b border-editor-line">
        <Search className="text-editor-accent mr-3" size={18} />
        <input
          ref={inputRef}
          className="bg-transparent border-none outline-none text-white w-full placeholder-gray-500 font-mono text-sm"
          placeholder={
            query.length === 0 ? t('command.placeholder') : `Search ${searchType.toLowerCase()}...`
          }
          value={query}
          onChange={(e) => {
            setQuery(e.target.value);
            setSelectedIndex(0);
          }}
          onKeyDown={handleKeyDown}
        />
        {loading && <Loader2 size={16} className="animate-spin text-editor-accent" />}
      </div>

      {/* Search Type Tabs */}
      <div className="flex items-center gap-1 px-4 py-2 border-b border-editor-line bg-editor-line/30">
        {(['All', 'Commands', 'Files', 'Symbols', 'Commits'] as SearchType[]).map((type) => (
          <button
            key={type}
            onClick={() => {
              setSearchType(type);
              setSelectedIndex(0);
            }}
            className={`px-3 py-1 rounded text-xs font-medium transition-colors ${
              searchType === type
                ? 'bg-editor-accent text-white'
                : 'text-gray-400 hover:text-white hover:bg-editor-line/50'
            }`}
          >
            {type}
          </button>
        ))}
      </div>

      {/* Results */}
      <div className="flex-1 overflow-y-auto py-2">
        {allResults.length === 0 && !loading ? (
          <div className="text-center py-8 text-gray-500 text-xs">
            {query.length > 0 ? 'No results found' : 'Type to search...'}
          </div>
        ) : (
          allResults.map((result, idx) => {
            const Icon = getIcon(result);
            const isSelected = idx === selectedIndex;

            if (result.type === 'command') {
              const cmd = result.data as CommandInfo;
              return (
                <div
                  key={`cmd-${cmd.id}`}
                  className={`px-4 py-2 flex items-center gap-3 cursor-pointer ${
                    isSelected ? 'bg-editor-selection' : 'hover:bg-editor-line'
                  }`}
                  onClick={() => onNavigate(cmd.name, 'command')}
                  onMouseEnter={() => setSelectedIndex(idx)}
                >
                  <Icon size={14} className={isSelected ? 'text-white' : 'text-gray-500'} />
                  <div className="flex-1 min-w-0">
                    <div
                      className={`text-sm font-mono truncate ${isSelected ? 'text-white' : 'text-editor-fg'}`}
                    >
                      {cmd.name}
                    </div>
                    <div
                      className={`text-[10px] truncate ${isSelected ? 'text-gray-300' : 'text-gray-500'}`}
                    >
                      {cmd.description}
                    </div>
                  </div>
                  {isSelected && <ArrowRight size={12} className="text-white" />}
                </div>
              );
            } else {
              const searchResult = result.data as SearchResult;
              return (
                <div
                  key={`search-${searchResult.result_type}-${idx}`}
                  className={`px-4 py-2 flex items-center gap-3 cursor-pointer ${
                    isSelected ? 'bg-editor-selection' : 'hover:bg-editor-line'
                  }`}
                  onClick={() => onNavigate(searchResult.file_path || searchResult.content, 'file')}
                  onMouseEnter={() => setSelectedIndex(idx)}
                >
                  <Icon size={14} className={isSelected ? 'text-white' : 'text-gray-500'} />
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2">
                      <span className="text-xs font-medium text-editor-accent uppercase tracking-wide">
                        {searchResult.result_type}
                      </span>
                      {searchResult.file_path && (
                        <span className="text-xs text-editor-fg truncate">
                          {searchResult.file_path}
                          {searchResult.line_number && `:${searchResult.line_number}`}
                        </span>
                      )}
                    </div>
                    <div
                      className={`text-sm text-editor-fg mt-0.5 truncate ${isSelected ? 'text-white' : 'text-editor-fg'}`}
                    >
                      {searchResult.content}
                    </div>
                    {searchResult.highlight && (
                      <div className="text-xs text-editor-accent mt-0.5 font-mono truncate">
                        {searchResult.highlight}
                      </div>
                    )}
                  </div>
                  {isSelected && <ArrowRight size={12} className="text-white" />}
                </div>
              );
            }
          })
        )}
      </div>

      <div className="bg-editor-line/30 px-4 py-1 border-t border-editor-line text-[10px] text-gray-500 flex justify-between">
        <span>
          {allResults.length > 0 &&
            `${allResults.length} result${allResults.length !== 1 ? 's' : ''}`}
        </span>
        <span>HyperSearch</span>
      </div>
    </div>
  );
};

export default CommandPalette;
