import { invoke } from '@tauri-apps/api/tauri';

export type CommandResult<T> = {
  success: true;
  data: T;
};

export type CommandError = {
  success: false;
  error: string;
  code?: string;
};

export type ApiResponse<T> = CommandResult<T> | CommandError;

export function isCommandResult<T>(result: ApiResponse<T>): result is CommandResult<T> {
  return result.success === true;
}

export function isCommandError(result: ApiResponse<unknown>): result is CommandError {
  return result.success === false;
}

class IPCError extends Error {
  constructor(message: string, public command: string) {
    super(message);
    this.name = 'IPCError';
  }
}

class IPCCommandNotFoundError extends IPCError {
  constructor(command: string, message: string) {
    super(`Command '${command}' not implemented: ${message}`, command);
    this.name = 'IPCCommandNotFoundError';
  }
}

class TauriClient {
  async invokeCommand<T>(command: string, args?: any): Promise<T> {
    try {
      const result = await invoke<ApiResponse<T>>(command, args);

      if (isCommandError(result)) {
        throw new IPCError(result.error, command);
      }

      if (isCommandResult(result)) {
        return result.data;
      }

      throw new IPCError('Unknown response format', command);
    } catch (error) {
      const err = error as any;

      if (err?.message?.includes('command not found') ||
          err?.message?.includes('handler not found')) {
        throw new IPCCommandNotFoundError(command, err.message);
      }

      throw new IPCError(err?.message || 'Unknown IPC error', command);
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

  async conditionalInvoke<T>(
    command: string,
    args?: any,
    options: {
      fallback?: () => Promise<T>;
      defaultValue?: T;
      showMessage?: boolean;
    } = {}
  ): Promise<T> {
    try {
      return await this.invokeCommand<T>(command, args);
    } catch (error) {
      const err = error as IPCError;

      if (err instanceof IPCCommandNotFoundError) {
        if (options.fallback) {
          console.warn(`Using fallback for ${command}`);
          return await options.fallback();
        }

        if (options.defaultValue !== undefined) {
          console.warn(`Using default value for ${command}`);
          return options.defaultValue;
        }

        if (options.showMessage !== false) {
          console.error(`Feature '${command}' is not available in this version`);
        }

        throw err;
      }

      throw error;
    }
  }

  async getRecentRepos() {
    return this.invokeCommand<any[]>('get_recent_repos');
  }

  async getBranches() {
    return this.invokeCommand<any[]>('get_branches');
  }

  async loadRepo(path: string) {
    return this.invokeCommand<any>('load_repo', { path });
  }

  async getFileDiff(fileId: string, base?: string, head?: string) {
    return this.invokeCommand<any[]>('get_file_diff', { fileId, base, head });
  }

  async addComment(
    taskId: string,
    filePath: string,
    content: string,
    lineNumber?: number,
    type?: string,
    severity?: string,
    tags?: string[]
  ) {
    return this.invokeCommand<any>('add_comment', {
      taskId,
      filePath,
      lineNumber,
      content,
      type,
      severity,
      tags
    });
  }

  async updateComment(commentId: string, content: string, severity?: string, tags?: string[]) {
    return this.invokeCommand<boolean>('update_comment', {
      commentId,
      content,
      severity,
      tags
    });
  }

  async deleteComment(commentId: string) {
    return this.invokeCommand<boolean>('delete_comment', { commentId });
  }

  async getComments(taskId: string) {
    return this.invokeCommand<any[]>('get_comments', { taskId });
  }

  async getHeatmap() {
    return this.invokeCommand<any[]>('get_heatmap');
  }

  async getFileTree() {
    return this.invokeCommand<any[]>('get_file_tree');
  }

  async getChecklist() {
    return this.invokeCommand<any[]>('get_checklist');
  }

  async getBlame(fileId: string, lineNumber?: number) {
    return this.invokeCommand<any>('get_blame', { fileId, lineNumber });
  }

  async readFileContent(filePath: string) {
    return this.invokeCommand<string>('read_file_content', { filePath });
  }

  async createLocalTask(name: string, repoPath: string, baseRef: string, itemsText: string) {
    return this.invokeCommand<any>('create_task', {
      payload: {
        name,
        repoPath,
        baseRef,
        itemsText
      }
    });
  }

  async listTasks(status?: 'all' | 'active' | 'pending' | 'completed') {
    return this.invokeCommand<any[]>('list_tasks', { status });
  }

  async getReviewTemplates() {
    return this.invokeCommand<any[]>('get_review_templates');
  }

  async createTemplate(title: string, category: string, content: string, tags?: string[]) {
    return this.invokeCommand<any>('create_template', { title, category, content, tags });
  }

  async saveUserSetting(key: string, value: any) {
    return this.invokeCommand<boolean>('save_user_setting', { key, value });
  }

  async getUserSetting(key: string) {
    return this.invokeCommand<any | null>('get_user_setting', { key });
  }

  async gerritGetInstancesSimple() {
    return this.conditionalInvoke<any[]>(
      'gerrit_get_instances_simple',
      undefined,
      { defaultValue: [] }
    );
  }

  async gerritCreateInstanceSimple(name: string, url: string, username: string, password: string) {
    return this.conditionalInvoke<any>(
      'gerrit_create_instance_simple',
      { name, url, username, password }
    );
  }

  async gerritSearchChangesSimple(query: string, status?: string, limit?: number) {
    return this.conditionalInvoke<any[]>(
      'gerrit_search_changes_simple',
      { query, status, limit },
      { defaultValue: [] }
    );
  }

  async gerritImportChangeSimple(changeId: string, downloadFiles: boolean = true) {
    return this.conditionalInvoke<any>(
      'gerrit_import_change_simple',
      { changeId, downloadFiles }
    );
  }
}

export const tauriClient = new TauriClient();
