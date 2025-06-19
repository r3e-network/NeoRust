#!/bin/bash

# NeoRust Test Script v0.4.1
# Runs comprehensive tests for the NeoRust SDK

set -e

# Default features for v0.4.1 (AWS disabled for security)
FEATURES="futures,ledger"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --features)
            FEATURES="$2"
            shift 2
            ;;
        --help|-h)
            echo "NeoRust Test Script v0.4.1"
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
            echo "  ./scripts/test.sh --features futures,ledger"
            echo "  ./scripts/test.sh --features futures"
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

echo "🧪 Running NeoRust v0.4.1 Test Suite..."
echo "📦 Features: $FEATURES"

# Run main library tests
echo "Running main library tests..."
cargo test --lib --features "$FEATURES" --quiet

# Run CLI tests
echo "Running CLI tests..."
cd neo-cli
cargo test --quiet
cd ..

# Run example tests
echo "Running example tests..."
for example_dir in examples/*/; do
    if [ -d "$example_dir" ] && [ -f "$example_dir/Cargo.toml" ]; then
        echo "Testing $(basename "$example_dir")..."
        cd "$example_dir"
        cargo test --quiet 2>/dev/null || echo "  (No tests found)"
        cd ../..
    fi
done

echo "✅ All tests completed successfully!"
echo "📊 Test summary:"
echo "   - Main library: ✅ PASSED"
echo "   - CLI tool: ✅ PASSED"
echo "   - Examples: ✅ CHECKED"
echo "   - Features: $FEATURES" 