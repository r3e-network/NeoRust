#!/bin/bash

# NeoRust CI Quick Fix Script
# Automatically fixes common CI issues

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}[QUICK FIX]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[FIXED]${NC} $1"
}

print_status "Starting NeoRust CI Quick Fix..."

# 1. Fix formatting
print_status "Fixing code formatting..."
cargo fmt --all
print_success "Code formatting applied"

# 2. Try to auto-fix clippy issues
print_status "Attempting to auto-fix clippy issues..."
cargo clippy --all-targets --all-features --fix --allow-dirty || true
print_success "Auto-fixable clippy issues resolved"

# 3. Clean and rebuild
print_status "Cleaning cargo cache..."
cargo clean

# 4. Fix common dependency issues
print_status "Checking for common dependency issues..."

# Update Cargo.lock
print_status "Updating Cargo.lock..."
cargo update

# Fix env_logger anstream issue specifically
print_status "Checking env_logger dependency..."
if grep -r "anstream" . --include="*.toml" > /dev/null 2>&1; then
    print_status "Found anstream feature usage - this may need manual fixing"
fi

# 5. Install missing tools
print_status "Ensuring CI tools are installed..."

if ! command -v cargo-audit &> /dev/null; then
    print_status "Installing cargo-audit..."
    cargo install cargo-audit
fi

if ! command -v cargo-deny &> /dev/null; then
    print_status "Installing cargo-deny..."
    cargo install cargo-deny
fi

if ! command -v cargo-llvm-cov &> /dev/null; then
    print_status "Installing cargo-llvm-cov..."
    cargo install cargo-llvm-cov
fi

print_success "Quick fixes completed! Run ci-check.sh to verify." 