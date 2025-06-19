import React from 'react';
import { render, screen } from '../../__tests__/utils/test-utils';
import LoadingScreen from '../LoadingScreen';

describe('LoadingScreen Component', () => {
  it('renders with default props', () => {
    render(<LoadingScreen />);
    
    // Check default message
    expect(screen.getByText('Loading Neo N3 Wallet...')).toBeInTheDocument();
    
    // Check title
    expect(screen.getByText('Neo N3 Wallet')).toBeInTheDocument();
    
    // Check Neo logo
    expect(screen.getByText('N3')).toBeInTheDocument();
    
    // Should show spinning dots animation (not progress bar)
    const progressBar = screen.queryByText(/%/);
    expect(progressBar).not.toBeInTheDocument();
  });

  it('renders with custom message', () => {
    const customMessage = 'Connecting to Neo network...';
    render(<LoadingScreen message={customMessage} />);
    
    expect(screen.getByText(customMessage)).toBeInTheDocument();
    expect(screen.queryByText('Loading Neo N3 Wallet...')).not.toBeInTheDocument();
  });

  it('renders with progress bar when progress is provided', () => {
    render(<LoadingScreen progress={50} />);
    
    // Should show progress percentage
    expect(screen.getByText('50%')).toBeInTheDocument();
    
    // Progress bar should be present
    const progressElement = screen.getByText('50%');
    expect(progressElement).toBeInTheDocument();
  });

  it('renders progress bar at 0%', () => {
    render(<LoadingScreen progress={0} />);
    
    expect(screen.getByText('0%')).toBeInTheDocument();
  });

  it('renders progress bar at 100%', () => {
    render(<LoadingScreen progress={100} />);
    
    expect(screen.getByText('100%')).toBeInTheDocument();
  });

  it('renders with both custom message and progress', () => {
    const customMessage = 'Synchronizing blockchain...';
    render(<LoadingScreen message={customMessage} progress={75} />);
    
    expect(screen.getByText(customMessage)).toBeInTheDocument();
    expect(screen.getByText('75%')).toBeInTheDocument();
    expect(screen.getByText('Neo N3 Wallet')).toBeInTheDocument();
  });

  it('has correct CSS classes for styling', () => {
    const { container } = render(<LoadingScreen />);
    
    // Check main container has fixed positioning and gradient background
    const mainDiv = container.firstChild as HTMLElement;
    expect(mainDiv).toHaveClass('fixed', 'inset-0', 'bg-gradient-to-br');
  });

  it('renders floating animation particles', () => {
    const { container } = render(<LoadingScreen />);
    
    // Check for animated background particles
    const particles = container.querySelectorAll('.absolute.w-2.h-2');
    expect(particles.length).toBe(6);
  });

  it('renders without progress when progress is undefined', () => {
    render(<LoadingScreen progress={undefined} />);
    
    // Should not show any percentage
    expect(screen.queryByText(/%/)).not.toBeInTheDocument();
  });

  it('handles edge case with very high progress value', () => {
    render(<LoadingScreen progress={150} />);
    
    // Should still render the value (even if >100%)
    expect(screen.getByText('150%')).toBeInTheDocument();
  });

  it('handles negative progress value', () => {
    render(<LoadingScreen progress={-10} />);
    
    // Should still render the value
    expect(screen.getByText('-10%')).toBeInTheDocument();
  });
}); 