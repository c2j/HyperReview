import { useState, useEffect, useCallback, useRef } from 'react';
import { useApiClient } from '../api/client';
import type { Comment } from '../api/types';

interface UseCommentsOptions {
  filePath: string;
  pollInterval?: number; // milliseconds
  enabled?: boolean;
}

interface UseCommentsResult {
  comments: Comment[];
  loading: boolean;
  error: string | null;
  addComment: (lineNumber: number, content: string) => Promise<Comment | null>;
  updateComment: (commentId: string, content: string) => Promise<Comment | null>;
  deleteComment: (commentId: string) => Promise<boolean>;
  refreshComments: () => Promise<void>;
}

/**
 * Hook for managing comments with real-time updates via polling
 * Polls for new comments at regular intervals
 */
export function useComments({
  filePath,
  pollInterval = 5000, // 5 seconds default
  enabled = true
}: UseCommentsOptions): UseCommentsResult {
  const { addComment: addCommentApi, updateComment: updateCommentApi, deleteComment: deleteCommentApi, getComments } = useApiClient();

  const [comments, setComments] = useState<Comment[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const pollIntervalRef = useRef<NodeJS.Timeout | null>(null);
  const isMountedRef = useRef(true);

  // Fetch comments from the backend
  const fetchComments = useCallback(async () => {
    if (!filePath || !enabled) return;

    try {
      setLoading(true);
      setError(null);

      const fetchedComments = await getComments(filePath);

      if (isMountedRef.current) {
        setComments(fetchedComments);
      }
    } catch (err) {
      if (isMountedRef.current) {
        setError(err instanceof Error ? err.message : 'Failed to fetch comments');
      }
    } finally {
      if (isMountedRef.current) {
        setLoading(false);
      }
    }
  }, [filePath, enabled, getComments]);

  // Add a new comment
  const addComment = useCallback(async (
    lineNumber: number,
    content: string
  ): Promise<Comment | null> => {
    try {
      const newComment = await addCommentApi(filePath, lineNumber, content);

      // Refresh comments after adding
      await fetchComments();

      return newComment;
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to add comment');
      return null;
    }
  }, [filePath, addCommentApi, fetchComments]);

  // Update an existing comment
  const updateComment = useCallback(async (
    commentId: string,
    content: string
  ): Promise<Comment | null> => {
    try {
      const updatedComment = await updateCommentApi(commentId, content);

      // Refresh comments after updating
      await fetchComments();

      return updatedComment;
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to update comment');
      return null;
    }
  }, [updateCommentApi, fetchComments]);

  // Delete a comment
  const deleteComment = useCallback(async (
    commentId: string
  ): Promise<boolean> => {
    try {
      const success = await deleteCommentApi(commentId);

      if (success) {
        // Refresh comments after deleting
        await fetchComments();
      }

      return success;
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to delete comment');
      return false;
    }
  }, [deleteCommentApi, fetchComments]);

  // Set up polling for real-time updates
  useEffect(() => {
    if (!enabled) return;

    // Initial fetch
    fetchComments();

    // Set up polling
    pollIntervalRef.current = setInterval(() => {
      fetchComments();
    }, pollInterval);

    // Cleanup on unmount or when dependencies change
    return () => {
      if (pollIntervalRef.current) {
        clearInterval(pollIntervalRef.current);
        pollIntervalRef.current = null;
      }
    };
  }, [enabled, pollInterval, fetchComments]);

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      isMountedRef.current = false;
      if (pollIntervalRef.current) {
        clearInterval(pollIntervalRef.current);
      }
    };
  }, []);

  return {
    comments,
    loading,
    error,
    addComment,
    updateComment,
    deleteComment,
    refreshComments: fetchComments
  };
}