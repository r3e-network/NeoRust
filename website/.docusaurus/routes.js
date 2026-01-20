import React from 'react';
import ComponentCreator from '@docusaurus/ComponentCreator';

export default [
  {
    path: '/blog',
    component: ComponentCreator('/blog', 'e99'),
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
    path: '/blog/neorust-v1.0.0-docs-quality',
    component: ComponentCreator('/blog/neorust-v1.0.0-docs-quality', '774'),
    exact: true
  },
  {
    path: '/blog/neorust-v1.0.0-release',
    component: ComponentCreator('/blog/neorust-v1.0.0-release', '4f1'),
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
    component: ComponentCreator('/blog/tags/blockchain', '6da'),
    exact: true
  },
  {
    path: '/blog/tags/dapp',
    component: ComponentCreator('/blog/tags/dapp', '603'),
    exact: true
  },
  {
    path: '/blog/tags/documentation',
    component: ComponentCreator('/blog/tags/documentation', '606'),
    exact: true
  },
  {
    path: '/blog/tags/neo-3',
    component: ComponentCreator('/blog/tags/neo-3', '934'),
    exact: true
  },
  {
    path: '/blog/tags/quality',
    component: ComponentCreator('/blog/tags/quality', '116'),
    exact: true
  },
  {
    path: '/blog/tags/release',
    component: ComponentCreator('/blog/tags/release', 'c46'),
    exact: true
  },
  {
    path: '/blog/tags/rust',
    component: ComponentCreator('/blog/tags/rust', 'e98'),
    exact: true
  },
  {
    path: '/blog/tags/sdk',
    component: ComponentCreator('/blog/tags/sdk', '56d'),
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
    component: ComponentCreator('/cli', '4c1'),
    routes: [
      {
        path: '/cli',
        component: ComponentCreator('/cli', '242'),
        routes: [
          {
            path: '/cli',
            component: ComponentCreator('/cli', 'f76'),
            routes: [
              {
                path: '/cli/commands',
                component: ComponentCreator('/cli/commands', 'c46'),
                exact: true,
                sidebar: "cliSidebar"
              },
              {
                path: '/cli/configuration',
                component: ComponentCreator('/cli/configuration', '51b'),
                exact: true,
                sidebar: "cliSidebar"
              },
              {
                path: '/cli/intro',
                component: ComponentCreator('/cli/intro', '936'),
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
    component: ComponentCreator('/docs', 'a4b'),
    routes: [
      {
        path: '/docs',
        component: ComponentCreator('/docs', 'b3f'),
        routes: [
          {
            path: '/docs',
            component: ComponentCreator('/docs', 'dbd'),
            routes: [
              {
                path: '/docs/getting-started/installation',
                component: ComponentCreator('/docs/getting-started/installation', 'b95'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/getting-started/quick-start',
                component: ComponentCreator('/docs/getting-started/quick-start', 'dc1'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/intro',
                component: ComponentCreator('/docs/intro', 'a79'),
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
    component: ComponentCreator('/sdk', '5bc'),
    routes: [
      {
        path: '/sdk',
        component: ComponentCreator('/sdk', '62c'),
        routes: [
          {
            path: '/sdk',
            component: ComponentCreator('/sdk', '7b7'),
            routes: [
              {
                path: '/sdk/api-reference',
                component: ComponentCreator('/sdk/api-reference', 'b8c'),
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
                component: ComponentCreator('/sdk/installation', 'f58'),
                exact: true,
                sidebar: "sdkSidebar"
              },
              {
                path: '/sdk/intro',
                component: ComponentCreator('/sdk/intro', '1b9'),
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
