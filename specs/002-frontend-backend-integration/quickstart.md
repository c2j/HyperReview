# Quick Start: Frontend-Backend Integration

**Date**: 2025-12-14
**Feature**: 002-frontend-backend-integration

## Overview

This guide helps developers quickly understand and implement the frontend-backend integration for HyperReview. The backend is complete—focus is on integrating the React frontend with existing Tauri IPC commands.

## Prerequisites

### Development Environment
- Node.js 18+ and npm/yarn
- Rust 1.75+ and Cargo
- Git (for testing with real repositories)

### Project Setup
```bash
# Clone and setup project
git clone <repo>
cd HyperReview

# Install frontend dependencies
npm install

# Verify backend builds
cd src-tauri
cargo check
cargo build
cd ..
```

## Architecture Overview

```
┌─────────────────────┐
│   React Frontend    │
│  (TypeScript/TSX)   │
└──────────┬──────────┘
           │ Tauri IPC
           │ invoke()
           ↓
┌─────────────────────┐
│  Rust Backend       │
│  (Tauri Commands)   │
└──────────┬──────────┘
           │
           ↓
    ┌─────────────┐
    │ SQLite DB   │
    │ (Metadata)  │
    └─────────────┘
           │
           ↓
    ┌─────────────┐
    │ Git Repo    │
    │ (Code)      │
    └─────────────┘
```

## Integration Pattern

### 1. Create API Service Layer

**Location**: `src/services/api.ts`

```typescript
// Wrapper for all Tauri IPC calls
export const api = {
  // Repository Management
  async openRepoDialog(): Promise<string | null> {
    return await invoke('open_repo_dialog');
  },

  async getRecentRepos(): Promise<Repo[]> {
    return await invoke('get_recent_repos');
  },

  async getBranches(): Promise<Branch[]> {
    return await invoke('get_branches');
  },

  async loadRepo(path: string): Promise<Repo> {
    return await invoke('load_repo', { path });
  },

  // Code Review
  async getFileDiff(
    filePath: string,
    oldCommit?: string,
    newCommit?: string
  ): Promise<DiffLine[]> {
    return await invoke('get_file_diff', {
      file_path: filePath,
      old_commit: oldCommit,
      new_commit: newCommit
    });
  },

  async addComment(
    filePath: string,
    lineNumber: number,
    content: string
  ): Promise<Comment> {
    return await invoke('add_comment', {
      file_path: filePath,
      line_number: lineNumber,
      content
    });
  },

  // ... add all 21 commands
};
```

### 2. Type Definitions

**Location**: `src/types/api.ts`

```typescript
// Match backend models exactly
export interface Repo {
  path: string;
  current_branch: string;
  last_opened: string;
  head_commit: string;
  remote_url?: string;
  is_active: boolean;
}

export interface DiffLine {
  old_line_number?: number;
  new_line_number?: number;
  content: string;
  line_type: 'Added' | 'Removed' | 'Context' | 'Header';
  severity?: 'Error' | 'Warning' | 'Info' | 'Success';
  message?: string;
  hunk_header?: string;
}

// ... all other types
```

### 3. Custom Hook for State

**Location**: `src/hooks/useRepository.ts`

```typescript
import { useState, useEffect } from 'react';
import { api } from '../services/api';
import type { Repo, Branch } from '../types/api';

export function useRepository() {
  const [currentRepo, setCurrentRepo] = useState<Repo | null>(null);
  const [branches, setBranches] = useState<Branch[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadRepository = async (path: string) => {
    try {
      setLoading(true);
      setError(null);
      const repo = await api.loadRepo(path);
      setCurrentRepo(repo);
      const branchList = await api.getBranches();
      setBranches(branchList);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load repository');
    } finally {
      setLoading(false);
    }
  };

  const openRepoDialog = async () => {
    try {
      setLoading(true);
      setError(null);
      const path = await api.openRepoDialog();
      if (path) {
        await loadRepository(path);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to open repository');
    } finally {
      setLoading(false);
    }
  };

  return {
    currentRepo,
    branches,
    loading,
    error,
    loadRepository,
    openRepoDialog
  };
}
```

### 4. Component Integration

**Location**: `src/components/RepositorySelector.tsx`

```tsx
import React from 'react';
import { useRepository } from '../hooks/useRepository';

export function RepositorySelector() {
  const { currentRepo, branches, loading, error, openRepoDialog } = useRepository();

  return (
    <div>
      <button onClick={openRepoDialog} disabled={loading}>
        {loading ? 'Loading...' : 'Open Repository'}
      </button>

      {error && (
        <div className="error">
          Error: {error}
        </div>
      )}

      {currentRepo && (
        <div className="repo-info">
          <h3>{currentRepo.path}</h3>
          <p>Current Branch: {currentRepo.current_branch}</p>
          <select value={currentRepo.current_branch}>
            {branches.map(branch => (
              <option key={branch.name} value={branch.name}>
                {branch.name} {branch.is_current ? '(current)' : ''}
              </option>
            ))}
          </select>
        </div>
      )}
    </div>
  );
}
```

### 5. Diff Viewer with Virtual Scrolling

**Location**: `src/components/DiffViewer.tsx`

```tsx
import React, { useState, useEffect } from 'react';
import { FixedSizeList as List } from 'react-window';
import { api } from '../services/api';
import type { DiffLine } from '../types/api';

interface DiffViewerProps {
  filePath: string;
}

export function DiffViewer({ filePath }: DiffViewerProps) {
  const [diff, setDiff] = useState<DiffLine[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const loadDiff = async () => {
      try {
        setLoading(true);
        const diffData = await api.getFileDiff(filePath);
        setDiff(diffData);
      } catch (err) {
        console.error('Failed to load diff:', err);
      } finally {
        setLoading(false);
      }
    };

    loadDiff();
  }, [filePath]);

  const Row = ({ index, style }: { index: number; style: React.CSSProperties }) => {
    const line = diff[index];
    const lineClass = `diff-line ${line.line_type.toLowerCase()}`;

    return (
      <div style={style} className={lineClass}>
        <span className="line-number">
          {line.old_line_number ?? ''} {line.new_line_number ?? ''}
        </span>
        <span className="content">{line.content}</span>
        {line.message && (
          <span className="analysis">{line.message}</span>
        )}
      </div>
    );
  };

  if (loading) return <div>Loading diff...</div>;

  return (
    <List
      height={600}
      itemCount={diff.length}
      itemSize={30}
      width="100%"
    >
      {Row}
    </List>
  );
}
```

### 6. Comment System

**Location**: `src/components/CommentForm.tsx`

```tsx
import React, { useState } from 'react';
import { api } from '../services/api';

interface CommentFormProps {
  filePath: string;
  lineNumber: number;
  onCommentAdded: (comment: Comment) => void;
}

export function CommentForm({ filePath, lineNumber, onCommentAdded }: CommentFormProps) {
  const [content, setContent] = useState('');
  const [submitting, setSubmitting] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!content.trim()) return;

    try {
      setSubmitting(true);
      const comment = await api.addComment(filePath, lineNumber, content);
      onCommentAdded(comment);
      setContent('');
    } catch (err) {
      alert(`Failed to add comment: ${err}`);
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      <textarea
        value={content}
        onChange={(e) => setContent(e.target.value)}
        placeholder={`Comment on line ${lineNumber}...`}
        rows={3}
      />
      <button type="submit" disabled={submitting || !content.trim()}>
        {submitting ? 'Adding...' : 'Add Comment'}
      </button>
    </form>
  );
}
```

## Error Handling Strategy

### Global Error Boundary

**Location**: `src/components/ErrorBoundary.tsx`

```tsx
import React, { Component, ErrorInfo, ReactNode } from 'react';

interface Props {
  children: ReactNode;
}

interface State {
  hasError: boolean;
  error?: Error;
}

export class ErrorBoundary extends Component<Props, State> {
  public state: State = {
    hasError: false
  };

  public static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  public componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error('Uncaught error:', error, errorInfo);
  }

  public render() {
    if (this.state.hasError) {
      return (
        <div className="error-boundary">
          <h2>Something went wrong.</h2>
          <details>
            <summary>Error details</summary>
            <pre>{this.state.error?.toString()}</pre>
          </details>
        </div>
      );
    }

    return this.props.children;
  }
}
```

### Toast Notification System

**Location**: `src/hooks/useToast.ts`

```typescript
import { useState, useCallback } from 'react';

type ToastType = 'success' | 'error' | 'info' | 'warning';

interface Toast {
  id: string;
  type: ToastType;
  message: string;
}

export function useToast() {
  const [toasts, setToasts] = useState<Toast[]>([]);

  const showToast = useCallback((type: ToastType, message: string) => {
    const id = Math.random().toString(36).substring(7);
    setToasts(prev => [...prev, { id, type, message }]);

    // Auto-dismiss after 5 seconds
    setTimeout(() => {
      setToasts(prev => prev.filter(t => t.id !== id));
    }, 5000);
  }, []);

  const dismiss = useCallback((id: string) => {
    setToasts(prev => prev.filter(t => t.id !== id));
  }, []);

  return { toasts, showToast, dismiss };
}
```

## State Management

### Zustand Store

**Location**: `src/store/reviewStore.ts`

```typescript
import { create } from 'zustand';
import type { Repo, Branch, DiffLine, Comment } from '../types/api';

interface ReviewState {
  currentRepo: Repo | null;
  branches: Branch[];
  currentDiff: DiffLine[];
  comments: Comment[];
  loading: boolean;
  error: string | null;

  // Actions
  setCurrentRepo: (repo: Repo | null) => void;
  setBranches: (branches: Branch[]) => void;
  setCurrentDiff: (diff: DiffLine[]) => void;
  addComment: (comment: Comment) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
}

export const useReviewStore = create<ReviewState>((set) => ({
  currentRepo: null,
  branches: [],
  currentDiff: [],
  comments: [],
  loading: false,
  error: null,

  setCurrentRepo: (repo) => set({ currentRepo: repo }),
  setBranches: (branches) => set({ branches }),
  setCurrentDiff: (diff) => set({ currentDiff: diff }),
  addComment: (comment) => set((state) => ({ comments: [...state.comments, comment] })),
  setLoading: (loading) => set({ loading }),
  setError: (error) => set({ error }),
}));
```

## Testing Strategy

### Unit Tests

**Location**: `src/services/__tests__/api.test.ts`

```typescript
import { api } from '../api';

// Mock Tauri invoke
jest.mock('@tauri-apps/api/core', () => ({
  invoke: jest.fn()
}));

import { invoke } from '@tauri-apps/api/core';

describe('API Service', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  test('loadRepo should call invoke with correct params', async () => {
    const mockRepo = { path: '/test/repo', current_branch: 'main' };
    invoke.mockResolvedValue(mockRepo);

    const result = await api.loadRepo('/test/repo');

    expect(invoke).toHaveBeenCalledWith('load_repo', { path: '/test/repo' });
    expect(result).toEqual(mockRepo);
  });

  test('addComment should handle errors', async () => {
    invoke.mockRejectedValue(new Error('Database error'));

    await expect(api.addComment('file.ts', 10, 'Test comment'))
      .rejects.toThrow('Database error');
  });
});
```

### Integration Tests

**Location**: `src/__tests__/review-workflow.test.tsx`

```tsx
import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { App } from '../App';

// Mock Tauri APIs
Object.defineProperty(window, '__TAURI__', {
  value: {
    invoke: jest.fn()
  }
});

describe('Review Workflow', () => {
  test('complete workflow: open repo → view diff → add comment', async () => {
    const mockInvoke = window.__TAURI__.invoke;
    mockInvoke
      .mockResolvedValueOnce({ path: '/test/repo', current_branch: 'main' }) // loadRepo
      .mockResolvedValueOnce([{ line_type: 'Added', content: 'new line' }]); // getFileDiff

    render(<App />);

    // Open repository
    fireEvent.click(screen.getByText('Open Repository'));
    await waitFor(() => {
      expect(mockInvoke).toHaveBeenCalledWith('open_repo_dialog');
    });

    // View diff
    fireEvent.click(screen.getByText('View Diff'));
    await waitFor(() => {
      expect(mockInvoke).toHaveBeenCalledWith('get_file_diff', {
        file_path: expect.any(String)
      });
    });
  });
});
```

## Performance Tips

### 1. Virtual Scrolling for Large Lists
```tsx
import { FixedSizeList as List } from 'react-window';

// For diffs with 10k+ lines
<List
  height={600}
  itemCount={diffLines.length}
  itemSize={30}
  overscanCount={10} // Render extra rows for smooth scrolling
>
  {Row}
</List>
```

### 2. Debounce Search Input
```typescript
import { useDebounce } from 'use-debounce';

function SearchBox() {
  const [query, setQuery] = useState('');
  const [debouncedQuery] = useDebounce(query, 300);

  useEffect(() => {
    if (debouncedQuery) {
      search(debouncedQuery);
    }
  }, [debouncedQuery]);

  return <input value={query} onChange={(e) => setQuery(e.target.value)} />;
}
```

### 3. Memoize Expensive Components
```tsx
import { memo, useMemo } from 'react';

const DiffViewer = memo(({ diff }: { diff: DiffLine[] }) => {
  const stats = useMemo(() => ({
    added: diff.filter(d => d.line_type === 'Added').length,
    removed: diff.filter(d => d.line_type === 'Removed').length
  }), [diff]);

  return (
    <div>
      <Stats added={stats.added} removed={stats.removed} />
      {/* Render diff */}
    </div>
  );
});
```

## Common Pitfalls

### ❌ Don't: Mix Business Logic in Components
```tsx
// BAD: Business logic in component
function Component() {
  const [data, setData] = useState([]);

  const processData = async () => {
    const raw = await invoke('get_data');
    const processed = raw.map(item => ({
      ...item,
      computed: expensiveCalculation(item)
    }));
    setData(processed);
  };
}
```

### ✅ Do: Separate Concerns
```typescript
// GOOD: Business logic in service
export const dataService = {
  async getProcessedData() {
    const raw = await invoke('get_data');
    return raw.map(item => ({
      ...item,
      computed: expensiveCalculation(item)
    }));
  }
};

// Component just uses the service
function Component() {
  const [data, setData] = useState([]);
  useEffect(() => {
    dataService.getProcessedData().then(setData);
  }, []);
}
```

### ❌ Don't: Ignore Error Handling
```typescript
// BAD: No error handling
async function loadRepo(path: string) {
  const repo = await invoke('load_repo', { path });
  setRepo(repo);
}
```

### ✅ Do: Comprehensive Error Handling
```typescript
// GOOD: Proper error handling
async function loadRepo(path: string) {
  try {
    setLoading(true);
    const repo = await invoke('load_repo', { path });
    setRepo(repo);
    setError(null);
  } catch (err) {
    setError(err instanceof Error ? err.message : 'Failed to load repository');
    setRepo(null);
  } finally {
    setLoading(false);
  }
}
```

## Debugging Tips

### 1. Enable Tauri Debug Logging
```bash
# Terminal 1
npm run tauri dev

# Backend logs will appear in terminal
```

### 2. Frontend DevTools
```typescript
// Add to main.tsx for development
if (import.meta.env.DEV) {
  window.addEventListener('keydown', (e) => {
    if (e.ctrlKey && e.key === 'd') {
      // Toggle debug mode
      console.log('Debug mode:', { currentRepo, state });
    }
  });
}
```

### 3. IPC Call Logging
```typescript
// Wrap invoke calls for debugging
const invokeWithLogging = async (command: string, params?: any) => {
  console.log(`IPC Call: ${command}`, params);
  try {
    const result = await invoke(command, params);
    console.log(`IPC Success: ${command}`, result);
    return result;
  } catch (err) {
    console.error(`IPC Error: ${command}`, err);
    throw err;
  }
};
```

## Next Steps

1. **Review the API Contract**: See [contracts/ipc-commands.md](./contracts/ipc-commands.md)
2. **Understand the Data Model**: See [data-model.md](./data-model.md)
3. **Read the Research**: See [research.md](./research.md)
4. **Check Implementation Tasks**: Run `/speckit.tasks` to generate task breakdown

## Resources

- Tauri IPC Guide: https://tauri.app/v2/guides/building/ipc
- React Best Practices: https://react.dev/learn
- TypeScript Handbook: https://www.typescriptlang.org/docs/
- Zustand State Management: https://github.com/pmndrs/zustand
- React Virtual: https://github.com/tanstack/react-virtual

## Support

For questions or issues:
- Review the specification: [spec.md](./spec.md)
- Check the implementation plan: [plan.md](./plan.md)
- Reference backend docs: `src-tauri/docs/api.md`
