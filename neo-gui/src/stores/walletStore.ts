import { create } from 'zustand';
import { devtools, persist } from 'zustand/middleware';
import { invoke } from '@tauri-apps/api/core';

export interface Account {
  address: string;
  label: string;
  isDefault: boolean;
  publicKey: string;
  scriptHash: string;
}

export interface WalletInfo {
  id: string;
  name: string;
  path: string;
  accounts: Account[];
  isOpen: boolean;
  isDefault: boolean;
  version: string;
  scrypt: {
    n: number;
    r: number;
    p: number;
  };
}

export interface Balance {
  asset: string;
  symbol: string;
  amount: string;
  decimals: number;
  lastUpdated: number;
}

export interface Transaction {
  hash: string;
  blockIndex: number;
  blockTime: number;
  size: number;
  version: number;
  nonce: number;
  sender: string;
  sysFee: string;
  netFee: string;
  validUntilBlock: number;
  signers: Array<{
    account: string;
    scopes: string;
  }>;
  attributes: any[];
  script: string;
  witnesses: any[];
  vmState: string;
  exception?: string;
}

export interface WalletState {
  // Wallet management
  wallets: WalletInfo[];
  currentWallet: WalletInfo | null;
  isLoading: boolean;
  error: string | null;

  // Account management
  currentAccount: Account | null;
  balances: Record<string, Balance[]>; // address -> balances

  // Transaction management
  transactions: Record<string, Transaction[]>; // address -> transactions
  pendingTransactions: Transaction[];

  // Actions
  loadWallets: () => Promise<void>;
  createWallet: (name: string, password: string) => Promise<WalletInfo>;
  openWallet: (path: string, password: string) => Promise<WalletInfo>;
  closeWallet: (walletId: string) => Promise<void>;
  deleteWallet: (walletId: string) => Promise<void>;

  setCurrentWallet: (wallet: WalletInfo | null) => void;
  setCurrentAccount: (account: Account | null) => void;

  // Account actions
  createAccount: (walletId: string, label?: string) => Promise<Account>;
  importAccount: (
    walletId: string,
    privateKey: string,
    label?: string
  ) => Promise<Account>;
  exportAccount: (address: string, password: string) => Promise<string>;

  // Balance actions
  refreshBalances: (address: string) => Promise<void>;
  getBalance: (address: string, asset: string) => Balance | null;

  // Transaction actions
  sendTransaction: (params: {
    from: string;
    to: string;
    asset: string;
    amount: string;
    password: string;
  }) => Promise<Transaction>;

  refreshTransactions: (address: string) => Promise<void>;

  // Utility actions
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  clearError: () => void;
}

export const useWalletStore = create<WalletState>()(
  devtools(
    persist(
      (set, get) => ({
        // Initial state
        wallets: [],
        currentWallet: null,
        isLoading: false,
        error: null,
        currentAccount: null,
        balances: {},
        transactions: {},
        pendingTransactions: [],

        // Wallet management actions
        loadWallets: async () => {
          set({ isLoading: true, error: null });
          try {
            const wallets = await invoke<WalletInfo[]>('list_wallets');
            set({ wallets, isLoading: false });
          } catch (error) {
            set({
              error:
                error instanceof Error
                  ? error.message
                  : 'Failed to load wallets',
              isLoading: false,
            });
          }
        },

        createWallet: async (name: string, password: string) => {
          set({ isLoading: true, error: null });
          try {
            const wallet = await invoke<WalletInfo>('create_wallet', {
              request: { name, password },
            });

            const { wallets } = get();
            set({
              wallets: [...wallets, wallet],
              currentWallet: wallet,
              isLoading: false,
            });

            return wallet;
          } catch (error) {
            const errorMessage =
              error instanceof Error
                ? error.message
                : 'Failed to create wallet';
            set({ error: errorMessage, isLoading: false });
            throw new Error(errorMessage);
          }
        },

        openWallet: async (path: string, password: string) => {
          set({ isLoading: true, error: null });
          try {
            const wallet = await invoke<WalletInfo>('open_wallet', {
              request: { path, password },
            });

            set({
              currentWallet: wallet,
              currentAccount: wallet.accounts[0] || null,
              isLoading: false,
            });

            return wallet;
          } catch (error) {
            const errorMessage =
              error instanceof Error ? error.message : 'Failed to open wallet';
            set({ error: errorMessage, isLoading: false });
            throw new Error(errorMessage);
          }
        },

        closeWallet: async (walletId: string) => {
          try {
            await invoke('close_wallet', { walletId });
            const { currentWallet } = get();

            if (currentWallet?.id === walletId) {
              set({
                currentWallet: null,
                currentAccount: null,
              });
            }
          } catch (error) {
            set({
              error:
                error instanceof Error
                  ? error.message
                  : 'Failed to close wallet',
            });
          }
        },

        deleteWallet: async (walletId: string) => {
          set({ isLoading: true, error: null });
          try {
            await invoke('delete_wallet', { walletId });
            const { wallets, currentWallet } = get();

            const newWallets = wallets.filter(w => w.id !== walletId);
            const newCurrentWallet =
              currentWallet?.id === walletId ? null : currentWallet;

            set({
              wallets: newWallets,
              currentWallet: newCurrentWallet,
              currentAccount: newCurrentWallet ? get().currentAccount : null,
              isLoading: false,
            });
          } catch (error) {
            set({
              error:
                error instanceof Error
                  ? error.message
                  : 'Failed to delete wallet',
              isLoading: false,
            });
          }
        },

        setCurrentWallet: wallet => {
          set({
            currentWallet: wallet,
            currentAccount: wallet?.accounts[0] || null,
          });
        },

        setCurrentAccount: account => {
          set({ currentAccount: account });
        },

        // Account management
        createAccount: async (walletId: string, label?: string) => {
          set({ isLoading: true, error: null });
          try {
            const account = await invoke<Account>('create_address', {
              request: { walletId, label, count: 1 },
            });

            // Update wallet with new account
            const { wallets } = get();
            const updatedWallets = wallets.map(w =>
              w.id === walletId
                ? { ...w, accounts: [...w.accounts, account] }
                : w
            );

            set({ wallets: updatedWallets, isLoading: false });
            return account;
          } catch (error) {
            const errorMessage =
              error instanceof Error
                ? error.message
                : 'Failed to create account';
            set({ error: errorMessage, isLoading: false });
            throw new Error(errorMessage);
          }
        },

        importAccount: async (
          walletId: string,
          privateKey: string,
          label?: string
        ) => {
          set({ isLoading: true, error: null });
          try {
            const account = await invoke<Account>('import_address', {
              request: { walletId, privateKey, label },
            });

            // Update wallet with imported account
            const { wallets } = get();
            const updatedWallets = wallets.map(w =>
              w.id === walletId
                ? { ...w, accounts: [...w.accounts, account] }
                : w
            );

            set({ wallets: updatedWallets, isLoading: false });
            return account;
          } catch (error) {
            const errorMessage =
              error instanceof Error
                ? error.message
                : 'Failed to import account';
            set({ error: errorMessage, isLoading: false });
            throw new Error(errorMessage);
          }
        },

        exportAccount: async (address: string, password: string) => {
          try {
            const privateKey = await invoke<string>('export_address', {
              request: { address, password },
            });
            return privateKey;
          } catch (error) {
            const errorMessage =
              error instanceof Error
                ? error.message
                : 'Failed to export account';
            set({ error: errorMessage });
            throw new Error(errorMessage);
          }
        },

        // Balance management
        refreshBalances: async (address: string) => {
          try {
            const balances = await invoke<Balance[]>('get_balance', {
              address,
            });
            const { balances: currentBalances } = get();

            set({
              balances: {
                ...currentBalances,
                [address]: balances,
              },
            });
          } catch (error) {
            set({
              error:
                error instanceof Error
                  ? error.message
                  : 'Failed to refresh balances',
            });
          }
        },

        getBalance: (address: string, asset: string) => {
          const { balances } = get();
          const addressBalances = balances[address] || [];
          return addressBalances.find(b => b.asset === asset) || null;
        },

        // Transaction management
        sendTransaction: async params => {
          set({ isLoading: true, error: null });
          try {
            const transaction = await invoke<Transaction>('send_transaction', {
              request: params,
            });

            const { pendingTransactions } = get();
            set({
              pendingTransactions: [...pendingTransactions, transaction],
              isLoading: false,
            });

            return transaction;
          } catch (error) {
            const errorMessage =
              error instanceof Error
                ? error.message
                : 'Failed to send transaction';
            set({ error: errorMessage, isLoading: false });
            throw new Error(errorMessage);
          }
        },

        refreshTransactions: async (address: string) => {
          try {
            const transactions = await invoke<Transaction[]>(
              'get_transaction_history',
              {
                address,
              }
            );

            const { transactions: currentTransactions } = get();
            set({
              transactions: {
                ...currentTransactions,
                [address]: transactions,
              },
            });
          } catch (error) {
            set({
              error:
                error instanceof Error
                  ? error.message
                  : 'Failed to refresh transactions',
            });
          }
        },

        // Utility actions
        setLoading: loading => set({ isLoading: loading }),
        setError: error => set({ error }),
        clearError: () => set({ error: null }),
      }),
      {
        name: 'neo-wallet-store',
        partialize: state => ({
          // Only persist non-sensitive data
          wallets: state.wallets.map(w => ({
            ...w,
            isOpen: false, // Don't persist open state
          })),
        }),
      }
    ),
    {
      name: 'wallet-store',
    }
  )
);
