# Known Issues

## Documentation Tests

Most documentation examples compile under `cargo test -p neo3`. Remaining `ignore`/`no_run` blocks are intentional for examples that require external resources (live RPC endpoints, hardware wallets, etc.).

### Issue Details

The main reasons examples are not executed in CI are:
1. Some examples require a live Neo RPC endpoint (network access and stable endpoints are not guaranteed in all CI environments).
2. Some examples require optional hardware integrations (e.g., Ledger) or OS-specific features.
3. Some examples are intended as illustrative snippets rather than deterministic tests.

### Temporary Solution

Examples are marked with `no_run` or `ignore` where appropriate to prevent flakiness while maintaining documentation value.

### Long-term Solution

A targeted review is still useful to:
1. Prefer `compile`/`no_run` for networked examples and keep them minimal.
2. Add clearly-documented env vars for opt-in live tests where applicable.
3. Periodically validate that examples still compile with current module paths.

### Affected Files

- src/lib.rs
- src/neo_builder/mod.rs
- src/neo_builder/script/script_builder.rs
- src/neo_builder/transaction/*.rs
- src/neo_clients/*.rs
- src/neo_crypto/*.rs
- src/neo_protocol/*.rs
- src/neo_types/*.rs
- src/neo_contract/*.rs

### Tracking Issue

TODO: Create GitHub issue to track the documentation test fixes.
