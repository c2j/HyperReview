# Feature Specification: Frontend Integration - Local/Remote Review Separation

**Feature Branch**: `001-frontend-integration`
**Created**: 2025-01-18
**Status**: Draft
**Input**: User description: "开启009号新需求项。当前产品是一个基于rust的tauri框架开发的Desktop桌面应用程序，前台使用react框架开发，代码在frontend目录下；后台是tauri 1.8框架开发，代码在src-tauri下。当前已经实现了一部分的前后台功能，能完成本地（Local）的代码评审，可通过npm run tauri dev 方式来启动桌面应用。另外基于用户新诉求重新开发了前端，将Local评审和Remote评审在界面上进行了分离，代码放在tobemerged/HyperReview_Frontend 目录下。现在需要将 tobemerged/HyperReview_Frontend 下的功能代码合并到 当前项目的 frontend 下， 并对接已实现的tauri后台。如果是需要新开发的后台功能，可暂不实现"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Initial Mode Selection (Priority: P1)

User starts the application and is presented with a welcome screen that allows them to choose between Local code review and Remote (Gerrit) review modes. This provides clear separation of review workflows and helps users understand the available options before starting work.

**Why this priority**: This is the entry point for all user interactions and establishes the fundamental separation between Local and Remote review workflows. Without this clear separation, users would be confused about which mode they're in and what actions are available.

**Independent Test**: User can open the application, see a welcome screen with two distinct options (Local Review and Remote Review), select one mode, and proceed to the corresponding review interface without errors or confusion.

**Acceptance Scenarios**:

1. **Given** the application is launched, **When** the user has not configured Gerrit, **Then** the welcome screen displays a priority setup banner guiding them to configure Gerrit first
2. **Given** the application is launched, **When** the user clicks "Open Local Repo", **Then** the repository selection dialog opens and the application enters Local review mode
3. **Given** the application is launched, **When** the user clicks "Import Remote", **Then** the Gerrit configuration/import flow is initiated
4. **Given** the application is launched with a previously configured Gerrit server, **When** the welcome screen loads, **Then** the user can click "Import Remote" and see the Gerrit import modal without needing to reconfigure
5. **Given** the user has opened repositories recently, **When** the welcome screen loads, **Then** a history section displays up to 3 recently opened repositories with their last opened time and branch

---

### User Story 2 - Local Code Review Workflow (Priority: P1)

User selects Local review mode and is presented with a dedicated interface for reviewing local code changes. The interface includes a Local-specific toolbar, task tree for local tasks, and right panel optimized for local review features. Users can open repositories, create local tasks, view file diffs, add comments, and mark files as approved/needed changes.

**Why this priority**: Local code review is the primary use case for the application. This feature must work seamlessly to provide value to users reviewing code in their local repositories without requiring network access or remote server connections.

**Independent Test**: User can complete a full local review workflow: open repository → compare branches → select files → add comments → mark files as approved → submit review → complete task.

**Acceptance Scenarios**:

1. **Given** the user is in Local review mode, **When** they click "Open Repository", **Then** the repository selection dialog opens and allows them to select a local Git repository
2. **Given** a repository is loaded in Local mode, **When** the Local Toolbar is displayed, **Then** it shows repository-specific actions (Open Repo, New Task) and does not show Remote/Gerrit-specific actions
3. **Given** the user is in Local mode with a loaded repository, **When** they view the left panel, **Then** they see a Local Task Tree displaying local review tasks
4. **Given** the user is in Local mode, **When** they click "New Task", **Then** the New Task modal opens with options to create or import local tasks
5. **Given** the user is reviewing a file in Local mode, **When** they use the right panel, **Then** it displays Local-specific information (file details, heatmap, review stats) appropriate for local review
6. **Given** the user is in Local mode, **When** the status bar is displayed, **Then** it shows Local-specific status (current repository, branch comparison) without Remote/Gerrit indicators

---

### User Story 3 - Remote Code Review Workflow (Priority: P1)

User selects Remote (Gerrit) review mode and is presented with a dedicated interface for reviewing code changes from a remote Gerrit server. The interface includes a Remote-specific toolbar, task tree showing Gerrit changes, and right panel optimized for Gerrit review features. Users can import changes, review patch sets, add comments, and submit reviews to Gerrit.

**Why this priority**: Remote code review via Gerrit is a critical workflow for teams collaborating on code changes. This feature enables users to review, comment on, and approve/reject changes from their Gerrit server directly within the application.

**Independent Test**: User can complete a full remote review workflow: configure Gerrit → import change → review patches → add comments → submit review to Gerrit → see updated status.

**Acceptance Scenarios**:

1. **Given** the user is in Remote review mode, **When** they have not configured Gerrit, **Then** the Gerrit Server Configuration modal opens automatically
2. **Given** the user is in Remote mode with a configured Gerrit, **When** they click "Import Remote", **Then** the Gerrit Import modal opens allowing them to search and import changes by change number or ID
3. **Given** a Gerrit change is imported in Remote mode, **When** the Remote Toolbar is displayed, **Then** it shows Remote-specific actions (Import from Gerrit, Gerrit Server Settings) and does not show Local-specific repository actions
4. **Given** the user is in Remote mode with imported changes, **When** they view the left panel, **Then** they see a Remote Task Tree displaying Gerrit changes with their status, owner, and review counts
5. **Given** the user is reviewing a Gerrit change in Remote mode, **When** they use the right panel, **Then** it displays Remote-specific information (Gerrit labels, patch sets, messages, file stats) appropriate for Gerrit review
6. **Given** the user is in Remote mode, **When** the status bar is displayed, **Then** it shows Remote-specific status (Gerrit server, change number, patch set) without Local repository indicators
7. **Given** the user is in Remote mode and has reviewed a change, **When** they submit a review, **Then** the review is posted to the Gerrit server and the change status updates accordingly

---

### User Story 4 - Mode Switching (Priority: P2)

User can switch between Local and Remote review modes at any time through the TitleBar. When switching, the interface updates to show mode-specific components (toolbar, task tree, right panel) and the visual theme adapts to indicate the current mode (e.g., different accent colors for Local vs Remote).

**Why this priority**: Mode switching provides flexibility for users who need to work on both local and remote reviews in the same session. Clear visual distinction between modes prevents user confusion about which workflow they're currently in.

**Independent Test**: User can switch from Local to Remote mode (and vice versa), see the interface components update accordingly, and continue working in the new mode without errors or data loss.

**Acceptance Scenarios**:

1. **Given** the user is in Local review mode, **When** they click "Remote" in the TitleBar mode toggle, **Then** the interface switches to Remote mode showing the Remote Toolbar, Remote Task Tree, and Remote Right Panel
2. **Given** the user is in Remote review mode, **When** they click "Local" in the TitleBar mode toggle, **Then** the interface switches to Local mode showing the Local Toolbar, Local Task Tree, and Local Right Panel
3. **Given** the user switches modes, **When** the transition occurs, **Then** the visual theme changes (e.g., accent color from blue for Local to purple for Remote) to clearly indicate the current mode
4. **Given** the user switches modes, **When** the transition occurs, **Then** a notification appears confirming the mode switch (e.g., "Switched to REMOTE mode")
5. **Given** the user has unsaved changes in Local mode, **When** they switch to Remote mode, **Then** they are prompted to save or discard their work before proceeding

---

### User Story 5 - Recent Repository History (Priority: P2)

User can quickly reopen recently used repositories from the welcome screen. The application displays a history of up to 3 recently opened repositories with their last opened time, branch, and mode (Local or Remote). Users can click on a history item to quickly reload that repository in the appropriate mode.

**Why this priority**: Recent history improves user productivity by reducing the number of clicks needed to resume work on frequently accessed repositories. This is a convenience feature that enhances the user experience but is not critical for core functionality.

**Independent Test**: User opens a repository in Local mode, closes the app, reopens it, and sees the repository in the recent history list. Clicking the history item reloads the repository in the correct mode.

**Acceptance Scenarios**:

1. **Given** the user opens a repository in Local mode, **When** they return to the welcome screen, **Then** the repository appears in the history section with the correct path, branch, and last opened time
2. **Given** the user imports a Gerrit change in Remote mode, **When** they return to the welcome screen, **Then** the change appears in the history section marked as Remote
3. **Given** the welcome screen shows more than 3 recent repositories, **When** it displays the history, **Then** only the 3 most recently opened items are shown
4. **Given** the user clicks a repository from history, **When** the item is selected, **Then** the application loads that repository in the correct mode (Local or Remote) and displays the appropriate interface
5. **Given** the user clicks the "Clear History" button, **When** the action completes, **Then** all recent repositories are removed from the history display

---

### Edge Cases

- What happens when the user tries to switch modes while a modal is open?
- What happens when the Gerrit server is unreachable during Remote review?
- What happens when the local repository has been deleted since the last session?
- What happens when there are merge conflicts in the local repository?
- What happens when a Gerrit change is updated while the user is reviewing it?
- What happens when the user has no Git repository loaded and tries to perform Local review actions?
- What happens when the user's network connection is lost during Remote review?
- How does the system handle very large repositories with thousands of files?
- What happens when the user attempts to review a file that has been deleted in one branch?
- What happens when multiple patch sets are available for a Gerrit change?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Application MUST display a welcome screen on startup with two primary options: Local Review and Remote (Gerrit) Review
- **FR-002**: Application MUST support Local review mode with dedicated components: LocalToolBar, LocalTaskTree, and LocalRightPanel
- **FR-003**: Application MUST support Remote review mode with dedicated components: RemoteToolBar, RemoteTaskTree, and RemoteRightPanel
- **FR-004**: Application MUST allow users to switch between Local and Remote modes through the TitleBar with a mode toggle control
- **FR-005**: Application MUST display mode-specific visual themes (e.g., different accent colors) to clearly indicate whether the user is in Local or Remote mode
- **FR-006**: Application MUST preserve the existing Tauri backend IPC integration for all Local review operations (repository loading, file diffs, comments, etc.)
- **FR-007**: Application MUST maintain compatibility with the existing Tauri backend commands when merging the new frontend code
- **FR-008**: Application MUST display a priority setup banner on the welcome screen when Gerrit is not configured
- **FR-009**: Application MUST open the Gerrit Server Configuration modal when users click "Import Remote" without a configured Gerrit server
- **FR-010**: Application MUST display a Gerrit Import modal allowing users to search and import changes by change number or ID when Gerrit is configured
- **FR-011**: Application MUST display recent repository history on the welcome screen (up to 3 items) with path, branch, last opened time, and mode indication
- **FR-012**: Application MUST allow users to clear the recent repository history
- **FR-013**: Application MUST restore the last session (repository path, branches, mode) when the application is reopened if a valid previous session exists
- **FR-014**: Application MUST show notifications when mode switching occurs (e.g., "Switched to LOCAL mode")
- **FR-015**: Application MUST provide a local AI analyzer functionality in the Local mode toolbar
- **FR-016**: Application MUST display Gerrit-specific information in Remote mode (labels, patch sets, messages, file stats)
- **FR-017**: Application MUST support creating local tasks in Local mode with title, type, and file selection
- **FR-018**: Application MUST support importing Gerrit changes in Remote mode and displaying them in the Remote Task Tree
- **FR-019**: Application MUST maintain all existing functionality from the current frontend that is not specific to Local/Remote separation
- **FR-020**: Application MUST ensure that new backend features required for Remote mode (e.g., full Gerrit integration) are NOT implemented as part of this merge (mock data is acceptable)

### Key Entities

- **Review Mode**: Represents the current operational mode of the application (Local or Remote), determining which components and features are available
- **Local Task**: Represents a code review task created for local repository changes, with attributes including title, type, status, files, and review decisions
- **Remote Change (Gerrit Change)**: Represents a code change imported from a remote Gerrit server, with attributes including change number, project, branch, patch sets, files, labels, messages, and review status
- **Repository**: Represents a local Git repository with path, branch, and last opened timestamp for history tracking
- **Gerrit Server Configuration**: Represents the connection settings for a Gerrit server including URL, authentication token, and connection status
- **Review Comment**: Represents a user comment added during code review, with content, line number, file path, and type (concern, question, approval)

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can complete a full Local code review workflow (open repo → compare branches → add comments → submit) within 3 minutes of application launch
- **SC-002**: Users can complete a full Remote (Gerrit) code review workflow (configure Gerrit → import change → review patches → submit) within 5 minutes of application launch
- **SC-003**: Users can switch between Local and Remote modes in under 2 seconds without application errors or data loss
- **SC-004**: Application startup time with the merged frontend is under 5 seconds on modern hardware
- **SC-005**: Visual distinction between Local and Remote modes is clear enough that 95% of users correctly identify the current mode without hesitation in usability testing
- **SC-006**: All existing Local review features (repository operations, file diffs, comments, heatmap, stats) continue to function after the merge with 100% feature parity
- **SC-007**: The merged frontend integrates with the existing Tauri backend IPC layer with 0 broken commands or deprecated function calls
- **SC-008**: Recent repository history displays and functions correctly for 100% of test cases involving opening, closing, and reopening repositories
- **SC-009**: Mode switching notifications appear within 1 second of the user clicking the mode toggle and remain visible for at least 2 seconds
- **SC-010**: The welcome screen priority setup banner for Gerrit configuration appears 100% of the time when no Gerrit server is configured

## Assumptions

- The existing Tauri backend (src-tauri) provides sufficient IPC commands for Local review operations (repository loading, file diffs, comments, tasks, etc.)
- Remote review features that require new backend functionality will use mock data as stated in the requirements (new backend features are out of scope)
- The user has basic familiarity with Git operations and the Gerrit review workflow
- The development team has access to the tobemerged/HyperReview_Frontend source code to review and merge
- React, TypeScript, and Tauri version compatibility between the current frontend and the new frontend can be resolved without major refactoring
- The application will run on desktop platforms (Windows, macOS, Linux) as specified in the existing Tauri setup
- Code review comments and local task data are stored locally by the backend and do not require external databases
- Gerrit server configuration (URL, token) will be stored in localStorage as implemented in the new frontend (this is acceptable for the merge)
- Users will have write access to local repositories to create and switch branches during Local review
- Network connectivity is available for Remote (Gerrit) review operations, but not required for Local review operations
