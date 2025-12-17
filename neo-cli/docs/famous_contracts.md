# Working with Well-known Contracts and Tokens

Historically, this repository referenced a separate `neo-cli famous ...` command group for “well-known” contracts.

The current CLI surfaces these workflows primarily through:

- `neo-cli de-fi token` for common token metadata lookups (by symbol or script hash)
- `neo-cli de-fi balance` for quick balance checks
- `neo-cli contract ...` for generic contract deployment/invocation flows

## Token Shortcuts

```bash
# Resolve common symbols (NEO / GAS / FLM / bNEO, etc.) or use a script hash directly
neo-cli de-fi token NEO
neo-cli de-fi token GAS
neo-cli de-fi token 0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5
```

## Balance Checks

```bash
neo-cli de-fi balance GAS NZKvXidwBhnV8rNXh2eXtpm5bH1rkofaDz
```

## Generic Contract Workflows

Use `neo-cli contract --help` for the full set of contract-related commands supported by your build.

```bash
neo-cli contract list-native-contracts
```

