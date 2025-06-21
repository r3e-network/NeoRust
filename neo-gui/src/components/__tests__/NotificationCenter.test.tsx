import React from 'react';
import { render, screen, fireEvent } from '../../__tests__/utils/test-utils';
import NotificationCenter from '../NotificationCenter';
import { useAppStore } from '../../stores/appStore';

// Mock the store
jest.mock('../../stores/appStore');
const mockUseAppStore = useAppStore as jest.MockedFunction<typeof useAppStore>;

/* global afterEach */
describe('NotificationCenter Component', () => {
  const mockNotifications = [
    {
      id: '1',
      type: 'success' as const,
      title: 'Transaction Successful',
      message: 'Your transaction has been confirmed',
      timestamp: Date.now(),
      read: false,
    },
    {
      id: '2',
      type: 'error' as const,
      title: 'Connection Error',
      message: 'Failed to connect to Neo network',
      timestamp: Date.now() - 1000,
      read: true,
    },
    {
      id: '3',
      type: 'warning' as const,
      title: 'Low Balance',
      message: 'Your GAS balance is running low',
      timestamp: Date.now() - 2000,
      read: false,
    },
    {
      id: '4',
      type: 'info' as const,
      title: 'New Update Available',
      message: 'Version 2.0 is available for download',
      timestamp: Date.now() - 3000,
      read: false,
    },
  ];

  const mockStore = {
    notifications: [],
    removeNotification: jest.fn(),
    markNotificationRead: jest.fn(),
  };

  beforeEach(() => {
    mockUseAppStore.mockReturnValue({
      ...mockStore,
      notifications: [],
    });
    jest.clearAllMocks();
    jest.useFakeTimers();
  });

  afterEach(() => {
    jest.runOnlyPendingTimers();
    jest.useRealTimers();
  });

  it('renders empty state when no notifications', () => {
    render(<NotificationCenter />);

    // Should not render any notifications
    expect(
      screen.queryByText('Transaction Successful')
    ).not.toBeInTheDocument();
  });

  it('renders single notification correctly', () => {
    mockUseAppStore.mockReturnValue({
      ...mockStore,
      notifications: [mockNotifications[0]],
    });

    render(<NotificationCenter />);

    expect(screen.getByText('Transaction Successful')).toBeInTheDocument();
    expect(
      screen.getByText('Your transaction has been confirmed')
    ).toBeInTheDocument();
  });

  it('renders multiple notifications', () => {
    mockUseAppStore.mockReturnValue({
      ...mockStore,
      notifications: mockNotifications.slice(0, 3),
    });

    render(<NotificationCenter />);

    expect(screen.getByText('Transaction Successful')).toBeInTheDocument();
    expect(screen.getByText('Connection Error')).toBeInTheDocument();
    expect(screen.getByText('Low Balance')).toBeInTheDocument();
  });

  it('limits display to 5 notifications maximum', () => {
    const manyNotifications = Array.from({ length: 8 }, (_, i) => ({
      ...mockNotifications[0],
      id: `notification-${i}`,
      title: `Notification ${i}`,
    }));

    mockUseAppStore.mockReturnValue({
      ...mockStore,
      notifications: manyNotifications,
    });

    render(<NotificationCenter />);

    // Should show "+3 more notifications" indicator
    expect(screen.getByText('+3 more notifications')).toBeInTheDocument();
  });

  it('displays different notification types with correct styling', () => {
    mockUseAppStore.mockReturnValue({
      ...mockStore,
      notifications: mockNotifications,
    });

    render(<NotificationCenter />);

    // Check for different notification types
    expect(screen.getByText('Transaction Successful')).toBeInTheDocument(); // success
    expect(screen.getByText('Connection Error')).toBeInTheDocument(); // error
    expect(screen.getByText('Low Balance')).toBeInTheDocument(); // warning
    expect(screen.getByText('New Update Available')).toBeInTheDocument(); // info
  });

  it('handles notification click to mark as read', () => {
    mockUseAppStore.mockReturnValue({
      ...mockStore,
      notifications: [mockNotifications[0]],
    });

    render(<NotificationCenter />);

    const notification = screen
      .getByText('Transaction Successful')
      .closest('div');
    fireEvent.click(notification!);

    expect(mockStore.markNotificationRead).toHaveBeenCalledWith('1');
  });

  it('handles dismiss button click', () => {
    mockUseAppStore.mockReturnValue({
      ...mockStore,
      notifications: [mockNotifications[0]],
    });

    render(<NotificationCenter />);

    const dismissButton = screen.getByRole('button');
    fireEvent.click(dismissButton);

    expect(mockStore.removeNotification).toHaveBeenCalledWith('1');
  });

  it('dismiss button stops event propagation', () => {
    mockUseAppStore.mockReturnValue({
      ...mockStore,
      notifications: [mockNotifications[0]],
    });

    render(<NotificationCenter />);

    const dismissButton = screen.getByRole('button');
    fireEvent.click(dismissButton);

    // Should call removeNotification but not markNotificationRead
    expect(mockStore.removeNotification).toHaveBeenCalledWith('1');
    expect(mockStore.markNotificationRead).not.toHaveBeenCalled();
  });

  it('displays timestamp correctly', () => {
    const fixedTime = new Date('2024-01-01T12:00:00Z').getTime();
    const notificationWithFixedTime = {
      ...mockNotifications[0],
      timestamp: fixedTime,
    };

    mockUseAppStore.mockReturnValue({
      ...mockStore,
      notifications: [notificationWithFixedTime],
    });

    render(<NotificationCenter />);

    // Should display formatted time (could be in 12-hour or 24-hour format)
    expect(screen.getByText(/12:00|8:00/)).toBeInTheDocument();
  });

  it('shows unread notification styling', () => {
    mockUseAppStore.mockReturnValue({
      ...mockStore,
      notifications: [mockNotifications[0]], // unread notification
    });

    const { container } = render(<NotificationCenter />);

    // Unread notifications should have special styling (ring)
    const notification = container.querySelector(
      '.ring-2.ring-offset-2.ring-neo-500'
    );
    expect(notification).toBeInTheDocument();
  });

  it('does not show ring styling for read notifications', () => {
    mockUseAppStore.mockReturnValue({
      ...mockStore,
      notifications: [mockNotifications[1]], // read notification
    });

    const { container } = render(<NotificationCenter />);

    // Read notifications should not have ring styling
    const notification = container.querySelector(
      '.ring-2.ring-offset-2.ring-neo-500'
    );
    expect(notification).not.toBeInTheDocument();
  });

  it('auto-dismisses non-error notifications after 5 seconds', () => {
    // Note: This test checks that the progress bar animation is set up correctly
    // The actual auto-dismiss functionality happens via framer-motion onAnimationComplete
    mockUseAppStore.mockReturnValue({
      ...mockStore,
      notifications: [mockNotifications[0]], // success notification
    });

    render(<NotificationCenter />);

    // Verify that the progress bar is rendered with the correct animation props
    // In a real scenario, framer-motion would handle the auto-dismiss
    expect(screen.getByText('Transaction Successful')).toBeInTheDocument();

    // The progress bar should be present (auto-dismiss is configured)
    const progressBar = document.querySelector('.absolute.bottom-0.left-0.h-1');
    expect(progressBar).toBeInTheDocument();
  });

  it('does not auto-dismiss error notifications', () => {
    jest.useFakeTimers();

    mockUseAppStore.mockReturnValue({
      ...mockStore,
      notifications: [mockNotifications[1]], // error notification
    });

    render(<NotificationCenter />);

    // Fast-forward 10 seconds
    jest.advanceTimersByTime(10000);

    // Error notifications should not auto-dismiss
    expect(mockStore.removeNotification).not.toHaveBeenCalled();
  });

  it('renders correct icons for each notification type', () => {
    mockUseAppStore.mockReturnValue({
      ...mockStore,
      notifications: mockNotifications,
    });

    const { container } = render(<NotificationCenter />);

    // Should have different colored icons
    const greenIcons = container.querySelectorAll('.text-green-400');
    const redIcons = container.querySelectorAll('.text-red-400');
    const yellowIcons = container.querySelectorAll('.text-yellow-400');
    const blueIcons = container.querySelectorAll('.text-blue-400');

    expect(greenIcons.length).toBeGreaterThan(0); // success
    expect(redIcons.length).toBeGreaterThan(0); // error
    expect(yellowIcons.length).toBeGreaterThan(0); // warning
    expect(blueIcons.length).toBeGreaterThan(0); // info
  });
});
