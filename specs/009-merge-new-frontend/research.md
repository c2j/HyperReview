# Research Document: Merge New Frontend with Local/Remote Review Mode Separation

**Feature**: Merge new frontend from `tobemerged/HyperReview_Frontend` into existing `frontend` directory
**Date**: 2025-01-18
**Purpose**: Resolve technical unknowns and establish best practices for the merge

## Research Task 1: React Frontend Merge Patterns

### Decision: Incremental Migration with Feature Flag Pattern

**Rationale**:
- Allows testing merged components incrementally without breaking existing functionality
- Enables A/B comparison of old vs new components
- Facilitates rollback if issues arise
- Aligns with Tauri desktop app patterns where state is preserved between transitions

**Alternatives Considered**:
1. **Big Bang Migration**: Replace entire frontend at once
   - *Rejected because*: Too risky, difficult to debug, no rollback path, violates constitution's "no regressions" requirement
2. **Side-by-Side Implementation**: Keep both codebases and use routing
   - *Rejected because*: Increases bundle size, complex state management, violates constitution's performance constraints (<15MB bundle)
3. **Component Library Extraction**: Create shared component library first
   - *Rejected because*: Over-engineering for this use case, adds unnecessary complexity and build pipeline steps

### Implementation Strategy

#### Phase A: Component Organization
```
frontend/src/components/
├── local/              # Local mode specific
│   ├── LocalToolBar.tsx
│   ├── LocalTaskTree.tsx
│   └── LocalRightPanel.tsx
├── remote/             # Remote mode specific
│   ├── RemoteToolBar.tsx
│   ├── RemoteTaskTree.tsx
│   └── RemoteRightPanel.tsx
└── shared/             # Shared across modes
    ├── DiffView.tsx
    ├── TitleBar.tsx
    └── Modal.tsx
```

#### Phase B: Conflict Resolution
- **Duplicate component names**: Use namespace prefix or subdirectories (e.g., `LocalToolBar` vs `RemoteToolBar`)
- **Functionality conflicts**: Create wrapper components that abstract differences, preserve existing implementations when unclear
- **State management**: Migrate existing Zustand store incrementally, add mode-specific state slices

#### Phase C: Testing Strategy
- Keep existing tests running throughout migration
- Add snapshot tests for new components
- E2E tests for mode switching flows
- Performance regression tests for UI interactions

### Code Example: Component Migration Pattern

```typescript
// App.tsx - Mode-based component selection
import { LocalToolBar } from './components/local/LocalToolBar';
import { RemoteToolBar } from './components/remote/RemoteToolBar';

const App: React.FC = () => {
  const { mode } = useAppStore();

  return (
    <>
      {mode === 'local' ? <LocalToolBar /> : <RemoteToolBar />}
      {/* ... */}
    </>
  );
};
```

## Research Task 2: Tauri IPC Compatibility

### Decision: IPC Client Abstraction Layer with Graceful Degradation

**Rationale**:
- Provides clean interface between frontend and Tauri backend
- Enables feature detection and graceful degradation for missing commands
- Simplifies testing by mocking at the abstraction layer
- Maintains constitution's security requirement (no direct filesystem access)

**Alternatives Considered**:
1. **Direct invoke calls throughout codebase**
   - *Rejected because*: Difficult to maintain, error handling scattered, violates DRY principle
2. **Command generation from Tauri bindings**
   - *Rejected because*: TypeScript types may not match existing backend exactly, requires build-time integration
3. **Two separate IPC clients (old and new)**
   - *Rejected because*: Increases complexity, potential conflicts, harder to maintain

### Implementation Strategy

#### IPC Client Architecture

```typescript
// frontend/src/services/tauri-client.ts
import { invoke } from '@tauri-apps/api/core';

type CommandResult<T> = Result<T, string>;

class TauriClient {
  async invokeCommand<T>(command: string, args?: any): Promise<CommandResult<T>> {
    try {
      const result = await invoke<T>(command, args);
      return { success: true, data: result };
    } catch (error) {
      console.error(`IPC command failed: ${command}`, error);
      return { success: false, error: String(error) };
    }
  }

  async hasCommand(command: string): Promise<boolean> {
    try {
      await invoke(command, { __feature_detect__: true });
      return true;
    } catch {
      return false;
    }
  }

  // Existing Local review commands
  async getRecentRepos() {
    return this.invokeCommand<Repo[]>('get_recent_repos');
  }

  async getFileDiff(fileId: string) {
    return this.invokeCommand<DiffLine[]>('get_file_diff', { fileId });
  }

  // New Remote review commands (with graceful degradation)
  async getRemoteChanges() {
    const hasCommand = await this.hasCommand('get_remote_changes');
    if (!hasCommand) {
      return { success: false, error: 'Remote commands not yet implemented' };
    }
    return this.invokeCommand('get_remote_changes');
  }
}

export const tauriClient = new TauriClient();
```

#### TypeScript Interface Definitions

```typescript
// frontend/src/api/commands.ts
export interface Repo {
  path: string;
  branch: string;
  lastOpened: string;
}

export interface DiffLine {
  oldLineNumber?: number;
  newLineNumber?: number;
  content: string;
  type: 'added' | 'removed' | 'context' | 'header';
  severity?: 'ERROR' | 'WARNING' | 'INFO' | 'SUCCESS';
  message?: string;
}

export type ReviewMode = 'local' | 'remote';
```

#### Error Handling Pattern

```typescript
// frontend/src/utils/error-handler.ts
export class IPCErrorHandler {
  static handleCommandError(error: any, context: string): void {
    if (error.message?.includes('command not found')) {
      toast.error(`Feature not available: ${context}`);
    } else if (error.message?.includes('backend error')) {
      toast.error(`Backend operation failed: ${error.message}`);
    } else {
      toast.error(`Unexpected error: ${error.message}`);
    }
  }
}
```

## Research Task 3: React Mode Switching UI Patterns

### Decision: Mode State in Zustand Store with Conditional Rendering

**Rationale**:
- Zustand already used in existing frontend (constitution requirement)
- Simple, performant state management
- Enables persistence to localStorage easily
- Clear separation of concerns (mode state vs UI state)

**Alternatives Considered**:
1. **URL-based routing (/local, /remote)**
   - *Rejected because*: Desktop apps don't use URL routing, adds unnecessary complexity, violates performance goals
2. **React Context API for mode**
   - *Rejected because*: Constitution specifies Zustand, Context creates unnecessary re-renders, harder to persist
3. **Separate window instances per mode**
   - *Rejected because*: Resource intensive, violates constitution's performance constraints, poor UX

### Implementation Strategy

#### Zustand Store Definition

```typescript
// frontend/src/store/useAppStore.ts
import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface AppState {
  mode: 'local' | 'remote';
  setMode: (mode: 'local' | 'remote') => void;

  // Settings
  editorFontSize: number;
  setEditorFontSize: (size: number) => void;
  fontLigatures: boolean;
  setFontLigatures: (enabled: boolean) => void;
  vimMode: boolean;
  setVimMode: (enabled: boolean) => void;
}

export const useAppStore = create<AppState>()(
  persist(
    (set) => ({
      mode: 'local',
      setMode: (mode) => set({ mode }, false, 'mode'),

      editorFontSize: 14,
      setEditorFontSize: (editorFontSize) => set({ editorFontSize }, false, 'editorFontSize'),

      fontLigatures: true,
      setFontLigatures: (fontLigatures) => set({ fontLigatures }, false, 'fontLigatures'),

      vimMode: false,
      setVimMode: (vimMode) => set({ vimMode }, false, 'vimMode'),
    }),
    {
      name: 'hyperreview-app-state',
      storage: createJSONStorage(() => localStorage),
    }
  )
);
```

#### Mode Toggle Component

```typescript
// frontend/src/components/ModeToggle.tsx
import { useAppStore } from '../store/useAppStore';

export const ModeToggle: React.FC = () => {
  const { mode, setMode } = useAppStore();

  return (
    <div className="flex items-center gap-2">
      <button
        onClick={() => setMode('local')}
        className={`px-4 py-2 rounded ${
          mode === 'local' ? 'bg-blue-500 text-white' : 'bg-gray-200'
        }`}
      >
        Local
      </button>
      <button
        onClick={() => setMode('remote')}
        className={`px-4 py-2 rounded ${
          mode === 'remote' ? 'bg-purple-500 text-white' : 'bg-gray-200'
        }`}
      >
        Remote
      </button>
    </div>
  );
};
```

#### Conditional Rendering Pattern

```typescript
// App.tsx
const App: React.FC = () => {
  const { mode } = useAppStore();

  return (
    <div className="flex h-screen">
      <TitleBar />
      {mode === 'local' ? (
        <LocalTaskTree />
      ) : (
        <RemoteTaskTree />
      )}
      <DiffView />
      {mode === 'local' ? (
        <LocalRightPanel />
      ) : (
        <RemoteRightPanel />
      )}
    </div>
  );
};
```

#### Mode Transition Performance

```typescript
// Use React.memo for expensive components
const LocalTaskTree = React.memo(() => {/* ... */});
const RemoteTaskTree = React.memo(() => {/* ... */});

// Lazy load mode-specific components
const RemoteRightPanel = React.lazy(() =>
  import('./components/remote/RemoteRightPanel')
);
```

## Research Task 4: React Editor Settings Patterns

### Decision: Dynamic CSS Variables with Zustand Persistence

**Rationale**:
- CSS variables provide global, performant style updates without re-renders
- Zustand persist middleware handles localStorage automatically
- Constitution requires performance (<60fps UI)
- Aligns with existing Tailwind configuration

**Alternatives Considered**:
1. **Inline styles or style prop**
   - *Rejected because*: Causes re-renders, harder to maintain, violates performance goals
2. **Styled-components or Emotion**
   - *Rejected because*: Adds dependencies, constitution specifies Tailwind, increases bundle size
3. **CSS-in-JS (JSS)**
   - *Rejected because*: Runtime CSS generation, performance overhead, not needed with CSS variables

### Implementation Strategy

#### CSS Variables Approach

```typescript
// frontend/src/App.tsx
import { useAppStore } from './store/useAppStore';

const App: React.FC = () => {
  const { editorFontSize, fontLigatures, vimMode } = useAppStore();

  const dynamicStyles = useMemo(() => `
    :root {
      --editor-font-size: ${editorFontSize}px;
      --editor-ligatures: ${fontLigatures ? 'normal' : 'none'};
      --editor-cursor: ${vimMode ? 'block' : 'auto'};
    }
    pre[class*="language-"],
    code[class*="language-"],
    .font-mono,
    .diff-line-content {
      font-size: var(--editor-font-size) !important;
      font-variant-ligatures: var(--editor-ligatures) !important;
    }
    ${vimMode ? '.cursor-text { cursor: var(--editor-cursor) !important; }' : ''}
  `, [editorFontSize, fontLigatures, vimMode]);

  return (
    <>
      <style>{dynamicStyles}</style>
      {/* ... */}
    </>
  );
};
```

#### Settings Modal Integration

```typescript
// frontend/src/components/settings/EditorSettings.tsx
import { useAppStore } from '../../store/useAppStore';

export const EditorSettings: React.FC = () => {
  const {
    editorFontSize,
    setEditorFontSize,
    fontLigatures,
    setFontLigatures,
    vimMode,
    setVimMode,
  } = useAppStore();

  return (
    <div className="space-y-4">
      {/* Font Size Slider */}
      <div>
        <label className="block text-sm font-medium">Font Size</label>
        <input
          type="range"
          min="10"
          max="24"
          value={editorFontSize}
          onChange={(e) => setEditorFontSize(Number(e.target.value))}
          className="w-full"
        />
        <span className="text-sm text-gray-500">{editorFontSize}px</span>
      </div>

      {/* Ligatures Toggle */}
      <label className="flex items-center gap-2">
        <input
          type="checkbox"
          checked={fontLigatures}
          onChange={(e) => setFontLigatures(e.target.checked)}
        />
        <span>Enable Font Ligatures</span>
      </label>

      {/* Vim Mode Toggle */}
      <label className="flex items-center gap-2">
        <input
          type="checkbox"
          checked={vimMode}
          onChange={(e) => setVimMode(e.target.checked)}
        />
        <span>Vim Mode (Block Cursor)</span>
      </label>
    </div>
  );
};
```

#### Persistence Strategy

```typescript
// Using Zustand persist middleware (already defined in useAppStore.ts)
// Settings automatically saved to localStorage and restored on app restart

// For Tauri-specific settings that need backend storage:
// frontend/src/services/settings-service.ts
import { tauriClient } from './tauri-client';

export class SettingsService {
  async saveSetting<T>(key: string, value: T): Promise<void> {
    return tauriClient.invokeCommand('save_user_setting', { key, value });
  }

  async loadSetting<T>(key: string): Promise<T | null> {
    const result = await tauriClient.invokeCommand<T>('get_user_setting', { key });
    return result.success ? result.data : null;
  }
}
```

### Performance Considerations

- **CSS Variables**: Browser-optimized, no re-renders required when changed
- **useMemo**: Recomputes styles only when dependencies change
- **Throttle Settings Updates**: Debounce slider changes to avoid excessive style updates
- **Test Bundle Impact**: Settings components should add <100KB to bundle

## Additional Findings

### Component Duplication Resolution

When merging `tobemerged/HyperReview_Frontend` with existing `frontend`, handle duplicate components:

1. **Identify duplicates**:
   ```bash
   find frontend/src/components -name "*.tsx" -o -name "*.ts"
   find tobemerged/HyperReview_Frontend/components -name "*.tsx" -o -name "*.ts"
   # Compare and identify conflicts
   ```

2. **Resolution strategy**:
   - Keep newer component version if functionality is enhanced
   - Create `ComponentName.tsx.bak` backups for old versions
   - For shared functionality, create wrapper component that conditionally renders

3. **Example**:
   ```typescript
   // frontend/src/components/SharedModal.tsx
   import { Modal as NewModal } from './remote/Modal';
   import { Modal as OldModal } from './shared/Modal';

   export const SharedModal: React.FC<ModalProps> = (props) => {
     const { mode } = useAppStore();
     return mode === 'remote' ? <NewModal {...props} /> : <OldModal {...props} />;
   };
   ```

### Build Configuration Updates

```typescript
// frontend/vite.config.ts
export default defineConfig(async () => {
  return {
    // ... existing config ...

    // Ensure proper alias resolution for component paths
    resolve: {
      alias: {
        '@components': path.resolve(__dirname, './src/components'),
        '@services': path.resolve(__dirname, './src/services'),
        '@store': path.resolve(__dirname, './src/store'),
        '@api': path.resolve(__dirname, './src/api'),
      },
    },
  };
});
```

## Risk Assessment

| Risk | Impact | Mitigation |
|-------|---------|------------|
| Duplicate component names causing import conflicts | Medium | Use namespace prefixes, organize in subdirectories |
| Breaking changes to existing IPC interface | High | Create abstraction layer, add feature detection |
| Performance regression from new components | Medium | Keep existing tests, add performance benchmarks |
| State management conflicts (Zustand store) | Medium | Incremental migration, slice-based architecture |
| Bundle size increase beyond 15MB | Low | Tree shaking, code splitting, lazy loading |

## Recommendations

1. **Start with Phase 0**: Set up component directory structure and IPC abstraction layer
2. **Migrate incrementally**: One component at a time, test thoroughly
3. **Monitor performance**: Use React DevTools and Tauri devtools throughout
4. **Document changes**: Keep changelog for each migrated component
5. **Backup existing code**: Use git branches for safe rollback
6. **Continuous testing**: Run `npm run check:all` after each component merge

## Next Steps

1. Review and approve this research document
2. Proceed to Phase 1: Design & Contracts
3. Create data-model.md
4. Generate API contracts (IPC interfaces, TypeScript types)
5. Update agent context files
