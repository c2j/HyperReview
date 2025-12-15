/**
 * Environment Configuration for HyperReview
 * Handles environment variables and configuration for different build modes
 */

export enum Environment {
  DEVELOPMENT = 'development',
  PRODUCTION = 'production',
  TEST = 'test'
}

export interface AppConfig {
  env: Environment;
  isDevelopment: boolean;
  isProduction: boolean;
  isTest: boolean;

  // API Configuration
  api: {
    baseUrl: string;
    timeout: number;
    retries: number;
  };

  // Tauri Configuration
  tauri: {
    invokeTimeout: number;
    maxConcurrency: number;
  };

  // Performance Configuration
  performance: {
    enableMetrics: boolean;
    enableMemoryTracking: boolean;
    maxCacheSize: number;
    cacheTTL: number;
  };

  // Security Configuration
  security: {
    enableEncryption: boolean;
    enableCSP: boolean;
    allowedOrigins: string[];
  };

  // Feature Flags
  features: {
    enableAnalytics: boolean;
    enableTelemetry: boolean;
    enableExternalIntegrations: boolean;
    enableExperimentalFeatures: boolean;
  };

  // UI Configuration
  ui: {
    theme: 'dark' | 'light' | 'system';
    animationsEnabled: boolean;
    compactMode: boolean;
  };
}

// Default configuration
const defaultConfig: AppConfig = {
  env: (import.meta.env.MODE as Environment) || Environment.DEVELOPMENT,
  isDevelopment: import.meta.env.DEV,
  isProduction: import.meta.env.PROD,
  isTest: import.meta.env.MODE === 'test',

  api: {
    baseUrl: import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080',
    timeout: parseInt(import.meta.env.VITE_API_TIMEOUT || '30000'),
    retries: parseInt(import.meta.env.VITE_API_RETRIES || '3')
  },

  tauri: {
    invokeTimeout: parseInt(import.meta.env.VITE_TAURI_INVOKE_TIMEOUT || '30000'),
    maxConcurrency: parseInt(import.meta.env.VITE_TAURI_MAX_CONCURRENCY || '10')
  },

  performance: {
    enableMetrics: import.meta.env.VITE_ENABLE_METRICS === 'true',
    enableMemoryTracking: import.meta.env.VITE_ENABLE_MEMORY_TRACKING === 'true',
    maxCacheSize: parseInt(import.meta.env.VITE_MAX_CACHE_SIZE || '100'),
    cacheTTL: parseInt(import.meta.env.VITE_CACHE_TTL || '3600')
  },

  security: {
    enableEncryption: import.meta.env.VITE_ENABLE_ENCRYPTION === 'true',
    enableCSP: import.meta.env.VITE_ENABLE_CSP === 'true',
    allowedOrigins: (import.meta.env.VITE_ALLOWED_ORIGINS || '').split(',').filter(Boolean)
  },

  features: {
    enableAnalytics: import.meta.env.VITE_ENABLE_ANALYTICS !== 'false',
    enableTelemetry: import.meta.env.VITE_ENABLE_TELEMETRY === 'true',
    enableExternalIntegrations: import.meta.env.VITE_ENABLE_EXTERNAL_INTEGRATIONS !== 'false',
    enableExperimentalFeatures: import.meta.env.VITE_ENABLE_EXPERIMENTAL === 'true'
  },

  ui: {
    theme: (import.meta.env.VITE_UI_THEME as 'dark' | 'light' | 'system') || 'dark',
    animationsEnabled: import.meta.env.VITE_ANIMATIONS_ENABLED !== 'false',
    compactMode: import.meta.env.VITE_COMPACT_MODE === 'true'
  }
};

// Environment-specific overrides
const envConfigs: Record<Environment, Partial<AppConfig>> = {
  [Environment.DEVELOPMENT]: {
    api: {
      baseUrl: import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080',
      timeout: 60000,
      retries: 5
    },
    performance: {
      enableMetrics: true,
      enableMemoryTracking: true,
      maxCacheSize: parseInt(import.meta.env.VITE_MAX_CACHE_SIZE || '100'),
      cacheTTL: parseInt(import.meta.env.VITE_CACHE_TTL || '3600')
    },
    security: {
      enableEncryption: import.meta.env.VITE_ENABLE_ENCRYPTION === 'true',
      enableCSP: false,
      allowedOrigins: (import.meta.env.VITE_ALLOWED_ORIGINS || '').split(',').filter(Boolean)
    },
    features: {
      enableAnalytics: import.meta.env.VITE_ENABLE_ANALYTICS !== 'false',
      enableTelemetry: true,
      enableExternalIntegrations: import.meta.env.VITE_ENABLE_EXTERNAL_INTEGRATIONS !== 'false',
      enableExperimentalFeatures: import.meta.env.VITE_ENABLE_EXPERIMENTAL === 'true'
    }
  },

  [Environment.PRODUCTION]: {
    api: {
      baseUrl: import.meta.env.VITE_API_BASE_URL || 'https://api.hyperreview.com',
      timeout: 30000,
      retries: 3
    },
    performance: {
      enableMetrics: true,
      enableMemoryTracking: false,
      maxCacheSize: parseInt(import.meta.env.VITE_MAX_CACHE_SIZE || '500'),
      cacheTTL: parseInt(import.meta.env.VITE_CACHE_TTL || '7200')
    },
    security: {
      enableEncryption: import.meta.env.VITE_ENABLE_ENCRYPTION === 'true',
      enableCSP: true,
      allowedOrigins: (import.meta.env.VITE_ALLOWED_ORIGINS || 'https://hyperreview.com').split(',').filter(Boolean)
    },
    features: {
      enableAnalytics: import.meta.env.VITE_ENABLE_ANALYTICS !== 'false',
      enableTelemetry: false,
      enableExternalIntegrations: import.meta.env.VITE_ENABLE_EXTERNAL_INTEGRATIONS !== 'false',
      enableExperimentalFeatures: import.meta.env.VITE_ENABLE_EXPERIMENTAL === 'true'
    }
  },

  [Environment.TEST]: {
    api: {
      baseUrl: 'http://localhost:3001',
      timeout: 10000,
      retries: 1
    },
    performance: {
      enableMetrics: false,
      enableMemoryTracking: false,
      maxCacheSize: 10,
      cacheTTL: 60
    },
    security: {
      enableEncryption: false,
      enableCSP: false,
      allowedOrigins: []
    },
    features: {
      enableAnalytics: false,
      enableTelemetry: false,
      enableExternalIntegrations: false,
      enableExperimentalFeatures: false
    }
  }
};

// Merge configurations
const mergedConfig = (() => {
  const env = defaultConfig.env;
  const envOverride = envConfigs[env];

  return {
    ...defaultConfig,
    ...envOverride,
    api: {
      ...defaultConfig.api,
      ...(envOverride.api || {})
    },
    tauri: {
      ...defaultConfig.tauri,
      ...(envOverride.tauri || {})
    },
    performance: {
      ...defaultConfig.performance,
      ...(envOverride.performance || {})
    },
    security: {
      ...defaultConfig.security,
      ...(envOverride.security || {})
    },
    features: {
      ...defaultConfig.features,
      ...(envOverride.features || {})
    },
    ui: {
      ...defaultConfig.ui,
      ...(envOverride.ui || {})
    }
  } as AppConfig;
})();

// Validation function
export function validateConfig(config: AppConfig): void {
  const errors: string[] = [];

  if (!config.api.baseUrl) {
    errors.push('VITE_API_BASE_URL is required');
  }

  if (config.api.timeout < 0) {
    errors.push('VITE_API_TIMEOUT must be positive');
  }

  if (config.performance.maxCacheSize < 0) {
    errors.push('VITE_MAX_CACHE_SIZE must be positive');
  }

  if (errors.length > 0) {
    throw new Error(`Configuration validation failed:\n${errors.join('\n')}`);
  }
}

// Export singleton instance
let appConfig: AppConfig | null = null;

export function getConfig(): AppConfig {
  if (!appConfig) {
    validateConfig(mergedConfig);
    appConfig = mergedConfig;
  }
  return appConfig;
}

// Helper functions
export const isDev = () => getConfig().isDevelopment;
export const isProd = () => getConfig().isProduction;
export const isTest = () => getConfig().isTest;

export const getEnv = () => getConfig().env;
export const getApiConfig = () => getConfig().api;
export const getTauriConfig = () => getConfig().tauri;
export const getPerformanceConfig = () => getConfig().performance;
export const getSecurityConfig = () => getConfig().security;
export const getFeatureFlags = () => getConfig().features;
export const getUIConfig = () => getConfig().ui;

// Initialize configuration on module load
if (typeof window !== 'undefined') {
  getConfig();

  // Log configuration in development
  if (isDev()) {
    console.log('HyperReview Configuration:', {
      env: getEnv(),
      features: getFeatureFlags(),
      performance: getPerformanceConfig()
    });
  }
}
