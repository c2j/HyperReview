# Custom Hooks Documentation

This document provides comprehensive documentation for all custom React hooks used in the HyperReview application.

## Table of Contents

- [Overview](#overview)
- [Core Hooks](#core-hooks)
  - [`useIPC`](./useIPC.ts) - Generic IPC communication hook
  - [`useRepository`](./useRepository.ts) - Repository state management
  - [`useComments`](./useComments.ts) - Comment system management
- [Hook Patterns](#hook-patterns)
- [Error Handling](#error-handling)
- [Best Practices](#best-practices)

## Overview

HyperReview uses custom React hooks to abstract IPC (Inter-Process Communication) calls to the Rust backend and manage component state. All hooks follow consistent patterns for error handling, loading states, and data management.

### Design Principles

1. **Consistency**: All hooks follow the same pattern for state management
2. **Error Handling**: Integrated error handling with user-friendly messages
3. **Loading States**: Built-in loading indicators
4. **Type Safety**: Full TypeScript support with proper type definitions
5. **Performance**: Optimized for minimal re-renders

## Core Hooks

### `useIPC`

**Location**: `./useIPC.ts`

**Purpose**: Generic hook for making IPC calls to the Tauri Rust backend.

**Signature**:
```typescript
function useIPC<T, R>(command: string): (args?: T) => Promise<R>
```

**Parameters**:
- `command` (string): The IPC command name to invoke

**Returns**: A function that accepts optional arguments and returns a Promise with the result

**Example**:
```typescript
const getBranchesHook = useIPC<[], Branch[]>('get_branches');

// Usage
const branches = await getBranchesHook();
```

**Error Handling**:
- Automatically logs errors to console
- Throws error for upstream handling
- Does not handle UI state (loading/error) - use wrapper hooks for that

**Implementation Details**:
```typescript
function useIPC<T, R>(command: string) {
  const call = useCallback(async (args?: T): Promise<R> => {
    try {
      const result = await invoke<R>(command, (args as any) || {});
      return result;
    } catch (error) {
      console.error(`IPC call failed for ${command}:`, error);
      throw error;
    }
  }, [command]);

  return call;
}
```

---

### `useRepository`

**Location**: `./useRepository.ts`

**Purpose**: Manages repository state including loading, branches, and current repository.

**State**:
- `currentRepo`: Current repository metadata or null
- `branches`: Array of branches for current repository
- `loading`: Loading state indicator
- `error`: Error message or null

**Methods**:
- `loadRepository(path: string)`: Load a repository by path
- `openRepoDialog()`: Open native file dialog to select repository

**Example**:
```typescript
function RepositorySelector() {
  const {
    currentRepo,
    branches,
    loading,
    error,
    loadRepository,
    openRepoDialog
  } = useRepository();

  return (
    <div>
      <button onClick={openRepoDialog}>Open Repository</button>
      {loading && <Spinner />}
      {error && <ErrorMessage message={error} />}
      {currentRepo && (
        <div>
          <h3>{currentRepo.path}</h3>
          <select value={currentRepo.current_branch}>
            {branches.map(branch => (
              <option key={branch.name} value={branch.name}>
                {branch.name}
              </option>
            ))}
          </select>
        </div>
      )}
    </div>
  );
}
```

**Error Handling**:
- Displays user-friendly error messages
- Provides retry mechanisms where appropriate
- Logs detailed errors to console for debugging

---

### `useComments`

**Location**: `./useComments.ts`

**Purpose**: Manages comment system state and operations.

**State**:
- `comments`: Array of comments for current file
- `loading`: Loading state for comments
- `submitting`: Submitting state for new comments

**Methods**:
- `addComment(filePath, lineNumber, content)`: Add a new comment
- `updateComment(commentId, content)`: Update existing comment
- `deleteComment(commentId)`: Delete a comment
- `getComments(filePath)`: Fetch comments for a file

**Example**:
```typescript
function CommentForm({ filePath, lineNumber }) {
  const { addComment, submitting } = useComments();
  const [content, setContent] = useState('');

  const handleSubmit = async (e) => {
    e.preventDefault();
    await addComment(filePath, lineNumber, content);
    setContent('');
  };

  return (
    <form onSubmit={handleSubmit}>
      <textarea
        value={content}
        onChange={(e) => setContent(e.target.value)}
        disabled={submitting}
      />
      <button type="submit" disabled={submitting || !content.trim()}>
        {submitting ? 'Adding...' : 'Add Comment'}
      </button>
    </form>
  );
}
```

**Offline Support**:
- Automatically creates offline drafts when network is unavailable
- Retries failed submissions when connection is restored
- Shows toast notifications for offline status

---

## Hook Patterns

### Standard Hook Structure

All custom hooks follow this pattern:

```typescript
export function useCustomHook() {
  // State
  const [data, setData] = useState<Type>(initialValue);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Operations
  const operation = useCallback(async (params) => {
    setLoading(true);
    setError(null);
    try {
      const result = await apiCall(params);
      setData(result);
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Unknown error';
      setError(errorMessage);
      throw err;
    } finally {
      setLoading(false);
    }
  }, []);

  // Return state and operations
  return {
    data,
    loading,
    error,
    operation
  };
}
```

### Error Handling Pattern

```typescript
import { useErrorStore } from '../utils/errorHandler';

export function useHookWithErrorHandling() {
  const { showError, showSuccess } = useErrorStore();

  const operation = useCallback(async () => {
    try {
      const result = await apiCall();
      showSuccess('Operation completed successfully');
      return result;
    } catch (error) {
      showError('Operation failed');
      throw error;
    }
  }, []);

  return { operation };
}
```

### Loading State Pattern

```typescript
export function useHookWithLoading() {
  const [loading, setLoading] = useState(false);

  const operation = useCallback(async () => {
    setLoading(true);
    try {
      return await apiCall();
    } finally {
      setLoading(false);
    }
  }, []);

  return { loading, operation };
}
```

---

## Error Handling

### Error Store Integration

All hooks should integrate with the centralized error handling system:

```typescript
import { useErrorStore } from '../utils/errorHandler';

const { addError, showToast } = useErrorStore();

// For critical errors
addError({
  severity: ErrorSeverity.ERROR,
  title: 'Operation Failed',
  message: 'Failed to load repository',
  retryable: true
});

// For user feedback
showToast({
  severity: ErrorSeverity.SUCCESS,
  title: 'Success',
  message: 'Repository loaded successfully'
});
```

### Network Error Handling

For operations that may go offline:

```typescript
import { NetworkErrorHandler } from '../utils/errorHandler';
import { offlineCache } from '../utils/offlineCache';

const operation = useCallback(async () => {
  const { result, isOffline, draftId } = await NetworkErrorHandler.handleWithOffline(
    () => apiCall(),
    'review_submission',
    submissionData,
    'gitlab'
  );

  if (isOffline) {
    console.log('Operation saved as draft:', draftId);
  }

  return result;
}, []);
```

---

## Best Practices

### 1. Keep Hooks Focused

Each hook should have a single, well-defined responsibility:

```typescript
// ✅ Good: Focused hook
function useBranches() {
  // Repository-specific branch logic
}

// ❌ Bad: Multiple responsibilities
function useRepositoryData() {
  // Branches, commits, tags, diffs, comments...
}
```

### 2. Use TypeScript Generics

Always use proper TypeScript types:

```typescript
// ✅ Good: Typed hook
function useIPC<T, R>(command: string): (args?: T) => Promise<R>

// ❌ Bad: No types
function useIPC(command) // Returns any
```

### 3. Memoize Expensive Operations

Use `useCallback` for functions that are dependencies:

```typescript
const operation = useCallback(async (params) => {
  return await apiCall(params);
}, []); // Stable reference
```

### 4. Handle Loading States

Always provide loading indicators:

```typescript
const [loading, setLoading] = useState(false);

// Set loading before operation
setLoading(true);
try {
  return await apiCall();
} finally {
  setLoading(false);
}
```

### 5. Provide Clear Error Messages

Make errors actionable for users:

```typescript
setError('Failed to load repository. Please check the path and try again.');
```

### 6. Cleanup Side Effects

Clean up timeouts, listeners, etc.:

```typescript
useEffect(() => {
  const interval = setInterval(() => {
    // Poll for updates
  }, 5000);

  return () => clearInterval(interval);
}, []);
```

### 7. Document Hook Usage

Include JSDoc comments for all hooks:

```typescript
/**
 * Hook for managing repository operations
 *
 * @returns {Object} Repository management interface
 * @returns {Function} loadRepository - Load a repository by path
 * @returns {Function} openRepoDialog - Open file dialog
 *
 * @example
 * const { loadRepository, currentRepo } = useRepository();
 * await loadRepository('/path/to/repo');
 */
export function useRepository() {
  // Implementation
}
```

---

## Testing Hooks

### Unit Testing

Use React Testing Library to test hooks:

```typescript
import { renderHook, act } from '@testing-library/react';
import { useRepository } from './useRepository';

test('should load repository', async () => {
  const { result } = renderHook(() => useRepository());

  await act(async () => {
    await result.current.loadRepository('/test/repo');
  });

  expect(result.current.currentRepo).toBeDefined();
  expect(result.current.loading).toBe(false);
});
```

### Mocking IPC Calls

```typescript
jest.mock('@tauri-apps/api/core', () => ({
  invoke: jest.fn()
}));

test('should call invoke with correct command', async () => {
  const { invoke } = require('@tauri-apps/api/core');
  invoke.mockResolvedValue(mockData);

  const { result } = renderHook(() => useIPC('get_branches'));

  await act(async () => {
    await result.current();
  });

  expect(invoke).toHaveBeenCalledWith('get_branches', {});
});
```

---

## Performance Considerations

### 1. Avoid Unnecessary Re-renders

```typescript
// ✅ Good: Memoize callbacks
const handleClick = useCallback(() => {
  onAction();
}, [onAction]);

// ❌ Bad: New function on every render
const handleClick = () => {
  onAction();
};
```

### 2. Use Appropriate Dependencies

```typescript
// ✅ Good: Correct dependencies
const operation = useCallback(() => {
  return apiCall(params);
}, [params]); // Only depends on params

// ❌ Bad: Missing dependencies
const operation = useCallback(() => {
  return apiCall(params);
}, []); // Forgets params
```

### 3. Lazy Load Heavy Hooks

For hooks that initialize expensive resources:

```typescript
const useExpensiveHook = () => {
  const [initialized, setInitialized] = useState(false);

  useEffect(() => {
    if (!initialized) {
      initializeExpensiveResource();
      setInitialized(true);
    }
  }, [initialized]);

  // Return hook interface
};
```

---

## API Reference

### useIPC

```typescript
/**
 * Generic IPC communication hook
 * @param command - IPC command name
 * @returns Function to invoke the command
 */
function useIPC<T, R>(command: string): (args?: T) => Promise<R>
```

### useRepository

```typescript
/**
 * Repository state management hook
 * @returns Repository management interface
 */
function useRepository(): {
  currentRepo: Repository | null;
  branches: Branch[];
  loading: boolean;
  error: string | null;
  loadRepository: (path: string) => Promise<void>;
  openRepoDialog: () => Promise<void>;
}
```

### useComments

```typescript
/**
 * Comment system management hook
 * @returns Comment operations interface
 */
function useComments(): {
  comments: Comment[];
  loading: boolean;
  submitting: boolean;
  addComment: (filePath: string, lineNumber: number, content: string) => Promise<void>;
  updateComment: (commentId: string, content: string) => Promise<void>;
  deleteComment: (commentId: string) => Promise<void>;
  getComments: (filePath: string) => Promise<void>;
}
```

---

## Contributing

When adding new hooks:

1. Follow the established patterns in this document
2. Add JSDoc documentation
3. Include TypeScript types
4. Add unit tests
5. Update this README
6. Consider adding to the Storybook if applicable

---

## Troubleshooting

### Common Issues

**Hook not updating component:**
- Check dependencies in `useCallback` or `useEffect`
- Ensure state setters are properly typed

**IPC calls failing:**
- Verify command name matches backend
- Check error handling in hook
- Review Tauri IPC documentation

**Memory leaks:**
- Clean up timers, listeners, subscriptions
- Use `useEffect` cleanup functions

---

## Resources

- [React Hooks Documentation](https://react.dev/reference/react)
- [Tauri IPC Guide](https://tauri.app/v2/guides/building/ipc)
- [TypeScript Generics](https://www.typescriptlang.org/docs/handbook/2/generics.html)
- [React Testing Library](https://testing-library.com/docs/react-testing-library/intro/)
