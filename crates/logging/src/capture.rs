use std::io::{self, Write};
use std::sync::{Arc, Mutex};

/// Shared in-memory log capture for tests.
///
/// This writer is intentionally unbounded and intended for scoped, bounded test
/// assertions only. Do not expose it to attacker-controlled log volume.
///
/// # Examples
///
/// ```
/// use bluetape_rs_logging::{CapturedLogs, capture_subscriber, with_default};
///
/// let captured = CapturedLogs::new();
/// let subscriber = capture_subscriber(captured.clone(), "info")?;
///
/// with_default(subscriber, || {
///     tracing::info!("captured message");
/// });
///
/// assert!(captured.to_lossy_string().contains("captured message"));
/// # Ok::<(), tracing_subscriber::filter::ParseError>(())
/// ```
#[derive(Debug, Clone, Default)]
pub struct CapturedLogs {
    inner: Arc<Mutex<Vec<u8>>>,
}

impl CapturedLogs {
    /// Creates an empty log capture buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use bluetape_rs_logging::CapturedLogs;
    ///
    /// let captured = CapturedLogs::new();
    /// assert!(captured.to_lossy_string().is_empty());
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns captured logs as UTF-8 text, replacing invalid bytes if present.
    ///
    /// # Examples
    ///
    /// ```
    /// use bluetape_rs_logging::{CapturedLogs, capture_subscriber, with_default};
    ///
    /// let captured = CapturedLogs::new();
    /// let subscriber = capture_subscriber(captured.clone(), "debug")?;
    /// with_default(subscriber, || tracing::debug!("debug detail"));
    ///
    /// assert!(captured.to_lossy_string().contains("debug detail"));
    /// # Ok::<(), tracing_subscriber::filter::ParseError>(())
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if another thread panicked while holding the internal capture
    /// buffer mutex.
    pub fn to_lossy_string(&self) -> String {
        let bytes = self.inner.lock().expect("captured log mutex poisoned");
        String::from_utf8_lossy(&bytes).into_owned()
    }

    /// Clears the captured log buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use bluetape_rs_logging::{CapturedLogs, capture_subscriber, with_default};
    ///
    /// let captured = CapturedLogs::new();
    /// let subscriber = capture_subscriber(captured.clone(), "info")?;
    /// with_default(subscriber, || tracing::info!("first"));
    ///
    /// captured.clear();
    /// assert!(captured.to_lossy_string().is_empty());
    /// # Ok::<(), tracing_subscriber::filter::ParseError>(())
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if another thread panicked while holding the internal capture
    /// buffer mutex.
    pub fn clear(&self) {
        self.inner
            .lock()
            .expect("captured log mutex poisoned")
            .clear();
    }
}

/// Writer used by [`CapturedLogs`].
///
/// Values are created by the [`tracing_subscriber::fmt::MakeWriter`]
/// implementation on [`CapturedLogs`].
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
