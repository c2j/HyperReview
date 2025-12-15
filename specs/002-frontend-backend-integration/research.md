# Research: Frontend-Backend Integration

**Date**: 2025-12-14
**Feature**: 002-frontend-backend-integration

## Research Questions & Decisions

### RQ1: Frontend State Management for Tauri IPC

**Question**: What state management approach should be used for React components calling Tauri IPC?

**Decision**: Zustand for lightweight state management
- **Rationale**: Simple, minimal boilerplate, perfect for Tauri apps
- **Alternatives Considered**: Redux Toolkit (too complex), React Context (boilerplate)
- **Best Practice**: Keep state close to IPC calls, avoid prop drilling

### RQ2: TypeScript Type Safety Across IPC Boundary

**Question**: How to ensure TypeScript interfaces match Rust backend models?

**Decision**: Manual alignment with backend documentation
- **Rationale**: Backend is stable, use `src-tauri/docs/api.md` as source of truth
- **Alternatives Considered**: Type generation from Rust (too complex for this integration)
- **Best Practice**: Test serialization/deserialization with real data

### RQ3: Error Handling Pattern

**Question**: What pattern for handling backend errors in React components?

**Decision**: Centralized error handling with Result<T, String>
- **Rationale**: Backend returns `Result<T, String>`, need unified toast system
- **Alternatives Considered**: Per-component error handling (duplication)
- **Best Practice**: Use Tauri invoke with try/catch, display user-friendly messages

### RQ4: Loading State Management

**Question**: How to handle async operations and loading states?

**Decision**: Component-level loading flags with global loading context
- **Rationale**: Some operations (repo opening) block entire app, others (comments) are local
- **Alternatives Considered**: Global loading spinner (too broad)
- **Best Practice**: Granular loading states per operation type

### RQ5: Virtual Scrolling for Large Diffs

**Question**: How to render large diffs (10k+ lines) efficiently?

**Decision**: Implement react-window or react-virtualized
- **Rationale**: Backend already caches diffs, need efficient frontend rendering
- **Alternatives Considered**: Pagination (breaks diff flow), full render (performance)
- **Best Practice**: Virtualize visible rows only, maintain diff context

### RQ6: Data Caching Strategy

**Question**: Should frontend cache backend responses?

**Decision**: Minimal caching, rely on backend LRU cache
- **Rationale**: Backend has sophisticated caching (T014), keep frontend simple
- **Alternatives Considered**: React Query (overkill), Redux cache (complex)
- **Best Practice**: Cache only recently viewed files, invalidate on branch switch

### RQ7: Performance Monitoring Integration

**Question**: How to expose backend performance metrics to frontend?

**Decision**: No frontend metrics needed, backend monitoring sufficient
- **Rationale**: Backend tracks all performance (T090), no user-facing metrics needed
- **Alternatives Considered**: Frontend timing (duplicate effort)
- **Best Practice**: Backend is source of truth for performance

## Key Findings

### Backend Completeness
- ✅ All 21 IPC commands implemented and documented
- ✅ SQLite schema finalized with 8 tables
- ✅ Performance monitoring active (<200ms SLA tracking)
- ✅ Security audit in place (command injection prevention)
- ✅ LRU caching implemented for diffs

### Integration Pattern
```
React Component
  ↓
Custom Hook (useIPC)
  ↓
Tauri invoke()
  ↓
Rust Command
  ↓
SQLite/git2-rs
```

### Critical Success Factors
1. **Type Alignment**: TS interfaces must match Rust models exactly
2. **Error UX**: Clear messages for all failure scenarios
3. **Performance**: Virtual scrolling for large diffs
4. **State Sync**: Reactive updates when backend data changes

## Recommendations

### Immediate Actions
1. Create TypeScript interfaces matching backend models
2. Implement API service layer with Tauri invoke wrappers
3. Add error boundary components for graceful error handling
4. Set up virtual scrolling for diff viewer

### Testing Strategy
1. Unit tests for API service functions
2. Integration tests for critical workflows (open repo → view diff → add comment)
3. E2E tests for complete review process

### Risk Mitigation
1. **Data Model Mismatch**: Extensive testing of serialization
2. **Performance Issues**: Virtual scrolling + backend caching
3. **Error Complexity**: Centralized error handling system

## References

- Tauri IPC Guide: https://tauri.app/v2/guides/building/ipc
- React + Tauri Best Practices: https://tauri.app/v2/guides/building/react
- Backend API: `/Volumes/Raiden_C2J/Projects/Desktop_Projects/CR/HyperReview/src-tauri/docs/api.md`
- Backend Commands: `/Volumes/Raiden_C2J/Projects/Desktop_Projects/CR/HyperReview/src-tauri/src/commands.rs`
