// @ts-check

/**
 * NeoRust SDK Sidebar Configuration v1.0.1
 * Simplified navigation for the Rust SDK documentation
 * @type {import('@docusaurus/plugin-content-docs').SidebarsConfig}
 */

const sdkSidebar = {
  sdkSidebar: [
    // SDK Overview
    {
      type: 'doc',
      id: 'intro',
      label: '🦀 Rust SDK Overview',
    },

    // Getting Started
    {
      type: 'category',
      label: '🚀 Getting Started',
      collapsed: false,
      items: [
        'installation',
        'quick-start',
        'examples',
      ],
    },

    // Core Documentation
    {
      type: 'category',
      label: '📚 Core Concepts',
      collapsed: true,
      items: [
        'wallets',
        'transactions',
        'contracts',
        'tokens',
        'neox',
      ],
    },

    // API Reference
    {
      type: 'category',
      label: '📖 Reference',
      collapsed: true,
      items: [
        'api-reference',
        'troubleshooting',
      ],
    },
  ],
};

module.exports = sdkSidebar; 