// This is a simple search implementation for a static site
// In a production environment, you would use a proper search index

const Fuse = require('fuse.js');
const fs = require('fs');
const path = require('path');

// Production search index - this would be generated from your content
const searchIndex = [
  {
    id: 'getting-started',
    title: 'Getting Started with Neo Rust SDK',
    content: 'Learn how to get started with the Neo Rust SDK. Installation, basic setup, and your first transaction.',
    url: '/docs/getting-started',
    category: 'Documentation',
    tags: ['beginner', 'setup', 'installation']
  },
  {
    id: 'wallet-management',
    title: 'Wallet Management',
    content: 'Create, import, and manage Neo wallets using the Rust SDK. Support for WIF, NEP-6, and hardware wallets.',
    url: '/docs/wallet-management',
    category: 'Documentation',
    tags: ['wallet', 'security', 'keys']
  },
  {
    id: 'smart-contracts',
    title: 'Smart Contract Interaction',
    content: 'Deploy and interact with smart contracts on the Neo blockchain. Contract invocation, deployment, and testing.',
    url: '/docs/smart-contracts',
    category: 'Documentation',
    tags: ['contracts', 'deployment', 'invoke']
  },
  {
    id: 'transactions',
    title: 'Transaction Building',
    content: 'Build, sign, and send transactions on the Neo network. Transaction attributes, signers, and witnesses.',
    url: '/docs/transactions',
    category: 'Documentation',
    tags: ['transactions', 'signing', 'witnesses']
  },
  {
    id: 'rpc-client',
    title: 'RPC Client Usage',
    content: 'Connect to Neo nodes using the RPC client. Query blockchain data, invoke contracts, and monitor events.',
    url: '/docs/rpc-client',
    category: 'Documentation',
    tags: ['rpc', 'client', 'blockchain']
  },
  {
    id: 'neofs-integration',
    title: 'NeoFS Integration',
    content: 'Store and retrieve files using NeoFS. Container management, object operations, and access control.',
    url: '/docs/neofs',
    category: 'Documentation',
    tags: ['neofs', 'storage', 'files']
  },
  {
    id: 'examples-basic',
    title: 'Basic Examples',
    content: 'Simple examples to get you started. Account creation, balance checking, and basic transactions.',
    url: '/examples/basic',
    category: 'Examples',
    tags: ['examples', 'tutorial', 'basic']
  },
  {
    id: 'examples-advanced',
    title: 'Advanced Examples',
    content: 'Advanced usage patterns. Multi-signature transactions, contract deployment, and complex operations.',
    url: '/examples/advanced',
    category: 'Examples',
    tags: ['examples', 'advanced', 'multisig']
  },
  {
    id: 'api-reference',
    title: 'API Reference',
    content: 'Complete API documentation for all modules, structs, and functions in the Neo Rust SDK.',
    url: '/api',
    category: 'Reference',
    tags: ['api', 'reference', 'documentation']
  },
  {
    id: 'troubleshooting',
    title: 'Troubleshooting Guide',
    content: 'Common issues and solutions when working with the Neo Rust SDK. Error handling and debugging tips.',
    url: '/docs/troubleshooting',
    category: 'Documentation',
    tags: ['troubleshooting', 'errors', 'debugging']
  }
];

// Configure Fuse.js for fuzzy search
const fuseOptions = {
  includeScore: true,
  threshold: 0.4, // Lower = more strict matching
  ignoreLocation: true,
  keys: [
    {
      name: 'title',
      weight: 0.4
    },
    {
      name: 'content',
      weight: 0.3
    },
    {
      name: 'tags',
      weight: 0.2
    },
    {
      name: 'category',
      weight: 0.1
    }
  ]
};

const fuse = new Fuse(searchIndex, fuseOptions);

exports.handler = async (event, context) => {
  // Set CORS headers
  const headers = {
    'Access-Control-Allow-Origin': '*',
    'Access-Control-Allow-Headers': 'Content-Type',
    'Access-Control-Allow-Methods': 'GET, POST, OPTIONS',
    'Content-Type': 'application/json'
  };

  // Handle preflight requests
  if (event.httpMethod === 'OPTIONS') {
    return {
      statusCode: 200,
      headers,
      body: ''
    };
  }

  try {
    // Parse query parameters
    const query = event.queryStringParameters?.q || '';
    const category = event.queryStringParameters?.category || '';
    const limit = parseInt(event.queryStringParameters?.limit || '10');
    const offset = parseInt(event.queryStringParameters?.offset || '0');

    // Validate input
    if (!query.trim()) {
      return {
        statusCode: 400,
        headers,
        body: JSON.stringify({
          error: 'Search query is required',
          message: 'Please provide a search query using the "q" parameter'
        })
      };
    }

    if (query.length > 100) {
      return {
        statusCode: 400,
        headers,
        body: JSON.stringify({
          error: 'Query too long',
          message: 'Search query must be 100 characters or less'
        })
      };
    }

    // Perform search
    let results = fuse.search(query);

    // Filter by category if specified
    if (category) {
      results = results.filter(result => 
        result.item.category.toLowerCase() === category.toLowerCase()
      );
    }

    // Apply pagination
    const totalResults = results.length;
    const paginatedResults = results.slice(offset, offset + limit);

    // Format results
    const formattedResults = paginatedResults.map(result => ({
      id: result.item.id,
      title: result.item.title,
      content: result.item.content,
      url: result.item.url,
      category: result.item.category,
      tags: result.item.tags,
      score: Math.round((1 - result.score) * 100), // Convert to percentage relevance
      snippet: generateSnippet(result.item.content, query)
    }));

    // Get available categories for filtering
    const categories = [...new Set(searchIndex.map(item => item.category))];

    // Return results
    return {
      statusCode: 200,
      headers,
      body: JSON.stringify({
        query,
        results: formattedResults,
        pagination: {
          total: totalResults,
          limit,
          offset,
          hasMore: offset + limit < totalResults
        },
        filters: {
          categories,
          selectedCategory: category || null
        },
        suggestions: generateSuggestions(query, results.length === 0)
      })
    };

  } catch (error) {
    console.error('Search error:', error);
    
    return {
      statusCode: 500,
      headers,
      body: JSON.stringify({
        error: 'Internal server error',
        message: 'An error occurred while processing your search request'
      })
    };
  }
};

// Generate content snippet with highlighted search terms
function generateSnippet(content, query, maxLength = 150) {
  const words = query.toLowerCase().split(/\s+/);
  const contentLower = content.toLowerCase();
  
  // Find the best position to start the snippet
  let bestPosition = 0;
  let maxMatches = 0;
  
  for (let i = 0; i <= content.length - maxLength; i += 10) {
    const snippet = content.substr(i, maxLength).toLowerCase();
    const matches = words.reduce((count, word) => {
      return count + (snippet.includes(word) ? 1 : 0);
    }, 0);
    
    if (matches > maxMatches) {
      maxMatches = matches;
      bestPosition = i;
    }
  }
  
  // Extract snippet
  let snippet = content.substr(bestPosition, maxLength);
  
  // Ensure we don't cut words in half
  if (bestPosition > 0) {
    const firstSpace = snippet.indexOf(' ');
    if (firstSpace > 0) {
      snippet = snippet.substr(firstSpace + 1);
    }
    snippet = '...' + snippet;
  }
  
  if (bestPosition + maxLength < content.length) {
    const lastSpace = snippet.lastIndexOf(' ');
    if (lastSpace > 0) {
      snippet = snippet.substr(0, lastSpace);
    }
    snippet = snippet + '...';
  }
  
  return snippet;
}

// Generate search suggestions
function generateSuggestions(query, noResults) {
  const suggestions = [];
  
  if (noResults) {
    // Suggest similar terms
    const commonTerms = [
      'wallet', 'transaction', 'contract', 'rpc', 'account', 
      'signature', 'blockchain', 'neofs', 'examples', 'api'
    ];
    
    const queryLower = query.toLowerCase();
    const similar = commonTerms.filter(term => 
      term.includes(queryLower) || queryLower.includes(term)
    );
    
    if (similar.length > 0) {
      suggestions.push(...similar.slice(0, 3));
    } else {
      suggestions.push('getting started', 'wallet management', 'smart contracts');
    }
  }
  
  return suggestions;
}