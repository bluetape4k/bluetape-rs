use tracing_subscriber::EnvFilter;
use tracing_subscriber::filter::{FromEnvError, ParseError};
use tracing_subscriber::fmt::format::FmtSpan;

use crate::capture::CapturedLogs;

/// Builds a text subscriber using `RUST_LOG` when present.
pub fn text_subscriber() -> Result<impl tracing::Subscriber + Send + Sync, FromEnvError> {
    let filter = EnvFilter::builder().with_regex(false).from_env()?;
    Ok(tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_span_events(FmtSpan::CLOSE)
        .finish())
}

/// Builds a text subscriber using an explicit filter directive.
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
pub fn with_default<S, F, R>(subscriber: S, f: F) -> R
where
    S: tracing::Subscriber + Send + Sync + 'static,
    F: FnOnce() -> R,
{
    tracing::subscriber::with_default(subscriber, f)
}
