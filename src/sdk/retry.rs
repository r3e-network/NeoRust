//! Lightweight retry helper used by the high-level [`crate::sdk::Neo`] API.
//!
//! This intentionally does not duplicate the lower-level
//! [`crate::neo_clients::RetryClient`] / [`crate::neo_clients::CircuitBreaker`]
//! infrastructure — it only wraps an async closure with a bounded retry budget
//! and provider-aware [`NeoError`] mapping so the call sites in
//! `sdk/mod.rs` stay readable.

use std::future::Future;
use std::time::Duration;

use crate::{neo_clients::ProviderError, neo_error::unified::NeoError};

/// Default base delay between retries used by the high-level SDK.
pub(crate) const DEFAULT_RETRY_DELAY: Duration = Duration::from_millis(250);

/// Retry an asynchronous, fallible operation up to `attempts` total times.
///
/// `attempts` is the total number of attempts (so `attempts = 1` means *no*
/// retry — the operation runs exactly once). Failures are logged via
/// `tracing` at `warn` level on every non-terminal attempt, and the final
/// error keeps its provider classification with `context` woven into the message. Deterministic
/// failures return immediately without consuming the retry budget.
pub(crate) async fn retry_network<T, F, Fut>(
	context: &str,
	attempts: u32,
	delay: Duration,
	mut operation: F,
) -> Result<T, NeoError>
where
	F: FnMut() -> Fut,
	Fut: Future<Output = Result<T, ProviderError>>,
{
	let attempts = attempts.max(1);
	for attempt in 1..=attempts {
		match operation().await {
			Ok(value) => return Ok(value),
			Err(err) => {
				if !err.is_retryable() || attempt == attempts {
					return Err(NeoError::provider(context, err));
				}
				let retry_delay = err.retry_after().unwrap_or(delay);

				tracing::warn!(
					attempt = attempt,
					max_attempts = attempts,
					context = %context,
					error = %err,
					retry_delay = ?retry_delay,
					"sdk operation failed; retrying"
				);
				tokio::time::sleep(retry_delay).await;
			},
		}
	}

	unreachable!("the retry loop executes at least once")
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::sync::atomic::{AtomicUsize, Ordering};
	use std::sync::Arc;

	#[tokio::test]
	async fn returns_first_success_without_extra_attempts() {
		let calls = Arc::new(AtomicUsize::new(0));
		let calls_clone = calls.clone();
		let result: Result<u32, NeoError> =
			retry_network("unit", 3, Duration::from_millis(1), move || {
				let calls = calls_clone.clone();
				async move {
					calls.fetch_add(1, Ordering::SeqCst);
					Ok::<u32, ProviderError>(7)
				}
			})
			.await;
		assert_eq!(result.unwrap(), 7);
		assert_eq!(calls.load(Ordering::SeqCst), 1);
	}

	#[tokio::test]
	async fn maps_final_failure_with_context() {
		let calls = Arc::new(AtomicUsize::new(0));
		let calls_clone = calls.clone();
		let result: Result<(), NeoError> =
			retry_network("fetch block", 2, Duration::from_millis(1), move || {
				let calls = calls_clone.clone();
				async move {
					calls.fetch_add(1, Ordering::SeqCst);
					Err::<(), _>(ProviderError::JsonRpcError(crate::neo_clients::JsonRpcError {
						code: 429,
						message: "too many requests".to_string(),
						data: None,
					}))
				}
			})
			.await;
		let err = result.unwrap_err();
		assert_eq!(calls.load(Ordering::SeqCst), 2);
		assert!(err.is_retryable());
		assert!(err.message().contains("fetch block"));
		assert!(err.message().contains("too many requests"));
	}

	#[tokio::test]
	async fn stops_after_a_deterministic_provider_error() {
		let calls = Arc::new(AtomicUsize::new(0));
		let calls_clone = calls.clone();
		let result: Result<(), NeoError> =
			retry_network("validate address", 3, Duration::from_millis(1), move || {
				let calls = calls_clone.clone();
				async move {
					calls.fetch_add(1, Ordering::SeqCst);
					Err::<(), _>(ProviderError::InvalidAddress)
				}
			})
			.await;

		assert_eq!(calls.load(Ordering::SeqCst), 1);
		assert!(!result.unwrap_err().is_retryable());
	}

	#[tokio::test]
	async fn honors_provider_retry_after_hint() {
		let calls = Arc::new(AtomicUsize::new(0));
		let calls_clone = calls.clone();

		let result = tokio::time::timeout(
			Duration::from_millis(100),
			retry_network("rate limited", 2, Duration::from_millis(1), move || {
				let calls = calls_clone.clone();
				async move {
					if calls.fetch_add(1, Ordering::SeqCst) == 0 {
						Err(ProviderError::JsonRpcError(crate::neo_clients::JsonRpcError {
							code: 429,
							message: "too many requests".to_string(),
							data: Some(serde_json::json!({
								"rate": { "backoff_seconds": 2 }
							})),
						}))
					} else {
						Ok(7_u32)
					}
				}
			}),
		)
		.await;

		assert!(result.is_err(), "the provider's two-second backoff must be honored");
		assert_eq!(calls.load(Ordering::SeqCst), 1);
	}
}
