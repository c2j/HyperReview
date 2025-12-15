# Error Handling System

A comprehensive error handling system for the HyperReview application that provides centralized error management, user-friendly notifications, and robust IPC error handling.

## Overview

The error handling system consists of:

1. **Error Store** - Zustand-based state management for errors and notifications
2. **Error Formatter** - Converts technical errors into user-friendly messages
3. **IPC Error Handler** - Specialized handlers for Tauri IPC operations
4. **Repository Error Handler** - Domain-specific error handling for Git operations
5. **Toast Notification System** - Visual feedback for errors and successes
6. **Error Boundary** - React component error catching
7. **Loading States** - Visual feedback for async operations

## Quick Start

### 1. Add ToastContainer to Your App

```tsx
import ToastContainer from './components/ToastContainer';

function App() {
  return (
    <div>
      {/* Your app content */}
      <ToastContainer />
    </div>
  );
}
```

### 2. Handle Async Operations

```tsx
import { handleAsyncErrorWithToast, showSuccess, showError } from './utils/errorHandler';

const MyComponent = () => {
  const loadData = async () => {
    const result = await handleAsyncErrorWithToast(
      async () => {
        // Your async operation here
        return await someAPI();
      },
      'Data loaded successfully!' // Success message
    );

    if (result) {
      // Handle success
      console.log('Data:', result);
    }
  };

  return (
    <button onClick={loadData}>
      Load Data
    </button>
  );
};
```

### 3. Show Notifications

```tsx
import { showSuccess, showInfo, showWarning, showError } from './utils/errorHandler';

// Success notification
showSuccess('Operation completed successfully!');

// Info notification
showInfo('Please wait while we process your request...');

// Warning notification
showWarning('You have unsaved changes');

// Error notification
showError('Failed to save your changes');
```

## API Reference

### Error Store

```typescript
import { useErrorStore } from './utils/errorHandler';

const {
  errors,      // Array of AppError
  toasts,      // Array of ToastNotification
  isLoading,   // Boolean loading state
  addError,    // (error) => string (returns error ID)
  removeError, // (id) => void
  showToast,   // (toast) => string (returns toast ID)
  removeToast, // (id) => void
  setLoading   // (loading) => void
} = useErrorStore();
```

### Error Types

```typescript
interface AppError {
  id: string;
  severity: 'ERROR' | 'WARNING' | 'INFO' | 'SUCCESS';
  title: string;
  message: string;
  details?: string;
  code?: string | number;
  timestamp: number;
  context?: Record<string, any>;
  retryable?: boolean;
  dismissible?: boolean;
}

interface ToastNotification {
  id: string;
  severity: 'ERROR' | 'WARNING' | 'INFO' | 'SUCCESS';
  title: string;
  message: string;
  duration?: number; // milliseconds, 0 = manual dismiss only
  timestamp: number;
  actions?: Array<{
    label: string;
    action: () => void;
  }>;
}
```

### Error Formatter

```typescript
import { ErrorFormatter } from './utils/errorHandler';

// Format an error
const formatted = ErrorFormatter.format(error);
// Returns: { title, message, severity, code }

// Convenience methods
const success = ErrorFormatter.formatSuccess('Saved!');
const info = ErrorFormatter.formatInfo('Processing...');
const warning = ErrorFormatter.formatWarning('Check your input');
const error = ErrorFormatter.formatError('Something went wrong');
```

### IPC Error Handler

```typescript
import { IPCErrorHandler } from './utils/errorHandler';

// Handle async operation with error management
const result = await IPCErrorHandler.handle(
  () => someAsyncOperation(),
  'context-string', // Optional context for tracking
  'Custom error message' // Optional custom message
);
// Returns: result or null if error

// Handle with success toast
const result = await IPCErrorHandler.handleWithToast(
  () => someAsyncOperation(),
  'Operation completed!' // Success message
);
// Shows success toast on completion
```

### Repository Error Handler

```typescript
import { RepositoryErrorHandler } from './utils/errorHandler';

// Specialized error handlers for Git operations
RepositoryErrorHandler.handleRepoNotFound(path);
RepositoryErrorHandler.handleBranchNotFound(branch);
RepositoryErrorHandler.handleLoadError(path, error);
RepositoryErrorHandler.handleAccessDenied(path);
```

### Convenience Functions

```typescript
import {
  showSuccess,
  showInfo,
  showWarning,
  showError,
  handleAsyncError,
  handleAsyncErrorWithToast
} from './utils/errorHandler';

// Quick notifications
showSuccess('Done!');
showInfo('Loading...');
showWarning('Check this');
showError('Failed!');

// Quick error handling
const result = await handleAsyncError(
  () => operation(),
  'context'
);

const result = await handleAsyncErrorWithToast(
  () => operation(),
  'Success!'
);
```

## Error Boundary

Wrap components that might error:

```tsx
import ErrorBoundary from './components/ErrorBoundary';

<ErrorBoundary
  onError={(error, errorInfo) => {
    // Log to analytics, etc.
    console.error('Component error:', error, errorInfo);
  }}
>
  <MyComponentThatMightError />
</ErrorBoundary>
```

Custom fallback UI:

```tsx
<ErrorBoundary
  fallback={
    <div>Custom error UI</div>
  }
>
  <MyComponent />
</ErrorBoundary>
```

## Loading States

### Inline Spinner

```tsx
import LoadingSpinner from './components/LoadingSpinner';

<LoadingSpinner size="md" color="accent" text="Loading..." />
```

### Button Spinner

```tsx
import { ButtonSpinner } from './components/LoadingSpinner';

<button disabled>
  <ButtonSpinner />
  Processing...
</button>
```

### Overlay Spinner

```tsx
import { LoadingOverlay } from './components/LoadingSpinner';

<LoadingOverlay isLoading={isLoading} text="Loading data...">
  <div>Your content here</div>
</LoadingOverlay>
```

## Best Practices

### 1. Use Context

Always provide context when handling errors:

```typescript
await handleAsyncError(
  () => loadRepository(path),
  'RepositoryLoad',
  `Failed to load repository: ${path}`
);
```

### 2. Use Domain-Specific Handlers

```typescript
// Good
RepositoryErrorHandler.handleRepoNotFound(path);

// Okay
showError(`Repository not found: ${path}`);
```

### 3. Provide Retry Capability

Mark errors as retryable when appropriate:

```typescript
addError({
  severity: 'ERROR',
  title: 'Network Error',
  message: 'Failed to fetch data',
  retryable: true
});
```

### 4. Use Appropriate Severity Levels

- **ERROR**: Something failed, user needs to take action
- **WARNING**: Potential issue, user should be aware
- **INFO**: Informational message
- **SUCCESS**: Operation completed successfully

### 5. Auto-Dismiss Non-Critical Errors

Errors are automatically dismissed after 10 seconds (except ERROR severity).

### 6. Loading States

Always show loading states for async operations:

```typescript
const [loading, setLoading] = useState(false);

const handleClick = async () => {
  setLoading(true);
  try {
    await someOperation();
  } finally {
    setLoading(false);
  }
};
```

## Integration with IPC Hooks

The error handling system integrates seamlessly with the IPC hooks:

```typescript
// In useIPC.ts, errors are automatically caught and logged
const call = useCallback(async (...args: T): Promise<R> => {
  try {
    const result = await invoke<R>(command, args[0] || {});
    return result;
  } catch (error) {
    console.error(`IPC call failed for ${command}:`, error);
    throw error;
  }
}, [command]);
```

You can wrap IPC calls with error handlers:

```typescript
const openRepo = useOpenRepoDialog();

const handleOpenRepo = async () => {
  const result = await handleAsyncErrorWithToast(
    () => openRepo(),
    'Repository opened successfully!'
  );

  if (result) {
    // Handle success
  }
};
```

## Custom Toast Actions

Add action buttons to toasts:

```tsx
showToast({
  severity: 'WARNING',
  title: 'Unsaved Changes',
  message: 'You have unsaved changes. What would you like to do?',
  actions: [
    {
      label: 'Save',
      action: () => saveChanges()
    },
    {
      label: 'Discard',
      action: () => discardChanges()
    }
  ]
});
```

## Troubleshooting

### Toast Not Showing

1. Ensure `ToastContainer` is rendered in your app
2. Check that z-index is not being overridden
3. Verify the toast is not being immediately dismissed

### Errors Not Being Caught

1. Ensure async operations are properly awaited
2. Check that errors are being thrown (not just logged)
3. Verify ErrorBoundary is wrapping your component

### Loading State Not Updating

1. Ensure `setLoading` is called in try/finally block
2. Check that state updates are triggering re-renders

## Examples

See the following files for complete examples:
- `components/ToastContainer.tsx` - Toast display component
- `components/ErrorBoundary.tsx` - Error boundary implementation
- `components/LoadingSpinner.tsx` - Loading state components
- `utils/errorHandler.ts` - Core error handling utilities
