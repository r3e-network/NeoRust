import React, { useEffect, useState, useCallback } from 'react';
import { motion } from 'framer-motion';
import {
  WalletIcon,
  CubeIcon,
  ChartBarIcon,
  ArrowTrendingUpIcon,
  ArrowTrendingDownIcon,
  EyeIcon,
  EyeSlashIcon,
  PlusIcon,
} from '@heroicons/react/24/outline';
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  PieChart,
  Pie,
  Cell,
} from 'recharts';
import { useAppStore } from '../stores/appStore';
import { invoke } from '@tauri-apps/api/core';
import CreateWalletModal from '../components/modals/CreateWalletModal';

// Mock data for charts (keep for visualization until backend provides history)
const priceData = [
  { name: 'Jan', neo: 12, gas: 0.5 },
  { name: 'Feb', neo: 15, gas: 0.6 },
  { name: 'Mar', neo: 18, gas: 0.7 },
  { name: 'Apr', neo: 22, gas: 0.8 },
  { name: 'May', neo: 25, gas: 0.9 },
  { name: 'Jun', neo: 28, gas: 1.0 },
];

const portfolioData = [
  { name: 'NEO', value: 65, color: '#10B981' },
  { name: 'GAS', value: 25, color: '#3B82F6' },
  { name: 'Other Tokens', value: 10, color: '#8B5CF6' },
];

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

export default function Dashboard() {
  const { currentWallet, wallets, networkType, addWallet, addNotification } = useAppStore();
  const [balanceVisible, setBalanceVisible] = useState(true);
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [loading, setLoading] = useState(false);
  const [transactions, setTransactions] = useState<Transaction[]>([]);
  const [stats, setStats] = useState({
    totalValue: 0,
    neoBalance: 0,
    gasBalance: 0,
    nftCount: 0,
    change24h: 0,
  });

  const loadTransactions = useCallback(async () => {
    if (!currentWallet) return;

    try {
      const result = await invoke('get_transaction_history', {
        walletId: currentWallet.id,
      });
      setTransactions(result as Transaction[]);
    } catch (error) {
      console.error('Failed to load transactions:', error);
    }
  }, [currentWallet]);

  useEffect(() => {
    if (currentWallet) {
      // In a real app, we would fetch prices here to calculate total value
      // For now, we simulate values based on balance
      const neo = parseFloat(currentWallet.balance.neo);
      const gas = parseFloat(currentWallet.balance.gas);
      const total = neo * 15 + gas * 5; // Mock prices

      setStats({
        totalValue: total,
        neoBalance: neo,
        gasBalance: gas,
        nftCount: 12, // Mock NFT count
        change24h: 5.2,
      });
      loadTransactions();
    }
  }, [currentWallet, loadTransactions]);

  const handleCreateWallet = async (name: string, password: string) => {
    setLoading(true);
    try {
      const result = (await invoke('create_wallet', {
        request: { name, password },
      })) as any;

      if (result.success && result.data) {
        const walletData = result.data;
        const defaultAddress =
          walletData.accounts && walletData.accounts.length > 0
            ? walletData.accounts[0].address
            : 'NX8GreRFGFK5wpGMWetpX93HmtrezGogzk';

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
      throw error;
    } finally {
      setLoading(false);
    }
  };

  const formatCurrency = (amount: number) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
    }).format(amount);
  };

  const formatTime = (timestamp: number) => {
    return new Intl.RelativeTimeFormat('en', { numeric: 'auto' }).format(
      Math.floor((timestamp - Date.now()) / (1000 * 60 * 60)),
      'hour'
    );
  };

  if (!currentWallet) {
    return (
      <div className='flex items-center justify-center h-96'>
        <div className='text-center'>
          <WalletIcon className='mx-auto h-12 w-12 text-gray-400' />
          <h3 className='mt-2 text-sm font-medium text-gray-900'>
            No wallet connected
          </h3>
          <p className='mt-1 text-sm text-gray-500'>
            Connect or create a wallet to view your dashboard.
          </p>
          <div className='mt-6'>
            <button
              type='button'
              onClick={() => setShowCreateModal(true)}
              className='inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-green-600 hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500'
            >
              <PlusIcon className='-ml-1 mr-2 h-5 w-5' />
              Create Wallet
            </button>
          </div>
        </div>
        <CreateWalletModal
          isOpen={showCreateModal}
          onClose={() => setShowCreateModal(false)}
          onSubmit={handleCreateWallet}
          loading={loading}
        />
      </div>
    );
  }

  return (
    <div className='space-y-6'>
      {/* Header */}
      <div className='md:flex md:items-center md:justify-between'>
        <div className='flex-1 min-w-0'>
          <h2 className='text-2xl font-bold leading-7 text-gray-900 sm:text-3xl sm:truncate'>
            Dashboard
          </h2>
          <p className='mt-1 text-sm text-gray-500'>
            Welcome back! Here&apos;s what&apos;s happening with your Neo
            wallet.
          </p>
        </div>
        <div className='mt-4 flex md:mt-0 md:ml-4'>
          <span
            className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
              networkType === 'mainnet'
                ? 'bg-green-100 text-green-800'
                : 'bg-yellow-100 text-yellow-800'
            }`}
          >
            {networkType.charAt(0).toUpperCase() + networkType.slice(1)}
          </span>
        </div>
      </div>

      {/* Stats Cards */}
      <div className='grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-4'>
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.1 }}
          className='bg-white overflow-hidden shadow rounded-lg'
        >
          <div className='p-5'>
            <div className='flex items-center'>
              <div className='flex-shrink-0'>
                <WalletIcon className='h-6 w-6 text-gray-400' />
              </div>
              <div className='ml-5 w-0 flex-1'>
                <dl>
                  <dt className='text-sm font-medium text-gray-500 truncate'>
                    Total Value
                  </dt>
                  <dd className='flex items-baseline'>
                    <div className='text-2xl font-semibold text-gray-900'>
                      {balanceVisible
                        ? formatCurrency(stats.totalValue)
                        : '••••••'}
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
            <div className='text-sm'>
              <div
                className={`flex items-center ${
                  stats.change24h >= 0 ? 'text-green-600' : 'text-red-600'
                }`}
              >
                {stats.change24h >= 0 ? (
                  <ArrowTrendingUpIcon className='h-4 w-4 mr-1' />
                ) : (
                  <ArrowTrendingDownIcon className='h-4 w-4 mr-1' />
                )}
                {Math.abs(stats.change24h)}% from yesterday
              </div>
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
                <div className='h-6 w-6 rounded-full bg-green-500 flex items-center justify-center'>
                  <span className='text-xs font-bold text-white'>N</span>
                </div>
              </div>
              <div className='ml-5 w-0 flex-1'>
                <dl>
                  <dt className='text-sm font-medium text-gray-500 truncate'>
                    NEO Balance
                  </dt>
                  <dd className='text-2xl font-semibold text-gray-900'>
                    {balanceVisible ? stats.neoBalance.toFixed(2) : '••••'}
                  </dd>
                </dl>
              </div>
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
                <div className='h-6 w-6 rounded-full bg-blue-500 flex items-center justify-center'>
                  <span className='text-xs font-bold text-white'>G</span>
                </div>
              </div>
              <div className='ml-5 w-0 flex-1'>
                <dl>
                  <dt className='text-sm font-medium text-gray-500 truncate'>
                    GAS Balance
                  </dt>
                  <dd className='text-2xl font-semibold text-gray-900'>
                    {balanceVisible ? stats.gasBalance.toFixed(2) : '••••'}
                  </dd>
                </dl>
              </div>
            </div>
          </div>
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.4 }}
          className='bg-white overflow-hidden shadow rounded-lg'
        >
          <div className='p-5'>
            <div className='flex items-center'>
              <div className='flex-shrink-0'>
                <CubeIcon className='h-6 w-6 text-gray-400' />
              </div>
              <div className='ml-5 w-0 flex-1'>
                <dl>
                  <dt className='text-sm font-medium text-gray-500 truncate'>
                    NFTs Owned
                  </dt>
                  <dd className='text-2xl font-semibold text-gray-900'>
                    {stats.nftCount}
                  </dd>
                </dl>
              </div>
            </div>
          </div>
        </motion.div>
      </div>

      {/* Charts Section */}
      <div className='grid grid-cols-1 lg:grid-cols-2 gap-6'>
        {/* Price Chart */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.5 }}
          className='bg-white shadow rounded-lg p-6'
        >
          <div className='flex items-center justify-between mb-4'>
            <h3 className='text-lg font-medium text-gray-900'>Price History</h3>
            <ChartBarIcon className='h-5 w-5 text-gray-400' />
          </div>
          <div className='h-64'>
            <ResponsiveContainer width='100%' height='100%'>
              <LineChart data={priceData}>
                <CartesianGrid strokeDasharray='3 3' />
                <XAxis dataKey='name' />
                <YAxis />
                <Tooltip />
                <Line
                  type='monotone'
                  dataKey='neo'
                  stroke='#10B981'
                  strokeWidth={2}
                  name='NEO'
                />
                <Line
                  type='monotone'
                  dataKey='gas'
                  stroke='#3B82F6'
                  strokeWidth={2}
                  name='GAS'
                />
              </LineChart>
            </ResponsiveContainer>
          </div>
        </motion.div>

        {/* Portfolio Distribution */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.6 }}
          className='bg-white shadow rounded-lg p-6'
        >
          <div className='flex items-center justify-between mb-4'>
            <h3 className='text-lg font-medium text-gray-900'>
              Portfolio Distribution
            </h3>
          </div>
          <div className='h-64'>
            <ResponsiveContainer width='100%' height='100%'>
              <PieChart>
                <Pie
                  data={portfolioData}
                  cx='50%'
                  cy='50%'
                  innerRadius={60}
                  outerRadius={100}
                  paddingAngle={5}
                  dataKey='value'
                >
                  {portfolioData.map((entry, index) => (
                    <Cell key={`cell-${index}`} fill={entry.color} />
                  ))}
                </Pie>
                <Tooltip />
              </PieChart>
            </ResponsiveContainer>
          </div>
          <div className='mt-4 space-y-2'>
            {portfolioData.map(item => (
              <div
                key={item.name}
                className='flex items-center justify-between'
              >
                <div className='flex items-center'>
                  <div
                    className='w-3 h-3 rounded-full mr-2'
                    style={{ backgroundColor: item.color }}
                  />
                  <span className='text-sm text-gray-600'>{item.name}</span>
                </div>
                <span className='text-sm font-medium text-gray-900'>
                  {item.value}%
                </span>
              </div>
            ))}
          </div>
        </motion.div>
      </div>

      {/* Recent Activity */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.7 }}
        className='bg-white shadow rounded-lg'
      >
        <div className='px-6 py-4 border-b border-gray-200'>
          <h3 className='text-lg font-medium text-gray-900'>Recent Activity</h3>
        </div>
        <div className='divide-y divide-gray-200'>
          {transactions.map(tx => (
            <div key={tx.id} className='px-6 py-4 hover:bg-gray-50'>
              <div className='flex items-center justify-between'>
                <div className='flex items-center'>
                  <div
                    className={`h-8 w-8 rounded-full flex items-center justify-center ${
                      tx.type === 'send'
                        ? 'bg-red-100'
                        : tx.type === 'receive'
                          ? 'bg-green-100'
                          : 'bg-purple-100'
                    }`}
                  >
                    <span
                      className={`text-xs font-bold ${
                        tx.type === 'send'
                          ? 'text-red-600'
                          : tx.type === 'receive'
                            ? 'text-green-600'
                            : 'text-purple-600'
                      }`}
                    >
                      {tx.type === 'send'
                        ? '↗'
                        : tx.type === 'receive'
                          ? '↙'
                          : '🎨'}
                    </span>
                  </div>
                  <div className='ml-4'>
                    <div className='text-sm font-medium text-gray-900'>
                      {tx.type === 'send' && `Sent ${tx.amount} ${tx.asset}`}
                      {tx.type === 'receive' &&
                        `Received ${tx.amount} ${tx.asset}`}
                    </div>
                    <div className='text-sm text-gray-500'>
                      {formatTime(tx.timestamp)}
                    </div>
                  </div>
                </div>
                <div className='flex items-center'>
                  <span
                    className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
                      tx.status === 'confirmed'
                        ? 'bg-green-100 text-green-800'
                        : 'bg-yellow-100 text-yellow-800'
                    }`}
                  >
                    {tx.status}
                  </span>
                </div>
              </div>
            </div>
          ))}
        </div>
        <div className='px-6 py-3 bg-gray-50'>
          <button className='text-sm text-green-600 hover:text-green-500 font-medium'>
            View all transactions →
          </button>
        </div>
      </motion.div>

      <CreateWalletModal
        isOpen={showCreateModal}
        onClose={() => setShowCreateModal(false)}
        onSubmit={handleCreateWallet}
        loading={loading}
      />
    </div>
  );
}
