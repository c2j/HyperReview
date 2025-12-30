import '@testing-library/jest-dom';

// Mock Tauri API
jest.mock('@tauri-apps/api/tauri', () => ({
  invoke: jest.fn(),
  convertFileSrc: jest.fn(),
}));

jest.mock('@tauri-apps/api/event', () => ({
  listen: jest.fn(),
  emit: jest.fn(),
}));

// Mock environment
process.env.NODE_ENV = 'test';