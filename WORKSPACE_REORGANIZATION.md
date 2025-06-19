# NeoRust Workspace Reorganization Plan

This document outlines the steps to reorganize the NeoRust SDK into a professional, maintainable structure.

## 🎯 Goals

1. **Clear Separation**: Each component in its own crate
2. **Professional Naming**: Consistent `neo3-` prefix
3. **Minimal Dependencies**: Reduce coupling between crates
4. **Better Organization**: Logical grouping of functionality
5. **Easier Maintenance**: Clear ownership and boundaries

## 📋 Reorganization Steps

### Phase 1: Create New Crate Structure

```bash
# Create crates directory
mkdir -p crates

# Create individual crate directories
mkdir -p crates/neo3
mkdir -p crates/neo3-types
mkdir -p crates/neo3-crypto
mkdir -p crates/neo3-rpc
mkdir -p crates/neo3-builder
mkdir -p crates/neo3-wallets
mkdir -p crates/neo3-contracts
mkdir -p crates/neo3-macros
```

### Phase 2: Move and Refactor Code

#### 1. **neo3-types** (Foundation)
Move from `src/neo_types/` to `crates/neo3-types/src/`
- Basic types: Address, ScriptHash, etc.
- Transaction types
- Block types
- Contract types
- Serialization utilities

#### 2. **neo3-crypto** (Cryptography)
Move from `src/neo_crypto/` to `crates/neo3-crypto/src/`
- Key generation
- Signing/verification
- Hash functions
- Encryption

#### 3. **neo3-rpc** (Network Communication)
Move from `src/neo_clients/` to `crates/neo3-rpc/src/`
- RPC client
- HTTP/WebSocket providers
- Request/response types
- Connection management

#### 4. **neo3-builder** (Construction)
Move from `src/neo_builder/` to `crates/neo3-builder/src/`
- Transaction builder
- Script builder
- Witness construction

#### 5. **neo3-wallets** (Wallet Management)
Move from `src/neo_wallets/` to `crates/neo3-wallets/src/`
- Wallet creation/management
- NEP-6 support
- Account handling
- Key storage

#### 6. **neo3-contracts** (Smart Contracts)
Move from `src/neo_contract/` to `crates/neo3-contracts/src/`
- Contract deployment
- Method invocation
- Standard contracts (NEP-17, NEP-11)
- Event handling

#### 7. **neo3** (Main SDK)
Update `src/` to be a facade that re-exports from other crates
- Prelude module
- High-level APIs
- Feature management

### Phase 3: Update Workspace Configuration

```toml
# Root Cargo.toml
[workspace]
resolver = "2"
members = [
    "crates/neo3",
    "crates/neo3-types",
    "crates/neo3-crypto",
    "crates/neo3-rpc",
    "crates/neo3-builder",
    "crates/neo3-wallets",
    "crates/neo3-contracts",
    "crates/neo3-macros",
    "apps/neo-cli",
    "apps/neo-gui",
]

[workspace.package]
version = "0.5.0"
authors = ["R3E Network <team@r3e.network>"]
edition = "2021"
license = "MIT OR Apache-2.0"
repository = "https://github.com/R3E-Network/NeoRust"
homepage = "https://neorust.org"
documentation = "https://docs.rs/neo3"

[workspace.dependencies]
# Shared dependencies with unified versions
tokio = { version = "1.45", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
# ... other shared dependencies
```

### Phase 4: Individual Crate Configuration

#### Example: `crates/neo3-types/Cargo.toml`
```toml
[package]
name = "neo3-types"
version.workspace = true
authors.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
description = "Core types and primitives for Neo N3 blockchain"

[dependencies]
serde.workspace = true
serde_json.workspace = true
thiserror.workspace = true
hex = "0.4"
base64 = "0.21"

[dev-dependencies]
tokio.workspace = true
criterion = "0.5"

[[bench]]
name = "serialization"
harness = false
```

### Phase 5: Update Import Paths

Replace all internal imports:
```rust
// Old
use crate::neo_types::ScriptHash;

// New
use neo3_types::ScriptHash;
```

### Phase 6: Create Crate-Specific Documentation

Each crate should have:
1. **README.md** - Overview and usage
2. **CHANGELOG.md** - Version history
3. **src/lib.rs** - Comprehensive module docs
4. **examples/** - Usage examples

### Phase 7: Reorganize Examples

```
examples/
├── basic/
│   ├── 01_connect_to_network.rs
│   ├── 02_create_wallet.rs
│   ├── 03_check_balance.rs
│   ├── 04_send_transaction.rs
│   └── README.md
├── intermediate/
│   ├── deploy_contract.rs
│   ├── invoke_contract.rs
│   ├── nep17_operations.rs
│   ├── multi_sig_wallet.rs
│   └── README.md
└── advanced/
    ├── defi_swap.rs
    ├── nft_marketplace.rs
    ├── oracle_integration.rs
    ├── cross_chain_bridge.rs
    └── README.md
```

### Phase 8: Update CI/CD Workflows

Create comprehensive GitHub Actions workflows:

#### `.github/workflows/ci.yml`
```yaml
name: CI

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

env:
  RUST_BACKTRACE: 1

jobs:
  test:
    name: Test
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable, beta]
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: ${{ matrix.rust }}
      - uses: Swatinem/rust-cache@v2
      - run: cargo test --workspace --all-features

  lint:
    name: Lint
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - run: cargo fmt --all -- --check
      - run: cargo clippy --workspace --all-features -- -D warnings

  docs:
    name: Documentation
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo doc --workspace --no-deps --all-features
```

## 🔄 Migration Benefits

1. **Better Modularity**: Each crate can be used independently
2. **Clearer Dependencies**: Explicit inter-crate dependencies
3. **Easier Testing**: Test each crate in isolation
4. **Better Documentation**: Crate-specific docs
5. **Flexible Versioning**: Version crates independently
6. **Improved Compilation**: Only rebuild changed crates
7. **Professional Structure**: Industry-standard organization

## 📊 Dependency Graph

```
neo3 (main crate)
├── neo3-types
├── neo3-crypto
│   └── neo3-types
├── neo3-rpc
│   └── neo3-types
├── neo3-builder
│   ├── neo3-types
│   └── neo3-crypto
├── neo3-wallets
│   ├── neo3-types
│   └── neo3-crypto
└── neo3-contracts
    ├── neo3-types
    ├── neo3-rpc
    └── neo3-builder
```

## ✅ Validation Checklist

- [ ] All tests pass after reorganization
- [ ] Documentation builds correctly
- [ ] Examples compile and run
- [ ] No circular dependencies
- [ ] Clean module boundaries
- [ ] Consistent naming throughout
- [ ] All public APIs documented
- [ ] Benchmarks still work
- [ ] CI/CD pipelines pass
- [ ] Published crates work correctly