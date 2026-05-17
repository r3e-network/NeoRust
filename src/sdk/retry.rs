//! Lightweight retry helper used by the high-level [`crate::sdk::Neo`] API.
//!
//! This intentionally does not duplicate the lower-level
//! [`crate::neo_clients::RetryClient`] / [`crate::neo_clients::CircuitBreaker`]
//! infrastructure — it only wraps an async closure with a bounded retry budget
//! and uniform [`NeoError::network`] mapping so the call sites in
//! `sdk/mod.rs` stay readable.

use std::future::Future;
use std::time::Duration;

use crate::neo_error::unified::NeoError;

/// Default base delay between retries used by the high-level SDK.
pub(crate) const DEFAULT_RETRY_DELAY: Duration = Duration::from_millis(250);

/// Retry an asynchronous, fallible operation up to `attempts` total times.
///
/// `attempts` is the total number of attempts (so `attempts = 1` means *no*
/// retry — the operation runs exactly once). Failures are logged via
/// `tracing` at `warn` level on every non-terminal attempt, and the final
/// error is mapped to [`NeoError::Network`] with `context` woven into the
/// message so users can see *which* call failed without reading a stack
/// trace.
pub(crate) async fn retry_network<T, E, F, Fut>(
	context: &str,
	attempts: u32,
	delay: Duration,
	mut operation: F,
) -> Result<T, NeoError>
where
	F: FnMut() -> Fut,
	Fut: Future<Output = Result<T, E>>,
	E: std::fmt::Display,
{
	let attempts = attempts.max(1);
	let mut last_err: Option<E> = None;
	for attempt in 1..=attempts {
		match operation().await {
			Ok(value) => return Ok(value),
			Err(err) => {
				if attempt < attempts {
					tracing::warn!(
						attempt = attempt,
						max_attempts = attempts,
						context = %context,
						error = %err,
						"sdk operation failed; retrying"
					);
					tokio::time::sleep(delay).await;
				}
				last_err = Some(err);
			},
		}
	}

	let err = last_err.expect("retry loop ran at least once");
	Err(NeoError::network(context, err))
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
					Ok::<u32, &'static str>(7)
				}
			})
			.await;
		assert_eq!(result.unwrap(), 7);
		assert_eq!(calls.load(Ordering::SeqCst), 1);
	}

	#[tokio::test]
	async fn maps_final_failure_to_network_error_with_context() {
		let calls = Arc::new(AtomicUsize::new(0));
		let calls_clone = calls.clone();
		let result: Result<(), NeoError> =
			retry_network("fetch block", 2, Duration::from_millis(1), move || {
				let calls = calls_clone.clone();
				async move {
					calls.fetch_add(1, Ordering::SeqCst);
					Err::<(), _>("upstream 503")
				}
			})
			.await;
		let err = result.unwrap_err();
		assert_eq!(calls.load(Ordering::SeqCst), 2);
		assert!(err.is_retryable());
		assert!(err.message().contains("fetch block"));
		assert!(err.message().contains("upstream 503"));
	}
}
