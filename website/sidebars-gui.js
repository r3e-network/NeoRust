// @ts-check

/**
 * NeoRust Desktop GUI Sidebar Configuration v0.4.1
 * Simplified navigation for the Desktop GUI documentation
 * @type {import('@docusaurus/plugin-content-docs').SidebarsConfig}
 */

const guiSidebar = {
  guiSidebar: [
    // GUI Overview
    {
      type: 'doc',
      id: 'intro',
      label: '🖥️ Desktop GUI Overview',
    },

    // Getting Started
    {
      type: 'category',
      label: '🚀 Getting Started',
      collapsed: false,
      items: [
        'installation',
        'first-wallet',
        'basic-operations',
        'token-operations',
      ],
    },

    // Configuration
    {
      type: 'category',
      label: '⚙️ Configuration',
      collapsed: true,
      items: [
        'wallet-management',
        'settings',
        'transactions',
      ],
    },
  ],
};

module.exports = guiSidebar; 