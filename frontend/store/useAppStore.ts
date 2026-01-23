import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';

export type ReviewMode = 'local' | 'remote';

export interface EditorSettings {
  fontSize: number;
  enableLigatures: boolean;
  vimMode: boolean;
  theme: 'light' | 'dark' | 'system';
  lineWrapping: boolean;
  showLineNumbers: boolean;
  syntaxHighlighting: boolean;
}

export interface AppState {
  mode: ReviewMode;
  lastMode?: ReviewMode;
  modeSwitchTimestamp: number;

  editorSettings: EditorSettings;

  setMode: (mode: ReviewMode) => void;
  setEditorFontSize: (size: number) => void;
  setEditorLigatures: (enabled: boolean) => void;
  setVimMode: (enabled: boolean) => void;
  setEditorTheme: (theme: 'light' | 'dark' | 'system') => void;
  updateEditorSettings: (settings: Partial<EditorSettings>) => void;
}

const DEFAULT_EDITOR_SETTINGS: EditorSettings = {
  fontSize: 14,
  enableLigatures: true,
  vimMode: false,
  theme: 'system',
  lineWrapping: false,
  showLineNumbers: true,
  syntaxHighlighting: true,
};

export const useAppStore = create<AppState>()(
  persist(
    (set) => ({
      mode: 'local',
      lastMode: undefined,
      modeSwitchTimestamp: Date.now(),

      editorSettings: DEFAULT_EDITOR_SETTINGS,

      setMode: (mode) => set((state) => ({ mode, lastMode: state.mode, modeSwitchTimestamp: Date.now() })),
      setEditorFontSize: (editorFontSize) => set({ editorSettings: { ...DEFAULT_EDITOR_SETTINGS, fontSize: editorFontSize } }),
      setEditorLigatures: (enableLigatures) => set({ editorSettings: { ...DEFAULT_EDITOR_SETTINGS, enableLigatures } }),
      setVimMode: (vimMode) => set({ editorSettings: { ...DEFAULT_EDITOR_SETTINGS, vimMode } }),
      setEditorTheme: (theme) => set({ editorSettings: { ...DEFAULT_EDITOR_SETTINGS, theme } }),
      updateEditorSettings: (settings) => set((state) => ({ editorSettings: { ...state.editorSettings, ...settings } })),
    }),
    {
      name: 'hyperreview-app-state',
      storage: createJSONStorage(() => localStorage),
    }
  )
);
