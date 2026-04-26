# NeoFS Module

This module provides REST and HTTP gateway access for NeoFS, Neo's decentralized object storage system.

## Features

- Container requests through a configured REST gateway.
- Object upload, download, listing, and deletion through gateway endpoints.
- Multipart upload helpers for gateways that expose multipart endpoints.
- Access-control data types for callers that integrate a native NeoFS signer.

Native signed operations such as bearer-token minting, session-token negotiation, and owner-only ACL updates require a native NeoFS backend. The REST gateway client reports those operations as unsupported instead of generating credentials that a NeoFS node would not accept.

## Usage

```rust
use neo3::neo_fs::client::{NeoFSClient, NeoFSConfig, DEFAULT_TESTNET_REST_API};
use neo3::neo_fs::{NeoFSAuth, NeoFSService};
use std::env;

async fn example() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = env::var("NEOFS_ENDPOINT")
        .unwrap_or_else(|_| DEFAULT_TESTNET_REST_API.to_string());
    let wallet_address = env::var("NEOFS_WALLET")
        .unwrap_or_else(|_| "owner-demo-address".to_string());

    let config = NeoFSConfig {
        endpoint,
        auth: Some(NeoFSAuth {
            wallet_address,
            private_key: None,
        }),
        timeout_sec: 10,
        insecure: false,
    };

    let client = NeoFSClient::new(config);
    let containers = client.list_containers().await?;
    println!("containers: {}", containers.len());
    Ok(())
}
```

## Multipart Uploads

```rust
let upload = client.initiate_multipart_upload(&container_id, &object).await?;
let part1 = client.upload_part(&upload, 1, data1).await?;
let part2 = client.upload_part(&upload, 2, data2).await?;
let result = client.complete_multipart_upload(&upload, vec![part1, part2]).await?;
println!("object id: {}", result.object_id);
```

See the [examples directory](../../examples/neo_fs/) for complete usage examples.
