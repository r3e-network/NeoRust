#!/bin/bash

# Local Rust verification script based on GitHub Actions workflows
# This script runs the same checks that run in CI/CD

set -e

echo "🦀 Running Rust verification checks..."

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

# Set environment variables (matching GitHub Actions)
export CARGO_TERM_COLOR=always
export RUST_BACKTRACE=1
export NEORUST_SKIP_NETWORK_TESTS=1

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Not in project root directory. Please run from the root of the NeoRust project."
    exit 1
fi

print_section "Checking Rust toolchain"
rustc --version
cargo --version
print_success "Rust toolchain ready"

print_section "Checking formatting"
if cargo fmt --all -- --check; then
    print_success "Code formatting is correct"
else
    print_error "Code formatting issues found. Run 'cargo fmt --all' to fix."
    exit 1
fi

print_section "Running Clippy lints"
if cargo clippy --lib --tests --all-features -- -D warnings; then
    print_success "No Clippy warnings found in main library"
else
    print_error "Clippy found issues in main library. Please fix the warnings above."
    exit 1
fi

print_section "Building (no default features)"
if cargo build --verbose --no-default-features; then
    print_success "Build successful (no default features)"
else
    print_error "Build failed (no default features)"
    exit 1
fi

print_section "Building (all features)"
if cargo build --verbose --all-features; then
    print_success "Build successful (all features)"
else
    print_error "Build failed (all features)"
    exit 1
fi

print_section "Running core library tests"
if cargo test --lib --all-features --quiet; then
    print_success "Core library tests passed"
else
    print_error "Core library tests failed"
    exit 1
fi

print_section "Running documentation check"
if cargo doc --lib --no-deps --document-private-items --quiet; then
    print_success "Documentation builds successfully"
else
    print_error "Documentation build failed"
    exit 1
fi

print_section "Running cargo audit (security check)"
if command -v cargo-audit &> /dev/null; then
    if cargo audit; then
        print_success "No security vulnerabilities found"
    else
        print_warning "Security audit found issues (check .cargo/audit.toml for ignored advisories)"
    fi
else
    print_warning "cargo-audit not installed. Run 'cargo install cargo-audit' to enable security checks."
fi

print_section "Running benchmarks (if stable toolchain)"
if rustc --version | grep -q "stable"; then
    if cargo bench --no-run --quiet; then
        print_success "Benchmarks compile successfully"
    else
        print_warning "Benchmark compilation failed"
    fi
else
    print_warning "Skipping benchmarks (not on stable toolchain)"
fi

print_section "Checking basic examples compile"
basic_examples=("examples/basic" "examples/intermediate")
for example_dir in "${basic_examples[@]}"; do
    if [ -d "$example_dir" ]; then
        echo "Checking example: $example_dir"
        if (cd "$example_dir" && cargo check --quiet); then
            print_success "$example_dir compiles successfully"
        else
            print_warning "$example_dir has compilation issues (non-critical)"
        fi
    fi
done

echo -e "\n${GREEN}🎉 All Rust verification checks passed!${NC}"
echo -e "${BLUE}Your code is ready for CI/CD.${NC}"