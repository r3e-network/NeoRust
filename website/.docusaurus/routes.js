import React from 'react';
import ComponentCreator from '@docusaurus/ComponentCreator';

export default [
  {
    path: '/blog',
    component: ComponentCreator('/blog', 'ace'),
    exact: true
  },
  {
    path: '/blog/archive',
    component: ComponentCreator('/blog/archive', '182'),
    exact: true
  },
  {
    path: '/blog/authors',
    component: ComponentCreator('/blog/authors', '0b7'),
    exact: true
  },
  {
    path: '/blog/building-first-neo-dapp',
    component: ComponentCreator('/blog/building-first-neo-dapp', '21b'),
    exact: true
  },
  {
    path: '/blog/neorust-v0.4.1-release',
    component: ComponentCreator('/blog/neorust-v0.4.1-release', '003'),
    exact: true
  },
  {
    path: '/blog/tags',
    component: ComponentCreator('/blog/tags', '287'),
    exact: true
  },
  {
    path: '/blog/tags/beginner',
    component: ComponentCreator('/blog/tags/beginner', '039'),
    exact: true
  },
  {
    path: '/blog/tags/blockchain',
    component: ComponentCreator('/blog/tags/blockchain', '888'),
    exact: true
  },
  {
    path: '/blog/tags/dapp',
    component: ComponentCreator('/blog/tags/dapp', '603'),
    exact: true
  },
  {
    path: '/blog/tags/neo-3',
    component: ComponentCreator('/blog/tags/neo-3', 'd97'),
    exact: true
  },
  {
    path: '/blog/tags/release',
    component: ComponentCreator('/blog/tags/release', 'f24'),
    exact: true
  },
  {
    path: '/blog/tags/rust',
    component: ComponentCreator('/blog/tags/rust', '1c2'),
    exact: true
  },
  {
    path: '/blog/tags/sdk',
    component: ComponentCreator('/blog/tags/sdk', '5a2'),
    exact: true
  },
  {
    path: '/blog/tags/tutorial',
    component: ComponentCreator('/blog/tags/tutorial', '8bb'),
    exact: true
  },
  {
    path: '/examples',
    component: ComponentCreator('/examples', 'd58'),
    exact: true
  },
  {
    path: '/search',
    component: ComponentCreator('/search', '5de'),
    exact: true
  },
  {
    path: '/cli',
    component: ComponentCreator('/cli', '3c3'),
    routes: [
      {
        path: '/cli',
        component: ComponentCreator('/cli', 'd6f'),
        routes: [
          {
            path: '/cli',
            component: ComponentCreator('/cli', 'b1b'),
            routes: [
              {
                path: '/cli/commands',
                component: ComponentCreator('/cli/commands', 'c46'),
                exact: true,
                sidebar: "cliSidebar"
              },
              {
                path: '/cli/configuration',
                component: ComponentCreator('/cli/configuration', 'e9f'),
                exact: true,
                sidebar: "cliSidebar"
              },
              {
                path: '/cli/intro',
                component: ComponentCreator('/cli/intro', 'ec7'),
                exact: true,
                sidebar: "cliSidebar"
              }
            ]
          }
        ]
      }
    ]
  },
  {
    path: '/docs',
    component: ComponentCreator('/docs', '003'),
    routes: [
      {
        path: '/docs',
        component: ComponentCreator('/docs', '61f'),
        routes: [
          {
            path: '/docs',
            component: ComponentCreator('/docs', '895'),
            routes: [
              {
                path: '/docs/getting-started/installation',
                component: ComponentCreator('/docs/getting-started/installation', '876'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/getting-started/quick-start',
                component: ComponentCreator('/docs/getting-started/quick-start', '01c'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/intro',
                component: ComponentCreator('/docs/intro', '289'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/testing',
                component: ComponentCreator('/docs/testing', '5bd'),
                exact: true,
                sidebar: "tutorialSidebar"
              }
            ]
          }
        ]
      }
    ]
  },
  {
    path: '/gui',
    component: ComponentCreator('/gui', '17e'),
    routes: [
      {
        path: '/gui',
        component: ComponentCreator('/gui', 'b31'),
        routes: [
          {
            path: '/gui',
            component: ComponentCreator('/gui', 'b94'),
            routes: [
              {
                path: '/gui/basic-operations',
                component: ComponentCreator('/gui/basic-operations', 'e04'),
                exact: true,
                sidebar: "guiSidebar"
              },
              {
                path: '/gui/first-wallet',
                component: ComponentCreator('/gui/first-wallet', '61d'),
                exact: true,
                sidebar: "guiSidebar"
              },
              {
                path: '/gui/installation',
                component: ComponentCreator('/gui/installation', 'bc3'),
                exact: true,
                sidebar: "guiSidebar"
              },
              {
                path: '/gui/intro',
                component: ComponentCreator('/gui/intro', 'ad2'),
                exact: true,
                sidebar: "guiSidebar"
              },
              {
                path: '/gui/settings',
                component: ComponentCreator('/gui/settings', '0e9'),
                exact: true,
                sidebar: "guiSidebar"
              },
              {
                path: '/gui/token-operations',
                component: ComponentCreator('/gui/token-operations', 'bb6'),
                exact: true,
                sidebar: "guiSidebar"
              },
              {
                path: '/gui/transactions',
                component: ComponentCreator('/gui/transactions', '17c'),
                exact: true,
                sidebar: "guiSidebar"
              },
              {
                path: '/gui/wallet-management',
                component: ComponentCreator('/gui/wallet-management', '164'),
                exact: true,
                sidebar: "guiSidebar"
              }
            ]
          }
        ]
      }
    ]
  },
  {
    path: '/sdk',
    component: ComponentCreator('/sdk', '14a'),
    routes: [
      {
        path: '/sdk',
        component: ComponentCreator('/sdk', '3bd'),
        routes: [
          {
            path: '/sdk',
            component: ComponentCreator('/sdk', '319'),
            routes: [
              {
                path: '/sdk/api-reference',
                component: ComponentCreator('/sdk/api-reference', '4b9'),
                exact: true,
                sidebar: "sdkSidebar"
              },
              {
                path: '/sdk/contracts',
                component: ComponentCreator('/sdk/contracts', '9e6'),
                exact: true,
                sidebar: "sdkSidebar"
              },
              {
                path: '/sdk/examples',
                component: ComponentCreator('/sdk/examples', '164'),
                exact: true,
                sidebar: "sdkSidebar"
              },
              {
                path: '/sdk/installation',
                component: ComponentCreator('/sdk/installation', 'ce5'),
                exact: true,
                sidebar: "sdkSidebar"
              },
              {
                path: '/sdk/intro',
                component: ComponentCreator('/sdk/intro', 'ccf'),
                exact: true,
                sidebar: "sdkSidebar"
              },
              {
                path: '/sdk/quick-start',
                component: ComponentCreator('/sdk/quick-start', 'b28'),
                exact: true,
                sidebar: "sdkSidebar"
              },
              {
                path: '/sdk/tokens',
                component: ComponentCreator('/sdk/tokens', '590'),
                exact: true,
                sidebar: "sdkSidebar"
              },
              {
                path: '/sdk/transactions',
                component: ComponentCreator('/sdk/transactions', '3d4'),
                exact: true,
                sidebar: "sdkSidebar"
              },
              {
                path: '/sdk/troubleshooting',
                component: ComponentCreator('/sdk/troubleshooting', '294'),
                exact: true,
                sidebar: "sdkSidebar"
              },
              {
                path: '/sdk/wallets',
                component: ComponentCreator('/sdk/wallets', '3a1'),
                exact: true,
                sidebar: "sdkSidebar"
              }
            ]
          }
        ]
      }
    ]
  },
  {
    path: '/',
    component: ComponentCreator('/', 'e5f'),
    exact: true
  },
  {
    path: '*',
    component: ComponentCreator('*'),
  },
];
