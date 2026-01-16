import React, { useState } from 'react';
import { Search, GitPullRequest, Loader2 } from 'lucide-react';
import { simpleGerritService, SimpleChange } from '../services/gerrit-simple-service';

interface GerritImportModalProps {
  onClose: () => void;
  onImport: (change: SimpleChange) => void;
}

const GerritImportModal: React.FC<GerritImportModalProps> = ({ onClose, onImport }) => {
  const [importType, setImportType] = useState<'id' | 'search'>('id');
  const [changeId, setChangeId] = useState('');
  const [searchQuery, setSearchQuery] = useState('');
  const [isImporting, setIsImporting] = useState(false);
  const [isSearching, setIsSearching] = useState(false);
  const [searchResults, setSearchResults] = useState<SimpleChange[]>([]);

  const handleImportById = async () => {
    if (!changeId.trim()) return;

    setIsImporting(true);
    try {
      const change = await simpleGerritService.importChange(changeId);
      if (change) {
        onImport(change);
        onClose();
      } else {
        alert('Failed to import change. Please check the change ID and try again.');
      }
    } catch (error) {
      console.error('Failed to import change:', error);
      alert('Failed to import change: ' + (error as Error).message);
    } finally {
      setIsImporting(false);
    }
  };

  const handleSearch = async () => {
    if (!searchQuery.trim()) return;

    setIsSearching(true);
    try {
      const results = await simpleGerritService.searchChanges(searchQuery);
      setSearchResults(results);
    } catch (error) {
      console.error('Failed to search changes:', error);
      alert('Failed to search changes: ' + (error as Error).message);
    } finally {
      setIsSearching(false);
    }
  };

  const handleSelectChange = (change: SimpleChange) => {
    onImport(change);
    onClose();
  };

  return (
    <div className="flex flex-col gap-4">
      <div className="flex gap-2 mb-2">
        <button
          onClick={() => setImportType('id')}
          className={`flex-1 px-3 py-2 rounded text-xs font-medium transition-colors ${
            importType === 'id'
              ? 'bg-editor-accent text-white'
              : 'bg-editor-line text-gray-400 hover:text-white'
          }`}
        >
          <div className="flex items-center gap-2">
            <GitPullRequest size={14} />
            <span>Import by ID</span>
          </div>
        </button>
        <button
          onClick={() => setImportType('search')}
          className={`flex-1 px-3 py-2 rounded text-xs font-medium transition-colors ${
            importType === 'search'
              ? 'bg-editor-accent text-white'
              : 'bg-editor-line text-gray-400 hover:text-white'
          }`}
        >
          <div className="flex items-center gap-2">
            <Search size={14} />
            <span>Search Changes</span>
          </div>
        </button>
      </div>

      {importType === 'id' && (
        <>
          <div>
            <label className="text-xs text-gray-400 mb-2 block font-medium">
              Gerrit Change ID
            </label>
            <div className="relative">
              <GitPullRequest
                className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500"
                size={16}
              />
              <input
                type="text"
                value={changeId}
                onChange={(e) => setChangeId(e.target.value)}
                placeholder="e.g. #12345 or Iabc123..."
                className="w-full bg-editor-line/50 border border-editor-line rounded pl-9 pr-3 py-2 text-sm text-white placeholder-gray-500 focus:outline-none focus:border-editor-accent transition-colors"
                autoFocus
                onKeyDown={(e) => {
                  if (e.key === 'Enter' && !isImporting) {
                    handleImportById();
                  }
                }}
              />
            </div>
            <div className="text-[10px] text-gray-500 mt-1">
              Enter a Gerrit Change ID to import the change with all files and comments
            </div>
          </div>

          <div className="flex justify-end gap-2 pt-3 border-t border-editor-line mt-1">
            <button
              onClick={onClose}
              className="px-4 py-1.5 rounded text-xs hover:bg-editor-line text-gray-300 transition-colors"
            >
              Cancel
            </button>
            <button
              onClick={handleImportById}
              disabled={!changeId.trim() || isImporting}
              className="px-4 py-1.5 rounded text-xs bg-editor-accent text-white hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed transition-colors font-medium shadow-sm flex items-center gap-2"
            >
              {isImporting ? (
                <>
                  <Loader2 size={14} className="animate-spin" />
                  Importing...
                </>
              ) : (
                'Import Change'
              )}
            </button>
          </div>
        </>
      )}

      {importType === 'search' && (
        <>
          <div>
            <label className="text-xs text-gray-400 mb-2 block font-medium">
              Search Gerrit Changes
            </label>
            <div className="relative">
              <Search
                className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500"
                size={16}
              />
              <input
                type="text"
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                placeholder="e.g. status:open project:hyperreview owner:alice"
                className="w-full bg-editor-line/50 border border-editor-line rounded pl-9 pr-3 py-2 text-sm text-white placeholder-gray-500 focus:outline-none focus:border-editor-accent transition-colors"
                autoFocus
                onKeyDown={(e) => {
                  if (e.key === 'Enter' && !isSearching) {
                    handleSearch();
                  }
                }}
              />
            </div>
            <div className="text-[10px] text-gray-500 mt-1">
              Use Gerrit query syntax to search for changes (e.g., status:open, project:name)
            </div>
          </div>

          <button
            onClick={handleSearch}
            disabled={!searchQuery.trim() || isSearching}
            className="px-4 py-1.5 rounded text-xs bg-editor-line text-white hover:bg-editor-line/80 disabled:opacity-50 disabled:cursor-not-allowed transition-colors font-medium shadow-sm flex items-center gap-2 mb-3"
          >
            {isSearching ? (
              <>
                <Loader2 size={14} className="animate-spin" />
                Searching...
              </>
            ) : (
              'Search'
            )}
          </button>

          {searchResults.length > 0 && (
            <div className="max-h-[200px] overflow-y-auto">
              <div className="text-xs text-gray-400 mb-2 font-semibold">
                {searchResults.length} change(s) found
              </div>
              <div className="space-y-2">
                {searchResults.map((change) => (
                  <div
                    key={change.id}
                    onClick={() => handleSelectChange(change)}
                    className="p-3 rounded bg-editor-line/30 hover:bg-editor-line/50 cursor-pointer transition-colors border border-editor-line/50"
                  >
                    <div className="flex items-start justify-between">
                      <div className="flex-1">
                        <div className="text-sm font-medium text-editor-fg mb-1">
                          #{change.change_number}: {change.subject}
                        </div>
                        <div className="flex items-center gap-3 text-[10px] text-gray-500">
                          <span>Project: {change.project}</span>
                          <span>Owner: {change.owner}</span>
                          <span>Branch: {change.branch}</span>
                          <span>
                            {change.insertions > 0 && (
                              <span className="text-green-500">+{change.insertions}</span>
                            )}
                            {change.deletions > 0 && (
                              <span className="text-red-500">-{change.deletions}</span>
                            )}
                          </span>
                        </div>
                      </div>
                      <div className="text-[10px] px-2 py-1 rounded bg-editor-accent/20 text-editor-accent">
                        {change.status}
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}

          {searchResults.length === 0 && searchQuery && !isSearching && (
            <div className="text-center py-4">
              <div className="text-sm text-gray-400">No changes found</div>
              <div className="text-xs text-gray-500 mt-1">
                Try a different search query or check your connection settings
              </div>
            </div>
          )}

          <div className="flex justify-end pt-3 border-t border-editor-line mt-1">
            <button
              onClick={onClose}
              className="px-4 py-1.5 rounded text-xs hover:bg-editor-line text-gray-300 transition-colors"
            >
              Cancel
            </button>
          </div>
        </>
      )}
    </div>
  );
};

export default GerritImportModal;
