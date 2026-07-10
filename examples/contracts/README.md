# Contract Examples

These examples demonstrate Neo N3 contract manifests, script construction, method invocation,
events, deployment concepts, and the SDK's typed native and token contract wrappers.

Neo N3 contracts use a manifest ABI and NeoVM parameters rather than an Ethereum Solidity ABI.
Start with these SDK surfaces:

- `SmartContractTrait` for low-level read and invoke helpers.
- `FungibleTokenContract` and `NonFungibleTokenContract` for NEP-17 and NEP-11 contracts.
- `ContractParameter` and `ScriptBuilder` for explicit NeoVM calls.
- `ContractManagement` for native deployment and update operations.

Run an example from the workspace root:

```bash
cargo run -p examples-contracts --example neo_contract_interaction
cargo run -p examples-contracts --example methods
```

For Neo X EVM contracts, use the Alloy-backed APIs in `neo3::neo_x`; the bridge binding in
`src/neo_x/bridge/evm_bridge.rs` is a compact `alloy::sol!` example.
