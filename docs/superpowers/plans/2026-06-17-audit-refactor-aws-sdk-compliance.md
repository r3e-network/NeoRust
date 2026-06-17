# NeoRust 审计 / 重构 / AWS-SDK 规范一致性 实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 对 NeoRust (neo3 v1.4.0) 进行四阶段审计与修复:① 修掉已知小问题;② 把公共 builder API 补齐到 AWS Rust SDK 规范(`#[must_use]` 覆盖);③ 修正代码与文档的不一致;④ 用 benchmark 定位并优化真实热点。

**Architecture:** 代码库已处于健康状态(workspace 0 编译错误,clippy 仅 1 个警告,AWS 风格错误模型 `ProvideErrorMetadata`/`NeoError`/`is_retryable` 已落地)。本计划不重写架构,只做**外科手术式**的、可独立 review 的修复,每个阶段一个 commit,直接在 `master` 上推进(按用户要求)。所有改动遵循"先测量/先定位 → 改 → 验证编译+测试+clippy"的闭环。

**Tech Stack:** Rust 2021 (rust-version 1.83), `thiserror`, `tracing`, `criterion` 0.8 (benches), `tokio`。CI 把 warning 当 error。

**审计基线事实(2026-06-17, HEAD = 806cf51f):**
- `cargo check --workspace --all-targets`:✅ 0 error
- `cargo clippy --workspace --all-targets`:1 warning(`items_after_test_module` @ `src/lib.rs:596`)
- 公共 API 表面:~1735 个 pub 项
- `#[must_use]` 覆盖:9 个(但 80 个消费 `mut self` 返回 `Self` 的 builder setter 需要它)
- 生产代码 panic:绝大多数在 `#[test]` 断言里(可接受),少数 `unwrap_or_else(|_| panic!(...))` 已有 fallible 版本和文档(可接受)
- README/Cargo.toml/CHANGELOG 版本号一致(均 v1.4.0);`ws` feature 真实存在,README `features=["ws"]` 正确

---

## 阶段 1:修复已知小问题

### Task 1.1:修复 `items_after_test_module` clippy 警告

**Files:**
- Modify: `src/lib.rs:686-694`(把 `#[cfg(feature="futures")] pub use futures;` 和 `#[cfg(feature="ledger")] pub use coins_ledger;` 移到 `mod tests`(第 595 行)之前)

**根因:** `lib.rs` 在 `#[cfg(test)] mod tests { ... }`(595–686 行)之后还有两个 `pub use` re-export。Clippy 的 `items_after_test_module` lint 要求所有非测试 item 在 test module 之前。

- [ ] **Step 1: 把两个 re-export 上移**

将以下两块从文件末尾(第 688–693 行)剪切:
```rust
// Explicitly mark external dependencies with cfg_attr for docs.rs
#[cfg(feature = "futures")]
pub use futures;

#[cfg(feature = "ledger")]
pub use coins_ledger;
```
粘贴到 `pub mod prelude;`(第 593 行)之后、`#[cfg(test)] mod tests {`(第 595 行)之前。

- [ ] **Step 2: 验证 clippy 干净**

Run: `cargo clippy --workspace --all-targets 2>&1 | grep -E "warning|error" | grep -v "generated.*warning" | grep -v "Cargo.toml"`
Expected: 无输出(0 warning)。

- [ ] **Step 3: 验证编译 + 测试仍通过**

Run: `cargo test --lib --no-run 2>&1 | tail -3`
Expected: `Finished` 无 error。

- [ ] **Step 4: Commit**

```bash
git add src/lib.rs
git commit -m "fix: move re-exports before test module to clear clippy items_after_test_module"
```

---

## 阶段 2:AWS SDK 规范 — builder `#[must_use]` 覆盖

### 背景(AWS Rust SDK 规范)

AWS Rust SDK (`aws-sdk-rust`) 的每个 builder 的 setter,只要**消费 `self` 并返回 `Self`**,都标注 `#[must_use]`。原因:调用 `Builder::field(x)` 而不接收返回值会静默丢弃改动(因为 self 被 move)。`#[must_use]` 让编译器在误用时报警告。

NeoRust 的 commit `e0c46869` 已经开始做但只覆盖了 9 处,实际有 80 个符合模式的方法。本阶段补齐。

**判定规则(逐方法套用):**
- 方法签名形如 `pub fn xxx(mut self, ...) -> Self` → **加 `#[must_use]`**
- 方法签名形如 `pub fn xxx(&mut self, ...) -> ()` 或 `-> Result<(), _>` → **不加**(原地修改)

### Task 2.1:给 `sdk/mod.rs` 的消费-self builder setter 补 `#[must_use]`

**Files:** Modify `src/sdk/mod.rs`

- [ ] **Step 1: 列出本文件所有候选方法**

Run: `grep -nE "pub fn [a-z_]+\(mut self[^)]*\) -> Self" src/sdk/mod.rs`

- [ ] **Step 2: 逐个为每个匹配方法在 `pub fn` 上方加 `#[must_use]`**

示例修改(对每个候选重复):
```rust
// 改前
pub fn with_memo(mut self, memo: impl Into<String>) -> Self {
// 改后
#[must_use]
pub fn with_memo(mut self, memo: impl Into<String>) -> Self {
```

- [ ] **Step 3: 验证 clippy 不报 `useless_attribute` 且无新警告**

Run: `cargo clippy -p neo3 --lib 2>&1 | grep -E "warning|error"`
Expected: 无输出。

- [ ] **Step 4: 验证编译**

Run: `cargo build -p neo3 --lib 2>&1 | tail -3`
Expected: `Finished`。

- [ ] **Step 5: Commit**

```bash
git add src/sdk/mod.rs
git commit -m "style(sdk): add #[must_use] to consuming-self builder setters (AWS SDK pattern)"
```

### Task 2.2:给 `sdk/transaction_simulator.rs` + `sdk/websocket.rs` + `sdk/hd_wallet.rs` 补 `#[must_use]`

**Files:** Modify `src/sdk/transaction_simulator.rs`, `src/sdk/websocket.rs`, `src/sdk/hd_wallet.rs`

- [ ] **Step 1: 对三个文件各执行 grep 找候选**

Run: `for f in src/sdk/transaction_simulator.rs src/sdk/websocket.rs src/sdk/hd_wallet.rs; do echo "== $f =="; grep -nE "pub fn [a-z_]+\(mut self[^)]*\) -> Self" "$f"; done`

- [ ] **Step 2: 逐个加 `#[must_use]`**(规则同 2.1)

- [ ] **Step 3: 验证**

Run: `cargo clippy -p neo3 --lib 2>&1 | grep -E "warning|error"`
Expected: 无输出。

- [ ] **Step 4: Commit**

```bash
git add src/sdk/transaction_simulator.rs src/sdk/websocket.rs src/sdk/hd_wallet.rs
git commit -m "style(sdk): add #[must_use] to builder setters in transaction_simulator/websocket/hd_wallet"
```

### Task 2.3:给 `neo_wallets` + `neo_error` 补 `#[must_use]`

**Files:** Modify `src/neo_wallets/wallet/wallet.rs`, `src/neo_error/unified.rs`

- [ ] **Step 1: grep 候选**

Run: `for f in src/neo_wallets/wallet/wallet.rs src/neo_error/unified.rs; do echo "== $f =="; grep -nE "pub fn [a-z_]+\(mut self[^)]*\) -> Self" "$f"; done`

注意 `neo_error/unified.rs` 已有部分 `#[must_use]`,只补缺的。

- [ ] **Step 2: 逐个加 `#[must_use]`**(规则同 2.1)

- [ ] **Step 3: 验证**

Run: `cargo clippy -p neo3 --lib 2>&1 | grep -E "warning|error"`
Expected: 无输出。

- [ ] **Step 4: Commit**

```bash
git add src/neo_wallets/wallet/wallet.rs src/neo_error/unified.rs
git commit -m "style(wallets,error): add #[must_use] to consuming-self builder setters"
```

### Task 2.4:给 `neo_clients` 补 `#[must_use]`

**Files:** Modify `src/neo_clients/rpc/rpc_client.rs`, `src/neo_clients/production_client.rs`, `src/neo_clients/connection_pool.rs`, `src/neo_clients/circuit_breaker.rs`, `src/neo_clients/cache.rs`, `src/neo_clients/rate_limiter.rs`, `src/neo_clients/rpc/transports/retry.rs`

- [ ] **Step 1: grep 候选(遍历整个目录)**

Run: `grep -rnE "pub fn [a-z_]+\(mut self[^)]*\) -> Self" src/neo_clients/`

- [ ] **Step 2: 逐个加 `#[must_use]`**(规则同 2.1)

- [ ] **Step 3: 验证**

Run: `cargo clippy -p neo3 --lib 2>&1 | grep -E "warning|error"`
Expected: 无输出。

- [ ] **Step 4: Commit**

```bash
git add src/neo_clients/
git commit -m "style(clients): add #[must_use] to consuming-self builder setters"
```

### Task 2.5:给 `neo_fs` + `monitoring` 补 `#[must_use]`

**Files:** Modify `src/neo_fs/client.rs`, `src/neo_fs/object.rs`, `src/neo_fs/container.rs`, `src/neo_fs/mod.rs`, `src/monitoring/mod.rs`

- [ ] **Step 1: grep 候选**

Run: `grep -rnE "pub fn [a-z_]+\(mut self[^)]*\) -> Self" src/neo_fs/ src/monitoring/`

- [ ] **Step 2: 逐个加 `#[must_use]`**(规则同 2.1)

- [ ] **Step 3: 验证**

Run: `cargo clippy -p neo3 --lib 2>&1 | grep -E "warning|error"`
Expected: 无输出。

- [ ] **Step 4: 全 workspace 最终 clippy 复检**

Run: `cargo clippy --workspace --all-targets 2>&1 | grep -cE "^warning|^error"`
Expected: `0`

- [ ] **Step 5: Commit**

```bash
git add src/neo_fs/ src/monitoring/
git commit -m "style(neo_fs,monitoring): add #[must_use] to consuming-self builder setters"
```

---

## 阶段 3:文档与代码一致性审计

### Task 3.1:核对 README 的代码示例与实际 API

**Files:** Possibly modify `README.md`

- [ ] **Step 1: 提取 README 所有 ```rust / ```toml 代码块,核对 import 路径、类型名、方法名**

Run: `grep -nE "neo3::|features = " README.md`

逐条核对:
- `neo3 = "1.4.0"` ✅(与 Cargo.toml 一致)
- `features = ["ws"]` ✅(ws feature 真实存在)
- 每个 `use neo3::xxx` 路径是否真实存在

Run: 对照 `grep -rnE "pub (fn|struct|trait) (Neo|NeoBuilder|Token|Balance|SdkConfig)\b" src/`

- [ ] **Step 2: 修正任何不一致的路径/方法名**(若有)

- [ ] **Step 3: 验证 README 代码块可编译(抽检关键示例)**

把 README 里的 `sdk::Neo` 示例与 `examples/high_level_sdk.rs` 比对,确保 API 调用一致。

- [ ] **Step 4: Commit(若有改动)**

```bash
git add README.md
git commit -m "docs: fix README code samples to match actual public API"
```
(若无需改动,跳过 commit 并记录"已核对一致")

### Task 3.2:核对 lib.rs 顶层 doc 的示例

**Files:** Possibly modify `src/lib.rs:9-443`

- [ ] **Step 1: 核对 lib.rs doc 里 `use neo3::sdk::Neo;` / `Neo::testnet()` / `Neo::from_env()` / `Neo::connect()` 是否真实存在**

Run: `grep -nE "pub (async )?fn (testnet|from_env|connect|get_block_height)\b" src/sdk/mod.rs`

- [ ] **Step 2: 核对 lib.rs doc 里低层示例 `HttpProvider::new` / `RpcClient::new` / `APITrait` / `Account::create` / `Account::from_wif` 路径**

Run: `grep -rnE "pub fn (new|create|from_wif)\b" src/neo_clients/ src/neo_protocol/ | head`

- [ ] **Step 3: 修正不一致**(若有)

- [ ] **Step 4: Commit(若有改动)**

```bash
git add src/lib.rs
git commit -m "docs: sync lib.rs top-level doc examples with actual API"
```

### Task 3.3:补齐 `wallet_benchmarks.rs` 缺失的 `[[bench]]` 配置

**Files:** Modify `Cargo.toml`

**根因:** `benches/wallet_benchmarks.rs` 文件存在,但 `Cargo.toml` 只声明了 `crypto_benchmarks`、`script_builder_benchmarks`、`gas_estimator_benchmarks` 三个 `[[bench]]`。`wallet_benchmarks` 没被注册,`cargo bench` 不会跑它。

- [ ] **Step 1: 查看 wallet_benchmarks.rs 的 harness 设置**

Run: `head -20 benches/wallet_benchmarks.rs`
确认是否用 `criterion_main!`(若是 → `harness = false`)。

- [ ] **Step 2: 在 Cargo.toml 加 `[[bench]]` 段**

在最后一个 `[[bench]]` 段(`gas_estimator_benchmarks`)之后加:
```toml
[[bench]]
name = "wallet_benchmarks"
harness = false
```
(若 Step 1 显示用的是标准 `test` harness,则 `harness = true` 或省略——以实际为准。)

- [ ] **Step 3: 验证 bench 能被发现**

Run: `cargo bench --no-run 2>&1 | grep -i "wallet" | head`
Expected: 出现 wallet_benchmarks 编译目标。

- [ ] **Step 4: Commit**

```bash
git add Cargo.toml
git commit -m "build: register wallet_benchmarks in Cargo.toml [[bench]] (was orphaned)"
```

---

## 阶段 4:性能优化(先测量后优化)

### Task 4.1:跑现有 benchmark,建立基线

**Files:** 无修改,仅测量

- [ ] **Step 1: 编译所有 bench**

Run: `cargo bench --no-run 2>&1 | tail -5`
Expected: `Finished`。

- [ ] **Step 2: 跑 crypto + script_builder bench(快),记录基线**

Run: `cargo bench --bench crypto_benchmarks -- --warm-up-time 1 --measurement-time 3 2>&1 | grep -E "time:|bench_" | head -40`

把输出记录到本计划文件的"基线"节(下面),作为优化前后对比依据。

**基线记录区(填充实测值):**
```
crypto_sign:        [待填]
crypto_verify:      [待填]
script_build:       [待填]
```

- [ ] **Step 3: 分析热点**

只有当某项 benchmark 显示出明确的可优化点(如多余的 clone/分配、可预算的 buffer)时,才进入 Task 4.2。若 benchmark 全部健康 → 本阶段结论为"性能基线健康,无可量化收益的优化点",结束。

### Task 4.2:针对性优化(条件触发,仅当 4.1 发现明确热点)

**Files:** 视 4.1 结论而定

- [ ] **Step 1: 针对 4.1 发现的最热点,读源码定位冗余分配/clone**

- [ ] **Step 2: 改动**(保持 API 不变,只优化实现)

- [ ] **Step 3: 重跑该 bench,对比基线确认有提升且无回退**

Run: `cargo bench --bench <name> -- --warm-up-time 1 --measurement-time 3`
Expected: 目标项时间下降,其他项无回退。

- [ ] **Step 4: clippy + test 复检**

Run: `cargo clippy --workspace --all-targets 2>&1 | grep -cE "^warning|^error"` → 期望 0
Run: `cargo test --lib 2>&1 | tail -5` → 期望 passed

- [ ] **Step 5: Commit**

```bash
git add <files>
git commit -m "perf(<area>): <具体优化描述>"
```

---

## 全局收尾

### Task 5.1:全量验证

- [ ] **Step 1: 全 workspace clippy 0 warning**

Run: `cargo clippy --workspace --all-targets -- -D warnings 2>&1 | tail -5`
Expected: `Finished`,无 warning(因为 `-D warnings`)。

- [ ] **Step 2: 全 workspace 编译**

Run: `cargo check --workspace --all-targets 2>&1 | tail -3`
Expected: `Finished`。

- [ ] **Step 3: lib 单测**

Run: `cargo test --lib 2>&1 | tail -10`
Expected: 全 passed(或记录任何 pre-existing 失败并说明是否本次引入)。

- [ ] **Step 4: 汇总本次改动**

Run: `git log --oneline 806cf51f..HEAD`
把 commit 列表整理成给用户的总结。

---

## Self-Review 记录

- **Spec coverage:** 阶段 1 = clippy 修复;阶段 2 = AWS SDK builder 规范;阶段 3 = 文档一致性 + 孤儿 bench;阶段 4 = 性能(测量驱动)。覆盖用户选的 4 个方向。
- **Placeholder:** 所有步骤都有具体命令或具体代码模式;Task 4.2 是条件触发,触发条件明确(4.1 发现热点)。
- **Type consistency:** 本计划不引入新类型,只改 attribute/文档/实现,无类型一致性问题。
