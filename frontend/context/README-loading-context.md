# Loading Context

A centralized loading state management system for the HyperReview application. Provides granular loading states for different operations and integrates with the error handling system.

## Overview

The Loading Context provides:
- Granular loading states for different operations (repository, diff, tasks, analysis)
- Integration with the global error store
- Convenience hooks for specific loading states
- Helper functions to wrap async operations with loading indicators

## Setup

### 1. Wrap Your App with LoadingProvider

```tsx
import { LoadingProvider } from './context/LoadingContext';

function App() {
  return (
    <LoadingProvider>
      {/* Your app content */}
    </LoadingProvider>
  );
}
```

### 2. Use Loading States in Components

```tsx
import { useLoading } from './context/LoadingContext';

function MyComponent() {
  const { isRepositoryLoading, setRepositoryLoading } = useLoading();

  const loadRepository = async () => {
    setRepositoryLoading(true);
    try {
      await loadRepo();
    } finally {
      setRepositoryLoading(false);
    }
  };

  return (
    <div>
      {isRepositoryLoading ? (
        <LoadingSpinner text="Loading repository..." />
      ) : (
        <button onClick={loadRepository}>
          Load Repository
        </button>
      )}
    </div>
  );
}
```

## API Reference

### LoadingProvider

```tsx
<LoadingProvider>
  <YourApp />
</LoadingProvider>
```

Wraps your app and provides loading state management.

### useLoading Hook

```tsx
import { useLoading } from './context/LoadingContext';

const {
  isRepositoryLoading,  // Boolean
  isDiffLoading,        // Boolean
  isTaskLoading,        // Boolean
  isAnalysisLoading,    // Boolean
  setRepositoryLoading, // (loading: boolean) => void
  setDiffLoading,       // (loading: boolean) => void
  setTaskLoading,       // (loading: boolean) => void
  setAnalysisLoading,   // (loading: boolean) => void
  isAnyLoading          // Boolean (any operation loading)
} = useLoading();
```

### Specific Loading Hooks

Use these hooks for specific loading states:

```tsx
import {
  useRepositoryLoading,
  useDiffLoading,
  useTaskLoading,
  useAnalysisLoading
} from './context/LoadingContext';

// Repository loading
const { isLoading, setLoading } = useRepositoryLoading();

// Diff loading
const { isLoading, setLoading } = useDiffLoading();

// Task loading
const { isLoading, setLoading } = useTaskLoading();

// Analysis loading
const { isLoading, setLoading } = useAnalysisLoading();

// Any loading (combined)
const isAnyLoading = useAnyLoading();
```

### useWithLoading Hook

Automatically manage loading states for async operations:

```tsx
import { useWithLoading } from './context/LoadingContext';

function MyComponent() {
  const {
    withRepositoryLoading,
    withDiffLoading,
    withTaskLoading,
    withAnalysisLoading
  } = useWithLoading();

  const loadRepo = async () => {
    const result = await withRepositoryLoading(async () => {
      // Your async operation here
      return await fetchRepository();
    });

    if (result) {
      // Handle success
      console.log('Repository loaded:', result);
    }
  };

  return (
    <button onClick={loadRepo}>
      Load Repository
    </button>
  );
}
```

## Loading States

### Repository Loading
Used for operations that load or modify repositories:
- Opening a repository
- Loading repository metadata
- Syncing with remote
- Switching branches

### Diff Loading
Used for diff-related operations:
- Loading file diffs
- Comparing branches
- Loading file history
- Getting blame information

### Task Loading
Used for task-related operations:
- Loading tasks
- Creating tasks
- Updating task status
- Importing tasks

### Analysis Loading
Used for analysis operations:
- Generating heatmaps
- Running security scans
- Analyzing complexity
- Loading checklists

## Integration with Error Store

The loading context automatically updates the global error store:

```typescript
// When any loading state changes, the error store is notified
useErrorStore.getState().setLoading(isAnyLoading);
```

This allows other parts of the application to know when operations are in progress.

## Usage Examples

### Repository Operations

```tsx
function RepositoryLoader() {
  const { isLoading, setLoading } = useRepositoryLoading();

  const openRepository = async () => {
    setLoading(true);
    try {
      const path = await openRepoDialog();
      if (path) {
        await loadRepository(path);
        showSuccess('Repository loaded successfully!');
      }
    } catch (error) {
      showError('Failed to load repository');
    } finally {
      setLoading(false);
    }
  };

  return (
    <button onClick={openRepository} disabled={isLoading}>
      {isLoading ? 'Loading...' : 'Open Repository'}
    </button>
  );
}
```

### Diff Operations

```tsx
function DiffViewer({ filePath }) {
  const { isLoading, setLoading } = useDiffLoading();
  const { currentDiff, setCurrentDiff } = useReviewStore();

  useEffect(() => {
    const loadDiff = async () => {
      setLoading(true);
      try {
        const diff = await getFileDiff({ file_path: filePath });
        setCurrentDiff(diff);
      } catch (error) {
        showError('Failed to load diff');
      } finally {
        setLoading(false);
      }
    };

    loadDiff();
  }, [filePath]);

  return (
    <div>
      {isLoading ? (
        <LoadingSpinner text="Loading diff..." />
      ) : (
        <DiffView diff={currentDiff} />
      )}
    </div>
  );
}
```

### Task Operations

```tsx
function TaskList() {
  const { isLoading, setLoading } = useTaskLoading();
  const { tasks, setTasks } = useTaskStore();

  const loadTasks = async () => {
    const result = await withTaskLoading(async () => {
      return await getTasks();
    });

    if (result) {
      setTasks(result);
    }
  };

  return (
    <div>
      {isLoading ? (
        <LoadingSpinner text="Loading tasks..." />
      ) : (
        <TaskTree tasks={tasks} onRefresh={loadTasks} />
      )}
    </div>
  );
}
```

### Using with IPC Hooks

```tsx
function BranchSelector() {
  const { isLoading, setLoading } = useRepositoryLoading();
  const getBranches = useGetBranches();

  const handleLoadBranches = async () => {
    const result = await withRepositoryLoading(async () => {
      return await getBranches();
    });

    if (result) {
      // Handle branches
      console.log('Branches:', result);
    }
  };

  return (
    <button onClick={handleLoadBranches} disabled={isLoading}>
      {isLoading ? (
        <>
          <ButtonSpinner />
          Loading Branches...
        </>
      ) : (
        'Load Branches'
      )}
    </button>
  );
}
```

## Best Practices

### 1. Use Specific Loading States

Choose the most appropriate loading state for your operation:

```tsx
// Good - Using specific loading state
const { setDiffLoading } = useDiffLoading();

// Okay - Using general loading state
const { isAnyLoading } = useLoading();
```

### 2. Always Reset Loading State

Use try/finally to ensure loading state is reset:

```tsx
const loadData = async () => {
  setLoading(true);
  try {
    await operation();
  } finally {
    setLoading(false); // Always executed
  }
};
```

### 3. Use withLoading Helper

For cleaner code, use the withLoading helper:

```tsx
// Clean and simple
const result = await withDiffLoading(async () => {
  return await getFileDiff({ file_path });
});

// Manual loading state management
setDiffLoading(true);
try {
  const result = await getFileDiff({ file_path });
} finally {
  setDiffLoading(false);
}
```

### 4. Show Loading in UI

Always provide visual feedback when operations are loading:

```tsx
{isLoading ? (
  <LoadingSpinner text="Loading..." />
) : (
  <YourComponent />
)}
```

### 5. Disable Actions During Loading

Prevent multiple simultaneous operations:

```tsx
<button onClick={handleAction} disabled={isLoading}>
  {isLoading ? 'Processing...' : 'Submit'}
</button>
```

## Troubleshooting

### Loading State Not Updating

1. Ensure you're using the correct hook (e.g., `useRepositoryLoading` for repository operations)
2. Check that `setLoading` is being called
3. Verify the component is re-rendering after state change

### Global Loading Not Working

1. Ensure `LoadingProvider` wraps your app
2. Check that error store is initialized
3. Verify `isAnyLoading` is being used correctly

### Multiple Loading States

If you have multiple async operations, consider using specific loading states:

```tsx
// Good - Different loading states for different operations
const { isRepositoryLoading } = useRepositoryLoading();
const { isDiffLoading } = useDiffLoading();

// May be confusing
const { isAnyLoading } = useLoading();
```

## Advanced Usage

### Custom Loading State

Create custom loading states by combining existing states:

```tsx
function CustomComponent() {
  const { isRepository isDiffLoading } = useLoadingLoading,();

  // Custom combined loading state
  const isCustomLoading = isRepositoryLoading || isDiffLoading;

  return (
    <div>
      {isCustomLoading && <LoadingSpinner />}
      {/* Your content */}
    </div>
  );
}
```

### Loading State Persistence

Persist loading state across navigation:

```tsx
function usePersistentLoading(key: string) {
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    // Restore loading state on mount
    const persisted = sessionStorage.getItem(`loading_${key}`);
    if (persisted === 'true') {
      setLoading(true);
    }
  }, [key]);

  const setLoadingWithPersistence = (value: boolean) => {
    setLoading(value);
    sessionStorage.setItem(`loading_${key}`, value.toString());
  };

  return { isLoading: loading, setLoading: setLoadingWithPersistence };
}
```

## Integration Examples

See the following for complete examples:
- `context/LoadingContext.tsx` - Loading context implementation
- `components/LoadingSpinner.tsx` - Loading indicator components
- `utils/errorHandler.ts` - Error handling integration
