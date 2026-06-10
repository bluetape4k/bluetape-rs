//! Tracing conventions and subscriber builders.
//!
//! This crate never installs a process-global subscriber. Applications can use
//! the returned subscriber with `tracing::subscriber::set_global_default` or
//! `tracing::subscriber::with_default`.
//!
//! ```
//! use bluetape_rs_logging::{CorrelationId, CORRELATION_ID_FIELD};
//!
//! let id = CorrelationId::new("request-1").expect("correlation id");
//! assert_eq!(id.as_str(), "request-1");
//! assert_eq!(CORRELATION_ID_FIELD, "correlation.id");
//! ```

use std::fmt::{self, Display};
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::filter::{FromEnvError, ParseError};
use tracing_subscriber::fmt::format::FmtSpan;

/// Conventional trace field for a request identifier.
pub const REQUEST_ID_FIELD: &str = "request.id";

/// Conventional trace field for a task identifier.
pub const TASK_ID_FIELD: &str = "task.id";

/// Conventional trace field for a correlation identifier.
pub const CORRELATION_ID_FIELD: &str = "correlation.id";

/// Maximum accepted correlation identifier length in bytes.
pub const MAX_CORRELATION_ID_LEN: usize = 256;

/// A non-empty, single-line correlation identifier.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CorrelationId(String);

/// Reason a [`CorrelationId`] value was rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorrelationIdError {
    /// The identifier is empty or contains only whitespace.
    Blank,
    /// The identifier is longer than [`MAX_CORRELATION_ID_LEN`] bytes.
    TooLong {
        /// Actual identifier length in bytes.
        len: usize,
        /// Maximum accepted identifier length in bytes.
        max: usize,
    },
    /// The identifier contains a control, separator, or bidirectional control character.
    UnsafeCharacter {
        /// Rejected character.
        ch: char,
    },
}

impl Display for CorrelationIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blank => f.write_str("correlation id must contain visible text"),
            Self::TooLong { len, max } => write!(
                f,
                "correlation id is {len} bytes, exceeding the {max} byte limit"
            ),
            Self::UnsafeCharacter { ch } => {
                write!(
                    f,
                    "correlation id contains unsafe character U+{:04X}",
                    *ch as u32
                )
            }
        }
    }
}

impl std::error::Error for CorrelationIdError {}

impl CorrelationId {
    /// Creates a correlation identifier when `value` contains visible text, no
    /// control characters, and at most [`MAX_CORRELATION_ID_LEN`] bytes.
    pub fn new(value: impl Into<String>) -> Result<Self, CorrelationIdError> {
        let value = value.into();
        let len = value.len();
        if len > MAX_CORRELATION_ID_LEN {
            return Err(CorrelationIdError::TooLong {
                len,
                max: MAX_CORRELATION_ID_LEN,
            });
        }
        if !value.chars().any(|ch| !ch.is_whitespace()) {
            return Err(CorrelationIdError::Blank);
        }
        if let Some(ch) = value.chars().find(|ch| !is_safe_correlation_char(*ch)) {
            return Err(CorrelationIdError::UnsafeCharacter { ch });
        }
        Ok(Self(value))
    }

    /// Returns the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn is_safe_correlation_char(ch: char) -> bool {
    !ch.is_control()
        && !matches!(
            ch,
            '\u{061c}'
                | '\u{2028}'
                | '\u{2029}'
                | '\u{200e}'
                | '\u{200f}'
                | '\u{202a}'..='\u{202e}'
                | '\u{2066}'..='\u{2069}'
        )
}

impl Display for CorrelationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Shared in-memory log capture for tests.
///
/// This writer is intentionally unbounded and intended for scoped, bounded test
/// assertions only. Do not expose it to attacker-controlled log volume.
#[derive(Debug, Clone, Default)]
pub struct CapturedLogs {
    inner: Arc<Mutex<Vec<u8>>>,
}

impl CapturedLogs {
    /// Creates an empty log capture buffer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns captured logs as UTF-8 text, replacing invalid bytes if present.
    pub fn to_lossy_string(&self) -> String {
        let bytes = self.inner.lock().expect("captured log mutex poisoned");
        String::from_utf8_lossy(&bytes).into_owned()
    }

    /// Clears the captured log buffer.
    pub fn clear(&self) {
        self.inner
            .lock()
            .expect("captured log mutex poisoned")
            .clear();
    }
}

/// Writer used by [`CapturedLogs`].
pub struct CapturedLogWriter {
    inner: Arc<Mutex<Vec<u8>>>,
}

impl Write for CapturedLogWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner
            .lock()
            .expect("captured log mutex poisoned")
            .extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<'writer> tracing_subscriber::fmt::MakeWriter<'writer> for CapturedLogs {
    type Writer = CapturedLogWriter;

    fn make_writer(&'writer self) -> Self::Writer {
        CapturedLogWriter {
            inner: Arc::clone(&self.inner),
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[test]
    fn accepts_visible_correlation_ids() {
        let id = CorrelationId::new(" request-1 ").expect("visible id");
        assert_eq!(id.as_str(), " request-1 ");
        assert_eq!(id.to_string(), " request-1 ");
        assert_eq!(CorrelationId::new(" \t"), Err(CorrelationIdError::Blank));
    }

    #[test]
    fn rejects_control_or_oversized_correlation_ids_with_typed_errors() {
        assert_eq!(
            CorrelationId::new("request\nforged"),
            Err(CorrelationIdError::UnsafeCharacter { ch: '\n' })
        );
        assert_eq!(
            CorrelationId::new("request\rforged"),
            Err(CorrelationIdError::UnsafeCharacter { ch: '\r' })
        );
        assert_eq!(
            CorrelationId::new("request\tforged"),
            Err(CorrelationIdError::UnsafeCharacter { ch: '\t' })
        );
        assert_eq!(
            CorrelationId::new("request\u{2028}forged"),
            Err(CorrelationIdError::UnsafeCharacter { ch: '\u{2028}' })
        );
        assert_eq!(
            CorrelationId::new("request\u{2029}forged"),
            Err(CorrelationIdError::UnsafeCharacter { ch: '\u{2029}' })
        );
        assert_eq!(
            CorrelationId::new("request\u{202e}forged"),
            Err(CorrelationIdError::UnsafeCharacter { ch: '\u{202e}' })
        );
        assert_eq!(
            CorrelationId::new("request\u{061c}forged"),
            Err(CorrelationIdError::UnsafeCharacter { ch: '\u{061c}' })
        );
        assert_eq!(
            CorrelationId::new("x".repeat(MAX_CORRELATION_ID_LEN + 1)),
            Err(CorrelationIdError::TooLong {
                len: MAX_CORRELATION_ID_LEN + 1,
                max: MAX_CORRELATION_ID_LEN,
            })
        );
        assert!(CorrelationId::new("x".repeat(MAX_CORRELATION_ID_LEN)).is_ok());
    }

    #[test]
    fn exposes_field_names() {
        assert_eq!(REQUEST_ID_FIELD, "request.id");
        assert_eq!(TASK_ID_FIELD, "task.id");
        assert_eq!(CORRELATION_ID_FIELD, "correlation.id");
    }

    #[test]
    fn scoped_subscriber_does_not_require_global_installation() {
        let subscriber = text_subscriber_with_filter("info").expect("valid filter");
        let value = with_default(subscriber, || {
            info!(request.id = "r1", "captured by scoped subscriber");
            42
        });
        assert_eq!(value, 42);
    }

    #[test]
    fn capture_subscriber_records_logs_for_tests() {
        let captured = CapturedLogs::new();
        let subscriber = capture_subscriber(captured.clone(), "info").expect("valid filter");

        with_default(subscriber, || {
            info!(correlation.id = "c1", "captured event");
        });

        let logs = captured.to_lossy_string();
        assert!(logs.contains("captured event"));
        assert!(logs.contains("correlation.id=\"c1\""));
        captured.clear();
        assert!(captured.to_lossy_string().is_empty());
    }

    #[test]
    fn explicit_filter_builders_reject_invalid_directives() {
        let invalid_filter = "crate1::mod1=warn=info";
        assert!(text_subscriber_with_filter(invalid_filter).is_err());
        assert!(capture_subscriber(CapturedLogs::new(), invalid_filter).is_err());
    }

    #[test]
    fn explicit_filter_builders_disable_regex_value_matching() {
        assert!(text_subscriber_with_filter("target[{field=hello.*}]=info").is_ok());
    }
}
