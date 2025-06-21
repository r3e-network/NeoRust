import React, { ReactElement } from 'react';
import { render, RenderOptions } from '@testing-library/react';
import { BrowserRouter } from 'react-router-dom';

// Custom render function that includes providers
const AllTheProviders = ({ children }: { children: React.ReactNode }) => {
  return <>{children}</>;
};

// Render with router for components that need routing
const WithRouter = ({ children }: { children: React.ReactNode }) => {
  return <BrowserRouter>{children}</BrowserRouter>;
};

const customRender = (
  ui: ReactElement,
  options?: Omit<RenderOptions, 'wrapper'>
) => render(ui, { wrapper: AllTheProviders, ...options });

const renderWithRouter = (
  ui: ReactElement,
  options?: Omit<RenderOptions, 'wrapper'>
) => render(ui, { wrapper: WithRouter, ...options });

export * from '@testing-library/react';
export { customRender as render, renderWithRouter };

// Mock data generators
export const generateMockWallet = (overrides = {}) => ({
  address: 'NQyJR9dCZeYJNgK7N5Z9vHW3pJy4QfqPqY',
  balance: {
    NEO: '100',
    GAS: '1000.5',
  },
  isConnected: true,
  ...overrides,
});

export const generateMockNotification = (overrides = {}) => ({
  id: Math.random().toString(36).substr(2, 9),
  type: 'success' as const,
  title: 'Test Notification',
  message: 'This is a test notification',
  timestamp: Date.now(),
  ...overrides,
});

export const generateMockTransaction = (overrides = {}) => ({
  hash: '0x1234567890abcdef',
  blockIndex: 123456,
  vmState: 'HALT',
  gasConsumed: '1.23456789',
  size: 456,
  timestamp: Date.now(),
  ...overrides,
});

// Test IDs for easier element selection
export const testIds = {
  app: 'app',
  dashboard: 'dashboard',
  wallet: 'wallet',
  navigation: 'navigation',
  notificationCenter: 'notification-center',
  loadingScreen: 'loading-screen',
} as const;
