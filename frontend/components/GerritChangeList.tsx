import React, { useState, useEffect } from 'react';
import { GitPullRequest, FileText, ChevronRight, Loader2, AlertCircle, RefreshCw, Check, Download, X, CheckCircle, XCircle } from 'lucide-react';
import { simpleGerritService, SimpleChange } from '../services/gerrit-simple-service';

interface GerritChangeListProps {
  onSelectChange: (change: SimpleChange) => void;
  onClose: () => void;
  refreshKey?: number;
}

const GerritChangeList: React.FC<GerritChangeListProps> = ({ onSelectChange, onClose, refreshKey }) => {
  const [changes, setChanges] = useState<SimpleChange[]>([]);
  const [loading, setLoading] = useState(false);
  const [loadingServer, setLoadingServer] = useState(false);
  const [importing, setImporting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [showServerChanges, setShowServerChanges] = useState(false);
  const [serverChanges, setServerChanges] = useState<SimpleChange[]>([]);
  const [selectedChanges, setSelectedChanges] = useState<Set<string>>(new Set());
  const [searchQuery, setSearchQuery] = useState('');
  const [importMessage, setImportMessage] = useState<{ type: 'success' | 'error', text: string } | null>(null);

  const loadChanges = async () => {
    setLoading(true);
    setError(null);

    try {
      const importedChanges = simpleGerritService.getImportedChanges();
      console.log('GerritChangeList: Loaded imported changes:', importedChanges.length);
      setChanges(importedChanges);
    } catch (err) {
      console.error('GerritChangeList: Failed to load changes:', err);
      setError('Failed to load changes: ' + (err as Error).message);
    } finally {
      setLoading(false);
    }
  };

  const loadServerChanges = async (query: string = '') => {
    setLoadingServer(true);
    setError(null);

    try {
      console.log('GerritChangeList: Loading server changes with query:', query);
      const results = await simpleGerritService.searchChanges(query);
      console.log('GerritChangeList: Loaded server changes:', results.length);
      setServerChanges(results);
      setShowServerChanges(true);
    } catch (err) {
      console.error('GerritChangeList: Failed to load server changes:', err);
      setError('Failed to load server changes: ' + (err as Error).message);
    } finally {
      setLoadingServer(false);
    }
  };

  const handleRefresh = async () => {
    await loadChanges();
  };

  const handleRefreshServer = async () => {
    await loadServerChanges(searchQuery);
  };

  const handleSearch = async () => {
    console.log('GerritChangeList: Search button clicked with query:', searchQuery);
    await loadServerChanges(searchQuery);
  };

  const toggleChangeSelection = (changeId: string) => {
    const newSelected = new Set(selectedChanges);
    if (newSelected.has(changeId)) {
      newSelected.delete(changeId);
    } else {
      newSelected.add(changeId);
    }
    setSelectedChanges(newSelected);
  };

  const handleImportSelected = async () => {
    if (selectedChanges.size === 0) return;

    setImporting(true);
    setImportMessage(null);

    const selectedServerChanges = serverChanges.filter(c => selectedChanges.has(c.id));
    let importedCount = 0;
    let failedCount = 0;

    for (const change of selectedServerChanges) {
      try {
        await simpleGerritService.importChange(`#${change.change_number}`);
        importedCount++;
      } catch (err) {
        console.error('Failed to import change:', change.id, err);
        failedCount++;
      }
    }

    setSelectedChanges(new Set());

    if (failedCount === 0 && importedCount > 0) {
      setImportMessage({
        type: 'success',
        text: `Successfully imported ${importedCount} change${importedCount !== 1 ? 's' : ''}. View them below.`
      });

      await loadChanges();

      setTimeout(() => {
        setShowServerChanges(false);
        setServerChanges([]);
        setImportMessage(null);
      }, 2000);
    } else if (failedCount > 0) {
      setImportMessage({
        type: 'error',
        text: `Failed to import ${failedCount} change${failedCount !== 1 ? 's' : ''}`
      });
    }

    setTimeout(() => setImportMessage(null), 5000);
    setImporting(false);
  };

  const filteredServerChanges = serverChanges.filter(change =>
    change.subject.toLowerCase().includes(searchQuery.toLowerCase()) ||
    change.project.toLowerCase().includes(searchQuery.toLowerCase()) ||
    change.owner.toLowerCase().includes(searchQuery.toLowerCase()) ||
    change.change_number.toString().includes(searchQuery)
  );

  const getStatusColor = (status: string) => {
    switch (status.toUpperCase()) {
      case 'NEW':
        return 'bg-blue-500/20 text-blue-400';
      case 'MERGED':
        return 'bg-green-500/20 text-green-400';
      case 'ABANDONED':
        return 'bg-gray-500/20 text-gray-400';
      default:
        return 'bg-yellow-500/20 text-yellow-400';
    }
  };

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffHours = Math.floor(diffMs / (1000 * 60 * 60));

    if (diffHours < 1) {
      return 'Just now';
    } else if (diffHours < 24) {
      return `${diffHours}h ago`;
    } else if (diffHours < 168) {
      return `${Math.floor(diffHours / 24)}d ago`;
    } else {
      return date.toLocaleDateString();
    }
  };

  const handleSelectChange = (change: SimpleChange) => {
    onSelectChange(change);
    onClose();
  };

  const handleLoadServerChanges = async () => {
    await loadServerChanges();
  };

  useEffect(() => {
    loadChanges();
  }, [refreshKey]);

  if (error) {
    return (
      <div className="flex flex-col items-center justify-center h-[500px]">
        <AlertCircle size={48} className="text-red-400 mb-3" />
        <div className="text-sm text-red-400 mb-1">Error loading changes</div>
        <div className="text-xs text-gray-500">{error}</div>
      </div>
    );
  }

  if (loading) {
    return (
      <div className="flex flex-col items-center justify-center h-[500px]">
        <Loader2 size={48} className="animate-spin text-editor-accent" />
        <div className="text-sm text-gray-400 mt-3">Loading changes...</div>
      </div>
    );
  }

  if (showServerChanges) {
    return (
      <div className="flex flex-col h-full">
        <div className="px-4 py-2 bg-editor-accent/10 border-b border-editor-accent/20 text-xs text-gray-400">
          Select changes to import. After importing, they'll appear in the "Imported Changes" view.
        </div>
        <div className="flex items-center justify-between px-4 py-3 border-b border-editor-line bg-editor-line/20">
          <div className="flex items-center gap-3">
            <button
              onClick={() => {
                setShowServerChanges(false);
                setSelectedChanges(new Set());
                setServerChanges([]);
              }}
              className="p-1 rounded hover:bg-editor-line text-gray-500 hover:text-white transition-colors"
              title="Back to imported changes"
            >
              <X size={16} />
            </button>
            <div>
              <h3 className="text-sm font-medium text-editor-fg">Server Changes</h3>
              <p className="text-[10px] text-gray-500">
                {filteredServerChanges.length} change{filteredServerChanges.length !== 1 ? 's' : ''} available
              </p>
            </div>
          </div>
          <div className="flex items-center gap-2">
            <button
              onClick={handleRefreshServer}
              disabled={loadingServer}
              className="p-1.5 rounded hover:bg-editor-line text-gray-500 hover:text-white transition-colors disabled:opacity-30 disabled:cursor-not-allowed"
              title="Refresh server changes"
            >
              {loadingServer ? (
                <Loader2 size={16} className="animate-spin" />
              ) : (
                <RefreshCw size={16} />
              )}
            </button>
          </div>
        </div>

        <div className="px-4 py-3 border-b border-editor-line">
          <div className="flex gap-2">
            <input
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === 'Enter') {
                  handleSearch();
                }
              }}
              placeholder="Search changes by subject, project, owner, or number..."
              className="flex-1 bg-editor-line/50 border border-editor-line rounded px-3 py-2 text-sm text-white placeholder-gray-500 focus:outline-none focus:border-editor-accent transition-colors"
            />
            <button
              onClick={handleSearch}
              disabled={loadingServer}
              className="px-4 py-2 bg-editor-accent text-white rounded text-sm hover:bg-blue-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {loadingServer ? 'Searching...' : 'Search'}
            </button>
          </div>
        </div>

        {selectedChanges.size > 0 && (
          <div className="px-4 py-2 bg-editor-accent/20 border-b border-editor-accent/30 flex items-center justify-between">
            <span className="text-xs text-editor-accent">
              {selectedChanges.size} change{selectedChanges.size !== 1 ? 's' : ''} selected
            </span>
            <button
              onClick={handleImportSelected}
              disabled={importing}
              className="flex items-center gap-1 px-3 py-1.5 bg-editor-accent text-white rounded text-xs hover:bg-blue-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {importing ? (
                <>
                  <Loader2 size={14} className="animate-spin" />
                  Importing...
                </>
              ) : (
                <>
                  <Download size={14} />
                  Import Selected
                </>
              )}
            </button>
          </div>
        )}

        {importMessage && (
          <div className={`px-4 py-2 border-b flex items-center justify-between ${
            importMessage.type === 'success'
              ? 'bg-green-500/20 border-green-500/30 text-green-400'
              : 'bg-red-500/20 border-red-500/30 text-red-400'
          }`}>
            <span className="text-xs flex items-center gap-2">
              {importMessage.type === 'success' ? (
                <CheckCircle size={14} />
              ) : (
                <XCircle size={14} />
              )}
              {importMessage.text}
            </span>
            <button
              onClick={() => setImportMessage(null)}
              className="text-xs hover:underline"
            >
              <X size={14} />
            </button>
          </div>
        )}

        <div className="flex-1 overflow-y-auto px-4 py-3">
          {loadingServer ? (
            <div className="flex flex-col items-center justify-center h-64">
              <Loader2 size={32} className="animate-spin text-editor-accent" />
              <div className="text-sm text-gray-400 mt-3">Loading from server...</div>
            </div>
          ) : filteredServerChanges.length === 0 ? (
            <div className="flex flex-col items-center justify-center h-64 text-center">
              <GitPullRequest size={48} className="text-gray-500 mb-3" />
              <div className="text-sm text-gray-400 mb-1">No changes found</div>
              <div className="text-xs text-gray-500">
                Try adjusting your search query or refresh the list
              </div>
            </div>
          ) : (
            <div className="space-y-2">
              {filteredServerChanges.map(change => (
                <div
                  key={change.id}
                  className={`p-3 rounded border transition-all ${
                    selectedChanges.has(change.id)
                      ? 'bg-editor-accent/20 border-editor-accent'
                      : 'border-editor-line/50 hover:bg-editor-line hover:border-editor-accent/50'
                  }`}
                >
                  <div className="flex items-start gap-3">
                    <button
                      onClick={() => toggleChangeSelection(change.id)}
                      className={`mt-0.5 w-5 h-5 rounded border-2 flex items-center justify-center transition-colors ${
                        selectedChanges.has(change.id)
                          ? 'bg-editor-accent border-editor-accent'
                          : 'border-gray-500 hover:border-editor-accent'
                      }`}
                    >
                      {selectedChanges.has(change.id) && <Check size={12} className="text-white" />}
                    </button>
                    <div className="flex-1 min-w-0">
                      <div className="text-xs text-gray-500 mb-1">
                        #{change.change_number}
                      </div>
                      <div className={`text-sm font-medium mb-1 ${getStatusColor(change.status)}`}>
                        {change.subject}
                      </div>
                      <div className="flex items-center gap-2 text-[10px] text-gray-500">
                        <span>{change.project}</span>
                        {' • '}
                        <span>{change.owner}</span>
                      </div>
                    </div>
                    <div className="flex flex-col items-end gap-2">
                      <span className={`px-2 py-1 rounded text-xs font-medium ${getStatusColor(change.status)}`}>
                        {change.status}
                      </span>
                      <span className="text-xs text-gray-500">
                        {formatDate(change.updated)}
                      </span>
                    </div>
                  </div>
                  <div className="flex items-center gap-3 text-[10px] text-gray-500 mt-2 ml-8">
                    <div className="flex items-center gap-2">
                      {change.insertions > 0 && (
                        <span className="text-green-500">+{change.insertions}</span>
                      )}
                      {change.deletions > 0 && (
                        <span className="text-red-500">-{change.deletions}</span>
                      )}
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    );
  }

  if (changes.length === 0) {
    return (
      <div className="flex flex-col h-full">
        <div className="flex items-center justify-between px-4 py-3 border-b border-editor-line bg-editor-line/20">
          <div>
            <h3 className="text-sm font-medium text-editor-fg">Imported Changes</h3>
            <p className="text-[10px] text-gray-500">
              0 changes available
            </p>
          </div>
          <button
            onClick={handleLoadServerChanges}
            disabled={loadingServer}
            className="flex items-center gap-1 px-3 py-1.5 bg-editor-accent text-white rounded text-xs hover:bg-blue-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <RefreshCw size={14} />
            Load from Server
          </button>
        </div>
        <div className="flex flex-col items-center justify-center flex-1">
          <GitPullRequest size={48} className="text-gray-500 mb-3" />
          <div className="text-sm text-gray-400 mb-1">No imported changes yet</div>
          <div className="text-xs text-gray-500 mb-4">
            Load changes from server to get started
          </div>
          <button
            onClick={handleLoadServerChanges}
            disabled={loadingServer}
            className="flex items-center gap-2 px-4 py-2 bg-editor-accent text-white rounded text-sm hover:bg-blue-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {loadingServer ? (
              <>
                <Loader2 size={16} className="animate-spin" />
                Loading...
              </>
            ) : (
              <>
                <RefreshCw size={16} />
                Load from Server
              </>
            )}
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full">
      <div className="flex items-center justify-between px-4 py-3 border-b border-editor-line bg-editor-line/20">
        <div>
          <h3 className="text-sm font-medium text-editor-fg">Imported Changes</h3>
          <p className="text-[10px] text-gray-500">
            {changes.length} change{changes.length !== 1 ? 's' : ''} available
          </p>
        </div>
        <div className="flex items-center gap-2">
          <button
            onClick={handleLoadServerChanges}
            disabled={loadingServer}
            className="flex items-center gap-1 px-3 py-1.5 bg-editor-line text-gray-300 rounded text-xs hover:bg-editor-line/80 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            title="Load changes from server"
          >
            <RefreshCw size={14} />
            Load from Server
          </button>
          <button
            onClick={handleRefresh}
            disabled={loading}
            className="p-1.5 rounded hover:bg-editor-line text-gray-500 hover:text-white transition-colors disabled:opacity-30 disabled:cursor-not-allowed"
            title="Refresh imported changes"
          >
            {loading ? (
              <Loader2 size={16} className="animate-spin" />
            ) : (
              <RefreshCw size={16} />
            )}
          </button>
        </div>
      </div>

      {importMessage && (
        <div className={`px-4 py-2 border-b flex items-center justify-between ${
          importMessage.type === 'success'
            ? 'bg-green-500/20 border-green-500/30 text-green-400'
            : 'bg-red-500/20 border-red-500/30 text-red-400'
        }`}>
          <span className="text-xs flex items-center gap-2">
            {importMessage.type === 'success' ? (
              <CheckCircle size={14} />
            ) : (
              <XCircle size={14} />
            )}
            {importMessage.text}
          </span>
          <button
            onClick={() => setImportMessage(null)}
            className="text-xs hover:underline"
          >
            <X size={14} />
          </button>
        </div>
      )}

      <div className="flex-1 overflow-y-auto px-4 py-3">
        {changes.map(change => (
          <button
            key={change.id}
            onClick={() => handleSelectChange(change)}
            className="text-left p-4 rounded border border-editor-line/50 hover:bg-editor-line hover:border-editor-accent transition-all mb-2 w-full"
          >
            <div className="flex items-start gap-3">
              <div className="flex-1 min-w-0">
                <div className="text-xs text-gray-500 mb-1">
                  #{change.change_number}
                </div>
                <div className={`text-sm font-medium mb-1 ${getStatusColor(change.status)}`}>
                  {change.subject}
                </div>
                <div className="flex items-center gap-2 text-[10px] text-gray-500">
                  <span>{change.project}</span>
                  {' • '}
                  <span>{change.owner}</span>
                </div>
              </div>
              <div className="flex-shrink-0">
                <FileText size={16} className="text-gray-500 flex-shrink-0" />
              </div>
              <div className="flex flex-col items-end gap-3">
                <span className={`px-2 py-1 rounded text-xs font-medium ${getStatusColor(change.status)}`}>
                  {change.status}
                </span>
                <span className="text-xs text-gray-500">
                  {formatDate(change.updated)}
                </span>
              </div>
            </div>
            <div className="flex items-center gap-3 text-[10px] text-gray-500">
              <div className="flex items-center gap-2">
                {change.insertions > 0 && (
                  <span className="text-green-500">+{change.insertions}</span>
                )}
                {change.deletions > 0 && (
                  <span className="text-red-500">-{change.deletions}</span>
                )}
              </div>
              <ChevronRight size={16} className="text-gray-500 flex-shrink-0" />
            </div>
          </button>
        ))}
      </div>
    </div>
  );
};

export default GerritChangeList;
