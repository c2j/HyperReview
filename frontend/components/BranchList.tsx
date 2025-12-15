/**
 * Branch List Component
 * Displays repository branches with filtering and selection
 */

import React, { useState, useCallback, useMemo } from 'react';
import { GitBranch, GitMerge, RefreshCw, Search, CheckCircle } from 'lucide-react';
import { useBranches } from '../hooks/useRepository';
import { useLoading } from '../context/LoadingContext';
import type { Branch } from '../api/types';

interface BranchListProps {
  onBranchSelect?: (branch: Branch) => void;
  selectedBranch?: string;
  showRemoteBranches?: boolean;
  className?: string;
}

/**
 * Branch list with search and filtering
 */
const BranchList: React.FC<BranchListProps> = ({
  onBranchSelect,
  selectedBranch,
  showRemoteBranches = true,
  className = ''
}) => {
  const { branches, loadBranches } = useBranches();
  const { isRepositoryLoading } = useLoading();
  const [searchQuery, setSearchQuery] = useState('');
  const [showOnlyCurrent, setShowOnlyCurrent] = useState(false);

  // Filter branches based on search and settings
  const filteredBranches = useMemo(() => {
    let filtered = branches;

    // Filter by search query
    if (searchQuery) {
      const query = searchQuery.toLowerCase();
      filtered = filtered.filter(
        (branch) =>
          branch.name.toLowerCase().includes(query) ||
          branch.last_commit_message.toLowerCase().includes(query)
      );
    }

    // Show only current branch
    if (showOnlyCurrent) {
      filtered = filtered.filter((branch) => branch.is_current);
    }

    // Filter remote branches if not showing them
    if (!showRemoteBranches) {
      filtered = filtered.filter((branch) => !branch.is_remote);
    }

    return filtered;
  }, [branches, searchQuery, showOnlyCurrent, showRemoteBranches]);

  const handleRefresh = useCallback(() => {
    loadBranches();
  }, [loadBranches]);

  const handleBranchClick = useCallback(
    (branch: Branch) => {
      if (onBranchSelect) {
        onBranchSelect(branch);
      }
    },
    [onBranchSelect]
  );

  // Group branches by type
  const groupedBranches = useMemo(() => {
    return {
      current: filteredBranches.filter((b) => b.is_current),
      local: filteredBranches.filter((b) => !b.is_remote && !b.is_current),
      remote: showRemoteBranches ? filteredBranches.filter((b) => b.is_remote) : []
    };
  }, [filteredBranches, showRemoteBranches]);

  return (
    <div className={`space-y-4 ${className}`}>
      {/* Header with actions */}
      <div className="flex items-center justify-between">
        <h3 className="text-sm font-semibold text-editor-fg/80 uppercase tracking-wide">
          Branches
          <span className="ml-2 text-xs text-editor-fg/60">
            ({filteredBranches.length})
          </span>
        </h3>
        <button
          onClick={handleRefresh}
          disabled={isRepositoryLoading}
          className="p-2 hover:bg-editor-selection/30 rounded transition-colors disabled:opacity-50"
          title="Refresh branches"
        >
          <RefreshCw className={`w-4 h-4 ${isRepositoryLoading ? 'animate-spin' : ''}`} />
        </button>
      </div>

      {/* Search and filters */}
      <div className="space-y-2">
        {/* Search input */}
        <div className="relative">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-editor-fg/40" />
          <input
            type="text"
            placeholder="Search branches..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full pl-10 pr-4 py-2 bg-editor-bg border border-editor-line rounded text-sm text-editor-fg placeholder-editor-fg/40 focus:outline-none focus:border-editor-accent"
          />
        </div>

        {/* Filter options */}
        <div className="flex items-center gap-2">
          <label className="flex items-center gap-2 text-xs text-editor-fg/70 cursor-pointer">
            <input
              type="checkbox"
              checked={showOnlyCurrent}
              onChange={(e) => setShowOnlyCurrent(e.target.checked)}
              className="rounded border-editor-line bg-editor-bg text-editor-accent focus:ring-editor-accent"
            />
            <span>Current only</span>
          </label>

          {showRemoteBranches && (
            <label className="flex items-center gap-2 text-xs text-editor-fg/70 cursor-pointer">
              <input
                type="checkbox"
                checked={showRemoteBranches}
                onChange={() => {}}
                disabled
                className="rounded border-editor-line bg-editor-bg text-editor-accent"
              />
              <span>Show remote</span>
            </label>
          )}
        </div>
      </div>

      {/* Branch list */}
      <div className="space-y-3 max-h-96 overflow-y-auto">
        {/* Current branch */}
        {groupedBranches.current.length > 0 && (
          <BranchGroup
            title="Current Branch"
            branches={groupedBranches.current}
            onBranchClick={handleBranchClick}
            selectedBranch={selectedBranch}
          />
        )}

        {/* Local branches */}
        {groupedBranches.local.length > 0 && (
          <BranchGroup
            title="Local Branches"
            branches={groupedBranches.local}
            onBranchClick={handleBranchClick}
            selectedBranch={selectedBranch}
          />
        )}

        {/* Remote branches */}
        {groupedBranches.remote.length > 0 && (
          <BranchGroup
            title="Remote Branches"
            branches={groupedBranches.remote}
            onBranchClick={handleBranchClick}
            selectedBranch={selectedBranch}
            isRemote
          />
        )}

        {/* Empty state */}
        {filteredBranches.length === 0 && (
          <div className="text-center py-8 text-editor-fg/60 text-sm border border-editor-line rounded-lg border-dashed">
            <GitBranch className="w-8 h-8 mx-auto mb-2 opacity-50" />
            <div>No branches found</div>
            <div className="text-xs mt-1">
              {searchQuery ? 'Try a different search query' : 'Refresh to load branches'}
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

/**
 * Branch group component
 */
interface BranchGroupProps {
  title: string;
  branches: Branch[];
  onBranchClick: (branch: Branch) => void;
  selectedBranch?: string;
  isRemote?: boolean;
}

const BranchGroup: React.FC<BranchGroupProps> = ({
  title,
  branches,
  onBranchClick,
  selectedBranch,
  isRemote = false
}) => {
  return (
    <div className="space-y-1">
      <h4 className="text-xs font-semibold text-editor-fg/60 uppercase tracking-wide px-2">
        {title}
      </h4>
      <div className="space-y-1">
        {branches.map((branch) => (
          <BranchItem
            key={`${branch.name}-${branch.is_current}`}
            branch={branch}
            onClick={() => onBranchClick(branch)}
            isSelected={selectedBranch === branch.name}
            isRemote={isRemote}
          />
        ))}
      </div>
    </div>
  );
};

/**
 * Individual branch item
 */
interface BranchItemProps {
  branch: Branch;
  onClick: () => void;
  isSelected?: boolean;
  isRemote?: boolean;
}

const BranchItem: React.FC<BranchItemProps> = ({
  branch,
  onClick,
  isSelected = false,
  isRemote = false
}) => {
  return (
    <button
      onClick={onClick}
      className={`
        w-full flex items-center gap-3 px-3 py-2 rounded text-left transition-colors
        ${isSelected ? 'bg-editor-accent/20 border border-editor-accent/40' : 'hover:bg-editor-selection/30 border border-transparent'}
      `}
    >
      {/* Branch icon */}
      <div className="flex items-center gap-2">
        {branch.is_current ? (
          <CheckCircle className="w-4 h-4 text-green-500" />
        ) : (
          <GitBranch className="w-4 h-4 text-editor-fg/60" />
        )}
        {branch.is_remote && <GitMerge className="w-3 h-3 text-editor-fg/40" />}
      </div>

      {/* Branch info */}
      <div className="flex-1 min-w-0">
        <div className="flex items-center gap-2">
          <span className={`font-medium truncate ${isSelected ? 'text-editor-accent' : 'text-editor-fg'}`}>
            {branch.name}
          </span>
          {branch.is_current && (
            <span className="text-xs px-2 py-0.5 bg-green-500/20 text-green-400 rounded">
              Current
            </span>
          )}
          {isRemote && (
            <span className="text-xs px-2 py-0.5 bg-blue-500/20 text-blue-400 rounded">
              Remote
            </span>
          )}
        </div>
        <div className="text-xs text-editor-fg/60 truncate">
          {branch.last_commit_message}
        </div>
      </div>

      {/* Last commit date */}
      <div className="text-xs text-editor-fg/50 flex-shrink-0">
        {new Date(branch.last_commit_date).toLocaleDateString()}
      </div>
    </button>
  );
};

export default BranchList;
