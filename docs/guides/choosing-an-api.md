# Choosing the Right API Layer

NeoRust ships **two complementary layers**. Most applications use the
high-level layer for everything and only reach into the low-level modules for a
handful of advanced cases. This guide tells you which to reach for, and when to
switch.

> **TL;DR** — Start with `Neo::testnet().await?` from the high-level layer.
> Drop down only when the table below tells you to.

---

## Quick decision table

| You want to… | Use | Returns |
|---|---|---|
| Connect in one line, query block height / balance | `sdk::Neo` | `Balance`, `u32`, … |
| Send or transfer NEP-17 tokens with retry + signing handled | `sdk::Neo::transfer` | `TxHash` |
| Read a contract method without sending a tx | `sdk::Neo::invoke_read` | `InvocationResult` |
| Write/call a contract method (broadcasts a tx) | `sdk::Neo::invoke_write` | `TxHash` |
| Deploy a contract | `sdk::Neo::deploy_contract` | `TxHash` |
| Wait for a tx to confirm | `sdk::Neo::wait_for_confirmation` | `()` |
| HD / BIP-39 wallet generation | `sdk::hd_wallet::HDWallet` | `Account` |
| Preview a tx's fees/state-changes before sending | `sdk::transaction_simulator` | `SimulationResult` |
| Stream blocks / txs over WebSocket | `sdk::websocket::WebSocketClient` | events |
| Call a **specific** RPC method by hand (e.g. `getproof`, `getstateheight`) | `providers::RpcClient` | typed RPC response |
| Build a raw script / sign manually / control every byte | `builder::TransactionBuilder` + `builder::ScriptBuilder` | `Transaction` |
| Work with Neo X (EVM sidechain) | `neo_x::NeoXWallet`, `sdk::unified::EcosystemClient` | EVM types |
| Store/retrieve on NeoFS | `neo_fs::client` | NeoFS objects |

---

## Layer 1 — High-level: `sdk::Neo`

The opinionated, batteries-included client. **Start here.**

```no_run
use neo3::prelude::*; // brings Neo, NeoBuilder, Network, NeoError, … into scope

# #[tokio::main]
# async fn main() -> Result<(), NeoError> {
let neo = Neo::testnet().await?;           // or Neo::from_env() / Neo::connect(url)
let height = neo.get_block_height().await?;
let balance = neo.get_balance("NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc").await?;
println!("tip={height}, NEO={}", balance.neo);
# Ok(())
# }
```

What you get for free:
- **Automatic retry** with bounded budget (`SdkConfig::retries`).
- **Caching** of idempotent lookups (`SdkConfig::cache`).
- **Unified errors** — every call returns `NeoError` with `kind()`,
  `is_retryable()`, and human-readable recovery hints (mirrors the AWS SDK
  `ProvideErrorMetadata` pattern).
- **Configurability** via `Neo::builder().network(..).timeout(..).retries(..)`.

All errors are the single `neo3::prelude::NeoError` type (= `unified::NeoError`),
so `fn() -> Result<T, NeoError>` composes with `?` across the whole SDK.

---

## Layer 2 — Low-level modules

Reach here when the high-level layer does not expose exactly what you need.

### `providers::RpcClient` — raw JSON-RPC

Covers the full Neo N3 RPC surface (91 methods), including the state-root /
proof / iterator family that the high-level layer does not wrap:

```no_run
use neo3::neo_clients::{HttpProvider, RpcClient, APITrait};

# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
let client = RpcClient::new(HttpProvider::new("https://testnet1.neo.org:443")?);
let count = client.get_block_count().await?;
let state_height = client.get_state_height().await?;
# Ok(())
# }
```

Low-level calls return their own domain error (`ProviderError`, `ContractError`,
`BuilderError`, …), each of which converts into the unified `NeoError` via `?`
if you need a single error boundary.

### `builder::TransactionBuilder` — byte-level tx construction

When you need exact control over the script, signers, attributes, or fees:

```no_run
use neo3::neo_builder::{AccountSigner, ScriptBuilder, TransactionBuilder};
// build script, attach signers, set valid_until_block, then .sign().await?
```

### Domain modules

- `crypto` — key generation, signing, hashing, NEP-2.
- `wallets` — NEP-6 wallets, account import/export, encryption.
- `contract` — typed wrappers over native contracts (`NeoToken`,
  `GasToken`, `PolicyContract`, `RoleManagement`).
- `codec` — Neo VM binary serialization.

---

## When to drop down from high-level to low-level

Switch layers when **any** of these is true:

1. **You need an RPC method `Neo` does not wrap.** Rare — `Neo` covers the
   common ones (balance, transfer, invoke, deploy, confirm). For state proofs,
   session iterators, or mempool details, use `RpcClient` directly.
2. **You need byte-exact transaction control** — custom attributes, multiple
   signers with specific scopes, or hand-tuned network fees. Use
   `TransactionBuilder`.
3. **You want a different retry/cache policy** than `SdkConfig` allows, or want
   none at all. Use `RpcClient` + your own orchestration.
4. **You are building infrastructure** (an indexer, an exchange backend, a
   block explorer) where the overhead of the high-level layer is not justified.

The two layers interoperate: `Neo::client()` hands you the underlying
`RpcClient` if you need to make a one-off low-level call without reconnecting.

---

## See also

- Crate-level docs (`cargo doc -p neo3 --open`) — the `## Choosing an API
  layer` section is the short version of this guide.
- `examples/standalone/src/bin/high_level_sdk.rs` — high-level end-to-end.
- `examples/basic/`, `examples/neo_transactions/`, `examples/contracts/` —
  low-level patterns by topic.
