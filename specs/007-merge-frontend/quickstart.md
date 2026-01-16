# Quickstart Guide: Merge HyperReview Frontend

**Feature**: 007-merge-frontend
**Generated**: 2025-01-16
**Purpose**: Development setup and merge workflow guide

## Prerequisites

### Development Environment

- **Node.js**: 18.x or later
- **Rust**: 1.75 or later
- **Tauri CLI**: 2.x (installed via `cargo install tauri-cli`)
- **Git**: 2.x or later
- **Package Managers**: npm (Node.js), cargo (Rust)

### IDE/Editor

- **VS Code** (recommended) with extensions:
  - ESLint
  - Prettier
  - Rust Analyzer
  - Tauri
  - Tailwind CSS IntelliSense

---

## Project Structure

```
HyperReview/
├── src-tauri/              # Rust backend (NO MODIFICATIONS)
│   ├── src/
│   ├── Cargo.toml
│   └── tauri.conf.json
├── frontend/               # React frontend (MERGE TARGET)
│   ├── src/
│   │   ├── api/           # API client and types
│   │   ├── components/    # UI components (merge + preserve)
│   │   ├── context/       # React context providers
│   │   ├── hooks/         # Custom React hooks
│   │   ├── services/      # Business logic services
│   │   ├── store/         # Zustand stores
│   │   └── App.tsx        # Main app component
│   ├── package.json
│   └── tsconfig.json
└── tobemerged/HyperReview_Frontend/  # Source UI (reference only)
    ├── App.tsx
    ├── components/
    └── api/
```

---

## Development Setup

### 1. Clone Repository and Switch to Feature Branch

```bash
# Clone repository (if not already cloned)
git clone https://github.com/your-org/HyperReview.git
cd HyperReview

# Switch to feature branch
git checkout 007-merge-frontend
git pull origin 007-merge-frontend
```

### 2. Install Dependencies

```bash
# Install frontend dependencies
cd frontend
npm install

# Return to project root
cd ..

# Install Rust dependencies (if needed)
cd src-tauri
cargo fetch
cd ..
```

### 3. Run Development Server

```bash
# Start Tauri development server
npm run tauri dev

# Or run frontend only (for UI development)
cd frontend
npm run dev
```

The application will open in a new window with hot-reload enabled for frontend changes.

---

## Merge Workflow

### Overview

The merge follows this strategy:
1. **UI Priority**: Use HyperReview_Frontend components as the UI standard
2. **IPC Preservation**: Keep all existing IPC integrations from current frontend
3. **Service Preservation**: Maintain all existing services (Gerrit, review, etc.)
4. **Unique Component Preservation**: Keep components unique to current frontend
5. **Graceful Degradation**: Temporarily disable features that cannot be reconciled

---

## Step-by-Step Merge Guide

### Phase 1: Preparation

#### 1.1 Understand Source and Target

**Review HyperReview_Frontend (UI source)**:
```bash
# List components in HyperReview_Frontend
ls tobemerged/HyperReview_Frontend/components/

# Review main app structure
cat tobemerged/HyperReview_Frontend/App.tsx

# Review API types (if any)
ls tobemerged/HyperReview_Frontend/api/types/
```

**Review Current Frontend (target)**:
```bash
# List existing components
ls frontend/components/

# Review existing IPC client
cat frontend/api/client.ts

# Review existing services
ls frontend/services/
```

#### 1.2 Identify Component Conflicts

Compare component lists to identify conflicts:

```bash
# Find common component names
comm -12 \
  <(ls tobemerged/HyperReview_Frontend/components/ | sort) \
  <(ls frontend/components/ | sort)
```

**Expected conflicts** (based on analysis):
- DiffView.tsx
- CommandPalette.tsx
- NewTaskModal.tsx
- OpenRepoModal.tsx
- SettingsModal.tsx
- And others...

---

### Phase 2: Component Merge

#### 2.1 Merge App.tsx

**Strategy**: Use HyperReview_Frontend App.tsx as base, integrate IPC integrations

```bash
# Backup current App.tsx
cp frontend/src/App.tsx frontend/src/App.tsx.backup

# Copy HyperReview_Frontend App.tsx as starting point
cp tobemerged/HyperReview_Frontend/App.tsx frontend/src/App.tsx
```

**Manual integration steps**:
1. Open `frontend/src/App.tsx`
2. Review imports from HyperReview_Frontend
3. Add missing imports from current frontend:
   - `useApiClient` from `./api/client`
   - Existing services (gerrit-simple-service, etc.)
4. Add mode switching logic from HyperReview_Frontend:
   - `mode` state
   - `isGerritConfigured` state
   - Mode toggle handlers
5. Preserve existing IPC invocations
6. Test application loads without errors

#### 2.2 Merge DiffView Component

**Strategy**: Use HyperReview_Frontend DiffView.tsx, ensure IPC compatibility

```bash
# Copy HyperReview_Frontend version
cp tobemerged/HyperReview_Frontend/components/DiffView.tsx \
   frontend/src/components/DiffView.tsx

# Or create new file if current has different name
cp tobemerged/HyperReview_Frontend/components/DiffView.tsx \
   frontend/src/components/HyperReviewDiffView.tsx
```

**Manual integration steps**:
1. Review DiffView props and state
2. Ensure DiffLine interface matches current frontend types
3. Verify IPC calls for fetching diffs use existing client
4. Test with local repository diff
5. Test with Gerrit change diff

#### 2.3 Merge Mode-Specific Components

**Local Mode Components** (from HyperReview_Frontend):
```bash
# Copy local mode components
cp tobemerged/HyperReview_Frontend/components/LocalToolBar.tsx \
   frontend/src/components/LocalToolBar.tsx
cp tobemerged/HyperReview_Frontend/components/LocalTaskTree.tsx \
   frontend/src/components/LocalTaskTree.tsx
cp tobemerged/HyperReview_Frontend/components/LocalRightPanel.tsx \
   frontend/src/components/LocalRightPanel.tsx
```

**Remote Mode Components** (from HyperReview_Frontend):
```bash
# Copy remote mode components
cp tobemerged/HyperReview_Frontend/components/RemoteToolBar.tsx \
   frontend/src/components/RemoteToolBar.tsx
cp tobemerged/HyperReview_Frontend/components/RemoteTaskTree.tsx \
   frontend/src/components/RemoteTaskTree.tsx
cp tobemerged/HyperReview_Frontend/components/RemoteRightPanel.tsx \
   frontend/src/components/RemoteRightPanel.tsx
```

**Manual integration steps**:
1. Review component props and state
2. Connect to existing Zustand stores or create new ones
3. Ensure IPC calls use existing client
4. Test switching between modes

#### 2.4 Merge Modal Components

**Common Modals** (preserve functionality):
```bash
# Copy HyperReview_Frontend versions
cp tobemerged/HyperReview_Frontend/components/OpenRepoModal.tsx \
   frontend/src/components/OpenRepoModal.tsx
cp tobemerged/HyperReview_Frontend/components/NewTaskModal.tsx \
   frontend/src/components/NewTaskModal.tsx
cp tobemerged/HyperReview_Frontend/components/SettingsModal.tsx \
   frontend/src/components/SettingsModal.tsx
```

**Manual integration steps**:
1. Ensure existing IPC commands are used
2. Preserve any custom functionality from current frontend
3. Test modal open/close behavior

#### 2.5 Preserve Unique Components

**Current Frontend Unique Components**:
```bash
# These components are preserved as-is
# CredentialManager.tsx - Already exists in current frontend
# ExternalSubmissionDialog.tsx - Already exists in current frontend
# CommentCreator.tsx - Already exists in current frontend
# And others...
```

**Manual integration steps**:
1. Verify components still work with merged App.tsx
2. Update imports if component locations changed
3. Test functionality

---

### Phase 3: State Management Integration

#### 3.1 Application-Level State (Zustand)

Create or update application state store:

```typescript
// frontend/src/store/appStore.ts
import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface AppState {
  // Mode
  mode: 'local' | 'remote';
  setMode: (mode: 'local' | 'remote') => void;

  // User Settings
  fontSize: number;
  ligatures: boolean;
  vimMode: boolean;
  language: string;
  theme: 'light' | 'dark';
  updateSettings: (settings: Partial<AppState>) => void;

  // Panels
  leftWidth: number;
  rightWidth: number;
  showLeft: boolean;
  showRight: boolean;
  updatePanel: (panel: 'left' | 'right', updates: Partial<Panel>) => void;
}

export const useAppStore = create<AppState>()(
  persist(
    (set) => ({
      mode: 'local',
      setMode: (mode) => set({ mode }),

      fontSize: 14,
      ligatures: false,
      vimMode: false,
      language: 'en',
      theme: 'light',
      updateSettings: (settings) => set(settings),

      leftWidth: 260,
      rightWidth: 300,
      showLeft: true,
      showRight: true,
      updatePanel: (panel, updates) => set({ [`${panel}Width`]: updates.width, [`show${panel.charAt(0).toUpperCase() + panel.slice(1)}`]: updates.visible }),
    }),
    {
      name: 'hyperreview-app-store',
    }
  )
);
```

#### 3.2 Local Mode State (Zustand)

```typescript
// frontend/src/store/localModeStore.ts
import { create } from 'zustand';

interface LocalModeState {
  isRepoLoaded: boolean;
  selectedRepoPath: string | null;
  diffContext: { base: string; head: string };
  activeFilePath: string | null;

  setRepoLoaded: (loaded: boolean) => void;
  setSelectedRepoPath: (path: string | null) => void;
  setDiffContext: (context: Partial<{ base: string; head: string }>) => void;
  setActiveFilePath: (path: string | null) => void;
}

export const useLocalModeStore = create<LocalModeState>((set) => ({
  isRepoLoaded: false,
  selectedRepoPath: null,
  diffContext: { base: 'master', head: 'main' },
  activeFilePath: null,

  setRepoLoaded: (loaded) => set({ isRepoLoaded: loaded }),
  setSelectedRepoPath: (path) => set({ selectedRepoPath: path }),
  setDiffContext: (context) => set((state) => ({ diffContext: { ...state.diffContext, ...context } })),
  setActiveFilePath: (path) => set({ activeFilePath: path }),
}));
```

#### 3.3 Remote Mode State (Zustand)

```typescript
// frontend/src/store/remoteModeStore.ts
import { create } from 'zustand';

interface RemoteModeState {
  isGerritConfigured: boolean;
  gerritChanges: GerritChange[];
  selectedChangeNumber: number | null;

  setGerritConfigured: (configured: boolean) => void;
  setGerritChanges: (changes: GerritChange[]) => void;
  setSelectedChangeNumber: (number: number | null) => void;
}

export const useRemoteModeStore = create<RemoteModeState>((set) => ({
  isGerritConfigured: false,
  gerritChanges: [],
  selectedChangeNumber: null,

  setGerritConfigured: (configured) => set({ isGerritConfigured: configured }),
  setGerritChanges: (changes) => set({ gerritChanges: changes }),
  setSelectedChangeNumber: (number) => set({ selectedChangeNumber: number }),
}));
```

---

### Phase 4: IPC Integration

#### 4.1 Verify IPC Client

Ensure `frontend/src/api/client.ts` is preserved:

```bash
# Verify IPC client exists
ls -la frontend/src/api/client.ts

# Review IPC commands
grep -n "invoke\|__TAURI__" frontend/src/api/client.ts
```

**Key IPC commands** (must be preserved):
- `open_repo_dialog`
- `load_repo`
- `get_branches`
- `get_file_diff`
- `get_blame`
- `import_gerrit_change`
- `get_gerrit_changes`
- `submit_gerrit_review`
- And all others from current frontend...

#### 4.2 Update Component IPC Usage

For each merged component:
1. Review IPC invocations
2. Ensure they use existing client
3. Add missing IPC commands if needed
4. Test IPC calls work correctly

**Example**: Update LocalTaskTree to use existing IPC

```typescript
// Before (HyperReview_Frontend)
const branches = await getBranches(repoPath); // Missing implementation

// After (merged)
import { invoke } from '@tauri-apps/api/core';

const branches = await invoke<Branch[]>('get_branches', { path: repoPath });
```

---

### Phase 5: Services Integration

#### 5.1 Preserve Existing Services

Ensure all services are preserved:

```bash
# List existing services
ls frontend/src/services/

# Verify services are imported correctly
grep -r "import.*service" frontend/src/components/
```

**Key services to preserve**:
- `gerrit-simple-service.ts`
- `gerrit-instance-service.ts`
- `reviewService.ts`

#### 5.2 Update Service Usage

For each service:
1. Review service methods
2. Ensure merged components use services correctly
3. Add missing service calls if needed
4. Test service functionality

---

### Phase 6: Internationalization (i18n)

#### 6.1 Merge i18n Translations

```bash
# Backup current i18n
cp frontend/src/i18n.tsx frontend/src/i18n.tsx.backup

# Review HyperReview_Frontend i18n
cat tobemerged/HyperReview_Frontend/i18n.tsx

# Manually merge translations
# - Keep all existing translations from current frontend
# - Add new translations from HyperReview_Frontend
# - Resolve conflicts (prefer HyperReview_Frontend for UI text)
```

---

### Phase 7: Documentation Merge

#### 7.1 Merge Documentation Files

```bash
# Create docs directory in frontend (if not exists)
mkdir -p frontend/docs

# Copy and update documentation
cp tobemerged/HyperReview_Frontend/IPC.md frontend/docs/IPC.md
cp tobemerged/HyperReview_Frontend/OpenAPI.md frontend/docs/OpenAPI.md
cp tobemerged/HyperReview_Frontend/design-backend.md frontend/docs/design-backend.md

# Update documentation to reflect merged implementation
# - Add references to preserved services
# - Update examples to use merged components
# - Add migration notes
```

---

## Testing

### Run Tests

```bash
# Frontend tests
cd frontend
npm test

# Frontend test with coverage
npm test -- --coverage

# Rust tests (backend should remain unchanged)
cd ../src-tauri
cargo test
```

### Manual Testing Checklist

#### Local Mode
- [ ] Open repository via native dialog
- [ ] Load repository and view branches
- [ ] Select base and head branches
- [ ] View file diff
- [ ] Add inline comment
- [ ] Resize panels
- [ ] Toggle panel visibility

#### Remote Mode
- [ ] Switch to remote mode
- [ ] Configure Gerrit server
- [ ] Import Gerrit change
- [ ] View Gerrit change diff
- [ ] Add inline comment to Gerrit change
- [ ] Submit review to Gerrit

#### Mode Switching
- [ ] Switch from local to remote (user settings preserved, diff state reset)
- [ ] Switch from remote to local (user settings preserved, change selection reset)
- [ ] Verify mode switching completes within 500ms

#### IPC Integration
- [ ] Verify all IPC commands work correctly
- [ ] Test error handling for IPC failures
- [ ] Test network error recovery (Gerrit operations)

#### Unique Components
- [ ] Test CredentialManager (if preserved)
- [ ] Test ExternalSubmissionDialog (if preserved)
- [ ] Test other unique components

---

## Build & Package

### Development Build

```bash
# Build frontend only
cd frontend
npm run build

# Build Tauri app (includes frontend)
npm run tauri build
```

### Production Build

```bash
# Build for all platforms
npm run tauri build

# Build for specific platform
npm run tauri build --target x86_64-pc-windows-msvc  # Windows
npm run tauri build --target x86_64-apple-darwin   # macOS
npm run tauri build --target x86_64-unknown-linux-gnu # Linux
```

### Bundle Size Check

```bash
# Check bundle size (should be <15MB for Windows)
ls -lh src-tauri/target/release/bundle/nsis/*.exe
```

---

## Troubleshooting

### Common Issues

#### Issue: Application won't start after merge

**Symptoms**: Application crashes or shows blank screen

**Solutions**:
1. Check browser console for errors (DevTools)
2. Verify all imports are correct
3. Check TypeScript compilation errors
4. Ensure all IPC commands are defined in backend

#### Issue: Mode switching doesn't work

**Symptoms**: Clicking mode toggle has no effect

**Solutions**:
1. Check `mode` state is correctly managed in Zustand store
2. Verify mode toggle handler is properly connected
3. Check for console errors during mode switch
4. Ensure both LocalMode and RemoteMode stores are imported

#### Issue: Gerrit operations fail

**Symptoms**: Cannot import changes or submit reviews

**Solutions**:
1. Verify Gerrit server configuration is correct
2. Check IPC commands are properly invoked
3. Test Gerrit services independently
4. Check network connectivity to Gerrit server

#### Issue: Panel resizing breaks layout

**Symptoms**: Resizing panels causes layout issues

**Solutions**:
1. Check panel width state is correctly updated
2. Verify CSS handles panel width changes
3. Ensure minimum and maximum width constraints
4. Test panel resizing in both modes

#### Issue: IPC commands fail with "command not found"

**Symptoms**: IPC invoke throws "command not found" error

**Solutions**:
1. Verify command is defined in Rust backend (src-tauri/src/commands.rs)
2. Check command name is spelled correctly
3. Ensure Tauri CLI is up-to-date
4. Restart development server

---

## Git Workflow

### Commit Strategy

```bash
# Stage changes
git add frontend/

# Commit with conventional commit message
git commit -m "feat(merge): integrate HyperReview_Frontend UI components

- Merge App.tsx with mode switching support
- Add LocalToolBar, LocalTaskTree, LocalRightPanel
- Add RemoteToolBar, RemoteTaskTree, RemoteRightPanel
- Preserve existing IPC integrations and services
- Update state management with mode-specific stores
- Merge i18n translations
- Update documentation"

# Push to remote
git push origin 007-merge-frontend
```

### Create Pull Request

1. Go to GitHub repository
2. Click "New Pull Request"
3. Select `007-merge-frontend` branch
4. Compare to `main` branch
5. Fill PR description:
   - Summary of changes
   - Testing checklist
   - Known issues
   - Migration notes
6. Request review from team members

---

## Next Steps

After completing the merge:

1. **Run Full Test Suite**: Ensure all tests pass
2. **Code Review**: Submit PR for team review
3. **Fix Issues**: Address feedback from review
4. **Update Documentation**: Update README, user guides
5. **Merge to Main**: Merge PR after approval
6. **Release**: Create new release with merged frontend

---

## Resources

- [Feature Specification](./spec.md)
- [Data Model](./data-model.md)
- [IPC Contract](./contracts/ipc-contract.md)
- [TypeScript Types](./contracts/types-contract.md)
- [Tauri Documentation](https://tauri.app/v1/guides/)
- [React Documentation](https://react.dev/)
- [Zustand Documentation](https://docs.pmnd.rs/zustand)

---

**End of Quickstart Guide**
