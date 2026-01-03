// Comment Service for Gerrit integration
import { invoke } from '@tauri-apps/api/tauri';

export interface CreateCommentParams {
  changeId: string;
  filePath: string;
  line: number;
  message: string;
}

export interface CreateCommentResult {
  id: string;
  success: boolean;
  message: string;
}

export interface SimpleComment {
  id: string;
  changeId: string;
  filePath: string;
  line: number;
  message: string;
  author: string;
  created: string;
  unresolved: boolean;
}

export interface GetCommentsResult {
  comments: SimpleComment[];
  totalCount: number;
  success: boolean;
}

/**
 * Comment Service for managing Gerrit comments
 */
export class CommentService {
  /**
   * Create a new comment
   */
  async createComment(params: CreateCommentParams): Promise<CreateCommentResult> {
    try {
      console.log('CommentService: Creating comment', params);
      
      const result = await invoke<CreateCommentResult>('gerrit_create_comment_simple', {
        changeId: params.changeId,
        filePath: params.filePath,
        line: params.line,
        message: params.message,
      });
      
      console.log('CommentService: Comment created', result);
      return result;
    } catch (error) {
      console.error('CommentService: Failed to create comment', error);
      throw new Error('Failed to create comment: ' + (error as Error).message);
    }
  }

  /**
   * Get all comments for a change
   */
  async getComments(changeId: string): Promise<GetCommentsResult> {
    try {
      console.log('CommentService: Getting comments for change', changeId);
      
      const result = await invoke<GetCommentsResult>('gerrit_get_comments_simple', {
        changeId: changeId,
      });
      
      console.log('CommentService: Retrieved comments', result.totalCount);
      return result;
    } catch (error) {
      console.error('CommentService: Failed to get comments', error);
      throw new Error('Failed to get comments: ' + (error as Error).message);
    }
  }

  /**
   * Get unresolved comments for a change
   */
  async getUnresolvedComments(changeId: string): Promise<SimpleComment[]> {
    try {
      const result = await this.getComments(changeId);
      const unresolved = result.comments.filter(c => c.unresolved);
      console.log('CommentService: Found', unresolved.length, 'unresolved comments');
      return unresolved;
    } catch (error) {
      console.error('CommentService: Failed to get unresolved comments', error);
      return [];
    }
  }

  /**
   * Resolve a comment locally
   */
  async resolveComment(commentId: string): Promise<void> {
    console.log('CommentService: Resolving comment', commentId);
    // This would be implemented with a Tauri command
    // For now, we'll just update local state
  }

  /**
   * Delete a comment
   */
  async deleteComment(commentId: string): Promise<void> {
    console.log('CommentService: Deleting comment', commentId);
    // This would be implemented with a Tauri command
    // For now, we'll just update local state
  }
}

// Export singleton instance
export const commentService = new CommentService();

export default commentService;
