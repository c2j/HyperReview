import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import TaskProgress from '../../components/TaskProgress';

describe('TaskProgress Component', () => {
  describe('Basic Rendering', () => {
    it('should render progress bar with correct percentage', () => {
      render(
        <TaskProgress 
          completed={5} 
          total={10} 
          status="in_progress" 
        />
      );

      expect(screen.getByText('5/10 (50%)')).toBeInTheDocument();
    });

    it('should render in_progress status with clock icon', () => {
      render(
        <TaskProgress 
          completed={3} 
          total={8} 
          status="in_progress" 
        />
      );

      const icon = screen.getByRole('img', { hidden: true });
      expect(icon).toBeInTheDocument();
      expect(icon).toHaveClass('text-orange-500');
    });

    it('should render completed status with check icon', () => {
      render(
        <TaskProgress 
          completed={10} 
          total={10} 
          status="completed" 
        />
      );

      const icon = screen.getByRole('img', { hidden: true });
      expect(icon).toBeInTheDocument();
      expect(icon).toHaveClass('text-green-500');
    });

    it('should render archived status with archive icon', () => {
      render(
        <TaskProgress 
          completed={7} 
          total={10} 
          status="archived" 
        />
      );

      const icon = screen.getByRole('img', { hidden: true });
      expect(icon).toBeInTheDocument();
      expect(icon).toHaveClass('text-gray-500');
    });
  });

  describe('Size Variations', () => {
    it('should render small size progress bar', () => {
      const { container } = render(
        <TaskProgress 
          completed={3} 
          total={5} 
          status="in_progress" 
          size="sm"
        />
      );

      const progressBar = container.querySelector('.h-1');
      expect(progressBar).toBeInTheDocument();
    });

    it('should render medium size progress bar (default)', () => {
      const { container } = render(
        <TaskProgress 
          completed={3} 
          total={5} 
          status="in_progress" 
        />
      );

      const progressBar = container.querySelector('.h-2');
      expect(progressBar).toBeInTheDocument();
    });

    it('should render large size progress bar', () => {
      const { container } = render(
        <TaskProgress 
          completed={3} 
          total={5} 
          status="in_progress" 
          size="lg"
        />
      );

      const progressBar = container.querySelector('.h-3');
      expect(progressBar).toBeInTheDocument();
    });
  });

  describe('Progress Calculations', () => {
    it('should calculate 0% when no items are completed', () => {
      render(
        <TaskProgress 
          completed={0} 
          total={10} 
          status="in_progress" 
        />
      );

      expect(screen.getByText('0/10 (0%)')).toBeInTheDocument();
    });

    it('should calculate 100% when all items are completed', () => {
      render(
        <TaskProgress 
          completed={10} 
          total={10} 
          status="completed" 
        />
      );

      expect(screen.getByText('10/10 (100%)')).toBeInTheDocument();
    });

    it('should handle division by zero gracefully', () => {
      render(
        <TaskProgress 
          completed={0} 
          total={0} 
          status="in_progress" 
        />
      );

      expect(screen.getByText('0/0 (0%)')).toBeInTheDocument();
    });

    it('should round percentage correctly', () => {
      render(
        <TaskProgress 
          completed={3} 
          total={7} 
          status="in_progress" 
        />
      );

      expect(screen.getByText('3/7 (43%)')).toBeInTheDocument(); // 3/7 = 42.857...% rounds to 43%
    });

    it('should handle large numbers correctly', () => {
      render(
        <TaskProgress 
          completed={150} 
          total={200} 
          status="in_progress" 
        />
      );

      expect(screen.getByText('150/200 (75%)')).toBeInTheDocument();
    });
  });

  describe('Progress Bar Styling', () => {
    it('should apply correct color class for in_progress status', () => {
      const { container } = render(
        <TaskProgress 
          completed={5} 
          total={10} 
          status="in_progress" 
        />
      );

      const progressFill = container.querySelector('.bg-orange-500');
      expect(progressFill).toBeInTheDocument();
    });

    it('should apply correct color class for completed status', () => {
      const { container } = render(
        <TaskProgress 
          completed={10} 
          total={10} 
          status="completed" 
        />
      );

      const progressFill = container.querySelector('.bg-green-500');
      expect(progressFill).toBeInTheDocument();
    });

    it('should apply correct color class for archived status', () => {
      const { container } = render(
        <TaskProgress 
          completed={7} 
          total={10} 
          status="archived" 
        />
      );

      const progressFill = container.querySelector('.bg-gray-500');
      expect(progressFill).toBeInTheDocument();
    });

    it('should set correct width style for progress fill', () => {
      const { container } = render(
        <TaskProgress 
          completed={7} 
          total={10} 
          status="in_progress" 
        />
      );

      const progressFill = container.querySelector('.bg-orange-500');
      expect(progressFill).toHaveStyle({ width: '70%' });
    });

    it('should have transition class for smooth animations', () => {
      const { container } = render(
        <TaskProgress 
          completed={5} 
          total={10} 
          status="in_progress" 
        />
      );

      const progressFill = container.querySelector('.transition-all');
      expect(progressFill).toBeInTheDocument();
    });
  });

  describe('Edge Cases', () => {
    it('should handle more completed than total items', () => {
      render(
        <TaskProgress 
          completed={15} 
          total={10} 
          status="in_progress" 
        />
      );

      expect(screen.getByText('15/10 (150%)')).toBeInTheDocument();
    });

    it('should handle negative numbers gracefully', () => {
      render(
        <TaskProgress 
          completed={-5} 
          total={10} 
          status="in_progress" 
        />
      );

      expect(screen.getByText('-5/10 (-50%)')).toBeInTheDocument();
    });

    it('should handle decimal numbers', () => {
      render(
        <TaskProgress 
          completed={2.5} 
          total={5} 
          status="in_progress" 
        />
      );

      expect(screen.getByText('2.5/5 (50%)')).toBeInTheDocument();
    });
  });

  describe('Accessibility', () => {
    it('should have proper text contrast for readability', () => {
      render(
        <TaskProgress 
          completed={5} 
          total={10} 
          status="in_progress" 
        />
      );

      const percentageText = screen.getByText('5/10 (50%)');
      expect(percentageText).toHaveClass('text-editor-fg/70');
    });

    it('should maintain consistent spacing between elements', () => {
      const { container } = render(
        <TaskProgress 
          completed={5} 
          total={10} 
          status="in_progress" 
        />
      );

      const wrapper = container.firstChild;
      expect(wrapper).toHaveClass('gap-2');
    });
  });
});