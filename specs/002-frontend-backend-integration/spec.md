# Feature Specification: Frontend-Backend Integration

**Version**: 1.0
**Created**: 2025-12-14
**Feature ID**: 002
**Branch**: 002-frontend-backend-integration

## Overview

The HyperReview application currently has a complete React frontend interface and a fully implemented Tauri Rust backend with IPC APIs. However, the frontend is currently using mock data for all API calls. This feature focuses on integrating the React frontend with the Tauri backend to replace all mock implementations with actual backend communication.

## Problem Statement

Users currently see a functional-looking interface, but all data is placeholder/mock, making the application non-functional for actual code review workflows. The disconnect between frontend and backend prevents users from:
- Opening and browsing real repositories
- Viewing actual code diffs
- Adding and managing real review comments
- Using task management features
- Generating insights from real code

## User Scenarios & Testing

### Primary Scenarios

**Scenario 1: Repository Integration**
- Given I have a React interface for repository selection
- When I select a Git repository through the file picker
- Then I should see real repository metadata (branches, recent commits, current branch) loaded from the backend
- And the repository should be stored in the SQLite database for quick access

**Scenario 2: Code Review Workflow**
- Given I have opened a repository and am viewing a file
- When I request to see the diff between commits
- Then I should see the actual unified diff with line-by-line changes from git2-rs
- And static analysis results should be overlaid on the diff
- And I should be able to scroll through large diffs smoothly

**Scenario 3: Comment Management**
- Given I am viewing a file diff
- When I add a comment to a specific line
- Then the comment should be immediately stored in the SQLite database via backend
- And the comment should persist across sessions
- And I should see real-time updates if comments are added by others

**Scenario 4: Task and Analytics**
- Given I am in a review session
- When I view review statistics or task lists
- Then I should see real data aggregated from actual repository state
- And quality gates should reflect actual CI/CD integration
- And heatmaps should show real file impact scores

### Testing Scenarios

**T1: Repository Opening**
- User selects a Git repository → Backend loads metadata → Frontend displays real branch list and repository info
- Verify: No mock data visible, all information matches actual git repository state

**T2: Diff Viewing**
- User opens any file → Backend computes actual diff → Frontend displays real changes
- Verify: Diff lines match `git diff` output, performance is < 200ms for typical files

**T3: Comment Persistence**
- User adds comment → Backend stores in SQLite → Comment persists after app restart
- Verify: Comments appear in database, are visible on reload, and link to correct file/line

**T4: Multi-Repository Support**
- User opens multiple repositories → Each stored separately → Switch between them seamlessly
- Verify: Repository metadata doesn't mix, each has independent state

## Functional Requirements

### FR1: Tauri IPC Integration
- The React frontend must replace all mock API calls with Tauri `invoke()` calls
- All 21 backend IPC commands must be integrated: `open_repo_dialog`, `get_recent_repos`, `get_branches`, `load_repo`, `get_file_diff`, `add_comment`, `get_tasks`, `get_review_stats`, `get_quality_gates`, `get_review_templates`, `create_template`, `get_heatmap`, `get_checklist`, `get_blame`, `analyze_complexity`, `scan_security`, `submit_review`, `sync_repo`, `search`, `get_commands`, `get_tags`, `create_tag`

### FR2: Data Model Alignment
- Frontend TypeScript interfaces must match backend Rust data models exactly
- All data structures (Repo, Branch, DiffLine, Comment, Task, ReviewStats, HeatmapItem, etc.) must have consistent field names and types between frontend and backend

### FR3: Error Handling
- Frontend must handle all error cases returned from backend (Result<T, String>)
- Display user-friendly error messages for: invalid repository paths, missing files, network errors, permission issues
- Provide clear feedback when repository access is denied
- Provide recovery actions (retry, choose different repo, etc.)

### FR4: State Management
- Repository state must be managed centrally (current repository, active branches, loaded files)
- Frontend state should update reactively when backend data changes
- Implement loading states for all async operations

### FR5: Performance Requirements
- Repository opening must complete within 2 seconds
- Diff viewing must be interactive (60fps scrolling) using virtual scrolling for large files
- Comment operations must feel instant (< 200ms perceived latency)
- Implement lazy loading for repositories with >10k files
- Maintain responsiveness regardless of repository size

### FR6: Local Storage Integration
- All user data (recent repos, comments, tags, templates) must persist in SQLite via backend
- Implement data migration strategy for schema changes
- Cache frequently accessed data for performance

### FR7: Security Integration
- Input validation must happen on both frontend (UX) and backend (security)
- File paths must be validated before backend processing
- Prevent path traversal and injection attacks through frontend input sanitization

## Success Criteria

**SC1: Zero Mock Data**
- All UI components display real data from backend
- No placeholder text like "Lorem ipsum" or mock API responses
- All counters, lists, and statistics reflect actual repository state

**SC2: Complete Workflow Functionality**
- Users can complete full code review workflow: open repo → view diffs → add comments → manage tasks → submit reviews
- Each step integrates with real backend services
- End-to-end workflow takes less than 5 minutes for a typical PR

**SC3: Data Persistence**
- All user interactions persist across app restarts
- Comments, tags, tasks, and templates stored in SQLite
- Recent repositories list maintained with actual metadata

**SC4: Performance SLA**
- 95% of API calls complete within 200ms (as measured by backend performance monitoring)
- Repository opening: < 2 seconds
- File diff rendering: < 1 second for files under 1000 lines
- Large file diffs (10k+ lines) remain interactive

**SC5: Error Recovery**
- Users receive clear error messages for all failure scenarios
- Graceful degradation when backend services unavailable
- Automatic retry for transient failures

**SC6: Cross-Platform Consistency**
- Integration works identically on Windows, macOS, and Linux
- File system operations use platform-appropriate paths
- Database schema consistent across platforms

## Key Entities

### Repository
- Path (string): Absolute file system path to git repository
- Current Branch (string): Active branch name
- Metadata (object): Head commit, remote URL, last opened timestamp
- Status (enum): Active, Inactive, Error

### Diff
- File Path (string): Relative path within repository
- Lines (array): Each line with old_line_number, new_line_number, content, line_type
- Analysis (array): Static analysis findings per line
- Performance Metrics: Generation time, cache hits

### Comment
- ID (string): Unique identifier
- File Path (string): Target file
- Line Number (integer): Comment location
- Content (string): Comment text
- Author (string): User identifier from local configuration
- Timestamps (object): Created at, updated at
- Status (enum): Draft, Submitted, Rejected
- Tags (array): Associated tags

### Task
- ID (string): Unique identifier
- Title (string): Task description
- Status (enum): Active, Pending, Completed, Blocked
- Metadata (object): Priority, assignee, due date, custom fields

### Review Statistics
- Completion Percentage (number): 0-100%
- Files Reviewed (number): Count of reviewed files
- Pending Files (number): Count of unreviewed files
- Comment Count (number): Total comments
- Performance Metrics (object): Files per hour, estimated time remaining

## Assumptions

**A1: Backend Stability**
- The Tauri Rust backend is complete and stable with all IPC commands implemented
- Backend can handle concurrent requests from frontend
- SQLite database schema is finalized

**A2: Frontend Architecture**
- React frontend uses modern hooks and state management
- TypeScript is configured with strict mode
- Build system supports Tauri integration

**A3: File System Access**
- User has read access to repositories they want to review
- Git repositories are valid (not corrupted)
- File paths use standard OS conventions

**A4: Data Migration**
- No existing user data to migrate (new installation)
- Database will be initialized on first backend start
- Default templates and configurations provided by backend

**A5: User Identity**
- Lightweight user identification using local configuration files
- Each user has a simple profile (name, email) stored locally
- No complex authentication system required

**A6: Repository Size Handling**
- Support repositories of all sizes using on-demand loading
- Implement virtual scrolling for large diffs
- Use lazy loading for file trees and commit history
- Progressive enhancement based on repository characteristics

**A7: Repository Permissions**
- Unified read/write access model
- Assume user has full permissions to selected repositories
- Display clear error messages if access is denied
- No granular permission system required

## Dependencies

**D1: Backend API Documentation**
- All 21 IPC commands documented in `src-tauri/docs/api.md`
- Data models defined and stable
- Error codes and messages standardized

**D2: Tauri Framework**
- Tauri v2 configured and functional
- IPC bridge between React and Rust operational
- Security allowlist properly configured

**D3: Build System**
- Vite bundler configured for Tauri
- TypeScript compilation working
- Cross-platform builds functional

## Non-Goals

**NG1: Backend Feature Development**
- No new backend features will be implemented
- Focus is on integration, not feature expansion
- Backend API is considered complete

**NG2: UI/UX Redesign**
- Existing React interface design is maintained
- No major visual changes to components
- Focus on data integration, not visual improvements

**NG3: External System Integration**
- External systems (GitLab, Gerrit) integration not required for this feature
- Focus on local repository functionality
- External features remain backend-only until later

**NG4: Performance Optimization**
- No backend performance optimization in scope
- Assume backend meets performance requirements
- Frontend optimization only if needed for integration

## Acceptance Criteria

**AC1: Repository Management**
- [ ] `open_repo_dialog` returns real repository path
- [ ] `get_recent_repos` shows actual opened repositories from SQLite
- [ ] `get_branches` returns real branch list with commit metadata
- [ ] `load_repo` stores repository in SQLite with metadata

**AC2: Code Review**
- [ ] `get_file_diff` returns actual unified diff from git2
- [ ] Diff rendering matches `git diff` output exactly
- [ ] Static analysis overlays real findings on diff
- [ ] Large files scroll smoothly without lag

**AC3: Comment System**
- [ ] `add_comment` stores comment in SQLite
- [ ] Comments persist across app restarts
- [ ] Comment threads display correctly
- [ ] Comment editing and deletion works

**AC4: Task Management**
- [ ] `get_tasks` returns tasks from SQLite
- [ ] Task CRUD operations work via backend
- [ ] Task statistics calculated from real data
- [ ] Review progress tracking functional

**AC5: Analysis & Insights**
- [ ] `get_heatmap` returns real file impact scores
- [ ] `get_checklist` generates based on actual file types
- [ ] `get_blame` shows real git blame information
- [ ] `analyze_complexity` calculates real metrics

**AC6: Search & Configuration**
- [ ] `search` returns real results via ripgrep
- [ ] `get_tags` shows tags from SQLite
- [ ] `create_tag` persists tags to database
- [ ] Command palette shows actual available commands

**AC7: External Integration**
- [ ] `submit_review` integrates with GitLab/Gerrit (backend ready, frontend calls it)
- [ ] `sync_repo` performs actual git operations
- [ ] Error handling for network failures

## Technical Approach

**Frontend Changes Required:**
1. Replace all mock API service functions with Tauri `invoke()` calls
2. Update TypeScript interfaces to match backend data models
3. Add error handling for all async operations
4. Implement loading states and user feedback
5. Add input validation before backend calls

**Backend Considerations:**
1. All 21 IPC commands already implemented in Rust
2. SQLite schema finalized with all necessary tables
3. Performance monitoring utilities available
4. Security audit framework in place

**Integration Pattern:**
```
React Component → API Service (TypeScript) → Tauri invoke() → Rust Backend → SQLite
                                      ↓
                               Error Handling & State Update
```

## Risks & Mitigations

**R1: Data Model Mismatch**
- Risk: Frontend TypeScript types don't match backend Rust models
- Mitigation: Use shared type definitions or generate types from Rust
- Mitigation: Extensive testing of data serialization/deserialization

**R2: Performance Degradation**
- Risk: Real API calls slower than mock data
- Mitigation: Backend implements caching (already done)
- Mitigation: Frontend shows loading states appropriately
- Mitigation: Optimize slow operations with backend performance monitoring

**R3: Error Complexity**
- Risk: Backend errors not handled gracefully by frontend
- Mitigation: Comprehensive error handling in frontend
- Mitigation: User-friendly error messages
- Mitigation: Fallback behaviors for common errors

**R4: Platform Differences**
- Risk: File system paths differ across OS
- Mitigation: Backend handles OS-specific paths
- Mitigation: Test on all target platforms

**R5: Large File Handling**
- Risk: Very large diffs cause UI performance issues
- Mitigation: Implement virtual scrolling if needed
- Mitigation: Backend already supports diff caching
- Mitigation: Progressive loading for large files

## Timeline & Milestones

**Phase 1: Core Integration (Week 1)**
- Replace repository management mocks with real API calls
- Implement error handling for file operations
- Test repository opening and branch listing

**Phase 2: Review Workflow (Week 2)**
- Integrate diff viewing with real backend
- Connect comment system to SQLite
- Test end-to-end review workflow

**Phase 3: Advanced Features (Week 3)**
- Integrate task management
- Connect analysis and insights
- Test heatmap, checklist, and search

**Phase 4: Polish & Testing (Week 4)**
- Performance optimization
- Error handling improvements
- Cross-platform testing
- Documentation updates

## Success Metrics

**Quantitative:**
- 100% of mock API calls replaced with real backend integration
- 95% of operations complete within SLA (< 200ms for typical operations)
- Zero data loss across app restarts
- Support repositories up to 100k files
- Comment operations feel instant (< 200ms perceived latency)

**Qualitative:**
- Users can complete real code reviews without workarounds
- No confusion from seeing mock/placeholder data
- Smooth, responsive UI during all operations
- Clear error messages guide users to resolution
- Application feels production-ready

## Clarifications

### Session 2025-12-14

- Q: 用户认证需求 → A: 轻量级用户识别（基于本地配置文件的用户标识）
- Q: 大仓库处理策略 → A: 按需加载（所有大小仓库均支持，使用虚拟滚动和懒加载优化）
- Q: 仓库权限处理策略 → A: 统一读写访问（假设用户有完整权限，仅在失败时提示）

## References

- Backend API Documentation: `src-tauri/docs/api.md`
- Backend Implementation: `src-tauri/src/` (all modules)
- Database Schema: `src-tauri/src/storage/sqlite.rs`
- Tauri Configuration: `src-tauri/tauri.conf.json`
- Frontend Code: React components (location TBD)

---

**Next Steps:**
1. Review and approve this specification
2. Proceed to `/speckit.clarify` if questions remain
3. Proceed to `/speckit.plan` for implementation planning
