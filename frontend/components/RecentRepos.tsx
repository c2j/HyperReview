/**
 * Recent Repositories Component
 * Displays recently opened repositories with quick access
 */

import React, { useCallback, useMemo } from 'react';
import { FolderOpen, Clock, GitBranch, Loader2, RefreshCw } from 'lucide-react';
import { useRecentRepositories, useRepositoryActions } from '../hooks/useRepository';
import type { Repository } from '../api/types';

interface RecentReposProps {
  onRepositorySelect?: (repository: Repository) => void;
  maxDisplay?: number;
  showLastOpened?: boolean;
  className?: string;
}

/**
 * Recent repositories list with refresh capability
 */
const RecentRepos: React.FC<RecentReposProps> = ({
  onRepositorySelect,
  maxDisplay = 10,
  showLastOpened = true,
  className = ''
}) => {
  const { recentRepos, loading, loadRecentRepos } = useRecentRepositories();
  const { switchRepository } = useRepositoryActions();
  const [selectedRepo, setSelectedRepo] = React.useState<string | null>(null);

  // Limit display count
  const displayRepos = useMemo(() => {
    return recentRepos.slice(0, maxDisplay);
  }, [recentRepos, maxDisplay]);

  const handleRepositoryClick = useCallback(
    async (repo: Repository) => {
      setSelectedRepo(repo.path);
      const repository = await switchRepository(repo.path);
      if (repository && onRepositorySelect) {
        onRepositorySelect(repository);
      }
      setSelectedRepo(null);
    },
    [switchRepository, onRepositorySelect]
  );

  const handleRefresh = useCallback(() => {
    loadRecentRepos();
  }, [loadRecentRepos]);

  return (
    <div className={`space-y-3 ${className}`}>
      {/* Header */}
      <div className="flex items-center justify-between">
        <h3 className="text-sm font-semibold text-editor-fg/80 uppercase tracking-wide">
          Recent Repositories
          <span className="ml-2 text-xs text-editor-fg/60">
            ({recentRepos.length})
          </span>
        </h3>
        <button
          onClick={handleRefresh}
          disabled={loading}
          className="p-2 hover:bg-editor-selection/30 rounded transition-colors disabled:opacity-50"
          title="Refresh repositories"
        >
          <RefreshCw className={`w-4 h-4 ${loading ? 'animate-spin' : ''}`} />
        </button>
      </div>

      {/* Repository list */}
      <div className="space-y-2 max-h-96 overflow-y-auto">
        {loading && recentRepos.length === 0 ? (
          <div className="flex items-center justify-center py-8">
            <Loader2 className="w-6 h-6 animate-spin text-editor-accent" />
          </div>
        ) : displayRepos.length === 0 ? (
          <EmptyState onRefresh={handleRefresh} />
        ) : (
          displayRepos.map((repo) => (
            <RepositoryCard
              key={repo.path}
              repository={repo}
              onClick={() => handleRepositoryClick(repo)}
              isLoading={selectedRepo === repo.path}
              showLastOpened={showLastOpened}
            />
          ))
        )}
      </div>

      {/* Show more indicator */}
      {recentRepos.length > maxDisplay && (
        <div className="text-center text-xs text-editor-fg/60 py-2">
          Showing {maxDisplay} of {recentRepos.length} repositories
        </div>
      )}
    </div>
  );
};

/**
 * Empty state component
 */
interface EmptyStateProps {
  onRefresh?: () => void;
}

const EmptyState: React.FC<EmptyStateProps> = ({ onRefresh }) => {
  return (
    <div className="text-center py-8 text-editor-fg/60 text-sm border border-editor-line rounded-lg border-dashed">
      <FolderOpen className="w-8 h-8 mx-auto mb-2 opacity-50" />
      <div>No recent repositories</div>
      <div className="text-xs mt-1">
        Open a repository to see it here
      </div>
      {onRefresh && (
        <button
          onClick={onRefresh}
          className="mt-3 px-3 py-1 text-xs bg-editor-accent/20 hover:bg-editor-accent/30 text-editor-accent rounded transition-colors"
        >
          Refresh
        </button>
      )}
    </div>
  );
};

/**
 * Repository card component
 */
interface RepositoryCardProps {
  repository: Repository;
  onClick: () => void;
  isLoading?: boolean;
  showLastOpened?: boolean;
}

const RepositoryCard: React.FC<RepositoryCardProps> = ({
  repository,
  onClick,
  isLoading = false,
  showLastOpened = true
}) => {
  const handleClick = useCallback(() => {
    if (!isLoading) {
      onClick();
    }
  }, [isLoading, onClick]);

  // Format last opened date
  const formatLastOpened = useCallback((dateString?: string) => {
    if (!dateString) return '';

    const date = new Date(dateString);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffMins = Math.floor(diffMs / 60000);
    const diffHours = Math.floor(diffMs / 3600000);
    const diffDays = Math.floor(diffMs / 86400000);

    if (diffMins < 1) return 'Just now';
    if (diffMins < 60) return `${diffMins} minute${diffMins > 1 ? 's' : ''} ago`;
    if (diffHours < 24) return `${diffHours} hour${diffHours > 1 ? 's' : ''} ago`;
    if (diffDays < 7) return `${diffDays} day${diffDays > 1 ? 's' : ''} ago`;

    return date.toLocaleDateString();
  }, []);

  return (
    <button
      onClick={handleClick}
      disabled={isLoading}
      className={`
        w-full flex items-center gap-3 p-3 rounded-lg border transition-all text-left
        ${isLoading ? 'opacity-50 cursor-not-allowed' : 'hover:border-editor-accent/40 hover:bg-editor-selection/20 cursor-pointer'}
        ${repository.is_active ? 'border-green-500/30 bg-green-500/5' : 'border-editor-line'}
      `}
    >
      {/* Repository icon */}
      <div className="w-10 h-10 flex items-center justify-center bg-editor-accent/10 rounded border border-editor-accent/20 flex-shrink-0">
        {isLoading ? (
          <Loader2 className="w-5 h-5 animate-spin text-editor-accent" />
        ) : (
          <FolderOpen className="w-5 h-5 text-editor-accent" />
        )}
      </div>

      {/* Repository info */}
      <div className="flex-1 min-w-0">
        <div className="flex items-center gap-2">
          <span className="font-semibold text-editor-fg truncate">
            {repository.path.split('/').pop() || repository.path}
          </span>
          {repository.is_active && (
            <span className="text-xs px-2 py-0.5 bg-green-500/20 text-green-400 rounded">
              Active
            </span>
          )}
        </div>

        <div className="text-xs text-editor-fg/60 truncate mt-0.5">
          {repository.path}
        </div>

        <div className="flex items-center gap-3 mt-1">
          {/* Current branch */}
          <div className="flex items-center gap-1 text-xs text-editor-fg/70">
            <GitBranch className="w-3 h-3" />
            <span>{repository.current_branch}</span>
          </div>

          {/* Last opened */}
          {showLastOpened && (
            <div className="flex items-center gap-1 text-xs text-editor-fg/50">
              <Clock className="w-3 h-3" />
              <span>{formatLastOpened(repository.last_opened)}</span>
            </div>
          )}
        </div>
      </div>

      {/* Status indicator */}
      {!isLoading && (
        <div className="w-2 h-2 rounded-full bg-green-500/60 flex-shrink-0" />
      )}
    </button>
  );
};

/**
 * Compact version for sidebar
 */
interface CompactRecentReposProps {
  onRepositorySelect?: (repository: Repository) => void;
  maxDisplay?: number;
  className?: string;
}

export const CompactRecentRepos: React.FC<CompactRecentReposProps> = ({
  onRepositorySelect,
  maxDisplay = 5,
  className = ''
}) => {
  return (
    <RecentRepos
      onRepositorySelect={onRepositorySelect}
      maxDisplay={maxDisplay}
      showLastOpened={false}
      className={className}
    />
  );
};

export default RecentRepos;
