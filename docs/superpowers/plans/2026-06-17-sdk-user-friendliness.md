# SDK 用户友好性与专业性提升 实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: superpowers:subagent-driven-development 或 superpowers:executing-plans。Steps 用 `- [ ]` 复选框跟踪。

**Goal:** 把 NeoRust 从"功能完整"提升到"用户友好且专业":统一 prelude 的错误类型、让 prelude 暴露高层 SDK 入口、清理无法编译的散落示例、补"选择 API 层"决策文档、审计错误信息质量。

**Architecture:** SDK 功能已完整(91 RPC 方法、N3 状态/证明齐全、原生合约覆盖、AWS 风格错误模型)。本计划聚焦**公共 API 表面的用户友好性**,不改内部架构。错误处理已是良好分层(各领域 error → unified::NeoError 的 13 个 From 实现),只需让 prelude 指向 unified 即可让用户获得类型一致的体验。

**Tech Stack:** Rust 2021, `thiserror`, doctest 验证。用户指示"不需要向后兼容,只要最佳实现"。

**审计基线(2026-06-17, HEAD = 9d51a197):**
- prelude 导出 `NeoError`(=legacy `Neo3Error` 别名),但高层 `sdk::Neo` 返回 `unified::NeoError`(不同类型)
- `unified::NeoError` 有 13 个 `From` 实现,覆盖所有领域错误(ProviderError/BuilderError/ContractError/WalletError/CryptoError/TransactionError/Neo3Error/io/serde... )
- prelude 不导出高层 `sdk::Neo`/`NeoBuilder`/`SdkConfig`/`Token`/`Balance`
- 7 个根目录散落 `.rs`(high_level_sdk/gas_estimation/production_ready_client/v1_0_features/analyze_encryption_bottleneck/test_debug_vs_release/test_encryption_performance)被 git 跟踪但**不在任何 workspace/`[[bin]]`**,cargo 从不编译
- 全部 workspace examples(`cargo check --examples --workspace`)编译通过

---

## 阶段 F:prelude 改导出 unified NeoError

### Task F.1:prelude 的 NeoError 指向 unified

**Files:** Modify `src/prelude.rs:31`, `src/prelude.rs:14,18`(doc)

**根因:** `prelude.rs:31` 是 `pub use crate::neo_error::NeoError;`(legacy 别名 = `Neo3Error`)。高层 `sdk::Neo` 返回 `neo_error::unified::NeoError`。两者不同类型,用户 `use prelude::*` 后调用高层 API 会类型不匹配。unified 是官方推荐且有 `From` 覆盖所有领域错误。

- [ ] **Step 1: 把 prelude 的 NeoError 改指向 unified**

`src/prelude.rs:31` 当前:
```rust
// Core error type (legacy alias)
pub use crate::neo_error::NeoError;
```
改为:
```rust
// Unified error type (modern). Carries kind()/is_retryable()/recovery hints,
// and has From impls for every domain error (ProviderError, BuilderError,
// ContractError, WalletError, CryptoError, Neo3Error, io, serde, ...) so a
// single `fn() -> Result<T, NeoError>` boundary works with `?` across the SDK.
pub use crate::neo_error::unified::NeoError;
```

- [ ] **Step 2: 更新 prelude doc 表格行**

`src/prelude.rs:14-18` 区域,表格里的 Errors 行:
```
//! | Errors | Legacy `NeoError` alias (see [`crate::neo_error::unified::NeoError`] for the modern type) |
```
改为:
```
//! | Errors | [`NeoError`] — unified error with `kind()`/`is_retryable()`/recovery hints |
```
并补 doc 里的 intra-doc 链接(若需要):
```
//! [`NeoError`]: crate::neo_error::unified::NeoError
```

- [ ] **Step 3: 验证编译 + doctest**

Run: `cargo check -p neo3 --lib 2>&1 | tail -3` → 期望 `Finished`
Run: `cargo test --doc -p neo3 2>&1 | tail -5` → 期望 `207 passed`(或更多)

- [ ] **Step 4: Commit**

```bash
git add src/prelude.rs
git commit -m "refactor(prelude): export unified NeoError instead of legacy Neo3Error alias"
```

### Task F.2:验证内部代码无破坏(确认 unified::NeoError 不缺 From)

**Files:** 无修改,仅验证

- [ ] **Step 1: 全 workspace clippy**

Run: `cargo clippy --workspace --all-targets -- -D warnings 2>&1 | tail -3`
Expected: `Finished`,0 warning。

- [ ] **Step 2: lib + doctest 全过**

Run: `cargo test --lib 2>&1 | tail -3` → 期望 538 passed
Run: `cargo test --doc 2>&1 | tail -3` → 期望 207 passed

- [ ] **Step 3: 如有编译错误**(unified 缺某 From),补 `impl From<XxxError> for NeoError` 到 `src/neo_error/unified.rs`,参考现有 13 个 From 实现的模式。

---

## 阶段 G:prelude 加入高层 SDK 类型

### Task G.1:prelude 导出 sdk::Neo / NeoBuilder / SdkConfig / Token / Balance

**Files:** Modify `src/prelude.rs`(末尾追加高层 SDK 导出段)

**根因:** 根 README/lib doc 主推 `sdk::Neo`(一行连 TestNet),但 `use neo3::prelude::*` 不含 `Neo`,用户必须再写 `use neo3::sdk::Neo;`。prelude 应暴露推荐入口。无循环引用风险(`sdk/` 不 import `prelude`)。

- [ ] **Step 1: 确认这些类型的可见路径**

Run: `grep -nE "^pub (struct|enum) (Neo|NeoBuilder|SdkConfig|Token|Balance|Network)\b" src/sdk/mod.rs`
确认全在 `src/sdk/mod.rs` 顶层 pub。

- [ ] **Step 2: 在 prelude.rs 末尾追加高层 SDK 导出**

在 `src/prelude.rs` 末尾(`ToHexString` 那行之后)追加:
```rust

// === High-level SDK API ===
// The opinionated, batteries-included entry point. `Neo::testnet()`,
// `Neo::from_env()`, `Neo::connect()` — see the crate-level docs for guidance
// on when to use this vs the lower-level `providers::RpcClient`.
pub use crate::sdk::{Balance, Neo, NeoBuilder, Network, SdkConfig, Token};
```
(若 `Network` 实际是 enum 且名字相符则保留;以 grep 结果为准。)

- [ ] **Step 3: 验证编译 + doctest**

Run: `cargo check -p neo3 --lib 2>&1 | tail -3` → `Finished`
Run: `cargo test --doc -p neo3 2>&1 | tail -3` → 全 passed

- [ ] **Step 4: 新增一个 doctest 证明 prelude 现在能直接用 Neo**

在 `src/prelude.rs` 顶部 doc 的 `## What you get` 之后,补一个 `no_run` 示例:
```rust
//! ## Quick start via the prelude
//!
//! ```no_run
//! use neo3::prelude::*;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), NeoError> {
//! let neo = Neo::testnet().await?;
//! let height = neo.get_block_height().await?;
//! println!("tip = {height}");
//! # Ok(())
//! # }
//! ```
```

- [ ] **Step 5: Commit**

```bash
git add src/prelude.rs
git commit -m "feat(prelude): re-export high-level SDK entry (Neo/NeoBuilder/SdkConfig/Token/Balance)"
```

---

## 阶段 H:处理散落示例文件

### Task H.1:决定 7 个散落 .rs 的去留

**Files:** `examples/{high_level_sdk,gas_estimation,production_ready_client,v1_0_features,analyze_encryption_bottleneck,test_debug_vs_release,test_encryption_performance}.rs`

**根因:** 这 7 个文件被 git 跟踪,但不在任何 workspace member 的 `[[bin]]`,cargo 从不编译它们——无法验证 API 漂移。专业 SDK 不应有"从未编译过的示例"。

**决策(按文件性质):**
- `high_level_sdk.rs` / `gas_estimation.rs` / `production_ready_client.rs` / `v1_0_features.rs` —— **有价值的示例**:内容是真实用例。但需纳入构建或移入现有 example crate,否则会腐烂。
- `analyze_encryption_bottleneck.rs` / `test_debug_vs_release.rs` / `test_encryption_performance.rs` —— **一次性性能/调试脚本**(从文件名和 `test_` 前缀判断),非用户示例。

- [ ] **Step 1: 把 4 个有价值示例移入 existing example crate 或新建 crate,使其被编译**

方案(选工作量小的):在根 `Cargo.toml` 的 `[workspace] members` 加一个 `examples/standalone` crate,把这 4 个文件作为它的 `[[bin]]`。或更简单:把它们各自的内容并入已有的同类 example(如 `high_level_sdk` 内容并入 `examples/basic`)。

**推荐最小方案:** 为这 4 个文件在根 crate 加 `[[bin]]`(根 crate 已有 neo3 依赖),使其被编译验证。在 `Cargo.toml` 加:
```toml
[[bin]]
name = "example_high_level_sdk"
path = "examples/high_level_sdk.rs"

[[bin]]
name = "example_gas_estimation"
path = "examples/gas_estimation.rs"
# ... 对其他有价值的同理
```
注意:这些 bin 依赖 tokio/bip39 等,可能需补 `[dev-dependencies]` 或用 `required-features`。若编译失败且修复成本高,降级为删除。

- [ ] **Step 2: 3 个 test_/analyze 脚本删除**

```bash
git rm examples/analyze_encryption_bottleneck.rs examples/test_debug_vs_release.rs examples/test_encryption_performance.rs
```

- [ ] **Step 3: 验证被纳入的示例能编译**

Run: `cargo check --bins -p neo3 2>&1 | tail -5` → `Finished`
若编译失败:修复 API 漂移,或退回为删除该文件。

- [ ] **Step 4: Commit**

```bash
git add -A examples/ Cargo.toml
git commit -m "chore(examples): wire standalone examples into the build, drop throwaway perf scripts"
```

---

## 阶段 I:补"选择 API 层"指南文档

### Task I.1:写 WHICH_API.md 决策指南

**Files:** Create `docs/guides/choosing-an-api.md`(或 `WHICH_API.md`),并在 README/lib doc 链接它

**根因:** SDK 有两套入口(高层 `sdk::Neo` vs 低层 `providers::RpcClient`),但缺"我该用哪个"的决策指南。新用户不知道从哪开始。

- [ ] **Step 1: 创建决策指南**

`docs/guides/choosing-an-api.md` 内容大纲:
```markdown
# Choosing the Right API Layer

NeoRust offers two layers. Pick by what you need:

## Decision table

| You want to... | Use |
|---|---|
| Send/transfer tokens, check balance, one-liner connect | `sdk::Neo` (high-level) |
| Build a dApp backend, retry + caching out of the box | `sdk::Neo` |
| Match on a specific RPC error, control every byte | `providers::RpcClient` (low-level) |
| Construct raw scripts / sign manually | `neo_builder::TransactionBuilder` |
| Hardware wallet, HD derivation | `neo_wallets` + `sdk::hd_wallet` |

## High-level: sdk::Neo
[3-line example]

## Low-level: providers::RpcClient
[3-line example]

## When to drop down
- You need an RPC method not exposed by Neo (rare; most are covered)
- You need byte-exact transaction control
```

- [ ] **Step 2: 在 lib.rs 顶层 doc 的 "Choosing an API layer" 段链接它**

`src/lib.rs` 的 `## Choosing an API layer` 段末尾加:
```
//! For a full decision guide, see [Choosing an API Layer](https://github.com/R3E-Network/NeoRust/blob/master/docs/guides/choosing-an-api.md).
```

- [ ] **Step 3: Commit**

```bash
git add docs/guides/choosing-an-api.md src/lib.rs
git commit -m "docs: add 'Choosing an API Layer' decision guide"
```

---

## 阶段 J:审计错误信息质量

### Task J.1:抽查错误构造是否有 message + recovery

**Files:** 视审计结果而定

**目标:** 确保面向用户的错误都有可读 message + recovery hint,而非裸字符串或空 message。

- [ ] **Step 1: 找 message 为空或裸字符串的 NeoError 构造**

Run: `grep -rnE "NeoError::\w+ \{" src/ | grep -v test | head -40`
人工检查:每个是否有 `message:` 字段且非空;关键路径(transfer/balance/connect)是否有 `recovery:`。

- [ ] **Step 2: 找低质量错误(只有字符串的 legacy 风格)**

Run: `grep -rnE "NeoError::\w+\(\"" src/ | grep -v test`
这些是 legacy tuple 变体构造,缺结构化信息。若数量少(<20),逐个补 recovery。

- [ ] **Step 3: 修复发现的问题**(若有)

- [ ] **Step 4: Commit(若有改动)**

```bash
git add -A
git commit -m "refactor(error): add recovery hints to user-facing error constructors"
```

---

## 阶段 K:全量验证

- [ ] **Step 1: clippy 0 warning**
Run: `cargo clippy --workspace --all-targets -- -D warnings 2>&1 | tail -3` → `Finished`

- [ ] **Step 2: lib + doctest**
Run: `cargo test --lib 2>&1 | tail -3` → 538 passed
Run: `cargo test --doc 2>&1 | tail -3` → 全 passed

- [ ] **Step 3: 汇总 commit 列表**
Run: `git log --oneline 9d51a197..HEAD`

---

## Self-Review

- **覆盖:** F=prelude error 统一;G=prelude 高层入口;H=散落示例;I=API 层指南;J=错误质量。覆盖用户的 4 个选择。
- **Placeholder:** 每步有具体命令/代码。H.1 Step 1 有明确的降级路径(编译失败则删除)。
- **风险点:** F 改公共 API,但有 13 个 From 兜底 + doctest 验证;G 无循环引用(已验证);H 涉及删除文件,仅删 test_/analyze 脚本。
