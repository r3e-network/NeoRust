import React, { useState } from 'react';
import {
  WrenchScrewdriverIcon,
  CodeBracketIcon,
  DocumentTextIcon,
  ArrowsRightLeftIcon,
  KeyIcon,
  ClipboardDocumentIcon,
  CheckCircleIcon,
  ExclamationTriangleIcon,
  CubeIcon,
  LinkIcon,
  CalculatorIcon,
  ShieldCheckIcon,
  GlobeAltIcon,
  CommandLineIcon,
} from '@heroicons/react/24/outline';
import { useAppStore } from '../stores/appStore';
import { invoke } from '@tauri-apps/api/core';

type ToolCategory =
  | 'address'
  | 'script'
  | 'contract'
  | 'transaction'
  | 'crypto'
  | 'network';

interface Tool {
  id: string;
  name: string;
  description: string;
  category: ToolCategory;
  icon: React.ReactNode;
  component: React.ComponentType<any>;
}

export default function Tools() {
  const { addNotification } = useAppStore();
  const [selectedCategory, setSelectedCategory] =
    useState<ToolCategory>('address');
  const [selectedTool, setSelectedTool] = useState<string | null>(null);

  const categories = [
    {
      id: 'address' as ToolCategory,
      name: 'Address Tools',
      icon: <KeyIcon className='h-5 w-5' />,
    },
    {
      id: 'script' as ToolCategory,
      name: 'Script Builder',
      icon: <CodeBracketIcon className='h-5 w-5' />,
    },
    {
      id: 'contract' as ToolCategory,
      name: 'Contract Tools',
      icon: <DocumentTextIcon className='h-5 w-5' />,
    },
    {
      id: 'transaction' as ToolCategory,
      name: 'Transaction Tools',
      icon: <ArrowsRightLeftIcon className='h-5 w-5' />,
    },
    {
      id: 'crypto' as ToolCategory,
      name: 'Cryptography',
      icon: <ShieldCheckIcon className='h-5 w-5' />,
    },
    {
      id: 'network' as ToolCategory,
      name: 'Network Tools',
      icon: <GlobeAltIcon className='h-5 w-5' />,
    },
  ];

  const tools: Tool[] = [
    // Address Tools
    {
      id: 'address-converter',
      name: 'Address Converter',
      description: 'Convert between different Neo address formats',
      category: 'address',
      icon: <ArrowsRightLeftIcon className='h-5 w-5' />,
      component: AddressConverter,
    },
    {
      id: 'address-validator',
      name: 'Address Validator',
      description: 'Validate Neo N3 addresses',
      category: 'address',
      icon: <CheckCircleIcon className='h-5 w-5' />,
      component: AddressValidator,
    },
    {
      id: 'key-generator',
      name: 'Key Generator',
      description: 'Generate new private/public key pairs',
      category: 'address',
      icon: <KeyIcon className='h-5 w-5' />,
      component: KeyGenerator,
    },

    // Script Builder
    {
      id: 'script-builder',
      name: 'Script Builder',
      description: 'Build and test Neo VM scripts',
      category: 'script',
      icon: <CodeBracketIcon className='h-5 w-5' />,
      component: ScriptBuilder,
    },
    {
      id: 'opcode-reference',
      name: 'OpCode Reference',
      description: 'Neo VM OpCode documentation and examples',
      category: 'script',
      icon: <DocumentTextIcon className='h-5 w-5' />,
      component: OpCodeReference,
    },

    // Contract Tools
    {
      id: 'contract-inspector',
      name: 'Contract Inspector',
      description: 'Inspect deployed smart contracts',
      category: 'contract',
      icon: <CubeIcon className='h-5 w-5' />,
      component: ContractInspector,
    },
    {
      id: 'abi-decoder',
      name: 'ABI Decoder',
      description: 'Decode contract ABI and method calls',
      category: 'contract',
      icon: <DocumentTextIcon className='h-5 w-5' />,
      component: ABIDecoder,
    },

    // Transaction Tools
    {
      id: 'tx-builder',
      name: 'Transaction Builder',
      description: 'Build and sign custom transactions',
      category: 'transaction',
      icon: <WrenchScrewdriverIcon className='h-5 w-5' />,
      component: TransactionBuilder,
    },
    {
      id: 'tx-decoder',
      name: 'Transaction Decoder',
      description: 'Decode and analyze transactions',
      category: 'transaction',
      icon: <DocumentTextIcon className='h-5 w-5' />,
      component: TransactionDecoder,
    },
    {
      id: 'fee-calculator',
      name: 'Fee Calculator',
      description: 'Calculate transaction fees',
      category: 'transaction',
      icon: <CalculatorIcon className='h-5 w-5' />,
      component: FeeCalculator,
    },

    // Cryptography
    {
      id: 'hash-calculator',
      name: 'Hash Calculator',
      description: 'Calculate various hash functions',
      category: 'crypto',
      icon: <ShieldCheckIcon className='h-5 w-5' />,
      component: HashCalculator,
    },
    {
      id: 'signature-verifier',
      name: 'Signature Verifier',
      description: 'Verify digital signatures',
      category: 'crypto',
      icon: <CheckCircleIcon className='h-5 w-5' />,
      component: SignatureVerifier,
    },

    // Network Tools
    {
      id: 'rpc-client',
      name: 'RPC Client',
      description: 'Test Neo RPC endpoints',
      category: 'network',
      icon: <LinkIcon className='h-5 w-5' />,
      component: RPCClient,
    },
    {
      id: 'block-explorer',
      name: 'Block Explorer',
      description: 'Explore blocks and transactions',
      category: 'network',
      icon: <CubeIcon className='h-5 w-5' />,
      component: BlockExplorer,
    },
  ];

  const filteredTools = tools.filter(
    tool => tool.category === selectedCategory
  );
  const currentTool = selectedTool
    ? tools.find(t => t.id === selectedTool)
    : null;

  return (
    <div className='space-y-6'>
      {/* Header */}
      <div className='md:flex md:items-center md:justify-between'>
        <div className='flex-1 min-w-0'>
          <h2 className='text-2xl font-bold leading-7 text-gray-900 sm:text-3xl sm:truncate'>
            Neo N3 Tools
          </h2>
          <p className='mt-1 text-sm text-gray-500'>
            Comprehensive toolkit for Neo N3 development and blockchain
            interaction
          </p>
        </div>
      </div>

      <div className='grid grid-cols-1 lg:grid-cols-4 gap-6'>
        {/* Categories Sidebar */}
        <div className='lg:col-span-1'>
          <div className='bg-white shadow rounded-lg p-4'>
            <h3 className='text-lg font-medium text-gray-900 mb-4'>
              Tool Categories
            </h3>
            <nav className='space-y-2'>
              {categories.map(category => (
                <button
                  key={category.id}
                  onClick={() => {
                    setSelectedCategory(category.id);
                    setSelectedTool(null);
                  }}
                  className={`w-full flex items-center space-x-3 px-3 py-2 text-sm font-medium rounded-md transition-colors ${
                    selectedCategory === category.id
                      ? 'bg-green-100 text-green-700'
                      : 'text-gray-600 hover:bg-gray-50 hover:text-gray-900'
                  }`}
                >
                  {category.icon}
                  <span>{category.name}</span>
                </button>
              ))}
            </nav>
          </div>
        </div>

        {/* Tools List */}
        <div className='lg:col-span-1'>
          <div className='bg-white shadow rounded-lg p-4'>
            <h3 className='text-lg font-medium text-gray-900 mb-4'>
              {categories.find(c => c.id === selectedCategory)?.name}
            </h3>
            <div className='space-y-2'>
              {filteredTools.map(tool => (
                <button
                  key={tool.id}
                  onClick={() => setSelectedTool(tool.id)}
                  className={`w-full text-left p-3 rounded-lg border transition-colors ${
                    selectedTool === tool.id
                      ? 'border-green-500 bg-green-50'
                      : 'border-gray-200 hover:border-gray-300 hover:bg-gray-50'
                  }`}
                >
                  <div className='flex items-center space-x-3 mb-2'>
                    {tool.icon}
                    <span className='font-medium text-gray-900'>
                      {tool.name}
                    </span>
                  </div>
                  <p className='text-sm text-gray-500'>{tool.description}</p>
                </button>
              ))}
            </div>
          </div>
        </div>

        {/* Tool Content */}
        <div className='lg:col-span-2'>
          <div className='bg-white shadow rounded-lg p-6'>
            {currentTool ? (
              <div>
                <div className='flex items-center space-x-3 mb-6'>
                  {currentTool.icon}
                  <div>
                    <h3 className='text-lg font-medium text-gray-900'>
                      {currentTool.name}
                    </h3>
                    <p className='text-sm text-gray-500'>
                      {currentTool.description}
                    </p>
                  </div>
                </div>
                <currentTool.component addNotification={addNotification} />
              </div>
            ) : (
              <div className='text-center py-12'>
                <WrenchScrewdriverIcon className='mx-auto h-12 w-12 text-gray-400' />
                <h3 className='mt-2 text-sm font-medium text-gray-900'>
                  Select a Tool
                </h3>
                <p className='mt-1 text-sm text-gray-500'>
                  Choose a tool from the list to get started
                </p>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

// Tool Components

function AddressConverter({ addNotification }: { addNotification: any }) {
  const [input, setInput] = useState('');
  const [output, setOutput] = useState('');
  const [inputFormat, setInputFormat] = useState('address');
  const [outputFormat, setOutputFormat] = useState('scripthash');

  const handleConvert = async () => {
    if (!input.trim()) return;

    try {
      const result = await invoke('convert_address', {
        input: input.trim(),
        inputFormat,
        outputFormat,
      });
      setOutput(result as string);
    } catch (error) {
      addNotification({
        type: 'error',
        title: 'Conversion Failed',
        message: 'Invalid input or conversion not supported',
      });
    }
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
    addNotification({
      type: 'success',
      title: 'Copied',
      message: 'Result copied to clipboard',
    });
  };

  return (
    <div className='space-y-4'>
      <div className='grid grid-cols-2 gap-4'>
        <div>
          <label className='block text-sm font-medium text-gray-700 mb-2'>
            Input Format
          </label>
          <select
            value={inputFormat}
            onChange={e => setInputFormat(e.target.value)}
            className='w-full border border-gray-300 rounded-md px-3 py-2 focus:ring-green-500 focus:border-green-500'
          >
            <option value='address'>Address</option>
            <option value='scripthash'>Script Hash</option>
            <option value='publickey'>Public Key</option>
          </select>
        </div>
        <div>
          <label className='block text-sm font-medium text-gray-700 mb-2'>
            Output Format
          </label>
          <select
            value={outputFormat}
            onChange={e => setOutputFormat(e.target.value)}
            className='w-full border border-gray-300 rounded-md px-3 py-2 focus:ring-green-500 focus:border-green-500'
          >
            <option value='address'>Address</option>
            <option value='scripthash'>Script Hash</option>
            <option value='publickey'>Public Key</option>
          </select>
        </div>
      </div>

      <div>
        <label className='block text-sm font-medium text-gray-700 mb-2'>
          Input
        </label>
        <textarea
          value={input}
          onChange={e => setInput(e.target.value)}
          placeholder='Enter address, script hash, or public key...'
          className='w-full border border-gray-300 rounded-md px-3 py-2 focus:ring-green-500 focus:border-green-500'
          rows={3}
        />
      </div>

      <button
        onClick={handleConvert}
        className='w-full bg-green-600 text-white px-4 py-2 rounded-md hover:bg-green-700'
      >
        Convert
      </button>

      {output && (
        <div>
          <label className='block text-sm font-medium text-gray-700 mb-2'>
            Result
          </label>
          <div className='relative'>
            <textarea
              value={output}
              readOnly
              className='w-full border border-gray-300 rounded-md px-3 py-2 bg-gray-50'
              rows={3}
            />
            <button
              onClick={() => copyToClipboard(output)}
              className='absolute top-2 right-2 p-1 text-gray-400 hover:text-gray-600'
            >
              <ClipboardDocumentIcon className='h-4 w-4' />
            </button>
          </div>
        </div>
      )}
    </div>
  );
}

function AddressValidator({ addNotification }: { addNotification: any }) {
  const [address, setAddress] = useState('');
  const [result, setResult] = useState<{
    valid: boolean;
    type?: string;
    message?: string;
  } | null>(null);

  const handleValidate = async () => {
    if (!address.trim()) {
      addNotification({
        type: 'error',
        title: 'Validation Error',
        message: 'Please enter an address to validate',
      });
      return;
    }

    try {
      const validation = await invoke('validate_address', {
        address: address.trim(),
      });
      setResult(validation as any);
    } catch (error) {
      setResult({
        valid: false,
        message: 'Validation failed',
      });
      addNotification({
        type: 'error',
        title: 'Validation Failed',
        message:
          'Unable to validate the address. Please check the format and try again.',
      });
    }
  };

  return (
    <div className='space-y-4'>
      <div>
        <label className='block text-sm font-medium text-gray-700 mb-2'>
          Neo Address
        </label>
        <input
          type='text'
          value={address}
          onChange={e => setAddress(e.target.value)}
          placeholder='Enter Neo N3 address...'
          className='w-full border border-gray-300 rounded-md px-3 py-2 focus:ring-green-500 focus:border-green-500'
        />
      </div>

      <button
        onClick={handleValidate}
        className='w-full bg-green-600 text-white px-4 py-2 rounded-md hover:bg-green-700'
      >
        Validate Address
      </button>

      {result && (
        <div
          className={`p-4 rounded-md ${result.valid ? 'bg-green-50 border border-green-200' : 'bg-red-50 border border-red-200'}`}
        >
          <div className='flex items-center space-x-2'>
            {result.valid ? (
              <CheckCircleIcon className='h-5 w-5 text-green-600' />
            ) : (
              <ExclamationTriangleIcon className='h-5 w-5 text-red-600' />
            )}
            <span
              className={`font-medium ${result.valid ? 'text-green-800' : 'text-red-800'}`}
            >
              {result.valid ? 'Valid Address' : 'Invalid Address'}
            </span>
          </div>
          {result.type && (
            <p className='mt-1 text-sm text-gray-600'>Type: {result.type}</p>
          )}
          {result.message && (
            <p className='mt-1 text-sm text-gray-600'>{result.message}</p>
          )}
        </div>
      )}
    </div>
  );
}

function KeyGenerator({ addNotification }: { addNotification: any }) {
  const [keyPair, setKeyPair] = useState<{
    privateKey: string;
    publicKey: string;
    address: string;
  } | null>(null);
  const [generating, setGenerating] = useState(false);

  const generateKeyPair = async () => {
    setGenerating(true);
    try {
      const result = await invoke('generate_key_pair');
      setKeyPair(result as any);
    } catch (error) {
      addNotification({
        type: 'error',
        title: 'Generation Failed',
        message: 'Failed to generate key pair',
      });
    } finally {
      setGenerating(false);
    }
  };

  const copyToClipboard = (text: string, label: string) => {
    navigator.clipboard.writeText(text);
    addNotification({
      type: 'success',
      title: 'Copied',
      message: `${label} copied to clipboard`,
    });
  };

  return (
    <div className='space-y-4'>
      <div className='bg-yellow-50 border border-yellow-200 rounded-md p-4'>
        <div className='flex items-center space-x-2'>
          <ExclamationTriangleIcon className='h-5 w-5 text-yellow-600' />
          <span className='font-medium text-yellow-800'>Security Warning</span>
        </div>
        <p className='mt-1 text-sm text-yellow-700'>
          Never share your private key. Store it securely and use it only in
          trusted environments.
        </p>
      </div>

      <button
        onClick={generateKeyPair}
        disabled={generating}
        className='w-full bg-green-600 text-white px-4 py-2 rounded-md hover:bg-green-700 disabled:opacity-50'
      >
        {generating ? 'Generating...' : 'Generate New Key Pair'}
      </button>

      {keyPair && (
        <div className='space-y-4'>
          <div>
            <div className='flex items-center justify-between mb-2'>
              <label className='block text-sm font-medium text-gray-700'>
                Private Key
              </label>
              <button
                onClick={() =>
                  copyToClipboard(keyPair.privateKey, 'Private key')
                }
                className='text-sm text-green-600 hover:text-green-500'
              >
                Copy
              </button>
            </div>
            <input
              type='text'
              value={keyPair.privateKey}
              readOnly
              className='w-full border border-gray-300 rounded-md px-3 py-2 bg-gray-50 font-mono text-sm'
            />
          </div>

          <div>
            <div className='flex items-center justify-between mb-2'>
              <label className='block text-sm font-medium text-gray-700'>
                Public Key
              </label>
              <button
                onClick={() => copyToClipboard(keyPair.publicKey, 'Public key')}
                className='text-sm text-green-600 hover:text-green-500'
              >
                Copy
              </button>
            </div>
            <input
              type='text'
              value={keyPair.publicKey}
              readOnly
              className='w-full border border-gray-300 rounded-md px-3 py-2 bg-gray-50 font-mono text-sm'
            />
          </div>

          <div>
            <div className='flex items-center justify-between mb-2'>
              <label className='block text-sm font-medium text-gray-700'>
                Address
              </label>
              <button
                onClick={() => copyToClipboard(keyPair.address, 'Address')}
                className='text-sm text-green-600 hover:text-green-500'
              >
                Copy
              </button>
            </div>
            <input
              type='text'
              value={keyPair.address}
              readOnly
              className='w-full border border-gray-300 rounded-md px-3 py-2 bg-gray-50 font-mono text-sm'
            />
          </div>
        </div>
      )}
    </div>
  );
}

// Placeholder components for other tools
function ScriptBuilder() {
  return (
    <div className='text-center py-8'>
      <CommandLineIcon className='mx-auto h-12 w-12 text-gray-400' />
      <h3 className='mt-2 text-sm font-medium text-gray-900'>Script Builder</h3>
      <p className='mt-1 text-sm text-gray-500'>
        Advanced script building interface coming soon
      </p>
    </div>
  );
}

function OpCodeReference() {
  return (
    <div className='text-center py-8'>
      <DocumentTextIcon className='mx-auto h-12 w-12 text-gray-400' />
      <h3 className='mt-2 text-sm font-medium text-gray-900'>
        OpCode Reference
      </h3>
      <p className='mt-1 text-sm text-gray-500'>
        Neo VM OpCode documentation and examples
      </p>
    </div>
  );
}

function ContractInspector() {
  return (
    <div className='text-center py-8'>
      <CubeIcon className='mx-auto h-12 w-12 text-gray-400' />
      <h3 className='mt-2 text-sm font-medium text-gray-900'>
        Contract Inspector
      </h3>
      <p className='mt-1 text-sm text-gray-500'>
        Smart contract inspection tools
      </p>
    </div>
  );
}

function ABIDecoder() {
  return (
    <div className='text-center py-8'>
      <DocumentTextIcon className='mx-auto h-12 w-12 text-gray-400' />
      <h3 className='mt-2 text-sm font-medium text-gray-900'>ABI Decoder</h3>
      <p className='mt-1 text-sm text-gray-500'>
        Contract ABI decoding utilities
      </p>
    </div>
  );
}

function TransactionBuilder() {
  return (
    <div className='text-center py-8'>
      <WrenchScrewdriverIcon className='mx-auto h-12 w-12 text-gray-400' />
      <h3 className='mt-2 text-sm font-medium text-gray-900'>
        Transaction Builder
      </h3>
      <p className='mt-1 text-sm text-gray-500'>
        Custom transaction building interface
      </p>
    </div>
  );
}

function TransactionDecoder() {
  return (
    <div className='text-center py-8'>
      <DocumentTextIcon className='mx-auto h-12 w-12 text-gray-400' />
      <h3 className='mt-2 text-sm font-medium text-gray-900'>
        Transaction Decoder
      </h3>
      <p className='mt-1 text-sm text-gray-500'>
        Transaction analysis and decoding
      </p>
    </div>
  );
}

function FeeCalculator() {
  return (
    <div className='text-center py-8'>
      <CalculatorIcon className='mx-auto h-12 w-12 text-gray-400' />
      <h3 className='mt-2 text-sm font-medium text-gray-900'>Fee Calculator</h3>
      <p className='mt-1 text-sm text-gray-500'>
        Transaction fee estimation tools
      </p>
    </div>
  );
}

function HashCalculator() {
  return (
    <div className='text-center py-8'>
      <ShieldCheckIcon className='mx-auto h-12 w-12 text-gray-400' />
      <h3 className='mt-2 text-sm font-medium text-gray-900'>
        Hash Calculator
      </h3>
      <p className='mt-1 text-sm text-gray-500'>
        Cryptographic hash function utilities
      </p>
    </div>
  );
}

function SignatureVerifier() {
  return (
    <div className='text-center py-8'>
      <CheckCircleIcon className='mx-auto h-12 w-12 text-gray-400' />
      <h3 className='mt-2 text-sm font-medium text-gray-900'>
        Signature Verifier
      </h3>
      <p className='mt-1 text-sm text-gray-500'>
        Digital signature verification tools
      </p>
    </div>
  );
}

function RPCClient() {
  return (
    <div className='text-center py-8'>
      <LinkIcon className='mx-auto h-12 w-12 text-gray-400' />
      <h3 className='mt-2 text-sm font-medium text-gray-900'>RPC Client</h3>
      <p className='mt-1 text-sm text-gray-500'>
        Neo RPC endpoint testing interface
      </p>
    </div>
  );
}

function BlockExplorer() {
  return (
    <div className='text-center py-8'>
      <CubeIcon className='mx-auto h-12 w-12 text-gray-400' />
      <h3 className='mt-2 text-sm font-medium text-gray-900'>Block Explorer</h3>
      <p className='mt-1 text-sm text-gray-500'>
        Blockchain exploration interface
      </p>
    </div>
  );
}
