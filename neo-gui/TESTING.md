# Testing Guide for Neo GUI

This document provides comprehensive information about testing the Neo GUI frontend application.

## Testing Strategy

Our testing approach follows a multi-layered strategy:

1. **Unit Tests** - Test individual components, functions, and stores in isolation
2. **Integration Tests** - Test component interactions and data flow
3. **End-to-End Tests** - Test complete user workflows using Playwright
4. **Visual Regression Tests** - Ensure UI consistency across changes
5. **Performance Tests** - Monitor application performance metrics

## Test Structure

```
neo-gui/
├── src/
│   ├── __tests__/           # Test utilities and shared mocks
│   │   └── utils/
│   │       └── test-utils.tsx
│   ├── components/
│   │   └── __tests__/       # Component unit tests
│   ├── stores/
│   │   └── __tests__/       # Store unit tests
│   ├── pages/
│   │   └── __tests__/       # Page component tests
│   └── setupTests.ts        # Global test setup
├── e2e/                     # End-to-end tests
│   ├── app.spec.ts
│   └── dashboard.spec.ts
├── jest.config.js           # Jest configuration
├── playwright.config.ts     # Playwright configuration
└── coverage/               # Coverage reports (generated)
```

## Running Tests

### Unit and Integration Tests

```bash
# Run all tests
npm test

# Run tests in watch mode
npm run test:watch

# Run tests with coverage
npm run test:coverage

# Run specific test file
npm test -- Layout.test.tsx

# Run tests matching a pattern
npm test -- --testNamePattern="should render"
```

### End-to-End Tests

```bash
# Run E2E tests
npm run test:e2e

# Run E2E tests with UI
npm run test:e2e:ui

# Run specific E2E test
npx playwright test dashboard.spec.ts

# Debug E2E tests
npx playwright test --debug
```

### Code Quality Checks

```bash
# Run linter
npm run lint

# Fix linting issues
npm run lint:fix

# Check formatting
npm run format:check

# Fix formatting
npm run format

# Type checking
npm run type-check
```

## Test Categories

### 1. Unit Tests

**Components (`src/components/__tests__/`)**
- Test component rendering
- Test user interactions
- Test prop handling
- Test conditional rendering
- Test error states

**Stores (`src/stores/__tests__/`)**
- Test state management
- Test action dispatching
- Test state persistence
- Test computed values
- Test side effects

**Utilities (`src/utils/__tests__/`)**
- Test pure functions
- Test data transformations
- Test validation logic
- Test helper functions

### 2. Integration Tests

**Page Components (`src/pages/__tests__/`)**
- Test page functionality
- Test component integration
- Test data fetching
- Test routing behavior
- Test error boundaries

### 3. End-to-End Tests

**User Workflows (`e2e/`)**
- Test complete user journeys
- Test cross-browser compatibility
- Test responsive design
- Test accessibility
- Test performance

## Writing Tests

### Unit Test Example

```typescript
import { render, screen, fireEvent } from '../../../__tests__/utils/test-utils';
import { useAppStore } from '../../stores/appStore';
import MyComponent from '../MyComponent';

// Mock dependencies
jest.mock('../../stores/appStore');
const mockUseAppStore = useAppStore as jest.MockedFunction<typeof useAppStore>;

describe('MyComponent', () => {
  const mockStore = {
    data: [],
    loading: false,
    fetchData: jest.fn(),
  };

  beforeEach(() => {
    mockUseAppStore.mockReturnValue(mockStore);
    jest.clearAllMocks();
  });

  it('should render correctly', () => {
    render(<MyComponent />);
    expect(screen.getByText('Expected Text')).toBeInTheDocument();
  });

  it('should handle user interaction', async () => {
    render(<MyComponent />);
    
    const button = screen.getByRole('button', { name: /click me/i });
    fireEvent.click(button);
    
    expect(mockStore.fetchData).toHaveBeenCalled();
  });
});
```

### E2E Test Example

```typescript
import { test, expect } from '@playwright/test';

test.describe('Dashboard', () => {
  test('should display wallet information', async ({ page }) => {
    await page.goto('/');
    
    // Test navigation
    await page.getByText('Dashboard').click();
    await expect(page).toHaveURL('/');
    
    // Test content
    await expect(page.getByText('Wallet Balance')).toBeVisible();
    
    // Test interaction
    await page.getByRole('button', { name: 'Connect Wallet' }).click();
    await expect(page.getByText('Wallet Connected')).toBeVisible();
  });
});
```

## Test Utilities

### Custom Render Function

```typescript
import { render } from '@testing-library/react';
import { BrowserRouter } from 'react-router-dom';

const AllTheProviders = ({ children }) => {
  return (
    <BrowserRouter>
      {children}
    </BrowserRouter>
  );
};

const customRender = (ui, options) =>
  render(ui, { wrapper: AllTheProviders, ...options });

export { customRender as render };
```

### Mock Data Generators

```typescript
export const generateMockWallet = (overrides = {}) => ({
  address: 'NQyJR9dCZeYJNgK7N5Z9vHW3pJy4QfqPqY',
  balance: { NEO: '100', GAS: '1000.5' },
  isConnected: true,
  ...overrides,
});
```

## Mocking Strategies

### External Dependencies

```typescript
// Mock Tauri API
jest.mock('@tauri-apps/api', () => ({
  invoke: jest.fn(),
  listen: jest.fn(),
}));

// Mock Framer Motion
jest.mock('framer-motion', () => ({
  motion: {
    div: ({ children, ...props }) => <div {...props}>{children}</div>,
  },
  AnimatePresence: ({ children }) => children,
}));
```

### Store Mocking

```typescript
// Mock Zustand store
jest.mock('../stores/appStore');
const mockUseAppStore = useAppStore as jest.MockedFunction<typeof useAppStore>;

beforeEach(() => {
  mockUseAppStore.mockReturnValue({
    // Mock state and actions
  });
});
```

## Coverage Requirements

- **Unit Tests**: Minimum 80% code coverage
- **Integration Tests**: Critical user paths covered
- **E2E Tests**: Main workflows tested

### Coverage Reports

Coverage reports are generated in the `coverage/` directory:
- `coverage/index.html` - HTML report
- `coverage/lcov.info` - LCOV format for CI

## Continuous Integration

### GitHub Actions

Tests run automatically on:
- Push to main/master/develop branches
- Pull requests
- Scheduled nightly runs

### Pre-commit Hooks

Before each commit:
1. Linting and formatting checks
2. Type checking
3. Unit tests for changed files

## Performance Testing

### Bundle Size Monitoring

```bash
# Analyze bundle size
npm run build
npx bundlesize
```

### Lighthouse CI

```bash
# Run Lighthouse performance tests
npx lighthouse-ci autorun
```

## Debugging Tests

### Jest Debugging

```bash
# Debug specific test
node --inspect-brk node_modules/.bin/jest --runInBand Component.test.tsx
```

### Playwright Debugging

```bash
# Debug with browser UI
npx playwright test --debug

# Record new test
npx playwright codegen localhost:1420
```

## Best Practices

### General

1. **Test Behavior, Not Implementation**
   - Focus on what the user sees and does
   - Avoid testing internal state or methods

2. **Arrange, Act, Assert Pattern**
   ```typescript
   it('should update counter', () => {
     // Arrange
     render(<Counter initialValue={0} />);
     
     // Act
     fireEvent.click(screen.getByText('Increment'));
     
     // Assert
     expect(screen.getByText('1')).toBeInTheDocument();
   });
   ```

3. **Use Descriptive Test Names**
   - Good: `should display error message when API call fails`
   - Bad: `test error`

4. **Keep Tests Independent**
   - Each test should run in isolation
   - Use `beforeEach` to reset state

### Accessibility Testing

```typescript
import { axe, toHaveNoViolations } from 'jest-axe';

expect.extend(toHaveNoViolations);

it('should have no accessibility violations', async () => {
  const { container } = render(<MyComponent />);
  const results = await axe(container);
  expect(results).toHaveNoViolations();
});
```

## Troubleshooting

### Common Issues

1. **Tests timeout**
   - Increase timeout in Jest/Playwright config
   - Use `waitFor` for async operations

2. **Flaky E2E tests**
   - Add proper wait conditions
   - Use `toBeVisible()` instead of `toBeInTheDocument()`

3. **Mock not working**
   - Ensure mock is imported before component
   - Check mock path is correct

### Getting Help

- Check existing tests for patterns
- Review error messages carefully
- Use debugging tools (React DevTools, Browser DevTools)
- Ask team members for code review

## Contributing

When adding new features:

1. Write tests first (TDD)
2. Ensure tests pass locally
3. Update documentation if needed
4. Include tests in PR description

## Resources

- [Jest Documentation](https://jestjs.io/docs/getting-started)
- [Testing Library Docs](https://testing-library.com/docs/)
- [Playwright Documentation](https://playwright.dev/)
- [React Testing Patterns](https://react-testing-examples.com/) 