/**
 * Gerrit Instance Management Service
 * 
 * Frontend service for managing Gerrit server instances with comprehensive
 * error handling, validation, and state management.
 */

import { invoke } from '@tauri-apps/api/tauri';
import { 
  GerritInstance, 
  GerritInstanceService, 
  CreateInstanceParams, 
  CreateInstanceResult,
  ConnectionTestResult,
  ValidationError,
  ApiError,
  GerritErrorCode,
  isApiError
} from '../api/types/gerrit';


/**
 * Gerrit Instance Service Implementation
 */
export class GerritInstanceServiceImpl implements GerritInstanceService {
  private instances: Map<string, GerritInstance> = new Map();
  private activeInstanceId: string | null = null;
  private listeners: Set<(instance: GerritInstance | null) => void> = new Set();

  constructor() {
    this.loadInstances();
  }

  /**
   * Get all configured Gerrit instances
   */
  async getInstances(includeInactive: boolean = false): Promise<GerritInstance[]> {
    try {
      const response = await invoke<{ success: boolean; instances: GerritInstance[] }>('gerrit_get_instances', {
        params: { includeInactive }
      });

      if (!response.success) {
        throw new Error('Failed to fetch instances');
      }

      // Update local cache
      this.instances.clear();
      response.instances.forEach(instance => {
        this.instances.set(instance.id, instance);
      });

      return response.instances;
    } catch (error) {
      this.handleError('Failed to get instances', error);
      return [];
    }
  }

  /**
   * Create a new Gerrit instance configuration
   */
  async createInstance(params: CreateInstanceParams): Promise<CreateInstanceResult> {
    try {
      // Validate parameters
      const validationErrors = this.validateInstanceParams(params);
      if (validationErrors.length > 0) {
        throw this.createValidationError(validationErrors);
      }

      const response = await invoke<CreateInstanceResult>('gerrit_create_instance', {
        params
      });

      if (response.success) {
        // Update local cache
        this.instances.set(response.instance.id, response.instance);
        
        // Set as active if it's the first instance
        if (this.instances.size === 1) {
          await this.setActiveInstance(response.instance.id);
        }
      }

      return response;
    } catch (error) {
      this.handleError('Failed to create instance', error);
      throw error;
    }
  }

  /**
   * Update existing instance configuration
   */
  async updateInstance(id: string, updates: Partial<GerritInstance>): Promise<GerritInstance> {
    try {
      const instance = this.instances.get(id);
      if (!instance) {
        throw new Error(`Instance not found: ${id}`);
      }

      // Validate updates
      const updatedInstance = { ...instance, ...updates };
      const validationErrors = this.validateInstanceConfig(updatedInstance);
      if (validationErrors.length > 0) {
        throw this.createValidationError(validationErrors);
      }

      // Note: This would typically call a backend update command
      // For now, we'll update the local cache
      this.instances.set(id, updatedInstance);
      
      return updatedInstance;
    } catch (error) {
      this.handleError('Failed to update instance', error);
      throw error;
    }
  }

  /**
   * Delete Gerrit instance configuration
   */
  async deleteInstance(id: string): Promise<void> {
    try {
      if (!this.instances.has(id)) {
        throw new Error(`Instance not found: ${id}`);
      }

      // Note: This would typically call a backend delete command
      // For now, we'll just remove from local cache
      this.instances.delete(id);
      
      // Clear active instance if it was deleted
      if (this.activeInstanceId === id) {
        this.activeInstanceId = null;
        this.notifyActiveInstanceChange(null);
      }
    } catch (error) {
      this.handleError('Failed to delete instance', error);
      throw error;
    }
  }

  /**
   * Test connection to Gerrit instance
   */
  async testConnection(instanceId: string): Promise<ConnectionTestResult> {
    try {
      const response = await invoke<ConnectionTestResult>('gerrit_test_connection_by_id', {
        instance_id: instanceId
      });

      // Update instance status in cache
      const instance = this.instances.get(instanceId);
      if (instance) {
        instance.connectionStatus = response.success ? 'Connected' : 'Disconnected';
        if (response.success) {
          instance.lastConnected = new Date().toISOString();
          if (response.version) {
            instance.version = response.version;
          }
        }
        this.instances.set(instanceId, instance);
      }

      return response;
    } catch (error) {
      this.handleError('Connection test failed', error);
      throw error;
    }
  }

  /**
   * Set the active Gerrit instance
   */
  async setActiveInstance(instanceId: string): Promise<void> {
    try {
      if (!this.instances.has(instanceId)) {
        throw new Error(`Instance not found: ${instanceId}`);
      }

      // Call backend to persist the active state
      const success = await invoke<boolean>('gerrit_set_active_instance_simple', {
        instance_id: instanceId
      });

      if (!success) {
        throw new Error('Failed to set active instance in database');
      }

      // Update local cache - deactivate all instances
      this.instances.forEach(instance => {
        instance.isActive = false;
        this.instances.set(instance.id, instance);
      });

      // Set the new active instance
      const activeInstance = this.instances.get(instanceId)!;
      activeInstance.isActive = true;
      this.instances.set(instanceId, activeInstance);
      
      this.activeInstanceId = instanceId;
      this.notifyActiveInstanceChange(activeInstance);
    } catch (error) {
      this.handleError('Failed to set active instance', error);
      throw error;
    }
  }

  /**
   * Get the currently active Gerrit instance
   */
  async getActiveInstance(): Promise<GerritInstance | null> {
    if (!this.activeInstanceId) {
      return null;
    }
    return this.instances.get(this.activeInstanceId) || null;
  }

  /**
   * Validate instance configuration
   */
  validateInstanceConfig(config: Partial<GerritInstance>): ValidationError[] {
    // Import validation function from utils
    const { validateInstanceConfig: validateConfig } = require('../utils/validation');
    return validateConfig(config);
  }

  /**
   * Subscribe to active instance changes
   */
  subscribe(callback: (instance: GerritInstance | null) => void): () => void {
    this.listeners.add(callback);
    return () => {
      this.listeners.delete(callback);
    };
  }

  /**
   * Load instances from backend
   */
  private async loadInstances(): Promise<void> {
    try {
      const instances = await this.getInstances(true);
      
      // Find active instance
      const activeInstance = instances.find(i => i.isActive);
      this.activeInstanceId = activeInstance?.id || null;
      
      // Notify listeners
      this.notifyActiveInstanceChange(activeInstance || null);
    } catch (error) {
      console.error('Failed to load instances:', error);
    }
  }

  /**
   * Validate instance parameters
   */
  private validateInstanceParams(params: CreateInstanceParams): ValidationError[] {
    const errors: ValidationError[] = [];

    // Validate name
    if (!params.name || params.name.length < 3 || params.name.length > 50) {
      errors.push({
        field: 'name',
        message: 'Instance name must be between 3 and 50 characters'
      });
    } else if (!/^[a-zA-Z0-9\s\-]+$/.test(params.name)) {
      errors.push({
        field: 'name',
        message: 'Instance name can only contain alphanumeric characters, spaces, and hyphens'
      });
    }

    // Validate URL
    if (!params.url) {
      errors.push({
        field: 'url',
        message: 'Instance URL is required'
      });
    } else {
      try {
        const url = new URL(params.url);
        if (url.protocol !== 'https:') {
          errors.push({
            field: 'url',
            message: 'URL must use HTTPS protocol'
          });
        }
      } catch {
        errors.push({
          field: 'url',
          message: 'Invalid URL format'
        });
      }
    }

    // Validate username
    if (!params.username || params.username.length < 1 || params.username.length > 100) {
      errors.push({
        field: 'username',
        message: 'Username must be between 1 and 100 characters'
      });
    } else if (/\s/.test(params.username)) {
      errors.push({
        field: 'username',
        message: 'Username cannot contain whitespace'
      });
    }

    // Validate password
    if (!params.password || params.password.length < 1 || params.password.length > 500) {
      errors.push({
        field: 'password',
        message: 'Password must be between 1 and 500 characters'
      });
    }

    // Validate polling interval if provided
    if (params.pollingInterval !== undefined) {
      if (params.pollingInterval < 60 || params.pollingInterval > 3600) {
        errors.push({
          field: 'pollingInterval',
          message: 'Polling interval must be between 60 and 3600 seconds'
        });
      }
    }

    // Validate max changes if provided
    if (params.maxChanges !== undefined) {
      if (params.maxChanges < 1 || params.maxChanges > 500) {
        errors.push({
          field: 'maxChanges',
          message: 'Max changes must be between 1 and 500'
        });
      }
    }

    return errors;
  }

  /**
   * Create validation error
   */
  private createValidationError(errors: ValidationError[]): ApiError {
    return {
      success: false,
      errorCode: GerritErrorCode.INVALID_INSTANCE_NAME,
      message: 'Validation failed',
      details: { validationErrors: errors },
      timestamp: new Date().toISOString()
    };
  }

  /**
   * Handle errors consistently
   */
  private handleError(context: string, error: unknown): void {
    if (isApiError(error)) {
      console.error(`${context}:`, error.message);
      throw error;
    } else if (error instanceof Error) {
      const apiError: ApiError = {
        success: false,
        errorCode: GerritErrorCode.UNKNOWN_ERROR,
        message: `${context}: ${error.message}`,
        timestamp: new Date().toISOString()
      };
      console.error(apiError.message);
      throw apiError;
    } else {
      const apiError: ApiError = {
        success: false,
        errorCode: GerritErrorCode.UNKNOWN_ERROR,
        message: `${context}: Unknown error`,
        timestamp: new Date().toISOString()
      };
      console.error(apiError.message);
      throw apiError;
    }
  }

  /**
   * Notify listeners of active instance change
   */
  private notifyActiveInstanceChange(instance: GerritInstance | null): void {
    this.listeners.forEach(callback => {
      try {
        callback(instance);
      } catch (error) {
        console.error('Error in active instance change listener:', error);
      }
    });
  }
}

/**
 * Factory function to create Gerrit instance service
 */
export function createGerritInstanceService(): GerritInstanceService {
  return new GerritInstanceServiceImpl();
}