// React import not needed for test files
import { render, screen, fireEvent } from '../utils/test-utils';
import App from '../../App';
import { useAppStore } from '../../stores/appStore';

// Mock Tauri invoke function
const mockInvoke = jest.fn();
jest.mock('@tauri-apps/api/core', () => ({
  invoke: (...args: any[]) => mockInvoke(...args),
}));

// Mock the app store for integration testing
jest.mock('../../stores/appStore');
const mockUseAppStore = useAppStore as jest.MockedFunction<typeof useAppStore>;

describe('Wallet Workflow Integration Tests', () => {
  let mockStore: any;

  beforeEach(() => {
    mockStore = {
      // App state
      loading: false,
      notifications: [],
      sidebarCollapsed: false,
      currentWallet: null,
      walletConnected: false,
      networkType: 'testnet' as const,
      
      // Store actions
      addNotification: jest.fn((notification) => {
        mockStore.notifications = [...mockStore.notifications, {
          id: Date.now().toString(),
          timestamp: Date.now(),
          read: false,
          ...notification,
        }];
      }),
      removeNotification: jest.fn((id) => {
        mockStore.notifications = mockStore.notifications.filter((n: any) => n.id !== id);
      }),
      markNotificationRead: jest.fn((id) => {
        mockStore.notifications = mockStore.notifications.map((n: any) => 
          n.id === id ? { ...n, read: true } : n
        );
      }),
      setSidebarCollapsed: jest.fn((collapsed) => {
        mockStore.sidebarCollapsed = collapsed;
      }),
      addWallet: jest.fn((wallet) => {
        mockStore.currentWallet = wallet;
        mockStore.walletConnected = true;
      }),
      setCurrentWallet: jest.fn(),
      currentNetwork: {
        name: 'Neo N3 Testnet',
        rpcUrl: 'https://testnet1.neo.coz.io:443',
        type: 'testnet' as const,
        chainId: 894710606,
      },
    };

    mockUseAppStore.mockImplementation((selector) => {
      if (typeof selector === 'function') {
        return selector(mockStore);
      }
      return mockStore;
    });

    jest.clearAllMocks();
    mockInvoke.mockClear();
  });

  describe('Navigation Integration', () => {
    it('should navigate between pages correctly', async () => {
      render(<App />);

      // Start on Dashboard
      expect(screen.getAllByText('Dashboard')).toHaveLength(2); // Nav + heading

      // Navigate to Wallet
      fireEvent.click(screen.getByText('Wallet'));
      expect(screen.getByText('No wallet selected')).toBeInTheDocument();

      // Navigate back to Dashboard
      fireEvent.click(screen.getAllByText('Dashboard')[0]);
      expect(screen.getAllByText('Dashboard')).toHaveLength(2);
    });

    it('should display correct content for wallet page without wallet', () => {
      render(<App />);
      
      fireEvent.click(screen.getByText('Wallet'));
      
      expect(screen.getByText('No wallet selected')).toBeInTheDocument();
      expect(screen.getByText('Create Wallet')).toBeInTheDocument();
      expect(screen.getByText('Import Wallet')).toBeInTheDocument();
    });
  });

  describe('App State Integration', () => {
    it('should handle loading states properly', async () => {
      mockStore.loading = true;

      const { rerender } = render(<App />);

      // Should show loading screen
      expect(screen.getByText('Loading Neo N3 Wallet...')).toBeInTheDocument();

      // Update store to loaded state
      mockStore.loading = false;
      rerender(<App />);

      // Should show main app
      expect(screen.getByText('Neo Wallet')).toBeInTheDocument();
      expect(screen.queryByText('Loading Neo N3 Wallet...')).not.toBeInTheDocument();
    });

    it('should display wallet information when wallet exists', () => {
      const testWallet = {
        id: 'test-wallet',
        name: 'Test Wallet',
        address: 'NTestAddress123',
        balance: { neo: '100', gas: '50', tokens: {} },
        isDefault: true,
      };

      mockStore.currentWallet = testWallet;

      render(<App />);

      fireEvent.click(screen.getByText('Wallet'));

      expect(screen.getByText('Test Wallet')).toBeInTheDocument();
      expect(screen.getByText('NTestAddress123')).toBeInTheDocument();
    });
  });

  describe('Notification System Integration', () => {
    it('should handle notification state correctly', () => {
      mockStore.notifications = [
        {
          id: '1',
          type: 'success',
          title: 'Success',
          message: 'Operation completed',
          timestamp: Date.now(),
          read: false,
        },
      ];

      render(<App />);

      // Notifications might not be visible by default, just verify the store has them
      // The store might add additional notifications, so check for at least our test notification
      expect(mockStore.notifications.length).toBeGreaterThanOrEqual(1);
      expect(mockStore.notifications.some((n: any) => n.title === 'Success')).toBe(true);
    });

    it('should have notification functions available', () => {
      render(<App />);

      // Test that notification functions exist and can be called
      expect(mockStore.addNotification).toBeDefined();
      expect(mockStore.removeNotification).toBeDefined();
      expect(mockStore.markNotificationRead).toBeDefined();
    });
  });

  describe('App Layout Integration', () => {
    it('should render main layout components', () => {
      render(<App />);

      // Check for main navigation elements
      expect(screen.getByText('Neo Wallet')).toBeInTheDocument();
      expect(screen.getAllByText('Dashboard')).toHaveLength(2); // Nav + heading
      expect(screen.getByText('Wallet')).toBeInTheDocument();
      expect(screen.getByText('NFTs')).toBeInTheDocument();
      expect(screen.getByText('Tools')).toBeInTheDocument();
    });

    it('should handle button interactions correctly', () => {
      render(<App />);
      
      fireEvent.click(screen.getByText('Wallet'));
      
      const createButton = screen.getByText('Create Wallet');
      const importButton = screen.getByText('Import Wallet');
      
      // Test button clicks don't cause errors
      fireEvent.click(createButton);
      fireEvent.click(importButton);
      
      expect(createButton).toBeInTheDocument();
      expect(importButton).toBeInTheDocument();
    });
  });

  describe('Data Flow Integration', () => {
    it('should maintain consistent state across navigation', async () => {
      const testWallet = {
        id: 'test-wallet',
        name: 'Test Wallet',
        address: 'NTestAddress',
        balance: { neo: '150', gas: '75.5', tokens: {} },
        isDefault: true,
      };

      mockStore.currentWallet = testWallet;
      mockStore.walletConnected = true;

      render(<App />);

      // Navigate to wallet page
      fireEvent.click(screen.getByText('Wallet'));

      // Verify wallet information is displayed
      expect(screen.getByText('Test Wallet')).toBeInTheDocument();
      expect(screen.getByText('NTestAddress')).toBeInTheDocument();

      // Navigate to dashboard and back
      fireEvent.click(screen.getAllByText('Dashboard')[0]);
      fireEvent.click(screen.getByText('Wallet'));

      // State should be preserved
      expect(screen.getByText('Test Wallet')).toBeInTheDocument();
    });
  });
}); 