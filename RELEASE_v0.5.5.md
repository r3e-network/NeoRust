# Release 0.5.5 - 2026-01-20

## Highlights
- Synced embedded `neo_csharp` core/node/vm sources to the upstream v3.9.0 releases.
- Added Faun-era Policy helpers that fully unwrap iterators for blocked accounts and whitelist fee contracts.
- `getversion` now captures RPC settings plus protocol metadata (`standbycommittee`, `seedlist`).

## Breaking Changes
- High-level SDK balances now use exact decimal types instead of floating point:
  - `sdk::Balance.gas`: `f64` → `DecimalAmount` (8 decimals)
  - `sdk::TokenBalance.amount`: `f64` → `DecimalAmount` (token decimals)
  - `sdk::TokenBalance.decimals` removed (use `token.amount.decimals()`)

## Testing
- `cargo check -p neo3`
