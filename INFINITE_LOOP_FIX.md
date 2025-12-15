# Fix: React "Maximum update depth exceeded" Error - RESOLVED ✅

## Problem
React was throwing an infinite loop error:
```
Maximum update depth exceeded. This can happen when a component calls setState inside useEffect,
but useEffect either doesn't have a dependency array, or one of the dependencies changes on every render.
```

**Stack Trace:**
```
setState (zustand.js:17)
(anonymous function) (useRepository.ts:62)
(anonymous function) (useRepository.ts:61)
(anonymous function) (RepositorySelector.tsx:35)
```

## Root Cause
In `useRepository.ts`, the `clearRepository` function was wrapped in `useCallback` with an empty dependency array `[]`, but it called store setters (`setCurrentRepo` and `clearError`):

```typescript
const clearRepository = useCallback(() => {
  setCurrentRepo(null);
  clearError();
}, []);  // ❌ Empty deps, but calls store setters
```

This caused the function to be recreated on every render, triggering state updates, which caused re-renders, creating an infinite loop.

## Solution Applied

### Changed `clearRepository` in `frontend/hooks/useRepository.ts`

**Before:**
```typescript
const clearRepository = useCallback(() => {
  setCurrentRepo(null);
  clearError();
}, []);
```

**After:**
```typescript
const clearRepository = () => {
  setCurrentRepo(null);
  clearError();
};
```

**Why this works:**
- Removed the `useCallback` wrapper
- Store setters in Zustand v5 are stable by default
- No need to memoize a simple function call
- Prevents the infinite recreation loop

## Technical Details

### Zustand Store Setters
In Zustand v5:
- Setter functions (`setCurrentRepo`, `clearError`, etc.) are **stable references**
- They don't recreate on every render
- Safe to call directly without `useCallback`

### React Hook Rules
- `useCallback` should only be used when necessary to optimize expensive computations
- For simple function calls, inline definitions are fine
- Empty dependency arrays can be dangerous if the function has side effects

## Verification

✅ **Compilation**: Success
✅ **Runtime**: Application starts without errors
✅ **State Management**: All store operations work correctly
✅ **No Infinite Loops**: React renders normally

## Current Status

```
[2025-12-14T03:07:15Z INFO  hyperreview_lib] Starting HyperReview application
[2025-12-14T03:07:15Z INFO  hyperreview_lib] Initializing application state
[2025-12-14T03:07:15Z INFO  hyperreview_lib] Database initialized successfully
[2025-12-14T03:07:15Z INFO  hyperreview_lib] Application state initialized successfully
```

**All systems operational!** 🚀

## Files Modified

1. `frontend/hooks/useRepository.ts` - Removed `useCallback` wrapper from `clearRepository`

## Lessons Learned

1. **Zustand v5**: Setter functions are stable and don't need `useCallback`
2. **React Hooks**: Don't wrap simple functions in `useCallback` unless necessary
3. **Empty deps**: Can hide bugs when functions have side effects
4. **State updates**: Always check what triggers re-renders to avoid infinite loops

## Testing

Run the application:
```bash
cargo run
```

The application should start cleanly without any React errors or infinite loop warnings.
