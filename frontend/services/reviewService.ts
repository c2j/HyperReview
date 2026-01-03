// Review Service for Gerrit integration
import { invoke } from '@tauri-apps/api/tauri';

export interface Label {
  name: string;
  value: number;
}

export interface SubmitReviewParams {
  changeId: string;
  patchSetNumber: number;
  message: string;
  labels: Label[];
  commentIds: string[];
  draft?: boolean;
}

export interface SubmitReviewResult {
  reviewId: string;
  submittedComments: number;
  submittedLabels: Record<string, number>;
  success: boolean;
  message: string;
}

/**
 * Review Service for submitting reviews to Gerrit
 */
export class ReviewService {
  /**
   * Submit a complete review with comments and labels
   */
  async submitReview(params: SubmitReviewParams): Promise<SubmitReviewResult> {
    try {
      console.log('ReviewService: Submitting review', params);
      
      const labelsMap: Record<string, number> = {};
      params.labels.forEach(label => {
        labelsMap[label.name] = label.value;
      });
      
      const result = await invoke<SubmitReviewResult>('gerrit_submit_review_simple', {
        changeId: params.changeId,
        patchSetNumber: params.patchSetNumber,
        message: params.message || '',
        labels: labelsMap,
        commentIds: params.commentIds,
      });
      
      console.log('ReviewService: Review submitted', result);
      return result;
    } catch (error) {
      console.error('ReviewService: Failed to submit review', error);
      throw new Error('Failed to submit review: ' + (error as Error).message);
    }
  }

  /**
   * Get default labels for Code Review
   */
  getCodeReviewLabels(): Label[] {
    return [
      { name: 'Code-Review', value: -2 },
      { name: 'Code-Review', value: -1 },
      { name: 'Code-Review', value: 0 },
      { name: 'Code-Review', value: 1 },
      { name: 'Code-Review', value: 2 },
    ];
  }

  /**
   * Get default labels for Verified
   */
  getVerifiedLabels(): Label[] {
    return [
      { name: 'Verified', value: -1 },
      { name: 'Verified', value: 0 },
      { name: 'Verified', value: 1 },
    ];
  }

  /**
   * Format review message
   */
  formatReviewMessage(message: string, commentCount: number): string {
    if (!message) {
      return `Reviewed with ${commentCount} comment${commentCount !== 1 ? 's' : ''}`;
    }
    return message;
  }

  /**
   * Validate review before submission
   */
  validateReview(params: SubmitReviewParams): { valid: boolean; errors: string[] } {
    const errors: string[] = [];

    if (!params.changeId) {
      errors.push('Change ID is required');
    }

    if (!params.message && params.commentIds.length === 0) {
      errors.push('Either a message or comments are required');
    }

    if (params.labels.length === 0 && params.commentIds.length === 0) {
      errors.push('At least one label or comment is required');
    }

    return {
      valid: errors.length === 0,
      errors,
    };
  }
}

// Export singleton instance
export const reviewService = new ReviewService();

export default reviewService;
