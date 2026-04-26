# Neo CLI DeFi Module

The DeFi module exposes production-backed NEP-17 token operations that use Neo N3 RPC calls and signed transactions.

## Commands

```text
neo-cli de-fi token <CONTRACT>
neo-cli de-fi balance <CONTRACT> <ADDRESS>
neo-cli de-fi transfer <TOKEN> <TO> <AMOUNT> [DATA]
```

`<CONTRACT>` and `<TOKEN>` accept known symbols such as `NEO` and `GAS`, or a contract script hash.

## Requirements

- A reachable Neo N3 RPC endpoint.
- A wallet for state-changing transfers.
- Enough GAS to pay transaction fees.

Protocol-specific DeFi workflows such as swaps, staking, governance, and liquidity management should use `neo-cli contract call` or `neo-cli contract invoke` with verified contract hashes and method parameters. Dedicated adapters should only be added when the target protocol contract addresses, method signatures, slippage and fee rules, and signing flow are covered by tests.
