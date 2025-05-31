import React, { useState, useEffect } from 'react';
import { motion } from 'framer-motion';
import {
  PlusIcon,
  CubeIcon,
  MagnifyingGlassIcon,
  FunnelIcon,
  HeartIcon,
  ShareIcon,
} from '@heroicons/react/24/outline';
import { useAppStore } from '../stores/appStore';
import { invoke } from '@tauri-apps/api/tauri';

interface NFTItem {
  id: string;
  name: string;
  description: string;
  image: string;
  contract: string;
  tokenId: string;
  owner: string;
  price?: string;
  rarity: 'common' | 'rare' | 'epic' | 'legendary';
  attributes: Array<{ trait_type: string; value: string }>;
}

export default function NFT() {
  const { currentWallet, addNotification } = useAppStore();
  const [nfts, setNfts] = useState<NFTItem[]>([]);
  const [loading, setLoading] = useState(false);
  const [searchTerm, setSearchTerm] = useState('');
  const [selectedFilter, setSelectedFilter] = useState('all');
  const [showMintModal, setShowMintModal] = useState(false);

  useEffect(() => {
    if (currentWallet) {
      loadNFTs();
    }
  }, [currentWallet]);

  const loadNFTs = async () => {
    if (!currentWallet) return;
    
    setLoading(true);
    try {
      // Mock NFT data
      const mockNFTs: NFTItem[] = [
        {
          id: '1',
          name: 'CryptoKitty #42',
          description: 'A rare digital collectible cat',
          image: 'https://via.placeholder.com/300x300?text=CryptoKitty',
          contract: '0x1234...abcd',
          tokenId: '42',
          owner: currentWallet.address,
          rarity: 'rare',
          attributes: [
            { trait_type: 'Color', value: 'Blue' },
            { trait_type: 'Eyes', value: 'Green' },
            { trait_type: 'Rarity', value: 'Rare' }
          ]
        },
        {
          id: '2',
          name: 'Digital Art #123',
          description: 'Abstract digital artwork',
          image: 'https://via.placeholder.com/300x300?text=Digital+Art',
          contract: '0x5678...efgh',
          tokenId: '123',
          owner: currentWallet.address,
          rarity: 'epic',
          attributes: [
            { trait_type: 'Style', value: 'Abstract' },
            { trait_type: 'Artist', value: 'Neo Artist' }
          ]
        }
      ];
      setNfts(mockNFTs);
    } catch (error) {
      addNotification({
        type: 'error',
        title: 'Error',
        message: 'Failed to load NFTs'
      });
    } finally {
      setLoading(false);
    }
  };

  const handleMintNFT = async (data: any) => {
    setLoading(true);
    try {
      await invoke('mint_nft', data);
      addNotification({
        type: 'success',
        title: 'NFT Minted',
        message: 'Your NFT has been minted successfully'
      });
      setShowMintModal(false);
      loadNFTs();
    } catch (error) {
      addNotification({
        type: 'error',
        title: 'Error',
        message: 'Failed to mint NFT'
      });
    } finally {
      setLoading(false);
    }
  };

  const filteredNFTs = nfts.filter(nft => {
    const matchesSearch = nft.name.toLowerCase().includes(searchTerm.toLowerCase());
    const matchesFilter = selectedFilter === 'all' || nft.rarity === selectedFilter;
    return matchesSearch && matchesFilter;
  });

  const getRarityColor = (rarity: string) => {
    switch (rarity) {
      case 'common': return 'text-gray-600 bg-gray-100';
      case 'rare': return 'text-blue-600 bg-blue-100';
      case 'epic': return 'text-purple-600 bg-purple-100';
      case 'legendary': return 'text-yellow-600 bg-yellow-100';
      default: return 'text-gray-600 bg-gray-100';
    }
  };

  if (!currentWallet) {
    return (
      <div className="flex items-center justify-center h-96">
        <div className="text-center">
          <CubeIcon className="mx-auto h-12 w-12 text-gray-400" />
          <h3 className="mt-2 text-sm font-medium text-gray-900">No wallet connected</h3>
          <p className="mt-1 text-sm text-gray-500">
            Connect a wallet to view and manage your NFTs.
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="md:flex md:items-center md:justify-between">
        <div className="flex-1 min-w-0">
          <h2 className="text-2xl font-bold leading-7 text-gray-900 sm:text-3xl sm:truncate">
            NFT Collection
          </h2>
          <p className="mt-1 text-sm text-gray-500">
            Manage your digital collectibles and create new NFTs.
          </p>
        </div>
        <div className="mt-4 flex md:mt-0 md:ml-4">
          <button
            onClick={() => setShowMintModal(true)}
            className="inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-green-600 hover:bg-green-700"
          >
            <PlusIcon className="-ml-1 mr-2 h-5 w-5" />
            Mint NFT
          </button>
        </div>
      </div>

      {/* Search and Filters */}
      <div className="flex flex-col sm:flex-row gap-4">
        <div className="flex-1">
          <div className="relative">
            <MagnifyingGlassIcon className="absolute left-3 top-1/2 transform -translate-y-1/2 h-5 w-5 text-gray-400" />
            <input
              type="text"
              placeholder="Search NFTs..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className="block w-full pl-10 pr-3 py-2 border border-gray-300 rounded-md leading-5 bg-white placeholder-gray-500 focus:outline-none focus:placeholder-gray-400 focus:ring-1 focus:ring-green-500 focus:border-green-500"
            />
          </div>
        </div>
        <div className="flex items-center space-x-2">
          <FunnelIcon className="h-5 w-5 text-gray-400" />
          <select
            value={selectedFilter}
            onChange={(e) => setSelectedFilter(e.target.value)}
            className="block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-green-500 focus:border-green-500 sm:text-sm rounded-md"
          >
            <option value="all">All Rarities</option>
            <option value="common">Common</option>
            <option value="rare">Rare</option>
            <option value="epic">Epic</option>
            <option value="legendary">Legendary</option>
          </select>
        </div>
      </div>

      {/* NFT Grid */}
      {loading ? (
        <div className="flex justify-center py-12">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-green-600"></div>
        </div>
      ) : filteredNFTs.length === 0 ? (
        <div className="text-center py-12">
          <CubeIcon className="mx-auto h-12 w-12 text-gray-400" />
          <h3 className="mt-2 text-sm font-medium text-gray-900">No NFTs found</h3>
          <p className="mt-1 text-sm text-gray-500">
            {searchTerm ? 'Try adjusting your search terms.' : 'Start by minting your first NFT.'}
          </p>
        </div>
      ) : (
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
          {filteredNFTs.map((nft, index) => (
            <motion.div
              key={nft.id}
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: index * 0.1 }}
              className="bg-white rounded-lg shadow-md overflow-hidden hover:shadow-lg transition-shadow"
            >
              <div className="aspect-square bg-gray-200 relative">
                <img
                  src={nft.image}
                  alt={nft.name}
                  className="w-full h-full object-cover"
                />
                <div className="absolute top-2 right-2 flex space-x-1">
                  <button className="p-1 bg-white rounded-full shadow-sm hover:bg-gray-50">
                    <HeartIcon className="h-4 w-4 text-gray-600" />
                  </button>
                  <button className="p-1 bg-white rounded-full shadow-sm hover:bg-gray-50">
                    <ShareIcon className="h-4 w-4 text-gray-600" />
                  </button>
                </div>
                <div className="absolute top-2 left-2">
                  <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${getRarityColor(nft.rarity)}`}>
                    {nft.rarity}
                  </span>
                </div>
              </div>
              
              <div className="p-4">
                <h3 className="text-lg font-medium text-gray-900 truncate">{nft.name}</h3>
                <p className="text-sm text-gray-500 mt-1 line-clamp-2">{nft.description}</p>
                
                <div className="mt-3">
                  <div className="text-xs text-gray-500 mb-2">Attributes</div>
                  <div className="flex flex-wrap gap-1">
                    {nft.attributes.slice(0, 3).map((attr, i) => (
                      <span
                        key={i}
                        className="inline-flex items-center px-2 py-1 rounded-md text-xs font-medium bg-gray-100 text-gray-800"
                      >
                        {attr.trait_type}: {attr.value}
                      </span>
                    ))}
                    {nft.attributes.length > 3 && (
                      <span className="inline-flex items-center px-2 py-1 rounded-md text-xs font-medium bg-gray-100 text-gray-800">
                        +{nft.attributes.length - 3} more
                      </span>
                    )}
                  </div>
                </div>
                
                <div className="mt-4 flex items-center justify-between">
                  <div className="text-xs text-gray-500">
                    Token ID: {nft.tokenId}
                  </div>
                  <button className="text-sm text-green-600 hover:text-green-500 font-medium">
                    View Details
                  </button>
                </div>
              </div>
            </motion.div>
          ))}
        </div>
      )}

      {/* Mint Modal */}
      {showMintModal && (
        <MintNFTModal
          onClose={() => setShowMintModal(false)}
          onSubmit={handleMintNFT}
          loading={loading}
        />
      )}
    </div>
  );
}

function MintNFTModal({ onClose, onSubmit, loading }: {
  onClose: () => void;
  onSubmit: (data: any) => void;
  loading: boolean;
}) {
  const [formData, setFormData] = useState({
    name: '',
    description: '',
    image: '',
    attributes: [{ trait_type: '', value: '' }]
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSubmit(formData);
  };

  const addAttribute = () => {
    setFormData({
      ...formData,
      attributes: [...formData.attributes, { trait_type: '', value: '' }]
    });
  };

  const updateAttribute = (index: number, field: string, value: string) => {
    const newAttributes = [...formData.attributes];
    newAttributes[index] = { ...newAttributes[index], [field]: value };
    setFormData({ ...formData, attributes: newAttributes });
  };

  return (
    <div className="fixed inset-0 z-50 overflow-y-auto">
      <div className="flex items-center justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0">
        <div className="fixed inset-0 transition-opacity" onClick={onClose}>
          <div className="absolute inset-0 bg-gray-500 opacity-75"></div>
        </div>
        
        <div className="inline-block align-bottom bg-white rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-lg sm:w-full">
          <form onSubmit={handleSubmit}>
            <div className="bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4">
              <h3 className="text-lg font-medium text-gray-900 mb-4">Mint New NFT</h3>
              
              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700">Name</label>
                  <input
                    type="text"
                    value={formData.name}
                    onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                    className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-green-500 focus:border-green-500"
                    required
                  />
                </div>
                
                <div>
                  <label className="block text-sm font-medium text-gray-700">Description</label>
                  <textarea
                    value={formData.description}
                    onChange={(e) => setFormData({ ...formData, description: e.target.value })}
                    rows={3}
                    className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-green-500 focus:border-green-500"
                  />
                </div>
                
                <div>
                  <label className="block text-sm font-medium text-gray-700">Image URL</label>
                  <input
                    type="url"
                    value={formData.image}
                    onChange={(e) => setFormData({ ...formData, image: e.target.value })}
                    className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-green-500 focus:border-green-500"
                    required
                  />
                </div>
                
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">Attributes</label>
                  {formData.attributes.map((attr, index) => (
                    <div key={index} className="flex space-x-2 mb-2">
                      <input
                        type="text"
                        placeholder="Trait type"
                        value={attr.trait_type}
                        onChange={(e) => updateAttribute(index, 'trait_type', e.target.value)}
                        className="flex-1 border-gray-300 rounded-md shadow-sm focus:ring-green-500 focus:border-green-500"
                      />
                      <input
                        type="text"
                        placeholder="Value"
                        value={attr.value}
                        onChange={(e) => updateAttribute(index, 'value', e.target.value)}
                        className="flex-1 border-gray-300 rounded-md shadow-sm focus:ring-green-500 focus:border-green-500"
                      />
                    </div>
                  ))}
                  <button
                    type="button"
                    onClick={addAttribute}
                    className="text-sm text-green-600 hover:text-green-500"
                  >
                    + Add Attribute
                  </button>
                </div>
              </div>
            </div>
            
            <div className="bg-gray-50 px-4 py-3 sm:px-6 sm:flex sm:flex-row-reverse">
              <button
                type="submit"
                disabled={loading}
                className="w-full inline-flex justify-center rounded-md border border-transparent shadow-sm px-4 py-2 bg-green-600 text-base font-medium text-white hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500 sm:ml-3 sm:w-auto sm:text-sm disabled:opacity-50"
              >
                {loading ? 'Minting...' : 'Mint NFT'}
              </button>
              <button
                type="button"
                onClick={onClose}
                className="mt-3 w-full inline-flex justify-center rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-base font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500 sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm"
              >
                Cancel
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  );
} 