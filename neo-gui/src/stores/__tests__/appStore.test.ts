import { act, renderHook } from '@testing-library/react';
import { useAppStore } from '../appStore';

// Mock zustand persist
jest.mock('zustand/middleware', () => ({
  ...jest.requireActual('zustand/middleware'),
  persist: (fn: any) => fn,
  devtools: (fn: any) => fn,
}));

describe('AppStore', () => {
  beforeEach(() => {
    // Reset store before each test
    useAppStore.setState({
      wallets: [],
      currentWallet: null,
      walletConnected: false,
      networkType: 'testnet',
      notifications: [],
      loading: false,
      sidebarCollapsed: false,
      theme: 'light',
      currency: 'USD',
      language: 'en',
    });
  });

  describe('Initial State', () => {
    it('should have correct initial state', () => {
      const { result } = renderHook(() => useAppStore());

      expect(result.current.wallets).toEqual([]);
      expect(result.current.currentWallet).toBeNull();
      expect(result.current.walletConnected).toBe(false);
      expect(result.current.networkType).toBe('testnet');
      expect(result.current.notifications).toEqual([]);
      expect(result.current.loading).toBe(false);
      expect(result.current.sidebarCollapsed).toBe(false);
      expect(result.current.theme).toBe('light');
      expect(result.current.currency).toBe('USD');
      expect(result.current.language).toBe('en');
    });
  });

  describe('Wallet Management', () => {
    const mockWallet = {
      id: 'wallet-1',
      name: 'Test Wallet',
      address: 'NQyJR9dCZeYJNgK7N5Z9vHW3pJy4QfqPqY',
      balance: {
        neo: '100',
        gas: '1000.5',
        tokens: {},
      },
      isDefault: true,
    };

    it('should add a wallet', () => {
      const { result } = renderHook(() => useAppStore());

      act(() => {
        result.current.addWallet(mockWallet);
      });

      expect(result.current.wallets).toHaveLength(1);
      expect(result.current.wallets[0]).toEqual(mockWallet);
      expect(result.current.currentWallet).toEqual(mockWallet);
      expect(result.current.walletConnected).toBe(true);
    });

    it('should set current wallet', () => {
      const { result } = renderHook(() => useAppStore());

      act(() => {
        result.current.setCurrentWallet(mockWallet);
      });

      expect(result.current.currentWallet).toEqual(mockWallet);
      expect(result.current.walletConnected).toBe(true);
    });

    it('should remove a wallet', () => {
      const { result } = renderHook(() => useAppStore());

      // First add a wallet
      act(() => {
        result.current.addWallet(mockWallet);
      });

      expect(result.current.wallets).toHaveLength(1);

      // Then remove it
      act(() => {
        result.current.removeWallet(mockWallet.id);
      });

      expect(result.current.wallets).toHaveLength(0);
      expect(result.current.currentWallet).toBeNull();
      expect(result.current.walletConnected).toBe(false);
    });

    it('should update wallet balance', () => {
      const { result } = renderHook(() => useAppStore());
      const newBalance = {
        neo: '200',
        gas: '2000.5',
        tokens: { token1: '100' },
      };

      // First add a wallet
      act(() => {
        result.current.addWallet(mockWallet);
      });

      // Then update its balance
      act(() => {
        result.current.updateWalletBalance(mockWallet.id, newBalance);
      });

      expect(result.current.wallets[0].balance).toEqual(newBalance);
      expect(result.current.currentWallet?.balance).toEqual(newBalance);
    });
  });

  describe('Network Management', () => {
    const mockNetwork = {
      name: 'Custom Network',
      rpcUrl: 'https://custom.rpc.url',
      type: 'private' as const,
      chainId: 123456,
    };

    it('should set network type', () => {
      const { result } = renderHook(() => useAppStore());

      act(() => {
        result.current.setNetworkType('mainnet');
      });

      expect(result.current.networkType).toBe('mainnet');
    });

    it('should add a custom network', () => {
      const { result } = renderHook(() => useAppStore());
      const initialNetworkCount = result.current.networks.length;

      act(() => {
        result.current.addNetwork(mockNetwork);
      });

      expect(result.current.networks).toHaveLength(initialNetworkCount + 1);
      expect(result.current.networks).toContainEqual(mockNetwork);
    });

    it('should set current network', () => {
      const { result } = renderHook(() => useAppStore());

      act(() => {
        result.current.setCurrentNetwork(mockNetwork);
      });

      expect(result.current.currentNetwork).toEqual(mockNetwork);
      expect(result.current.networkType).toBe('private');
    });
  });

  describe('Notification Management', () => {
    const mockNotificationData = {
      type: 'success' as const,
      title: 'Test Notification',
      message: 'This is a test message',
    };

    it('should add a notification', () => {
      const { result } = renderHook(() => useAppStore());

      act(() => {
        result.current.addNotification(mockNotificationData);
      });

      expect(result.current.notifications).toHaveLength(1);

      const notification = result.current.notifications[0];
      expect(notification.type).toBe('success');
      expect(notification.title).toBe('Test Notification');
      expect(notification.message).toBe('This is a test message');
      expect(notification.read).toBe(false);
      expect(notification.id).toBeDefined();
      expect(notification.timestamp).toBeDefined();
    });

    it('should mark notification as read', () => {
      const { result } = renderHook(() => useAppStore());

      // Add a notification first
      act(() => {
        result.current.addNotification(mockNotificationData);
      });

      const notificationId = result.current.notifications[0].id;

      // Mark it as read
      act(() => {
        result.current.markNotificationRead(notificationId);
      });

      expect(result.current.notifications[0].read).toBe(true);
    });

    it('should remove a notification', () => {
      const { result } = renderHook(() => useAppStore());

      // Add a notification first
      act(() => {
        result.current.addNotification(mockNotificationData);
      });

      expect(result.current.notifications).toHaveLength(1);
      const notificationId = result.current.notifications[0].id;

      // Remove it
      act(() => {
        result.current.removeNotification(notificationId);
      });

      expect(result.current.notifications).toHaveLength(0);
    });

    it('should clear all notifications', () => {
      const { result } = renderHook(() => useAppStore());

      // Add multiple notifications
      act(() => {
        result.current.addNotification(mockNotificationData);
        result.current.addNotification({
          ...mockNotificationData,
          title: 'Second',
        });
      });

      expect(result.current.notifications).toHaveLength(2);

      // Clear all
      act(() => {
        result.current.clearNotifications();
      });

      expect(result.current.notifications).toHaveLength(0);
    });

    it('should limit notifications to 50', () => {
      const { result } = renderHook(() => useAppStore());

      // Add 51 notifications
      act(() => {
        for (let i = 0; i < 51; i++) {
          result.current.addNotification({
            ...mockNotificationData,
            title: `Notification ${i}`,
          });
        }
      });

      expect(result.current.notifications).toHaveLength(50);
    });
  });

  describe('UI State Management', () => {
    it('should set loading state', () => {
      const { result } = renderHook(() => useAppStore());

      act(() => {
        result.current.setLoading(true);
      });

      expect(result.current.loading).toBe(true);

      act(() => {
        result.current.setLoading(false);
      });

      expect(result.current.loading).toBe(false);
    });

    it('should toggle sidebar collapsed state', () => {
      const { result } = renderHook(() => useAppStore());

      act(() => {
        result.current.setSidebarCollapsed(true);
      });

      expect(result.current.sidebarCollapsed).toBe(true);
    });

    it('should set theme', () => {
      const { result } = renderHook(() => useAppStore());

      act(() => {
        result.current.setTheme('dark');
      });

      expect(result.current.theme).toBe('dark');
    });

    it('should set currency', () => {
      const { result } = renderHook(() => useAppStore());

      act(() => {
        result.current.setCurrency('EUR');
      });

      expect(result.current.currency).toBe('EUR');
    });

    it('should set language', () => {
      const { result } = renderHook(() => useAppStore());

      act(() => {
        result.current.setLanguage('zh');
      });

      expect(result.current.language).toBe('zh');
    });
  });
});
