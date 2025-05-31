import React, { useEffect } from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { AnimatePresence } from 'framer-motion';
import Layout from './components/Layout';

// Pages
import Dashboard from './pages/Dashboard';
import Wallet from './pages/Wallet';
import NFT from './pages/NFT';
import Tools from './pages/Tools';
import Analytics from './pages/Analytics';
import Settings from './pages/Settings';

// Stores
import { useAppStore } from './stores/appStore';

// Styles
import './index.css';

const App: React.FC = () => {
  const { loading, addNotification } = useAppStore();

  useEffect(() => {
    // Initialize app
    const initialize = async () => {
      try {
        // Add welcome notification
        addNotification({
          type: 'success',
          title: 'Welcome to Neo N3 Wallet',
          message: 'Your secure gateway to the Neo blockchain',
        });
      } catch (error) {
        console.error('Failed to initialize app:', error);
        addNotification({
          type: 'error',
          title: 'Initialization Error',
          message: 'Failed to initialize the application',
        });
      }
    };

    initialize();
  }, [addNotification]);

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gray-50">
        <div className="text-center">
          <div className="w-16 h-16 mx-auto mb-4 bg-gradient-to-br from-neo-400 to-neo-600 rounded-2xl flex items-center justify-center">
            <span className="text-white font-bold text-xl">N3</span>
          </div>
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-neo-600 mx-auto mb-4"></div>
          <p className="text-gray-600">Loading Neo N3 Wallet...</p>
        </div>
      </div>
    );
  }

  return (
    <Router>
      <div className="App min-h-screen bg-gray-50">
        <Routes>
          <Route path="/" element={<Layout />}>
            <Route index element={<Dashboard />} />
            <Route path="wallet" element={<Wallet />} />
            <Route path="nft" element={<NFT />} />
            <Route path="tools" element={<Tools />} />
            <Route path="analytics" element={<Analytics />} />
            <Route path="settings" element={<Settings />} />
          </Route>
        </Routes>
      </div>
    </Router>
  );
};

export default App; 