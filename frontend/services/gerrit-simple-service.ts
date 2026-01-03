/**
 * Simple Gerrit service for testing
 */
import { invoke } from '@tauri-apps/api/tauri';

export interface SimpleGerritInstance {
  id: string;
  name: string;
  url: string;
  username: string;
  password?: string;
  is_active: boolean;
  status: string;
}

export interface SimpleCreateParams {
  name: string;
  url: string;
  username: string;
  password: string;
}

export interface SimpleChange {
  id: string;
  change_number: number;
  subject: string;
  status: string;
  project: string;
  branch: string;
  topic: string | null;
  owner: string;
  updated: string;
  created: string;
  insertions: number;
  deletions: number;
  files: SimpleFileInfo[];
}

export interface SimpleFileInfo {
  path: string;
  change_type: string;
  insertions: number;
  deletions: number;
}

const IMPORTED_CHANGES_KEY = 'gerrit_imported_changes';

/**
 * Simple Gerrit service for testing
 */
export class SimpleGerritService {
  private testMode = false;
  private importedChanges: SimpleChange[] = [];

  private loadChangesFromStorage(): SimpleChange[] {
    try {
      const stored = localStorage.getItem(IMPORTED_CHANGES_KEY);
      if (stored) {
        const changes = JSON.parse(stored);
        console.log('SimpleGerritService: Loaded changes from storage:', changes.length);
        return changes;
      }
    } catch (error) {
      console.error('SimpleGerritService: Failed to load changes from storage:', error);
    }
    return [];
  }

  private saveChangesToStorage(changes: SimpleChange[]): void {
    try {
      localStorage.setItem(IMPORTED_CHANGES_KEY, JSON.stringify(changes));
      console.log('SimpleGerritService: Saved changes to storage:', changes.length);
    } catch (error) {
      console.error('SimpleGerritService: Failed to save changes to storage:', error);
    }
  }

  constructor() {
    this.importedChanges = this.loadChangesFromStorage();
    console.log('SimpleGerritService: Loaded', this.importedChanges.length, 'imported changes');
  }

  async getInstances(): Promise<SimpleGerritInstance[]> {
    try {
      console.log('SimpleGerritService: Getting instances...');

      if (this.testMode) {
        const testInstances: SimpleGerritInstance[] = [
          {
            id: "test-instance-1",
            name: "Test Gerrit Server",
            url: "https://gerrit.example.com",
            username: "testuser",
            is_active: true,
            status: "Connected"
          },
          {
            id: "test-instance-2",
            name: "Development Gerrit",
            url: "https://dev-gerrit.example.com",
            username: "devuser",
            is_active: false,
            status: "Disconnected"
          }
        ];
        console.log('SimpleGerritService: Using test mode data');
        return testInstances;
      }

      const response = await invoke<{ success: boolean; instances: SimpleGerritInstance[] }>('gerrit_get_instances_simple');
      console.log('SimpleGerritService: Got response:', response);

      if (response.success) {
        return response.instances;
      } else {
        throw new Error('Failed to fetch instances');
      }
    } catch (error) {
      console.error('SimpleGerritService: Failed to get instances:', error);
      return [];
    }
  }

  async createInstance(params: SimpleCreateParams): Promise<SimpleGerritInstance | null> {
    try {
      console.log('SimpleGerritService: Creating instance with params:', params);

      if (this.testMode) {
        const newInstance: SimpleGerritInstance = {
          id: `test-instance-${Date.now()}`,
          name: params.name,
          url: params.url,
          username: params.username,
          password: params.password,
          is_active: false,
          status: "Connected"
        };
        console.log('SimpleGerritService: Created test instance:', newInstance);
        return newInstance;
      }

      const instance = await invoke<SimpleGerritInstance>(
        'gerrit_create_instance_simple',
        {
          name: params.name,
          url: params.url,
          username: params.username,
          password: params.password
        }
      );
      console.log('SimpleGerritService: Created instance:', instance);
      return instance;
    } catch (error) {
      console.error('SimpleGerritService: Failed to create instance:', error);
      return null;
    }
  }

  setTestMode(enabled: boolean) {
    this.testMode = enabled;
    console.log('SimpleGerritService: Test mode set to:', enabled);
  }

  isTestMode(): boolean {
    return this.testMode;
  }

  async deleteInstance(instanceId: string): Promise<boolean> {
    try {
      console.log('SimpleGerritService: Deleting instance:', instanceId);

      if (this.testMode) {
        console.log('SimpleGerritService: Instance deleted (test mode):', true);
        return true;
      }

      const result = await invoke<boolean>('gerrit_delete_instance_simple', { instanceId });
      console.log('SimpleGerritService: Delete result:', result);
      return result;
    } catch (error) {
      console.error('SimpleGerritService: Failed to delete instance:', error);
      return false;
    }
  }

  async importChange(changeId: string): Promise<SimpleChange | null> {
    try {
      console.log('SimpleGerritService: Importing change:', changeId);

      if (this.testMode) {
        const change: SimpleChange = {
          id: `I${changeId.replace('#', '')}`,
          change_number: parseInt(changeId.replace('#', '')),
          subject: `Change #${changeId.replace('#', '')}: Test change subject`,
          status: 'NEW',
          project: 'test-project',
          branch: 'master',
          topic: 'feature/test',
          owner: 'testuser',
          updated: new Date().toISOString(),
          created: new Date().toISOString(),
          insertions: 523,
          deletions: 189,
          files: [
            {
              path: 'src/main.ts',
              change_type: 'MODIFIED',
              insertions: 45,
              deletions: 12,
            },
            {
              path: 'src/utils.ts',
              change_type: 'MODIFIED',
              insertions: 89,
              deletions: 34,
            },
            {
              path: 'src/components/Header.tsx',
              change_type: 'ADDED',
              insertions: 156,
              deletions: 0,
            },
            {
              path: 'tests/integration.test.ts',
              change_type: 'ADDED',
              insertions: 233,
              deletions: 0,
            },
          ],
        };

        this.importedChanges.push(change);
        this.saveChangesToStorage(this.importedChanges);
        console.log('SimpleGerritService: Imported test change:', change);
        console.log('SimpleGerritService: Total imported changes:', this.importedChanges.length);
        return change;
      }

      const change = await invoke<SimpleChange>('gerrit_import_change_simple', { changeId });
      console.log('SimpleGerritService: Imported change:', change);
      this.importedChanges.push(change);
      this.saveChangesToStorage(this.importedChanges);
      return change;
    } catch (error) {
      console.error('SimpleGerritService: Failed to import change:', error);
      return null;
    }
  }

  async searchChanges(query: string): Promise<SimpleChange[]> {
    try {
      console.log('SimpleGerritService: Searching changes with query:', query);
      console.log('SimpleGerritService: Test mode:', this.testMode);

      if (this.testMode) {
        const changes: SimpleChange[] = [
          {
            id: 'I12345',
            change_number: 12345,
            subject: 'Implement user authentication flow',
            status: 'NEW',
            project: 'hyperreview',
            branch: 'feature/auth',
            topic: 'authentication',
            owner: 'alice',
            updated: new Date().toISOString(),
            created: new Date().toISOString(),
            insertions: 234,
            deletions: 67,
            files: [],
          },
          {
            id: 'I12346',
            change_number: 12346,
            subject: 'Fix memory leak in diff viewer',
            status: 'NEW',
            project: 'hyperreview',
            branch: 'bugfix/memory',
            topic: 'performance',
            owner: 'bob',
            updated: new Date().toISOString(),
            created: new Date().toISOString(),
            insertions: 12,
            deletions: 8,
            files: [],
          },
        ];

        console.log('SimpleGerritService: Search results (test mode):', changes);
        return changes;
      }

      const changes = await invoke<SimpleChange[]>('gerrit_search_changes_simple', { query });
      console.log('SimpleGerritService: Search results:', changes);
      return changes;
    } catch (error) {
      console.error('SimpleGerritService: Failed to search changes:', error);
      return [];
    }
  }

  getImportedChanges(): SimpleChange[] {
    console.log('SimpleGerritService: Getting imported changes:', this.importedChanges.length);
    return [...this.importedChanges];
  }

  clearImportedChanges(): void {
    console.log('SimpleGerritService: Clearing imported changes');
    this.importedChanges = [];
    this.saveChangesToStorage(this.importedChanges);
  }

  removeImportedChange(changeId: string): boolean {
    const initialLength = this.importedChanges.length;
    this.importedChanges = this.importedChanges.filter(c => c.id !== changeId);
    this.saveChangesToStorage(this.importedChanges);
    const removed = initialLength !== this.importedChanges.length;
    console.log('SimpleGerritService: Removed change:', removed);
    return removed;
  }

  updateImportedChange(change: SimpleChange): void {
    const index = this.importedChanges.findIndex(c => c.id === change.id);
    if (index !== -1) {
      this.importedChanges[index] = change;
      this.saveChangesToStorage(this.importedChanges);
      console.log('SimpleGerritService: Updated change:', change.id);
    }
  }
}

export const simpleGerritService = new SimpleGerritService();

export default simpleGerritService;
