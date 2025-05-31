import React from 'react';
import ComponentCreator from '@docusaurus/ComponentCreator';

export default [
  {
    path: '/404',
    component: ComponentCreator('/404', 'cfc'),
    exact: true
  },
  {
    path: '/api-reference',
    component: ComponentCreator('/api-reference', 'a8b'),
    exact: true
  },
  {
    path: '/docs/',
    component: ComponentCreator('/docs/', '896'),
    exact: true
  },
  {
    path: '/examples/',
    component: ComponentCreator('/examples/', '628'),
    exact: true
  },
  {
    path: '/guides/',
    component: ComponentCreator('/guides/', '3e0'),
    exact: true
  },
  {
    path: '/playground',
    component: ComponentCreator('/playground', '859'),
    exact: true
  },
  {
    path: '/search',
    component: ComponentCreator('/search', '5de'),
    exact: true
  },
  {
    path: '/cli',
    component: ComponentCreator('/cli', 'f1f'),
    routes: [
      {
        path: '/cli',
        component: ComponentCreator('/cli', '02d'),
        routes: [
          {
            path: '/cli',
            component: ComponentCreator('/cli', '6a9'),
            routes: [
              {
                path: '/cli/commands',
                component: ComponentCreator('/cli/commands', '1c6'),
                exact: true,
                sidebar: "cliSidebar"
              },
              {
                path: '/cli/intro',
                component: ComponentCreator('/cli/intro', 'fd3'),
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
    component: ComponentCreator('/docs', '510'),
    routes: [
      {
        path: '/docs',
        component: ComponentCreator('/docs', 'c48'),
        routes: [
          {
            path: '/docs',
            component: ComponentCreator('/docs', '95e'),
            routes: [
              {
                path: '/docs/getting-started/first-wallet',
                component: ComponentCreator('/docs/getting-started/first-wallet', '341'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/getting-started/installation',
                component: ComponentCreator('/docs/getting-started/installation', '267'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/getting-started/quick-start',
                component: ComponentCreator('/docs/getting-started/quick-start', '09c'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/intro',
                component: ComponentCreator('/docs/intro', '61d'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/testing',
                component: ComponentCreator('/docs/testing', 'f86'),
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
    component: ComponentCreator('/gui', 'f4b'),
    routes: [
      {
        path: '/gui',
        component: ComponentCreator('/gui', '438'),
        routes: [
          {
            path: '/gui',
            component: ComponentCreator('/gui', '8c7'),
            routes: [
              {
                path: '/gui/developer-tools',
                component: ComponentCreator('/gui/developer-tools', 'e64'),
                exact: true,
                sidebar: "guiSidebar"
              },
              {
                path: '/gui/intro',
                component: ComponentCreator('/gui/intro', '0e9'),
                exact: true,
                sidebar: "guiSidebar"
              },
              {
                path: '/gui/nft-operations',
                component: ComponentCreator('/gui/nft-operations', 'bd7'),
                exact: true,
                sidebar: "guiSidebar"
              },
              {
                path: '/gui/wallet-management',
                component: ComponentCreator('/gui/wallet-management', '08f'),
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
    component: ComponentCreator('/sdk', '167'),
    routes: [
      {
        path: '/sdk',
        component: ComponentCreator('/sdk', '7eb'),
        routes: [
          {
            path: '/sdk',
            component: ComponentCreator('/sdk', '447'),
            routes: [
              {
                path: '/sdk/intro',
                component: ComponentCreator('/sdk/intro', 'cf8'),
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
    component: ComponentCreator('/', '2e1'),
    exact: true
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
