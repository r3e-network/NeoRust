# Local Verification Guide

This repo includes lightweight scripts that mirror the Rust parts of GitHub Actions. Run them before pushing to catch formatting/lint/test issues early.

## Quick Start

- `./ci-check`: Format + Clippy + `neo3` tests/doc tests + `neo-cli` tests (recommended)
- `./run-ci-checks.sh`: Similar checks with a summarized report
- `./check-ci.sh`: Minimal pre-push checks

## Manual Commands (Rust)

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Core library
NEORUST_SKIP_NETWORK_TESTS=1 cargo test -p neo3 --all-features
cargo test -p neo3 --doc --all-features

# CLI
cargo test -p neo-cli
```

## Packaging (Optional)

`cargo package` / `cargo publish --dry-run` may require access to the crates.io index. In restricted or proxied environments, these commands can fail even when builds/tests pass.

## Security (RustSec / cargo-audit)

On a typical developer machine:

```bash
cargo audit
```

In restricted environments where `~/.cargo` is read-only or file locks are blocked, you can use a writable copy of the advisory DB:

```bash
cp -a ~/.cargo/advisory-db /tmp/rustsec-advisory-db
cargo audit --db /tmp/rustsec-advisory-db --no-fetch
```

## Frontend / Tauri App (`neo-gui`)

The legacy `neo-gui` app is not part of the Rust workspace. If you work on it, verify it separately:

```bash
cd neo-gui
npm ci
npm run lint
npm test
```

---

**Happy coding! 🦀🌐**
