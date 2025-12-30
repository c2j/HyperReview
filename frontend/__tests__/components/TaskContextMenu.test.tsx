import { render, screen, fireEvent } from '@testing-library/react';
import TaskContextMenu from '../../components/TaskContextMenu';
import type { TaskSummary } from '../../types/task';

// Mock window.confirm
const mockConfirm = jest.fn();
Object.defineProperty(window, 'confirm', {
  value: mockConfirm,
  writable: true,
});

describe('TaskContextMenu Basic Tests', () => {
  const mockTask: TaskSummary = {
    id: 'test-task-id',
    name: 'Test Task',
    status: 'in_progress',
    progress: 0.5,
  };

  const mockCallbacks = {
    onEdit: jest.fn(),
    onDelete: jest.fn(),
    onArchive: jest.fn(),
    onExport: jest.fn(),
    onComplete: jest.fn(),
    onExternalSubmit: jest.fn(),
    onClose: jest.fn(),
  };

  const defaultPosition = { x: 100, y: 200 };

  beforeEach(() => {
    jest.clearAllMocks();
    mockConfirm.mockReturnValue(true);
  });

  describe('Basic Rendering', () => {
    test('should render menu with all items for in_progress task', () => {
      render(
        <TaskContextMenu
          task={mockTask}
          onEdit={mockCallbacks.onEdit}
          onDelete={mockCallbacks.onDelete}
          onArchive={mockCallbacks.onArchive}
          onExport={mockCallbacks.onExport}
          onComplete={mockCallbacks.onComplete}
          onExternalSubmit={mockCallbacks.onExternalSubmit}
          onClose={mockCallbacks.onClose}
          position={defaultPosition}
        />
      );

      // Check that all menu items are rendered
      expect(screen.getByText('Edit')).toBeTruthy();
      expect(screen.getByText('Export')).toBeTruthy();
      expect(screen.getByText('Submit to External')).toBeTruthy();
      expect(screen.getByText('Archive')).toBeTruthy();
      expect(screen.getByText('Mark Complete')).toBeTruthy();
      expect(screen.getByText('Delete')).toBeTruthy();
    });

    test('should disable edit and archive for archived task', () => {
      const archivedTask = { ...mockTask, status: 'archived' as const };
      
      render(
        <TaskContextMenu
          task={archivedTask}
          onEdit={mockCallbacks.onEdit}
          onDelete={mockCallbacks.onDelete}
          onArchive={mockCallbacks.onArchive}
          onExport={mockCallbacks.onExport}
          onComplete={mockCallbacks.onComplete}
          onExternalSubmit={mockCallbacks.onExternalSubmit}
          onClose={mockCallbacks.onClose}
          position={defaultPosition}
        />
      );

      const editButton = screen.getByText('Edit').closest('button');
      const archiveButton = screen.getByText('Archive').closest('button');
      const completeButton = screen.getByText('Mark Complete').closest('button');

      expect(editButton?.hasAttribute('disabled')).toBe(true);
      expect(archiveButton?.hasAttribute('disabled')).toBe(true);
      expect(completeButton?.hasAttribute('disabled')).toBe(true);
    });

    test('should disable complete for completed task', () => {
      const completedTask = { ...mockTask, status: 'completed' as const };
      
      render(
        <TaskContextMenu
          task={completedTask}
          onEdit={mockCallbacks.onEdit}
          onDelete={mockCallbacks.onDelete}
          onArchive={mockCallbacks.onArchive}
          onExport={mockCallbacks.onExport}
          onComplete={mockCallbacks.onComplete}
          onExternalSubmit={mockCallbacks.onExternalSubmit}
          onClose={mockCallbacks.onClose}
          position={defaultPosition}
        />
      );

      const completeButton = screen.getByText('Mark Complete').closest('button');
      expect(completeButton?.hasAttribute('disabled')).toBe(true);
    });
  });

  describe('Menu Actions', () => {
    test('should call onEdit and onClose when Edit is clicked', () => {
      render(
        <TaskContextMenu
          task={mockTask}
          onEdit={mockCallbacks.onEdit}
          onDelete={mockCallbacks.onDelete}
          onArchive={mockCallbacks.onArchive}
          onExport={mockCallbacks.onExport}
          onComplete={mockCallbacks.onComplete}
          onExternalSubmit={mockCallbacks.onExternalSubmit}
          onClose={mockCallbacks.onClose}
          position={defaultPosition}
        />
      );

      fireEvent.click(screen.getByText('Edit'));

      expect(mockCallbacks.onEdit).toHaveBeenCalledTimes(1);
      expect(mockCallbacks.onClose).toHaveBeenCalledTimes(1);
    });

    test('should call onExport and onClose when Export is clicked', () => {
      render(
        <TaskContextMenu
          task={mockTask}
          onEdit={mockCallbacks.onEdit}
          onDelete={mockCallbacks.onDelete}
          onArchive={mockCallbacks.onArchive}
          onExport={mockCallbacks.onExport}
          onComplete={mockCallbacks.onComplete}
          onExternalSubmit={mockCallbacks.onExternalSubmit}
          onClose={mockCallbacks.onClose}
          position={defaultPosition}
        />
      );

      fireEvent.click(screen.getByText('Export'));

      expect(mockCallbacks.onExport).toHaveBeenCalledTimes(1);
      expect(mockCallbacks.onClose).toHaveBeenCalledTimes(1);
    });

    test('should call onArchive and onClose when Archive is clicked', () => {
      render(
        <TaskContextMenu
          task={mockTask}
          onEdit={mockCallbacks.onEdit}
          onDelete={mockCallbacks.onDelete}
          onArchive={mockCallbacks.onArchive}
          onExport={mockCallbacks.onExport}
          onComplete={mockCallbacks.onComplete}
          onExternalSubmit={mockCallbacks.onExternalSubmit}
          onClose={mockCallbacks.onClose}
          position={defaultPosition}
        />
      );

      fireEvent.click(screen.getByText('Archive'));

      expect(mockCallbacks.onArchive).toHaveBeenCalledTimes(1);
      expect(mockCallbacks.onClose).toHaveBeenCalledTimes(1);
    });

    test('should call onComplete and onClose when Mark Complete is clicked', () => {
      render(
        <TaskContextMenu
          task={mockTask}
          onEdit={mockCallbacks.onEdit}
          onDelete={mockCallbacks.onDelete}
          onArchive={mockCallbacks.onArchive}
          onExport={mockCallbacks.onExport}
          onComplete={mockCallbacks.onComplete}
          onExternalSubmit={mockCallbacks.onExternalSubmit}
          onClose={mockCallbacks.onClose}
          position={defaultPosition}
        />
      );

      fireEvent.click(screen.getByText('Mark Complete'));

      expect(mockConfirm).toHaveBeenCalledWith('Mark "Test Task" as completed?');
      expect(mockCallbacks.onComplete).toHaveBeenCalledTimes(1);
      expect(mockCallbacks.onClose).toHaveBeenCalledTimes(1);
    });

    test('should call onDelete and onClose when Delete is clicked and confirmed', () => {
      render(
        <TaskContextMenu
          task={mockTask}
          onEdit={mockCallbacks.onEdit}
          onDelete={mockCallbacks.onDelete}
          onArchive={mockCallbacks.onArchive}
          onExport={mockCallbacks.onExport}
          onComplete={mockCallbacks.onComplete}
          onExternalSubmit={mockCallbacks.onExternalSubmit}
          onClose={mockCallbacks.onClose}
          position={defaultPosition}
        />
      );

      fireEvent.click(screen.getByText('Delete'));

      expect(mockConfirm).toHaveBeenCalledWith('Are you sure you want to delete "Test Task"?');
      expect(mockCallbacks.onDelete).toHaveBeenCalledTimes(1);
      expect(mockCallbacks.onClose).toHaveBeenCalledTimes(1);
    });

    test('should not call onDelete when Delete is cancelled', () => {
      mockConfirm.mockReturnValue(false);
      
      render(
        <TaskContextMenu
          task={mockTask}
          onEdit={mockCallbacks.onEdit}
          onDelete={mockCallbacks.onDelete}
          onArchive={mockCallbacks.onArchive}
          onExport={mockCallbacks.onExport}
          onComplete={mockCallbacks.onComplete}
          onExternalSubmit={mockCallbacks.onExternalSubmit}
          onClose={mockCallbacks.onClose}
          position={defaultPosition}
        />
      );

      fireEvent.click(screen.getByText('Delete'));

      expect(mockConfirm).toHaveBeenCalledWith('Are you sure you want to delete "Test Task"?');
      expect(mockCallbacks.onDelete).not.toHaveBeenCalled();
      expect(mockCallbacks.onClose).not.toHaveBeenCalled();
    });
  });

  describe('External Submit Menu', () => {
    test('should toggle external menu when Submit to External is clicked', () => {
      render(
        <TaskContextMenu
          task={mockTask}
          onEdit={mockCallbacks.onEdit}
          onDelete={mockCallbacks.onDelete}
          onArchive={mockCallbacks.onArchive}
          onExport={mockCallbacks.onExport}
          onComplete={mockCallbacks.onComplete}
          onExternalSubmit={mockCallbacks.onExternalSubmit}
          onClose={mockCallbacks.onClose}
          position={defaultPosition}
        />
      );

      const externalButton = screen.getByText('Submit to External').closest('button');
      
      // Initially, external menu should not be visible
      expect(screen.queryByText('Choose System...')).toBeFalsy();
      
      // Click to open external menu
      if (externalButton) {
        fireEvent.click(externalButton);
        expect(screen.getByText('Choose System...')).toBeTruthy();
        
        // Click again to close external menu
        fireEvent.click(externalButton);
        expect(screen.queryByText('Choose System...')).toBeFalsy();
      }
    });
  });

  describe('Visual Styling', () => {
    test('should apply danger styling to Delete button', () => {
      render(
        <TaskContextMenu
          task={mockTask}
          onEdit={mockCallbacks.onEdit}
          onDelete={mockCallbacks.onDelete}
          onArchive={mockCallbacks.onArchive}
          onExport={mockCallbacks.onExport}
          onComplete={mockCallbacks.onComplete}
          onExternalSubmit={mockCallbacks.onExternalSubmit}
          onClose={mockCallbacks.onClose}
          position={defaultPosition}
        />
      );

      const deleteButton = screen.getByText('Delete').closest('button');
      expect(deleteButton?.classList.contains('text-red-400')).toBe(true);
    });
  });

  describe('Edge Cases', () => {
    test('should handle task with empty name gracefully', () => {
      const emptyNameTask = { ...mockTask, name: '' };
      
      render(
        <TaskContextMenu
          task={emptyNameTask}
          onEdit={mockCallbacks.onEdit}
          onDelete={mockCallbacks.onDelete}
          onArchive={mockCallbacks.onArchive}
          onExport={mockCallbacks.onExport}
          onComplete={mockCallbacks.onComplete}
          onExternalSubmit={mockCallbacks.onExternalSubmit}
          onClose={mockCallbacks.onClose}
          position={defaultPosition}
        />
      );

      // Should still render without errors
      expect(screen.getByText('Delete')).toBeTruthy();
      
      // Click delete to trigger confirmation with empty name
      fireEvent.click(screen.getByText('Delete'));
      expect(mockConfirm).toHaveBeenCalledWith('Are you sure you want to delete ""?');
    });
  });
});