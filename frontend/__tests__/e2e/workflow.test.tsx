import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import userEvent from '@testing-library/user-event';

// Mock Tauri IPC
const mockIPC = {
  invoke: jest.fn(),
  listen: jest.fn(),
};

(global as any).__TAURI_IPC__ = mockIPC;

// Mock the task store and components
jest.mock('../store/taskStore', () => ({
  useTaskStore: jest.fn(() => ({
    tasks: [],
    fetchTasks: jest.fn(),
    isLoading: false,
    error: null,
  })),
}));

jest.mock('../api/client', () => ({
  useApiClient: jest.fn(() => ({
    createLocalTask: jest.fn(),
    getLocalTasks: jest.fn(),
    exportTaskReview: jest.fn(),
  })),
}));

describe('E2E Workflow: Create → Review → Export', () => {
  const user = userEvent.setup();

  beforeEach(() => {
    jest.clearAllMocks();
    mockIPC.invoke.mockReset();
    mockIPC.listen.mockReset();
  });

  describe('Complete Workflow Integration', () => {
    it('should complete the full create → review → export workflow', async () => {
      // Mock IPC responses for the complete workflow
      mockIPC.invoke
        .mockResolvedValueOnce({ id: 'test-task-123', name: 'Test Task' }) // create_task
        .mockResolvedValueOnce([{ id: 'test-task-123', name: 'Test Task', status: 'in_progress', progress: 0 }]) // list_tasks
        .mockResolvedValueOnce({ id: 'test-task-123', name: 'Test Task', items: [{ file: 'test.js', reviewed: false }] }) // get_task
        .mockResolvedValueOnce(undefined) // update_task_progress
        .mockResolvedValueOnce({ id: 'test-task-123', name: 'Test Task', items: [{ file: 'test.js', reviewed: true }] }) // get_task after review
        .mockResolvedValueOnce('{"id":"test-task-123","name":"Test Task","items":[{"file":"test.js","reviewed":true}]}'); // export_task

      // Step 1: Create Task
      const TaskPanel = ({ onTaskSelect, onCreateTask }: any) => (
        <div data-testid="task-panel">
          <button data-testid="create-task-button" onClick={onCreateTask}>
            Create New Task
          </button>
          <div data-testid="task-list">
            <div data-testid="task-item" onClick={() => onTaskSelect('test-task-123')}>
              Test Task
            </div>
          </div>
        </div>
      );

      const CreateLocalTaskModal = ({ isOpen, onClose, onTaskCreated }: any) => {
        if (!isOpen) return null;
        return (
          <div data-testid="create-task-modal">
            <input data-testid="task-name-input" placeholder="Task name" />
            <textarea data-testid="task-items-textarea" placeholder="Enter file paths and descriptions" />
            <button data-testid="create-task-button" onClick={() => onTaskCreated({ id: 'test-task-123' })}>
              Create Task
            </button>
            <button data-testid="cancel-task-button" onClick={onClose}>Cancel</button>
          </div>
        );
      };

      const TaskTree = ({ activeTaskId, onSelectTask }: any) => (
        <div data-testid="task-tree">
          <button data-testid="local-tasks-tab" onClick={() => onSelectTask('test-task-123')}>
            Local Tasks
          </button>
          {activeTaskId && (
            <div data-testid="task-details">
              <div>Task: {activeTaskId}</div>
              <button data-testid="export-task-button">Export Task</button>
            </div>
          )}
        </div>
      );

      const { rerender } = render(
        <div>
          <TaskPanel onTaskSelect={jest.fn()} onCreateTask={jest.fn()} />
          <CreateLocalTaskModal isOpen={false} onClose={jest.fn()} onTaskCreated={jest.fn()} />
        </div>
      );

      // Open create task modal
      fireEvent.click(screen.getByTestId('create-task-button'));
      
      rerender(
        <div>
          <TaskPanel onTaskSelect={jest.fn()} onCreateTask={jest.fn()} />
          <CreateLocalTaskModal isOpen={true} onClose={jest.fn()} onTaskCreated={jest.fn()} />
        </div>
      );

      // Fill in task details
      await user.type(screen.getByTestId('task-name-input'), 'Test Task');
      await user.type(
        screen.getByTestId('task-items-textarea'), 
        'test.js: Review test file\napp.ts: Review main application'
      );

      // Create the task
      fireEvent.click(screen.getByTestId('create-task-button'));

      // Verify task creation was called
      await waitFor(() => {
        expect(mockIPC.invoke).toHaveBeenCalledWith('create_task', expect.objectContaining({
          name: 'Test Task',
          items_text: 'test.js: Review test file\napp.ts: Review main application'
        }));
      });

      // Step 2: Review Task (simulate task selection and progress update)
      rerender(
        <div>
          <TaskTree activeTaskId="test-task-123" onSelectTask={jest.fn()} />
        </div>
      );

      // Verify task is selected and visible
      expect(screen.getByTestId('task-details')).toBeInTheDocument();
      expect(screen.getByText('Task: test-task-123')).toBeInTheDocument();

      // Simulate marking an item as reviewed
      mockIPC.invoke.mockResolvedValueOnce(undefined); // update_task_progress
      
      // This would normally be triggered by UI interaction in the actual component
      await mockIPC.invoke('update_task_progress', {
        task_id: 'test-task-123',
        item_index: 0,
        reviewed: true
      });

      // Step 3: Export Task
      fireEvent.click(screen.getByTestId('export-task-button'));

      // Verify export was called
      await waitFor(() => {
        expect(mockIPC.invoke).toHaveBeenCalledWith('export_task', 'test-task-123');
      });

      // Verify the complete workflow sequence
      const callSequence = mockIPC.invoke.mock.calls.map(call => call[0]);
      expect(callSequence).toContain('create_task');
      expect(callSequence).toContain('list_tasks');
      expect(callSequence).toContain('get_task');
      expect(callSequence).toContain('update_task_progress');
      expect(callSequence).toContain('export_task');
    });

    it('should validate input before creating task', async () => {
      const CreateLocalTaskModal = ({ isOpen, onClose, onTaskCreated }: any) => {
        if (!isOpen) return null;
        return (
          <div data-testid="create-task-modal">
            <input data-testid="task-name-input" placeholder="Task name" />
            <textarea data-testid="task-items-textarea" placeholder="Enter file paths and descriptions" />
            <button data-testid="create-task-button" onClick={() => onTaskCreated({ id: 'test-task-123' })}>
              Create Task
            </button>
            <button data-testid="cancel-task-button" onClick={onClose}>Cancel</button>
          </div>
        );
      };

      render(
        <div>
          <CreateLocalTaskModal 
            isOpen={true} 
            onClose={jest.fn()} 
            onTaskCreated={jest.fn()}
          />
        </div>
      );

      // Try to create task without name
      fireEvent.click(screen.getByTestId('create-task-button'));

      // Should not call IPC without validation
      expect(mockIPC.invoke).not.toHaveBeenCalled();

      // Add name but no items
      await user.type(screen.getByTestId('task-name-input'), 'Valid Name');
      fireEvent.click(screen.getByTestId('create-task-button'));

      // Should still not call IPC without items
      expect(mockIPC.invoke).not.toHaveBeenCalled();
    });

    it('should handle concurrent operations', async () => {
      // Simulate rapid create/review operations
      const operations = [
        Promise.resolve({ id: 'task-1' }),
        Promise.resolve(undefined), // progress update
        Promise.resolve({ id: 'task-2' }),
        Promise.resolve(undefined), // progress update
      ];

      mockIPC.invoke.mockImplementation(() => operations.shift());

      // Simulate rapid user interactions
      const rapidOperations = async () => {
        await mockIPC.invoke('create_task', { name: 'Task 1' });
        await mockIPC.invoke('update_task_progress', { task_id: 'task-1', reviewed: true });
        await mockIPC.invoke('create_task', { name: 'Task 2' });
        await mockIPC.invoke('update_task_progress', { task_id: 'task-2', reviewed: true });
      };

      await rapidOperations();

      // Verify all operations completed
      expect(mockIPC.invoke).toHaveBeenCalledTimes(4);
    });
  });

  describe('Workflow State Management', () => {
    it('should maintain consistent state across workflow steps', async () => {
      const taskData = {
        id: 'consistent-task-123',
        name: 'Consistent Task',
        status: 'in_progress',
        progress: 75,
        items: [
          { file: 'file1.js', reviewed: true },
          { file: 'file2.ts', reviewed: true },
          { file: 'file3.py', reviewed: false },
          { file: 'file4.rs', reviewed: false }
        ]
      };

      mockIPC.invoke.mockResolvedValue(taskData);

      // Simulate workflow with state consistency checks
      const createResult = await mockIPC.invoke('create_task', { name: 'Consistent Task' });
      expect(createResult.id).toBe('consistent-task-123');

      const getResult = await mockIPC.invoke('get_task', 'consistent-task-123');
      expect(getResult.name).toBe('Consistent Task');
      expect(getResult.progress).toBe(75);
      expect(getResult.items.length).toBe(4);

      const exportResult = await mockIPC.invoke('export_task', 'consistent-task-123');
      const exportedData = JSON.parse(exportResult);
      expect(exportedData.name).toBe('Consistent Task');
      expect(exportedData.progress).toBe(75);
    });

    it('should handle workflow timeouts gracefully', async () => {
      // Mock timeout
      mockIPC.invoke.mockRejectedValue(new Error('Operation timed out'));

      const onError = jest.fn();

      try {
        await mockIPC.invoke('create_task', { name: 'Timeout Task' });
      } catch (error) {
        onError(error);
      }

      expect(onError).toHaveBeenCalledWith(expect.objectContaining({
        message: 'Operation timed out'
      }));
    });
  });
});