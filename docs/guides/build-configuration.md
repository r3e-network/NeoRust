# Build Configuration Guide

## Overview

This guide covers common build configuration issues and their solutions when working with NeoRust, particularly focusing on dependency management and feature flags for different build environments.

## Common Build Issues

### YubiHSM MockHsm Release Build Error

**Problem**: When building in release mode, you may encounter this error:
```
error: MockHsm is not intended for use in release builds
 --> /Users/runner/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/yubihsm-0.42.1/src/mockhsm.rs:5:1
  |
5 | compile_error!("MockHsm is not intended for use in release builds");
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

**Root Cause**: The `yubihsm` crate is configured with the `mockhsm` feature, which includes `MockHsm` functionality that's designed only for development and testing, not production builds.

**Solution**: Configure the `yubihsm` dependency to use different features based on the build profile:

```toml
[dependencies]
# For development builds (debug mode)
yubihsm = { version = "0.42", features = ["mockhsm", "http", "usb"] }

# For production builds, use conditional compilation:
[target.'cfg(debug_assertions)'.dependencies]
yubihsm = { version = "0.42", features = ["mockhsm", "http", "usb"] }

[target.'cfg(not(debug_assertions))'.dependencies]
yubihsm = { version = "0.42", features = ["http", "usb"] }
```

**Alternative Solution**: Use feature flags to conditionally enable mock functionality:

```toml
[dependencies]
yubihsm = { version = "0.42", features = ["http", "usb"], optional = true }

[features]
default = []
hardware-security = ["yubihsm"]
mock-hsm = ["yubihsm/mockhsm"]
```

## Build Profiles and Feature Management

### Development vs Production Features

Different build environments require different feature sets:

#### Development Features
- `mockhsm`: Mock hardware security module for testing
- Debug logging and tracing
- Development-only dependencies

#### Production Features
- Hardware security modules without mock functionality
- Optimized cryptographic operations
- Minimal logging overhead

### Conditional Compilation

Use Rust's conditional compilation features to handle environment-specific code:

```rust
#[cfg(debug_assertions)]
use yubihsm::MockHsm;

#[cfg(not(debug_assertions))]
use yubihsm::Hsm;

// Development-only code
#[cfg(debug_assertions)]
fn create_test_hsm() -> MockHsm {
    MockHsm::new()
}

// Production code
#[cfg(not(debug_assertions))]
fn create_production_hsm() -> Result<Hsm, Error> {
    Hsm::connect("usb://")
}
```

## Feature Flag Best Practices

### 1. Environment-Specific Features

Organize features by environment:

```toml
[features]
default = ["production"]

# Environment features
development = ["mock-hsm", "debug-logging"]
testing = ["mock-hsm", "test-utils"]
production = ["hardware-security", "optimized-crypto"]

# Component features
mock-hsm = ["yubihsm/mockhsm"]
hardware-security = ["yubihsm/http", "yubihsm/usb"]
debug-logging = ["tracing/max_level_debug"]
optimized-crypto = ["ring/optimized"]
```

### 2. Conditional Dependencies

Use conditional dependencies to avoid including unnecessary crates:

```toml
[dependencies]
# Always included
tokio = { version = "1.32", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }

# Conditionally included
yubihsm = { version = "0.42", features = ["http", "usb"], optional = true }
mockall = { version = "0.13.0", optional = true }

[dev-dependencies]
# Development and testing only
mockall = "0.13.0"

[features]
hardware-security = ["yubihsm"]
mock-testing = ["mockall"]
```

## Build Scripts and Environment Detection

### Detecting Build Environment

Use build scripts to detect the build environment:

```rust
// build.rs
fn main() {
    // Detect if we're in a CI environment
    if std::env::var("CI").is_ok() {
        println!("cargo:rustc-cfg=ci_build");
    }
    
    // Detect release vs debug
    let profile = std::env::var("PROFILE").unwrap_or_default();
    if profile == "release" {
        println!("cargo:rustc-cfg=release_build");
    }
    
    // Check for specific features
    if std::env::var("CARGO_FEATURE_MOCK_HSM").is_ok() {
        println!("cargo:rustc-cfg=mock_hsm_enabled");
    }
}
```

### Using Build Configuration in Code

```rust
#[cfg(ci_build)]
const DEFAULT_TIMEOUT: u64 = 60; // Longer timeout for CI

#[cfg(not(ci_build))]
const DEFAULT_TIMEOUT: u64 = 30;

#[cfg(all(release_build, mock_hsm_enabled))]
compile_error!("Mock HSM should not be enabled in release builds");
```

## Troubleshooting Build Issues

### Common Error Patterns

1. **Feature Conflicts**: When incompatible features are enabled together
2. **Missing Dependencies**: When required dependencies are not included
3. **Platform-Specific Issues**: When dependencies don't support the target platform
4. **Version Conflicts**: When different crates require incompatible versions

### Debugging Build Configuration

Use these commands to debug build issues:

```bash
# Show all features and dependencies
cargo tree --features

# Build with specific features
cargo build --features "hardware-security"
cargo build --no-default-features --features "mock-hsm"

# Check feature resolution
cargo metadata --format-version 1 | jq '.packages[] | select(.name == "neo3") | .features'

# Verbose build output
cargo build --verbose
```

## Security Considerations

### Production Builds

- Never include mock or test features in production
- Use hardware security modules when available
- Enable all security-related features
- Disable debug logging in production

### Development Builds

- Use mock implementations for testing
- Enable comprehensive logging
- Include development tools and utilities
- Allow for rapid iteration and debugging

## Related Documentation

- [Installation Guide](installation.md): Basic installation and setup
- [Configuration Reference](../reference/configuration.md): Detailed configuration options
- [Security Best Practices](../guides/security.md): Security-focused configuration 