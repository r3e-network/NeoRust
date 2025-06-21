import React from 'react';

export default function Analytics() {
  return (
    <div className='space-y-6'>
      <div className='md:flex md:items-center md:justify-between'>
        <div className='flex-1 min-w-0'>
          <h2 className='text-2xl font-bold leading-7 text-gray-900 sm:text-3xl sm:truncate'>
            Analytics
          </h2>
          <p className='mt-1 text-sm text-gray-500'>
            Portfolio analytics and performance metrics.
          </p>
        </div>
      </div>

      <div className='bg-white shadow rounded-lg p-6'>
        <p className='text-gray-500'>Analytics dashboard coming soon...</p>
      </div>
    </div>
  );
}
