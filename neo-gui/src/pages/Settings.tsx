import React, { useState, useEffect } from 'react';
import { motion } from 'framer-motion';
import {
  CogIcon,
  ShieldCheckIcon,
  GlobeAltIcon,
  PaintBrushIcon,
  BellIcon,
  KeyIcon,
  ServerIcon,
  InformationCircleIcon,
} from '@heroicons/react/24/outline';
import { useAppStore } from '../stores/appStore';
import { invoke } from '@tauri-apps/api/core';

interface AppSettings {
  theme: 'light' | 'dark' | 'auto';
  language: string;
  currency: string;
  autoLockTimeout: number;
  showBalanceInFiat: boolean;
  enableNotifications: boolean;
  defaultNetwork: string;
  requirePasswordForTransactions: boolean;
  autoLogoutOnIdle: boolean;
  enableBiometricAuth: boolean;
  backupReminderInterval: number;
  enableDebugMode: boolean;
  logLevel: string;
  cacheSizeMb: number;
  maxTransactionHistory: number;
}

export default function Settings() {
  const { addNotification } = useAppStore();
  const [activeTab, setActiveTab] = useState<'general' | 'security' | 'network' | 'advanced'>('general');
  const [settings, setSettings] = useState<AppSettings>({
    theme: 'auto',
    language: 'english',
    currency: 'usd',
    autoLockTimeout: 15,
    showBalanceInFiat: true,
    enableNotifications: true,
    defaultNetwork: 'testnet',
    requirePasswordForTransactions: true,
    autoLogoutOnIdle: true,
    enableBiometricAuth: false,
    backupReminderInterval: 30,
    enableDebugMode: false,
    logLevel: 'info',
    cacheSizeMb: 100,
    maxTransactionHistory: 1000,
  });
  const [loading, setLoading] = useState(false);
  const [hasChanges, setHasChanges] = useState(false);

  useEffect(() => {
    loadSettings();
  }, []);

  const loadSettings = async () => {
    try {
      const result = await invoke('get_settings');
      if (result) {
        setSettings(result as AppSettings);
      }
    } catch (error) {
      console.error('Failed to load settings:', error);
    }
  };

  const saveSettings = async () => {
    setLoading(true);
    try {
      await invoke('update_settings', { settings });
      setHasChanges(false);
      addNotification({
        type: 'success',
        title: 'Settings Saved',
        message: 'Your settings have been updated successfully.',
      });
    } catch (error) {
      addNotification({
        type: 'error',
        title: 'Save Failed',
        message: 'Failed to save settings. Please try again.',
      });
    } finally {
      setLoading(false);
    }
  };

  const resetSettings = async () => {
    if (globalThis.confirm('Are you sure you want to reset all settings to default values?')) {
      setLoading(true);
      try {
        await invoke('reset_settings');
        await loadSettings();
        setHasChanges(false);
        addNotification({
          type: 'success',
          title: 'Settings Reset',
          message: 'All settings have been reset to default values.',
        });
      } catch (error) {
        addNotification({
          type: 'error',
          title: 'Reset Failed',
          message: 'Failed to reset settings. Please try again.',
        });
      } finally {
        setLoading(false);
      }
    }
  };

  const updateSetting = (key: keyof AppSettings, value: any) => {
    setSettings(prev => ({ ...prev, [key]: value }));
    setHasChanges(true);
  };

  const tabs = [
    { id: 'general', name: 'General', icon: CogIcon },
    { id: 'security', name: 'Security', icon: ShieldCheckIcon },
    { id: 'network', name: 'Network', icon: GlobeAltIcon },
    { id: 'advanced', name: 'Advanced', icon: ServerIcon },
  ];

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="md:flex md:items-center md:justify-between">
        <div className="flex-1 min-w-0">
          <h2 className="text-2xl font-bold leading-7 text-gray-900 sm:text-3xl sm:truncate">
            Settings
          </h2>
          <p className="mt-1 text-sm text-gray-500">
            Configure your wallet and application preferences.
          </p>
        </div>
        <div className="mt-4 flex space-x-3 md:mt-0 md:ml-4">
          {hasChanges && (
            <button
              onClick={resetSettings}
              disabled={loading}
              className="inline-flex items-center px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 disabled:opacity-50"
            >
              Reset
            </button>
          )}
          <button
            onClick={saveSettings}
            disabled={!hasChanges || loading}
            className="inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-green-600 hover:bg-green-700 disabled:opacity-50"
          >
            {loading ? 'Saving...' : 'Save Changes'}
          </button>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
        {/* Tab Navigation */}
        <div className="lg:col-span-1">
          <nav className="space-y-1">
            {tabs.map((tab) => (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id as any)}
                className={`${
                  activeTab === tab.id
                    ? 'bg-green-50 border-green-500 text-green-700'
                    : 'border-transparent text-gray-900 hover:bg-gray-50 hover:text-gray-900'
                } group border-l-4 px-3 py-2 flex items-center text-sm font-medium w-full`}
              >
                <tab.icon
                  className={`${
                    activeTab === tab.id ? 'text-green-500' : 'text-gray-400 group-hover:text-gray-500'
                  } flex-shrink-0 -ml-1 mr-3 h-6 w-6`}
                />
                <span className="truncate">{tab.name}</span>
              </button>
            ))}
          </nav>
        </div>

        {/* Settings Content */}
        <div className="lg:col-span-3">
          <motion.div
            key={activeTab}
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.2 }}
            className="bg-white shadow rounded-lg"
          >
            {/* General Settings */}
            {activeTab === 'general' && (
              <div className="p-6 space-y-6">
                <div>
                  <h3 className="text-lg font-medium text-gray-900 flex items-center">
                    <PaintBrushIcon className="h-5 w-5 mr-2" />
                    Appearance
                  </h3>
                  <div className="mt-4 space-y-4">
                    <div>
                      <label className="block text-sm font-medium text-gray-700">Theme</label>
                      <select
                        value={settings.theme}
                        onChange={(e) => updateSetting('theme', e.target.value)}
                        className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-green-500 focus:border-green-500"
                      >
                        <option value="light">Light</option>
                        <option value="dark">Dark</option>
                        <option value="auto">Auto (System)</option>
                      </select>
                    </div>

                    <div>
                      <label className="block text-sm font-medium text-gray-700">Language</label>
                      <select
                        value={settings.language}
                        onChange={(e) => updateSetting('language', e.target.value)}
                        className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-green-500 focus:border-green-500"
                      >
                        <option value="english">English</option>
                        <option value="chinese">中文</option>
                        <option value="japanese">日本語</option>
                        <option value="korean">한국어</option>
                        <option value="spanish">Español</option>
                        <option value="french">Français</option>
                        <option value="german">Deutsch</option>
                      </select>
                    </div>

                    <div>
                      <label className="block text-sm font-medium text-gray-700">Currency</label>
                      <select
                        value={settings.currency}
                        onChange={(e) => updateSetting('currency', e.target.value)}
                        className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-green-500 focus:border-green-500"
                      >
                        <option value="usd">USD ($)</option>
                        <option value="eur">EUR (€)</option>
                        <option value="cny">CNY (¥)</option>
                        <option value="jpy">JPY (¥)</option>
                        <option value="krw">KRW (₩)</option>
                        <option value="btc">BTC (₿)</option>
                        <option value="eth">ETH (Ξ)</option>
                      </select>
                    </div>
                  </div>
                </div>

                <div className="border-t border-gray-200 pt-6">
                  <h3 className="text-lg font-medium text-gray-900 flex items-center">
                    <BellIcon className="h-5 w-5 mr-2" />
                    Preferences
                  </h3>
                  <div className="mt-4 space-y-4">
                    <div className="flex items-center justify-between">
                      <div>
                        <label className="text-sm font-medium text-gray-700">Show balance in fiat currency</label>
                        <p className="text-sm text-gray-500">Display USD equivalent alongside crypto amounts</p>
                      </div>
                      <input
                        type="checkbox"
                        checked={settings.showBalanceInFiat}
                        onChange={(e) => updateSetting('showBalanceInFiat', e.target.checked)}
                        className="h-4 w-4 text-green-600 focus:ring-green-500 border-gray-300 rounded"
                      />
                    </div>

                    <div className="flex items-center justify-between">
                      <div>
                        <label className="text-sm font-medium text-gray-700">Enable notifications</label>
                        <p className="text-sm text-gray-500">Receive notifications for transactions and updates</p>
                      </div>
                      <input
                        type="checkbox"
                        checked={settings.enableNotifications}
                        onChange={(e) => updateSetting('enableNotifications', e.target.checked)}
                        className="h-4 w-4 text-green-600 focus:ring-green-500 border-gray-300 rounded"
                      />
                    </div>

                    <div>
                      <label className="block text-sm font-medium text-gray-700">
                        Auto-lock timeout: {settings.autoLockTimeout} minutes
                      </label>
                      <input
                        type="range"
                        min="5"
                        max="60"
                        value={settings.autoLockTimeout}
                        onChange={(e) => updateSetting('autoLockTimeout', parseInt(e.target.value))}
                        className="mt-1 w-full"
                      />
                      <div className="flex justify-between text-xs text-gray-500 mt-1">
                        <span>5 min</span>
                        <span>60 min</span>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            )}

            {/* Security Settings */}
            {activeTab === 'security' && (
              <div className="p-6 space-y-6">
                <div>
                  <h3 className="text-lg font-medium text-gray-900 flex items-center">
                    <KeyIcon className="h-5 w-5 mr-2" />
                    Authentication
                  </h3>
                  <div className="mt-4 space-y-4">
                    <div className="flex items-center justify-between">
                      <div>
                        <label className="text-sm font-medium text-gray-700">Require password for transactions</label>
                        <p className="text-sm text-gray-500">Always ask for password when sending transactions</p>
                      </div>
                      <input
                        type="checkbox"
                        checked={settings.requirePasswordForTransactions}
                        onChange={(e) => updateSetting('requirePasswordForTransactions', e.target.checked)}
                        className="h-4 w-4 text-green-600 focus:ring-green-500 border-gray-300 rounded"
                      />
                    </div>

                    <div className="flex items-center justify-between">
                      <div>
                        <label className="text-sm font-medium text-gray-700">Auto-logout on idle</label>
                        <p className="text-sm text-gray-500">Automatically lock wallet when inactive</p>
                      </div>
                      <input
                        type="checkbox"
                        checked={settings.autoLogoutOnIdle}
                        onChange={(e) => updateSetting('autoLogoutOnIdle', e.target.checked)}
                        className="h-4 w-4 text-green-600 focus:ring-green-500 border-gray-300 rounded"
                      />
                    </div>

                    <div className="flex items-center justify-between">
                      <div>
                        <label className="text-sm font-medium text-gray-700">Enable biometric authentication</label>
                        <p className="text-sm text-gray-500">Use fingerprint or face recognition (if available)</p>
                      </div>
                      <input
                        type="checkbox"
                        checked={settings.enableBiometricAuth}
                        onChange={(e) => updateSetting('enableBiometricAuth', e.target.checked)}
                        className="h-4 w-4 text-green-600 focus:ring-green-500 border-gray-300 rounded"
                      />
                    </div>
                  </div>
                </div>

                <div className="border-t border-gray-200 pt-6">
                  <h3 className="text-lg font-medium text-gray-900">Backup & Recovery</h3>
                  <div className="mt-4 space-y-4">
                    <div>
                      <label className="block text-sm font-medium text-gray-700">
                        Backup reminder interval: {settings.backupReminderInterval} days
                      </label>
                      <input
                        type="range"
                        min="7"
                        max="90"
                        value={settings.backupReminderInterval}
                        onChange={(e) => updateSetting('backupReminderInterval', parseInt(e.target.value))}
                        className="mt-1 w-full"
                      />
                      <div className="flex justify-between text-xs text-gray-500 mt-1">
                        <span>7 days</span>
                        <span>90 days</span>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            )}

            {/* Network Settings */}
            {activeTab === 'network' && (
              <div className="p-6 space-y-6">
                <div>
                  <h3 className="text-lg font-medium text-gray-900 flex items-center">
                    <GlobeAltIcon className="h-5 w-5 mr-2" />
                    Network Configuration
                  </h3>
                  <div className="mt-4 space-y-4">
                    <div>
                      <label className="block text-sm font-medium text-gray-700">Default Network</label>
                      <select
                        value={settings.defaultNetwork}
                        onChange={(e) => updateSetting('defaultNetwork', e.target.value)}
                        className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-green-500 focus:border-green-500"
                      >
                        <option value="mainnet">Mainnet</option>
                        <option value="testnet">Testnet</option>
                        <option value="private">Private Network</option>
                      </select>
                    </div>
                  </div>
                </div>

                <div className="border-t border-gray-200 pt-6">
                  <h3 className="text-lg font-medium text-gray-900">Custom RPC Endpoints</h3>
                  <div className="mt-4">
                    <div className="bg-gray-50 rounded-lg p-4">
                      <div className="flex items-center">
                        <InformationCircleIcon className="h-5 w-5 text-blue-500 mr-2" />
                        <p className="text-sm text-gray-600">
                          Custom RPC endpoint management will be available in the next update.
                        </p>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            )}

            {/* Advanced Settings */}
            {activeTab === 'advanced' && (
              <div className="p-6 space-y-6">
                <div>
                  <h3 className="text-lg font-medium text-gray-900 flex items-center">
                    <ServerIcon className="h-5 w-5 mr-2" />
                    Developer Options
                  </h3>
                  <div className="mt-4 space-y-4">
                    <div className="flex items-center justify-between">
                      <div>
                        <label className="text-sm font-medium text-gray-700">Enable debug mode</label>
                        <p className="text-sm text-gray-500">Show additional debugging information</p>
                      </div>
                      <input
                        type="checkbox"
                        checked={settings.enableDebugMode}
                        onChange={(e) => updateSetting('enableDebugMode', e.target.checked)}
                        className="h-4 w-4 text-green-600 focus:ring-green-500 border-gray-300 rounded"
                      />
                    </div>

                    <div>
                      <label className="block text-sm font-medium text-gray-700">Log Level</label>
                      <select
                        value={settings.logLevel}
                        onChange={(e) => updateSetting('logLevel', e.target.value)}
                        className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-green-500 focus:border-green-500"
                      >
                        <option value="error">Error</option>
                        <option value="warn">Warning</option>
                        <option value="info">Info</option>
                        <option value="debug">Debug</option>
                        <option value="trace">Trace</option>
                      </select>
                    </div>
                  </div>
                </div>

                <div className="border-t border-gray-200 pt-6">
                  <h3 className="text-lg font-medium text-gray-900">Performance</h3>
                  <div className="mt-4 space-y-4">
                    <div>
                      <label className="block text-sm font-medium text-gray-700">
                        Cache size: {settings.cacheSizeMb} MB
                      </label>
                      <input
                        type="range"
                        min="50"
                        max="500"
                        step="50"
                        value={settings.cacheSizeMb}
                        onChange={(e) => updateSetting('cacheSizeMb', parseInt(e.target.value))}
                        className="mt-1 w-full"
                      />
                      <div className="flex justify-between text-xs text-gray-500 mt-1">
                        <span>50 MB</span>
                        <span>500 MB</span>
                      </div>
                    </div>

                    <div>
                      <label className="block text-sm font-medium text-gray-700">
                        Max transaction history: {settings.maxTransactionHistory}
                      </label>
                      <input
                        type="range"
                        min="100"
                        max="5000"
                        step="100"
                        value={settings.maxTransactionHistory}
                        onChange={(e) => updateSetting('maxTransactionHistory', parseInt(e.target.value))}
                        className="mt-1 w-full"
                      />
                      <div className="flex justify-between text-xs text-gray-500 mt-1">
                        <span>100</span>
                        <span>5000</span>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            )}
          </motion.div>
        </div>
      </div>
    </div>
  );
}