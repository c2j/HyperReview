import { create } from 'zustand';
import type {
  GerritInstance,
  GerritChange,
  GerritComment,
  GerritReview
} from '../api/types/gerrit';

// Gerrit Instances state
interface GerritInstanceState {
  instances: GerritInstance[];
  activeInstance: GerritInstance | null;
  loading: boolean;
  error: string | null;

  // Actions
  setInstances: (instances: GerritInstance[]) => void;
  setActiveInstance: (instance: GerritInstance | null) => void;
  addInstance: (instance: GerritInstance) => void;
  updateInstance: (id: string, updates: Partial<GerritInstance>) => void;
  removeInstance: (id: string) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  clearError: () => void;
}

// Gerrit Changes state
interface GerritChangeState {
  changes: GerritChange[];
  currentChange: GerritChange | null;
  loading: boolean;
  error: string | null;

  // Actions
  setChanges: (changes: GerritChange[]) => void;
  setCurrentChange: (change: GerritChange | null) => void;
  addChange: (change: GerritChange) => void;
  updateChange: (id: string, updates: Partial<GerritChange>) => void;
  removeChange: (id: string) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  clearError: () => void;
}

// Gerrit Comments state
interface GerritCommentState {
  comments: GerritComment[];
  loading: boolean;
  error: string | null;

  // Actions
  setComments: (comments: GerritComment[]) => void;
  addComment: (comment: GerritComment) => void;
  updateComment: (id: string, updates: Partial<GerritComment>) => void;
  removeComment: (id: string) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  clearError: () => void;
}

// Gerrit Reviews state
interface GerritReviewState {
  reviews: GerritReview[];
  currentReview: GerritReview | null;
  loading: boolean;
  error: string | null;

  // Actions
  setReviews: (reviews: GerritReview[]) => void;
  setCurrentReview: (review: GerritReview | null) => void;
  addReview: (review: GerritReview) => void;
  updateReview: (id: string, updates: Partial<GerritReview>) => void;
  removeReview: (id: string) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  clearError: () => void;
}

// Create stores
export const useGerritInstances = create<GerritInstanceState>((set) => ({
  instances: [],
  activeInstance: null,
  loading: false,
  error: null,

  setInstances: (instances) => set({ instances }),
  setActiveInstance: (activeInstance) => set({ activeInstance }),
  addInstance: (instance) => set((state) => ({ instances: [...state.instances, instance] })),
  updateInstance: (id, updates) => set((state) => ({
    instances: state.instances.map((inst) => (inst.id === id ? { ...inst, ...updates } : inst))
  })),
  removeInstance: (id) => set((state) => ({ instances: state.instances.filter((inst) => inst.id !== id) })),
  setLoading: (loading) => set({ loading }),
  setError: (error) => set({ error }),
  clearError: () => set({ error: null })
}));

export const useGerritChanges = create<GerritChangeState>((set) => ({
  changes: [],
  currentChange: null,
  loading: false,
  error: null,

  setChanges: (changes) => set({ changes }),
  setCurrentChange: (currentChange) => set({ currentChange }),
  addChange: (change) => set((state) => ({ changes: [...state.changes, change] })),
  updateChange: (id, updates) => set((state) => ({
    changes: state.changes.map((ch) => (ch.id === id ? { ...ch, ...updates } : ch))
  })),
  removeChange: (id) => set((state) => ({ changes: state.changes.filter((ch) => ch.id !== id) })),
  setLoading: (loading) => set({ loading }),
  setError: (error) => set({ error }),
  clearError: () => set({ error: null })
}));

export const useGerritComments = create<GerritCommentState>((set) => ({
  comments: [],
  loading: false,
  error: null,

  setComments: (comments) => set({ comments }),
  addComment: (comment) => set((state) => ({ comments: [...state.comments, comment] })),
  updateComment: (id, updates) => set((state) => ({
    comments: state.comments.map((com) => (com.id === id ? { ...com, ...updates } : com))
  })),
  removeComment: (id) => set((state) => ({ comments: state.comments.filter((com) => com.id !== id) })),
  setLoading: (loading) => set({ loading }),
  setError: (error) => set({ error }),
  clearError: () => set({ error: null })
}));

export const useGerritReviews = create<GerritReviewState>((set) => ({
  reviews: [],
  currentReview: null,
  loading: false,
  error: null,

  setReviews: (reviews) => set({ reviews }),
  setCurrentReview: (currentReview) => set({ currentReview }),
  addReview: (review) => set((state) => ({ reviews: [...state.reviews, review] })),
  updateReview: (id, updates) => set((state) => ({
    reviews: state.reviews.map((rev) => (rev.id === id ? { ...rev, ...updates } : rev))
  })),
  removeReview: (id) => set((state) => ({ reviews: state.reviews.filter((rev) => rev.id !== id) })),
  setLoading: (loading) => set({ loading }),
  setError: (error) => set({ error }),
  clearError: () => set({ error: null })
}));
