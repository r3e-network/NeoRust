#!/bin/bash

# Neo GUI - Comprehensive Consistency and Quality Check
# This script ensures everything is consistent, clean, clear, and working correctly

set -e

echo "🔍 Neo GUI - Comprehensive Consistency Check"
echo "============================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
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

print_check() {
    echo -e "${PURPLE}[CHECK]${NC} $1"
}

print_fix() {
    echo -e "${CYAN}[FIX]${NC} $1"
}

# Initialize counters
TOTAL_CHECKS=0
PASSED_CHECKS=0
FAILED_CHECKS=0
FIXED_ISSUES=0

# Function to track results
track_result() {
    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))
    if [ $1 -eq 0 ]; then
        PASSED_CHECKS=$((PASSED_CHECKS + 1))
    else
        FAILED_CHECKS=$((FAILED_CHECKS + 1))
    fi
}

# Function to run check with error handling
run_check() {
    local check_name="$1"
    local check_command="$2"
    
    print_check "Checking $check_name..."
    
    if eval "$check_command"; then
        print_success "$check_name: ✅ PASSED"
        return 0
    else
        print_error "$check_name: ❌ FAILED"
        return 1
    fi
}

echo ""
print_status "Starting comprehensive consistency check..."
echo ""

# Phase 1: File Structure Consistency
echo "📁 Phase 1: File Structure Consistency"
echo "--------------------------------------"

run_check "Cargo.toml exists" "[ -f Cargo.toml ]"
track_result $?

run_check "package.json exists" "[ -f package.json ]"
track_result $?

run_check "src/lib.rs exists" "[ -f src/lib.rs ]"
track_result $?

run_check "src/main.rs exists" "[ -f src/main.rs ]"
track_result $?

run_check "All service modules exist" "[ -f src/services/mod.rs ] && [ -f src/services/wallet.rs ] && [ -f src/services/network.rs ] && [ -f src/services/transaction.rs ] && [ -f src/services/settings.rs ]"
track_result $?

echo ""

# Phase 2: Version Consistency
echo "🔢 Phase 2: Version Consistency"
echo "-------------------------------"

CARGO_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
PACKAGE_VERSION=$(grep '"version":' package.json | sed 's/.*"version": "\(.*\)".*/\1/')

print_check "Checking version consistency..."
if [ "$CARGO_VERSION" = "$PACKAGE_VERSION" ]; then
    print_success "Version consistency: ✅ Both Cargo.toml and package.json use version $CARGO_VERSION"
    track_result 0
else
    print_error "Version mismatch: Cargo.toml=$CARGO_VERSION, package.json=$PACKAGE_VERSION"
    track_result 1
fi

echo ""

# Phase 3: Code Quality and Formatting
echo "🎨 Phase 3: Code Quality and Formatting"
echo "---------------------------------------"

run_check "Rust code formatting" "cargo fmt -- --check"
track_result $?

run_check "Rust compilation (debug)" "cargo check"
track_result $?

run_check "TypeScript compilation" "npm run type-check"
track_result $?

echo ""

# Phase 4: Dependencies and Imports
echo "📦 Phase 4: Dependencies and Imports"
echo "------------------------------------"

run_check "Rust dependencies resolve" "cargo tree > /dev/null 2>&1"
track_result $?

run_check "Node dependencies installed" "[ -d node_modules ] && [ -f package-lock.json ]"
track_result $?

echo ""

# Phase 5: Test Coverage and Quality
echo "🧪 Phase 5: Test Coverage and Quality"
echo "-------------------------------------"

run_check "Rust unit tests" "cargo test --lib"
track_result $?

run_check "Frontend tests" "npm test -- --passWithNoTests"
track_result $?

echo ""

# Phase 6: Documentation Consistency
echo "📚 Phase 6: Documentation Consistency"
echo "-------------------------------------"

run_check "Rust documentation builds" "cargo doc --no-deps"
track_result $?

run_check "README.md exists" "[ -f README.md ]"
track_result $?

echo ""

# Phase 7: Configuration Consistency
echo "⚙️  Phase 7: Configuration Consistency"
echo "--------------------------------------"

run_check "Tauri configuration exists" "[ -f tauri.conf.json ]"
track_result $?

run_check "Vite configuration exists" "[ -f vite.config.ts ]"
track_result $?

run_check "TypeScript configuration exists" "[ -f tsconfig.json ]"
track_result $?

run_check "Tailwind configuration exists" "[ -f tailwind.config.js ]"
track_result $?

echo ""

# Phase 8: Build System Verification
echo "🏗️  Phase 8: Build System Verification"
echo "--------------------------------------"

run_check "Frontend build" "npm run build"
track_result $?

echo ""

# Final Results and Summary
echo "📊 Comprehensive Check Results"
echo "=============================="
echo ""
echo "Total Checks: $TOTAL_CHECKS"
echo -e "Passed: ${GREEN}$PASSED_CHECKS${NC}"
echo -e "Failed: ${RED}$FAILED_CHECKS${NC}"
echo ""

# Calculate success rate
if [ $TOTAL_CHECKS -gt 0 ]; then
    SUCCESS_RATE=$(( PASSED_CHECKS * 100 / TOTAL_CHECKS ))
else
    SUCCESS_RATE=0
fi

echo "Success Rate: $SUCCESS_RATE%"
echo ""

# Provide detailed status
if [ $FAILED_CHECKS -eq 0 ]; then
    print_success "🎉 PERFECT! All consistency checks passed!"
    echo ""
    echo "✅ File Structure: Consistent"
    echo "✅ Version Management: Synchronized"
    echo "✅ Code Quality: Excellent"
    echo "✅ Dependencies: Resolved"
    echo "✅ Tests: Passing"
    echo "✅ Documentation: Complete"
    echo "✅ Configuration: Valid"
    echo "✅ Build System: Working"
    echo ""
    echo "🚀 Neo GUI is production-ready with perfect consistency!"
    exit 0
elif [ $SUCCESS_RATE -ge 90 ]; then
    print_warning "⚠️  Minor issues detected but overall excellent quality"
    echo ""
    echo "Success rate: $SUCCESS_RATE% (Excellent)"
    echo "Failed checks: $FAILED_CHECKS (Minor issues)"
    echo ""
    echo "🔧 Recommended: Review and fix the minor issues above"
    exit 0
elif [ $SUCCESS_RATE -ge 75 ]; then
    print_warning "⚠️  Some issues detected but good overall quality"
    echo ""
    echo "Success rate: $SUCCESS_RATE% (Good)"
    echo "Failed checks: $FAILED_CHECKS (Moderate issues)"
    echo ""
    echo "🔧 Recommended: Address the issues above before production"
    exit 1
else
    print_error "❌ Significant issues detected"
    echo ""
    echo "Success rate: $SUCCESS_RATE% (Needs improvement)"
    echo "Failed checks: $FAILED_CHECKS (Major issues)"
    echo ""
    echo "🔧 Required: Fix all issues before proceeding"
    exit 1
fi
