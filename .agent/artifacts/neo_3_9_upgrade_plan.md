# Neo 3.9 Upgrade Plan for Rust SDK

## Overview
This document outlines the required changes to make the neo-rust-sdk compatible with Neo 3.9 based on the C# reference implementation analysis.

## Version Information
- **Current Rust SDK version**: Targets Neo 3.8
- **Target version**: Neo 3.9 (master-n3 branch)
- **C# Reference**: neo-project/neo repository, master-n3 branch

## Implementation Status

### ✅ Phase 1: Core Types (COMPLETED)

#### 1. Hardfork Enum
**File**: `src/neo_types/hardfork.rs` ✅ CREATED
- ✅ HF_Aspidochelone
- ✅ HF_Basilisk
- ✅ HF_Cockatrice
- ✅ HF_Domovoi
- ✅ HF_Echidna
- ✅ HF_Faun
- ✅ HF_Gorgon
- ✅ Serialization/Deserialization support
- ✅ Ordering and comparison
- ✅ Unit tests

#### 2. WhitelistedContract Type
**File**: `src/neo_types/whitelisted_contract.rs` ✅ CREATED
- ✅ `contract_hash: H160`
- ✅ `method: String`
- ✅ `arg_count: i32`
- ✅ `fixed_fee: i64`
- ✅ `from_stack_item()` parsing
- ✅ Unit tests

#### 3. ContractParameter i64 Support
**File**: `src/neo_types/contract/contract_parameter.rs` ✅ UPDATED
- ✅ Added `impl From<i64> for ContractParameter`

### ✅ Phase 2: Native Contracts (COMPLETED)

#### 1. Treasury Native Contract
**File**: `src/neo_contract/treasury.rs` ✅ CREATED
- ✅ `verify()` - Check if transaction is signed by committee
- ✅ `supported_standards()` - Returns NEP-26, NEP-27, NEP-30
- ✅ Unit tests

#### 2. Notary Native Contract
**File**: `src/neo_contract/notary.rs` ✅ CREATED
- ✅ `balanceOf()` - Get notary deposit balance
- ✅ `expirationOf()` - Get deposit expiration
- ✅ `getMaxNotValidBeforeDelta()` - Get max not valid before delta
- ✅ `lockDepositUntil()` - Lock deposit
- ✅ `withdraw()` - Withdraw deposit
- ✅ `setMaxNotValidBeforeDelta()` - Set max delta
- ✅ `NotaryDeposit` struct with `from_stack_item()`
- ✅ Unit tests

#### 3. PolicyContract Updates
**File**: `src/neo_contract/policy_contract.rs` ✅ UPDATED
- ✅ **New constants**:
  - `MAX_MILLISECONDS_PER_BLOCK` = 30,000
  - `MAX_MAX_VALID_UNTIL_BLOCK_INCREMENT` = 86,400
  - `MAX_MAX_TRACEABLE_BLOCKS` = 2,102,400
  - `DEFAULT_NOTARY_ASSISTED_ATTRIBUTE_FEE` = 10,000,000
  - And many more...

- ✅ **New methods (HF_Echidna)**:
  - `get_milliseconds_per_block()`
  - `set_milliseconds_per_block()`
  - `get_max_valid_until_block_increment()`
  - `set_max_valid_until_block_increment()`
  - `get_max_traceable_blocks()`
  - `set_max_traceable_blocks()`
  - `get_attribute_fee()`
  - `set_attribute_fee()`

- ✅ **New methods (HF_Faun)**:
  - `get_exec_pico_fee_factor()`
  - `set_whitelist_fee_contract()`
  - `remove_whitelist_fee_contract()`
  - `recover_fund()`

- ✅ Unit tests

#### 4. NeoToken Updates
**File**: `src/neo_contract/neo_token.rs` ✅ UPDATED
- ✅ `get_committee_address()` - Get committee multi-sig address (HF_Cockatrice)
- ✅ `get_candidate_vote()` - Get votes for a specific candidate
- ✅ `AccountState.last_gas_per_vote` - Added for voting reward tracking
- ✅ Updated `get_account_state()` to parse `last_gas_per_vote`

#### 5. Module Exports
**File**: `src/neo_contract/mod.rs` ✅ UPDATED
- ✅ Added `pub use treasury::*`
- ✅ Added `pub use notary::*`
- ✅ Added `mod treasury`
- ✅ Added `mod notary`

**File**: `src/neo_types/mod.rs` ✅ UPDATED
- ✅ Added `pub use hardfork::*`
- ✅ Added `pub use whitelisted_contract::*`
- ✅ Added `pub mod hardfork`
- ✅ Added `pub mod whitelisted_contract`

### ✅ Phase 3: Crypto Updates (COMPLETED)

#### CryptoLib Module
**File**: `src/neo_crypto/crypto_lib.rs` ✅ CREATED
- ✅ `sha3_512()` - SHA3-512 hash function (HF_Faun)
- ✅ `blake2b_512()` - Blake2b-512 hash function (HF_Faun)
- ✅ `verify_with_ed25519()` - Ed25519 signature verification (HF_Echidna)
- ✅ `recover_secp256k1()` - Recover public key from secp256k1 signature (HF_Echidna)
- ✅ `CryptoLibHashable` trait - Extension trait for hash methods on byte slices
- ✅ Unit tests (7 tests)

#### Dependencies Added
**File**: `Cargo.toml` ✅ UPDATED
- ✅ `sha3 = "0.10.8"` - For SHA3-512
- ✅ `blake2 = "0.10.6"` - For Blake2b-512
- ✅ `ed25519-dalek = "2.1.1"` - For Ed25519 signature verification
- ✅ Updated `k256` with `ecdsa-core` feature for key recovery

**File**: `src/neo_crypto/mod.rs` ✅ UPDATED
- ✅ Added `pub mod crypto_lib`
- ✅ Re-exported `sha3_512`, `blake2b_512`, `verify_with_ed25519`, `recover_secp256k1`, `CryptoLibHashable`

### ✅ Phase 4: Protocol Settings (COMPLETED)

**File**: `src/neo_config/config.rs` ✅ UPDATED
- ✅ Added HF_Echidna with **actual mainnet activation height** (7,300,000)
- ✅ Added HF_Echidna with **actual testnet activation height** (5,870,000)
- ✅ Added `is_hardfork_enabled(hardfork, block_height)` - Check if hardfork is active
- ✅ Added `get_hardfork_height(hardfork)` - Get activation height for a hardfork
- ✅ Added `set_hardfork_height(hardfork, height)` - Set activation height for a hardfork
- ✅ Added `testnet()` configuration with T5 testnet values
- ✅ Updated `mainnet()` with Neo 3.9 hardfork heights

## Testing Results

All implemented features have passing unit tests:
- ✅ `neo_types::hardfork::tests` - 5 tests passed
- ✅ `neo_types::whitelisted_contract::tests` - 2 tests passed
- ✅ `neo_contract::treasury::tests` - 3 tests passed
- ✅ `neo_contract::notary::tests` - 5 tests passed
- ✅ `neo_contract::policy_contract::tests` - 2 tests passed
- ✅ `neo_crypto::crypto_lib::tests` - 7 tests passed

**Neo 3.9 specific tests: 24 passed**
**Full library test suite: 381 passed**

## Files Created
1. ✅ `src/neo_types/hardfork.rs`
2. ✅ `src/neo_types/whitelisted_contract.rs`
3. ✅ `src/neo_contract/treasury.rs`
4. ✅ `src/neo_contract/notary.rs`
5. ✅ `src/neo_crypto/crypto_lib.rs`

## Files Modified
1. ✅ `src/neo_contract/policy_contract.rs`
2. ✅ `src/neo_contract/neo_token.rs`
3. ✅ `src/neo_contract/mod.rs`
4. ✅ `src/neo_types/mod.rs`
5. ✅ `src/neo_types/contract/contract_parameter.rs`
6. ✅ `src/neo_crypto/mod.rs`
7. ✅ `Cargo.toml`
8. ✅ `src/neo_config/config.rs`

## Implementation Complete ✅ Production Ready

All 4 phases of the Neo 3.9 upgrade have been implemented with production-ready values:

| Phase | Status | Description |
|-------|--------|-------------|
| 1. Core Types | ✅ Complete | Hardfork enum, WhitelistedContract type |
| 2. Native Contracts | ✅ Complete | Treasury, Notary, PolicyContract, NeoToken updates |
| 3. Crypto Updates | ✅ Complete | SHA3-512, Blake2b-512, Ed25519, secp256k1 recovery |
| 4. Protocol Settings | ✅ Complete | Hardfork configuration with real block heights |

### Hardfork Activation Heights

| Hardfork | Mainnet | Testnet |
|----------|---------|---------|
| HF_Aspidochelone | 1,730,000 | 210,000 |
| HF_Basilisk | 4,120,000 | 2,680,000 |
| HF_Cockatrice | 5,450,000 | 3,967,000 |
| HF_Domovoi | 5,570,000 | 4,144,000 |
| HF_Echidna | 7,300,000 | 5,870,000 |
| HF_Faun | Not activated | Not activated |
| HF_Gorgon | Not activated | Not activated |

## Future Work
1. 🔲 Update HF_Faun and HF_Gorgon block heights when announced
2. 🔲 Integration testing with Neo 3.9 nodes
3. 🔲 Backward compatibility testing with pre-Echidna nodes
