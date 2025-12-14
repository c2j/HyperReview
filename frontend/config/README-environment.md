# Environment Configuration

Comprehensive environment configuration system for HyperReview, supporting development, production, and test environments with TypeScript type safety.

## Overview

The environment configuration system provides:

- **Type-safe environment variables** with TypeScript support
- **Multiple environment support** (development, production, test)
- **Validation** for required and optional environment variables
- **Feature flags** for controlling application behavior
- **Security settings** for production deployments
- **Performance tuning** for different environments

## Quick Start

### 1. Copy Environment Template

```bash
cp .env.example .env
```

### 2. Configure Variables

Edit `.env` file and set the required variables for your environment.

### 3. Use Configuration in Code

```typescript
import { getConfig, isDev } from './config/environment';

// Access configuration
const config = getConfig();
console.log(`Running in ${config.env} mode`);

// Check environment
if (isDev()) {
  console.log('Development mode');
}
```

## Environment Variables

### Build Mode

| Variable | Type | Description | Default |
|----------|------|-------------|---------|
| `MODE` | `string` | Build mode: `development`, `production`, or `test` | `development` |

### API Configuration

| Variable | Type | Description | Default |
|----------|------|-------------|---------|
| `VITE_API_BASE_URL` | `string` | Base URL for API endpoints | `http://localhost:8080` |
| `VITE_API_TIMEOUT` | `number` | Request timeout in milliseconds | `30000` |
| `VITE_API_RETRIES` | `number` | Number of retry attempts | `3` |

### Tauri Configuration

| Variable | Type | Description | Default |
|----------|------|-------------|---------|
| `VITE_TAURI_INVOKE_TIMEOUT` | `number` | IPC invoke timeout in milliseconds | `30000` |
| `VITE_TAURI_MAX_CONCURRENCY` | `number` | Max concurrent IPC operations | `10` |

### Performance Configuration

| Variable | Type | Description | Default |
|----------|------|-------------|---------|
| `VITE_ENABLE_METRICS` | `boolean` | Enable performance metrics | `true` |
| `VITE_ENABLE_MEMORY_TRACKING` | `boolean` | Enable memory usage tracking | `true` |
| `VITE_MAX_CACHE_SIZE` | `number` | Maximum cache size in MB | `100` |
| `VITE_CACHE_TTL` | `number` | Cache time-to-live in seconds | `3600` |

### Security Configuration

| Variable | Type | Description | Default |
|----------|------|-------------|---------|
| `VITE_ENABLE_ENCRYPTION` | `boolean` | Enable data encryption | `true` |
| `VITE_ENABLE_CSP` | `boolean` | Enable Content Security Policy | `false` |
| `VITE_ALLOWED_ORIGINS` | `string[]` | Allowed CORS origins | `http://localhost:1420` |

### Feature Flags

| Variable | Type | Description | Default |
|----------|------|-------------|---------|
| `VITE_ENABLE_ANALYTICS` | `boolean` | Enable analytics tracking | `true` |
| `VITE_ENABLE_TELEMETRY` | `boolean` | Enable telemetry | `false` |
| `VITE_ENABLE_EXTERNAL_INTEGRATIONS` | `boolean` | Enable GitLab/Gerrit integration | `true` |
| `VITE_ENABLE_EXPERIMENTAL` | `boolean` | Enable experimental features | `false` |

### UI Configuration

| Variable | Type | Description | Default |
|----------|------|-------------|---------|
| `VITE_UI_THEME` | `string` | Default theme: `dark`, `light`, or `system` | `dark` |
| `VITE_ANIMATIONS_ENABLED` | `boolean` | Enable animations | `true` |
| `VITE_COMPACT_MODE` | `boolean` | Use compact UI mode | `false` |

## Environment-Specific Files

The application uses different `.env` files based on the `MODE` variable:

- `.env.development` - Loaded when `MODE=development`
- `.env.production` - Loaded when `MODE=production`
- `.env.test` - Loaded when `MODE=test`
- `.env` - Always loaded (can override environment-specific files)

### Priority Order

1. System environment variables
2. `.env` file
3. Environment-specific `.env.*` file

## API Reference

### getConfig()

Returns the complete application configuration with environment-specific overrides.

```typescript
import { getConfig } from './config/environment';

const config = getConfig();
console.log(config.env); // 'development' | 'production' | 'test'
console.log(config.api.baseUrl);
console.log(config.performance.enableMetrics);
```

### Environment Check Functions

```typescript
import { isDev, isProd, isTest } from './config/environment';

if (isDev()) {
  // Development-specific code
}

if (isProd()) {
  // Production-specific code
}
```

### Configuration Accessors

```typescript
import {
  getEnv,
  getApiConfig,
  getTauriConfig,
  getPerformanceConfig,
  getSecurityConfig,
  getFeatureFlags,
  getUIConfig
} from './config/environment';

// Access specific configuration sections
const apiConfig = getApiConfig();
const tauriConfig = getTauriConfig();
const features = getFeatureFlags();
```

### Environment Helpers

```typescript
import { env } from '../types/env';

// Convert string to boolean
const isEnabled = env.toBoolean(import.meta.env.VITE_ENABLE_METRICS);

// Convert string to number
const timeout = env.toNumber(import.meta.env.VITE_API_TIMEOUT, 30000);

// Convert string to array
const origins = env.toArray(import.meta.env.VITE_ALLOWED_ORIGINS);

// Check if feature is enabled
const hasAnalytics = env.isFeatureEnabled('analytics');

// Get typed env var
const port = env.getNumeric('VITE_DEV_PORT', 1420);
const theme = env.getString('VITE_UI_THEME', 'dark');
const enableCSP = env.getBoolean('VITE_ENABLE_CSP', false);
```

### Type-Safe Environment Access

```typescript
import type { ImportMetaEnv } from '../types/env';

// Access with type safety
const apiUrl: string = import.meta.env.VITE_API_BASE_URL;
const timeout: number = parseInt(import.meta.env.VITE_API_TIMEOUT);
const isEnabled: boolean = import.meta.env.VITE_ENABLE_METRICS === 'true';
```

### Validation

```typescript
import {
  validateEnvVars,
  validateEnvVar,
  validators
} from '../types/env';

// Validate required variables
validateEnvVars([
  'VITE_API_BASE_URL',
  'VITE_TAURI_INVOKE_TIMEOUT'
]);

// Validate specific variable with custom validator
validateEnvVar(
  'VITE_API_BASE_URL',
  validators.isUrl,
  'Must be a valid URL'
);

validateEnvVar(
  'VITE_UI_THEME',
  validators.isTheme,
  'Must be dark, light, or system'
);

validateEnvVar(
  'VITE_DEV_PORT',
  validators.isPort,
  'Must be a valid port number (1-65535)'
);
```

## Usage Examples

### Basic Configuration

```typescript
import { getConfig } from './config/environment';

function App() {
  const config = getConfig();

  // Use configuration
  useEffect(() => {
    if (config.performance.enableMetrics) {
      initMetrics();
    }
  }, [config.performance.enableMetrics]);

  return <div>{/* Your app */}</div>;
}
```

### Feature Flags

```typescript
import { getFeatureFlags } from './config/environment';

function Feature() {
  const { enableExternalIntegrations } = getFeatureFlags();

  if (!enableExternalIntegrations) {
    return null;
  }

  return <ExternalIntegrationComponent />;
}
```

### API Configuration

```typescript
import { getApiConfig } from './config/environment';

function ApiClient() {
  const { baseUrl, timeout, retries } = getApiConfig();

  // Configure API client
  const client = new ApiClient({
    baseURL: baseUrl,
    timeout,
    retries
  });

  return client;
}
```

### Performance Configuration

```typescript
import { getPerformanceConfig } from './config/environment';

function CacheManager() {
  const { maxCacheSize, cacheTTL, enableMemoryTracking } = getPerformanceConfig();

  useEffect(() => {
    initCache({
      maxSize: maxCacheSize,
      ttl: cacheTTL,
      trackMemory: enableMemoryTracking
    });
  }, [maxCacheSize, cacheTTL, enableMemoryTracking]);
}
```

### Tauri Integration

```typescript
import { getTauriConfig } from './config/environment';
import { invoke } from '@tauri-apps/api/core';

function TauriService() {
  const { invokeTimeout } = getTauriConfig();

  const callCommand = async <T>(command: string, args?: any): Promise<T | null> => {
    try {
      const result = await invoke<T>(command, args);
      return result;
    } catch (error) {
      console.error(`Command ${command} failed:`, error);
      return null;
    }
  };

  return { callCommand };
}
```

### Theme Configuration

```typescript
import { getUIConfig } from './config/environment';

function ThemeProvider({ children }) {
  const { theme, animationsEnabled } = getUIConfig();

  useEffect(() => {
    // Apply theme to document
    document.documentElement.setAttribute('data-theme', theme);
    document.documentElement.setAttribute('data-animations', animationsEnabled.toString());
  }, [theme, animationsEnabled]);

  return <>{children}</>;
}
```

## Best Practices

### 1. Use Type-Safe Access

```typescript
// Good - Type-safe
import { getConfig } from './config/environment';
const config = getConfig();
console.log(config.env);

// Bad - Stringly-typed
const mode = import.meta.env.MODE;
```

### 2. Validate Environment Variables

```typescript
// Validate at startup
validateEnvVars([
  'VITE_API_BASE_URL',
  'VITE_TAURI_INVOKE_TIMEOUT'
]);
```

### 3. Use Environment-Specific Files

```bash
# Development
cp .env.example .env.development

# Production
cp .env.example .env.production
```

### 4. Don't Commit Sensitive Data

```bash
# .env files with secrets should be in .gitignore
echo ".env" >> .gitignore
echo ".env.local" >> .gitignore
```

### 5. Use Feature Flags

```typescript
// Enable/disable features without code changes
if (getFeatureFlags().enableExperimentalFeatures) {
  // Experimental feature code
}
```

### 6. Provide Defaults

```typescript
// Always provide sensible defaults
const timeout = env.getNumeric('VITE_API_TIMEOUT', 30000);
```

### 7. Document Configuration

Document all environment variables in `.env.example`:

```bash
# Description of the variable
# Options: value1, value2, value3
# Default: default-value
VITE_VARIABLE_NAME=default-value
```

## Configuration Validation

The system validates configuration on startup:

```typescript
// Automatically validates configuration when imported
import './config/environment';

// Manual validation
import { validateConfig } from './config/environment';

try {
  validateConfig(config);
  console.log('Configuration is valid');
} catch (error) {
  console.error('Configuration validation failed:', error);
}
```

## Troubleshooting

### Environment Variable Not Defined

```
Error: Missing required environment variables:
VITE_API_BASE_URL
```

**Solution**: Add the missing variable to your `.env` file.

### Invalid Value

```
Error: Invalid value for VITE_API_BASE_URL: Must be a valid URL
```

**Solution**: Provide a valid URL (e.g., `http://localhost:8080`).

### Configuration Not Loading

**Solution**:
1. Check that the `.env` file exists in the correct location
2. Verify the file is not in `.gitignore` (unless intentional)
3. Ensure variable names start with `VITE_` prefix
4. Restart the development server after changing `.env` files

### Type Errors

```
Type 'string | undefined' is not assignable to type 'string'
```

**Solution**:
```typescript
// Provide default value
const value = env.getString('VITE_VAR', 'default');
```

## Environment-Specific Configurations

### Development

```bash
# .env.development
MODE=development
VITE_DEBUG=true
VITE_VERBOSE_LOGGING=true
VITE_ENABLE_CSP=false
VITE_ENABLE_TELEMETRY=true
```

Optimized for:
- Fast development iteration
- Detailed logging and metrics
- Relaxed security (CSP disabled)
- Telemetry enabled

### Production

```bash
# .env.production
MODE=production
VITE_DEBUG=false
VITE_ENABLE_CSP=true
VITE_ENABLE_TELEMETRY=false
VITE_ENABLE_SOURCE_MAPS=false
```

Optimized for:
- Security (CSP enabled)
- Performance (metrics without telemetry)
- No debug information
- Source maps disabled

### Test

```bash
# .env.test
MODE=test
VITE_ENABLE_METRICS=false
VITE_ENABLE_MEMORY_TRACKING=false
VITE_ENABLE_EXTERNAL_INTEGRATIONS=false
```

Optimized for:
- Fast test execution
- No external dependencies
- Minimal overhead
- Isolated environment

## Integration with Tauri

The environment configuration integrates with Tauri through:

1. **tauri.conf.json** - Defines plugin permissions and security settings
2. **Environment variables** - Configure runtime behavior
3. **TypeScript types** - Type-safe access to configuration

### Example: IPC Timeout Configuration

```typescript
// Frontend
import { getTauriConfig } from './config/environment';
import { invoke } from '@tauri-apps/api/core';

const { invokeTimeout } = getTauriConfig();

await invoke('command', { timeout: invokeTimeout });
```

```rust
// Backend (Rust)
#[tauri::command]
async fn command(timeout: u64) -> Result<String, String> {
    // Use timeout from frontend
    tokio::time::sleep(Duration::from_millis(timeout)).await;
    Ok("Done".to_string())
}
```

## Files

- `config/environment.ts` - Main configuration module
- `types/env.d.ts` - TypeScript environment variable types
- `.env.example` - Template for all environment variables
- `.env.development` - Development environment defaults
- `.env.production` - Production environment defaults
- `config/README-environment.md` - This documentation

## See Also

- [Tauri Configuration](https://tauri.app/v1/guides/building/configuration)
- [Vite Environment Variables](https://vitejs.dev/guide/env-and-mode.html)
- [TypeScript Type Definitions](https://www.typescriptlang.org/docs/handbook/declaration-files/introduction.html)
