//! Timeout, cancellation, and shutdown coordination helpers.

use std::error::Error;
use std::fmt;
use std::future::Future;
use std::time::Duration;

use tokio::sync::watch;
use tokio::time::Instant;

/// Error returned by timeout, deadline, cancellation, and shutdown helpers.
#[derive(Debug, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum AsyncControlError {
    /// The wrapped operation did not complete before its timeout or deadline.
    TimedOut,
    /// The caller-owned cancellation or shutdown signal completed first.
    Cancelled,
}

impl fmt::Display for AsyncControlError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimedOut => formatter.write_str("operation timed out"),
            Self::Cancelled => formatter.write_str("operation was cancelled"),
        }
    }
}

impl Error for AsyncControlError {}

/// Handle used to request cancellation for one or more [`CancellationToken`]s.
///
/// Cloning the source creates another owner that can request cancellation.
/// Dropping every source without calling [`CancellationSource::cancel`] also
/// wakes tokens; receivers treat a source-less channel as cancelled because no
/// owner remains able to make a later positive decision.
#[derive(Debug, Clone)]
pub struct CancellationSource {
    sender: watch::Sender<bool>,
}

impl CancellationSource {
    /// Creates a new cancellation source and its first token.
    ///
    /// # Examples
    ///
    /// ```
    /// use bluetape_rs_async::CancellationSource;
    ///
    /// let (source, token) = CancellationSource::new();
    /// assert!(!token.is_cancelled());
    /// source.cancel();
    /// assert!(token.is_cancelled());
    /// ```
    #[must_use]
    pub fn new() -> (Self, CancellationToken) {
        let (sender, receiver) = watch::channel(false);
        (Self { sender }, CancellationToken { receiver })
    }

    /// Creates another token linked to this source.
    ///
    /// Tokens are independent receivers over the same cancellation state. A
    /// late token observes an already-cancelled source immediately.
    #[must_use]
    pub fn token(&self) -> CancellationToken {
        CancellationToken {
            receiver: self.sender.subscribe(),
        }
    }

    /// Requests cancellation.
    ///
    /// This method is idempotent. It is not an error if all tokens have already
    /// been dropped.
    pub fn cancel(&self) {
        let _ = self.sender.send(true);
    }

    /// Returns `true` when cancellation has been requested.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        *self.sender.borrow()
    }
}

/// Receiver-side cancellation token.
///
/// A token is a listener, not an owner. Dropping a token never cancels sibling
/// tokens; only [`CancellationSource::cancel`] or dropping all sources does.
#[derive(Debug, Clone)]
pub struct CancellationToken {
    receiver: watch::Receiver<bool>,
}

impl CancellationToken {
    /// Returns `true` when cancellation has been requested.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        *self.receiver.borrow()
    }

    /// Waits until cancellation is requested or the source is dropped.
    ///
    /// Dropping the source is treated as cancellation because no owner remains
    /// able to make a later positive shutdown decision. In that source-drop
    /// case [`CancellationToken::is_cancelled`] still reflects the last observed
    /// boolean state, which can remain `false`.
    pub async fn cancelled(&mut self) {
        if *self.receiver.borrow() {
            return;
        }

        loop {
            if self.receiver.changed().await.is_err() {
                return;
            }
            if *self.receiver.borrow_and_update() {
                return;
            }
        }
    }
}

/// Trigger side of a shutdown signal pair.
///
/// This is a domain-named wrapper around [`CancellationSource`] for graceful
/// shutdown flows.
#[derive(Debug, Clone)]
pub struct ShutdownTrigger {
    source: CancellationSource,
}

impl ShutdownTrigger {
    /// Requests shutdown for all linked [`ShutdownSignal`] listeners.
    pub fn shutdown(&self) {
        self.source.cancel();
    }

    /// Creates another listener linked to this trigger.
    #[must_use]
    pub fn signal(&self) -> ShutdownSignal {
        ShutdownSignal {
            token: self.source.token(),
        }
    }

    /// Returns `true` when shutdown has been requested.
    #[must_use]
    pub fn is_shutdown_requested(&self) -> bool {
        self.source.is_cancelled()
    }
}

/// Listener side of a shutdown signal pair.
///
/// Dropping all triggers wakes listeners the same way an explicit shutdown
/// request does, because no owner remains able to keep the service running.
#[derive(Debug, Clone)]
pub struct ShutdownSignal {
    token: CancellationToken,
}

impl ShutdownSignal {
    /// Returns `true` when shutdown has been requested.
    #[must_use]
    pub fn is_shutdown_requested(&self) -> bool {
        self.token.is_cancelled()
    }

    /// Waits until shutdown is requested or the trigger is dropped.
    pub async fn wait(&mut self) {
        self.token.cancelled().await;
    }
}

/// Creates a shutdown trigger and its first listener.
#[must_use]
pub fn shutdown_signal() -> (ShutdownTrigger, ShutdownSignal) {
    let (source, token) = CancellationSource::new();
    (ShutdownTrigger { source }, ShutdownSignal { token })
}

/// Runs a future with a Tokio timeout.
///
/// Tokio cancels the wrapped future by dropping it when the timeout elapses.
/// Dropping this wrapper future still propagates caller cancellation normally.
///
/// # Errors
///
/// Returns [`AsyncControlError::TimedOut`] when the future does not complete
/// before `duration`.
pub async fn with_timeout<F, T>(duration: Duration, future: F) -> Result<T, AsyncControlError>
where
    F: Future<Output = T>,
{
    tokio::time::timeout(duration, future)
        .await
        .map_err(|_| AsyncControlError::TimedOut)
}

/// Runs a future until a Tokio deadline.
///
/// Tokio checks the deadline before polling the future. A future that never
/// yields can still run past the deadline before Tokio observes the timeout.
///
/// # Errors
///
/// Returns [`AsyncControlError::TimedOut`] when the future does not complete
/// before `deadline`.
pub async fn with_deadline<F, T>(deadline: Instant, future: F) -> Result<T, AsyncControlError>
where
    F: Future<Output = T>,
{
    tokio::time::timeout_at(deadline, future)
        .await
        .map_err(|_| AsyncControlError::TimedOut)
}

/// Runs a future until either it completes or cancellation is requested.
///
/// Dropping this wrapper future does not convert caller cancellation into
/// [`AsyncControlError::Cancelled`]; the inner future is dropped normally.
/// Dropping every source for `token` is treated the same as cancellation.
///
/// # Errors
///
/// Returns [`AsyncControlError::Cancelled`] when `token` completes first.
pub async fn run_until_cancelled<F, T>(
    mut token: CancellationToken,
    future: F,
) -> Result<T, AsyncControlError>
where
    F: Future<Output = T>,
{
    tokio::select! {
        biased;
        _ = token.cancelled() => Err(AsyncControlError::Cancelled),
        value = future => Ok(value),
    }
}

/// Runs a future until it completes, times out, or cancellation is requested.
///
/// Cancellation wins when the token and timeout are both ready.
/// Dropping every source for `token` is treated the same as cancellation.
///
/// # Errors
///
/// Returns [`AsyncControlError::Cancelled`] when `token` completes first, or
/// [`AsyncControlError::TimedOut`] when `duration` elapses first.
pub async fn with_timeout_or_cancel<F, T>(
    duration: Duration,
    mut token: CancellationToken,
    future: F,
) -> Result<T, AsyncControlError>
where
    F: Future<Output = T>,
{
    tokio::select! {
        biased;
        _ = token.cancelled() => Err(AsyncControlError::Cancelled),
        result = tokio::time::timeout(duration, future) => {
            result.map_err(|_| AsyncControlError::TimedOut)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::future::pending;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use tokio::sync::Notify;
    use tokio::time::{Duration, Instant, sleep};

    use super::*;

    struct DropCounter {
        counter: Arc<AtomicUsize>,
    }

    impl Drop for DropCounter {
        fn drop(&mut self) {
            self.counter.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn async_control_error_formats_public_error_messages() {
        assert_eq!(
            AsyncControlError::TimedOut.to_string(),
            "operation timed out"
        );
        assert_eq!(
            AsyncControlError::Cancelled.to_string(),
            "operation was cancelled"
        );
        assert!(AsyncControlError::TimedOut.source().is_none());
        assert!(AsyncControlError::Cancelled.source().is_none());
    }

    #[tokio::test]
    async fn with_timeout_returns_value_before_deadline() {
        let actual = with_timeout(Duration::from_secs(1), async { 7 }).await;

        assert_eq!(actual, Ok(7));
    }

    #[tokio::test(start_paused = true)]
    async fn with_timeout_reports_elapsed_operation() {
        let actual = with_timeout(Duration::from_millis(10), async {
            sleep(Duration::from_secs(1)).await;
            7
        })
        .await;

        assert_eq!(actual, Err(AsyncControlError::TimedOut));
    }

    #[tokio::test(start_paused = true)]
    async fn with_deadline_reports_elapsed_operation() {
        let deadline = Instant::now() + Duration::from_millis(10);
        let actual = with_deadline(deadline, async {
            sleep(Duration::from_secs(1)).await;
            7
        })
        .await;

        assert_eq!(actual, Err(AsyncControlError::TimedOut));
    }

    #[tokio::test(start_paused = true)]
    async fn with_timeout_or_cancel_reports_timeout_when_token_is_idle() {
        let (_source, token) = CancellationSource::new();

        let actual = with_timeout_or_cancel(Duration::from_millis(10), token, async {
            sleep(Duration::from_secs(1)).await;
            7
        })
        .await;

        assert_eq!(actual, Err(AsyncControlError::TimedOut));
    }

    #[tokio::test]
    async fn run_until_cancelled_returns_value_before_cancellation() {
        let (_source, token) = CancellationSource::new();

        let actual = run_until_cancelled(token, async { 7 }).await;

        assert_eq!(actual, Ok(7));
    }

    #[tokio::test]
    async fn cancellation_token_completes_when_all_sources_are_dropped() {
        let (source, mut token) = CancellationSource::new();

        drop(source);
        token.cancelled().await;

        assert!(!token.is_cancelled());
    }

    #[tokio::test]
    async fn run_until_cancelled_reports_cancelled_when_source_is_dropped() {
        let (source, token) = CancellationSource::new();

        drop(source);
        let actual = run_until_cancelled(token, pending::<()>()).await;

        assert_eq!(actual, Err(AsyncControlError::Cancelled));
    }

    #[tokio::test]
    async fn run_until_cancelled_reports_cancellation_and_drops_future() {
        let (source, token) = CancellationSource::new();
        let dropped = Arc::new(AtomicUsize::new(0));
        let started = Arc::new(Notify::new());

        let task = tokio::spawn({
            let dropped = Arc::clone(&dropped);
            let started = Arc::clone(&started);
            async move {
                run_until_cancelled(token, async move {
                    let _guard = DropCounter { counter: dropped };
                    started.notify_one();
                    pending::<()>().await;
                    7
                })
                .await
            }
        });

        started.notified().await;
        source.cancel();
        let actual = task.await.unwrap();

        assert_eq!(actual, Err(AsyncControlError::Cancelled));
        assert_eq!(dropped.load(Ordering::SeqCst), 1);
    }

    #[tokio::test(start_paused = true)]
    async fn with_timeout_or_cancel_prefers_cancellation() {
        let (source, token) = CancellationSource::new();
        source.cancel();

        let actual = with_timeout_or_cancel(Duration::from_millis(10), token, async {
            sleep(Duration::from_secs(1)).await;
            7
        })
        .await;

        assert_eq!(actual, Err(AsyncControlError::Cancelled));
    }

    #[tokio::test]
    async fn with_timeout_or_cancel_reports_cancelled_when_source_is_dropped() {
        let (source, token) = CancellationSource::new();

        drop(source);
        let actual = with_timeout_or_cancel(Duration::from_secs(1), token, pending::<()>()).await;

        assert_eq!(actual, Err(AsyncControlError::Cancelled));
    }

    #[tokio::test]
    async fn shutdown_signal_notifies_all_listeners() {
        let (trigger, mut signal) = shutdown_signal();
        let mut second = trigger.signal();

        let first_task = tokio::spawn(async move {
            signal.wait().await;
            signal.is_shutdown_requested()
        });
        let second_task = tokio::spawn(async move {
            second.wait().await;
            second.is_shutdown_requested()
        });

        trigger.shutdown();

        assert!(first_task.await.unwrap());
        assert!(second_task.await.unwrap());
        assert!(trigger.is_shutdown_requested());
    }

    #[tokio::test]
    async fn shutdown_signal_waits_until_trigger_is_dropped() {
        let (trigger, mut signal) = shutdown_signal();

        drop(trigger);
        signal.wait().await;

        assert!(!signal.is_shutdown_requested());
    }
}
