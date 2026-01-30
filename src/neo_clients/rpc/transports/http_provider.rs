// Code adapted from: https://github.com/althea-net/guac_rs/tree/master/web3/src/jsonrpc

use std::sync::atomic::{AtomicU64, Ordering};

use async_trait::async_trait;
use futures_util::StreamExt;
use http::HeaderValue;
use log::debug;
use reqwest::{header, Client, Error as ReqwestError};
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;
use url::Url;

use super::common::{JsonRpcError, Request, Response};
use crate::neo_clients::{Authorization, JsonRpcProvider, ProviderError};
use neo3::config::NeoConstants;

const MAX_ERROR_TEXT_BYTES: usize = 4 * 1024;

/// A low-level JSON-RPC Client over HTTP.
///
/// # Example
///
/// ```no_run
/// use neo3::neo_clients::{HttpProvider, RpcClient, APITrait};
/// use neo3::neo_config::NeoConstants;
/// use primitive_types::H256;
///
/// # async fn foo() -> Result<(), Box<dyn std::error::Error>> {
/// let provider = HttpProvider::new(NeoConstants::SEED_1)?;
/// let client = RpcClient::new(provider);
/// let block = client.get_block(H256::zero(), false).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct HttpProvider {
	id: AtomicU64,
	client: Client,
	url: Url,
}

#[derive(Error)]
/// Error thrown when sending an HTTP request
pub enum ClientError {
	/// Thrown if the request failed
	#[error(transparent)]
	ReqwestError(#[from] ReqwestError),
	#[error(transparent)]
	/// Thrown if the response could not be parsed
	JsonRpcError(#[from] JsonRpcError),

	#[error("Deserialization Error: {err}. Response: <redacted>")]
	/// Serde JSON Error
	SerdeJson {
		/// Underlying error
		err: serde_json::Error,
		/// The contents of the HTTP response that could not be deserialized
		text: String,
	},
}

impl std::fmt::Debug for ClientError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ClientError::ReqwestError(err) => f.debug_tuple("ReqwestError").field(err).finish(),
			ClientError::JsonRpcError(err) => f.debug_tuple("JsonRpcError").field(err).finish(),
			ClientError::SerdeJson { err, text } => f
				.debug_struct("SerdeJson")
				.field("err", err)
				.field("text_len", &text.len())
				.finish(),
		}
	}
}

impl From<ClientError> for ProviderError {
	fn from(src: ClientError) -> Self {
		match src {
			ClientError::ReqwestError(err) => ProviderError::HTTPError(err.into()),
			ClientError::JsonRpcError(err) => ProviderError::JsonRpcError(err),
			ClientError::SerdeJson { err, text } => {
				// Avoid logging raw response bodies (may be large and/or contain sensitive data).
				debug!("SerdeJson Error: {:#?} (response_len={})", err, text.len());
				ProviderError::SerdeJson(err)
			},
		}
	}
}

#[cfg_attr(target_arch = "wasm32", async_trait(? Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl JsonRpcProvider for HttpProvider {
	type Error = ClientError;

	async fn fetch<T: Serialize + Send + Sync, R: DeserializeOwned>(
		&self,
		method: &str,
		params: T,
	) -> Result<R, ClientError> {
		let next_id = self.id.fetch_add(1, Ordering::SeqCst);
		let payload = Request::new(next_id, method, params);

		let mut request = self.client.post(self.url.as_ref()).json(&payload);
		if let Some(timeout) = NeoConstants::rpc_request_timeout() {
			request = request.timeout(timeout);
		}
		let res = request.send().await?;
		let max_response_size = NeoConstants::max_rpc_message_size();
		let body =
			collect_body_with_limit(res.content_length(), res.bytes_stream(), max_response_size)
				.await?;

		let raw = match serde_json::from_slice(&body) {
			Ok(Response::Success { result, .. }) => result.to_owned(),
			Ok(Response::Error { error, .. }) => return Err(error.into()),
			Ok(_) => {
				let err = ClientError::SerdeJson {
					err: serde::de::Error::custom("unexpected notification over HTTP transport"),
					text: String::from_utf8_lossy(&body[..body.len().min(MAX_ERROR_TEXT_BYTES)])
						.to_string(),
				};
				return Err(err);
			},
			Err(err) => {
				return Err(ClientError::SerdeJson {
					err,
					text: String::from_utf8_lossy(&body[..body.len().min(MAX_ERROR_TEXT_BYTES)])
						.to_string(),
				})
			},
		};

		let res = serde_json::from_str(raw.get()).map_err(|err| {
			let raw_bytes = raw.get().as_bytes();
			let preview_len = raw_bytes.len().min(MAX_ERROR_TEXT_BYTES);
			ClientError::SerdeJson {
				err,
				text: String::from_utf8_lossy(&raw_bytes[..preview_len]).to_string(),
			}
		})?;

		Ok(res)
	}
}

async fn collect_body_with_limit<S>(
	content_length: Option<u64>,
	mut stream: S,
	max_response_size: usize,
) -> Result<Vec<u8>, ClientError>
where
	S: futures_util::stream::Stream<Item = Result<bytes::Bytes, ReqwestError>> + Unpin,
{
	if let Some(len) = content_length {
		let max = max_response_size as u64;
		if len > max {
			return Err(ClientError::SerdeJson {
				err: serde::de::Error::custom(format!(
					"HTTP response too large ({} bytes), max is {} bytes",
					len, max
				)),
				text: format!("<response Content-Length {} exceeds max {} bytes>", len, max),
			});
		}
	}

	let mut body: Vec<u8> = Vec::new();
	while let Some(chunk) = stream.next().await {
		let chunk = chunk.map_err(ClientError::ReqwestError)?;
		if body.len().saturating_add(chunk.len()) > max_response_size {
			let preview_len = body.len().min(MAX_ERROR_TEXT_BYTES);
			return Err(ClientError::SerdeJson {
				err: serde::de::Error::custom(format!(
					"HTTP response exceeded max size ({} bytes)",
					max_response_size
				)),
				text: String::from_utf8_lossy(&body[..preview_len]).to_string(),
			});
		}
		body.extend_from_slice(&chunk);
	}

	Ok(body)
}

impl Default for HttpProvider {
	/// Default HTTP Provider from SEED_1
	/// 
	/// # Panics
	/// Panics if NeoConstants::SEED_1 is not a valid URL. This is a compile-time
	/// constant and should always be valid.
	fn default() -> Self {
		let url = Url::parse(NeoConstants::SEED_1).unwrap_or_else(|e| {
			panic!(
				"NeoConstants::SEED_1 ('{}') is not a valid URL: {}. \
				This is a bug in the SDK configuration.",
				NeoConstants::SEED_1, e
			)
		});

		Self::new_with_client(url, Client::new())
	}
}

impl HttpProvider {
	/// Initializes a new HTTP Client
	///
	/// # Example
	///
	/// ```
	/// use neo3::neo_clients::HttpProvider;
	/// use url::Url;
	///
	/// // Using a string
	/// let provider = HttpProvider::new("http://localhost:10332")?;
	///
	/// // Using a &str
	/// let provider = HttpProvider::new("http://localhost:10332")?;
	///
	/// // Using a Url
	/// let url = Url::parse("http://localhost:10332").unwrap();
	/// let provider = HttpProvider::new(url)?;
	/// # Ok::<(), Box<dyn std::error::Error>>(())
	/// ```
	pub fn new<T: TryInto<Url>>(url: T) -> Result<Self, T::Error> {
		let url = url.try_into()?;
		Ok(Self::new_with_client(url, Client::new()))
	}

	/// The Url to which requests are made
	pub fn url(&self) -> &Url {
		&self.url
	}

	/// Mutable access to the Url to which requests are made
	pub fn url_mut(&mut self) -> &mut Url {
		&mut self.url
	}

	/// Initializes a new HTTP Client with authentication
	///
	/// # Example
	///
	/// ```
	/// use neo3::neo_clients::{HttpProvider, Authorization};
	/// use url::Url;
	///
	/// let url = Url::parse("http://localhost:10332").unwrap();
	/// let provider = HttpProvider::new_with_auth(url, Authorization::basic("admin", "good_password"))?;
	/// # Ok::<(), Box<dyn std::error::Error>>(())
	/// ```
	pub fn new_with_auth(
		url: impl Into<Url>,
		auth: Authorization,
	) -> Result<Self, HttpClientError> {
		let mut auth_value = HeaderValue::from_str(&auth.to_string())?;
		auth_value.set_sensitive(true);

		let mut headers = reqwest::header::HeaderMap::new();
		headers.insert(reqwest::header::AUTHORIZATION, auth_value);

		let client = Client::builder().default_headers(headers).build()?;

		Ok(Self::new_with_client(url, client))
	}

	/// Allows to customize the provider by providing your own http client
	///
	/// # Example
	///
	/// ```
	/// use neo3::neo_clients::HttpProvider;
	/// use url::Url;
	///
	/// let url = Url::parse("http://localhost:10332").unwrap();
	/// let client = reqwest::Client::builder().build().unwrap();
	/// let provider = HttpProvider::new_with_client(url, client);
	/// ```
	pub fn new_with_client(url: impl Into<Url>, client: reqwest::Client) -> Self {
		Self { id: AtomicU64::new(1), client, url: url.into() }
	}
}

impl Clone for HttpProvider {
	fn clone(&self) -> Self {
		Self { id: AtomicU64::new(1), client: self.client.clone(), url: self.url.clone() }
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use futures_util::stream::Stream;
	use std::{
		pin::Pin,
		task::{Context, Poll},
	};

	#[tokio::test]
	async fn rejects_oversized_content_length_without_reading_body() {
		struct PanicsIfPolled;

		impl Stream for PanicsIfPolled {
			type Item = Result<bytes::Bytes, ReqwestError>;

			fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
				panic!("body stream should not be polled when Content-Length exceeds max size");
			}
		}

		let max = NeoConstants::max_rpc_message_size();
		let err = collect_body_with_limit(Some((max as u64) + 1), PanicsIfPolled, max)
			.await
			.unwrap_err();
		assert!(matches!(err, ClientError::SerdeJson { .. }));
	}
}

#[derive(Error, Debug)]
/// Error thrown when dealing with Http clients
pub enum HttpClientError {
	/// Thrown if unable to build headers for client
	#[error(transparent)]
	InvalidHeader(#[from] header::InvalidHeaderValue),

	/// Thrown if unable to build client
	#[error(transparent)]
	ClientBuild(#[from] reqwest::Error),
}
