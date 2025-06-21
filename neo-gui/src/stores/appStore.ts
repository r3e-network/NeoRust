import { create } from 'zustand';
import { devtools, persist } from 'zustand/middleware';

export interface Notification {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info';
  title: string;
  message: string;
  timestamp: number;
  read: boolean;
}

export interface WalletInfo {
  id: string;
  name: string;
  address: string;
  balance: {
    neo: string;
    gas: string;
    tokens: Record<string, string>;
  };
  isDefault: boolean;
}

export interface NetworkConfig {
  name: string;
  rpcUrl: string;
  type: 'mainnet' | 'testnet' | 'private';
  chainId: number;
}

export interface AppState {
  // Wallet state
  wallets: WalletInfo[];
  currentWallet: WalletInfo | null;
  walletConnected: boolean;

  // Network state
  networkType: 'mainnet' | 'testnet' | 'private';
  currentNetwork: NetworkConfig | null;
  networks: NetworkConfig[];

  // UI state
  notifications: Notification[];
  loading: boolean;
  sidebarCollapsed: boolean;

  // Settings
  theme: 'light' | 'dark';
  currency: 'USD' | 'EUR' | 'CNY';
  language: 'en' | 'zh' | 'ja';

  // Actions
  setWallets: (wallets: WalletInfo[]) => void;
  setCurrentWallet: (wallet: WalletInfo | null) => void;
  setWalletConnected: (connected: boolean) => void;
  addWallet: (wallet: WalletInfo) => void;
  removeWallet: (walletId: string) => void;
  updateWalletBalance: (
    walletId: string,
    balance: WalletInfo['balance']
  ) => void;

  setNetworkType: (type: 'mainnet' | 'testnet' | 'private') => void;
  setCurrentNetwork: (network: NetworkConfig | null) => void;
  addNetwork: (network: NetworkConfig) => void;
  removeNetwork: (networkName: string) => void;

  addNotification: (
    notification: Omit<Notification, 'id' | 'timestamp' | 'read'>
  ) => void;
  markNotificationRead: (id: string) => void;
  removeNotification: (id: string) => void;
  clearNotifications: () => void;

  setLoading: (loading: boolean) => void;
  setSidebarCollapsed: (collapsed: boolean) => void;
  setTheme: (theme: 'light' | 'dark') => void;
  setCurrency: (currency: 'USD' | 'EUR' | 'CNY') => void;
  setLanguage: (language: 'en' | 'zh' | 'ja') => void;
}

const defaultNetworks: NetworkConfig[] = [
  {
    name: 'Neo N3 Mainnet',
    rpcUrl: 'https://mainnet1.neo.coz.io:443',
    type: 'mainnet',
    chainId: 860833102,
  },
  {
    name: 'Neo N3 Testnet',
    rpcUrl: 'https://testnet1.neo.coz.io:443',
    type: 'testnet',
    chainId: 894710606,
  },
];

export const useAppStore = create<AppState>()(
  devtools(
    persist(
      set => ({
        // Initial state
        wallets: [],
        currentWallet: null,
        walletConnected: false,

        networkType: 'testnet',
        currentNetwork: defaultNetworks[1], // Default to testnet
        networks: defaultNetworks,

        notifications: [],
        loading: false,
        sidebarCollapsed: false,

        theme: 'light',
        currency: 'USD',
        language: 'en',

        // Wallet actions
        setWallets: wallets => set({ wallets }),

        setCurrentWallet: wallet =>
          set({
            currentWallet: wallet,
            walletConnected: wallet !== null,
          }),

        setWalletConnected: connected => set({ walletConnected: connected }),

        addWallet: wallet =>
          set(state => ({
            wallets: [...state.wallets, wallet],
            currentWallet: state.currentWallet || wallet,
            walletConnected: true,
          })),

        removeWallet: walletId =>
          set(state => {
            const newWallets = state.wallets.filter(w => w.id !== walletId);
            const newCurrentWallet =
              state.currentWallet?.id === walletId
                ? newWallets[0] || null
                : state.currentWallet;

            return {
              wallets: newWallets,
              currentWallet: newCurrentWallet,
              walletConnected: newCurrentWallet !== null,
            };
          }),

        updateWalletBalance: (walletId, balance) =>
          set(state => ({
            wallets: state.wallets.map(wallet =>
              wallet.id === walletId ? { ...wallet, balance } : wallet
            ),
            currentWallet:
              state.currentWallet?.id === walletId
                ? { ...state.currentWallet, balance }
                : state.currentWallet,
          })),

        // Network actions
        setNetworkType: type => set({ networkType: type }),

        setCurrentNetwork: network =>
          set({
            currentNetwork: network,
            networkType: network?.type || 'testnet',
          }),

        addNetwork: network =>
          set(state => ({
            networks: [...state.networks, network],
          })),

        removeNetwork: networkName =>
          set(state => ({
            networks: state.networks.filter(n => n.name !== networkName),
          })),

        // Notification actions
        addNotification: notification =>
          set(state => ({
            notifications: [
              {
                ...notification,
                id: Date.now().toString(),
                timestamp: Date.now(),
                read: false,
              },
              ...state.notifications,
            ].slice(0, 50), // Keep only last 50 notifications
          })),

        markNotificationRead: id =>
          set(state => ({
            notifications: state.notifications.map(n =>
              n.id === id ? { ...n, read: true } : n
            ),
          })),

        removeNotification: id =>
          set(state => ({
            notifications: state.notifications.filter(n => n.id !== id),
          })),

        clearNotifications: () => set({ notifications: [] }),

        // UI actions
        setLoading: loading => set({ loading }),
        setSidebarCollapsed: collapsed => set({ sidebarCollapsed: collapsed }),
        setTheme: theme => set({ theme }),
        setCurrency: currency => set({ currency }),
        setLanguage: language => set({ language }),
      }),
      {
        name: 'neo-wallet-storage',
        partialize: state => ({
          // Persist only specific parts of the state
          networkType: state.networkType,
          currentNetwork: state.currentNetwork,
          networks: state.networks,
          theme: state.theme,
          currency: state.currency,
          language: state.language,
          sidebarCollapsed: state.sidebarCollapsed,
        }),
      }
    ),
    {
      name: 'neo-wallet-store',
    }
  )
);
