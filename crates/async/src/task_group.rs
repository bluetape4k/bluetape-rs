//! Bounded Tokio task group helpers.

use std::error::Error;
use std::fmt;
use std::future::Future;
use std::sync::Arc;

use tokio::task::{JoinError, JoinSet};

/// Default concurrency bound for callers that do not need a custom limit.
pub const DEFAULT_MAX_CONCURRENCY: usize = 16;

/// Maximum accepted concurrency bound.
///
/// This is intentionally conservative. Higher fan-out usually needs explicit
/// queueing, backpressure, or service-specific resource limits.
pub const MAX_CONCURRENCY: usize = 10_000;

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
    use tokio::time::{Duration, sleep};

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
}
