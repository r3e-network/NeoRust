import React, { useState, useEffect } from 'react';
import { motion } from 'framer-motion';
import {
  ArrowsRightLeftIcon,
  PlusIcon,
  MinusIcon,
  ChartBarIcon,
  CurrencyDollarIcon,
  TrophyIcon,
} from '@heroicons/react/24/outline';
import { useAppStore } from '../stores/appStore';
import { invoke } from '@tauri-apps/api/tauri';

interface Pool {
  id: string;
  tokenA: string;
  tokenB: string;
  reserveA: string;
  reserveB: string;
  totalLiquidity: string;
  apr: number;
  volume24h: string;
  fees24h: string;
}

interface Token {
  hash: string;
  symbol: string;
  name: string;
  decimals: number;
  price: number;
  balance: string;
}

export default function DeFi() {
  const { currentWallet, addNotification } = useAppStore();
  const [activeTab, setActiveTab] = useState<'swap' | 'liquidity' | 'stake'>('swap');
  const [pools, setPools] = useState<Pool[]>([]);
  const [tokens, setTokens] = useState<Token[]>([]);
  const [loading, setLoading] = useState(false);

  // Swap state
  const [fromToken, setFromToken] = useState<Token | null>(null);
  const [toToken, setToToken] = useState<Token | null>(null);
  const [fromAmount, setFromAmount] = useState('');
  const [toAmount, setToAmount] = useState('');
  const [slippage, setSlippage] = useState(0.5);

  useEffect(() => {
    loadTokens();
    loadPools();
  }, []);

  useEffect(() => {
    if (fromToken && toToken && fromAmount) {
      calculateSwapAmount();
    }
  }, [fromToken, toToken, fromAmount]);

  const loadTokens = async () => {
    // Mock token data
    setTokens([
      {
        hash: '0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5',
        symbol: 'NEO',
        name: 'Neo Token',
        decimals: 0,
        price: 25.50,
        balance: '45.5',
      },
      {
        hash: '0xd2a4cff31913016155e38e474a2c06d08be276cf',
        symbol: 'GAS',
        name: 'Gas Token',
        decimals: 8,
        price: 1.25,
        balance: '125.25',
      },
      {
        hash: '0x48c40d4666f93408be1bef038b6722404d9a4c2a',
        symbol: 'FLUND',
        name: 'Flamingo USD',
        decimals: 8,
        price: 1.00,
        balance: '1000.00',
      },
    ]);
  };

  const loadPools = async () => {
    try {
      const mockPools: Pool[] = [
        {
          id: 'neo-gas',
          tokenA: 'NEO',
          tokenB: 'GAS',
          reserveA: '1000000',
          reserveB: '5000000',
          totalLiquidity: '2236067.977',
          apr: 12.5,
          volume24h: '500000',
          fees24h: '1500',
        },
        {
          id: 'neo-flund',
          tokenA: 'NEO',
          tokenB: 'FLUND',
          reserveA: '500000',
          reserveB: '12750000',
          totalLiquidity: '2500000',
          apr: 8.3,
          volume24h: '250000',
          fees24h: '750',
        },
      ];
      setPools(mockPools);
    } catch (error) {
      console.error('Failed to load pools:', error);
    }
  };

  const calculateSwapAmount = async () => {
    if (!fromToken || !toToken || !fromAmount) return;

    // Mock calculation - in real implementation, this would call the DEX contract
    const rate = toToken.price / fromToken.price;
    const calculatedAmount = (parseFloat(fromAmount) * rate * (1 - slippage / 100)).toFixed(8);
    setToAmount(calculatedAmount);
  };

  const handleSwap = async () => {
    if (!currentWallet || !fromToken || !toToken || !fromAmount) return;

    setLoading(true);
    try {
      const result = await invoke('swap_tokens', {
        request: {
          fromToken: fromToken.hash,
          toToken: toToken.hash,
          amount: fromAmount,
          walletId: currentWallet.id,
          slippage,
        },
      });

      addNotification({
        type: 'success',
        title: 'Swap Initiated',
        message: `Swapping ${fromAmount} ${fromToken.symbol} for ${toAmount} ${toToken.symbol}`,
      });

      // Reset form
      setFromAmount('');
      setToAmount('');
    } catch (error) {
      addNotification({
        type: 'error',
        title: 'Swap Failed',
        message: 'Failed to execute swap transaction',
      });
    } finally {
      setLoading(false);
    }
  };

  const handleAddLiquidity = async (poolId: string, amountA: string, amountB: string) => {
    if (!currentWallet) return;

    setLoading(true);
    try {
      const pool = pools.find(p => p.id === poolId);
      if (!pool) return;

      await invoke('add_liquidity', {
        request: {
          tokenA: pool.tokenA,
          tokenB: pool.tokenB,
          amountA,
          amountB,
          walletId: currentWallet.id,
        },
      });

      addNotification({
        type: 'success',
        title: 'Liquidity Added',
        message: `Added liquidity to ${pool.tokenA}/${pool.tokenB} pool`,
      });
    } catch (error) {
      addNotification({
        type: 'error',
        title: 'Failed to Add Liquidity',
        message: 'Transaction failed',
      });
    } finally {
      setLoading(false);
    }
  };

  if (!currentWallet) {
    return (
      <div className="flex items-center justify-center h-96">
        <div className="text-center">
          <CurrencyDollarIcon className="mx-auto h-12 w-12 text-gray-400" />
          <h3 className="mt-2 text-sm font-medium text-gray-900">Connect Wallet</h3>
          <p className="mt-1 text-sm text-gray-500">
            Connect your wallet to access DeFi features.
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="md:flex md:items-center md:justify-between">
        <div className="flex-1 min-w-0">
          <h2 className="text-2xl font-bold leading-7 text-gray-900 sm:text-3xl sm:truncate">
            DeFi Hub
          </h2>
          <p className="mt-1 text-sm text-gray-500">
            Swap tokens, provide liquidity, and earn rewards on Neo blockchain.
          </p>
        </div>
      </div>

      {/* Tab Navigation */}
      <div className="border-b border-gray-200">
        <nav className="-mb-px flex space-x-8">
          {[
            { id: 'swap', name: 'Swap', icon: ArrowsRightLeftIcon },
            { id: 'liquidity', name: 'Liquidity', icon: PlusIcon },
            { id: 'stake', name: 'Stake', icon: TrophyIcon },
          ].map((tab) => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id as any)}
              className={`${
                activeTab === tab.id
                  ? 'border-green-500 text-green-600'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
              } whitespace-nowrap py-2 px-1 border-b-2 font-medium text-sm flex items-center`}
            >
              <tab.icon className="h-5 w-5 mr-2" />
              {tab.name}
            </button>
          ))}
        </nav>
      </div>

      {/* Swap Tab */}
      {activeTab === 'swap' && (
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <div className="lg:col-span-2">
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              className="bg-white shadow rounded-lg p-6"
            >
              <h3 className="text-lg font-medium text-gray-900 mb-6">Swap Tokens</h3>
              
              <div className="space-y-4">
                {/* From Token */}
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">From</label>
                  <div className="flex space-x-3">
                    <select
                      value={fromToken?.hash || ''}
                      onChange={(e) => setFromToken(tokens.find(t => t.hash === e.target.value) || null)}
                      className="flex-1 border-gray-300 rounded-md shadow-sm focus:ring-green-500 focus:border-green-500"
                    >
                      <option value="">Select token</option>
                      {tokens.map((token) => (
                        <option key={token.hash} value={token.hash}>
                          {token.symbol} - {token.name}
                        </option>
                      ))}
                    </select>
                    <input
                      type="number"
                      value={fromAmount}
                      onChange={(e) => setFromAmount(e.target.value)}
                      placeholder="0.0"
                      className="w-32 border-gray-300 rounded-md shadow-sm focus:ring-green-500 focus:border-green-500"
                    />
                  </div>
                  {fromToken && (
                    <p className="mt-1 text-sm text-gray-500">
                      Balance: {fromToken.balance} {fromToken.symbol}
                    </p>
                  )}
                </div>

                {/* Swap Direction */}
                <div className="flex justify-center">
                  <button
                    onClick={() => {
                      const temp = fromToken;
                      setFromToken(toToken);
                      setToToken(temp);
                      setFromAmount(toAmount);
                      setToAmount(fromAmount);
                    }}
                    className="p-2 border border-gray-300 rounded-full hover:bg-gray-50"
                  >
                    <ArrowsRightLeftIcon className="h-5 w-5 text-gray-400" />
                  </button>
                </div>

                {/* To Token */}
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">To</label>
                  <div className="flex space-x-3">
                    <select
                      value={toToken?.hash || ''}
                      onChange={(e) => setToToken(tokens.find(t => t.hash === e.target.value) || null)}
                      className="flex-1 border-gray-300 rounded-md shadow-sm focus:ring-green-500 focus:border-green-500"
                    >
                      <option value="">Select token</option>
                      {tokens.map((token) => (
                        <option key={token.hash} value={token.hash}>
                          {token.symbol} - {token.name}
                        </option>
                      ))}
                    </select>
                    <input
                      type="number"
                      value={toAmount}
                      readOnly
                      placeholder="0.0"
                      className="w-32 border-gray-300 rounded-md shadow-sm bg-gray-50"
                    />
                  </div>
                  {toToken && (
                    <p className="mt-1 text-sm text-gray-500">
                      Balance: {toToken.balance} {toToken.symbol}
                    </p>
                  )}
                </div>

                {/* Slippage */}
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Slippage Tolerance: {slippage}%
                  </label>
                  <input
                    type="range"
                    min="0.1"
                    max="5"
                    step="0.1"
                    value={slippage}
                    onChange={(e) => setSlippage(parseFloat(e.target.value))}
                    className="w-full"
                  />
                </div>

                {/* Swap Button */}
                <button
                  onClick={handleSwap}
                  disabled={!fromToken || !toToken || !fromAmount || loading}
                  className="w-full bg-green-600 text-white py-3 px-4 rounded-md hover:bg-green-700 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  {loading ? 'Swapping...' : 'Swap Tokens'}
                </button>
              </div>
            </motion.div>
          </div>

          {/* Swap Info */}
          <div>
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 0.1 }}
              className="bg-white shadow rounded-lg p-6"
            >
              <h4 className="text-lg font-medium text-gray-900 mb-4">Swap Details</h4>
              
              {fromToken && toToken && fromAmount && toAmount && (
                <div className="space-y-3">
                  <div className="flex justify-between">
                    <span className="text-sm text-gray-500">Exchange Rate</span>
                    <span className="text-sm font-medium">
                      1 {fromToken.symbol} = {(toToken.price / fromToken.price).toFixed(6)} {toToken.symbol}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-sm text-gray-500">Price Impact</span>
                    <span className="text-sm font-medium text-green-600">{'<0.01%'}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-sm text-gray-500">Minimum Received</span>
                    <span className="text-sm font-medium">
                      {(parseFloat(toAmount) * (1 - slippage / 100)).toFixed(6)} {toToken.symbol}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-sm text-gray-500">Network Fee</span>
                    <span className="text-sm font-medium">~0.001 GAS</span>
                  </div>
                </div>
              )}
            </motion.div>
          </div>
        </div>
      )}

      {/* Liquidity Tab */}
      {activeTab === 'liquidity' && (
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="bg-white shadow rounded-lg"
        >
          <div className="px-6 py-4 border-b border-gray-200">
            <h3 className="text-lg font-medium text-gray-900">Liquidity Pools</h3>
          </div>
          
          <div className="divide-y divide-gray-200">
            {pools.map((pool) => (
              <div key={pool.id} className="px-6 py-4">
                <div className="flex items-center justify-between">
                  <div>
                    <h4 className="text-lg font-medium text-gray-900">
                      {pool.tokenA}/{pool.tokenB}
                    </h4>
                    <div className="mt-1 text-sm text-gray-500">
                      TVL: ${parseFloat(pool.totalLiquidity).toLocaleString()} • APR: {pool.apr}%
                    </div>
                  </div>
                  <div className="flex space-x-2">
                    <button
                      onClick={() => handleAddLiquidity(pool.id, '10', '50')}
                      className="inline-flex items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md text-white bg-green-600 hover:bg-green-700"
                    >
                      <PlusIcon className="h-4 w-4 mr-1" />
                      Add
                    </button>
                    <button className="inline-flex items-center px-3 py-2 border border-gray-300 text-sm leading-4 font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50">
                      <MinusIcon className="h-4 w-4 mr-1" />
                      Remove
                    </button>
                  </div>
                </div>
                
                <div className="mt-4 grid grid-cols-3 gap-4 text-sm">
                  <div>
                    <span className="text-gray-500">24h Volume</span>
                    <div className="font-medium">${parseFloat(pool.volume24h).toLocaleString()}</div>
                  </div>
                  <div>
                    <span className="text-gray-500">24h Fees</span>
                    <div className="font-medium">${parseFloat(pool.fees24h).toLocaleString()}</div>
                  </div>
                  <div>
                    <span className="text-gray-500">Your Share</span>
                    <div className="font-medium">0%</div>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </motion.div>
      )}

      {/* Stake Tab */}
      {activeTab === 'stake' && (
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="bg-white shadow rounded-lg p-6"
        >
          <h3 className="text-lg font-medium text-gray-900 mb-6">Staking Pools</h3>
          
          <div className="text-center py-12">
            <TrophyIcon className="mx-auto h-12 w-12 text-gray-400" />
            <h3 className="mt-2 text-sm font-medium text-gray-900">Coming Soon</h3>
            <p className="mt-1 text-sm text-gray-500">
              Staking features will be available in the next update.
            </p>
          </div>
        </motion.div>
      )}
    </div>
  );
}
