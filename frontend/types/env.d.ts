/// <reference types="vite/client" />

/**
 * Environment Variable Type Definitions
 * This file provides TypeScript type definitions for all environment variables
 * used in the HyperReview application.
 */

interface ImportMetaEnv {
  // Build Mode
  readonly MODE: 'development' | 'production' | 'test';

  // API Configuration
  readonly VITE_API_BASE_URL: string;
  readonly VITE_API_TIMEOUT: string;
  readonly VITE_API_RETRIES: string;

  // Tauri Configuration
  readonly VITE_TAURI_INVOKE_TIMEOUT: string;
  readonly VITE_TAURI_MAX_CONCURRENCY: string;

  // Performance Configuration
  readonly VITE_ENABLE_METRICS: string;
  readonly VITE_ENABLE_MEMORY_TRACKING: string;
  readonly VITE_MAX_CACHE_SIZE: string;
  readonly VITE_CACHE_TTL: string;

  // Security Configuration
  readonly VITE_ENABLE_ENCRYPTION: string;
  readonly VITE_ENABLE_CSP: string;
  readonly VITE_ALLOWED_ORIGINS: string;

  // Feature Flags
  readonly VITE_ENABLE_ANALYTICS: string;
  readonly VITE_ENABLE_TELEMETRY: string;
  readonly VITE_ENABLE_EXTERNAL_INTEGRATIONS: string;
  readonly VITE_ENABLE_EXPERIMENTAL: string;

  // UI Configuration
  readonly VITE_UI_THEME: 'dark' | 'light' | 'system';
  readonly VITE_ANIMATIONS_ENABLED: string;
  readonly VITE_COMPACT_MODE: string;

  // Development Configuration
  readonly VITE_DEBUG: string;
  readonly VITE_VERBOSE_LOGGING: string;
  readonly VITE_DEV_PORT: string;

  // Production Configuration
  readonly VITE_ENABLE_SOURCE_MAPS: string;
  readonly VITE_ENABLE_SERVICE_WORKER: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}

/**
 * Environment Variable Utilities
 */
export const env = {
  // Boolean conversion helpers
  toBoolean: (value: string | undefined, defaultValue = false): boolean => {
    if (value === undefined) return defaultValue;
    return value === 'true' || value === '1';
  },

  toNumber: (value: string | undefined, defaultValue = 0): number => {
    if (value === undefined) return defaultValue;
    const parsed = parseInt(value, 10);
    return isNaN(parsed) ? defaultValue : parsed;
  },

  toString: (value: string | undefined, defaultValue = ''): string => {
    return value ?? defaultValue;
  },

  toArray: (value: string | undefined, defaultValue: string[] = []): string[] => {
    if (!value) return defaultValue;
    return value.split(',').map(v => v.trim()).filter(Boolean);
  },

  // Specific type conversions for common env vars
  isDevelopment: (): boolean => {
    return import.meta.env.MODE === 'development';
  },

  isProduction: (): boolean => {
    return import.meta.env.MODE === 'production';
  },

  isTest: (): boolean => {
    return import.meta.env.MODE === 'test';
  },

  // Feature flags
  isFeatureEnabled: (feature: string): boolean => {
    const key = `VITE_ENABLE_${feature.toUpperCase()}`;
    return env.toBoolean(import.meta.env[key as keyof ImportMetaEnv]);
  },

  // Get numeric env var
  getNumeric: (name: keyof ImportMetaEnv, defaultValue: number): number => {
    return env.toNumber(import.meta.env[name], defaultValue);
  },

  // Get string env var
  getString: (name: keyof ImportMetaEnv, defaultValue: string): string => {
    return env.toString(import.meta.env[name], defaultValue);
  },

  // Get boolean env var
  getBoolean: (name: keyof ImportMetaEnv, defaultValue = false): boolean => {
    return env.toBoolean(import.meta.env[name], defaultValue);
  },

  // Get array env var
  getArray: (name: keyof ImportMetaEnv, defaultValue: string[] = []): string[] => {
    return env.toArray(import.meta.env[name], defaultValue);
  }
};

/**
 * Type-safe environment variable access
 */
export type EnvVars = {
  [K in keyof ImportMetaEnv]: string;
};

export type EnvVarValue<K extends keyof ImportMetaEnv> = ImportMetaEnv[K];

/**
 * Validate required environment variables
 */
export function validateEnvVars(required: (keyof ImportMetaEnv)[]): void {
  const missing: string[] = [];

  for (const key of required) {
    if (!import.meta.env[key]) {
      missing.push(key);
    }
  }

  if (missing.length > 0) {
    throw new Error(
      `Missing required environment variables:\n${missing.join('\n')}\n\n` +
      'Please check your .env file or environment configuration.'
    );
  }
}

/**
 * Validate optional environment variables with custom validators
 */
export function validateEnvVar<K extends keyof ImportMetaEnv>(
  key: K,
  validator: (value: string) => boolean,
  errorMessage: string
): void {
  const value = import.meta.env[key];
  if (value && !validator(value)) {
    throw new Error(`Invalid value for ${key}: ${errorMessage}`);
  }
}

/**
 * Common validation functions
 */
export const validators = {
  isUrl: (value: string): boolean => {
    try {
      new URL(value);
      return true;
    } catch {
      return false;
    }
  },

  isPositiveNumber: (value: string): boolean => {
    const num = parseInt(value, 10);
    return !isNaN(num) && num >= 0;
  },

  isPort: (value: string): boolean => {
    const port = parseInt(value, 10);
    return !isNaN(port) && port > 0 && port <= 65535;
  },

  isTheme: (value: string): value is 'dark' | 'light' | 'system' => {
    return ['dark', 'light', 'system'].includes(value);
  }
};

// Example usage:
/*
import { env, validateEnvVars, validators } from './types/env';

// Check if feature is enabled
if (env.isFeatureEnabled('analytics')) {
  // Initialize analytics
}

// Get numeric value
const timeout = env.getNumeric('VITE_API_TIMEOUT', 30000);

// Validate required vars
validateEnvVars([
  'VITE_API_BASE_URL',
  'VITE_TAURI_INVOKE_TIMEOUT'
]);

// Validate specific var
validateEnvVar('VITE_API_BASE_URL', validators.isUrl, 'Must be a valid URL');
validateEnvVar('VITE_UI_THEME', validators.isTheme, 'Must be dark, light, or system');
*/
