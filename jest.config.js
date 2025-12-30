/** @type {import('jest').Config} */
export default {
  preset: 'ts-jest',
  testEnvironment: 'jsdom',
  setupFilesAfterEnv: ['<rootDir>/src/setupTests.ts'],
  moduleNameMapper: {
    '^@/(.*)$': '<rootDir>/frontend/$1',
    '^@components/(.*)$': '<rootDir>/frontend/components/$1',
    '^@hooks/(.*)$': '<rootDir>/frontend/hooks/$1',
    '^@store/(.*)$': '<rootDir>/frontend/store/$1',
    '^@types/(.*)$': '<rootDir>/frontend/types/$1',
    '^@utils/(.*)$': '<rootDir>/frontend/utils/$1',
    '^@services/(.*)$': '<rootDir>/frontend/services/$1',
    '\.(css|less|scss|sass)$': 'identity-obj-proxy',
  },
  transform: {
    '^.+\\.tsx?$': ['ts-jest', {
      tsconfig: {
        jsx: 'react-jsx',
      },
    }],
  },
  moduleFileExtensions: ['ts', 'tsx', 'js', 'jsx', 'json', 'node'],
  testMatch: [
    '**/__tests__/**/*.(ts|tsx)',
    '**/*.(test|spec).(ts|tsx)',
  ],
  collectCoverageFrom: [
    'src/**/*.{ts,tsx}',
    '!src/**/*.d.ts',
    '!src/index.tsx',
    '!src/vite-env.d.ts',
  ],
  coverageThreshold: {
    global: {
      branches: 80,
      functions: 80,
      lines: 80,
      statements: 80,
    },
  },
};