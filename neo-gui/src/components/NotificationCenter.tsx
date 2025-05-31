import React from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import {
  CheckCircleIcon,
  ExclamationTriangleIcon,
  InformationCircleIcon,
  XCircleIcon,
  XMarkIcon,
} from '@heroicons/react/24/outline';
import { useAppStore } from '../stores/appStore';

const iconMap = {
  success: CheckCircleIcon,
  error: XCircleIcon,
  warning: ExclamationTriangleIcon,
  info: InformationCircleIcon,
};

const colorMap = {
  success: 'bg-green-50 border-green-200 text-green-800',
  error: 'bg-red-50 border-red-200 text-red-800',
  warning: 'bg-yellow-50 border-yellow-200 text-yellow-800',
  info: 'bg-blue-50 border-blue-200 text-blue-800',
};

const iconColorMap = {
  success: 'text-green-400',
  error: 'text-red-400',
  warning: 'text-yellow-400',
  info: 'text-blue-400',
};

export default function NotificationCenter() {
  const { notifications, removeNotification, markNotificationRead } = useAppStore();

  const handleDismiss = (id: string) => {
    removeNotification(id);
  };

  const handleClick = (id: string) => {
    markNotificationRead(id);
  };

  return (
    <div className="fixed top-4 right-4 z-50 space-y-2 max-w-sm">
      <AnimatePresence>
        {notifications.slice(0, 5).map((notification) => {
          const Icon = iconMap[notification.type];
          const colorClass = colorMap[notification.type];
          const iconColorClass = iconColorMap[notification.type];

          return (
            <motion.div
              key={notification.id}
              initial={{ opacity: 0, x: 300, scale: 0.8 }}
              animate={{ opacity: 1, x: 0, scale: 1 }}
              exit={{ opacity: 0, x: 300, scale: 0.8 }}
              transition={{ duration: 0.3, type: 'spring', damping: 25 }}
              className={`relative rounded-lg border p-4 shadow-lg backdrop-blur-sm ${colorClass} ${
                !notification.read ? 'ring-2 ring-offset-2 ring-neo-500' : ''
              }`}
              onClick={() => handleClick(notification.id)}
            >
              <div className="flex items-start">
                <div className="flex-shrink-0">
                  <Icon className={`h-5 w-5 ${iconColorClass}`} />
                </div>
                <div className="ml-3 flex-1">
                  <h3 className="text-sm font-medium">{notification.title}</h3>
                  <p className="mt-1 text-sm opacity-90">{notification.message}</p>
                  <p className="mt-2 text-xs opacity-70">
                    {new Date(notification.timestamp).toLocaleTimeString()}
                  </p>
                </div>
                <div className="ml-4 flex-shrink-0">
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      handleDismiss(notification.id);
                    }}
                    className="inline-flex rounded-md p-1.5 hover:bg-black hover:bg-opacity-10 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-neo-500"
                  >
                    <XMarkIcon className="h-4 w-4" />
                  </button>
                </div>
              </div>

              {/* Progress bar for auto-dismiss */}
              {notification.type !== 'error' && (
                <motion.div
                  className="absolute bottom-0 left-0 h-1 bg-current opacity-30 rounded-b-lg"
                  initial={{ width: '100%' }}
                  animate={{ width: '0%' }}
                  transition={{ duration: 5, ease: 'linear' }}
                  onAnimationComplete={() => handleDismiss(notification.id)}
                />
              )}
            </motion.div>
          );
        })}
      </AnimatePresence>

      {/* Show count if more than 5 notifications */}
      {notifications.length > 5 && (
        <motion.div
          initial={{ opacity: 0, scale: 0.8 }}
          animate={{ opacity: 1, scale: 1 }}
          className="text-center"
        >
          <div className="inline-flex items-center px-3 py-1 rounded-full text-xs font-medium bg-gray-100 text-gray-600">
            +{notifications.length - 5} more notifications
          </div>
        </motion.div>
      )}
    </div>
  );
} 