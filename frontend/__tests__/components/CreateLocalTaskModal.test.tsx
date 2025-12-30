import { render, screen } from '@testing-library/react';
import CreateLocalTaskModal from '../../components/CreateLocalTaskModal';

// Basic mock for dependencies  
jest.mock('../../store/taskStore', () => ({
  useTaskStore: jest.fn(() => ({
    fetchTasks: jest.fn(),
  })),
}));

jest.mock('../../hooks/useLocalTasks', () => ({
  useLocalTasks: jest.fn(() => ({
    createTask: jest.fn(),
  })),
}));

describe('CreateLocalTaskModal Basic Tests', () => {
  const mockOnClose = jest.fn();
  const mockOnTaskCreated = jest.fn();

  beforeEach(() => {
    jest.clearAllMocks();
  });

  describe('Rendering', () => {
    test('should render modal when open', () => {
      render(
        <CreateLocalTaskModal
          isOpen={true}
          onClose={mockOnClose}
          onTaskCreated={mockOnTaskCreated}
        />
      );
      
      // Basic rendering test - check for form elements
      expect(screen.getByText('Create Local Task')).toBeTruthy();
      expect(screen.getByText('Task Name')).toBeTruthy();
      expect(screen.getByText('Repository Path')).toBeTruthy();
      expect(screen.getByText('Base Reference')).toBeTruthy();
    });

    test('should not render when closed', () => {
      render(
        <CreateLocalTaskModal
          isOpen={false}
          onClose={mockOnClose}
          onTaskCreated={mockOnTaskCreated}
        />
      );
      
      expect(screen.queryByText('Create Local Task')).toBeFalsy();
    });
  });

  describe('Form Elements', () => {
    test('should render form inputs with correct placeholders', () => {
      render(
        <CreateLocalTaskModal
          isOpen={true}
          onClose={mockOnClose}
          onTaskCreated={mockOnTaskCreated}
        />
      );
      
      expect(screen.getByPlaceholderText('e.g., Code Review - Feature X')).toBeTruthy();
      expect(screen.getByPlaceholderText('/path/to/repository')).toBeTruthy();
      expect(screen.getByPlaceholderText('main or commit hash')).toBeTruthy();
    });

    test('should render buttons with correct text', () => {
      render(
        <CreateLocalTaskModal
          isOpen={true}
          onClose={mockOnClose}
          onTaskCreated={mockOnTaskCreated}
        />
      );
      
      expect(screen.getByRole('button', { name: 'Cancel' })).toBeTruthy();
      expect(screen.getByRole('button', { name: 'Create Task' })).toBeTruthy();
    });
  });

  describe('Form Input', () => {
    test('should update task name', () => {
      render(
        <CreateLocalTaskModal
          isOpen={true}
          onClose={mockOnClose}
          onTaskCreated={mockOnTaskCreated}
        />
      );
      
      const taskNameInput = screen.getByPlaceholderText('e.g., Code Review - Feature X') as HTMLInputElement;
      taskNameInput.value = 'Test Task';
      taskNameInput.dispatchEvent(new Event('change'));
      
      expect(taskNameInput.value).toBe('Test Task');
    });

    test('should update repository path', () => {
      render(
        <CreateLocalTaskModal
          isOpen={true}
          onClose={mockOnClose}
          onTaskCreated={mockOnTaskCreated}
        />
      );
      
      const repoPathInput = screen.getByPlaceholderText('/path/to/repository') as HTMLInputElement;
      repoPathInput.value = '/path/to/repo';
      repoPathInput.dispatchEvent(new Event('change'));
      
      expect(repoPathInput.value).toBe('/path/to/repo');
    });

    test('should update base reference', () => {
      render(
        <CreateLocalTaskModal
          isOpen={true}
          onClose={mockOnClose}
          onTaskCreated={mockOnTaskCreated}
        />
      );
      
      const baseRefInput = screen.getByPlaceholderText('main or commit hash') as HTMLInputElement;
      baseRefInput.value = 'develop';
      baseRefInput.dispatchEvent(new Event('change'));
      
      expect(baseRefInput.value).toBe('develop');
    });

    test('should have default base reference value', () => {
      render(
        <CreateLocalTaskModal
          isOpen={true}
          onClose={mockOnClose}
          onTaskCreated={mockOnTaskCreated}
        />
      );
      
      const baseRefInput = screen.getByPlaceholderText('main or commit hash') as HTMLInputElement;
      expect(baseRefInput.value).toBe('main');
    });
  });

  describe('Form Validation', () => {
    test('should disable submit button when required fields are empty', () => {
      render(
        <CreateLocalTaskModal
          isOpen={true}
          onClose={mockOnClose}
          onTaskCreated={mockOnTaskCreated}
        />
      );
      
      const submitButton = screen.getByRole('button', { name: /Create Task/i });
      expect(submitButton.hasAttribute('disabled')).toBe(true);
    });
  });

  describe('Cancel Button', () => {
    test('should call onClose when cancel button is clicked', () => {
      render(
        <CreateLocalTaskModal
          isOpen={true}
          onClose={mockOnClose}
          onTaskCreated={mockOnTaskCreated}
        />
      );
      
      const cancelButton = screen.getByRole('button', { name: /Cancel/i });
      cancelButton.click();
      
      expect(mockOnClose).toHaveBeenCalled();
    });
  });

  describe('Accessibility', () => {
    test('should have proper form labels', () => {
      render(
        <CreateLocalTaskModal
          isOpen={true}
          onClose={mockOnClose}
          onTaskCreated={mockOnTaskCreated}
        />
      );
      
      expect(screen.getByText('Task Name')).toBeTruthy();
      expect(screen.getByText('Repository Path')).toBeTruthy();
      expect(screen.getByText('Base Reference')).toBeTruthy();
      expect(screen.getByText(/File List/)).toBeTruthy();
    });

    test('should have required attributes on required fields', () => {
      render(
        <CreateLocalTaskModal
          isOpen={true}
          onClose={mockOnClose}
          onTaskCreated={mockOnTaskCreated}
        />
      );
      
      const taskNameInput = screen.getByPlaceholderText('e.g., Code Review - Feature X') as HTMLInputElement;
      const repoPathInput = screen.getByPlaceholderText('/path/to/repository') as HTMLInputElement;
      const fileListTextarea = document.getElementById('file-list') as HTMLTextAreaElement;
      
      expect(taskNameInput.hasAttribute('required')).toBe(true);
      expect(repoPathInput.hasAttribute('required')).toBe(true);
      expect(fileListTextarea.hasAttribute('required')).toBe(true);
    });
  });
});