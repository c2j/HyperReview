/**
 * Repository Selector Component
 * Allows users to select and open Git repositories
 */

import React, { useState, useCallback } from 'react';
import { FolderOpen, Loader2 } from 'lucide-react';
import { useRepositoryActions } from '../hooks/useRepository';
import { useRecentRepositories } from '../hooks/useRepository';
import { useLoading } from '../context/LoadingContext';
import type { Repository } from '../api/types';

interface RepositorySelectorProps {
  onRepositorySelected?: (repository: Repository) => void;
  className?: string;
}

/**
 * Repository selector with dialog and recent repositories list
 */
const RepositorySelector: React.FC<RepositorySelectorProps> = ({
  onRepositorySelected,
  className = ''
}) => {
  const { openRepository, switchRepository } = useRepositoryActions();
  const { recentRepos, loadRecentRepos } = useRecentRepositories();
  const { isRepositoryLoading } = useLoading();
  const [selectedRepoPath, setSelectedRepoPath] = useState<string | null>(null);

  // Handle opening new repository
  const handleOpenRepository = useCallback(async () => {
    const repository = await openRepository();
    if (repository && onRepositorySelected) {
      onRepositorySelected(repository);
    }
  }, [openRepository, onRepositorySelected]);

  // Handle selecting from recent repositories
  const handleSelectRecent = useCallback(
    async (repo: Repository) => {
      setSelectedRepoPath(repo.path);
      const repository = await switchRepository(repo.path);
      if (repository && onRepositorySelected) {
        onRepositorySelected(repository);
      }
      setSelectedRepoPath(null);
    },
    [switchRepository, onRepositorySelected]
  );

  // Load recent repositories on mount
  const hasLoaded = React.useRef(false);
  React.useEffect(() => {
    if (!hasLoaded.current) {
      hasLoaded.current = true;
      loadRecentRepos();
    }
  }, [loadRecentRepos]);

  return (
    <div className={`space-y-4 ${className}`}>
      {/* Open Repository Button */}
      <button
        onClick={handleOpenRepository}
        disabled={isRepositoryLoading}
        className="w-full flex items-center justify-center gap-3 px-6 py-4 bg-editor-accent hover:bg-editor-accent/90 disabled:opacity-50 disabled:cursor-not-allowed text-white rounded-lg transition-colors font-semibold"
      >
        {isRepositoryLoading ? (
          <Loader2 className="w-5 h-5 animate-spin" />
        ) : (
          <FolderOpen className="w-5 h-5" />
        )}
        <span>{isRepositoryLoading ? 'Opening...' : 'Open Repository (Beta)'}</span>
      </button>

      {/* Recent Repositories */}
      <div className="space-y-2">
        <h3 className="text-sm font-semibold text-editor-fg/80 uppercase tracking-wide">
          Recent Repositories
        </h3>

        {recentRepos.length === 0 ? (
          <div className="text-center py-8 text-editor-fg/60 text-sm border border-editor-line rounded-lg border-dashed">
            <FolderOpen className="w-8 h-8 mx-auto mb-2 opacity-50" />
            <div>No recent repositories</div>
            <div className="text-xs mt-1">Open a repository to get started</div>
          </div>
        ) : (
          <div className="space-y-2 max-h-64 overflow-y-auto">
            {recentRepos.map((repo) => (
              <RepositoryItem
                key={repo.path}
                repository={repo}
                isLoading={selectedRepoPath === repo.path}
                onClick={() => handleSelectRecent(repo)}
              />
            ))}
          </div>
        )}
      </div>
    </div>
  );
};

/**
 * Individual repository item component
 */
interface RepositoryItemProps {
  repository: Repository;
  isLoading?: boolean;
  onClick?: () => void;
}

const RepositoryItem: React.FC<RepositoryItemProps> = ({
  repository,
  isLoading = false,
  onClick
}) => {
  const handleClick = useCallback(() => {
    if (!isLoading && onClick) {
      onClick();
    }
  }, [isLoading, onClick]);

  return (
    <button
      onClick={handleClick}
      disabled={isLoading}
      className="w-full flex items-center gap-3 px-4 py-3 bg-editor-bg hover:bg-editor-selection/30 disabled:opacity-50 disabled:cursor-not-allowed border border-editor-line rounded-lg transition-colors text-left group"
    >
      {/* Repository Icon */}
      <div className="w-10 h-10 flex items-center justify-center bg-editor-accent/10 rounded border border-editor-accent/20 group-hover:border-editor-accent/40 transition-colors">
        <FolderOpen className="w-5 h-5 text-editor-accent" />
      </div>

      {/* Repository Info */}
      <div className="flex-1 min-w-0">
        <div className="font-semibold text-editor-fg truncate">
          {repository.path.split('/').pop() || repository.path}
        </div>
        <div className="text-xs text-editor-fg/60 truncate">
          {repository.path}
        </div>
        <div className="flex items-center gap-2 mt-1">
          <span className="text-xs px-2 py-0.5 bg-blue-500/20 text-blue-400 rounded">
            {repository.current_branch}
          </span>
          {repository.is_active && (
            <span className="text-xs px-2 py-0.5 bg-green-500/20 text-green-400 rounded">
              Active
            </span>
          )}
        </div>
      </div>

      {/* Loading Spinner */}
      {isLoading && <Loader2 className="w-5 h-5 animate-spin text-editor-accent" />}
    </button>
  );
};

/**
 * Modal version of repository selector
 */
interface RepositorySelectorModalProps extends RepositorySelectorProps {
  isOpen: boolean;
  onClose: () => void;
}

export const RepositorySelectorModal: React.FC<RepositorySelectorModalProps> = ({
  isOpen,
  onClose,
  onRepositorySelected,
  ...props
}) => {
  if (!isOpen) {
    return null;
  }

  const handleRepositorySelected = useCallback(
    (repository: Repository) => {
      if (onRepositorySelected) {
        onRepositorySelected(repository);
      }
      onClose();
    },
    [onRepositorySelected, onClose]
  );

  return (
    <div className="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50">
      <div className="bg-editor-bg border border-editor-line rounded-lg shadow-xl w-full max-w-2xl mx-4 max-h-[80vh] overflow-hidden">
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-editor-line">
          <h2 className="text-xl font-bold text-editor-fg">Select Repository</h2>
          <button
            onClick={onClose}
            className="text-editor-fg/60 hover:text-editor-fg transition-colors"
          >
            ✕
          </button>
        </div>

        {/* Content */}
        <div className="p-6 overflow-y-auto">
          <RepositorySelector
            {...props}
            onRepositorySelected={handleRepositorySelected}
          />
        </div>
      </div>
    </div>
  );
};

export default RepositorySelector;
