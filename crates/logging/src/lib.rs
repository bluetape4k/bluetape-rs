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

mod capture;
mod correlation;
mod subscriber;

pub use capture::{CapturedLogWriter, CapturedLogs};
pub use correlation::{
    CORRELATION_ID_FIELD, CorrelationId, CorrelationIdError, MAX_CORRELATION_ID_LEN,
    REQUEST_ID_FIELD, TASK_ID_FIELD,
};
pub use subscriber::{
    capture_subscriber, text_subscriber, text_subscriber_with_filter, with_default,
};

#[cfg(test)]
mod tests;
