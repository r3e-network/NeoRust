#!/bin/bash

# NeoRust CI Check Script - Local GitHub Workflow Replication
# This script runs all the same checks as the GitHub CI workflow

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print status
print_status() {
    echo -e "${BLUE}[CI CHECK]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to run a command and track success/failure
run_check() {
    local check_name="$1"
    shift
    
    print_status "Running: $check_name"
    if "$@"; then
        print_success "$check_name passed"
        return 0
    else
        print_error "$check_name failed"
        return 1
    fi
}

# Set environment variables
export CARGO_TERM_COLOR=always
export RUST_BACKTRACE=1
export RUSTDOCFLAGS="-D warnings"

# Track overall success
OVERALL_SUCCESS=0

print_status "Starting NeoRust CI Check Pipeline..."
echo "=============================================="

# 1. Check formatting
if ! run_check "Code Formatting Check" cargo fmt --all -- --check; then
    print_warning "Running cargo fmt --all to fix formatting..."
    cargo fmt --all
    OVERALL_SUCCESS=1
fi

# 2. Run clippy
if ! run_check "Clippy Analysis" cargo clippy --all-targets --all-features -- -D warnings; then
    OVERALL_SUCCESS=1
fi

# 3. Build without default features
if ! run_check "Build (no default features)" cargo build --verbose --no-default-features; then
    OVERALL_SUCCESS=1
fi

# 4. Build with all features
if ! run_check "Build (all features)" cargo build --verbose --all-features; then
    OVERALL_SUCCESS=1
fi

# 5. Run main tests
if ! run_check "Main Test Suite" cargo test --all-features --no-fail-fast -- --nocapture; then
    OVERALL_SUCCESS=1
fi

# 6. Run CLI tests
if ! run_check "CLI Tests" bash -c "cd neo-cli && cargo test --all-features --no-fail-fast -- --nocapture"; then
    OVERALL_SUCCESS=1
fi

# 7. Security audit
if ! run_check "Security Audit" cargo audit; then
    print_warning "Security audit failed - checking if cargo-audit is installed..."
    if ! command -v cargo-audit &> /dev/null; then
        print_status "Installing cargo-audit..."
        cargo install cargo-audit
        run_check "Security Audit (retry)" cargo audit || OVERALL_SUCCESS=1
    else
        OVERALL_SUCCESS=1
    fi
fi

# 8. Dependency check
if ! run_check "Dependency Check" cargo deny check; then
    print_warning "Dependency check failed - checking if cargo-deny is installed..."
    if ! command -v cargo-deny &> /dev/null; then
        print_status "Installing cargo-deny..."
        cargo install cargo-deny
        run_check "Dependency Check (retry)" cargo deny check || OVERALL_SUCCESS=1
    else
        OVERALL_SUCCESS=1
    fi
fi

# 9. Documentation check
if ! run_check "Documentation Build" cargo doc --all-features --no-deps --document-private-items; then
    OVERALL_SUCCESS=1
fi

# 10. MSRV check (optional - requires specific Rust version)
print_status "Checking current Rust version..."
RUST_VERSION=$(rustc --version | cut -d' ' -f2)
print_status "Current Rust version: $RUST_VERSION"

echo "=============================================="

if [ $OVERALL_SUCCESS -eq 0 ]; then
    print_success "🎉 ALL CI CHECKS PASSED! Ready to push to GitHub."
else
    print_error "❌ Some CI checks failed. Please fix the issues above before pushing."
    exit 1
fi 