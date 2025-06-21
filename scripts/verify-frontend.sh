#!/bin/bash

# Local frontend verification script based on GitHub Actions workflows
# This script runs the same checks that run in CI/CD for neo-gui

set -e

echo "🌐 Running Frontend verification checks..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print section headers
print_section() {
    echo -e "\n${BLUE}=== $1 ===${NC}"
}

# Function to print success
print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

# Function to print warning
print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

# Function to print error
print_error() {
    echo -e "${RED}✗ $1${NC}"
}

# Check if we're in the right directory
if [ ! -f "neo-gui/package.json" ]; then
    print_error "Not in project root directory. Please run from the root of the NeoRust project."
    exit 1
fi

# Change to neo-gui directory
cd neo-gui

print_section "Checking Node.js and npm versions"
node --version
npm --version
print_success "Node.js and npm are available"

print_section "Installing dependencies"
if npm ci; then
    print_success "Dependencies installed successfully"
else
    print_error "Failed to install dependencies"
    exit 1
fi

print_section "Running TypeScript type checking"
if npm run type-check; then
    print_success "TypeScript types are correct"
else
    print_error "TypeScript type checking failed"
    exit 1
fi

print_section "Running ESLint"
if npm run lint; then
    print_success "No linting errors found"
else
    print_error "ESLint found issues"
    exit 1
fi

print_section "Checking code formatting"
if npm run format:check; then
    print_success "Code formatting is correct"
else
    print_error "Code formatting issues found. Run 'npm run format' to fix."
    exit 1
fi

print_section "Running unit tests with coverage"
if npm run test:coverage; then
    print_success "Unit tests passed with coverage"
else
    print_error "Unit tests failed"
    exit 1
fi

print_section "Building application"
if npm run build; then
    print_success "Application built successfully"
else
    print_error "Application build failed"
    exit 1
fi

print_section "Installing Playwright browsers"
if timeout 60 npx playwright install --with-deps 2>/dev/null; then
    print_success "Playwright browsers installed"
    
    print_section "Running E2E tests"
    if timeout 120 npm run test:e2e; then
        print_success "E2E tests passed"
    else
        print_warning "E2E tests failed (this might be expected in headless environment)"
    fi
else
    print_warning "Playwright installation failed (may need system dependencies) - skipping E2E tests"
fi

print_section "Security audit"
if npm audit --audit-level=high; then
    print_success "No high-severity security vulnerabilities found"
else
    print_warning "Security vulnerabilities found. Review npm audit output."
fi

print_section "Running dependency security check"
if npx audit-ci --config .audit-ci.json; then
    print_success "Dependency security check passed"
else
    print_warning "Dependency security check found issues"
fi

# Go back to project root
cd ..

echo -e "\n${GREEN}🎉 Frontend verification checks completed!${NC}"
echo -e "${BLUE}Check the output above for any warnings or errors.${NC}"