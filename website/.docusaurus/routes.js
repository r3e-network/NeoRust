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
    component: ComponentCreator('/cli', '9ba'),
    routes: [
      {
        path: '/cli',
        component: ComponentCreator('/cli', '1af'),
        routes: [
          {
            path: '/cli',
            component: ComponentCreator('/cli', '8ab'),
            routes: [
              {
                path: '/cli/commands',
                component: ComponentCreator('/cli/commands', 'c46'),
                exact: true,
                sidebar: "cliSidebar"
              },
              {
                path: '/cli/configuration',
                component: ComponentCreator('/cli/configuration', 'fd1'),
                exact: true,
                sidebar: "cliSidebar"
              },
              {
                path: '/cli/intro',
                component: ComponentCreator('/cli/intro', '671'),
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
    component: ComponentCreator('/docs', '053'),
    routes: [
      {
        path: '/docs',
        component: ComponentCreator('/docs', '6a5'),
        routes: [
          {
            path: '/docs',
            component: ComponentCreator('/docs', '769'),
            routes: [
              {
                path: '/docs/getting-started/installation',
                component: ComponentCreator('/docs/getting-started/installation', '88d'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/getting-started/quick-start',
                component: ComponentCreator('/docs/getting-started/quick-start', 'a2d'),
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
    component: ComponentCreator('/gui', '18a'),
    routes: [
      {
        path: '/gui',
        component: ComponentCreator('/gui', '4c5'),
        routes: [
          {
            path: '/gui',
            component: ComponentCreator('/gui', 'c6f'),
            routes: [
              {
                path: '/gui/basic-operations',
                component: ComponentCreator('/gui/basic-operations', '3f3'),
                exact: true,
                sidebar: "guiSidebar"
              },
              {
                path: '/gui/first-wallet',
                component: ComponentCreator('/gui/first-wallet', 'efc'),
                exact: true,
                sidebar: "guiSidebar"
              },
              {
                path: '/gui/installation',
                component: ComponentCreator('/gui/installation', '938'),
                exact: true,
                sidebar: "guiSidebar"
              },
              {
                path: '/gui/intro',
                component: ComponentCreator('/gui/intro', '0b2'),
                exact: true,
                sidebar: "guiSidebar"
              },
              {
                path: '/gui/settings',
                component: ComponentCreator('/gui/settings', 'be4'),
                exact: true,
                sidebar: "guiSidebar"
              },
              {
                path: '/gui/token-operations',
                component: ComponentCreator('/gui/token-operations', '3f2'),
                exact: true,
                sidebar: "guiSidebar"
              },
              {
                path: '/gui/transactions',
                component: ComponentCreator('/gui/transactions', '6fc'),
                exact: true,
                sidebar: "guiSidebar"
              },
              {
                path: '/gui/wallet-management',
                component: ComponentCreator('/gui/wallet-management', 'e0d'),
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
    component: ComponentCreator('/sdk', 'eeb'),
    routes: [
      {
        path: '/sdk',
        component: ComponentCreator('/sdk', '463'),
        routes: [
          {
            path: '/sdk',
            component: ComponentCreator('/sdk', 'c6c'),
            routes: [
              {
                path: '/sdk/api-reference',
                component: ComponentCreator('/sdk/api-reference', '641'),
                exact: true,
                sidebar: "sdkSidebar"
              },
              {
                path: '/sdk/contracts',
                component: ComponentCreator('/sdk/contracts', '933'),
                exact: true,
                sidebar: "sdkSidebar"
              },
              {
                path: '/sdk/examples',
                component: ComponentCreator('/sdk/examples', 'fd7'),
                exact: true,
                sidebar: "sdkSidebar"
              },
              {
                path: '/sdk/installation',
                component: ComponentCreator('/sdk/installation', '5d4'),
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
                component: ComponentCreator('/sdk/quick-start', 'ef2'),
                exact: true,
                sidebar: "sdkSidebar"
              },
              {
                path: '/sdk/tokens',
                component: ComponentCreator('/sdk/tokens', '3d2'),
                exact: true,
                sidebar: "sdkSidebar"
              },
              {
                path: '/sdk/transactions',
                component: ComponentCreator('/sdk/transactions', '221'),
                exact: true,
                sidebar: "sdkSidebar"
              },
              {
                path: '/sdk/troubleshooting',
                component: ComponentCreator('/sdk/troubleshooting', '760'),
                exact: true,
                sidebar: "sdkSidebar"
              },
              {
                path: '/sdk/wallets',
                component: ComponentCreator('/sdk/wallets', '1e2'),
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
