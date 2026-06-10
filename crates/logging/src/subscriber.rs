use tracing_subscriber::EnvFilter;
use tracing_subscriber::filter::{FromEnvError, ParseError};
use tracing_subscriber::fmt::format::FmtSpan;

use crate::capture::CapturedLogs;

/// Builds a text subscriber using `RUST_LOG` when present.
///
/// Regex matching is disabled for predictable backend-service filtering.
///
/// # Examples
///
/// ```no_run
/// use bluetape_rs_logging::text_subscriber;
///
/// let subscriber = text_subscriber()?;
/// tracing::subscriber::with_default(subscriber, || {
///     tracing::info!("service started");
/// });
/// # Ok::<(), tracing_subscriber::filter::FromEnvError>(())
/// ```
///
/// # Errors
///
/// Returns [`FromEnvError`] when `RUST_LOG` contains an invalid tracing filter
/// directive.
pub fn text_subscriber() -> Result<impl tracing::Subscriber + Send + Sync, FromEnvError> {
    let filter = EnvFilter::builder().with_regex(false).from_env()?;
    Ok(tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_span_events(FmtSpan::CLOSE)
        .finish())
}

/// Builds a text subscriber using an explicit filter directive.
///
/// # Examples
///
/// ```
/// use bluetape_rs_logging::{text_subscriber_with_filter, with_default};
///
/// let subscriber = text_subscriber_with_filter("info,bluetape_rs=debug")?;
/// with_default(subscriber, || tracing::debug!(target: "bluetape_rs", "enabled"));
/// # Ok::<(), tracing_subscriber::filter::ParseError>(())
/// ```
///
/// # Errors
///
/// Returns [`ParseError`] when `filter` is not a valid tracing filter directive.
pub fn text_subscriber_with_filter(
    filter: impl AsRef<str>,
) -> Result<impl tracing::Subscriber + Send + Sync, ParseError> {
    let filter = EnvFilter::builder()
        .with_regex(false)
        .parse(filter.as_ref())?;
    Ok(tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_span_events(FmtSpan::CLOSE)
        .finish())
}

/// Builds a scoped text subscriber that writes to `captured`.
///
/// # Examples
///
/// ```
/// use bluetape_rs_logging::{CapturedLogs, capture_subscriber, with_default};
///
/// let captured = CapturedLogs::new();
/// let subscriber = capture_subscriber(captured.clone(), "info")?;
///
/// with_default(subscriber, || tracing::info!("captured"));
/// assert!(captured.to_lossy_string().contains("captured"));
/// # Ok::<(), tracing_subscriber::filter::ParseError>(())
/// ```
///
/// # Errors
///
/// Returns [`ParseError`] when `filter` is not a valid tracing filter directive.
pub fn capture_subscriber(
    captured: CapturedLogs,
    filter: impl AsRef<str>,
) -> Result<impl tracing::Subscriber + Send + Sync, ParseError> {
    let filter = EnvFilter::builder()
        .with_regex(false)
        .parse(filter.as_ref())?;
    Ok(tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_span_events(FmtSpan::CLOSE)
        .with_writer(captured)
        .finish())
}

/// Runs `f` with the provided subscriber as a scoped default.
///
/// # Examples
///
/// ```
/// use bluetape_rs_logging::{CapturedLogs, capture_subscriber, with_default};
///
/// let captured = CapturedLogs::new();
/// let subscriber = capture_subscriber(captured.clone(), "info")?;
///
/// let result = with_default(subscriber, || {
///     tracing::info!("inside scope");
///     42
/// });
///
/// assert_eq!(result, 42);
/// assert!(captured.to_lossy_string().contains("inside scope"));
/// # Ok::<(), tracing_subscriber::filter::ParseError>(())
/// ```
pub fn with_default<S, F, R>(subscriber: S, f: F) -> R
where
    S: tracing::Subscriber + Send + Sync + 'static,
    F: FnOnce() -> R,
{
    tracing::subscriber::with_default(subscriber, f)
}
