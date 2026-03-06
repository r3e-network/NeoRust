# SDK Safety Refactor Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Replace a focused set of silent-fallback conversion behaviors with explicit, testable APIs that better match Rust SDK best practices.

**Architecture:** Keep the current public surface largely intact for compatibility, but add fallible conversion entrypoints in the address/script-hash and builder utility layers. Implement the safe paths first, cover them with focused unit tests, and wire obvious stubs to existing conversion primitives instead of returning placeholder values.

**Tech Stack:** Rust 2021, Cargo unit tests, `thiserror`, existing Neo SDK conversion traits.

### Task 1: Add failing tests for safe address/hash conversions

**Files:**
- Modify: `src/neo_types/address.rs`
- Modify: `src/neo_types/address_or_scripthash.rs`

**Step 1: Write failing tests**
- Add a test that `from_script_hash` returns the known Neo N3 address for a known script hash.
- Add a test that a new fallible `try_script_hash` path rejects an invalid address without panicking.
- Add a test that a new `try_from_script_hash_bytes(&[u8])` / `TryFrom<&[u8]>` path rejects non-20-byte input.

**Step 2: Run targeted tests to verify they fail**
Run: `cargo test -p neo3 neo_types::address::tests::test_from_script_hash --quiet`
Run: `cargo test -p neo3 neo_types::address_or_scripthash --quiet`
Expected: fail because the safe/fallible APIs do not exist or return the current placeholder behavior.

### Task 2: Implement minimal safe conversion APIs

**Files:**
- Modify: `src/neo_types/address.rs`
- Modify: `src/neo_types/address_or_scripthash.rs`

**Step 1: Implement minimal code**
- Replace the `from_script_hash` stub with the existing `ScriptHashExtension::to_address()` conversion.
- Add `AddressOrScriptHash::try_script_hash()` returning `Result<H160, TypeError>`.
- Add `AddressOrScriptHash::try_from_script_hash_bytes(&[u8])` and a `TryFrom<&[u8]>` impl.
- Keep legacy `script_hash()` and `From<Bytes>` behavior for compatibility, but route them through the safe helpers and document the fallback behavior clearly.

**Step 2: Run targeted tests to verify they pass**
Run: `cargo test -p neo3 neo_types::address::tests::test_from_script_hash --quiet`
Run: `cargo test -p neo3 neo_types::address_or_scripthash --quiet`
Expected: PASS.

### Task 3: Add fallible builder helper for multisig script hashes

**Files:**
- Modify: `src/neo_builder/utils.rs`
- Test: `src/neo_builder/utils.rs`

**Step 1: Write failing tests**
- Add a test that a new `try_public_keys_to_scripthash` returns an error for zero threshold.
- Add a test that it returns an error when threshold exceeds key count.

**Step 2: Run tests to verify they fail**
Run: `cargo test -p neo3 neo_builder::utils --quiet`
Expected: FAIL because the fallible helper does not exist.

**Step 3: Write minimal implementation**
- Add `try_public_keys_to_scripthash(...) -> Result<ScriptHash, BuilderError>`.
- Keep `public_keys_to_scripthash` as a compatibility wrapper that logs and falls back to zero hash.

**Step 4: Run tests to verify they pass**
Run: `cargo test -p neo3 neo_builder::utils --quiet`
Expected: PASS.

### Task 4: Validate focused changes

**Files:**
- Modify: `docs/plans/2026-03-06-sdk-safety-refactor.md`

**Step 1: Run focused validation**
Run: `cargo test -p neo3 neo_types::address::tests::test_from_script_hash --quiet`
Run: `cargo test -p neo3 neo_types::address_or_scripthash --quiet`
Run: `cargo test -p neo3 neo_builder::utils --quiet`
Expected: PASS.

**Step 2: Run broader library validation**
Run: `cargo test -p neo3 --lib --quiet`
Expected: same baseline status as before, or better.
