import { act, renderHook } from '@testing-library/react';
import { useWalletStore } from '../walletStore';

// Mock Tauri invoke function
const mockInvoke = jest.fn();
jest.mock('@tauri-apps/api/core', () => ({
  invoke: (...args: any[]) => mockInvoke(...args),
}));

// Mock zustand persist
jest.mock('zustand/middleware', () => ({
  ...jest.requireActual('zustand/middleware'),
  persist: (fn: any) => fn,
  devtools: (fn: any) => fn,
}));

describe('WalletStore', () => {
  beforeEach(() => {
    // Reset store before each test
    useWalletStore.setState({
      wallets: [],
      currentWallet: null,
      isLoading: false,
      error: null,
      currentAccount: null,
      balances: {},
      transactions: {},
      pendingTransactions: [],
    });

    mockInvoke.mockClear();
  });

  describe('Initial State', () => {
    it('should have correct initial state', () => {
      const { result } = renderHook(() => useWalletStore());

      expect(result.current.wallets).toEqual([]);
      expect(result.current.currentWallet).toBeNull();
      expect(result.current.isLoading).toBe(false);
      expect(result.current.error).toBeNull();
      expect(result.current.currentAccount).toBeNull();
      expect(result.current.balances).toEqual({});
      expect(result.current.transactions).toEqual({});
      expect(result.current.pendingTransactions).toEqual([]);
    });
  });

  describe('Wallet Management', () => {
    it('should load wallets successfully', async () => {
      const mockWallets = [
        {
          id: 'wallet-1',
          name: 'Test Wallet',
          path: '/path/to/wallet',
          accounts: [],
          isOpen: false,
          isDefault: true,
          version: '3.0',
          scrypt: { n: 16384, r: 8, p: 8 },
        },
      ];

      mockInvoke.mockResolvedValueOnce(mockWallets);

      const { result } = renderHook(() => useWalletStore());

      await act(async () => {
        await result.current.loadWallets();
      });

      expect(mockInvoke).toHaveBeenCalledWith('list_wallets');
      expect(result.current.wallets).toEqual(mockWallets);
      expect(result.current.isLoading).toBe(false);
      expect(result.current.error).toBeNull();
    });

    it('should handle load wallets error', async () => {
      mockInvoke.mockRejectedValueOnce(new Error('Failed to load'));

      const { result } = renderHook(() => useWalletStore());

      await act(async () => {
        await result.current.loadWallets();
      });

      expect(result.current.error).toBe('Failed to load');
      expect(result.current.isLoading).toBe(false);
      expect(result.current.wallets).toEqual([]);
    });

    it('should create wallet successfully', async () => {
      const mockWallet = {
        id: 'new-wallet',
        name: 'New Wallet',
        path: '/path/to/new-wallet',
        accounts: [
          {
            address: 'NTestAddress123',
            label: 'Account 1',
            isDefault: true,
            publicKey: 'public-key',
            scriptHash: 'script-hash',
          },
        ],
        isOpen: true,
        isDefault: true,
        version: '3.0',
        scrypt: { n: 16384, r: 8, p: 8 },
      };

      mockInvoke.mockResolvedValueOnce(mockWallet);

      const { result } = renderHook(() => useWalletStore());

      let createdWallet;
      await act(async () => {
        createdWallet = await result.current.createWallet(
          'New Wallet',
          'password123'
        );
      });

      expect(mockInvoke).toHaveBeenCalledWith('create_wallet', {
        request: { name: 'New Wallet', password: 'password123' },
      });
      expect(result.current.wallets).toContain(mockWallet);
      expect(result.current.currentWallet).toEqual(mockWallet);
      expect(createdWallet).toEqual(mockWallet);
    });

    it('should handle create wallet error', async () => {
      mockInvoke.mockRejectedValueOnce(new Error('Creation failed'));

      const { result } = renderHook(() => useWalletStore());

      await act(async () => {
        try {
          await result.current.createWallet('New Wallet', 'password');
        } catch (error) {
          expect(error).toBeInstanceOf(Error);
          expect((error as Error).message).toBe('Creation failed');
        }
      });

      expect(result.current.error).toBe('Creation failed');
      expect(result.current.isLoading).toBe(false);
    });

    it('should open wallet successfully', async () => {
      const mockWallet = {
        id: 'opened-wallet',
        name: 'Opened Wallet',
        path: '/path/to/wallet',
        accounts: [
          {
            address: 'NTestAddress456',
            label: 'Main Account',
            isDefault: true,
            publicKey: 'public-key-2',
            scriptHash: 'script-hash-2',
          },
        ],
        isOpen: true,
        isDefault: false,
        version: '3.0',
        scrypt: { n: 16384, r: 8, p: 8 },
      };

      mockInvoke.mockResolvedValueOnce(mockWallet);

      const { result } = renderHook(() => useWalletStore());

      await act(async () => {
        const wallet = await result.current.openWallet(
          '/path/to/wallet',
          'password'
        );
        expect(wallet).toEqual(mockWallet);
      });

      expect(result.current.currentWallet).toEqual(mockWallet);
      expect(result.current.currentAccount).toEqual(mockWallet.accounts[0]);
    });

    it('should close wallet successfully', async () => {
      // Set up initial state with open wallet
      const initialWallet = {
        id: 'wallet-to-close',
        name: 'Test Wallet',
        path: '/path',
        accounts: [],
        isOpen: true,
        isDefault: false,
        version: '3.0',
        scrypt: { n: 16384, r: 8, p: 8 },
      };

      useWalletStore.setState({
        currentWallet: initialWallet,
        currentAccount: null,
      });

      mockInvoke.mockResolvedValueOnce(null);

      const { result } = renderHook(() => useWalletStore());

      await act(async () => {
        await result.current.closeWallet('wallet-to-close');
      });

      expect(mockInvoke).toHaveBeenCalledWith('close_wallet', {
        walletId: 'wallet-to-close',
      });
      expect(result.current.currentWallet).toBeNull();
      expect(result.current.currentAccount).toBeNull();
    });

    it('should delete wallet successfully', async () => {
      const initialWallets = [
        {
          id: 'wallet-1',
          name: 'Wallet 1',
          path: '/path1',
          accounts: [],
          isOpen: false,
          isDefault: false,
          version: '3.0',
          scrypt: { n: 16384, r: 8, p: 8 },
        },
        {
          id: 'wallet-2',
          name: 'Wallet 2',
          path: '/path2',
          accounts: [],
          isOpen: false,
          isDefault: false,
          version: '3.0',
          scrypt: { n: 16384, r: 8, p: 8 },
        },
      ];

      useWalletStore.setState({
        wallets: initialWallets,
        currentWallet: initialWallets[0],
      });

      mockInvoke.mockResolvedValueOnce(null);

      const { result } = renderHook(() => useWalletStore());

      await act(async () => {
        await result.current.deleteWallet('wallet-1');
      });

      expect(result.current.wallets).toHaveLength(1);
      expect(result.current.wallets[0].id).toBe('wallet-2');
      expect(result.current.currentWallet).toBeNull();
    });
  });

  describe('Account Management', () => {
    it('should create account successfully', async () => {
      const mockAccount = {
        address: 'NNewAddress789',
        label: 'New Account',
        isDefault: false,
        publicKey: 'new-public-key',
        scriptHash: 'new-script-hash',
      };

      const initialWallet = {
        id: 'wallet-1',
        name: 'Test Wallet',
        path: '/path',
        accounts: [],
        isOpen: true,
        isDefault: false,
        version: '3.0',
        scrypt: { n: 16384, r: 8, p: 8 },
      };

      useWalletStore.setState({
        wallets: [initialWallet],
      });

      mockInvoke.mockResolvedValueOnce(mockAccount);

      const { result } = renderHook(() => useWalletStore());

      await act(async () => {
        const account = await result.current.createAccount(
          'wallet-1',
          'New Account'
        );
        expect(account).toEqual(mockAccount);
      });

      const updatedWallet = result.current.wallets.find(
        w => w.id === 'wallet-1'
      );
      expect(updatedWallet?.accounts).toContain(mockAccount);
    });

    it('should import account successfully', async () => {
      const mockAccount = {
        address: 'NImportedAddress',
        label: 'Imported Account',
        isDefault: false,
        publicKey: 'imported-public-key',
        scriptHash: 'imported-script-hash',
      };

      const initialWallet = {
        id: 'wallet-1',
        name: 'Test Wallet',
        path: '/path',
        accounts: [],
        isOpen: true,
        isDefault: false,
        version: '3.0',
        scrypt: { n: 16384, r: 8, p: 8 },
      };

      useWalletStore.setState({
        wallets: [initialWallet],
      });

      mockInvoke.mockResolvedValueOnce(mockAccount);

      const { result } = renderHook(() => useWalletStore());

      await act(async () => {
        const account = await result.current.importAccount(
          'wallet-1',
          'private-key-hex',
          'Imported Account'
        );
        expect(account).toEqual(mockAccount);
      });

      expect(mockInvoke).toHaveBeenCalledWith('import_address', {
        request: {
          walletId: 'wallet-1',
          privateKey: 'private-key-hex',
          label: 'Imported Account',
        },
      });
    });

    it('should export account successfully', async () => {
      mockInvoke.mockResolvedValueOnce('exported-private-key');

      const { result } = renderHook(() => useWalletStore());

      await act(async () => {
        const privateKey = await result.current.exportAccount(
          'NTestAddress',
          'password'
        );
        expect(privateKey).toBe('exported-private-key');
      });

      expect(mockInvoke).toHaveBeenCalledWith('export_address', {
        request: { address: 'NTestAddress', password: 'password' },
      });
    });
  });

  describe('Balance Management', () => {
    it('should refresh balances successfully', async () => {
      const mockBalances = [
        {
          asset: 'NEO',
          symbol: 'NEO',
          amount: '100',
          decimals: 0,
          lastUpdated: Date.now(),
        },
        {
          asset: 'GAS',
          symbol: 'GAS',
          amount: '1000.5',
          decimals: 8,
          lastUpdated: Date.now(),
        },
      ];

      mockInvoke.mockResolvedValueOnce(mockBalances);

      const { result } = renderHook(() => useWalletStore());

      await act(async () => {
        await result.current.refreshBalances('NTestAddress');
      });

      expect(result.current.balances['NTestAddress']).toEqual(mockBalances);
    });

    it('should get balance for specific asset', () => {
      const mockBalances = [
        {
          asset: 'NEO',
          symbol: 'NEO',
          amount: '100',
          decimals: 0,
          lastUpdated: Date.now(),
        },
      ];

      useWalletStore.setState({
        balances: {
          NTestAddress: mockBalances,
        },
      });

      const { result } = renderHook(() => useWalletStore());

      const neoBalance = result.current.getBalance('NTestAddress', 'NEO');
      expect(neoBalance).toEqual(mockBalances[0]);

      const gasBalance = result.current.getBalance('NTestAddress', 'GAS');
      expect(gasBalance).toBeNull();
    });
  });

  describe('Transaction Management', () => {
    it('should send transaction successfully', async () => {
      const mockTransaction = {
        hash: 'transaction-hash',
        blockIndex: 1000,
        blockTime: Date.now(),
        size: 250,
        version: 0,
        nonce: 123456,
        sender: 'NSenderAddress',
        sysFee: '0.01',
        netFee: '0.001',
        validUntilBlock: 1100,
        signers: [],
        attributes: [],
        script: 'script-data',
        witnesses: [],
        vmState: 'HALT',
      };

      mockInvoke.mockResolvedValueOnce(mockTransaction);

      const { result } = renderHook(() => useWalletStore());

      await act(async () => {
        const tx = await result.current.sendTransaction({
          from: 'NSenderAddress',
          to: 'NReceiverAddress',
          asset: 'NEO',
          amount: '10',
          password: 'password',
        });
        expect(tx).toEqual(mockTransaction);
      });

      expect(result.current.pendingTransactions).toContain(mockTransaction);
    });

    it('should refresh transactions successfully', async () => {
      const mockTransactions = [
        {
          hash: 'tx-hash-1',
          blockIndex: 1000,
          blockTime: Date.now(),
          size: 250,
          version: 0,
          nonce: 123456,
          sender: 'NSenderAddress',
          sysFee: '0.01',
          netFee: '0.001',
          validUntilBlock: 1100,
          signers: [],
          attributes: [],
          script: 'script-data',
          witnesses: [],
          vmState: 'HALT',
        },
      ];

      mockInvoke.mockResolvedValueOnce(mockTransactions);

      const { result } = renderHook(() => useWalletStore());

      await act(async () => {
        await result.current.refreshTransactions('NTestAddress');
      });

      expect(result.current.transactions['NTestAddress']).toEqual(
        mockTransactions
      );
    });
  });

  describe('Utility Actions', () => {
    it('should set loading state', () => {
      const { result } = renderHook(() => useWalletStore());

      act(() => {
        result.current.setLoading(true);
      });

      expect(result.current.isLoading).toBe(true);

      act(() => {
        result.current.setLoading(false);
      });

      expect(result.current.isLoading).toBe(false);
    });

    it('should set error state', () => {
      const { result } = renderHook(() => useWalletStore());

      act(() => {
        result.current.setError('Test error');
      });

      expect(result.current.error).toBe('Test error');
    });

    it('should clear error state', () => {
      useWalletStore.setState({ error: 'Some error' });

      const { result } = renderHook(() => useWalletStore());

      act(() => {
        result.current.clearError();
      });

      expect(result.current.error).toBeNull();
    });

    it('should set current wallet', () => {
      const mockWallet = {
        id: 'wallet-1',
        name: 'Test Wallet',
        path: '/path',
        accounts: [
          {
            address: 'NTestAddress',
            label: 'Account 1',
            isDefault: true,
            publicKey: 'public-key',
            scriptHash: 'script-hash',
          },
        ],
        isOpen: true,
        isDefault: false,
        version: '3.0',
        scrypt: { n: 16384, r: 8, p: 8 },
      };

      const { result } = renderHook(() => useWalletStore());

      act(() => {
        result.current.setCurrentWallet(mockWallet);
      });

      expect(result.current.currentWallet).toEqual(mockWallet);
      expect(result.current.currentAccount).toEqual(mockWallet.accounts[0]);
    });

    it('should set current account', () => {
      const mockAccount = {
        address: 'NTestAddress',
        label: 'Test Account',
        isDefault: true,
        publicKey: 'public-key',
        scriptHash: 'script-hash',
      };

      const { result } = renderHook(() => useWalletStore());

      act(() => {
        result.current.setCurrentAccount(mockAccount);
      });

      expect(result.current.currentAccount).toEqual(mockAccount);
    });
  });
});
