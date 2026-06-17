# Installation

Get started with NeoRust SDK by installing it in your development environment.

## System Requirements

- **Rust**: Version 1.70 or later
- **Cargo**: Rust's package manager (included with Rust)
- **Operating System**: Windows, macOS, or Linux

## Install Rust

If you don't have Rust installed, get it from [rustup.rs](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Create a New Project

```bash
cargo new my-neo-app
cd my-neo-app
```

## Add NeoRust Dependency

Add NeoRust to your `Cargo.toml`:

```toml
[dependencies]
neo3 = "2.0.1"
tokio = { version = "1.0", features = ["full"] }
```

## Feature Flags

NeoRust provides several optional features:

```toml
[dependencies]
neo3 = { version = "2.0.1", features = ["futures", "ledger"] }
```

### Available Features

- **`futures`** — Async/await support (re-exports the `futures` crate; recommended for most apps)
- **`ledger`** — Ledger hardware wallet types (feature gate is compile-checked; production use should include real-device signing tests in your release environment)
- **`ws`** — Modern WebSocket transport (`tokio-tungstenite`), for real-time blockchain events
- **`legacy-ws`** — Legacy WebSocket compatibility layer (fallback)
- **`ipc`** — IPC transport (Unix domain sockets / Windows named pipes)
- **`mock`** — `MockClient` and in-memory providers for offline tests/CI
- **`yubi`** / **`mock-hsm`** — YubiHSM support, or the YubiHSM mock backend for tests
- **`sgx`** / **`no_std`** — Experimental Intel SGX / specialized `no_std` build gates; not certified as general embedded or `wasm32-unknown-unknown` support

## Verify Installation

Create a simple test to verify everything works:

```rust
// src/main.rs
use neo3::prelude::*;

fn main() {
    println!("NeoRust SDK v2.0.1 is ready!");
    
    // Create a simple account
    let account = Account::create().expect("Failed to create account");
    println!("Generated address: {}", account.get_address());
}
```

Run it:

```bash
cargo run
```

You should see output like:
```
NeoRust SDK v2.0.1 is ready!
Generated address: NXXXXxxxXXXxxxXXXxxxXXXxxxXXXxxx
```

## Troubleshooting

### Build Errors

If you encounter build errors, make sure you have the latest stable Rust:

```bash
rustup update stable
```

### Platform-Specific Issues

#### macOS
You may need to install additional tools:
```bash
xcode-select --install
```

#### Windows
Ensure you have the Microsoft C++ Build Tools installed.

#### Linux
Install build essentials:
```bash
# Ubuntu/Debian
sudo apt update && sudo apt install build-essential

# CentOS/RHEL
sudo yum groupinstall "Development Tools"
```

## Next Steps

- [Quick Start Guide](./quick-start.md) - Your first Neo application
- [Examples](/examples) - Practical code examples 