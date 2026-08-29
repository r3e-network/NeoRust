# Installation

Get started with NeoRust SDK v3.0.0 by installing it in your Rust project.

## Prerequisites

- Rust 1.91 or later
- Cargo package manager

## Adding to Your Project

Add NeoRust to your `Cargo.toml`:

```toml
[dependencies]
neo3 = "3.0.0"
```

For specific features, use:

```toml
[dependencies]
neo3 = { version = "3.0.0", features = ["futures", "ledger"] }
```

## Available Features

- `futures` - Async/await support (recommended)
- `ledger` - Hardware wallet support
- `ws` - WebSocket client support

## Verification

Verify your installation:

```rust
use neo3::prelude::*;

fn main() {
    println!("NeoRust SDK v3.0.0 is ready!");
}
```

## Next Steps

- [Quick Start Guide](./quick-start.md)
- [Examples](./examples.md)
