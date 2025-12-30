import { create } from 'zustand';
import type { LocalTask, TaskSummary } from '../types/task';
import { useLocalTasks } from '../hooks/useLocalTasks';

interface TaskStore {
  tasks: TaskSummary[];
  currentTask: LocalTask | null;
  isLoading: boolean;
  error: string | null;
  fetchTasks: () => Promise<void>;
  fetchTask: (taskId: string) => Promise<void>;
  setCurrentTask: (task: LocalTask | null) => void;
}

export const useTaskStore = create<TaskStore>((set) => ({
  tasks: [],
  currentTask: null,
  isLoading: false,
  error: null,

  fetchTasks: async () => {
    set({ isLoading: true, error: null });
    try {
      const { listTasks } = useLocalTasks();
      const tasks = await listTasks();
      set({ tasks, isLoading: false });
    } catch (error) {
      set({ error: String(error), isLoading: false });
    }
  },

  fetchTask: async (taskId: string) => {
    set({ isLoading: true, error: null });
    try {
      const { getTask } = useLocalTasks();
      const task = await getTask(taskId);
      set({ currentTask: task, isLoading: false });
    } catch (error) {
      set({ error: String(error), isLoading: false });
    }
  },

  setCurrentTask: (task: LocalTask | null) => {
    set({ currentTask: task });
  },
}));
