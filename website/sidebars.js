/**
 * NeoRust Documentation Sidebar Configuration v1.4.0
 * Creating a simplified navigation structure for the documentation
 * @type {import('@docusaurus/plugin-content-docs').SidebarsConfig}
 */

const sidebars = {
  // Main tutorial sidebar
  tutorialSidebar: [
    // Introduction
    {
      type: 'doc',
      id: 'intro',
      label: '👋 Welcome to NeoRust',
    },

    // Getting Started
    {
      type: 'category',
      label: '🚀 Getting Started',
      collapsed: false,
      items: [
        'getting-started/installation',
        'getting-started/quick-start',
      ],
    },

    // Testing
    {
      type: 'doc',
      id: 'testing',
      label: '🧪 Testing',
    },
  ],
};

module.exports = sidebars; 