# NeoRust SDK Project Structure

This document outlines the professional organization of the NeoRust SDK project, designed for maintainability, scalability, and ease of use.

## 📁 Directory Structure

```
NeoRust/
├── .github/                      # GitHub specific files
│   ├── workflows/                # CI/CD workflows
│   │   ├── build-test.yml       # Main CI pipeline
│   │   ├── release.yml          # Release automation
│   ├── ISSUE_TEMPLATE/          # Issue templates
│   ├── PULL_REQUEST_TEMPLATE.md # PR template
│   └── dependabot.yml           # Dependency updates
│
├── src/                          # Main `neo3` library crate
│   ├── neo_builder/              # Transaction & script building
│   ├── neo_clients/              # JSON-RPC + transports (HTTP/WS/IPC)
│   ├── neo_codec/                # Serialization/deserialization
│   ├── neo_config/               # Network + runtime configuration
│   ├── neo_contract/             # Smart contract interactions
│   ├── neo_crypto/               # Cryptographic operations
│   ├── neo_error/                # Error types (legacy + unified)
│   ├── neo_fs/                   # NeoFS integration
│   ├── neo_protocol/             # Protocol/domain types
│   ├── neo_sgx/                  # SGX support (optional)
│   ├── neo_types/                # Core types and primitives
│   ├── neo_utils/                # Shared utilities
│   ├── neo_wallets/              # Wallet management
│   ├── neo_x/                    # Neo X / EVM compatibility layer
│   ├── sdk/                      # High-level ergonomic API (`sdk::Neo`)
│   ├── lib.rs                    # Crate root + re-exports
│   └── prelude.rs                # Convenience prelude exports
│
├── examples/                     # Example applications
│   ├── basic/                   # Basic usage examples
│   │   ├── src/
│   │   └── Cargo.toml
│   ├── intermediate/            # Intermediate examples
│   │   ├── src/
│   │   └── Cargo.toml
│   └── advanced/                # Advanced examples
│       ├── src/
│       └── Cargo.toml
│
├── neo-cli/                      # Command-line interface (workspace member)
│   ├── src/
│   ├── templates/                # Project generator templates
│   └── Cargo.toml
├── neo-gui-rs/                   # Native Rust GUI shell (workspace member)
│   ├── src/
│   └── Cargo.toml
├── neo-gui/                      # Legacy React/Tauri GUI (not a Cargo member)
│
├── docs/                        # Documentation
│   ├── src/                     # mdBook source content
│   ├── book/                    # Generated mdBook output (tracked)
│   └── guides/                  # Additional guides
│
├── tests/                      # Integration tests
│   └── *.rs
│
├── benches/                   # Benchmarks
│   └── *.rs
│
├── .cargo/                    # Cargo configuration
│   └── config.toml
├── vendor/                    # Vendored/patched deps (see `[patch.crates-io]`)
├── config/                    # App configuration (dev/prod)
├── assets/                    # Branding assets
├── website/                   # Project website sources
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

> Note: A multi-crate split under `crates/` is planned/tracked in `WORKSPACE_REORGANIZATION.md`.

## 🎯 Design Principles

### 1. **Modular Architecture**
- Major components live as modules under `src/` in the `neo3` crate
- Workspace applications (`neo-cli`, `neo-gui-rs`) are separate crates
- Clear separation of concerns
- Minimal cross-module coupling
- Well-defined public APIs

### 2. **Workspace Organization**
- Single workspace root with multiple member crates
- Shared dependencies managed at workspace level
- Consistent versioning across crates
- Unified build and test commands

### 3. **Professional Naming**
- Consistent naming conventions across modules and crates
- Core SDK modules prefixed with `neo_` for clarity (e.g., `neo_clients`, `neo_builder`)
- Clear, descriptive module names
- No abbreviations in public APIs

### 4. **Documentation First**
- Every crate has its own README
- Comprehensive rustdoc comments
- Example code in documentation
- Architecture decision records

## 📦 Responsibilities

### `neo3` (library crate)
- Contains the full SDK as internal modules under `src/`
- Exposes a high-level ergonomic API under `src/sdk/`
- Provides a `prelude` for convenient imports

### Workspace applications
- `neo-cli/`: CLI for common operations (wallets, contracts, NeoFS, etc.)
- `neo-gui-rs/`: Native Rust GUI shell for the SDK
- `neo-gui/`: Legacy React/Tauri GUI for historical parity

### Core modules (selected)
- `neo_clients`: JSON-RPC client + HTTP/WS/IPC transports
- `neo_builder`: transaction/script building + gas estimation
- `neo_wallets`: NEP-6 wallets + key management
- `neo_contract`: contract invocation + standard contracts

## 🔧 Development Workflow

### Building
```bash
# Build all crates
cargo build --workspace

# Build specific crate
cargo build -p neo3
cargo build -p neo-cli

# Build with all features
cargo build --all-features
```

### Testing
```bash
# Run all tests
cargo test --workspace

# Run specific crate tests
cargo test -p neo3
cargo test -p neo-cli

# Run integration tests
cargo test --tests
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
