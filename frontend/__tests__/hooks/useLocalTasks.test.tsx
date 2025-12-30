import { vi } from 'vitest';

// Mock Tauri API
vi.mock('@tauri-apps/api/tauri', () => ({
  invoke: vi.fn(),
}));

// Mock the task store
vi.mock('@store/taskStore', () => ({
  useTaskStore: () => ({
    tasks: [],
    fetchTasks: vi.fn(),
    isLoading: false,
    error: null,
  }),
}));

// Mock useLocalTasks hook
vi.mock('@hooks/useLocalTasks', () => ({
  useLocalTasks: () => ({
    createTask: vi.fn().mockResolvedValue({
      id: 'test-task-id',
      name: 'Test Task',
      repo_path: '/test/repo',
      base_ref: 'main',
      status: 'in_progress',
      total_items: 2,
      completed_items: 0,
      items: [
        { file: 'src/main.rs', reviewed: false, comments: [] },
        { file: 'src/lib.rs', reviewed: false, comments: [] },
      ],
    }),
    listTasks: vi.fn().mockResolvedValue([]),
    getTask: vi.fn(),
    updateProgress: vi.fn(),
    deleteTask: vi.fn(),
    archiveTask: vi.fn(),
  }),
}));

describe('Local Task Management', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  test('should create a task from text input', async () => {
    const { invoke } = await import('@tauri-apps/api/tauri');
    const mockInvoke = invoke as ReturnType<typeof vi.fn>;
    
    mockInvoke.mockResolvedValue({
      id: 'test-task-id',
      name: 'Test Task',
      repo_path: '/test/repo',
      base_ref: 'main',
      status: 'in_progress',
      total_items: 2,
      completed_items: 0,
      items: [
        { file: 'src/main.rs', reviewed: false, comments: [] },
        { file: 'src/lib.rs', reviewed: false, comments: [] },
      ],
    });

    const { useLocalTasks } = await import('@hooks/useLocalTasks');
    const { createTask } = useLocalTasks();

    const result = await createTask(
      'Test Task',
      '/test/repo',
      'main',
      'src/main.rs\nsrc/lib.rs'
    );

    expect(mockInvoke).toHaveBeenCalledWith('create_task', {
      payload: {
        name: 'Test Task',
        repo_path: '/test/repo',
        base_ref: 'main',
        items_text: 'src/main.rs\nsrc/lib.rs',
      },
    });

    expect(result).toEqual({
      id: 'test-task-id',
      name: 'Test Task',
      repo_path: '/test/repo',
      base_ref: 'main',
      status: 'in_progress',
      total_items: 2,
      completed_items: 0,
      items: [
        { file: 'src/main.rs', reviewed: false, comments: [] },
        { file: 'src/lib.rs', reviewed: false, comments: [] },
      ],
    });
  });

  test('should list tasks', async () => {
    const { invoke } = await import('@tauri-apps/api/tauri');
    const mockInvoke = invoke as ReturnType<typeof vi.fn>;
    
    mockInvoke.mockResolvedValue([
      {
        id: 'task-1',
        name: 'Task 1',
        status: 'in_progress',
        progress: 0.5,
      },
      {
        id: 'task-2',
        name: 'Task 2',
        status: 'completed',
        progress: 1.0,
      },
    ]);

    const { useLocalTasks } = await import('@hooks/useLocalTasks');
    const { listTasks } = useLocalTasks();

    const result = await listTasks();

    expect(mockInvoke).toHaveBeenCalledWith('list_tasks');
    expect(result).toHaveLength(2);
    expect(result[0].name).toBe('Task 1');
    expect(result[1].name).toBe('Task 2');
  });

  test('should update task progress', async () => {
    const { invoke } = await import('@tauri-apps/api/tauri');
    const mockInvoke = invoke as ReturnType<typeof vi.fn>;
    
    mockInvoke.mockResolvedValue(undefined);

    const { useLocalTasks } = await import('@hooks/useLocalTasks');
    const { updateProgress } = useLocalTasks();

    await updateProgress('task-1', 0, true);

    expect(mockInvoke).toHaveBeenCalledWith('update_task_progress', {
      taskId: 'task-1',
      itemIndex: 0,
      reviewed: true,
    });
  });
});