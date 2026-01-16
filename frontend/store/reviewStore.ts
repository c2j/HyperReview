import { create } from 'zustand';
import type {
  Repository,
  Branch,
  DiffLine,
  Comment,
  Task,
  ReviewStats,
  HeatmapItem,
  ChecklistItem,
  Tag,
  SearchResult
} from '../api/types';

// Repository state
interface RepositoryState {
  currentRepo: Repository | null;
  branches: Branch[];
  recentRepos: Repository[];
  selectedBaseBranch: string | null;
  selectedHeadBranch: string | null;
  loading: boolean;
  error: string | null;

  // Actions
  setCurrentRepo: (repo: Repository | null) => void;
  setBranches: (branches: Branch[]) => void;
  setRecentRepos: (repos: Repository[]) => void;
  setSelectedBaseBranch: (branch: string | null) => void;
  setSelectedHeadBranch: (branch: string | null) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  clearError: () => void;
}

// Diff and review state
interface ReviewState {
  currentDiff: DiffLine[];
  comments: Comment[];
  selectedFile: string | null;
  loading: boolean;
  error: string | null;

  // Actions
  setCurrentDiff: (diff: DiffLine[]) => void;
  setComments: (comments: Comment[]) => void;
  addComment: (comment: Comment) => void;
  updateComment: (id: string, updates: Partial<Comment>) => void;
  removeComment: (id: string) => void;
  setSelectedFile: (file: string | null) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  clearError: () => void;
}

// Task and analytics state
interface TaskState {
  tasks: Task[];
  reviewStats: ReviewStats | null;
  heatmap: HeatmapItem[];
  checklist: ChecklistItem[];
  loading: boolean;
  error: string | null;

  // Actions
  setTasks: (tasks: Task[]) => void;
  addTask: (task: Task) => void;
  updateTask: (id: string, updates: Partial<Task>) => void;
  removeTask: (id: string) => void;
  setReviewStats: (stats: ReviewStats) => void;
  setHeatmap: (heatmap: HeatmapItem[]) => void;
  setChecklist: (checklist: ChecklistItem[]) => void;
  updateChecklistItem: (id: string, updates: Partial<ChecklistItem>) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  clearError: () => void;
}

// Search and tags state
interface SearchState {
  searchResults: SearchResult[];
  tags: Tag[];
  loading: boolean;
  error: string | null;

  // Actions
  setSearchResults: (results: SearchResult[]) => void;
  setTags: (tags: Tag[]) => void;
  addTag: (tag: Tag) => void;
  updateTag: (id: string, updates: Partial<Tag>) => void;
  removeTag: (id: string) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  clearError: () => void;
}

// Combined store interface
interface ReviewStore extends RepositoryState, ReviewState, TaskState, SearchState {
  // Global actions
  reset: () => void;
  resetAll: () => void;
}

// Repository store
export const useRepositoryStore = create<RepositoryState>((set) => ({
  currentRepo: null,
  branches: [],
  recentRepos: [],
  selectedBaseBranch: null,
  selectedHeadBranch: null,
  loading: false,
  error: null,

  setCurrentRepo: (repo) => set({ currentRepo: repo }),
  setBranches: (branches) => set({ branches }),
  setRecentRepos: (repos) => set({ recentRepos: repos }),
  setSelectedBaseBranch: (branch) => set({ selectedBaseBranch: branch }),
  setSelectedHeadBranch: (branch) => set({ selectedHeadBranch: branch }),
  setLoading: (loading) => set({ loading }),
  setError: (error) => set({ error }),
  clearError: () => set({ error: null }),
}));

// Task store
export const useTaskStore = create<TaskState>((set) => ({
  tasks: [],
  reviewStats: null,
  heatmap: [],
  checklist: [],
  loading: false,
  error: null,

  setTasks: (tasks) => set({ tasks }),
  addTask: (task) => set((state) => ({ tasks: [...state.tasks, task] })),
  updateTask: (id, updates) =>
    set((state) => ({
      tasks: state.tasks.map((t) => (t.id === id ? { ...t, ...updates } : t)),
    })),
  removeTask: (id) =>
    set((state) => ({
      tasks: state.tasks.filter((t) => t.id !== id),
    })),
  setReviewStats: (stats) => set({ reviewStats: stats }),
  setHeatmap: (heatmap) => set({ heatmap }),
  setChecklist: (checklist) => set({ checklist }),
  updateChecklistItem: (id, updates) =>
    set((state) => ({
      checklist: state.checklist.map((item) =>
        item.id === id ? { ...item, ...updates } : item
      ),
    })),
  setLoading: (loading) => set({ loading }),
  setError: (error) => set({ error }),
  clearError: () => set({ error: null }),
}));

// Search store
export const useSearchStore = create<SearchState>((set) => ({
  searchResults: [],
  tags: [],
  loading: false,
  error: null,

  setSearchResults: (results) => set({ searchResults: results }),
  setTags: (tags) => set({ tags }),
  addTag: (tag) => set((state) => ({ tags: [...state.tags, tag] })),
  updateTag: (id, updates) =>
    set((state) => ({
      tags: state.tags.map((t) => (t.id === id ? { ...t, ...updates } : t)),
    })),
  removeTag: (id) =>
    set((state) => ({
      tags: state.tags.filter((t) => t.id !== id),
    })),
  setLoading: (loading) => set({ loading }),
  setError: (error) => set({ error }),
  clearError: () => set({ error: null }),
}));

// Combined store hook
export const useReviewStore = create<ReviewStore>((set) => ({
  // Repository state
  currentRepo: null,
  branches: [],
  recentRepos: [],
  selectedBaseBranch: null,
  selectedHeadBranch: null,
  loading: false,
  error: null,

  // Review state
  currentDiff: [],
  comments: [],
  selectedFile: null,

  // Task state
  tasks: [],
  reviewStats: null,
  heatmap: [],
  checklist: [],

  // Search state
  searchResults: [],
  tags: [],

  // Repository actions
  setCurrentRepo: (repo) => set({ currentRepo: repo }),
  setBranches: (branches) => set({ branches }),
  setRecentRepos: (repos) => set({ recentRepos: repos }),
  setSelectedBaseBranch: (branch) => set({ selectedBaseBranch: branch }),
  setSelectedHeadBranch: (branch) => set({ selectedHeadBranch: branch }),
  setLoading: (loading) => set({ loading }),
  setError: (error) => set({ error }),
  clearError: () => set({ error: null }),

  // Review actions
  setCurrentDiff: (diff) => set({ currentDiff: diff }),
  setComments: (comments) => set({ comments }),
  addComment: (comment) => set((state) => ({ comments: [...state.comments, comment] })),
  updateComment: (id, updates) =>
    set((state) => ({
      comments: state.comments.map((c) => (c.id === id ? { ...c, ...updates } : c)),
    })),
  removeComment: (id) =>
    set((state) => ({
      comments: state.comments.filter((c) => c.id !== id),
    })),
  setSelectedFile: (file) => set({ selectedFile: file }),

  // Task actions
  setTasks: (tasks) => set({ tasks }),
  addTask: (task) => set((state) => ({ tasks: [...state.tasks, task] })),
  updateTask: (id, updates) =>
    set((state) => ({
      tasks: state.tasks.map((t) => (t.id === id ? { ...t, ...updates } : t)),
    })),
  removeTask: (id) =>
    set((state) => ({
      tasks: state.tasks.filter((t) => t.id !== id),
    })),
  setReviewStats: (stats) => set({ reviewStats: stats }),
  setHeatmap: (heatmap) => set({ heatmap }),
  setChecklist: (checklist) => set({ checklist }),
  updateChecklistItem: (id, updates) =>
    set((state) => ({
      checklist: state.checklist.map((item) =>
        item.id === id ? { ...item, ...updates } : item
      ),
    })),

  // Search actions
  setSearchResults: (results) => set({ searchResults: results }),
  setTags: (tags) => set({ tags }),
  addTag: (tag) => set((state) => ({ tags: [...state.tags, tag] })),
  updateTag: (id, updates) =>
    set((state) => ({
      tags: state.tags.map((t) => (t.id === id ? { ...t, ...updates } : t)),
    })),
  removeTag: (id) =>
    set((state) => ({
      tags: state.tags.filter((t) => t.id !== id),
    })),

  // Global actions
  reset: () => set({ error: null }),
  resetAll: () =>
    set({
      currentRepo: null,
      branches: [],
      recentRepos: [],
      selectedBaseBranch: null,
      selectedHeadBranch: null,
      currentDiff: [],
      comments: [],
      selectedFile: null,
      tasks: [],
      reviewStats: null,
      heatmap: [],
      checklist: [],
      searchResults: [],
      tags: [],
      loading: false,
      error: null,
    }),
}));
