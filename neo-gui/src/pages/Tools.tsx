import React, { useState } from 'react';
import { motion } from 'framer-motion';
import {
  WrenchScrewdriverIcon,
  DocumentDuplicateIcon,
  CheckIcon,
} from '@heroicons/react/24/outline';

export default function Tools() {
  const [activeTab, setActiveTab] = useState('encode');
  const [input, setInput] = useState('');
  const [output, setOutput] = useState('');
  const [copied, setCopied] = useState(false);

  const tabs = [
    { id: 'encode', name: 'Encode/Decode', icon: WrenchScrewdriverIcon },
    { id: 'hash', name: 'Hash Functions', icon: WrenchScrewdriverIcon },
    { id: 'address', name: 'Address Utils', icon: WrenchScrewdriverIcon },
    { id: 'transaction', name: 'Transaction Tools', icon: WrenchScrewdriverIcon },
  ];

  const handleEncode = (type: string) => {
    try {
      switch (type) {
        case 'base64':
          setOutput(btoa(input));
          break;
        case 'hex':
          setOutput(Array.from(new TextEncoder().encode(input))
            .map(b => b.toString(16).padStart(2, '0')).join(''));
          break;
        default:
          setOutput('');
      }
    } catch (error) {
      setOutput('Error: Invalid input');
    }
  };

  const handleDecode = (type: string) => {
    try {
      switch (type) {
        case 'base64':
          setOutput(atob(input));
          break;
        case 'hex':
          const bytes = input.match(/.{1,2}/g)?.map(byte => parseInt(byte, 16)) || [];
          setOutput(new TextDecoder().decode(new Uint8Array(bytes)));
          break;
        default:
          setOutput('');
      }
    } catch (error) {
      setOutput('Error: Invalid input');
    }
  };

  const copyToClipboard = () => {
    navigator.clipboard.writeText(output);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="md:flex md:items-center md:justify-between">
        <div className="flex-1 min-w-0">
          <h2 className="text-2xl font-bold leading-7 text-gray-900 sm:text-3xl sm:truncate">
            Developer Tools
          </h2>
          <p className="mt-1 text-sm text-gray-500">
            Utilities for encoding, hashing, and blockchain development.
          </p>
        </div>
      </div>

      {/* Tabs */}
      <div className="border-b border-gray-200">
        <nav className="-mb-px flex space-x-8">
          {tabs.map((tab) => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id)}
              className={`py-2 px-1 border-b-2 font-medium text-sm ${
                activeTab === tab.id
                  ? 'border-green-500 text-green-600'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
              }`}
            >
              <div className="flex items-center">
                <tab.icon className="h-5 w-5 mr-2" />
                {tab.name}
              </div>
            </button>
          ))}
        </nav>
      </div>

      {/* Content */}
      <motion.div
        key={activeTab}
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.3 }}
        className="bg-white shadow rounded-lg p-6"
      >
        {activeTab === 'encode' && (
          <div className="space-y-6">
            <h3 className="text-lg font-medium text-gray-900">Encode/Decode Data</h3>
            
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Input
                </label>
                <textarea
                  value={input}
                  onChange={(e) => setInput(e.target.value)}
                  rows={6}
                  className="block w-full border-gray-300 rounded-md shadow-sm focus:ring-green-500 focus:border-green-500"
                  placeholder="Enter text to encode/decode..."
                />
                
                <div className="mt-4 flex flex-wrap gap-2">
                  <button
                    onClick={() => handleEncode('base64')}
                    className="px-3 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 bg-white hover:bg-gray-50"
                  >
                    Encode Base64
                  </button>
                  <button
                    onClick={() => handleDecode('base64')}
                    className="px-3 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 bg-white hover:bg-gray-50"
                  >
                    Decode Base64
                  </button>
                  <button
                    onClick={() => handleEncode('hex')}
                    className="px-3 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 bg-white hover:bg-gray-50"
                  >
                    Encode Hex
                  </button>
                  <button
                    onClick={() => handleDecode('hex')}
                    className="px-3 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 bg-white hover:bg-gray-50"
                  >
                    Decode Hex
                  </button>
                </div>
              </div>
              
              <div>
                <div className="flex items-center justify-between mb-2">
                  <label className="block text-sm font-medium text-gray-700">
                    Output
                  </label>
                  {output && (
                    <button
                      onClick={copyToClipboard}
                      className="flex items-center text-sm text-green-600 hover:text-green-500"
                    >
                      {copied ? (
                        <>
                          <CheckIcon className="h-4 w-4 mr-1" />
                          Copied!
                        </>
                      ) : (
                        <>
                          <DocumentDuplicateIcon className="h-4 w-4 mr-1" />
                          Copy
                        </>
                      )}
                    </button>
                  )}
                </div>
                <textarea
                  value={output}
                  readOnly
                  rows={6}
                  className="block w-full border-gray-300 rounded-md shadow-sm bg-gray-50 font-mono text-sm"
                  placeholder="Output will appear here..."
                />
              </div>
            </div>
          </div>
        )}

        {activeTab === 'hash' && (
          <div className="space-y-6">
            <h3 className="text-lg font-medium text-gray-900">Hash Functions</h3>
            <p className="text-sm text-gray-500">
              Generate cryptographic hashes for your data.
            </p>
            
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Input Data
                </label>
                <textarea
                  value={input}
                  onChange={(e) => setInput(e.target.value)}
                  rows={4}
                  className="block w-full border-gray-300 rounded-md shadow-sm focus:ring-green-500 focus:border-green-500"
                  placeholder="Enter data to hash..."
                />
                
                <div className="mt-4 grid grid-cols-2 gap-2">
                  <button className="px-3 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 bg-white hover:bg-gray-50">
                    SHA256
                  </button>
                  <button className="px-3 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 bg-white hover:bg-gray-50">
                    RIPEMD160
                  </button>
                  <button className="px-3 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 bg-white hover:bg-gray-50">
                    Keccak256
                  </button>
                  <button className="px-3 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 bg-white hover:bg-gray-50">
                    Blake2b
                  </button>
                </div>
              </div>
              
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Hash Output
                </label>
                <div className="space-y-3">
                  <div className="p-3 bg-gray-50 rounded-md">
                    <div className="text-xs text-gray-500 mb-1">SHA256</div>
                    <div className="font-mono text-sm break-all">
                      {input ? 'Click SHA256 to generate hash' : 'Enter input data'}
                    </div>
                  </div>
                  <div className="p-3 bg-gray-50 rounded-md">
                    <div className="text-xs text-gray-500 mb-1">RIPEMD160</div>
                    <div className="font-mono text-sm break-all">
                      {input ? 'Click RIPEMD160 to generate hash' : 'Enter input data'}
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {activeTab === 'address' && (
          <div className="space-y-6">
            <h3 className="text-lg font-medium text-gray-900">Address Utilities</h3>
            <p className="text-sm text-gray-500">
              Validate and convert Neo addresses and public keys.
            </p>
            
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Address/Public Key
                </label>
                <input
                  type="text"
                  value={input}
                  onChange={(e) => setInput(e.target.value)}
                  className="block w-full border-gray-300 rounded-md shadow-sm focus:ring-green-500 focus:border-green-500"
                  placeholder="Enter Neo address or public key..."
                />
              </div>
              
              <div className="flex flex-wrap gap-2">
                <button className="px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700">
                  Validate Address
                </button>
                <button className="px-4 py-2 border border-gray-300 rounded-md text-gray-700 bg-white hover:bg-gray-50">
                  Convert to Script Hash
                </button>
                <button className="px-4 py-2 border border-gray-300 rounded-md text-gray-700 bg-white hover:bg-gray-50">
                  Generate from Public Key
                </button>
              </div>
              
              <div className="mt-6 p-4 bg-gray-50 rounded-md">
                <h4 className="text-sm font-medium text-gray-900 mb-2">Results</h4>
                <div className="space-y-2 text-sm">
                  <div><span className="font-medium">Valid:</span> <span className="text-green-600">Yes</span></div>
                  <div><span className="font-medium">Type:</span> Neo N3 Address</div>
                  <div><span className="font-medium">Script Hash:</span> <span className="font-mono">0x1234...abcd</span></div>
                </div>
              </div>
            </div>
          </div>
        )}

        {activeTab === 'transaction' && (
          <div className="space-y-6">
            <h3 className="text-lg font-medium text-gray-900">Transaction Tools</h3>
            <p className="text-sm text-gray-500">
              Analyze and decode Neo transactions.
            </p>
            
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Transaction Hash or Raw Data
                </label>
                <textarea
                  value={input}
                  onChange={(e) => setInput(e.target.value)}
                  rows={4}
                  className="block w-full border-gray-300 rounded-md shadow-sm focus:ring-green-500 focus:border-green-500"
                  placeholder="Enter transaction hash or raw transaction data..."
                />
              </div>
              
              <div className="flex flex-wrap gap-2">
                <button className="px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700">
                  Decode Transaction
                </button>
                <button className="px-4 py-2 border border-gray-300 rounded-md text-gray-700 bg-white hover:bg-gray-50">
                  Calculate Fee
                </button>
                <button className="px-4 py-2 border border-gray-300 rounded-md text-gray-700 bg-white hover:bg-gray-50">
                  Verify Signature
                </button>
              </div>
              
              <div className="mt-6 p-4 bg-gray-50 rounded-md">
                <h4 className="text-sm font-medium text-gray-900 mb-2">Transaction Details</h4>
                <div className="space-y-2 text-sm">
                  <div><span className="font-medium">Hash:</span> <span className="font-mono">0xabcd...1234</span></div>
                  <div><span className="font-medium">Block:</span> 1,234,567</div>
                  <div><span className="font-medium">Size:</span> 250 bytes</div>
                  <div><span className="font-medium">Network Fee:</span> 0.001 GAS</div>
                  <div><span className="font-medium">System Fee:</span> 0.005 GAS</div>
                </div>
              </div>
            </div>
          </div>
        )}
      </motion.div>
    </div>
  );
} 