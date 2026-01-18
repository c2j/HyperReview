# Feature Specification: Merge New Frontend with Local/Remote Review Mode Separation

**Feature Branch**: `001-merge-new-frontend`
**Created**: 2025-01-18
**Status**: Draft
**Input**: User description: "开启009号新需求项。当前产品是一个基于rust的tauri框架开发的Desktop桌面应用程序，前台使用react框架开发，代码在frontend目录下；后台是tauri 1.8框架开发，代码在src-tauri下。当前已经实现了一部分的前后台功能，能完成本地（Local）的代码评审，可通过npm run tauri dev 方式来启动桌面应用。另外基于用户新诉求重新开发了前端，将Local评审和Remote评审在界面上进行了分离，代码放在tobemerged/HyperReview_Frontend 目录下。现在需要将 tobemerged/HyperReview_Frontend 下的功能代码合并到 当前项目的 frontend 下， 并对接已实现的tauri后台。如果是需要新开发的后台功能，可暂不实现"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Switch Between Local and Remote Review Modes (Priority: P1)

Users can toggle between Local review mode (for reviewing local git repository changes) and Remote review mode (for reviewing changes from remote platforms like Gerrit, CodeArts, or GitLab) within the application. The interface adapts to show appropriate tools, task trees, and panels for each mode.

**Why this priority**: This is the core feature that provides the value of the new frontend - clear separation and improved user experience for different review workflows. Without this, users cannot benefit from the redesigned interface.

**Independent Test**: Can be fully tested by launching the application, toggling between Local and Remote modes, and verifying that the UI changes correctly and appropriate components are displayed for each mode without requiring actual repository connections or backend API calls.

**Acceptance Scenarios**:

1. **Given** application is launched, **When** user clicks the mode toggle button (or menu option), **Then** application switches between Local and Remote review modes
2. **Given** application is in Local mode, **When** user switches to Remote mode, **Then** Local-specific components (LocalToolBar, LocalTaskTree, LocalRightPanel) are replaced with Remote-specific components (RemoteToolBar, RemoteTaskTree, RemoteRightPanel)
3. **Given** application is in Remote mode, **When** user switches to Local mode, **Then** Remote-specific components are replaced with Local-specific components and the interface shows the local repository context
4. **Given** user is in either mode, **When** application is restarted, **Then** the last selected mode is preserved and restored
5. **Given** no repository is loaded, **When** user switches modes, **Then** WelcomeView is displayed with appropriate options for the selected mode

---

### User Story 2 - Review Local Repository Changes (Priority: P1)

Users can open a local git repository and review code changes using the dedicated Local review interface. The interface provides task management, diff viewing, comment creation, and analysis tools optimized for local code review workflows.

**Why this priority**: This is the primary use case for the existing application and must continue to work seamlessly after the merge. Users rely on Local review for their daily code review workflow.

**Independent Test**: Can be fully tested by opening a local git repository, navigating through the Local review interface, viewing diffs, adding comments, and using the available tools without requiring any remote system connectivity or backend changes.

**Acceptance Scenarios**:

1. **Given** application is in Local mode, **When** user opens a local git repository using OpenRepoModal, **Then** repository loads and the Local review interface displays available branches and tasks
2. **Given** a local repository is open in Local mode, **When** user selects a task or branch, **Then** LocalTaskTree shows relevant files and DiffView displays changes
3. **Given** DiffView is displayed, **When** user adds comments to specific lines or files, **Then** comments are saved and appear in the LocalRightPanel
4. **Given** user is reviewing local changes, **When** user uses analysis tools (blame, checklist, heatmap), **Then** tools work using existing Tauri backend commands
5. **Given** user has completed reviewing a local task, **When** user submits their review, **Then** review status is updated and available actions are displayed

---

### User Story 3 - Access Remote Review Interface (Priority: P2)

Users can access the Remote review interface to prepare for reviewing changes from remote platforms. The interface provides placeholders for remote-specific workflows and integrates with existing Gerrit configuration where backend support exists.

**Why this priority**: While the new frontend provides Remote review components, full remote review functionality requires backend features that may not yet be implemented. This story ensures the UI is accessible and properly integrated for future use.

**Independent Test**: Can be tested by switching to Remote mode, viewing the Remote review interface, and verifying that existing Gerrit-related features (like Gerrit configuration dialogs) work as expected, without requiring actual remote API connections.

**Acceptance Scenarios**:

1. **Given** application is switched to Remote mode, **When** user views the Remote review interface, **Then** Remote-specific components (RemoteToolBar, RemoteTaskTree, RemoteRightPanel) are displayed
2. **Given** application is in Remote mode, **When** user accesses Gerrit server configuration, **Then** existing GerritServerModal and GerritImportModal functionality works as before
3. **Given** application is in Remote mode, **When** backend features are not yet implemented, **Then** appropriate placeholder UI or error messages guide users about available functionality
4. **Given** user is in Remote mode, **When** they attempt to import changes from a remote system, **Then** interface correctly calls existing Tauri backend commands (if implemented) or shows a clear message that the feature is not yet available

---

### User Story 4 - Access Enhanced Settings and Configuration (Priority: P3)

Users can access application settings with new features like font size, ligatures, and Vim mode configuration that were added in the new frontend. The SettingsModal integrates these new options with existing settings.

**Why this priority**: These are nice-to-have enhancements that improve the user experience but are not critical for core functionality. They can be implemented after the basic merge is complete.

**Independent Test**: Can be tested by opening the SettingsModal, toggling new settings (font size, ligatures, Vim mode), and verifying that the changes take effect in the UI without requiring backend changes.

**Acceptance Scenarios**:

1. **Given** SettingsModal is open, **When** user adjusts the editor font size slider, **Then** font size in the DiffView and other text areas updates immediately
2. **Given** SettingsModal is open, **When** user toggles font ligatures on/off, **Then** ligature rendering in code displays updates accordingly
3. **Given** SettingsModal is open, **When** user enables Vim mode, **Then** the cursor changes to block style and appropriate Vim-like behaviors are available
4. **Given** settings have been modified, **When** application is restarted, **Then** last configured settings are restored

---

### Edge Cases

- What happens when user switches modes while a review is in progress in the current mode?
- What happens when user attempts to access Remote review features that require unimplemented backend commands?
- How does the application handle corrupted or invalid local repository paths when switching modes?
- What happens when application loses connection to the Tauri backend during mode switching?
- How does the application behave when user has unsaved comments or changes and tries to switch modes or close the application?
- What happens when user switches languages (internationalization) while in different modes?
- How does the application handle large repositories with many files when switching between modes?
- What happens when backend returns errors for commands that work in one mode but not another?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The application MUST provide a mode toggle or menu option to switch between Local and Remote review modes
- **FR-002**: The application MUST display different sets of UI components for Local mode (LocalToolBar, LocalTaskTree, LocalRightPanel) and Remote mode (RemoteToolBar, RemoteTaskTree, RemoteRightPanel)
- **FR-003**: The application MUST preserve the currently selected mode and restore it on application restart
- **FR-004**: The application MUST integrate new frontend settings (font size, ligatures, Vim mode) with the existing SettingsModal
- **FR-005**: The application MUST maintain compatibility with all existing Tauri backend commands used by the existing frontend
- **FR-006**: The application MUST provide appropriate UI feedback or error messages when attempting to use Remote review features that require unimplemented backend commands
- **FR-007**: The application MUST support opening and reviewing local git repositories in Local mode using existing Tauri backend commands
- **FR-008**: The application MUST support creating, editing, and deleting comments in Local mode using existing Tauri backend commands
- **FR-009**: The application MUST support viewing diffs and using analysis tools (blame, checklist, heatmap) in Local mode using existing Tauri backend commands
- **FR-010**: The application MUST integrate existing Gerrit configuration and import dialogs (GerritServerModal, GerritImportModal) with the Remote review interface
- **FR-011**: The application MUST maintain all existing internationalization (i18n) support in the merged frontend
- **FR-012**: The application MUST preserve all existing functionality from the current frontend unless explicitly marked as deprecated
- **FR-013**: The application MUST support all existing modals (OpenRepoModal, NewTaskModal, SettingsModal, SubmitReviewModal, TagManagerModal, SyncStatusModal, BranchCompareModal) in both Local and Remote modes
- **FR-014**: The application MUST handle errors gracefully when switching modes or accessing features without crashing or losing user data
- **FR-015**: The application MUST support resizing left and right panels independently in both Local and Remote modes

### Key Entities *(include if feature involves data)*

- **Review Mode**: Represents the current review context (Local or Remote), determining which UI components and workflows are active
- **Local Review Context**: Contains information about the currently open local repository, selected branch, and active review tasks
- **Remote Review Context**: Contains information about remote platform connections (Gerrit, CodeArts, GitLab), imported changes, and remote-specific tasks
- **User Settings**: Stores user preferences including font size, ligature settings, Vim mode, and other display options
- **Review Session**: Tracks the current review progress, comments, and state within a specific mode

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can successfully switch between Local and Remote review modes within 1 second of clicking the mode toggle
- **SC-002**: All existing Local review functionality (open repo, view diff, add comments, use analysis tools) continues to work without regressions after the merge
- **SC-003**: The merged application can be launched successfully with `npm run tauri dev` without build errors or runtime crashes
- **SC-004**: At least 90% of existing test cases for the current frontend pass after the merge (if tests exist)
- **SC-005**: The application handles mode transitions without losing unsaved comments or review progress
- **SC-006**: Users can access and configure new settings (font size, ligatures, Vim mode) and see changes take effect immediately in the UI
- **SC-007**: The code merge maintains a reasonable file size and structure (no duplicate components that increase build size by more than 20% compared to the larger of the two codebases)
- **SC-008**: The merged frontend integrates with at least 95% of existing Tauri backend commands used in the current application
- **SC-009**: Users report improved satisfaction with the Local/Remote mode separation in user feedback (measured by at least 80% positive response in initial user testing)
- **SC-010**: The application handles errors gracefully (no crashes) when attempting to use Remote features with unimplemented backend, showing clear user guidance

## Assumptions

- The existing Tauri backend in `src-tauri` has sufficient commands to support Local review functionality
- The new frontend in `tobemerged/HyperReview_Frontend` uses the same or compatible TypeScript/React framework versions as the existing frontend
- The new frontend's API client is compatible with the existing Tauri IPC interface
- Internationalization (i18n) support exists in both frontends and can be merged without conflicts
- Users are familiar with the existing application interface and will understand the new mode separation
- Build tools and dependencies (npm, Vite, Tauri) are already set up and working in the current project
- No breaking changes are required to the Tauri backend to support the merged frontend
- The project's git repository has sufficient history to facilitate the merge and resolve conflicts
- Team members have access to both codebases and understand architectural decisions

## Non-Functional Requirements

- **Performance**: Mode switching and UI updates must complete within 1 second on typical hardware
- **Maintainability**: The merged codebase must be well-organized with clear separation between Local and Remote components
- **Compatibility**: The merged frontend must maintain compatibility with the existing Tauri 1.8 backend
- **Usability**: The mode separation must be intuitive and discoverable for existing users
- **Stability**: No crashes or data loss when switching modes or accessing features
- **Extensibility**: The architecture must support adding new Remote review platforms (CodeArts, GitLab) in the future

## Dependencies

- Existing Tauri backend in `src-tauri` must remain functional
- Current frontend dependencies in `frontend/package.json` must be compatible with the new frontend
- Existing internationalization resources must be accessible
- Build configuration (vite.config.ts) must support the merged codebase

## Out of Scope

- Implementation of new Tauri backend commands required for full Remote review functionality (e.g., Gerrit API integration, CodeArts API integration)
- Rewriting or refactoring the existing Tauri backend unless necessary for compatibility
- Migrating user data or settings from the old to the new format (unless required by the merge)
- Performance optimization beyond ensuring acceptable response times
- Extensive documentation or user guide updates (only what's necessary to explain mode separation)
- Automated testing infrastructure (existing tests must pass, but new tests are not required as part of this feature)
