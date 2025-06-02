/**
 * NeoRust CLI Tools Sidebar Configuration v0.4.1
 * Simplified navigation for the CLI Tools documentation
 * @type {import('@docusaurus/plugin-content-docs').SidebarsConfig}
 */

const cliSidebar = {
  cliSidebar: [
    // CLI Overview
    {
      type: 'doc',
      id: 'intro',
      label: '⌨️ CLI Tools Overview',
    },

    // Commands
    {
      type: 'category',
      label: '📝 Commands',
      collapsed: false,
      items: [
        'commands',
      ],
    },

    // Configuration
    {
      type: 'category', 
      label: '⚙️ Configuration',
      collapsed: false,
      items: [
        'configuration',
      ],
    },
  ],
};

module.exports = cliSidebar; 