import React, { useState, useEffect, useCallback } from 'react';
import { motion } from 'framer-motion';
import {
  PlusIcon,
  WalletIcon,
  ArrowUpIcon,
  ArrowDownIcon,
  DocumentDuplicateIcon,
  EyeIcon,
  EyeSlashIcon,
  CogIcon,
} from '@heroicons/react/24/outline';
import { useAppStore } from '../stores/appStore';
import { invoke } from '@tauri-apps/api/core';

interface Transaction {
  id: string;
  type: 'send' | 'receive';
  asset: string;
  amount: string;
  address: string;
  timestamp: number;
  status: 'confirmed' | 'pending' | 'failed';
  txHash: string;
}

export default function Wallet() {
  const { currentWallet, wallets, addWallet, addNotification } = useAppStore();
  const [showCreateModal, setShowCreateModal] = useState(false);
  // const [showImportModal, setShowImportModal] = useState(false);
  // const [showSendModal, setShowSendModal] = useState(false);
  const [balanceVisible, setBalanceVisible] = useState(true);
  const [transactions, setTransactions] = useState<Transaction[]>([]);
  const [loading, setLoading] = useState(false);

  const loadTransactions = useCallback(async () => {
    if (!currentWallet) return;

    setLoading(true);
    try {
      const result = await invoke('get_transaction_history', {
        walletId: currentWallet.id,
      });
      setTransactions(result as Transaction[]);
    } catch (error) {
      console.error('Failed to load transactions:', error);
      addNotification({
        type: 'error',
        title: 'Error',
        message: 'Failed to load transaction history',
      });
    } finally {
      setLoading(false);
    }
  }, [currentWallet, addNotification]);

  useEffect(() => {
    if (currentWallet) {
      loadTransactions();
    }
  }, [currentWallet, loadTransactions]);

  const handleCreateWallet = async (name: string, password: string) => {
    setLoading(true);
    try {
      const result = (await invoke('create_wallet', {
        request: { name, password },
      })) as any;

      console.log('Create wallet result:', result);

      if (result.success && result.data) {
        const walletData = result.data;

        // Create a default address for the wallet
        const defaultAddress =
          walletData.accounts && walletData.accounts.length > 0
            ? walletData.accounts[0].address
            : 'NX8GreRFGFK5wpGMWetpX93HmtrezGogzk'; // Mock address

        addWallet({
          id: walletData.id,
          name: walletData.name,
          address: defaultAddress,
          balance: { neo: '0', gas: '0', tokens: {} },
          isDefault: wallets.length === 0,
        });

        addNotification({
          type: 'success',
          title: 'Wallet Created',
          message: `Wallet "${name}" created successfully`,
        });

        setShowCreateModal(false);
      } else {
        throw new Error(result.error || 'Failed to create wallet');
      }
    } catch (error) {
      console.error('Failed to create wallet:', error);
      addNotification({
        type: 'error',
        title: 'Error',
        message:
          error instanceof Error ? error.message : 'Failed to create wallet',
      });
    } finally {
      setLoading(false);
    }
  };

  const formatCurrency = (amount: string) => {
    const num = parseFloat(amount);
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
    }).format(num * 25); // Mock price conversion
  };

  const formatTime = (timestamp: number) => {
    return new Date(timestamp).toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
    addNotification({
      type: 'success',
      title: 'Copied',
      message: 'Address copied to clipboard',
    });
  };

  if (!currentWallet) {
    return (
      <div className='flex items-center justify-center h-96'>
        <div className='text-center'>
          <WalletIcon className='mx-auto h-12 w-12 text-gray-400' />
          <h3 className='mt-2 text-sm font-medium text-gray-900'>
            No wallet selected
          </h3>
          <p className='mt-1 text-sm text-gray-500'>
            Create a new wallet or import an existing one to get started.
          </p>
          <div className='mt-6 flex justify-center space-x-3'>
            <button
              onClick={() => setShowCreateModal(true)}
              className='inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-green-600 hover:bg-green-700'
            >
              <PlusIcon className='-ml-1 mr-2 h-5 w-5' />
              Create Wallet
            </button>
            <button
              onClick={() => {
                /* setShowImportModal(true) */
              }}
              className='inline-flex items-center px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50'
            >
              Import Wallet
            </button>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className='space-y-6'>
      {/* Header */}
      <div className='md:flex md:items-center md:justify-between'>
        <div className='flex-1 min-w-0'>
          <h2 className='text-2xl font-bold leading-7 text-gray-900 sm:text-3xl sm:truncate'>
            {currentWallet.name}
          </h2>
          <div className='mt-1 flex items-center space-x-2'>
            <p className='text-sm text-gray-500 font-mono'>
              {currentWallet.address}
            </p>
            <button
              onClick={() => copyToClipboard(currentWallet.address)}
              className='text-gray-400 hover:text-gray-600'
            >
              <DocumentDuplicateIcon className='h-4 w-4' />
            </button>
          </div>
        </div>
        <div className='mt-4 flex space-x-3 md:mt-0 md:ml-4'>
          <button
            onClick={() => {
              /* setShowSendModal(true) */
            }}
            className='inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-green-600 hover:bg-green-700'
          >
            <ArrowUpIcon className='-ml-1 mr-2 h-5 w-5' />
            Send
          </button>
          <button className='inline-flex items-center px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50'>
            <ArrowDownIcon className='-ml-1 mr-2 h-5 w-5' />
            Receive
          </button>
          <button className='inline-flex items-center px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50'>
            <CogIcon className='-ml-1 mr-2 h-5 w-5' />
            Settings
          </button>
        </div>
      </div>

      {/* Balance Cards */}
      <div className='grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3'>
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.1 }}
          className='bg-white overflow-hidden shadow rounded-lg'
        >
          <div className='p-5'>
            <div className='flex items-center'>
              <div className='flex-shrink-0'>
                <div className='h-8 w-8 rounded-full bg-green-500 flex items-center justify-center'>
                  <span className='text-sm font-bold text-white'>N</span>
                </div>
              </div>
              <div className='ml-5 w-0 flex-1'>
                <dl>
                  <dt className='text-sm font-medium text-gray-500 truncate'>
                    NEO Balance
                  </dt>
                  <dd className='flex items-baseline'>
                    <div className='text-2xl font-semibold text-gray-900'>
                      {balanceVisible ? currentWallet.balance.neo : '••••'}
                    </div>
                    <button
                      onClick={() => setBalanceVisible(!balanceVisible)}
                      className='ml-2 text-gray-400 hover:text-gray-600'
                    >
                      {balanceVisible ? (
                        <EyeSlashIcon className='h-4 w-4' />
                      ) : (
                        <EyeIcon className='h-4 w-4' />
                      )}
                    </button>
                  </dd>
                </dl>
              </div>
            </div>
          </div>
          <div className='bg-gray-50 px-5 py-3'>
            <div className='text-sm text-gray-600'>
              {balanceVisible
                ? formatCurrency(currentWallet.balance.neo)
                : '••••••'}
            </div>
          </div>
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.2 }}
          className='bg-white overflow-hidden shadow rounded-lg'
        >
          <div className='p-5'>
            <div className='flex items-center'>
              <div className='flex-shrink-0'>
                <div className='h-8 w-8 rounded-full bg-blue-500 flex items-center justify-center'>
                  <span className='text-sm font-bold text-white'>G</span>
                </div>
              </div>
              <div className='ml-5 w-0 flex-1'>
                <dl>
                  <dt className='text-sm font-medium text-gray-500 truncate'>
                    GAS Balance
                  </dt>
                  <dd className='flex items-baseline'>
                    <div className='text-2xl font-semibold text-gray-900'>
                      {balanceVisible ? currentWallet.balance.gas : '••••'}
                    </div>
                  </dd>
                </dl>
              </div>
            </div>
          </div>
          <div className='bg-gray-50 px-5 py-3'>
            <div className='text-sm text-gray-600'>
              {balanceVisible
                ? formatCurrency(currentWallet.balance.gas)
                : '••••••'}
            </div>
          </div>
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.3 }}
          className='bg-white overflow-hidden shadow rounded-lg'
        >
          <div className='p-5'>
            <div className='flex items-center'>
              <div className='flex-shrink-0'>
                <WalletIcon className='h-8 w-8 text-gray-400' />
              </div>
              <div className='ml-5 w-0 flex-1'>
                <dl>
                  <dt className='text-sm font-medium text-gray-500 truncate'>
                    Total Value
                  </dt>
                  <dd className='text-2xl font-semibold text-gray-900'>
                    {balanceVisible
                      ? formatCurrency(
                          (
                            parseFloat(currentWallet.balance.neo) +
                            parseFloat(currentWallet.balance.gas)
                          ).toString()
                        )
                      : '••••••'}
                  </dd>
                </dl>
              </div>
            </div>
          </div>
          <div className='bg-gray-50 px-5 py-3'>
            <div className='text-sm text-green-600'>+2.5% from last week</div>
          </div>
        </motion.div>
      </div>

      {/* Transaction History */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.4 }}
        className='bg-white shadow rounded-lg'
      >
        <div className='px-6 py-4 border-b border-gray-200'>
          <h3 className='text-lg font-medium text-gray-900'>
            Transaction History
          </h3>
        </div>

        {loading ? (
          <div className='px-6 py-8 text-center'>
            <div className='animate-spin rounded-full h-8 w-8 border-b-2 border-green-600 mx-auto'></div>
            <p className='mt-2 text-sm text-gray-500'>
              Loading transactions...
            </p>
          </div>
        ) : transactions.length === 0 ? (
          <div className='px-6 py-8 text-center'>
            <p className='text-sm text-gray-500'>No transactions yet</p>
          </div>
        ) : (
          <div className='divide-y divide-gray-200'>
            {transactions.map(tx => (
              <div key={tx.id} className='px-6 py-4 hover:bg-gray-50'>
                <div className='flex items-center justify-between'>
                  <div className='flex items-center'>
                    <div
                      className={`h-8 w-8 rounded-full flex items-center justify-center ${
                        tx.type === 'send' ? 'bg-red-100' : 'bg-green-100'
                      }`}
                    >
                      {tx.type === 'send' ? (
                        <ArrowUpIcon className='h-4 w-4 text-red-600' />
                      ) : (
                        <ArrowDownIcon className='h-4 w-4 text-green-600' />
                      )}
                    </div>
                    <div className='ml-4'>
                      <div className='text-sm font-medium text-gray-900'>
                        {tx.type === 'send' ? 'Sent' : 'Received'} {tx.amount}{' '}
                        {tx.asset}
                      </div>
                      <div className='text-sm text-gray-500'>
                        {tx.type === 'send' ? 'To' : 'From'}:{' '}
                        {tx.address.slice(0, 10)}...{tx.address.slice(-6)}
                      </div>
                    </div>
                  </div>
                  <div className='text-right'>
                    <div className='text-sm text-gray-900'>
                      {formatTime(tx.timestamp)}
                    </div>
                    <div
                      className={`text-xs ${
                        tx.status === 'confirmed'
                          ? 'text-green-600'
                          : tx.status === 'pending'
                            ? 'text-yellow-600'
                            : 'text-red-600'
                      }`}
                    >
                      {tx.status}
                    </div>
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </motion.div>

      {/* Modals would be implemented here */}
      {showCreateModal && (
        <CreateWalletModal
          onClose={() => setShowCreateModal(false)}
          onSubmit={handleCreateWallet}
          loading={loading}
        />
      )}
    </div>
  );
}

// Modal components would be implemented separately
function CreateWalletModal({
  onClose,
  onSubmit,
  loading,
}: {
  onClose: () => void;
  onSubmit: (name: string, password: string) => void;
  loading: boolean;
}) {
  const [name, setName] = useState('');
  const [password, setPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (password !== confirmPassword) {
      globalThis.alert('Passwords do not match');
      return;
    }
    onSubmit(name, password);
  };

  return (
    <div className='fixed inset-0 z-50 overflow-y-auto'>
      <div className='flex items-center justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0'>
        <div className='fixed inset-0 transition-opacity' onClick={onClose}>
          <div className='absolute inset-0 bg-gray-500 opacity-75'></div>
        </div>

        <div className='inline-block align-bottom bg-white rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-lg sm:w-full'>
          <form onSubmit={handleSubmit}>
            <div className='bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4'>
              <h3 className='text-lg font-medium text-gray-900 mb-4'>
                Create New Wallet
              </h3>

              <div className='space-y-4'>
                <div>
                  <label className='block text-sm font-medium text-gray-700'>
                    Wallet Name
                  </label>
                  <input
                    type='text'
                    value={name}
                    onChange={e => setName(e.target.value)}
                    className='mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-green-500 focus:border-green-500'
                    required
                  />
                </div>

                <div>
                  <label className='block text-sm font-medium text-gray-700'>
                    Password
                  </label>
                  <input
                    type='password'
                    value={password}
                    onChange={e => setPassword(e.target.value)}
                    className='mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-green-500 focus:border-green-500'
                    required
                  />
                </div>

                <div>
                  <label className='block text-sm font-medium text-gray-700'>
                    Confirm Password
                  </label>
                  <input
                    type='password'
                    value={confirmPassword}
                    onChange={e => setConfirmPassword(e.target.value)}
                    className='mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-green-500 focus:border-green-500'
                    required
                  />
                </div>
              </div>
            </div>

            <div className='bg-gray-50 px-4 py-3 sm:px-6 sm:flex sm:flex-row-reverse'>
              <button
                type='submit'
                disabled={loading}
                className='w-full inline-flex justify-center rounded-md border border-transparent shadow-sm px-4 py-2 bg-green-600 text-base font-medium text-white hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500 sm:ml-3 sm:w-auto sm:text-sm disabled:opacity-50'
              >
                {loading ? 'Creating...' : 'Create Wallet'}
              </button>
              <button
                type='button'
                onClick={onClose}
                className='mt-3 w-full inline-flex justify-center rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-base font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500 sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm'
              >
                Cancel
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  );
}
