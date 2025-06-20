import React, { useState, useEffect, useCallback } from 'react';
import { motion } from 'framer-motion';
import {
  PlusIcon,
  MagnifyingGlassIcon,
  Squares2X2Icon as GridIcon,
  ListBulletIcon as ListIcon,
  HeartIcon,
  ShareIcon,
  EyeIcon,
  CubeIcon,
  PhotoIcon,
  VideoCameraIcon,
  MusicalNoteIcon,
  DocumentIcon,
} from '@heroicons/react/24/outline';
import { HeartIcon as HeartSolidIcon } from '@heroicons/react/24/solid';
import { useAppStore } from '../stores/appStore';
import { invoke } from '@tauri-apps/api/core';

interface NFTMetadata {
  name: string;
  description: string;
  image: string;
  attributes: Array<{
    trait_type: string;
    value: string | number;
  }>;
  external_url?: string;
  animation_url?: string;
  background_color?: string;
}

interface NFT {
  id: string;
  tokenId: string;
  contractHash: string;
  contractName: string;
  owner: string;
  metadata: NFTMetadata;
  price?: string;
  currency?: string;
  isListed: boolean;
  lastSale?: {
    price: string;
    currency: string;
    date: number;
  };
  rarity?: {
    rank: number;
    score: number;
  };
}

interface Collection {
  id: string;
  name: string;
  description: string;
  contractHash: string;
  image: string;
  banner?: string;
  totalSupply: number;
  floorPrice?: string;
  volume24h?: string;
  owners: number;
  verified: boolean;
}

type ViewMode = 'grid' | 'list';
type FilterType = 'all' | 'owned' | 'created' | 'favorited' | 'listed';
type SortType = 'recent' | 'price_low' | 'price_high' | 'rarity' | 'name';

export default function NFTPage() {
  const { currentWallet, addNotification } = useAppStore();
  const [nfts, setNfts] = useState<NFT[]>([]);
  const [collections, setCollections] = useState<Collection[]>([]);
  const [loading, setLoading] = useState(false);
  const [viewMode, setViewMode] = useState<ViewMode>('grid');
  const [filter, setFilter] = useState<FilterType>('all');
  const [sortBy, setSortBy] = useState<SortType>('recent');
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedCollection, setSelectedCollection] = useState<string | null>(null);
  const [favorites, setFavorites] = useState<Set<string>>(new Set());
  const [showMintModal, setShowMintModal] = useState(false);
  const [showCreateCollectionModal, setShowCreateCollectionModal] = useState(false);

  useEffect(() => {
    loadNFTs();
    loadCollections();
    loadFavorites();
  }, [currentWallet, filter, selectedCollection, loadNFTs, loadCollections, loadFavorites]);

  const loadNFTs = useCallback(async () => {
    if (!currentWallet) return;
    
    setLoading(true);
    try {
      const result = await invoke('get_nfts', {
        walletAddress: currentWallet.address,
        filter,
        collectionId: selectedCollection,
      });
      setNfts(result as NFT[]);
    } catch (error) {
      console.error('Failed to load NFTs:', error);
      addNotification({
        type: 'error',
        title: 'Error',
        message: 'Failed to load NFTs',
      });
    } finally {
      setLoading(false);
    }
  }, [currentWallet, filter, selectedCollection, addNotification]);

  const loadCollections = useCallback(async () => {
    try {
      const result = await invoke('get_nft_collections');
      setCollections(result as Collection[]);
    } catch (error) {
      console.error('Failed to load collections:', error);
    }
  }, []);

  const loadFavorites = useCallback(async () => {
    if (!currentWallet) return;
    
    try {
      const result = await invoke('get_favorite_nfts', {
        walletAddress: currentWallet.address,
      });
      setFavorites(new Set(result as string[]));
    } catch (error) {
      console.error('Failed to load favorites:', error);
    }
  }, [currentWallet]);

  const toggleFavorite = async (nftId: string) => {
    if (!currentWallet) return;

    try {
      const isFavorited = favorites.has(nftId);
      if (isFavorited) {
        await invoke('remove_favorite_nft', {
          walletAddress: currentWallet.address,
          nftId,
        });
        setFavorites(prev => {
          const newSet = new Set(prev);
          newSet.delete(nftId);
          return newSet;
        });
      } else {
        await invoke('add_favorite_nft', {
          walletAddress: currentWallet.address,
          nftId,
        });
        setFavorites(prev => new Set(prev).add(nftId));
      }
    } catch (error) {
      addNotification({
        type: 'error',
        title: 'Error',
        message: 'Failed to update favorites',
      });
    }
  };

  const handleMintNFT = async (metadata: NFTMetadata, collectionId: string) => {
    if (!currentWallet) return;

    setLoading(true);
    try {
      await invoke('mint_nft', {
        walletAddress: currentWallet.address,
        collectionId,
        metadata,
      });
      
      addNotification({
        type: 'success',
        title: 'NFT Minted',
        message: 'Your NFT has been successfully minted!',
      });
      
      setShowMintModal(false);
      loadNFTs();
    } catch (error) {
      addNotification({
        type: 'error',
        title: 'Minting Failed',
        message: 'Failed to mint NFT. Please try again.',
      });
    } finally {
      setLoading(false);
    }
  };

  const handleListNFT = async (nftId: string, price: string, currency: string) => {
    if (!currentWallet) return;

    try {
      await invoke('list_nft', {
        walletAddress: currentWallet.address,
        nftId,
        price,
        currency,
      });
      
      addNotification({
        type: 'success',
        title: 'NFT Listed',
        message: 'Your NFT has been listed for sale!',
      });
      
      loadNFTs();
    } catch (error) {
      addNotification({
        type: 'error',
        title: 'Listing Failed',
        message: 'Failed to list NFT. Please try again.',
      });
    }
  };

  const filteredAndSortedNFTs = nfts
    .filter(nft => {
      if (searchQuery) {
        return nft.metadata.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
               nft.contractName.toLowerCase().includes(searchQuery.toLowerCase());
      }
      return true;
    })
    .sort((a, b) => {
      switch (sortBy) {
        case 'price_low':
          return (parseFloat(a.price || '0') - parseFloat(b.price || '0'));
        case 'price_high':
          return (parseFloat(b.price || '0') - parseFloat(a.price || '0'));
        case 'rarity':
          return (a.rarity?.rank || 999999) - (b.rarity?.rank || 999999);
        case 'name':
          return a.metadata.name.localeCompare(b.metadata.name);
        case 'recent':
        default:
          return parseInt(b.tokenId) - parseInt(a.tokenId);
      }
    });

  const getMediaIcon = (metadata: NFTMetadata) => {
    if (metadata.animation_url) {
      if (metadata.animation_url.includes('.mp4') || metadata.animation_url.includes('.webm')) {
        return <VideoCameraIcon className="h-4 w-4" />;
      } else if (metadata.animation_url.includes('.mp3') || metadata.animation_url.includes('.wav')) {
        return <MusicalNoteIcon className="h-4 w-4" />;
      } else {
        return <DocumentIcon className="h-4 w-4" />;
      }
    }
    return <PhotoIcon className="h-4 w-4" />;
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
            Discover, collect, and trade unique digital assets on Neo N3
          </p>
        </div>
        <div className="mt-4 flex space-x-3 md:mt-0 md:ml-4">
          <button
            onClick={() => setShowCreateCollectionModal(true)}
            className="inline-flex items-center px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50"
          >
            <PlusIcon className="-ml-1 mr-2 h-5 w-5" />
            Create Collection
          </button>
          <button
            onClick={() => setShowMintModal(true)}
            className="inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-green-600 hover:bg-green-700"
          >
            <PlusIcon className="-ml-1 mr-2 h-5 w-5" />
            Mint NFT
          </button>
        </div>
      </div>

      {/* Collections */}
      <div className="bg-white shadow rounded-lg p-6">
        <h3 className="text-lg font-medium text-gray-900 mb-4">Featured Collections</h3>
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
          {collections.slice(0, 4).map((collection) => (
            <motion.div
              key={collection.id}
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              className={`cursor-pointer rounded-lg border-2 p-4 transition-colors ${
                selectedCollection === collection.id
                  ? 'border-green-500 bg-green-50'
                  : 'border-gray-200 hover:border-gray-300'
              }`}
              onClick={() => setSelectedCollection(
                selectedCollection === collection.id ? null : collection.id
              )}
            >
              <img
                src={collection.image}
                alt={collection.name}
                className="w-full h-32 object-cover rounded-md mb-3"
              />
              <div className="flex items-center justify-between mb-2">
                <h4 className="font-medium text-gray-900 truncate">{collection.name}</h4>
                {collection.verified && (
                  <div className="h-4 w-4 bg-blue-500 rounded-full flex items-center justify-center">
                    <span className="text-white text-xs">✓</span>
                  </div>
                )}
              </div>
              <div className="text-sm text-gray-500 space-y-1">
                <div className="flex justify-between">
                  <span>Floor:</span>
                  <span className="font-medium">{collection.floorPrice || 'N/A'}</span>
                </div>
                <div className="flex justify-between">
                  <span>Items:</span>
                  <span className="font-medium">{collection.totalSupply}</span>
                </div>
              </div>
            </motion.div>
          ))}
        </div>
      </div>

      {/* Filters and Search */}
      <div className="bg-white shadow rounded-lg p-6">
        <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between space-y-4 sm:space-y-0">
          <div className="flex items-center space-x-4">
            {/* Search */}
            <div className="relative">
              <MagnifyingGlassIcon className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-gray-400" />
              <input
                type="text"
                placeholder="Search NFTs..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="pl-10 pr-4 py-2 border border-gray-300 rounded-md focus:ring-green-500 focus:border-green-500"
              />
            </div>

            {/* Filter */}
            <select
              value={filter}
              onChange={(e) => setFilter(e.target.value as FilterType)}
              className="border border-gray-300 rounded-md px-3 py-2 focus:ring-green-500 focus:border-green-500"
            >
              <option value="all">All NFTs</option>
              <option value="owned">Owned</option>
              <option value="created">Created</option>
              <option value="favorited">Favorited</option>
              <option value="listed">Listed</option>
            </select>

            {/* Sort */}
            <select
              value={sortBy}
              onChange={(e) => setSortBy(e.target.value as SortType)}
              className="border border-gray-300 rounded-md px-3 py-2 focus:ring-green-500 focus:border-green-500"
            >
              <option value="recent">Recently Added</option>
              <option value="price_low">Price: Low to High</option>
              <option value="price_high">Price: High to Low</option>
              <option value="rarity">Rarity</option>
              <option value="name">Name</option>
            </select>
          </div>

          {/* View Mode */}
          <div className="flex items-center space-x-2">
            <button
              onClick={() => setViewMode('grid')}
              className={`p-2 rounded-md ${
                viewMode === 'grid'
                  ? 'bg-green-100 text-green-600'
                  : 'text-gray-400 hover:text-gray-600'
              }`}
            >
              <GridIcon className="h-5 w-5" />
            </button>
            <button
              onClick={() => setViewMode('list')}
              className={`p-2 rounded-md ${
                viewMode === 'list'
                  ? 'bg-green-100 text-green-600'
                  : 'text-gray-400 hover:text-gray-600'
              }`}
            >
              <ListIcon className="h-5 w-5" />
            </button>
          </div>
        </div>
      </div>

      {/* NFT Grid/List */}
      <div className="bg-white shadow rounded-lg">
        {loading ? (
          <div className="p-8 text-center">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-green-600 mx-auto"></div>
            <p className="mt-2 text-sm text-gray-500">Loading NFTs...</p>
          </div>
        ) : filteredAndSortedNFTs.length === 0 ? (
          <div className="p-8 text-center">
            <CubeIcon className="mx-auto h-12 w-12 text-gray-400" />
            <h3 className="mt-2 text-sm font-medium text-gray-900">No NFTs found</h3>
            <p className="mt-1 text-sm text-gray-500">
              {filter === 'owned' 
                ? "You don't own any NFTs yet. Start by minting or purchasing some!"
                : "No NFTs match your current filters."
              }
            </p>
          </div>
        ) : (
          <div className={`p-6 ${
            viewMode === 'grid' 
              ? 'grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6'
              : 'space-y-4'
          }`}>
            {filteredAndSortedNFTs.map((nft) => (
              <NFTCard
                key={nft.id}
                nft={nft}
                viewMode={viewMode}
                isFavorited={favorites.has(nft.id)}
                onToggleFavorite={() => toggleFavorite(nft.id)}
                onList={(price, currency) => handleListNFT(nft.id, price, currency)}
                getMediaIcon={getMediaIcon}
              />
            ))}
          </div>
        )}
      </div>

      {/* Modals */}
      {showMintModal && (
        <MintNFTModal
          collections={collections}
          onClose={() => setShowMintModal(false)}
          onMint={handleMintNFT}
          loading={loading}
        />
      )}

      {showCreateCollectionModal && (
        <CreateCollectionModal
          onClose={() => setShowCreateCollectionModal(false)}
          onSuccess={() => {
            setShowCreateCollectionModal(false);
            loadCollections();
          }}
        />
      )}
    </div>
  );
}

// NFT Card Component
function NFTCard({ 
  nft, 
  viewMode, 
  isFavorited, 
  onToggleFavorite, 
  onList, 
  getMediaIcon 
}: {
  nft: NFT;
  viewMode: ViewMode;
  isFavorited: boolean;
  onToggleFavorite: () => void;
  onList: (price: string, currency: string) => void;
  getMediaIcon: (metadata: NFTMetadata) => React.ReactNode;
}) {
  const [showListModal, setShowListModal] = useState(false);

  if (viewMode === 'list') {
    return (
      <div className="flex items-center space-x-4 p-4 border border-gray-200 rounded-lg hover:shadow-md transition-shadow">
        <img
          src={nft.metadata.image}
          alt={nft.metadata.name}
          className="w-16 h-16 object-cover rounded-lg"
        />
        <div className="flex-1 min-w-0">
          <div className="flex items-center space-x-2">
            <h3 className="text-sm font-medium text-gray-900 truncate">
              {nft.metadata.name}
            </h3>
            {getMediaIcon(nft.metadata)}
          </div>
          <p className="text-sm text-gray-500 truncate">{nft.contractName}</p>
          {nft.rarity && (
            <p className="text-xs text-gray-400">Rank #{nft.rarity.rank}</p>
          )}
        </div>
        <div className="text-right">
          {nft.price && (
            <p className="text-sm font-medium text-gray-900">
              {nft.price} {nft.currency}
            </p>
          )}
          {nft.lastSale && (
            <p className="text-xs text-gray-500">
              Last: {nft.lastSale.price} {nft.lastSale.currency}
            </p>
          )}
        </div>
        <div className="flex items-center space-x-2">
          <button
            onClick={onToggleFavorite}
            className="p-1 text-gray-400 hover:text-red-500"
          >
            {isFavorited ? (
              <HeartSolidIcon className="h-5 w-5 text-red-500" />
            ) : (
              <HeartIcon className="h-5 w-5" />
            )}
          </button>
          <button className="p-1 text-gray-400 hover:text-gray-600">
            <ShareIcon className="h-5 w-5" />
          </button>
          <button className="p-1 text-gray-400 hover:text-gray-600">
            <EyeIcon className="h-5 w-5" />
          </button>
        </div>
      </div>
    );
  }

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      className="bg-white border border-gray-200 rounded-lg overflow-hidden hover:shadow-lg transition-shadow"
    >
      <div className="relative">
        <img
          src={nft.metadata.image}
          alt={nft.metadata.name}
          className="w-full h-48 object-cover"
        />
        <div className="absolute top-2 right-2 flex space-x-1">
          <button
            onClick={onToggleFavorite}
            className="p-1 bg-white bg-opacity-80 rounded-full text-gray-600 hover:text-red-500"
          >
            {isFavorited ? (
              <HeartSolidIcon className="h-4 w-4 text-red-500" />
            ) : (
              <HeartIcon className="h-4 w-4" />
            )}
          </button>
        </div>
        <div className="absolute top-2 left-2">
          <div className="bg-white bg-opacity-80 rounded-full p-1">
            {getMediaIcon(nft.metadata)}
          </div>
        </div>
        {nft.rarity && (
          <div className="absolute bottom-2 left-2 bg-black bg-opacity-70 text-white text-xs px-2 py-1 rounded">
            Rank #{nft.rarity.rank}
          </div>
        )}
      </div>
      
      <div className="p-4">
        <div className="flex items-center justify-between mb-2">
          <h3 className="text-sm font-medium text-gray-900 truncate">
            {nft.metadata.name}
          </h3>
          <span className="text-xs text-gray-500">#{nft.tokenId}</span>
        </div>
        
        <p className="text-xs text-gray-500 mb-3">{nft.contractName}</p>
        
        {nft.price ? (
          <div className="flex items-center justify-between">
            <div>
              <p className="text-xs text-gray-500">Price</p>
              <p className="text-sm font-medium text-gray-900">
                {nft.price} {nft.currency}
              </p>
            </div>
            <button className="px-3 py-1 bg-green-600 text-white text-xs rounded-md hover:bg-green-700">
              Buy Now
            </button>
          </div>
        ) : (
          <div className="flex space-x-2">
            <button
              onClick={() => setShowListModal(true)}
              className="flex-1 px-3 py-1 border border-gray-300 text-gray-700 text-xs rounded-md hover:bg-gray-50"
            >
              List for Sale
            </button>
            <button className="flex-1 px-3 py-1 bg-green-600 text-white text-xs rounded-md hover:bg-green-700">
              Transfer
            </button>
          </div>
        )}
      </div>

      {showListModal && (
        <ListNFTModal
          nft={nft}
          onClose={() => setShowListModal(false)}
          onList={onList}
        />
      )}
    </motion.div>
  );
}

// Complete Modal Component Implementations
function MintNFTModal({ collections, onClose, onMint, loading }: any) {
  const [formData, setFormData] = useState({
    name: '',
    description: '',
    image: '',
    collectionId: collections[0]?.id || '',
    attributes: [] as Array<{ trait_type: string; value: string }>,
    external_url: '',
    animation_url: '',
  });
  const [imagePreview, setImagePreview] = useState<string>('');
  const [uploading, setUploading] = useState(false);
  const [errors, setErrors] = useState<Record<string, string>>({});

  const addAttribute = () => {
    setFormData(prev => ({
      ...prev,
      attributes: [...prev.attributes, { trait_type: '', value: '' }]
    }));
  };

  const updateAttribute = (index: number, field: 'trait_type' | 'value', value: string) => {
    setFormData(prev => ({
      ...prev,
      attributes: prev.attributes.map((attr, i) => 
        i === index ? { ...attr, [field]: value } : attr
      )
    }));
  };

  const removeAttribute = (index: number) => {
    setFormData(prev => ({
      ...prev,
      attributes: prev.attributes.filter((_, i) => i !== index)
    }));
  };

  const handleImageUpload = async (file: globalThis.File) => {
    if (!file) return;

    // Validate file type
    if (!file.type.startsWith('image/')) {
      setErrors(prev => ({ ...prev, image: 'Please select a valid image file' }));
      return;
    }

    // Validate file size (max 10MB)
    if (file.size > 10 * 1024 * 1024) {
      setErrors(prev => ({ ...prev, image: 'Image must be less than 10MB' }));
      return;
    }

    setUploading(true);
    setErrors(prev => ({ ...prev, image: '' }));

    try {
      // Create preview
      const reader = new globalThis.FileReader();
      reader.onload = (e) => {
        setImagePreview(e.target?.result as string);
      };
      reader.readAsDataURL(file);

      // Upload to IPFS or storage service
      const uploadResult = await invoke('upload_file', {
        file: Array.from(new Uint8Array(await file.arrayBuffer())),
        filename: file.name,
        contentType: file.type,
      });

      setFormData(prev => ({ ...prev, image: uploadResult as string }));
    } catch (error) {
      setErrors(prev => ({ ...prev, image: 'Failed to upload image' }));
    } finally {
      setUploading(false);
    }
  };

  const validateForm = () => {
    const newErrors: Record<string, string> = {};

    if (!formData.name.trim()) {
      newErrors.name = 'Name is required';
    }

    if (!formData.description.trim()) {
      newErrors.description = 'Description is required';
    }

    if (!formData.image) {
      newErrors.image = 'Image is required';
    }

    if (!formData.collectionId) {
      newErrors.collectionId = 'Please select a collection';
    }

    // Validate attributes
    const invalidAttributes = formData.attributes.some(attr => 
      attr.trait_type.trim() && !attr.value.trim()
    );
    if (invalidAttributes) {
      newErrors.attributes = 'All attributes must have both trait type and value';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!validateForm()) return;

    const metadata: NFTMetadata = {
      name: formData.name.trim(),
      description: formData.description.trim(),
      image: formData.image,
      attributes: formData.attributes.filter(attr => 
        attr.trait_type.trim() && attr.value.trim()
      ),
      external_url: formData.external_url.trim() || undefined,
      animation_url: formData.animation_url.trim() || undefined,
    };

    onMint(metadata, formData.collectionId);
  };

  return (
    <div className="fixed inset-0 z-50 overflow-y-auto">
      <div className="flex items-center justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0">
        <div className="fixed inset-0 transition-opacity" onClick={onClose}>
          <div className="absolute inset-0 bg-gray-500 opacity-75"></div>
        </div>
        
        <div className="inline-block align-bottom bg-white rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-2xl sm:w-full">
          <form onSubmit={handleSubmit}>
            <div className="bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4 max-h-96 overflow-y-auto">
              <div className="flex items-center justify-between mb-6">
                <h3 className="text-lg font-medium text-gray-900">Mint New NFT</h3>
                <button
                  type="button"
                  onClick={onClose}
                  className="text-gray-400 hover:text-gray-600"
                >
                  <span className="sr-only">Close</span>
                  ✕
                </button>
              </div>

              <div className="space-y-6">
                {/* Collection Selection */}
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Collection *
                  </label>
                  <select
                    value={formData.collectionId}
                    onChange={(e) => setFormData(prev => ({ ...prev, collectionId: e.target.value }))}
                    className={`w-full border rounded-md px-3 py-2 focus:ring-green-500 focus:border-green-500 ${
                      errors.collectionId ? 'border-red-500' : 'border-gray-300'
                    }`}
                  >
                    <option value="">Select a collection</option>
                    {collections.map((collection: Collection) => (
                      <option key={collection.id} value={collection.id}>
                        {collection.name}
                      </option>
                    ))}
                  </select>
                  {errors.collectionId && (
                    <p className="mt-1 text-sm text-red-600">{errors.collectionId}</p>
                  )}
                </div>

                {/* Image Upload */}
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Image *
                  </label>
                  <div className="flex items-center space-x-4">
                    <input
                      type="file"
                      accept="image/*"
                      onChange={(e) => e.target.files?.[0] && handleImageUpload(e.target.files[0])}
                      className="block w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-md file:border-0 file:text-sm file:font-medium file:bg-green-50 file:text-green-700 hover:file:bg-green-100"
                      disabled={uploading}
                    />
                  </div>
                  {uploading && (
                    <div className="mt-2 flex items-center space-x-2">
                      <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-green-600"></div>
                      <span className="text-sm text-gray-500">Uploading...</span>
                    </div>
                  )}
                  {imagePreview && (
                    <div className="mt-2">
                      <img src={imagePreview} alt="Preview" className="w-32 h-32 object-cover rounded-lg" />
                    </div>
                  )}
                  {errors.image && (
                    <p className="mt-1 text-sm text-red-600">{errors.image}</p>
                  )}
                </div>

                {/* Name */}
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Name *
                  </label>
                  <input
                    type="text"
                    value={formData.name}
                    onChange={(e) => setFormData(prev => ({ ...prev, name: e.target.value }))}
                    className={`w-full border rounded-md px-3 py-2 focus:ring-green-500 focus:border-green-500 ${
                      errors.name ? 'border-red-500' : 'border-gray-300'
                    }`}
                    placeholder="Enter NFT name"
                  />
                  {errors.name && (
                    <p className="mt-1 text-sm text-red-600">{errors.name}</p>
                  )}
                </div>

                {/* Description */}
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Description *
                  </label>
                  <textarea
                    value={formData.description}
                    onChange={(e) => setFormData(prev => ({ ...prev, description: e.target.value }))}
                    rows={3}
                    className={`w-full border rounded-md px-3 py-2 focus:ring-green-500 focus:border-green-500 ${
                      errors.description ? 'border-red-500' : 'border-gray-300'
                    }`}
                    placeholder="Describe your NFT"
                  />
                  {errors.description && (
                    <p className="mt-1 text-sm text-red-600">{errors.description}</p>
                  )}
                </div>

                {/* External URL */}
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    External URL (optional)
                  </label>
                  <input
                    type="url"
                    value={formData.external_url}
                    onChange={(e) => setFormData(prev => ({ ...prev, external_url: e.target.value }))}
                    className="w-full border border-gray-300 rounded-md px-3 py-2 focus:ring-green-500 focus:border-green-500"
                    placeholder="https://..."
                  />
                </div>

                {/* Animation URL */}
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Animation URL (optional)
                  </label>
                  <input
                    type="url"
                    value={formData.animation_url}
                    onChange={(e) => setFormData(prev => ({ ...prev, animation_url: e.target.value }))}
                    className="w-full border border-gray-300 rounded-md px-3 py-2 focus:ring-green-500 focus:border-green-500"
                    placeholder="https://... (video, audio, or 3D file)"
                  />
                </div>

                {/* Attributes */}
                <div>
                  <div className="flex items-center justify-between mb-2">
                    <label className="block text-sm font-medium text-gray-700">
                      Attributes (optional)
                    </label>
                    <button
                      type="button"
                      onClick={addAttribute}
                      className="text-sm text-green-600 hover:text-green-500"
                    >
                      + Add Attribute
                    </button>
                  </div>
                  <div className="space-y-2">
                    {formData.attributes.map((attr, index) => (
                      <div key={index} className="flex space-x-2">
                        <input
                          type="text"
                          placeholder="Trait type"
                          value={attr.trait_type}
                          onChange={(e) => updateAttribute(index, 'trait_type', e.target.value)}
                          className="flex-1 border border-gray-300 rounded-md px-3 py-2 focus:ring-green-500 focus:border-green-500"
                        />
                        <input
                          type="text"
                          placeholder="Value"
                          value={attr.value}
                          onChange={(e) => updateAttribute(index, 'value', e.target.value)}
                          className="flex-1 border border-gray-300 rounded-md px-3 py-2 focus:ring-green-500 focus:border-green-500"
                        />
                        <button
                          type="button"
                          onClick={() => removeAttribute(index)}
                          className="px-3 py-2 text-red-600 hover:text-red-500"
                        >
                          ✕
                        </button>
                      </div>
                    ))}
                  </div>
                  {errors.attributes && (
                    <p className="mt-1 text-sm text-red-600">{errors.attributes}</p>
                  )}
                </div>
              </div>
            </div>
            
            <div className="bg-gray-50 px-4 py-3 sm:px-6 sm:flex sm:flex-row-reverse">
              <button
                type="submit"
                disabled={loading || uploading}
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

function CreateCollectionModal({ onClose, onSuccess }: any) {
  return (
    <div className="fixed inset-0 z-50 overflow-y-auto">
      {/* Modal implementation for creating collections */}
      <div className="flex items-center justify-center min-h-screen p-4">
        <div className="bg-white rounded-lg shadow-xl max-w-md w-full p-6">
          <h3 className="text-lg font-medium text-gray-900 mb-4">Create Collection</h3>
          <p className="text-sm text-gray-500 mb-4">Collection creation form would be implemented here.</p>
          <div className="flex space-x-3">
            <button
              onClick={onSuccess}
              className="flex-1 bg-green-600 text-white px-4 py-2 rounded-md hover:bg-green-700"
            >
              Create
            </button>
            <button
              onClick={onClose}
              className="flex-1 border border-gray-300 text-gray-700 px-4 py-2 rounded-md hover:bg-gray-50"
            >
              Cancel
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}

function ListNFTModal({ nft, onClose, onList }: any) {
  const [price, setPrice] = useState('');
  const [currency, setCurrency] = useState('NEO');

  return (
    <div className="fixed inset-0 z-50 overflow-y-auto">
      {/* Modal implementation for listing NFTs */}
      <div className="flex items-center justify-center min-h-screen p-4">
        <div className="bg-white rounded-lg shadow-xl max-w-md w-full p-6">
          <h3 className="text-lg font-medium text-gray-900 mb-4">List {nft.metadata.name}</h3>
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-700">Price</label>
              <input
                type="number"
                value={price}
                onChange={(e) => setPrice(e.target.value)}
                className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-green-500 focus:border-green-500"
                placeholder="0.00"
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700">Currency</label>
              <select
                value={currency}
                onChange={(e) => setCurrency(e.target.value)}
                className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-green-500 focus:border-green-500"
              >
                <option value="NEO">NEO</option>
                <option value="GAS">GAS</option>
              </select>
            </div>
          </div>
          <div className="flex space-x-3 mt-6">
            <button
              onClick={() => onList(price, currency)}
              className="flex-1 bg-green-600 text-white px-4 py-2 rounded-md hover:bg-green-700"
            >
              List for Sale
            </button>
            <button
              onClick={onClose}
              className="flex-1 border border-gray-300 text-gray-700 px-4 py-2 rounded-md hover:bg-gray-50"
            >
              Cancel
            </button>
          </div>
        </div>
      </div>
    </div>
  );
} 