#!/bin/bash

# NeoRust Build Script v0.4.1
# This script builds the NeoRust project with all features
# Usage: ./build.sh [--release] [--features <features>]

set -e

# Default features for v0.4.1 (AWS disabled for security)
DEFAULT_FEATURES="futures,ledger"

# Function to display help
show_help() {
    cat << EOF
NeoRust Build Script v0.4.1

Usage: $0 [OPTIONS]

OPTIONS:
    -h, --help                 Show this help message
    -r, --release             Build in release mode
    -f, --features FEATURES   Specify features (default: $DEFAULT_FEATURES)
    --all-features            Build with all available features
    --no-default-features     Build without default features
    --verbose                 Verbose output

EXAMPLES:
    $0                        # Build with default features
    $0 --release              # Release build with default features
    $0 --features "futures"   # Build with only futures feature

echo "NeoRust Build Script v0.4.1"
echo "Building NeoRust with optimized settings..."
echo ""

# Display current configuration
echo "📋 Build Configuration:"
echo "   Mode: $BUILD_MODE"
echo "   Features: $FEATURES_FLAG"
echo "Note: AWS feature is disabled in v0.4.1 for security reasons"
echo ""

# Build the project
echo "🏗️  Building NeoRust v0.4.1..."

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --features)
            FEATURES="$2"
            shift 2
            ;;
        --help|-h)
            echo "NeoRust Build Script v0.4.1"
            echo ""
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --features FEATURES    Comma-separated list of features to enable"
            echo "  --help, -h            Show this help message"
            echo ""
            echo "Available features:"
            echo "  futures               Enable async/futures support"
            echo "  ledger                Enable Ledger hardware wallet support"
            echo ""
            echo "Examples:"
            echo "  ./scripts/build.sh --features futures,ledger"
            echo "  ./scripts/build.sh --features futures"
            echo ""
            echo "Note: AWS feature is disabled in v0.4.1 for security reasons"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

echo "📦 Features: $FEATURES"

# Build main library
echo "Building main library..."
cargo build --release --features "$FEATURES"

# Build CLI
echo "Building CLI..."
cd neo-cli
cargo build --release
cd ..

# Build examples
echo "Building examples..."
for example_dir in examples/*/; do
    if [ -d "$example_dir" ]; then
        echo "Building $(basename "$example_dir")..."
        cd "$example_dir"
        cargo build --release
        cd ../..
    fi
done

echo "✅ Build completed successfully!"
echo "📊 Build summary:"
echo "   - Main library: ✅"
echo "   - CLI tool: ✅"
echo "   - Examples: ✅"
echo "   - Features: $FEATURES" 