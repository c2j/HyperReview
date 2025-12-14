import { invoke } from '@tauri-apps/api/core';
import { useCallback } from 'react';

// Generic IPC hook
function useIPC<T, R>(command: string) {
  const call = useCallback(async (args?: T): Promise<R> => {
    try {
      const result = await invoke<R>(command, (args as any) || {});
      return result;
    } catch (error) {
      console.error(`IPC call failed for ${command}:`, error);
      throw error;
    }
  }, [command]);

  return call;
}

// Repository hooks
export const useOpenRepoDialog = () => {
  return useCallback(async (): Promise<string | null> => {
    try {
      // In Tauri v2, use invoke to call the backend dialog command
      // The backend will use the OS native dialog
      const result = await invoke<string | null>('open_repo_dialog_frontend');
      return result;
    } catch (error) {
      console.error('Failed to open repository dialog:', error);
      throw error;
    }
  }, []);
};

export const useGetRecentRepos = () => {
  return useIPC<[], any[]>('get_recent_repos');
};

export const useGetBranches = () => {
  return useIPC<[], any[]>('get_branches');
};

export const useLoadRepo = () => {
  return useIPC<{ path: string }, any>('load_repo');
};

// Diff and comment hooks
export const useGetFileDiff = () => {
  return useIPC<{ params: { file_path: string; old_commit?: string; new_commit?: string } }, any[]>(
    'get_file_diff'
  );
};

export const useAddComment = () => {
  return useIPC<{ params: { file_path: string; line_number: number; content: string } }, any>(
    'add_comment'
  );
};

export const useUpdateComment = () => {
  return useIPC<{ params: { comment_id: string; content: string } }, any>('update_comment');
};

export const useDeleteComment = () => {
  return useIPC<{ comment_id: string }, boolean>('delete_comment');
};

export const useGetComments = () => {
  return useIPC<{ file_path: string }, any[]>('get_comments');
};

// Task management hooks
export const useGetTasks = () => {
  return useIPC<[], any[]>('get_tasks');
};

export const useGetReviewStats = () => {
  return useIPC<[], any>('get_review_stats');
};

export const useGetQualityGates = () => {
  return useIPC<[], any[]>('get_quality_gates');
};

export const useGetReviewTemplates = () => {
  return useIPC<[], any[]>('get_review_templates');
};

export const useCreateTemplate = () => {
  return useIPC<{ name: string; content: string }, any>('create_template');
};

// Analysis hooks
export const useGetHeatmap = () => {
  return useIPC<[], any[]>('get_heatmap');
};

export const useGetChecklist = () => {
  return useIPC<{ file_path: string }, any[]>('get_checklist');
};

export const useGetBlame = () => {
  return useIPC<{ file_path: string; commit?: string }, any>('get_blame');
};

export const useReadFileContent = () => {
  return useIPC<{ params: { file_path: string } }, string>('read_file_content');
};

export const useAnalyzeComplexity = () => {
  return useIPC<{ file_path: string }, any>('analyze_complexity');
};

export const useScanSecurity = () => {
  return useIPC<{ file_path: string }, any[]>('scan_security');
};

// External integration hooks
export const useSubmitReview = () => {
  return useIPC<{ system: string; review_data: any }, any>('submit_review');
};

export const useSyncRepo = () => {
  return useIPC<[], any>('sync_repo');
};

// Search and configuration hooks
export const useSearch = () => {
  return useIPC<{ query: string }, any[]>('search');
};

export const useGetCommands = () => {
  return useIPC<[], any[]>('get_commands');
};

export const useGetTags = () => {
  return useIPC<[], any[]>('get_tags');
};

export const useCreateTag = () => {
  return useIPC<{ label: string; color: string }, any>('create_tag');
};
