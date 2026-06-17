// Copyright (c) 2023-2025 R3E Network
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Neo File Storage (NeoFS)
//!
//! NeoFS is a decentralized distributed object storage network integrated with
//! the Neo Blockchain. It provides a robust platform for storing, retrieving,
//! and managing digital assets with blockchain-level security.
//!
//! ## Overview
//!
//! This module provides Rust bindings to interact with NeoFS services, including:
//!
//! - **Container Management**: Create, retrieve, list, and delete NeoFS containers
//! - **Object Operations**: Upload, download, and manage objects in containers
//! - **Access Control**: Manage permissions and generate access tokens
//! - **Extended Features**: Support for multipart uploads and specialized storage operations
//!
//! ## Backend Scope
//!
//! The bundled client targets NeoFS REST and HTTP gateways. Gateway-backed object and container
//! requests are sent to the configured endpoint and parsed strictly. Native signed operations
//! such as bearer-token minting, session-token negotiation, and owner-only ACL updates require a
//! native NeoFS signer/backend; the REST gateway client reports those as unsupported instead of
//! fabricating credentials.
//!
//! ## Example (TestNet)
//!
//! ```no_run
//! use neo3::neo_fs::client::{NeoFSClient, NeoFSConfig, DEFAULT_TESTNET_REST_API};
//! use neo3::neo_fs::{NeoFSAuth, NeoFSService};
//! use std::env;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Set this to the owner identifier used by the NeoFS gateway you're targeting.
//!     let wallet_address = env::var("NEOFS_WALLET")?;
//!
//!     let config = NeoFSConfig::builder()
//!         .endpoint(DEFAULT_TESTNET_REST_API.to_string())
//!         .auth(NeoFSAuth { wallet_address, private_key: None })
//!         .timeout_sec(10)
//!         .insecure(false)
//!         .build();
//!
//!     let client = NeoFSClient::new(config);
//!     let containers = client.list_containers().await?;
//!     println!("Found {} containers", containers.len());
//!     Ok(())
//! }
//! ```

pub mod acl;
pub mod client;
pub mod container;
pub mod error;
pub mod object;
pub mod types;

pub use client::NeoFSClient;
pub use error::{NeoFSError, NeoFSResult};

// Re-export types directly from types module
pub use acl::{BearerToken, SessionToken};
pub use container::Container;
pub use object::{MultipartUpload, MultipartUploadResult, Object, Part};
pub use types::{
	AccessPermission, Attributes, ContainerId, ObjectId, ObjectType, OwnerId, PlacementPolicy,
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Default mainnet NeoFS gRPC endpoint.
pub const DEFAULT_MAINNET_ENDPOINT: &str = "grpc.mainnet.fs.neo.org:8082";

/// Default testnet NeoFS gRPC endpoint.
pub const DEFAULT_TESTNET_ENDPOINT: &str = "grpc.testnet.fs.neo.org:8082";

/// Default NeoFS endpoint (alias for mainnet gRPC endpoint).
pub const DEFAULT_ENDPOINT: &str = DEFAULT_MAINNET_ENDPOINT;

/// Default mainnet NeoFS HTTP gateway (typically used for object download/public access).
pub const DEFAULT_MAINNET_HTTP_GATEWAY: &str = "https://http.mainnet.fs.neo.org";

/// Default testnet NeoFS HTTP gateway (typically used for object download/public access).
pub const DEFAULT_TESTNET_HTTP_GATEWAY: &str = "https://http.testnet.fs.neo.org";

/// Default mainnet NeoFS REST API base URL.
pub const DEFAULT_MAINNET_REST_API: &str = "https://rest.mainnet.fs.neo.org";

/// Default testnet NeoFS REST API base URL.
pub const DEFAULT_TESTNET_REST_API: &str = "https://rest.testnet.fs.neo.org";

/// Represents a NeoFS service provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NeoFSConfig {
	/// The NeoFS service endpoint URL.
	///
	/// Prefer a REST API base URL (e.g. `DEFAULT_TESTNET_REST_API`). If a gRPC host:port string is
	/// provided (e.g. `grpc.testnet.fs.neo.org:8082`), `NeoFSClient::new` will map it to a default
	/// REST base URL.
	pub endpoint: String,
	/// Authentication information, typically from a Neo wallet
	pub auth: Option<NeoFSAuth>,
	/// Timeout for NeoFS operations in seconds
	pub timeout_sec: u64,
	/// Specifies whether to use insecure connections
	pub insecure: bool,
}

impl NeoFSConfig {
	/// Creates a new builder for the configuration
	pub fn builder() -> NeoFSConfigBuilder {
		NeoFSConfigBuilder::default()
	}
}

/// Builder for `NeoFSConfig`
#[derive(Debug, Default, Clone)]
pub struct NeoFSConfigBuilder {
	endpoint: Option<String>,
	auth: Option<NeoFSAuth>,
	timeout_sec: Option<u64>,
	insecure: Option<bool>,
}

impl NeoFSConfigBuilder {
	/// Sets the endpoint
	#[must_use]
	pub fn endpoint(mut self, val: String) -> Self {
		self.endpoint = Some(val);
		self
	}

	/// Sets the authentication information
	#[must_use]
	pub fn auth(mut self, val: NeoFSAuth) -> Self {
		self.auth = Some(val);
		self
	}

	/// Sets the timeout in seconds
	#[must_use]
	pub fn timeout_sec(mut self, val: u64) -> Self {
		self.timeout_sec = Some(val);
		self
	}

	/// Sets whether to use insecure connections
	#[must_use]
	pub fn insecure(mut self, val: bool) -> Self {
		self.insecure = Some(val);
		self
	}

	/// Builds the `NeoFSConfig`
	pub fn build(self) -> NeoFSConfig {
		let default = NeoFSConfig::default();
		NeoFSConfig {
			endpoint: self.endpoint.unwrap_or(default.endpoint),
			auth: self.auth.or(default.auth),
			timeout_sec: self.timeout_sec.unwrap_or(default.timeout_sec),
			insecure: self.insecure.unwrap_or(default.insecure),
		}
	}
}

impl Default for NeoFSConfig {
	fn default() -> Self {
		Self {
			endpoint: DEFAULT_TESTNET_REST_API.to_string(),
			auth: None,
			timeout_sec: 60,
			insecure: false,
		}
	}
}

/// Authentication information for NeoFS
#[derive(Clone, Serialize, Deserialize)]
pub struct NeoFSAuth {
	/// The wallet account used for authentication
	pub wallet_address: String,
	/// The private key to sign NeoFS requests
	pub private_key: Option<String>,
}

impl fmt::Debug for NeoFSAuth {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("NeoFSAuth")
			.field("wallet_address", &self.wallet_address)
			.field("private_key", &self.private_key.as_ref().map(|_| "<redacted>"))
			.finish()
	}
}

/// Service trait for interacting with NeoFS
#[async_trait]
pub trait NeoFSService {
	/// Creates a new container in NeoFS
	async fn create_container(&self, container: &Container) -> NeoFSResult<ContainerId>;

	/// Gets a container by its ID
	async fn get_container(&self, id: &ContainerId) -> NeoFSResult<Container>;

	/// Lists all containers owned by the current account
	async fn list_containers(&self) -> NeoFSResult<Vec<ContainerId>>;

	/// Deletes a container by its ID
	async fn delete_container(&self, id: &ContainerId) -> NeoFSResult<bool>;

	/// Uploads an object to a container
	async fn put_object(
		&self,
		container_id: &ContainerId,
		object: &Object,
	) -> NeoFSResult<ObjectId>;

	/// Gets an object by its ID from a container
	async fn get_object(
		&self,
		container_id: &ContainerId,
		object_id: &ObjectId,
	) -> NeoFSResult<Object>;

	/// Lists all objects in a container
	async fn list_objects(&self, container_id: &ContainerId) -> NeoFSResult<Vec<ObjectId>>;

	/// Deletes an object by its ID from a container
	async fn delete_object(
		&self,
		container_id: &ContainerId,
		object_id: &ObjectId,
	) -> NeoFSResult<bool>;

	/// Creates a bearer token for accessing objects in a container
	async fn create_bearer_token(
		&self,
		container_id: &ContainerId,
		permissions: Vec<AccessPermission>,
		expires_sec: u64,
	) -> NeoFSResult<BearerToken>;

	/// Gets a session token for the current account
	async fn get_session_token(&self) -> NeoFSResult<SessionToken>;

	/// Initiates a multipart upload for a large object
	async fn initiate_multipart_upload(
		&self,
		container_id: &ContainerId,
		object: &Object,
	) -> NeoFSResult<MultipartUpload>;

	/// Uploads a part of a multipart upload
	async fn upload_part(
		&self,
		upload: &MultipartUpload,
		part_number: u32,
		data: Vec<u8>,
	) -> NeoFSResult<Part>;

	/// Completes a multipart upload
	async fn complete_multipart_upload(
		&self,
		upload: &MultipartUpload,
		parts: Vec<Part>,
	) -> NeoFSResult<MultipartUploadResult>;

	/// Aborts a multipart upload
	async fn abort_multipart_upload(&self, upload: &MultipartUpload) -> NeoFSResult<bool>;
}
