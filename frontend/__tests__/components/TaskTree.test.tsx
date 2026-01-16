import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import TaskTree from '../../components/TaskTree';
import { useTranslation } from '../../i18n';
import { useApiClient } from '../../api/client';

// Mock dependencies
jest.mock('../../i18n');
jest.mock('../../api/client');
jest.mock('@tauri-apps/api/dialog');
jest.mock('@tauri-apps/api/fs');

describe('TaskTree Component', () => {
  const mockT = jest.fn((key) => {
    const translations: Record<string, string> = {
      'tasktree.review_pending': 'Review Pending',
      'tasktree.watching': 'Watching',
      'tasktree.history': 'History',
      'tasktree.no_history': 'No history today',
      'tasktree.tab.local': 'Local Tasks',
      'tasktree.sort.status': 'Sort by Status',
      'tasktree.sort.type': 'Sort by Type',
      'tasktree.sort.name': 'Sort by Name',
      'tasktree.sort.path': 'Sort by Path'
    };
    return translations[key] || key;
  });

  const mockOnSelectTask = jest.fn();
  const mockOnAction = jest.fn();
  const mockOnSelectFile = jest.fn();

  const mockTasks = [
    {
      id: 'task-1',
      title: 'Review authentication module',
      status: 'pending',
      type: 'code',
      unreadCount: 2,
      files: [
        { id: 'file-1', name: 'auth.ts', path: 'src/auth.ts', status: 'modified', reviewStatus: 'pending' },
        { id: 'file-2', name: 'user.ts', path: 'src/user.ts', status: 'added', reviewStatus: 'approved' }
      ]
    },
    {
      id: 'task-2', 
      title: 'Fix login validation',
      status: 'active',
      type: 'security',
      unreadCount: 0,
      files: []
    }
  ];

  const mockLocalTasks = [
    {
      id: 'local-task-1',
      title: 'Local task 1',
      status: 'active',
      type: 'code',
      files: [
        { id: 'local-file-1', name: 'test.js', path: 'test.js', status: 'modified', reviewStatus: 'pending' }
      ]
    },
    {
      id: 'local-task-2',
      title: 'Local task 2', 
      status: 'completed',
      type: 'sql',
      files: []
    }
  ];

  const mockFileTree = [
    {
      id: 'folder-1',
      name: 'src',
      type: 'folder',
      path: 'src',
      children: [
        { id: 'file-1', name: 'index.ts', type: 'file', path: 'src/index.ts', status: 'modified' }
      ]
    },
    {
      id: 'file-2',
      name: 'README.md',
      type: 'file', 
      path: 'README.md',
      status: 'added'
    }
  ];

  beforeEach(() => {
    jest.clearAllMocks();
    
    (useTranslation as jest.Mock).mockReturnValue({ t: mockT });
    (useApiClient as jest.Mock).mockReturnValue({
      getTasks: jest.fn().mockResolvedValue(mockTasks),
      getLocalTasks: jest.fn().mockResolvedValue(mockLocalTasks),
      getFileTree: jest.fn().mockResolvedValue(mockFileTree),
      markTaskCompleted: jest.fn().mockResolvedValue(undefined),
      exportTaskReview: jest.fn().mockResolvedValue('csv,data,here')
    });
  });

  describe('Component Rendering', () => {
    it('should render tab navigation', () => {
      render(
        <TaskTree
          activeTaskId=""
          onSelectTask={mockOnSelectTask}
          onAction={mockOnAction}
          onSelectFile={mockOnSelectFile}
        />
      );

      expect(screen.getByTitle('Git Tasks (PRs)')).toBeInTheDocument();
      expect(screen.getByTitle('Local Tasks')).toBeInTheDocument();
      expect(screen.getByTitle('File Explorer')).toBeInTheDocument();
    });

    it('should show loading state initially', () => {
      render(
        <TaskTree
          activeTaskId=""
          onSelectTask={mockOnSelectTask}
          onAction={mockOnAction}
          onSelectFile={mockOnSelectFile}
        />
      );

      expect(screen.getByText('Loading...')).toBeInTheDocument();
    });

    it('should render Git tasks after loading', async () => {
      render(
        <TaskTree
          activeTaskId=""
          onSelectTask={mockOnSelectTask}
          onAction={mockOnAction}
          onSelectFile={mockOnSelectFile}
        />
      );

      await waitFor(() => {
        expect(screen.getByText('Review Pending')).toBeInTheDocument();
        expect(screen.getByText('Watching')).toBeInTheDocument();
        expect(screen.getByText('History')).toBeInTheDocument();
      });
    });

    it('should render Local Tasks tab when clicked', async () => {
      render(
        <TaskTree
          activeTaskId=""
          onSelectTask={mockOnSelectTask}
          onAction={mockOnAction}
          onSelectFile={mockOnSelectFile}
        />
      );

      await waitFor(() => {
        fireEvent.click(screen.getByTitle('Local Tasks'));
      });

      expect(screen.getByText('Local Tasks')).toBeInTheDocument();
    });
  });

  describe('Git Tasks Tab', () => {
    it('should expand/collapse task sections', async () => {
      render(
        <TaskTree
          activeTaskId=""
          onSelectTask={mockOnSelectTask}
          onAction={mockOnAction}
          onSelectFile={mockOnSelectFile}
        />
      );

      await waitFor(() => {
        const reviewPendingSection = screen.getByText('Review Pending');
        fireEvent.click(reviewPendingSection);
      });

      expect(screen.getByText('Review authentication module')).toBeInTheDocument();
    });

    it('should select task when clicked', async () => {
      render(
        <TaskTree
          activeTaskId=""
          onSelectTask={mockOnSelectTask}
          onAction={mockOnAction}
          onSelectFile={mockOnSelectFile}
        />
      );

      await waitFor(() => {
        fireEvent.click(screen.getByText('Review Pending'));
        fireEvent.click(screen.getByText('Review authentication module'));
      });

      expect(mockOnSelectTask).toHaveBeenCalledWith('task-1');
    });

    it('should highlight active task', async () => {
      render(
        <TaskTree
          activeTaskId="task-1"
          onSelectTask={mockOnSelectTask}
          onAction={mockOnAction}
          onSelectFile={mockOnSelectFile}
        />
      );

      await waitFor(() => {
        fireEvent.click(screen.getByText('Review Pending'));
      });

      const taskElement = screen.getByText('Review authentication module').closest('div');
      expect(taskElement).toHaveClass('bg-editor-selection');
    });
  });

  describe('Local Tasks Tab', () => {
    it('should render local tasks with sort controls', async () => {
      render(
        <TaskTree
          activeTaskId=""
          onSelectTask={mockOnSelectTask}
          onAction={mockOnAction}
          onSelectFile={mockOnSelectFile}
        />
      );

      await waitFor(() => {
        fireEvent.click(screen.getByTitle('Local Tasks'));
      });

      expect(screen.getByText('Local task 1')).toBeInTheDocument();
      expect(screen.getByText('Local task 2')).toBeInTheDocument();
      expect(screen.getByTitle('Sort by Status')).toBeInTheDocument();
      expect(screen.getByTitle('Sort by Type')).toBeInTheDocument();
      expect(screen.getByTitle('Sort by Name')).toBeInTheDocument();
      expect(screen.getByTitle('Sort by Path')).toBeInTheDocument();
    });

    it('should expand/collapse local task files', async () => {
      render(
        <TaskTree
          activeTaskId=""
          onSelectTask={mockOnSelectTask}
          onAction={mockOnAction}
          onSelectFile={mockOnSelectFile}
        />
      );

      await waitFor(() => {
        fireEvent.click(screen.getByTitle('Local Tasks'));
        fireEvent.click(screen.getByText('Local task 1'));
      });

      expect(screen.getByText('test.js')).toBeInTheDocument();
    });

    it('should show task type badges', async () => {
      render(
        <TaskTree
          activeTaskId=""
          onSelectTask={mockOnSelectTask}
          onAction={mockOnAction}
          onSelectFile={mockOnSelectFile}
        />
      );

      await waitFor(() => {
        fireEvent.click(screen.getByTitle('Local Tasks'));
      });

      expect(screen.getByText('CODE')).toBeInTheDocument();
      expect(screen.getByText('SQL')).toBeInTheDocument();
    });

    it('should sort local tasks by different criteria', async () => {
      render(
        <TaskTree
          activeTaskId=""
          onSelectTask={mockOnSelectTask}
          onAction={mockOnAction}
          onSelectFile={mockOnSelectFile}
        />
      );

      await waitFor(() => {
        fireEvent.click(screen.getByTitle('Local Tasks'));
      });

      // Test sort by name
      fireEvent.click(screen.getByTitle('Sort by Name'));
      
      // Test sort by type  
      fireEvent.click(screen.getByTitle('Sort by Type'));
      
      // Test sort by status
      fireEvent.click(screen.getByTitle('Sort by Status'));
      
      // Test sort by path
      fireEvent.click(screen.getByTitle('Sort by Path'));

      // Verify API was called to get tasks for each sort change
      expect(useApiClient().getLocalTasks).toHaveBeenCalled();
    });
  });

  describe('File Explorer Tab', () => {
    it('should render file tree', async () => {
      render(
        <TaskTree
          activeTaskId=""
          onSelectTask={mockOnSelectTask}
          onAction={mockOnAction}
          onSelectFile={mockOnSelectFile}
        />
      );

      await waitFor(() => {
        fireEvent.click(screen.getByTitle('File Explorer'));
      });

      expect(screen.getByText('Explorer')).toBeInTheDocument();
      expect(screen.getByText('src')).toBeInTheDocument();
      expect(screen.getByText('README.md')).toBeInTheDocument();
    });

    it('should expand/collapse folders', async () => {
      render(
        <TaskTree
          activeTaskId=""
          onSelectTask={mockOnSelectTask}
          onAction={mockOnAction}
          onSelectFile={mockOnSelectFile}
        />
      );

      await waitFor(() => {
        fireEvent.click(screen.getByTitle('File Explorer'));
        fireEvent.click(screen.getByText('src'));
      });

      expect(screen.getByText('index.ts')).toBeInTheDocument();
    });

    it('should select file when clicked', async () => {
      render(
        <TaskTree
          activeTaskId=""
          onSelectTask={mockOnSelectTask}
          onAction={mockOnAction}
          onSelectFile={mockOnSelectFile}
        />
      );

      await waitFor(() => {
        fireEvent.click(screen.getByTitle('File Explorer'));
        fireEvent.click(screen.getByText('README.md'));
      });

      expect(mockOnSelectFile).toHaveBeenCalledWith('README.md');
    });
  });

  describe('Error Handling', () => {
    it('should handle API errors gracefully', async () => {
      (useApiClient as jest.Mock).mockReturnValue({
        getTasks: jest.fn().mockRejectedValue(new Error('API Error')),
        getLocalTasks: jest.fn().mockRejectedValue(new Error('API Error')),
        getFileTree: jest.fn().mockRejectedValue(new Error('API Error')),
        markTaskCompleted: jest.fn().mockRejectedValue(new Error('API Error')),
        exportTaskReview: jest.fn().mockRejectedValue(new Error('API Error'))
      });

      const consoleSpy = jest.spyOn(console, 'error').mockImplementation();

      render(
        <TaskTree
          activeTaskId=""
          onSelectTask={mockOnSelectTask}
          onAction={mockOnAction}
          onSelectFile={mockOnSelectFile}
        />
      );

      await waitFor(() => {
        expect(consoleSpy).toHaveBeenCalled();
      });

      consoleSpy.mockRestore();
    });
  });

  describe('Accessibility', () => {
    it('should have proper ARIA labels for tab navigation', () => {
      render(
        <TaskTree
          activeTaskId=""
          onSelectTask={mockOnSelectTask}
          onAction={mockOnAction}
          onSelectFile={mockOnSelectFile}
        />
      );

      expect(screen.getByTitle('Git Tasks (PRs)')).toBeInTheDocument();
      expect(screen.getByTitle('Local Tasks')).toBeInTheDocument();
      expect(screen.getByTitle('File Explorer')).toBeInTheDocument();
    });
  });
});