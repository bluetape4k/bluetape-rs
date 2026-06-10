//! Tokio-first helpers for bounded async task execution.
//!
//! The helpers in this crate make task lifecycle policy explicit: callers
//! choose between first-error execution with sibling cancellation and collect-all
//! execution that records every operation result.
//!
//! ```
//! use bluetape_rs_async::try_map_bounded;
//!
//! # async fn demo() -> Result<(), bluetape_rs_async::TaskGroupError<&'static str>> {
//! let values = try_map_bounded([1, 2, 3], 2, |value| async move {
//!     Ok::<_, &'static str>(value * 2)
//! })
//! .await?;
//!
//! assert_eq!(values, vec![2, 4, 6]);
//! # Ok(())
//! # }
//! ```

use std::error::Error;
use std::fmt;
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::watch;
use tokio::task::{JoinError, JoinSet};
use tokio::time::Instant;

/// Default concurrency bound for callers that do not need a custom limit.
pub const DEFAULT_MAX_CONCURRENCY: usize = 16;

/// Maximum accepted concurrency bound.
///
/// This is intentionally conservative. Higher fan-out usually needs explicit
/// queueing, backpressure, or service-specific resource limits.
pub const MAX_CONCURRENCY: usize = 10_000;

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
    /// able to make a later positive shutdown decision.
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

/// Error returned by bounded task helpers.
#[derive(Debug)]
#[non_exhaustive]
pub enum TaskGroupError<E> {
    /// `max_concurrency` must be greater than zero.
    ZeroConcurrency,
    /// `max_concurrency` exceeded [`MAX_CONCURRENCY`].
    ExcessiveConcurrency {
        /// Rejected concurrency bound.
        max_concurrency: usize,
        /// Largest accepted concurrency bound.
        upper_bound: usize,
    },
    /// An operation failed while running in first-error mode.
    TaskFailed {
        /// Zero-based input index.
        index: usize,
        /// Caller-provided failure cause.
        error: E,
    },
    /// A spawned Tokio task failed to join.
    TaskJoinFailed {
        /// Zero-based input index when the failed task reported it.
        index: Option<usize>,
        /// Tokio join error.
        source: JoinError,
    },
}

impl<E> fmt::Display for TaskGroupError<E>
where
    E: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroConcurrency => {
                formatter.write_str("max_concurrency must be greater than zero")
            }
            Self::ExcessiveConcurrency {
                max_concurrency,
                upper_bound,
            } => write!(
                formatter,
                "max_concurrency must be less than or equal to {upper_bound}, got {max_concurrency}"
            ),
            Self::TaskFailed { index, error } => {
                write!(formatter, "task {index} failed: {error}")
            }
            Self::TaskJoinFailed {
                index: Some(index),
                source,
            } => write!(formatter, "task {index} failed to join: {source}"),
            Self::TaskJoinFailed {
                index: None,
                source,
            } => write!(formatter, "task failed to join: {source}"),
        }
    }
}

impl<E> Error for TaskGroupError<E>
where
    E: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::TaskFailed { error, .. } => Some(error),
            Self::TaskJoinFailed { source, .. } => Some(source),
            Self::ZeroConcurrency | Self::ExcessiveConcurrency { .. } => None,
        }
    }
}

/// A successful operation result captured by [`map_bounded_collect`].
#[derive(Debug, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub struct TaskSuccess<T> {
    /// Zero-based input index.
    pub index: usize,
    /// Operation output.
    pub value: T,
}

/// A failed operation result captured by [`map_bounded_collect`].
#[derive(Debug, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub struct TaskFailure<E> {
    /// Zero-based input index.
    pub index: usize,
    /// Caller-provided failure cause.
    pub error: E,
}

/// Operation results captured by [`map_bounded_collect`].
#[derive(Debug, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub struct TaskGroupReport<T, E> {
    /// Successful operation outputs sorted by input index.
    pub successes: Vec<TaskSuccess<T>>,
    /// Operation failures sorted by input index.
    pub failures: Vec<TaskFailure<E>>,
}

impl<T, E> TaskGroupReport<T, E> {
    /// Returns `true` when no operation failed.
    #[must_use]
    pub fn is_success(&self) -> bool {
        self.failures.is_empty()
    }

    /// Returns the total number of completed operations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.successes.len() + self.failures.len()
    }

    /// Returns `true` when the report contains no operation result.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.successes.is_empty() && self.failures.is_empty()
    }
}

enum TaskOutcome<T, E> {
    Success { index: usize, value: T },
    Failure { index: usize, error: E },
}

/// Runs operations with a bounded number of Tokio tasks.
///
/// Results are returned in input order. On the first operation or join failure,
/// all sibling tasks are aborted and drained before the error is returned.
///
/// # Errors
///
/// Returns [`TaskGroupError::ZeroConcurrency`] or
/// [`TaskGroupError::ExcessiveConcurrency`] when `max_concurrency` is invalid,
/// [`TaskGroupError::TaskFailed`] for the first operation error, or
/// [`TaskGroupError::TaskJoinFailed`] when a spawned Tokio task panics or is
/// cancelled by the runtime.
pub async fn try_map_bounded<I, F, Fut, T, E>(
    items: I,
    max_concurrency: usize,
    operation: F,
) -> Result<Vec<T>, TaskGroupError<E>>
where
    I: IntoIterator,
    I::Item: Send + 'static,
    F: Fn(I::Item) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<T, E>> + Send + 'static,
    T: Send + 'static,
    E: Send + 'static,
{
    validate_max_concurrency(max_concurrency)?;

    let mut tasks = JoinSet::new();
    let mut indexed_items = items.into_iter().enumerate();
    let operation = Arc::new(operation);
    let mut results = Vec::new();

    fill_tasks(
        &mut tasks,
        &mut indexed_items,
        max_concurrency,
        &operation,
        &mut results,
    );

    while let Some(result) = tasks.join_next().await {
        match result {
            Ok(TaskOutcome::Success { index, value }) => {
                results[index] = Some(value);
                fill_tasks(
                    &mut tasks,
                    &mut indexed_items,
                    max_concurrency,
                    &operation,
                    &mut results,
                );
            }
            Ok(TaskOutcome::Failure { index, error }) => {
                shutdown_tasks(&mut tasks).await;
                return Err(TaskGroupError::TaskFailed { index, error });
            }
            Err(source) => {
                shutdown_tasks(&mut tasks).await;
                return Err(TaskGroupError::TaskJoinFailed {
                    index: None,
                    source,
                });
            }
        }
    }

    Ok(results.into_iter().flatten().collect())
}

/// Runs operations with bounded concurrency and records every operation result.
///
/// Operation errors are stored in the returned [`TaskGroupReport`] instead of
/// cancelling sibling tasks. Tokio join failures still abort and drain remaining
/// tasks because they indicate a task panic or runtime-level cancellation.
///
/// # Errors
///
/// Returns [`TaskGroupError::ZeroConcurrency`] or
/// [`TaskGroupError::ExcessiveConcurrency`] when `max_concurrency` is invalid,
/// or [`TaskGroupError::TaskJoinFailed`] when a spawned Tokio task panics or is
/// cancelled by the runtime.
pub async fn map_bounded_collect<I, F, Fut, T, E>(
    items: I,
    max_concurrency: usize,
    operation: F,
) -> Result<TaskGroupReport<T, E>, TaskGroupError<E>>
where
    I: IntoIterator,
    I::Item: Send + 'static,
    F: Fn(I::Item) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<T, E>> + Send + 'static,
    T: Send + 'static,
    E: Send + 'static,
{
    validate_max_concurrency(max_concurrency)?;

    let mut tasks = JoinSet::new();
    let mut indexed_items = items.into_iter().enumerate();
    let operation = Arc::new(operation);
    let mut successes = Vec::new();
    let mut failures = Vec::new();
    let mut slots = Vec::new();

    fill_tasks(
        &mut tasks,
        &mut indexed_items,
        max_concurrency,
        &operation,
        &mut slots,
    );

    while let Some(result) = tasks.join_next().await {
        match result {
            Ok(TaskOutcome::Success { index, value }) => {
                successes.push(TaskSuccess { index, value });
            }
            Ok(TaskOutcome::Failure { index, error }) => {
                failures.push(TaskFailure { index, error });
            }
            Err(source) => {
                shutdown_tasks(&mut tasks).await;
                return Err(TaskGroupError::TaskJoinFailed {
                    index: None,
                    source,
                });
            }
        }

        fill_tasks(
            &mut tasks,
            &mut indexed_items,
            max_concurrency,
            &operation,
            &mut slots,
        );
    }

    successes.sort_by_key(|success| success.index);
    failures.sort_by_key(|failure| failure.index);

    Ok(TaskGroupReport {
        successes,
        failures,
    })
}

fn validate_max_concurrency<E>(max_concurrency: usize) -> Result<(), TaskGroupError<E>> {
    if max_concurrency == 0 {
        return Err(TaskGroupError::ZeroConcurrency);
    }
    if max_concurrency > MAX_CONCURRENCY {
        return Err(TaskGroupError::ExcessiveConcurrency {
            max_concurrency,
            upper_bound: MAX_CONCURRENCY,
        });
    }
    Ok(())
}

fn fill_tasks<I, F, Fut, T, E>(
    tasks: &mut JoinSet<TaskOutcome<T, E>>,
    indexed_items: &mut std::iter::Enumerate<I>,
    max_concurrency: usize,
    operation: &Arc<F>,
    slots: &mut Vec<Option<T>>,
) where
    I: Iterator,
    I::Item: Send + 'static,
    F: Fn(I::Item) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<T, E>> + Send + 'static,
    T: Send + 'static,
    E: Send + 'static,
{
    while tasks.len() < max_concurrency {
        let Some((index, item)) = indexed_items.next() else {
            break;
        };

        while slots.len() <= index {
            slots.push(None);
        }

        let operation = Arc::clone(operation);
        tasks.spawn(async move {
            match operation(item).await {
                Ok(value) => TaskOutcome::Success { index, value },
                Err(error) => TaskOutcome::Failure { index, error },
            }
        });
    }
}

async fn shutdown_tasks<T, E>(tasks: &mut JoinSet<TaskOutcome<T, E>>)
where
    T: Send + 'static,
    E: Send + 'static,
{
    tasks.abort_all();
    while tasks.join_next().await.is_some() {}
}

#[cfg(test)]
mod tests {
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

    #[tokio::test]
    async fn try_map_bounded_preserves_input_order() {
        let values = try_map_bounded([3, 1, 2], 2, |value| async move {
            sleep(Duration::from_millis((4 - value) * 10)).await;
            Ok::<_, &'static str>(value * 10)
        })
        .await
        .unwrap();

        assert_eq!(values, vec![30, 10, 20]);
    }

    #[tokio::test]
    async fn try_map_bounded_respects_concurrency_bound() {
        let current = Arc::new(AtomicUsize::new(0));
        let peak = Arc::new(AtomicUsize::new(0));

        let values = try_map_bounded(0..10, 3, {
            let current = Arc::clone(&current);
            let peak = Arc::clone(&peak);
            move |value| {
                let current = Arc::clone(&current);
                let peak = Arc::clone(&peak);
                async move {
                    let active = current.fetch_add(1, Ordering::SeqCst) + 1;
                    peak.fetch_max(active, Ordering::SeqCst);
                    sleep(Duration::from_millis(5)).await;
                    current.fetch_sub(1, Ordering::SeqCst);
                    Ok::<_, &'static str>(value)
                }
            }
        })
        .await
        .unwrap();

        assert_eq!(values, (0..10).collect::<Vec<_>>());
        assert!(peak.load(Ordering::SeqCst) <= 3);
    }

    #[tokio::test]
    async fn try_map_bounded_aborts_and_drains_siblings_on_first_error() {
        let started = Arc::new(Notify::new());
        let dropped = Arc::new(AtomicUsize::new(0));

        let actual = try_map_bounded(0..2, 2, {
            let started = Arc::clone(&started);
            let dropped = Arc::clone(&dropped);
            move |value| {
                let started = Arc::clone(&started);
                let dropped = Arc::clone(&dropped);
                async move {
                    if value == 0 {
                        started.notified().await;
                        return Err("boom");
                    }

                    let _guard = DropCounter { counter: dropped };
                    started.notify_one();
                    sleep(Duration::from_secs(60)).await;
                    Ok::<_, &'static str>(value)
                }
            }
        })
        .await;

        assert!(matches!(
            actual,
            Err(TaskGroupError::TaskFailed {
                index: 0,
                error: "boom"
            })
        ));
        assert_eq!(dropped.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn map_bounded_collect_records_all_operation_results() {
        let report = map_bounded_collect(0..5, 2, |value| async move {
            if value % 2 == 0 {
                Ok(value * 10)
            } else {
                Err(value)
            }
        })
        .await
        .unwrap();

        assert!(!report.is_success());
        assert_eq!(report.len(), 5);
        assert_eq!(
            report.successes,
            vec![
                TaskSuccess { index: 0, value: 0 },
                TaskSuccess {
                    index: 2,
                    value: 20
                },
                TaskSuccess {
                    index: 4,
                    value: 40
                },
            ]
        );
        assert_eq!(
            report.failures,
            vec![
                TaskFailure { index: 1, error: 1 },
                TaskFailure { index: 3, error: 3 },
            ]
        );
    }

    #[tokio::test]
    async fn rejects_zero_concurrency() {
        let actual =
            try_map_bounded([1], 0, |value| async move { Ok::<_, &'static str>(value) }).await;

        assert!(matches!(actual, Err(TaskGroupError::ZeroConcurrency)));
    }

    #[tokio::test]
    async fn rejects_excessive_concurrency() {
        let actual = try_map_bounded([1], MAX_CONCURRENCY + 1, |value| async move {
            Ok::<_, &'static str>(value)
        })
        .await;

        assert!(matches!(
            actual,
            Err(TaskGroupError::ExcessiveConcurrency {
                max_concurrency,
                upper_bound: MAX_CONCURRENCY
            }) if max_concurrency == MAX_CONCURRENCY + 1
        ));
    }

    #[tokio::test]
    async fn reports_join_failure_and_drains_remaining_tasks() {
        let dropped = Arc::new(AtomicUsize::new(0));

        let actual = try_map_bounded(0..2, 2, {
            let dropped = Arc::clone(&dropped);
            move |value| {
                let dropped = Arc::clone(&dropped);
                async move {
                    if value == 0 {
                        panic!("task panic");
                    }

                    let _guard = DropCounter { counter: dropped };
                    sleep(Duration::from_secs(60)).await;
                    Ok::<_, &'static str>(value)
                }
            }
        })
        .await;

        assert!(matches!(
            actual,
            Err(TaskGroupError::TaskJoinFailed { source, .. }) if source.is_panic()
        ));
        assert_eq!(dropped.load(Ordering::SeqCst), 1);
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

    #[tokio::test]
    async fn run_until_cancelled_returns_value_before_cancellation() {
        let (_source, token) = CancellationSource::new();

        let actual = run_until_cancelled(token, async { 7 }).await;

        assert_eq!(actual, Ok(7));
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
                    sleep(Duration::from_secs(60)).await;
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
}
