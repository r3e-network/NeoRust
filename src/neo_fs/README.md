# NeoFS Module

This module provides functionality for interacting with NeoFS, Neo's decentralized object storage system.

## Features

- Container management: Create, get, list, and delete containers (REST gateway)
- Object operations: Upload, download, list, and delete objects (REST gateway)
- Multipart uploads: Scaffold for multipart flows via the REST gateway
- Access control: Types are present, but token signing/auth is not implemented yet

## Usage

### Basic Usage

```rust
use neo3::prelude::*;
use neo3::neo_fs::client::{NeoFSClient, NeoFSConfig, DEFAULT_TESTNET_REST_API};
use neo3::neo_fs::{NeoFSAuth, NeoFSService};
use std::env;

async fn example() -> Result<(), Box<dyn std::error::Error>> {
    // Configure NeoFS client
    let endpoint = env::var("NEOFS_ENDPOINT").unwrap_or_else(|_| DEFAULT_TESTNET_REST_API.to_string());
    let wallet_address = env::var("NEOFS_WALLET").unwrap_or_else(|_| "owner-demo-address".to_string());

    let config = NeoFSConfig {
        endpoint,
        auth: Some(NeoFSAuth {
            wallet_address,
            private_key: None, // request signing not implemented; do not provide a private key
        }),
        timeout_sec: 10,
        insecure: false,
    };

    let client = NeoFSClient::new(config);

    // List containers for the configured owner identifier.
    // Depending on the gateway, this may require additional auth/session integration.
    let _containers = client.list_containers().await?;

    Ok(())
}
```

### Multipart Uploads

For large files, you can use multipart uploads:

```rust
// Initiate multipart upload (gateway/session support required server-side)
let upload = client.initiate_multipart_upload(&container_id, &object).await?;

// Upload parts
let part1 = client.upload_part(&upload, 1, data1).await?;
let part2 = client.upload_part(&upload, 2, data2).await?;

// Complete multipart upload
let result = client.complete_multipart_upload(&upload, vec![part1, part2]).await?;
```

## Access Control

NeoFS supports fine-grained access control through ACLs:

```rust
// Token signing/auth is not implemented yet. These calls currently return NotImplemented.
// let permissions = vec![AccessPermission::GetObject, AccessPermission::PutObject];
// let token = client.create_bearer_token(&container_id, permissions, 3600).await?;
```

## Implementation Status

> **Important Note**: NeoFS support in this SDK is currently **experimental** and focuses on a
> REST-style client scaffold. Authentication and token signing/verification are not implemented
> yet; providing `NeoFSAuth.private_key` is intentionally rejected to avoid a false sense of security.

## Examples

See the [examples directory](../../examples/neo_fs/) for complete usage examples.
