# Tests

This directory contains comprehensive tests for the NeoRust SDK, organized by type and scope.

## 📁 Test Organization

### Unit Tests (`unit/`)
Component-specific tests that test individual modules in isolation:
- `crypto/` - Cryptographic operations tests
- `rpc/` - RPC client functionality tests  
- `types/` - Core type validation tests
- `builder/` - Transaction and script builder tests

### Integration Tests (`integration/`)
End-to-end tests that verify component interactions:
- `wallet_operations.rs` - Complete wallet workflow tests
- `transaction_lifecycle.rs` - Full transaction creation to confirmation
- `contract_interaction.rs` - Smart contract deployment and invocation
- `network_resilience.rs` - Network failure and recovery scenarios

### Common Test Utilities (`common/`)
Shared testing infrastructure and helpers:
- `test_utils.rs` - Common testing utilities and helpers
- `mock_provider.rs` - Mock RPC provider for offline testing
- `test_accounts.rs` - Predefined test accounts and keys
- `assertions.rs` - Custom assertion macros

### Test Fixtures (`fixtures/`)
Static test data and mock responses:
- `rpc_responses/` - Sample RPC response data
- `transactions/` - Example transaction data
- `contracts/` - Test contract NEF files and manifests
- `wallets/` - Test wallet files

## 🚀 Running Tests

### All Tests
```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run with parallel execution
cargo test -- --test-threads=4
```

### Specific Test Categories
```bash
# Unit tests only
cargo test --test unit_*

# Integration tests only  
cargo test --test integration_*

# Specific module tests
cargo test crypto
cargo test rpc
```

### Test Features
```bash
# Run tests with network access
cargo test --features network-tests

# Run tests with hardware wallet support
cargo test --features hardware-tests

# Run all feature tests
cargo test --all-features
```

## 🧪 Test Types

### Unit Tests
- **Fast execution** - Run in milliseconds
- **No external dependencies** - Fully isolated
- **High coverage** - Test edge cases and error conditions
- **Deterministic** - Same results every time

### Integration Tests
- **Real network interaction** - Test against TestNet
- **Component interaction** - Verify module integration
- **End-to-end workflows** - Complete user scenarios
- **Performance validation** - Response time and throughput

### Property-Based Tests
- **Random input generation** - Test with varied inputs
- **Invariant verification** - Ensure properties hold
- **Edge case discovery** - Find unexpected scenarios
- **Regression prevention** - Catch breaking changes

## 🔧 Test Configuration

### Environment Variables
```bash
# Network endpoints for testing
export NEO_TESTNET_URL="https://testnet1.neo.org:443"
export NEO_MAINNET_URL="https://mainnet1.neo.org:443"

# Test account configuration
export TEST_PRIVATE_KEY="your-test-private-key"
export TEST_CONTRACT_HASH="test-contract-hash"

# Test behavior configuration  
export RUST_LOG=debug
export TEST_TIMEOUT=30
```

### Test Networks
- **TestNet**: Default for integration tests
- **Local**: Neo-Express for development
- **MainNet**: Limited read-only tests only

## 📊 Coverage

### Current Coverage Targets
- **Unit Tests**: >90% line coverage
- **Integration Tests**: >80% scenario coverage
- **Critical Paths**: 100% coverage for security-sensitive code

### Coverage Reports
```bash
# Generate coverage report
cargo tarpaulin --out Html

# Coverage with specific features
cargo tarpaulin --features network-tests --out Html

# Open coverage report
open tarpaulin-report.html
```

## 🔒 Security Testing

### Test Categories
- **Input Validation**: Malformed data handling
- **Key Management**: Private key security
- **Network Security**: TLS and certificate validation
- **Transaction Security**: Signature validation and replay protection

### Security Test Commands
```bash
# Run security-focused tests
cargo test security

# Test with sanitizers
RUSTFLAGS="-Z sanitizer=address" cargo test

# Memory leak detection
valgrind cargo test
```

## 🚀 Performance Testing

### Benchmarks
```bash
# Run benchmarks
cargo bench

# Specific benchmark suites
cargo bench --bench crypto
cargo bench --bench rpc
```

### Load Testing
- **Concurrent connections** - Multiple RPC clients
- **Transaction throughput** - Batch transaction processing
- **Memory usage** - Long-running process validation
- **Network resilience** - Connection failure recovery

## 🤝 Contributing Tests

### Adding New Tests

1. **Choose appropriate category**:
   - Unit tests for isolated functionality
   - Integration tests for component interaction
   - End-to-end tests for user workflows

2. **Follow naming conventions**:
   - `test_` prefix for test functions
   - Descriptive names: `test_wallet_creation_with_valid_entropy`
   - Module organization: `crypto::test_signature_verification`

3. **Include proper documentation**:
   ```rust
   /// Tests wallet creation with various entropy sources
   /// 
   /// This test verifies that wallets can be created with:
   /// - System entropy (default)
   /// - User-provided entropy
   /// - Hardware entropy sources
   #[test]
   fn test_wallet_creation_with_entropy() {
       // Test implementation
   }
   ```

### Test Quality Guidelines

- **Arrange-Act-Assert**: Clear test structure
- **Single responsibility**: One concept per test
- **Descriptive assertions**: Clear failure messages
- **Cleanup**: Proper resource cleanup
- **Deterministic**: Reproducible results

### Mock and Fixture Guidelines

- **Realistic data**: Use production-like test data
- **Edge cases**: Include boundary conditions
- **Error scenarios**: Test failure modes
- **Version compatibility**: Support multiple Neo N3 versions

## 🔍 Debugging Tests

### Failed Tests
```bash
# Run specific failed test
cargo test test_name -- --exact

# Show test output
cargo test test_name -- --nocapture

# Stop on first failure
cargo test -- --fail-fast
```

### Test Debugging
```bash
# Debug mode with logging
RUST_LOG=debug cargo test

# Run under debugger
rust-gdb --args cargo test test_name

# Memory debugging
valgrind cargo test test_name
```

## 📈 Continuous Integration

### GitHub Actions
- **Automated testing** on every PR
- **Multi-platform testing** (Linux, Windows, macOS)
- **Coverage reporting** with codecov
- **Security scanning** with cargo-audit

### Test Environments
- **Clean environments** for each test run
- **Dependency caching** for faster execution
- **Parallel execution** for improved performance
- **Artifact preservation** for debugging

## 🔗 Additional Resources

- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Cargo Test Documentation](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
- [Neo N3 Testing Best Practices](https://docs.neo.org/develop/test)
- [Property-Based Testing in Rust](https://github.com/AltSysrq/proptest)