import { render, screen, fireEvent } from '@testing-library/react';
import '@testing-library/jest-dom';
import TaskPanel from '../../components/TaskPanel';
import { useTaskStore } from '../../store/taskStore';
import { TaskStatus } from '../../types/task';

// Mock the task store
jest.mock('../../store/taskStore');

const mockUseTaskStore = useTaskStore as jest.MockedFunction<typeof useTaskStore>;

describe('TaskPanel Component', () => {
  const mockFetchTasks = jest.fn();
  const mockOnTaskSelect = jest.fn();
  const mockOnCreateTask = jest.fn();

  const mockTasks = [
    {
      id: 'task-1',
      name: 'Review authentication module',
      status: 'in_progress' as TaskStatus,
      progress: 45,
      createdAt: '2024-01-15T10:00:00Z',
      updatedAt: '2024-01-15T14:30:00Z'
    },
    {
      id: 'task-2',
      name: 'Fix login validation',
      status: 'completed' as TaskStatus,
      progress: 100,
      createdAt: '2024-01-14T09:00:00Z',
      updatedAt: '2024-01-14T16:45:00Z'
    },
    {
      id: 'task-3',
      name: 'Update user profile API',
      status: 'archived' as TaskStatus,
      progress: 75,
      createdAt: '2024-01-13T11:00:00Z',
      updatedAt: '2024-01-13T15:20:00Z'
    }
  ];

  beforeEach(() => {
    jest.clearAllMocks();
    mockUseTaskStore.mockReturnValue({
      tasks: mockTasks,
      fetchTasks: mockFetchTasks,
      isLoading: false,
      error: null
    });
  });

  describe('Component Rendering', () => {
    it('should render the task panel with header and create button', () => {
      render(
        <TaskPanel 
          onTaskSelect={mockOnTaskSelect} 
          onCreateTask={mockOnCreateTask} 
        />
      );

      expect(screen.getByText('Local Tasks')).toBeInTheDocument();
      expect(screen.getByText('3')).toBeInTheDocument(); // Task count
      expect(screen.getByTitle('Create New Task')).toBeInTheDocument();
    });

    it('should render filter buttons for all status types', () => {
      render(
        <TaskPanel 
          onTaskSelect={mockOnTaskSelect} 
          onCreateTask={mockOnCreateTask} 
        />
      );

      expect(screen.getByText('All')).toBeInTheDocument();
      expect(screen.getByText('in progress')).toBeInTheDocument();
      expect(screen.getByText('completed')).toBeInTheDocument();
      expect(screen.getByText('archived')).toBeInTheDocument();
    });

    it('should display loading state', () => {
      mockUseTaskStore.mockReturnValue({
        tasks: [],
        fetchTasks: mockFetchTasks,
        isLoading: true,
        error: null
      });

      render(
        <TaskPanel 
          onTaskSelect={mockOnTaskSelect} 
          onCreateTask={mockOnCreateTask} 
        />
      );

      expect(screen.getByText('Loading tasks...')).toBeInTheDocument();
    });

    it('should display error state', () => {
      const errorMessage = 'Failed to load tasks';
      mockUseTaskStore.mockReturnValue({
        tasks: [],
        fetchTasks: mockFetchTasks,
        isLoading: false,
        error: errorMessage
      });

      render(
        <TaskPanel 
          onTaskSelect={mockOnTaskSelect} 
          onCreateTask={mockOnCreateTask} 
        />
      );

      expect(screen.getByText(`Error: ${errorMessage}`)).toBeInTheDocument();
    });
  });

  describe('Task Display', () => {
    it('should display all tasks when filter is set to all', () => {
      render(
        <TaskPanel 
          onTaskSelect={mockOnTaskSelect} 
          onCreateTask={mockOnCreateTask} 
        />
      );

      expect(screen.getByText('Review authentication module')).toBeInTheDocument();
      expect(screen.getByText('Fix login validation')).toBeInTheDocument();
      expect(screen.getByText('Update user profile API')).toBeInTheDocument();
    });

    it('should display task status icons correctly', () => {
      render(
        <TaskPanel 
          onTaskSelect={mockOnTaskSelect} 
          onCreateTask={mockOnCreateTask} 
        />
      );

      const statusIcons = screen.getAllByRole('img', { hidden: true });
      expect(statusIcons).toHaveLength(3);
    });

    it('should display task progress bars with correct percentages', () => {
      render(
        <TaskPanel 
          onTaskSelect={mockOnTaskSelect} 
          onCreateTask={mockOnCreateTask} 
        />
      );

      expect(screen.getByText('45%')).toBeInTheDocument();
      expect(screen.getByText('100%')).toBeInTheDocument();
      expect(screen.getByText('75%')).toBeInTheDocument();
    });

    it('should display empty state when no tasks exist', () => {
      mockUseTaskStore.mockReturnValue({
        tasks: [],
        fetchTasks: mockFetchTasks,
        isLoading: false,
        error: null
      });

      render(
        <TaskPanel 
          onTaskSelect={mockOnTaskSelect} 
          onCreateTask={mockOnCreateTask} 
        />
      );

      expect(screen.getByText('No tasks found')).toBeInTheDocument();
      expect(screen.getByText('Create a task from a file list to review code offline')).toBeInTheDocument();
    });
  });

  describe('Filtering Functionality', () => {
    it('should filter tasks by in_progress status', () => {
      render(
        <TaskPanel 
          onTaskSelect={mockOnTaskSelect} 
          onCreateTask={mockOnCreateTask} 
        />
      );

      fireEvent.click(screen.getByText('in progress'));

      expect(screen.getByText('Review authentication module')).toBeInTheDocument();
      expect(screen.queryByText('Fix login validation')).not.toBeInTheDocument();
      expect(screen.queryByText('Update user profile API')).not.toBeInTheDocument();
      expect(screen.getByText('1')).toBeInTheDocument(); // Filtered count
    });

    it('should filter tasks by completed status', () => {
      render(
        <TaskPanel 
          onTaskSelect={mockOnTaskSelect} 
          onCreateTask={mockOnCreateTask} 
        />
      );

      fireEvent.click(screen.getByText('completed'));

      expect(screen.queryByText('Review authentication module')).not.toBeInTheDocument();
      expect(screen.getByText('Fix login validation')).toBeInTheDocument();
      expect(screen.queryByText('Update user profile API')).not.toBeInTheDocument();
      expect(screen.getByText('1')).toBeInTheDocument(); // Filtered count
    });

    it('should show clear filter button when filter is active', () => {
      render(
        <TaskPanel 
          onTaskSelect={mockOnTaskSelect} 
          onCreateTask={mockOnCreateTask} 
        />
      );

      fireEvent.click(screen.getByText('in progress'));
      
      expect(screen.getByText('Clear filter')).toBeInTheDocument();
    });

    it('should clear filter when clear filter button is clicked', () => {
      render(
        <TaskPanel 
          onTaskSelect={mockOnTaskSelect} 
          onCreateTask={mockOnCreateTask} 
        />
      );

      fireEvent.click(screen.getByText('in progress'));
      fireEvent.click(screen.getByText('Clear filter'));

      expect(screen.getByText('Review authentication module')).toBeInTheDocument();
      expect(screen.getByText('Fix login validation')).toBeInTheDocument();
      expect(screen.getByText('Update user profile API')).toBeInTheDocument();
      expect(screen.queryByText('Clear filter')).not.toBeInTheDocument();
    });

    it('should show appropriate empty message for filtered results', () => {
      render(
        <TaskPanel 
          onTaskSelect={mockOnTaskSelect} 
          onCreateTask={mockOnCreateTask} 
        />
      );

      fireEvent.click(screen.getByText('archived'));

      expect(screen.getByText('No archived tasks')).toBeInTheDocument();
    });
  });

  describe('User Interactions', () => {
    it('should call onCreateTask when create button is clicked', () => {
      render(
        <TaskPanel 
          onTaskSelect={mockOnTaskSelect} 
          onCreateTask={mockOnCreateTask} 
        />
      );

      fireEvent.click(screen.getByTitle('Create New Task'));

      expect(mockOnCreateTask).toHaveBeenCalledTimes(1);
    });

    it('should call onTaskSelect when a task is clicked', () => {
      render(
        <TaskPanel 
          onTaskSelect={mockOnTaskSelect} 
          onCreateTask={mockOnCreateTask} 
        />
      );

      fireEvent.click(screen.getByText('Review authentication module'));

      expect(mockOnTaskSelect).toHaveBeenCalledWith('task-1');
    });

    it('should call fetchTasks on component mount', () => {
      render(
        <TaskPanel 
          onTaskSelect={mockOnTaskSelect} 
          onCreateTask={mockOnCreateTask} 
        />
      );

      expect(mockFetchTasks).toHaveBeenCalledTimes(1);
    });
  });

  describe('Edge Cases', () => {
    it('should handle tasks with very long names', () => {
      const longNameTask = {
        id: 'task-long',
        name: 'This is an extremely long task name that should be truncated properly in the UI to prevent layout issues and maintain good user experience',
        status: 'in_progress' as TaskStatus,
        progress: 50,
        createdAt: '2024-01-15T10:00:00Z',
        updatedAt: '2024-01-15T14:30:00Z'
      };

      mockUseTaskStore.mockReturnValue({
        tasks: [longNameTask],
        fetchTasks: mockFetchTasks,
        isLoading: false,
        error: null
      });

      render(
        <TaskPanel 
          onTaskSelect={mockOnTaskSelect} 
          onCreateTask={mockOnCreateTask} 
        />
      );

      const taskElement = screen.getByText(longNameTask.name);
      expect(taskElement).toBeInTheDocument();
      expect(taskElement).toHaveClass('line-clamp-2');
    });

    it('should handle tasks with 0% progress', () => {
      const zeroProgressTask = {
        id: 'task-zero',
        name: 'New task with no progress',
        status: 'in_progress' as TaskStatus,
        progress: 0,
        createdAt: '2024-01-15T10:00:00Z',
        updatedAt: '2024-01-15T14:30:00Z'
      };

      mockUseTaskStore.mockReturnValue({
        tasks: [zeroProgressTask],
        fetchTasks: mockFetchTasks,
        isLoading: false,
        error: null
      });

      render(
        <TaskPanel 
          onTaskSelect={mockOnTaskSelect} 
          onCreateTask={mockOnCreateTask} 
        />
      );

      expect(screen.getByText('0%')).toBeInTheDocument();
    });

    it('should handle tasks with 100% progress', () => {
      const fullProgressTask = {
        id: 'task-full',
        name: 'Completed task with full progress',
        status: 'completed' as TaskStatus,
        progress: 100,
        createdAt: '2024-01-15T10:00:00Z',
        updatedAt: '2024-01-15T14:30:00Z'
      };

      mockUseTaskStore.mockReturnValue({
        tasks: [fullProgressTask],
        fetchTasks: mockFetchTasks,
        isLoading: false,
        error: null
      });

      render(
        <TaskPanel 
          onTaskSelect={mockOnTaskSelect} 
          onCreateTask={mockOnCreateTask} 
        />
      );

      expect(screen.getByText('100%')).toBeInTheDocument();
    });
  });

  describe('Accessibility', () => {
    it('should have proper ARIA labels for interactive elements', () => {
      render(
        <TaskPanel 
          onTaskSelect={mockOnTaskSelect} 
          onCreateTask={mockOnCreateTask} 
        />
      );

      expect(screen.getByTitle('Create New Task')).toBeInTheDocument();
    });

    it('should be keyboard navigable', () => {
      render(
        <TaskPanel 
          onTaskSelect={mockOnTaskSelect} 
          onCreateTask={mockOnCreateTask} 
        />
      );

      const createButton = screen.getByTitle('Create New Task');
      expect(createButton).toHaveAttribute('tabindex', '0');
    });
  });
});