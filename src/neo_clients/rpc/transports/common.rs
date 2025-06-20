// Code adapted from: https://github.com/althea-net/guac_rs/tree/master/web3/src/jsonrpc

use std::fmt;

use base64::{engine::general_purpose, Engine};
use jsonwebtoken::{encode, errors::Error, get_current_timestamp, Algorithm, EncodingKey, Header};
use primitive_types::U256;
use serde::{
	de::{self, MapAccess, Unexpected, Visitor},
	Deserialize, Serialize,
};
use serde_json::{value::RawValue, Value};
use thiserror::Error;

use neo3::prelude::Bytes;

/// A JSON-RPC 2.0 error
#[derive(Deserialize, Debug, Clone, Error, PartialEq)]
pub struct JsonRpcError {
	/// The error code
	pub code: i64,
	/// The error message
	pub message: String,
	/// Additional data
	pub data: Option<Value>,
}

/// Recursively traverses the value, looking for hex data that it can extract.
///
/// Inspired by neo-js logic:
/// <https://github.com/neo-io/neo.js/blob/9f990c57f0486728902d4b8e049536f2bb3487ee/packages/providers/src.ts/json-rpc-provider.ts#L25-L53>
fn spelunk_revert(value: &Value) -> Option<Bytes> {
	match value {
		Value::String(s) => Some(s.as_bytes().to_vec()),
		Value::Object(o) => o.values().flat_map(spelunk_revert).next(),
		_ => None,
	}
}

impl JsonRpcError {
	/// Determine if the error output of the `neo_call` RPC request is a revert
	///
	/// Note that this may return false positives if called on an error from
	/// other RPC requests
	pub fn is_revert(&self) -> bool {
		// Ganache says "revert" not "reverted"
		self.message.contains("revert")
	}

	/// Attempt to extract revert data from the JsonRpcError be recursively
	/// traversing the error's data field
	///
	/// This returns the first hex it finds in the data object, and its
	/// behavior may change with `serde_json` internal changes.
	///
	/// If no hex object is found, it will return an empty bytes IFF the error
	/// is a revert
	///
	/// Inspired by neo-js logic:
	/// <https://github.com/neo-io/neo.js/blob/9f990c57f0486728902d4b8e049536f2bb3487ee/packages/providers/src.ts/json-rpc-provider.ts#L25-L53>
	pub fn as_revert_data(&self) -> Option<Bytes> {
		self.is_revert()
			.then(|| self.data.as_ref().and_then(spelunk_revert).unwrap_or_default())
	}

	// Decode revert data (if any) into a decodeable type
	// pub fn decode_revert_data<E: AbiDecode>(&self) -> Option<E> {
	// 	E::decode(&self.as_revert_data()?).ok()
	// }
}

impl fmt::Display for JsonRpcError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "(code: {}, message: {}, data: {:?})", self.code, self.message, self.data)
	}
}

fn is_zst<T>(_t: &T) -> bool {
	std::mem::size_of::<T>() == 0
}

#[derive(Serialize, Deserialize, Debug)]
/// A JSON-RPC request
pub struct Request<'a, T> {
	id: u64,
	jsonrpc: &'a str,
	method: &'a str,
	#[serde(skip_serializing_if = "is_zst")]
	params: T,
}

impl<'a, T> Request<'a, T> {
	/// Creates a new JSON RPC request
	pub fn new(id: u64, method: &'a str, params: T) -> Self {
		Self { id, jsonrpc: "2.0", method, params }
	}
}

/// A JSON-RPC response
#[derive(Debug)]
pub enum Response<'a> {
	Success { id: u64, result: &'a RawValue },
	Error { id: u64, error: JsonRpcError },
	Notification { method: &'a str, params: Params<'a> },
}

#[derive(Deserialize, Debug)]
pub struct Params<'a> {
	pub subscription: U256,
	#[serde(borrow)]
	pub result: &'a RawValue,
}

// Note: Custom deserialization is required because serde's untagged enum support
// has limitations with borrowing (see https://github.com/serde-rs/serde/issues/1183)
impl<'de: 'a, 'a> Deserialize<'de> for Response<'a> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		struct ResponseVisitor<'a>(&'a ());
		impl<'de: 'a, 'a> Visitor<'de> for ResponseVisitor<'a> {
			type Value = Response<'a>;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("a valid jsonrpc 2.0 response object")
			}

			fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
			where
				A: MapAccess<'de>,
			{
				let mut jsonrpc = false;

				// response & error
				let mut id = None;
				// only response
				let mut result = None;
				// only error
				let mut error = None;
				// only notification
				let mut method = None;
				let mut params = None;

				while let Some(key) = map.next_key()? {
					match key {
						"jsonrpc" => {
							if jsonrpc {
								return Err(de::Error::duplicate_field("jsonrpc"));
							}

							let value = map.next_value()?;
							if value != "2.0" {
								return Err(de::Error::invalid_value(
									Unexpected::Str(value),
									&"2.0",
								));
							}

							jsonrpc = true;
						},
						"id" => {
							if id.is_some() {
								return Err(de::Error::duplicate_field("id"));
							}

							let value: u64 = map.next_value()?;
							id = Some(value);
						},
						"result" => {
							if result.is_some() {
								return Err(de::Error::duplicate_field("result"));
							}

							let value: &RawValue = map.next_value()?;
							result = Some(value);
						},
						"error" => {
							if error.is_some() {
								return Err(de::Error::duplicate_field("error"));
							}

							let value: JsonRpcError = map.next_value()?;
							error = Some(value);
						},
						"method" => {
							if method.is_some() {
								return Err(de::Error::duplicate_field("method"));
							}

							let value: &str = map.next_value()?;
							method = Some(value);
						},
						"params" => {
							if params.is_some() {
								return Err(de::Error::duplicate_field("params"));
							}

							let value: Params = map.next_value()?;
							params = Some(value);
						},
						key =>
							return Err(de::Error::unknown_field(
								key,
								&["id", "jsonrpc", "result", "error", "params", "method"],
							)),
					}
				}

				// jsonrpc version must be present in all responses
				if !jsonrpc {
					return Err(de::Error::missing_field("jsonrpc"));
				}

				match (id, result, error, method, params) {
					(Some(id), Some(result), None, None, None) =>
						Ok(Response::Success { id, result }),
					(Some(id), None, Some(error), None, None) => Ok(Response::Error { id, error }),
					(None, None, None, Some(method), Some(params)) =>
						Ok(Response::Notification { method, params }),
					_ => Err(de::Error::custom(
						"response must be either a success/error or notification object",
					)),
				}
			}
		}

		deserializer.deserialize_map(ResponseVisitor(&()))
	}
}

/// Basic or bearer authentication in http or websocket transport
///
/// Use to inject username and password or an auth token into requests
#[derive(Clone, Debug)]
pub enum Authorization {
	/// HTTP Basic Auth
	Basic(String),
	/// Bearer Auth
	Bearer(String),
	/// If you need to override the Authorization header value
	Raw(String),
}

impl Authorization {
	/// Make a new basic auth
	pub fn basic(username: impl AsRef<str>, password: impl AsRef<str>) -> Self {
		let username = username.as_ref();
		let password = password.as_ref();
		let auth_secret = general_purpose::STANDARD.encode(format!("{username}:{password}"));
		Self::Basic(auth_secret)
	}

	/// Make a new bearer auth
	pub fn bearer(token: impl Into<String>) -> Self {
		Self::Bearer(token.into())
	}

	/// Override the Authorization header with your own string
	pub fn raw(token: impl Into<String>) -> Self {
		Self::Raw(token.into())
	}
}

impl fmt::Display for Authorization {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Authorization::Basic(auth_secret) => write!(f, "Basic {auth_secret}"),
			Authorization::Bearer(token) => write!(f, "Bearer {token}"),
			Authorization::Raw(s) => write!(f, "{s}"),
		}
	}
}

/// Default algorithm used for JWT token signing.
const DEFAULT_ALGORITHM: Algorithm = Algorithm::HS256;

/// JWT secret length in bytes.
pub const JWT_SECRET_LENGTH: usize = 32;

/// Generates a bearer token from a JWT secret
pub struct JwtKey([u8; JWT_SECRET_LENGTH]);

impl JwtKey {
	/// Wrap given slice in `Self`. Returns an error if slice.len() != `JWT_SECRET_LENGTH`.
	pub fn from_slice(key: &[u8]) -> Result<Self, String> {
		if key.len() != JWT_SECRET_LENGTH {
			return Err(format!(
				"Invalid key length. Expected {} got {}",
				JWT_SECRET_LENGTH,
				key.len()
			));
		}
		let mut res = [0; JWT_SECRET_LENGTH];
		res.copy_from_slice(key);
		Ok(Self(res))
	}

	/// Decode the given string from hex (no 0x prefix), and attempt to create a key from it.
	pub fn from_hex(hex: &str) -> Result<Self, String> {
		let bytes = hex::decode(hex).map_err(|e| format!("Invalid hex: {}", e))?;
		Self::from_slice(&bytes)
	}

	/// Returns a reference to the underlying byte array.
	pub fn as_bytes(&self) -> &[u8; JWT_SECRET_LENGTH] {
		&self.0
	}

	/// Consumes the key, returning its underlying byte array.
	pub fn into_bytes(self) -> [u8; JWT_SECRET_LENGTH] {
		self.0
	}
}

/// Contains the JWT secret and claims parameters.
pub struct JwtAuth {
	key: EncodingKey,
	id: Option<String>,
	clv: Option<String>,
}

impl JwtAuth {
	/// Create a new [JwtAuth] from a secret key, and optional `id` and `clv` claims.
	pub fn new(secret: JwtKey, id: Option<String>, clv: Option<String>) -> Self {
		Self { key: EncodingKey::from_secret(secret.as_bytes()), id, clv }
	}

	/// Generate a JWT token with `claims.iat` set to current time.
	pub fn generate_token(&self) -> Result<String, Error> {
		let claims = self.generate_claims_at_timestamp();
		self.generate_token_with_claims(&claims)
	}

	/// Generate a JWT token with the given claims.
	fn generate_token_with_claims(&self, claims: &Claims) -> Result<String, Error> {
		let header = Header::new(DEFAULT_ALGORITHM);
		encode(&header, claims, &self.key)
	}

	/// Generate a `Claims` struct with `iat` set to current time
	fn generate_claims_at_timestamp(&self) -> Claims {
		Claims { iat: get_current_timestamp(), id: self.id.clone(), clv: self.clv.clone() }
	}

	/// Validate a JWT token given the secret key and return the originally signed `TokenData`.
	pub fn validate_token(
		token: &str,
		secret: &JwtKey,
	) -> Result<jsonwebtoken::TokenData<Claims>, Error> {
		let mut validation = jsonwebtoken::Validation::new(DEFAULT_ALGORITHM);
		validation.validate_exp = false;
		validation.required_spec_claims.remove("exp");

		jsonwebtoken::decode::<Claims>(
			token,
			&jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
			&validation,
		)
		.map_err(Into::into)
	}
}

/// Claims struct as defined in <https://github.com/neo/execution-apis/blob/main/src/engine/authentication.md#jwt-claims>
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Claims {
	/// issued-at claim. Represented as seconds passed since UNIX_EPOCH.
	iat: u64,
	/// Optional unique identifier for the CL node.
	id: Option<String>,
	/// Optional client version for the CL node.
	clv: Option<String>,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn deser_response() {
		let _ =
			serde_json::from_str::<Response<'_>>(r#"{"jsonrpc":"2.0","result":19}"#).unwrap_err();
		let _ = serde_json::from_str::<Response<'_>>(r#"{"jsonrpc":"3.0","result":19,"id":1}"#)
			.unwrap_err();

		let response: Response<'_> =
			serde_json::from_str(r#"{"jsonrpc":"2.0","result":19,"id":1}"#).unwrap();

		match response {
			Response::Success { id, result } => {
				assert_eq!(id, 1);
				let result: u64 = serde_json::from_str(result.get()).unwrap();
				assert_eq!(result, 19);
			},
			_ => {
				assert!(false, "Expected Success response but got: {:?}", response);
			},
		}

		let response: Response<'_> = serde_json::from_str(
			r#"{"jsonrpc":"2.0","error":{"code":-32000,"message":"error occurred"},"id":2}"#,
		)
		.unwrap();

		match response {
			Response::Error { id, error } => {
				assert_eq!(id, 2);
				assert_eq!(error.code, -32000);
				assert_eq!(error.message, "error occurred");
				assert!(error.data.is_none());
			},
			_ => {
				assert!(false, "Expected Error response but got: {:?}", response);
			},
		}

		let response: Response<'_> =
			serde_json::from_str(r#"{"jsonrpc":"2.0","result":"0xfa","id":0}"#).unwrap();

		match response {
			Response::Success { id, result } => {
				assert_eq!(id, 0);
				let result: String = serde_json::from_str(result.get()).unwrap();
				assert_eq!(i64::from_str_radix(result.trim_start_matches("0x"), 16).unwrap(), 250);
			},
			_ => {
				assert!(false, "Expected Success response but got: {:?}", response);
			},
		}
	}

	#[test]
	fn ser_request() {
		let request: Request<()> = Request::new(0, "neo_chainId", ());
		assert_eq!(
			&serde_json::to_string(&request).unwrap(),
			r#"{"id":0,"jsonrpc":"2.0","method":"neo_chainId"}"#
		);

		let request: Request<()> = Request::new(300, "method_name", ());
		assert_eq!(
			&serde_json::to_string(&request).unwrap(),
			r#"{"id":300,"jsonrpc":"2.0","method":"method_name"}"#
		);

		let request: Request<u32> = Request::new(300, "method_name", 1);
		assert_eq!(
			&serde_json::to_string(&request).unwrap(),
			r#"{"id":300,"jsonrpc":"2.0","method":"method_name","params":1}"#
		);
	}

	#[test]
	fn test_roundtrip() {
		let jwt_secret = [42; 32];
		let auth = JwtAuth::new(
			JwtKey::from_slice(&jwt_secret).unwrap(),
			Some("42".into()),
			Some("Lighthouse".into()),
		);
		let claims = auth.generate_claims_at_timestamp();
		let token = auth.generate_token_with_claims(&claims).unwrap();

		assert_eq!(
			JwtAuth::validate_token(&token, &JwtKey::from_slice(&jwt_secret).unwrap())
				.unwrap()
				.claims,
			claims
		);
	}
}
