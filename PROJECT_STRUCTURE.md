# NeoRust SDK Project Structure

This document outlines the professional organization of the NeoRust SDK project, designed for maintainability, scalability, and ease of use.

## 📁 Directory Structure

```
NeoRust/
├── .github/                      # GitHub specific files
│   ├── workflows/                # CI/CD workflows
│   │   ├── ci.yml               # Main CI pipeline
│   │   ├── release.yml          # Release automation
│   │   ├── security.yml         # Security scanning
│   │   └── docs.yml             # Documentation building
│   ├── ISSUE_TEMPLATE/          # Issue templates
│   ├── PULL_REQUEST_TEMPLATE.md # PR template
│   └── dependabot.yml           # Dependency updates
│
├── crates/                       # Core SDK crates (workspace members)
│   ├── neo3/                    # Main SDK crate
│   │   ├── src/
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── neo3-builder/            # Transaction & script building
│   │   ├── src/
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── neo3-crypto/             # Cryptographic operations
│   │   ├── src/
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── neo3-types/              # Core types and primitives
│   │   ├── src/
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── neo3-rpc/                # RPC client implementation
│   │   ├── src/
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── neo3-wallets/            # Wallet management
│   │   ├── src/
│   │   ├── Cargo.toml
│   │   └── README.md
│   ├── neo3-contracts/          # Smart contract interactions
│   │   ├── src/
│   │   ├── Cargo.toml
│   │   └── README.md
│   └── neo3-macros/             # Procedural macros
│       ├── src/
│       ├── Cargo.toml
│       └── README.md
│
├── examples/                     # Example applications
│   ├── basic/                   # Basic usage examples
│   │   ├── 01_connect.rs
│   │   ├── 02_wallet.rs
│   │   ├── 03_transfer.rs
│   │   └── README.md
│   ├── intermediate/            # Intermediate examples
│   │   ├── contract_deploy.rs
│   │   ├── token_operations.rs
│   │   └── README.md
│   └── advanced/                # Advanced examples
│       ├── defi_integration.rs
│       ├── nft_marketplace.rs
│       └── README.md
│
├── apps/                        # Applications
│   ├── neo-cli/                # Command-line interface
│   │   ├── src/
│   │   ├── Cargo.toml
│   │   └── README.md
│   └── neo-gui/                # Desktop GUI application
│       ├── src/
│       ├── src-tauri/
│       ├── package.json
│       ├── Cargo.toml
│       └── README.md
│
├── docs/                        # Documentation
│   ├── book/                   # mdBook documentation
│   │   ├── src/
│   │   └── book.toml
│   ├── api/                    # API documentation
│   ├── guides/                 # User guides
│   │   ├── getting-started.md
│   │   ├── wallet-guide.md
│   │   └── contract-guide.md
│   └── architecture/           # Architecture docs
│       ├── design.md
│       └── security.md
│
├── tests/                      # Integration tests
│   ├── common/                # Common test utilities
│   ├── integration/           # Integration test suites
│   └── fixtures/              # Test fixtures
│
├── benches/                   # Benchmarks
│   ├── crypto.rs
│   ├── transaction.rs
│   └── rpc.rs
│
├── scripts/                   # Development scripts
│   ├── build.sh              # Build script
│   ├── test.sh               # Test runner
│   ├── release.sh            # Release script
│   └── check.sh              # Pre-commit checks
│
├── .cargo/                    # Cargo configuration
│   └── config.toml
│
├── Cargo.toml                 # Workspace manifest
├── Cargo.lock                 # Lock file
├── README.md                  # Main README
├── LICENSE-MIT               # MIT License
├── LICENSE-APACHE            # Apache License
├── CONTRIBUTING.md           # Contributing guidelines
├── CODE_OF_CONDUCT.md        # Code of conduct
├── CHANGELOG.md              # Change log
├── SECURITY.md               # Security policy
├── rust-toolchain.toml       # Rust toolchain specification
└── deny.toml                 # Cargo deny configuration
```

## 🎯 Design Principles

### 1. **Modular Architecture**
- Each major component is a separate crate
- Clear separation of concerns
- Minimal inter-crate dependencies
- Well-defined public APIs

### 2. **Workspace Organization**
- Single workspace root with multiple member crates
- Shared dependencies managed at workspace level
- Consistent versioning across crates
- Unified build and test commands

### 3. **Professional Naming**
- Crates prefixed with `neo3-` for clarity
- Consistent naming conventions
- Clear, descriptive module names
- No abbreviations in public APIs

### 4. **Documentation First**
- Every crate has its own README
- Comprehensive rustdoc comments
- Example code in documentation
- Architecture decision records

## 📦 Crate Responsibilities

### `neo3` - Main SDK Crate
- Re-exports from other crates
- High-level convenience APIs
- Prelude module for common imports
- Feature flag management

### `neo3-types` - Core Types
- Basic primitives (Address, ScriptHash, etc.)
- Transaction types
- Block and header types
- Serialization traits

### `neo3-crypto` - Cryptography
- Key generation and management
- Signing and verification
- Hash functions
- Encryption/decryption

### `neo3-rpc` - RPC Client
- HTTP/WebSocket providers
- Request/response types
- Connection pooling
- Error handling

### `neo3-builder` - Builders
- Transaction builder
- Script builder
- Witness builder
- Contract invocation builder

### `neo3-wallets` - Wallet Management
- NEP-6 wallet support
- Account management
- Key storage
- Hardware wallet integration

### `neo3-contracts` - Smart Contracts
- Contract deployment
- Method invocation
- Event monitoring
- Standard contracts (NEP-17, NEP-11)

### `neo3-macros` - Procedural Macros
- Derive macros for common traits
- Attribute macros for contracts
- Function-like macros for DSL

## 🔧 Development Workflow

### Building
```bash
# Build all crates
cargo build --workspace

# Build specific crate
cargo build -p neo3-rpc

# Build with all features
cargo build --all-features
```

### Testing
```bash
# Run all tests
cargo test --workspace

# Run specific crate tests
cargo test -p neo3-crypto

# Run integration tests
cargo test --test '*'
```

### Documentation
```bash
# Build and open documentation
cargo doc --open --no-deps

# Build with private items
cargo doc --document-private-items
```

## 📋 Best Practices

### Code Organization
1. Keep modules focused and single-purpose
2. Use `mod.rs` for module organization
3. Separate public API from implementation
4. Group related functionality together

### Error Handling
1. Use custom error types per crate
2. Implement std::error::Error
3. Provide context in error messages
4. Use Result<T, E> consistently

### Testing
1. Unit tests in src files
2. Integration tests in tests/
3. Doc tests for examples
4. Property-based testing where appropriate

### Documentation
1. Document all public items
2. Include examples in docs
3. Link to related items
4. Explain "why" not just "what"

## 🚀 Release Process

1. **Version Bump**
   - Update version in all Cargo.toml files
   - Update CHANGELOG.md
   - Update documentation

2. **Quality Checks**
   - Run full test suite
   - Check documentation builds
   - Run clippy and fmt
   - Security audit

3. **Release**
   - Tag release in git
   - Publish to crates.io
   - Update GitHub release
   - Announce on channels

## 🔐 Security Considerations

- Regular dependency audits
- Security policy in SECURITY.md
- Responsible disclosure process
- Security-focused code reviews
- Fuzzing for critical components

## 📈 Performance Guidelines

- Benchmark critical paths
- Profile before optimizing
- Document performance characteristics
- Consider zero-copy where possible
- Async-first design

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines on:
- Code style
- Commit messages
- Pull request process
- Issue reporting
- Development setup