# Quickstart Guide: Merge New Frontend with Local/Remote Review Mode Separation

**Feature**: 009-merge-new-frontend
**Date**: 2025-01-18
**Purpose**: Get started quickly with the merged frontend implementation

## Prerequisites

Before starting, ensure you have:

1. **Development Environment**
   - Node.js 18+ installed
   - Rust 1.75+ installed
   - Tauri CLI installed (`cargo install tauri-cli`)

2. **Access to Codebases**
   - `frontend/` - Existing frontend codebase
   - `tobemerged/HyperReview_Frontend/` - New frontend codebase
   - `src-tauri/` - Existing Tauri backend

3. **Dependencies Installed**
   ```bash
   npm install
   ```

4. **Git Repository Status**
   - On branch `009-merge-new-frontend`
   - Clean working directory (no uncommitted changes)

---

## Quick Start Steps

### Step 1: Review the Plan and Research

Read the implementation plan and research documentation to understand the approach:

```bash
# Read the implementation plan
cat specs/009-merge-new-frontend/plan.md

# Read the research findings
cat specs/009-merge-new-frontend/research.md

# Read the data model
cat specs/009-merge-new-frontend/data-model.md

# Review IPC contracts
cat specs/009-merge-new-frontend/contracts/ipc-interfaces.md
```

**Key Takeaways**:
- We're using incremental migration with feature flags
- Component architecture: Adapter pattern with mode-specific subdirectories
- IPC abstraction layer for graceful degradation
- CSS variables for editor settings (performance optimization)

---

### Step 2: Set Up Development Environment

Start the Tauri development server:

```bash
# Start the application in development mode
npm run tauri dev
```

**Expected Output**:
- Tauri window opens automatically
- Development server runs on `http://localhost:1422`
- Hot Module Replacement (HMR) enabled for frontend changes

**Troubleshooting**:
- If window doesn't open, check Tauri is installed: `cargo tauri --version`
- If port 1422 is in use, modify in `vite.config.ts`

---

### Step 3: Understand the Current Application

Before making changes, explore the existing application:

1. **Navigate Local Review Mode**
   - Open a local git repository
   - Explore the task tree, diff view, and right panel
   - Add a comment to see the comment workflow

2. **Explore Existing Components**
   ```bash
   # List existing frontend components
   ls -la frontend/src/components/

   # List existing stores
   ls -la frontend/src/store/

   # List existing API hooks
   ls -la frontend/src/hooks/
   ```

3. **Understand State Management**
   - Check how state flows through components
   - Note how Zustand stores are used
   - Identify existing context providers

4. **Test Existing Features**
   - Open repository dialog works?
   - Diff viewing works?
   - Comment creation works?
   - Settings modal works?

---

### Step 4: Explore the New Frontend

Before merging, understand what's being added:

1. **Explore New Component Structure**
   ```bash
   # List new frontend components
   ls -la tobemerged/HyperReview_Frontend/components/

   # Note mode-specific components:
   # - LocalToolBar.tsx
   # - RemoteToolBar.tsx
   # - LocalTaskTree.tsx
   # - RemoteTaskTree.tsx
   # - LocalRightPanel.tsx
   # - RemoteRightPanel.tsx
   ```

2. **Review New Features**
   - **Mode Separation**: Notice how Local vs Remote components differ
   - **Enhanced Settings**: Check font size, ligatures, Vim mode in new App.tsx
   - **Dynamic CSS Variables**: See how new frontend applies editor settings

3. **Compare Architectures**
   - **State Management**: New frontend uses component-level `useState`
   - **API Layer**: New frontend uses mock functions with `MOCK` flag
   - **Entry Point**: New frontend uses WelcomeView for initial mode selection

4. **Identify Duplicates**
   ```bash
   # Find duplicate component names
   comm -12 <(ls frontend/src/components/ | sort) <(ls tobemerged/HyperReview_Frontend/components/ | sort)
   ```

---

### Step 5: Prepare for Migration

Set up the necessary directories and files:

```bash
# Create component subdirectories for mode-specific organization
mkdir -p frontend/src/components/local
mkdir -p frontend/src/components/remote
mkdir -p frontend/src/components/shared
mkdir -p frontend/src/components/toolbars
mkdir -p frontend/src/components/task-trees
mkdir -p frontend/src/components/panels

# Create IPC abstraction layer directory
mkdir -p frontend/src/ipc

# Create contexts directory
mkdir -p frontend/src/contexts
```

---

### Step 6: Start with Phase 1 - Core Infrastructure

Follow the implementation plan's Phase 1:

1. **Create Settings Context**
   - Create `frontend/src/contexts/SettingsContext.tsx`
   - Implement Tauri Store integration
   - Add editor settings state (fontSize, ligatures, vimMode)

2. **Create Review Mode Context**
   - Create `frontend/src/contexts/ReviewModeContext.tsx`
   - Implement mode state ('local' | 'remote')
   - Add mode persistence to Tauri Store

3. **Create IPC Client Abstraction**
   - Create `frontend/src/ipc/IPCClient.ts`
   - Implement error handling and graceful degradation
   - Add command availability detection

4. **Update CSS Variables**
   - Add editor settings CSS variables to `frontend/src/index.css`
   - Test that font size, ligatures, cursor style updates work

---

### Step 7: Migrate Components Incrementally

Following the research document's migration checklist:

**Day 1-2: Component Organization**
```bash
# Move mode-specific components from new frontend
cp tobemerged/HyperReview_Frontend/components/LocalToolBar.tsx frontend/src/components/toolbars/
cp tobemerged/HyperReview_Frontend/components/RemoteToolBar.tsx frontend/src/components/toolbars/
cp tobemerged/HyperReview_Frontend/components/LocalTaskTree.tsx frontend/src/components/task-trees/
cp tobemerged/HyperReview_Frontend/components/RemoteTaskTree.tsx frontend/src/components/task-trees/
cp tobemerged/HyperReview_Frontend/components/LocalRightPanel.tsx frontend/src/components/panels/
cp tobemerged/HyperReview_Frontend/components/RemoteRightPanel.tsx frontend/src/components/panels/
```

**Day 3-5: Component Integration**
1. Refactor existing `ToolBar.tsx` to use mode context
2. Refactor existing `TaskTree.tsx` to support mode switching
3. Refactor existing `RightPanel.tsx` to display mode-specific panels
4. Update `App.tsx` to integrate mode toggle UI

---

### Step 8: Test Mode Switching

After integrating mode switching, test thoroughly:

1. **Manual Testing**
   - Start application in Local mode
   - Click mode toggle button
   - Verify Remote mode components appear
   - Click mode toggle again
   - Verify Local mode components reappear
   - Refresh application
   - Verify mode is preserved (not reset to default)

2. **Component Testing**
   ```bash
   # Run existing frontend tests
   npm test
   ```

3. **Integration Testing**
   - Test opening repository in Local mode
   - Test viewing diff in both modes
   - Test adding comments in both modes
   - Test that existing features still work in Local mode

---

### Step 9: Integrate Editor Settings

After mode switching works, add enhanced editor settings:

1. **Update Settings Modal**
   - Add font size slider (12-24px range)
   - Add ligatures toggle
   - Add Vim mode toggle
   - Test live preview (settings apply immediately)

2. **Apply Settings to DiffView**
   - Verify font size changes in DiffView
   - Verify ligatures toggle affects code display
   - Verify Vim mode changes cursor to block style

3. **Test Settings Persistence**
   - Change a setting
   - Close application
   - Reopen application
   - Verify setting is restored

---

### Step 10: API Compatibility Verification

Ensure the merged frontend works with the existing Tauri backend:

1. **Test All Existing Commands**
   - `get_recent_repos`
   - `get_branches`
   - `load_repo`
   - `get_file_diff`
   - `add_comment`
   - `update_comment`
   - `delete_comment`

2. **Test Remote Commands (with Graceful Degradation)**
   - `gerrit_get_instances_simple` (should work)
   - `gerrit_create_instance_simple` (should work)
   - `gerrit_search_changes_simple` (should work)
   - `gerrit_import_change_simple` (should work)

3. **Verify Error Handling**
   - Try to invoke non-existent command
   - Verify graceful error message is shown
   - Verify application doesn't crash

---

### Step 11: Performance Verification

Verify performance meets constitution requirements:

1. **Bundle Size Check**
   ```bash
   # Build application
   npm run tauri build

   # Check Windows bundle size
   ls -lh src-tauri/target/release/bundle/
   # Should be < 15MB
   ```

2. **Command Response Time**
   - Use browser DevTools to measure IPC command times
   - Verify commands complete in <200ms (as per constitution)

3. **UI Performance**
   - Test mode switching completes in <1 second
   - Verify 60fps during interactions
   - Test scrolling performance with large diffs (>5000 lines)

---

### Step 12: Final Testing

Before submitting for review:

1. **Run Full Test Suite**
   ```bash
   # Run frontend tests
   npm test

   # Run backend tests
   cd src-tauri && cargo test
   ```

2. **Manual Regression Testing**
   - [ ] Repository loading works
   - [ ] Branch selection works
   - [ ] Diff viewing works
   - [ ] Comment creation works
   - [ ] Settings changes work
   - [ ] Mode switching works
   - [ ] Local review workflow works
   - [ ] Gerrit integration works
   - [ ] No crashes or errors in console

3. **Cross-Platform Testing** (if possible)
   - Test on Windows
   - Test on macOS
   - Test on Linux

---

## Common Tasks and Commands

### Development

```bash
# Start development server
npm run tauri dev

# Run frontend tests
npm test

# Run tests in watch mode
npm run test:watch

# Type checking
npm run typecheck

# Linting
npm run lint

# Format code
npm run format
```

### Build

```bash
# Build for all platforms
npm run tauri build

# Build for specific platform
npm run tauri build --target windows
npm run tauri build --target macos
npm run tauri build --target linux

# Check bundle size
npm run tauri size
```

### Git

```bash
# Check status
git status

# Add changed files
git add .

# Commit changes
git commit -m "feat: merge new frontend with mode separation"

# Push changes
git push
```

---

## Troubleshooting

### Issue: Application Won't Start

**Symptom**: `npm run tauri dev` fails to start

**Solutions**:
1. Check Tauri installation:
   ```bash
   cargo tauri --version
   ```

2. Check Node.js version:
   ```bash
   node --version  # Should be 18+
   ```

3. Clear Tauri cache:
   ```bash
   cargo clean
   rm -rf src-tauri/target/
   ```

---

### Issue: HMR Not Working

**Symptom**: Frontend changes don't reflect in the running app

**Solutions**:
1. Check Vite dev server is running on port 1422
2. Verify `vite.config.ts` has correct HMR configuration
3. Restart dev server:
   ```bash
   # Stop (Ctrl+C) and restart
   npm run tauri dev
   ```

---

### Issue: Mode Switching Not Working

**Symptom**: Clicking mode toggle has no effect

**Solutions**:
1. Check if `ReviewModeContext` is properly wrapping the app
2. Verify `setMode` function is being called
3. Check console for errors
4. Verify mode is being persisted to Tauri Store

---

### Issue: CSS Variables Not Applying

**Symptom**: Editor settings don't change appearance

**Solutions**:
1. Verify CSS variables are defined in `index.css`:
   ```css
   :root {
     --editor-font-size: 14px;
     --editor-ligatures: normal;
     --editor-cursor: auto;
   }
   ```

2. Check if `useEditorStyles` hook is being called in `App.tsx`
3. Verify CSS classes are using the variables:
   ```css
   .font-mono {
     font-size: var(--editor-font-size) !important;
   }
   ```

---

### Issue: IPC Commands Failing

**Symptom**: Frontend commands show errors

**Solutions**:
1. Check if backend command exists in `src-tauri/src/lib.rs`
2. Verify command name matches frontend invoke call
3. Check backend logs for detailed error messages
4. Verify `tauri.conf.json` allowlist includes necessary permissions

---

### Issue: Type Errors After Merge

**Symptom**: TypeScript compilation fails

**Solutions**:
1. Run type check:
   ```bash
   npm run typecheck
   ```

2. Check for missing imports in merged files
3. Verify API types match backend command signatures
4. Check if `api-types.md` needs updates

---

## Next Steps After Quickstart

Once you've completed the quickstart:

1. **Review and Approve Plan**
   - Review `specs/009-merge-new-frontend/plan.md`
   - Review `specs/009-merge-new-frontend/research.md`
   - Review `specs/009-merge-new-frontend/data-model.md`
   - Review contract files in `specs/009-merge-new-frontend/contracts/`

2. **Proceed to `/speckit.tasks`**
   - Generate actionable tasks from the plan
   - Tasks will be ordered by dependencies
   - Each task will be independent and testable

3. **Implementation**
   - Follow the tasks from `specs/009-merge-new-frontend/tasks.md`
   - Implement incrementally
   - Test each task before moving to the next

---

## Helpful Resources

### Documentation

- **Tauri Documentation**: https://tauri.app/v1/guides/
- **React Documentation**: https://react.dev/
- **Zustand Documentation**: https://github.com/pmndrs/zustand
- **Tailwind CSS**: https://tailwindcss.com/docs

### Key Files

- **Plan**: `specs/009-merge-new-frontend/plan.md`
- **Research**: `specs/009-merge-new-frontend/research.md`
- **Data Model**: `specs/009-merge-new-frontend/data-model.md`
- **IPC Interfaces**: `specs/009-merge-new-frontend/contracts/ipc-interfaces.md`
- **API Types**: `specs/009-merge-new-frontend/contracts/api-types.md`
- **Constitution**: `.specify/memory/constitution.md`

---

## Support

If you encounter issues during development:

1. **Check Console**: Use browser DevTools and terminal for error messages
2. **Review Logs**: Check Tauri logs in `src-tauri/` directory
3. **Consult Research**: Review findings in `research.md`
4. **Verify Implementation**: Compare your code with plan recommendations

---

**Good luck with the merge!** 🚀
