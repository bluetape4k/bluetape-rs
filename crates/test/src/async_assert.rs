use std::future::Future;
use std::time::Duration;

use tokio::time::{Instant, sleep_until, timeout_at};

const MIN_POLL_INTERVAL: Duration = Duration::from_millis(1);

/// Error returned by asynchronous assertion helpers.
///
/// The error keeps the caller-provided assertion error when one is available,
/// which lets tests report the last observed failure instead of only a generic
/// timeout.
///
/// # Examples
///
/// ```
/// use bluetape_rs_test::AsyncAssertError;
///
/// let error: AsyncAssertError<&'static str> = AsyncAssertError::Timeout {
///     last_error: Some("not ready"),
/// };
/// assert!(error.to_string().contains("not ready"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AsyncAssertError<E> {
    /// The condition never succeeded before the timeout.
    Timeout {
        /// Last observed assertion error before the timeout, when one was produced.
        last_error: Option<E>,
    },
    /// The condition failed after initially succeeding.
    BecameUnstable {
        /// Assertion error that ended the stable period.
        error: E,
    },
}

impl<E> std::fmt::Display for AsyncAssertError<E>
where
    E: std::fmt::Display,
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Timeout {
                last_error: Some(error),
            } => write!(
                formatter,
                "condition did not succeed before timeout: {error}"
            ),
            Self::Timeout { last_error: None } => {
                formatter.write_str("condition did not succeed before timeout")
            }
            Self::BecameUnstable { error } => {
                write!(formatter, "condition became unstable: {error}")
            }
        }
    }
}

impl<E> std::error::Error for AsyncAssertError<E>
where
    E: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Timeout {
                last_error: Some(error),
            } => Some(error),
            Self::BecameUnstable { error } => Some(error),
            Self::Timeout { last_error: None } => None,
        }
    }
}
/// Retries `condition` until it succeeds or `timeout` expires.
///
/// The condition is evaluated immediately, then retried at `interval` until it
/// returns `Ok(())` or the timeout deadline is reached.
///
/// # Examples
///
/// ```
/// use std::sync::Arc;
/// use std::sync::atomic::{AtomicUsize, Ordering};
/// use std::time::Duration;
/// use bluetape_rs_test::eventually;
///
/// # async fn demo() -> Result<(), bluetape_rs_test::AsyncAssertError<&'static str>> {
/// let attempts = Arc::new(AtomicUsize::new(0));
/// let observed = Arc::clone(&attempts);
///
/// eventually(Duration::from_secs(1), Duration::from_millis(10), move || {
///     let observed = Arc::clone(&observed);
///     async move {
///         if observed.fetch_add(1, Ordering::SeqCst) >= 1 {
///             Ok(())
///         } else {
///             Err("not ready")
///         }
///     }
/// }).await?;
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// Returns [`AsyncAssertError::Timeout`] when the condition does not return
/// `Ok(())` before the timeout expires. The error includes the last observed
/// assertion failure when one was produced.
pub async fn eventually<F, Fut, E>(
    timeout: Duration,
    interval: Duration,
    mut condition: F,
) -> Result<(), AsyncAssertError<E>>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<(), E>>,
{
    let deadline = Instant::now() + timeout;
    let interval = normalize_poll_interval(interval);
    let mut last_error: Option<E> = None;

    loop {
        match timeout_at(deadline, condition()).await {
            Err(_) => return Err(AsyncAssertError::Timeout { last_error }),
            Ok(Ok(())) => return Ok(()),
            Ok(Err(error)) => last_error = Some(error),
        }

        if Instant::now() >= deadline {
            return Err(AsyncAssertError::Timeout { last_error });
        }
        sleep_until(next_poll_deadline(interval, deadline)).await;
    }
}
/// Requires `condition` to stay successful for `duration`.
///
/// The condition is evaluated repeatedly until `duration` has elapsed. The
/// helper succeeds only when every evaluation returns `Ok(())`.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
/// use bluetape_rs_test::consistently;
///
/// # async fn demo() -> Result<(), bluetape_rs_test::AsyncAssertError<&'static str>> {
/// consistently(Duration::from_millis(20), Duration::from_millis(5), || async {
///     Ok(())
/// }).await?;
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// Returns [`AsyncAssertError::BecameUnstable`] with the first assertion error
/// observed before the stable duration elapses.
pub async fn consistently<F, Fut, E>(
    duration: Duration,
    interval: Duration,
    mut condition: F,
) -> Result<(), AsyncAssertError<E>>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<(), E>>,
{
    let deadline = Instant::now() + duration;
    let interval = normalize_poll_interval(interval);

    loop {
        match timeout_at(deadline, condition()).await {
            Err(_) => return Ok(()),
            Ok(Ok(())) => {}
            Ok(Err(error)) => return Err(AsyncAssertError::BecameUnstable { error }),
        }

        if Instant::now() >= deadline {
            return Ok(());
        }
        sleep_until(next_poll_deadline(interval, deadline)).await;
    }
}

fn normalize_poll_interval(interval: Duration) -> Duration {
    interval.max(MIN_POLL_INTERVAL)
}

fn next_poll_deadline(interval: Duration, deadline: Instant) -> Instant {
    let next = Instant::now() + interval;
    if next > deadline { deadline } else { next }
}
