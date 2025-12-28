import { invoke } from '@tauri-apps/api/tauri';
import type { LocalTask, TaskItem, TaskSummary } from '@types/task';

export function useLocalTasks() {
  const createTask = async (
    name: string,
    repoPath: string,
    baseRef: string,
    itemsText: string,
  ): Promise<LocalTask> => {
    return await invoke<LocalTask>('create_task', {
      payload: {
        name,
        repoPath,
        baseRef,
        itemsText,
      },
    });
  };

  const listTasks = async (): Promise<TaskSummary[]> => {
    return await invoke<TaskSummary[]>('list_tasks');
  };

  const getTask = async (taskId: string): Promise<LocalTask> => {
    return await invoke<LocalTask>('get_task', { taskId });
  };

  const updateProgress = async (
    taskId: string,
    itemIndex: number,
    reviewed: boolean,
  ): Promise<void> => {
    await invoke('update_task_progress', { taskId, itemIndex, reviewed });
  };

  const deleteTask = async (taskId: string): Promise<void> => {
    await invoke('delete_task', { taskId });
  };

  const archiveTask = async (taskId: string): Promise<void> => {
    await invoke('archive_task', { taskId });
  };

  const reimportTaskText = async (taskId: string, itemsText: string): Promise<LocalTask> => {
    return await invoke<LocalTask>('reimport_task_text', { taskId, itemsText });
  };

  const updateTask = async (
    taskId: string,
    name?: string,
    baseRef?: string,
  ): Promise<LocalTask> => {
    return await invoke<LocalTask>('update_task', {
      taskId,
      name,
      baseRef,
    });
  };

  const exportTask = async (taskId: string): Promise<string> => {
    return await invoke<string>('export_task', { taskId });
  };

  const exportAllTasks = async (): Promise<string> => {
    return await invoke<string>('export_all_tasks');
  };

  const submitToGerrit = async (
    taskId: string,
    gerritUrl: string,
    username: string,
    changeId: string,
    revisionId: string,
    score?: number,
  ): Promise<any> => {
    return await invoke('submit_task_to_gerrit', {
      taskId,
      gerrit_url: gerritUrl,
      username,
      change_id: changeId,
      revision_id: revisionId,
      score,
    });
  };

  const submitToCodeArts = async (
    taskId: string,
    projectId: string,
    mrId: number,
    approval?: string,
  ): Promise<any> => {
    return await invoke('submit_task_to_codearts', {
      taskId,
      projectId,
      mrId,
      approval,
    });
  };

  const submitToCustomApi = async (
    taskId: string,
    endpoint: string,
    method: string,
    apiUrl: string,
  ): Promise<any> => {
    return await invoke('submit_task_to_custom_api', {
      taskId,
      endpoint,
      method,
      apiUrl,
    });
  };

  return {
    createTask,
    listTasks,
    getTask,
    updateProgress,
    deleteTask,
    archiveTask,
    reimportTaskText,
    updateTask,
    exportTask,
    exportAllTasks,
    submitToGerrit,
    submitToCodeArts,
    submitToCustomApi,
  };
}
