#!/bin/bash

# NeoRust Build Script v0.4.0
# Builds the NeoRust SDK with specified features

set -e

# Default features for v0.4.0 (AWS disabled for security)
FEATURES="futures,ledger"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --features)
            FEATURES="$2"
            shift 2
            ;;
        --help|-h)
            echo "NeoRust Build Script v0.4.0"
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
            echo "Note: AWS feature is disabled in v0.4.0 for security reasons"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

echo "🏗️  Building NeoRust v0.4.0..."
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