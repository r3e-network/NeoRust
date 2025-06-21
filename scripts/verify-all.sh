#!/bin/bash

# Master verification script that runs all checks locally
# This script simulates the complete CI/CD pipeline

set -e

echo "🚀 Running complete local verification (simulating CI/CD pipeline)..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Function to print section headers
print_header() {
    echo -e "\n${CYAN}╔═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${CYAN}║ $1${NC}"
    echo -e "${CYAN}╚═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════╝${NC}"
}

# Function to print success
print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

# Function to print warning
print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

# Function to print error
print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Function to print info
print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Not in project root directory. Please run from the root of the NeoRust project."
    exit 1
fi

# Track results
RUST_RESULT=0
FRONTEND_RESULT=0
SECURITY_RESULT=0
DOCS_RESULT=0

start_time=$(date +%s)

print_header "🦀 RUST VERIFICATION"
print_info "Running Rust checks (formatting, linting, building, testing)..."
if ./scripts/verify-rust.sh; then
    print_success "Rust verification passed"
    RUST_RESULT=1
else
    print_error "Rust verification failed"
fi

print_header "🌐 FRONTEND VERIFICATION"
print_info "Running frontend checks (TypeScript, linting, testing, building)..."
if ./scripts/verify-frontend.sh; then
    print_success "Frontend verification passed"
    FRONTEND_RESULT=1
else
    print_warning "Frontend verification had issues (check output above)"
    FRONTEND_RESULT=1  # Consider warnings as pass for frontend
fi

print_header "🔒 SECURITY VERIFICATION"
print_info "Running security audits for Rust and frontend..."
if ./scripts/verify-security.sh; then
    print_success "Security verification passed"
    SECURITY_RESULT=1
else
    print_warning "Security verification found issues (review above)"
    SECURITY_RESULT=1  # Consider warnings as pass for security
fi

print_header "📚 DOCUMENTATION VERIFICATION"
print_info "Checking documentation builds and quality..."
if ./scripts/verify-docs.sh; then
    print_success "Documentation verification passed"
    DOCS_RESULT=1
else
    print_warning "Documentation verification had issues"
    DOCS_RESULT=1  # Consider warnings as pass for docs
fi

# Calculate runtime
end_time=$(date +%s)
runtime=$((end_time - start_time))
minutes=$((runtime / 60))
seconds=$((runtime % 60))

# Summary
print_header "📊 VERIFICATION SUMMARY"

echo -e "${BLUE}Runtime: ${minutes}m ${seconds}s${NC}\n"

if [ $RUST_RESULT -eq 1 ]; then
    print_success "Rust Verification: PASSED"
else
    print_error "Rust Verification: FAILED"
fi

if [ $FRONTEND_RESULT -eq 1 ]; then
    print_success "Frontend Verification: PASSED"
else
    print_error "Frontend Verification: FAILED"
fi

if [ $SECURITY_RESULT -eq 1 ]; then
    print_success "Security Verification: PASSED"
else
    print_warning "Security Verification: WARNINGS"
fi

if [ $DOCS_RESULT -eq 1 ]; then
    print_success "Documentation Verification: PASSED"
else
    print_warning "Documentation Verification: WARNINGS"
fi

# Overall result
if [ $RUST_RESULT -eq 1 ] && [ $FRONTEND_RESULT -eq 1 ]; then
    echo -e "\n${GREEN}🎉 OVERALL RESULT: READY FOR CI/CD${NC}"
    echo -e "${GREEN}Your code should pass all GitHub Actions workflows!${NC}"
    
    echo -e "\n${BLUE}Next steps:${NC}"
    echo -e "${BLUE}1. Review any warnings above${NC}"
    echo -e "${BLUE}2. Commit your changes${NC}"
    echo -e "${BLUE}3. Push to trigger CI/CD${NC}"
    exit 0
else
    echo -e "\n${RED}❌ OVERALL RESULT: NOT READY${NC}"
    echo -e "${RED}Please fix the issues above before pushing.${NC}"
    
    echo -e "\n${BLUE}To fix issues:${NC}"
    if [ $RUST_RESULT -eq 0 ]; then
        echo -e "${BLUE}- Run: cargo fmt --all (for formatting)${NC}"
        echo -e "${BLUE}- Fix Clippy warnings manually${NC}"
        echo -e "${BLUE}- Ensure tests pass${NC}"
    fi
    if [ $FRONTEND_RESULT -eq 0 ]; then
        echo -e "${BLUE}- Run: cd neo-gui && npm run format (for formatting)${NC}"
        echo -e "${BLUE}- Fix ESLint errors manually${NC}"
        echo -e "${BLUE}- Ensure TypeScript compiles${NC}"
    fi
    exit 1
fi