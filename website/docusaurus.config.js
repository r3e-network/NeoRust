// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const {themes} = require('prism-react-renderer');
const lightCodeTheme = themes.github;
const darkCodeTheme = themes.dracula;

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'NeoRust',
  tagline: 'Production-Ready Neo N3 Development Suite',
  favicon: 'img/favicon.ico',

  // Set the production url of your site here
  url: 'https://neorust.netlify.app',
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often '/<projectName>/'
  baseUrl: '/',

  // GitHub pages deployment config.
  // If you aren't using GitHub pages, you don't need these.
  organizationName: 'R3E-Network', // Usually your GitHub org/user name.
  projectName: 'NeoRust', // Usually your repo name.

  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',

  // Even if you don't use internalization, you can use this field to set useful
  // metadata like html lang. For example, if your site is Chinese, you may want
  // to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  presets: [
    [
      'classic',
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: require.resolve('./sidebars.js'),
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            'https://github.com/R3E-Network/NeoRust/tree/main/website/',
        },
        blog: {
          showReadingTime: true,
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            'https://github.com/R3E-Network/NeoRust/tree/main/website/',
        },
        theme: {
          customCss: require.resolve('./src/css/custom.css'),
        },
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      // Replace with your project's social card
      image: 'img/neorust-social-card.jpg',
      navbar: {
        title: 'NeoRust',
        logo: {
          alt: 'NeoRust Logo',
          src: 'img/logo.svg',
        },
        items: [
          {
            type: 'docSidebar',
            sidebarId: 'tutorialSidebar',
            position: 'left',
            label: 'Documentation',
          },
          {
            to: '/gui',
            label: 'Desktop GUI',
            position: 'left',
          },
          {
            to: '/cli',
            label: 'CLI Tools',
            position: 'left',
          },
          {
            to: '/sdk',
            label: 'Rust SDK',
            position: 'left',
          },
          {
            to: '/blog',
            label: 'Blog',
            position: 'left'
          },
          {
            href: 'https://docs.rs/neo3',
            label: 'API Docs',
            position: 'right',
          },
          {
            href: 'https://github.com/R3E-Network/NeoRust',
            label: 'GitHub',
            position: 'right',
          },
        ],
      },
      footer: {
        style: 'dark',
        links: [
          {
            title: 'Documentation',
            items: [
              {
                label: 'Getting Started',
                to: '/docs/intro',
              },
              {
                label: 'Desktop GUI',
                to: '/gui',
              },
              {
                label: 'CLI Tools',
                to: '/cli',
              },
              {
                label: 'Rust SDK',
                to: '/sdk',
              },
            ],
          },
          {
            title: 'Community',
            items: [
              {
                label: 'GitHub Discussions',
                href: 'https://github.com/R3E-Network/NeoRust/discussions',
              },
              {
                label: 'Issues',
                href: 'https://github.com/R3E-Network/NeoRust/issues',
              },
              {
                label: 'Neo Discord',
                href: 'https://discord.gg/neo',
              },
            ],
          },
          {
            title: 'Resources',
            items: [
              {
                label: 'Blog',
                to: '/blog',
              },
              {
                label: 'API Reference',
                href: 'https://docs.rs/neo3',
              },
              {
                label: 'Crates.io',
                href: 'https://crates.io/crates/neo3',
              },
              {
                label: 'GitHub',
                href: 'https://github.com/R3E-Network/NeoRust',
              },
            ],
          },
        ],
        copyright: `Copyright © ${new Date().getFullYear()} R3E Network. Built with Docusaurus.`,
      },
      prism: {
        theme: lightCodeTheme,
        darkTheme: darkCodeTheme,
        additionalLanguages: ['rust', 'toml', 'bash'],
      },
      colorMode: {
        defaultMode: 'light',
        disableSwitch: false,
        respectPrefersColorScheme: true,
      },
      algolia: {
        // The application ID provided by Algolia
        appId: 'YOUR_APP_ID',
        // Public API key: it is safe to commit it
        apiKey: 'YOUR_SEARCH_API_KEY',
        indexName: 'neorust',
        // Optional: see doc section below
        contextualSearch: true,
        // Optional: Specify domains where the navigation should occur through window.location instead on history.push
        externalUrlRegex: 'external\\.com|domain\\.com',
        // Optional: Replace parts of the item URLs from Algolia. Useful when using the same search index for multiple deployments using a different baseUrl. You can use regexp or string in the `from` param. For example: localhost:3000 vs myCompany.com/docs
        replaceSearchResultPathname: {
          from: '/docs/', // or as RegExp: /\/docs\//
          to: '/',
        },
        // Optional: Algolia search parameters
        searchParameters: {},
        // Optional: path for search page that enabled by default (`false` to disable it)
        searchPagePath: 'search',
      },
    }),

  plugins: [
    [
      '@docusaurus/plugin-content-docs',
      {
        id: 'gui',
        path: 'gui',
        routeBasePath: 'gui',
        sidebarPath: require.resolve('./sidebars-gui.js'),
      },
    ],
    [
      '@docusaurus/plugin-content-docs',
      {
        id: 'cli',
        path: 'cli',
        routeBasePath: 'cli',
        sidebarPath: require.resolve('./sidebars-cli.js'),
      },
    ],
    [
      '@docusaurus/plugin-content-docs',
      {
        id: 'sdk',
        path: 'sdk',
        routeBasePath: 'sdk',
        sidebarPath: require.resolve('./sidebars-sdk.js'),
      },
    ],
  ],
};

module.exports = config; 