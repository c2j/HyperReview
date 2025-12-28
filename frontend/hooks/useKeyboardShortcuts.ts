import { useEffect, useCallback } from 'react';

export interface KeyboardShortcutOptions {
  ctrl?: boolean;
  shift?: boolean;
  alt?: boolean;
  meta?: boolean;
  key: string;
  callback: (e: KeyboardEvent) => void;
  enabled?: boolean;
}

export function useKeyboardShortcuts(options: KeyboardShortcutOptions[]) {
  const checkModifiers = (e: KeyboardEvent, opts: KeyboardShortcutOptions): boolean => {
    const ctrlOk = opts.ctrl === undefined || e.ctrlKey === opts.ctrl;
    const shiftOk = opts.shift === undefined || e.shiftKey === opts.shift;
    const altOk = opts.alt === undefined || e.altKey === opts.alt;
    const metaOk = opts.meta === undefined || e.metaKey === opts.meta;
    const keyOk = e.key.toLowerCase() === opts.key.toLowerCase();

    return ctrlOk && shiftOk && altOk && metaOk && keyOk;
  };

  const handleKeyDown = useCallback(
    (e: KeyboardEvent) => {
      for (const opt of options) {
        if (opt.enabled !== false && checkModifiers(e, opt)) {
          e.preventDefault();
          opt.callback(e);
          break;
        }
      }
    },
    [options],
  );

  useEffect(() => {
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [handleKeyDown]);
}

export function useKeyboardShortcut(
  shortcut: Omit<KeyboardShortcutOptions, 'key'> & { key: string },
  enabled: boolean = true,
) {
  useKeyboardShortcuts([{ ...shortcut, enabled }]);
}
