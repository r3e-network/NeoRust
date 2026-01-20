import React from 'react';
import ComponentCreator from '@docusaurus/ComponentCreator';

export default [
  {
    path: '/blog',
    component: ComponentCreator('/blog', '606'),
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
    path: '/blog/neorust-v1.0.1-docs-quality',
    component: ComponentCreator('/blog/neorust-v1.0.1-docs-quality', '403'),
    exact: true
  },
  {
    path: '/blog/neorust-v1.0.1-release',
    component: ComponentCreator('/blog/neorust-v1.0.1-release', '244'),
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
    component: ComponentCreator('/blog/tags/blockchain', '341'),
    exact: true
  },
  {
    path: '/blog/tags/dapp',
    component: ComponentCreator('/blog/tags/dapp', '603'),
    exact: true
  },
  {
    path: '/blog/tags/documentation',
    component: ComponentCreator('/blog/tags/documentation', 'ea2'),
    exact: true
  },
  {
    path: '/blog/tags/neo-3',
    component: ComponentCreator('/blog/tags/neo-3', 'a4f'),
    exact: true
  },
  {
    path: '/blog/tags/quality',
    component: ComponentCreator('/blog/tags/quality', 'c5e'),
    exact: true
  },
  {
    path: '/blog/tags/release',
    component: ComponentCreator('/blog/tags/release', '699'),
    exact: true
  },
  {
    path: '/blog/tags/rust',
    component: ComponentCreator('/blog/tags/rust', '477'),
    exact: true
  },
  {
    path: '/blog/tags/sdk',
    component: ComponentCreator('/blog/tags/sdk', 'd18'),
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
    component: ComponentCreator('/cli', '901'),
    routes: [
      {
        path: '/cli',
        component: ComponentCreator('/cli', '79b'),
        routes: [
          {
            path: '/cli',
            component: ComponentCreator('/cli', '05d'),
            routes: [
              {
                path: '/cli/commands',
                component: ComponentCreator('/cli/commands', 'c46'),
                exact: true,
                sidebar: "cliSidebar"
              },
              {
                path: '/cli/configuration',
                component: ComponentCreator('/cli/configuration', '73d'),
                exact: true,
                sidebar: "cliSidebar"
              },
              {
                path: '/cli/intro',
                component: ComponentCreator('/cli/intro', '1eb'),
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
    component: ComponentCreator('/docs', 'edf'),
    routes: [
      {
        path: '/docs',
        component: ComponentCreator('/docs', '6ed'),
        routes: [
          {
            path: '/docs',
            component: ComponentCreator('/docs', 'c1d'),
            routes: [
              {
                path: '/docs/getting-started/installation',
                component: ComponentCreator('/docs/getting-started/installation', 'cfa'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/getting-started/quick-start',
                component: ComponentCreator('/docs/getting-started/quick-start', 'e99'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/intro',
                component: ComponentCreator('/docs/intro', '36e'),
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
    path: '/sdk',
    component: ComponentCreator('/sdk', '315'),
    routes: [
      {
        path: '/sdk',
        component: ComponentCreator('/sdk', 'd6d'),
        routes: [
          {
            path: '/sdk',
            component: ComponentCreator('/sdk', 'df2'),
            routes: [
              {
                path: '/sdk/api-reference',
                component: ComponentCreator('/sdk/api-reference', 'dcc'),
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
                component: ComponentCreator('/sdk/installation', '590'),
                exact: true,
                sidebar: "sdkSidebar"
              },
              {
                path: '/sdk/intro',
                component: ComponentCreator('/sdk/intro', '4b1'),
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
