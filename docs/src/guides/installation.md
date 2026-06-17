# Installation Guide

## Prerequisites

- Rust and Cargo (stable or nightly)
- Optional: Ledger hardware device (for ledger features)
- Optional: YubiHSM device (for hardware security module features)

## Installation

Add NeoRust to your `Cargo.toml`:

```toml
[dependencies]
neo3 = "2.0.0"
```

Note: The crate is published as `neo3` but is imported as `neo` in code:

```rust,no_run
use neo3::prelude::*;
```

## Features

NeoRust provides several features to customize functionality:

- `futures`: Enables async/futures support (recommended)
- `ws`: Enables the modern WebSocket transport
- `ipc`: Enables IPC (Unix domain sockets / Windows named pipes) transport
- `ledger`: Enables hardware wallet support via Ledger devices
- `yubi` / `mock-hsm`: YubiHSM support and its mock for testing
- `legacy-ws`: Compatibility WebSocket transport (fallback)

Hardware wallet feature gates are compile-checked, but production releases should include real-device tests. The `sgx` and `no_std` flags are experimental specialized build gates and are not certified as general embedded or WASM support.

Example of enabling specific features:

```toml
[dependencies]
neo3 = { version = "2.0.0", features = ["futures", "ws", "ledger"] }
```

You can disable default features with:

```toml
[dependencies]
neo3 = { version = "2.0.0", default-features = false, features = ["futures"] }
```

## Verifying Installation

To verify that the SDK is installed correctly, create a simple test program:

```rust,no_run
use neo3::prelude::*;

fn main() {
    println!("NeoRust SDK installed successfully!");
}
```

Compile and run the program:

```bash
cargo run
```

If the program compiles and runs without errors, the SDK is installed correctly.

<!-- toc -->
