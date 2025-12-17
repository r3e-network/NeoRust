# DeFi Commands in Neo CLI

This document describes the `de-fi` command group in `neo-cli`. The focus of this module is:

- **NEP-17 token helpers** (info + balance queries)
- **Protocol-specific command scaffolding** (Flamingo / NeoBurger / NeoCompound / GrandShare)

Some protocol integrations are still implemented as placeholders and will return a clear error explaining what’s needed for a production-grade implementation.

## Prerequisites

- Most commands require access to an RPC endpoint (MainNet/TestNet/custom).
- For balance queries, prefer passing an explicit address to avoid relying on wallet state.

Use `neo-cli de-fi --help` (and subcommand `--help`) to see the exact arguments for your version.

## Token Operations

### Get Token Information

```bash
neo-cli de-fi token NEO
neo-cli de-fi token GAS
neo-cli de-fi token 0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5
```

### Check Token Balance (Recommended: Provide Address)

```bash
neo-cli de-fi balance GAS NZKvXidwBhnV8rNXh2eXtpm5bH1rkofaDz
```

### Transfer Tokens

`transfer` is intended to build and submit a NEP-17 `transfer` call. This typically requires a loaded/unlocked account for signing.

```bash
neo-cli de-fi transfer GAS NZKvXidwBhnV8rNXh2eXtpm5bH1rkofaDz 10
neo-cli de-fi transfer NEO NZKvXidwBhnV8rNXh2eXtpm5bH1rkofaDz 100 "optional data"
```

## Flamingo Finance (Scaffold)

```bash
neo-cli de-fi flamingo swap GAS NEO 10
neo-cli de-fi flamingo add-liquidity NEO GAS 10 5
neo-cli de-fi flamingo remove-liquidity NEO GAS 10
neo-cli de-fi flamingo stake FLM 100
neo-cli de-fi flamingo claim-rewards
```

## NeoBurger (Scaffold)

```bash
neo-cli de-fi neo-burger wrap 100
neo-cli de-fi neo-burger unwrap 100
neo-cli de-fi neo-burger claim-gas
neo-cli de-fi neo-burger get-rate
```

## NeoCompound (Scaffold)

```bash
neo-cli de-fi neo-compound deposit GAS 50
neo-cli de-fi neo-compound withdraw GAS 25
neo-cli de-fi neo-compound compound GAS
neo-cli de-fi neo-compound get-apy GAS
```

## GrandShare (Scaffold)

```bash
neo-cli de-fi grand-share submit-proposal "Title" "Description" 1000
neo-cli de-fi grand-share vote 123 --approve
neo-cli de-fi grand-share fund-project 456 500
neo-cli de-fi grand-share claim-funds 456
```

