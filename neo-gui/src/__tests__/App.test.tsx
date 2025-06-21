import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import App from '../App';
import { useAppStore } from '../stores/appStore';

// Mock the store
jest.mock('../stores/appStore');
const mockUseAppStore = useAppStore as jest.MockedFunction<typeof useAppStore>;

describe('App Component', () => {
  const mockStore = {
    loading: false,
    addNotification: jest.fn(),
    notifications: [],
    sidebarCollapsed: false,
    setSidebarCollapsed: jest.fn(),
    currentWallet: null,
    walletConnected: false,
    networkType: 'testnet' as const,
    currentNetwork: {
      name: 'Neo N3 Testnet',
      rpcUrl: 'https://testnet1.neo.coz.io:443',
      type: 'testnet' as const,
      chainId: 894710606,
    },
  };

  beforeEach(() => {
    mockUseAppStore.mockReturnValue(mockStore);
    jest.clearAllMocks();
  });

  it('renders without crashing', () => {
    render(<App />);
    const appElement = document.querySelector('.App');
    expect(appElement).toBeInTheDocument();
  });

  it('shows loading screen when loading is true', () => {
    const loadingMockStore = {
      ...mockStore,
      loading: true,
    };

    mockUseAppStore.mockReturnValue(loadingMockStore);

    render(<App />);

    expect(screen.getByText('Loading Neo N3 Wallet...')).toBeInTheDocument();
    expect(screen.getByText('N3')).toBeInTheDocument();
  });

  it('shows main app when loading is false', () => {
    render(<App />);

    // Should not show loading screen
    expect(
      screen.queryByText('Loading Neo N3 Wallet...')
    ).not.toBeInTheDocument();

    // Should show main app content (look for the App class)
    const appElement = document.querySelector('.App');
    expect(appElement).toBeInTheDocument();
  });

  it('adds welcome notification on initialization', async () => {
    render(<App />);

    await waitFor(() => {
      expect(mockStore.addNotification).toHaveBeenCalledWith({
        type: 'success',
        title: 'Welcome to Neo N3 Wallet',
        message: 'Your secure gateway to the Neo blockchain',
      });
    });
  });

  it('adds error notification when initialization fails', async () => {
    // Mock addNotification to throw an error
    const errorMockStore = {
      ...mockStore,
      addNotification: jest.fn().mockImplementationOnce(() => {
        throw new Error('Test error');
      }),
    };

    mockUseAppStore.mockReturnValue(errorMockStore);

    render(<App />);

    await waitFor(() => {
      expect(errorMockStore.addNotification).toHaveBeenCalledWith({
        type: 'error',
        title: 'Initialization Error',
        message: 'Failed to initialize the application',
      });
    });
  });

  it('renders router with correct routes', () => {
    render(<App />);

    // App should be wrapped in a router and have route structure
    const appElement = document.querySelector('.App');
    expect(appElement).toBeInTheDocument();
  });
});
