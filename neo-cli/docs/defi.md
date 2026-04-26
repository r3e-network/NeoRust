# DeFi Commands in Neo CLI

The `de-fi` command group provides production-backed NEP-17 token operations.

## Prerequisites

- A reachable Neo N3 RPC endpoint.
- A wallet for state-changing transfers.
- Enough GAS for transaction fees.

Use `neo-cli de-fi --help` and subcommand help for the exact arguments supported by your build.

## Token Information

```bash
neo-cli de-fi token NEO
neo-cli de-fi token GAS
neo-cli de-fi token 0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5
```

## Balance

```bash
neo-cli de-fi balance GAS NZKvXidwBhnV8rNXh2eXtpm5bH1rkofaDz
```

## Transfer

```bash
neo-cli de-fi transfer GAS NZKvXidwBhnV8rNXh2eXtpm5bH1rkofaDz 10
neo-cli de-fi transfer NEO NZKvXidwBhnV8rNXh2eXtpm5bH1rkofaDz 100 "optional data"
```

For protocol-specific swaps, staking, liquidity, or governance workflows, use `neo-cli contract call` or `neo-cli contract invoke` with verified contract hashes and method parameters.
