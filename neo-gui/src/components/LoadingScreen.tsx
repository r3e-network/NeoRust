import React from 'react';
import { motion } from 'framer-motion';

interface LoadingScreenProps {
  message?: string;
  progress?: number;
}

export default function LoadingScreen({
  message = 'Loading Neo N3 Wallet...',
  progress,
}: LoadingScreenProps) {
  return (
    <div className='fixed inset-0 bg-gradient-to-br from-gray-50 to-gray-100 flex items-center justify-center z-50'>
      <div className='text-center'>
        {/* Neo Logo */}
        <motion.div
          initial={{ scale: 0.8, opacity: 0 }}
          animate={{ scale: 1, opacity: 1 }}
          transition={{ duration: 0.5 }}
          className='mb-8'
        >
          <div className='w-20 h-20 mx-auto bg-gradient-to-br from-neo-400 to-neo-600 rounded-2xl flex items-center justify-center shadow-lg'>
            <span className='text-white font-bold text-2xl'>N3</span>
          </div>
        </motion.div>

        {/* Loading Animation */}
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ delay: 0.2, duration: 0.5 }}
          className='mb-6'
        >
          {progress !== undefined ? (
            // Progress bar
            <div className='w-64 mx-auto'>
              <div className='bg-gray-200 rounded-full h-2 mb-2'>
                <motion.div
                  className='bg-gradient-to-r from-neo-400 to-neo-600 h-2 rounded-full'
                  initial={{ width: 0 }}
                  animate={{ width: `${progress}%` }}
                  transition={{ duration: 0.3 }}
                />
              </div>
              <div className='text-sm text-gray-600'>
                {Math.round(progress)}%
              </div>
            </div>
          ) : (
            // Spinning dots
            <div className='flex justify-center space-x-2'>
              {[0, 1, 2].map(i => (
                <motion.div
                  key={i}
                  className='w-3 h-3 bg-neo-500 rounded-full'
                  animate={{
                    scale: [1, 1.2, 1],
                    opacity: [0.7, 1, 0.7],
                  }}
                  transition={{
                    duration: 1.5,
                    repeat: Infinity,
                    delay: i * 0.2,
                  }}
                />
              ))}
            </div>
          )}
        </motion.div>

        {/* Loading Message */}
        <motion.div
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.4, duration: 0.5 }}
        >
          <h2 className='text-xl font-semibold text-gray-900 mb-2'>
            Neo N3 Wallet
          </h2>
          <p className='text-gray-600'>{message}</p>
        </motion.div>

        {/* Subtle animation background */}
        <div className='absolute inset-0 overflow-hidden pointer-events-none'>
          {[...Array(6)].map((_, i) => (
            <motion.div
              key={i}
              className='absolute w-2 h-2 bg-neo-200 rounded-full opacity-30'
              style={{
                left: `${Math.random() * 100}%`,
                top: `${Math.random() * 100}%`,
              }}
              animate={{
                y: [-20, -100],
                opacity: [0, 0.6, 0],
              }}
              transition={{
                duration: 3 + Math.random() * 2,
                repeat: Infinity,
                delay: Math.random() * 2,
              }}
            />
          ))}
        </div>
      </div>
    </div>
  );
}
