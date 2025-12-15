import { create } from 'zustand';

// ============================================================================
// Error Types and Interfaces
// ============================================================================

export enum ErrorSeverity {
  ERROR = 'ERROR',
  WARNING = 'WARNING',
  INFO = 'INFO',
  SUCCESS = 'SUCCESS'
}

export interface AppError {
  id: string;
  severity: ErrorSeverity;
  title: string;
  message: string;
  details?: string;
  code?: string | number;
  timestamp: number;
  context?: Record<string, any>;
  retryable?: boolean;
  dismissible?: boolean;
}

export interface ToastNotification {
  id: string;
  severity: ErrorSeverity;
  title: string;
  message: string;
  duration?: number; // in milliseconds, undefined = auto
  timestamp: number;
  actions?: Array<{
    label: string;
    action: () => void;
  }>;
}

// ============================================================================
// Error Store (Zustand)
// ============================================================================

interface ErrorState {
  errors: AppError[];
  toasts: ToastNotification[];
  isLoading: boolean;

  // Error actions
  addError: (error: Omit<AppError, 'id' | 'timestamp'>) => string;
  removeError: (id: string) => void;
  clearErrors: () => void;
  clearErrorByContext: (context: string) => void;

  // Toast actions
  showToast: (toast: Omit<ToastNotification, 'id' | 'timestamp'>) => string;
  removeToast: (id: string) => void;
  clearToasts: () => void;

  // Loading state
  setLoading: (loading: boolean) => void;
}

export const useErrorStore = create<ErrorState>((set, get) => ({
  errors: [],
  toasts: [],
  isLoading: false,

  addError: (errorData) => {
    const id = `error_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    const error: AppError = {
      id,
      timestamp: Date.now(),
      dismissible: true,
      ...errorData
    };

    set((state) => ({
      errors: [...state.errors, error]
    }));

    // Auto-remove non-critical errors after 10 seconds
    if (error.severity !== ErrorSeverity.ERROR) {
      setTimeout(() => {
        get().removeError(id);
      }, 10000);
    }

    return id;
  },

  removeError: (id) => {
    set((state) => ({
      errors: state.errors.filter((e) => e.id !== id)
    }));
  },

  clearErrors: () => {
    set({ errors: [] });
  },

  clearErrorByContext: (context) => {
    set((state) => ({
      errors: state.errors.filter((e) => e.context?.context !== context)
    }));
  },

  showToast: (toastData) => {
    const id = `toast_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    const toast: ToastNotification = {
      id,
      timestamp: Date.now(),
      duration: 4000, // Default 4 seconds
      ...toastData
    };

    set((state) => ({
      toasts: [...state.toasts, toast]
    }));

    // Auto-remove toast after duration
    if (toast.duration !== 0) {
      setTimeout(() => {
        get().removeToast(id);
      }, toast.duration);
    }

    return id;
  },

  removeToast: (id) => {
    set((state) => ({
      toasts: state.toasts.filter((t) => t.id !== id)
    }));
  },

  clearToasts: () => {
    set({ toasts: [] });
  },

  setLoading: (loading) => {
    set({ isLoading: loading });
  }
}));

// ============================================================================
// Error Message Formatter
// ============================================================================

export class ErrorFormatter {
  static format(error: unknown): {
    title: string;
    message: string;
    severity: ErrorSeverity;
    code?: string | number;
  } {
    // Handle string errors
    if (typeof error === 'string') {
      return {
        title: 'Error',
        message: error,
        severity: ErrorSeverity.ERROR
      };
    }

    // Handle Error objects
    if (error instanceof Error) {
      const message = error.message || 'An unexpected error occurred';
      const code = (error as any).code;

      // Git-related errors
      if (message.includes('git') || message.includes('repository')) {
        return {
          title: 'Git Operation Failed',
          message: this.extractReadableMessage(message),
          severity: ErrorSeverity.ERROR,
          code
        };
      }

      // IPC errors
      if (message.includes('IPC') || message.includes('invoke')) {
        return {
          title: 'Backend Communication Error',
          message: 'Failed to communicate with the backend service',
          severity: ErrorSeverity.ERROR,
          code
        };
      }

      // File system errors
      if (message.includes('ENOENT') || message.includes('EACCES') || message.includes('permission')) {
        return {
          title: 'File System Error',
          message: this.extractReadableMessage(message),
          severity: ErrorSeverity.ERROR,
          code
        };
      }

      // Network errors
      if (message.includes('network') || message.includes('fetch') || message.includes('HTTP')) {
        return {
          title: 'Network Error',
          message: 'Failed to fetch data from the server',
          severity: ErrorSeverity.ERROR,
          code
        };
      }

      // Default error
      return {
        title: 'Error',
        message: this.extractReadableMessage(message),
        severity: ErrorSeverity.ERROR,
        code
      };
    }

    // Handle Tauri invoke errors
    if ((error as any)?.name === 'InvokeError') {
      return {
        title: 'Backend Error',
        message: (error as any)?.message || 'Backend operation failed',
        severity: ErrorSeverity.ERROR,
        code: (error as any)?.code
      };
    }

    // Handle unknown errors
    return {
      title: 'Unknown Error',
      message: 'An unexpected error occurred',
      severity: ErrorSeverity.ERROR
    };
  }

  private static extractReadableMessage(message: string): string {
    // Remove technical details and make more user-friendly
    const readable = message
      .replace(/Error: /gi, '')
      .replace(/^git /gi, '')
      .replace(/\n.*$/g, '') // Remove multi-line details
      .trim();

    // Common error translations
    const translations: Record<string, string> = {
      'could not find repository': 'Repository not found. Please check the path.',
      'repository not found': 'Repository not found. Please check the path.',
      'permission denied': 'Permission denied. Please check your access rights.',
      'no such file or directory': 'File or directory not found.',
      'pathspec': 'Invalid path specified.',
      'branch not found': 'Branch not found.',
      'commit not found': 'Commit not found.'
    };

    for (const [key, value] of Object.entries(translations)) {
      if (readable.toLowerCase().includes(key)) {
        return value;
      }
    }

    return readable;
  }

  static formatSuccess(message: string): Omit<ToastNotification, 'id' | 'timestamp'> {
    return {
      severity: ErrorSeverity.SUCCESS,
      title: 'Success',
      message
    };
  }

  static formatInfo(message: string): Omit<ToastNotification, 'id' | 'timestamp'> {
    return {
      severity: ErrorSeverity.INFO,
      title: 'Information',
      message
    };
  }

  static formatWarning(message: string): Omit<ToastNotification, 'id' | 'timestamp'> {
    return {
      severity: ErrorSeverity.WARNING,
      title: 'Warning',
      message
    };
  }

  static formatError(message: string): Omit<ToastNotification, 'id' | 'timestamp'> {
    return {
      severity: ErrorSeverity.ERROR,
      title: 'Error',
      message
    };
  }
}

// ============================================================================
// IPC Error Handler
// ============================================================================

export class IPCErrorHandler {
  static async handle<T>(
    operation: () => Promise<T>,
    context?: string,
    customErrorMessage?: string
  ): Promise<T | null> {
    try {
      const result = await operation();
      return result;
    } catch (error) {
      const formatted = ErrorFormatter.format(error);

      // Add to error store
      useErrorStore.getState().addError({
        severity: formatted.severity,
        title: formatted.title,
        message: customErrorMessage || formatted.message,
        code: formatted.code,
        context: context ? { context } : undefined,
        retryable: true
      });

      return null;
    }
  }

  static async handleWithToast<T>(
    operation: () => Promise<T>,
    successMessage: string,
    context?: string
  ): Promise<T | null> {
    try {
      const result = await operation();

      // Show success toast
      useErrorStore.getState().showToast(
        ErrorFormatter.formatSuccess(successMessage)
      );

      return result;
    } catch (error) {
      const formatted = ErrorFormatter.format(error);

      // Show error toast
      useErrorStore.getState().showToast(
        ErrorFormatter.formatError(formatted.message)
      );

      // Also add to error store for tracking
      useErrorStore.getState().addError({
        severity: formatted.severity,
        title: formatted.title,
        message: formatted.message,
        code: formatted.code,
        context: context ? { context } : undefined,
        retryable: true
      });

      return null;
    }
  }
}

// ============================================================================
// Repository Error Handler
// ============================================================================

export class RepositoryErrorHandler {
  // Repository opening errors
  static handleRepoNotFound(path: string) {
    useErrorStore.getState().addError({
      severity: ErrorSeverity.ERROR,
      title: 'Repository Not Found',
      message: `The repository at "${path}" could not be found or is not a valid Git repository.`,
      context: { path, operation: 'open' },
      retryable: false
    });
  }

  static handleAccessDenied(path: string) {
    useErrorStore.getState().addError({
      severity: ErrorSeverity.ERROR,
      title: 'Access Denied',
      message: `You do not have permission to access the repository at "${path}".`,
      context: { path, operation: 'open' },
      retryable: false
    });
  }

  static handleOpenError(path: string, error: unknown) {
    const formatted = ErrorFormatter.format(error);
    const context = { path, operation: 'open' };

    // Determine appropriate error message based on error type
    let message = `Failed to open repository at "${path}": ${formatted.message}`;
    let severity = ErrorSeverity.ERROR;
    let retryable = true;

    // Check for specific error conditions
    const errorMsg = error instanceof Error ? error.message.toLowerCase() : '';
    if (errorMsg.includes('not a git repository') || errorMsg.includes('could not find repository')) {
      message = `The directory "${path}" is not a valid Git repository. Please select a directory that contains a .git folder.`;
      retryable = false;
    } else if (errorMsg.includes('permission denied') || errorMsg.includes('eacces')) {
      message = `Permission denied when trying to access "${path}". Please check your file system permissions.`;
      retryable = false;
    } else if (errorMsg.includes('corrupted') || errorMsg.includes('broken')) {
      message = `The Git repository at "${path}" appears to be corrupted. Please repair it using Git tools.`;
      retryable = false;
    }

    useErrorStore.getState().addError({
      severity,
      title: 'Failed to Open Repository',
      message,
      details: error instanceof Error ? error.stack : undefined,
      context,
      retryable
    });
  }

  static handleOpenCancelled() {
    useErrorStore.getState().showToast(
      ErrorFormatter.formatInfo('Repository selection cancelled')
    );
  }

  // Repository loading errors
  static handleLoadError(path: string, error: unknown) {
    const formatted = ErrorFormatter.format(error);
    useErrorStore.getState().addError({
      severity: ErrorSeverity.ERROR,
      title: 'Failed to Load Repository',
      message: `Could not load repository at "${path}": ${formatted.message}`,
      details: error instanceof Error ? error.stack : undefined,
      context: { path, operation: 'load' },
      retryable: true
    });
  }

  static handleLoadSuccess(path: string) {
    useErrorStore.getState().showToast(
      ErrorFormatter.formatSuccess(`Successfully loaded repository: ${path}`)
    );
  }

  // Branch-related errors
  static handleBranchNotFound(branch: string) {
    useErrorStore.getState().addError({
      severity: ErrorSeverity.ERROR,
      title: 'Branch Not Found',
      message: `The branch "${branch}" does not exist in this repository.`,
      context: { branch, operation: 'branch_switch' },
      retryable: true
    });
  }

  static handleBranchLoadError(error: unknown) {
    const formatted = ErrorFormatter.format(error);
    useErrorStore.getState().addError({
      severity: ErrorSeverity.ERROR,
      title: 'Failed to Load Branches',
      message: `Could not retrieve branches: ${formatted.message}`,
      details: error instanceof Error ? error.stack : undefined,
      context: { operation: 'load_branches' },
      retryable: true
    });
  }

  static handleBranchSwitchError(branch: string, error: unknown) {
    const formatted = ErrorFormatter.format(error);
    const errorMsg = error instanceof Error ? error.message.toLowerCase() : '';

    let message = `Failed to switch to branch "${branch}": ${formatted.message}`;
    let retryable = true;

    if (errorMsg.includes('not found') || errorMsg.includes('does not exist')) {
      message = `Branch "${branch}" does not exist in this repository.`;
      retryable = false;
    } else if (errorMsg.includes('commit') && errorMsg.includes('not found')) {
      message = `Cannot switch to branch "${branch}": associated commit not found.`;
      retryable = true;
    }

    useErrorStore.getState().addError({
      severity: ErrorSeverity.ERROR,
      title: 'Failed to Switch Branch',
      message,
      context: { branch, operation: 'branch_switch' },
      retryable
    });
  }

  static handleBranchSwitchSuccess(branch: string) {
    useErrorStore.getState().showToast(
      ErrorFormatter.formatSuccess(`Switched to branch: ${branch}`)
    );
  }

  // Recent repositories errors
  static handleRecentReposLoadError(error: unknown) {
    const formatted = ErrorFormatter.format(error);
    useErrorStore.getState().addError({
      severity: ErrorSeverity.WARNING,
      title: 'Failed to Load Recent Repositories',
      message: `Could not retrieve recently opened repositories: ${formatted.message}`,
      details: error instanceof Error ? error.stack : undefined,
      context: { operation: 'load_recent' },
      retryable: true
    });
  }

  static handleRecentReposEmpty() {
    useErrorStore.getState().showToast(
      ErrorFormatter.formatInfo('No recently opened repositories found')
    );
  }

  // Repository switching errors
  static handleSwitchError(path: string, error: unknown) {
    const formatted = ErrorFormatter.format(error);
    useErrorStore.getState().addError({
      severity: ErrorSeverity.ERROR,
      title: 'Failed to Switch Repository',
      message: `Could not switch to repository at "${path}": ${formatted.message}`,
      details: error instanceof Error ? error.stack : undefined,
      context: { path, operation: 'switch' },
      retryable: true
    });
  }

  static handleSwitchSuccess(path: string) {
    useErrorStore.getState().showToast(
      ErrorFormatter.formatSuccess(`Switched to repository: ${path}`)
    );
  }

  // General repository validation
  static handleInvalidRepository(path: string, reason: string) {
    useErrorStore.getState().addError({
      severity: ErrorSeverity.ERROR,
      title: 'Invalid Repository',
      message: `The directory "${path}" is not a valid Git repository: ${reason}`,
      context: { path, operation: 'validate' },
      retryable: false
    });
  }

  static handleCorruptedRepository(path: string) {
    useErrorStore.getState().addError({
      severity: ErrorSeverity.ERROR,
      title: 'Corrupted Repository',
      message: `The Git repository at "${path}" appears to be corrupted and cannot be opened. Please repair it using Git tools before retrying.`,
      context: { path, operation: 'validate' },
      retryable: false,
      dismissible: true
    });
  }

  // Generic repository operation error
  static handleOperationError(operation: string, error: unknown, context?: Record<string, any>) {
    const formatted = ErrorFormatter.format(error);
    useErrorStore.getState().addError({
      severity: ErrorSeverity.ERROR,
      title: 'Repository Operation Failed',
      message: `${operation} failed: ${formatted.message}`,
      details: error instanceof Error ? error.stack : undefined,
      context: { operation, ...context },
      retryable: true
    });
  }
}

// ============================================================================
// Network Error Handler with Offline Support
// ============================================================================

import { offlineCache, OfflineDraft } from './offlineCache';

export class NetworkErrorHandler {
  /**
   * Handle network failures with offline draft preservation
   */
  static async handleWithOffline<T>(
    operation: () => Promise<T>,
    draftType: OfflineDraft['type'],
    draftData: any,
    system?: string,
    successMessage?: string
  ): Promise<{ result: T | null; isOffline: boolean; draftId?: string }> {
    // Check if online
    if (!offlineCache.isOnline()) {
      // Create offline draft
      const draft = offlineCache.createDraft(draftType, draftData, system);

      useErrorStore.getState().showToast({
        severity: ErrorSeverity.WARNING,
        title: 'Offline Mode',
        message: 'You are offline. The operation has been saved and will be synced when you are back online.',
        duration: 5000
      });

      return { result: null, isOffline: true, draftId: draft.id };
    }

    try {
      // Try the operation
      const result = await operation();

      if (successMessage) {
        useErrorStore.getState().showToast(
          ErrorFormatter.formatSuccess(successMessage)
        );
      }

      return { result, isOffline: false };
    } catch (error) {
      const formatted = ErrorFormatter.format(error);

      // Check if it's a network-related error
      const isNetworkError = formatted.message.toLowerCase().includes('network') ||
                            formatted.message.toLowerCase().includes('fetch') ||
                            formatted.message.toLowerCase().includes('connection') ||
                            formatted.message.toLowerCase().includes('timeout');

      if (isNetworkError) {
        // Create offline draft for retry
        const draft = offlineCache.createDraft(draftType, draftData, system);

        useErrorStore.getState().showToast({
          severity: ErrorSeverity.WARNING,
          title: 'Network Error',
          message: 'Network request failed. The operation has been saved as a draft and will be retried automatically.',
          duration: 5000,
          actions: [
            {
              label: 'Retry Now',
              action: () => {
                // Trigger retry of failed drafts
                offlineCache.retryFailedDrafts(async (draftToRetry) => {
                  if (draftToRetry.id === draft.id) {
                    // Re-attempt the original operation
                    try {
                      await operation();
                      offlineCache.markAsSynced(draft.id);
                      useErrorStore.getState().showToast(
                        ErrorFormatter.formatSuccess(successMessage || 'Operation completed successfully')
                      );
                    } catch (retryError) {
                      offlineCache.markAsFailed(draft.id, retryError instanceof Error ? retryError.message : String(retryError));
                    }
                  }
                });
              }
            }
          ]
        });

        return { result: null, isOffline: true, draftId: draft.id };
      }

      // Non-network error, handle normally
      useErrorStore.getState().showToast(
        ErrorFormatter.formatError(formatted.message)
      );

      return { result: null, isOffline: false };
    }
  }

  /**
   * Retry failed network operations
   */
  static async retryFailedOperations(
    retryFn: (draft: OfflineDraft) => Promise<void>
  ): Promise<void> {
    await offlineCache.retryFailedDrafts(retryFn);

    const stats = offlineCache.getStats();
    if (stats.failed > 0) {
      useErrorStore.getState().showToast({
        severity: ErrorSeverity.WARNING,
        title: 'Retry Complete',
        message: `${stats.failed} operation(s) still failed. They will be retried automatically.`,
        duration: 5000
      });
    } else {
      useErrorStore.getState().showToast({
        severity: ErrorSeverity.SUCCESS,
        title: 'Sync Complete',
        message: 'All offline operations have been successfully synced.',
        duration: 3000
      });
    }
  }

  /**
   * Monitor network status and show notifications
   */
  static setupNetworkMonitoring(): () => void {
    const cleanupOnline = offlineCache.onOnline(() => {
      useErrorStore.getState().showToast({
        severity: ErrorSeverity.SUCCESS,
        title: 'Back Online',
        message: 'Connection restored. Syncing pending operations...',
        duration: 3000
      });
    });

    const cleanupOffline = offlineCache.onOffline(() => {
      useErrorStore.getState().showToast({
        severity: ErrorSeverity.WARNING,
        title: 'Offline Mode',
        message: 'You are now offline. Operations will be saved as drafts.',
        duration: 5000
      });
    });

    // Return cleanup function
    return () => {
      cleanupOnline();
      cleanupOffline();
    };
  }

  /**
   * Get offline draft statistics for UI display
   */
  static getOfflineStats() {
    return offlineCache.getStats();
  }

  /**
   * Clear all offline drafts
   */
  static clearOfflineDrafts() {
    offlineCache.clearDrafts();
    useErrorStore.getState().showToast({
      severity: ErrorSeverity.INFO,
      title: 'Drafts Cleared',
      message: 'All offline drafts have been cleared.',
      duration: 3000
    });
  }
}

// ============================================================================// Export convenience functions
// ============================================================================

export const showSuccess = (message: string) => {
  useErrorStore.getState().showToast(ErrorFormatter.formatSuccess(message));
};

export const showInfo = (message: string) => {
  useErrorStore.getState().showToast(ErrorFormatter.formatInfo(message));
};

export const showWarning = (message: string) => {
  useErrorStore.getState().showToast(ErrorFormatter.formatWarning(message));
};

export const showError = (message: string, title?: string) => {
  const formatted = ErrorFormatter.formatError(message);
  if (title) {
    formatted.title = title;
  }
  useErrorStore.getState().showToast(formatted);
};

export const handleAsyncError = IPCErrorHandler.handle;
export const handleAsyncErrorWithToast = IPCErrorHandler.handleWithToast;
