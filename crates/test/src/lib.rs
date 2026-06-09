//! Reusable test support helpers.
//!
//! ```
//! use bluetape_rs_test::TempDir;
//!
//! let temp = TempDir::new("bluetape-rs-test").expect("temp dir");
//! assert!(temp.path().exists());
//! temp.close().expect("cleanup");
//! ```

use std::future::Future;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::task::{JoinError, JoinSet};
use tokio::time::{Instant, sleep_until, timeout_at};

static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(0);
const MIN_POLL_INTERVAL: Duration = Duration::from_millis(1);
const DEFAULT_WORKERS: usize = 16;
const MIN_WORKERS: usize = 1;
const MAX_WORKERS: usize = 2_000;
const DEFAULT_ROUNDS: usize = 2;
const MIN_ROUNDS: usize = 1;
const MAX_ROUNDS: usize = 1_000_000;

type SyncTestBlock<E> = Arc<dyn Fn() -> Result<(), E> + Send + Sync + 'static>;
type AsyncTestBlock<E> =
    Arc<dyn Fn() -> Pin<Box<dyn Future<Output = Result<(), E>> + Send>> + Send + Sync + 'static>;

/// Error returned by asynchronous assertion helpers.
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

/// Configuration for bounded concurrent test execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct ConcurrentConfig {
    /// Number of asynchronous worker tasks to spawn.
    pub workers: usize,
    /// Number of times each worker runs the operation.
    pub iterations_per_worker: usize,
}

impl ConcurrentConfig {
    /// Creates a bounded concurrent execution config.
    #[must_use]
    pub const fn new(workers: usize, iterations_per_worker: usize) -> Self {
        Self {
            workers,
            iterations_per_worker,
        }
    }
}

/// Error returned by concurrent test execution.
#[derive(Debug)]
#[non_exhaustive]
pub enum ConcurrentAssertError<E> {
    /// No test block was registered.
    NoBlocks,
    /// `workers` was outside the accepted range.
    InvalidWorkers {
        /// Rejected worker count.
        workers: usize,
    },
    /// `rounds` was outside the accepted range.
    InvalidRounds {
        /// Rejected round count.
        rounds: usize,
    },
    /// `workers` must be greater than zero.
    ZeroWorkers,
    /// `iterations_per_worker` must be greater than zero.
    ZeroIterations,
    /// The worker count is smaller than the number of registered blocks.
    TooFewWorkers {
        /// Configured worker count.
        workers: usize,
        /// Number of registered blocks.
        blocks: usize,
    },
    /// A registered test block failed.
    TestBlockFailed {
        /// Zero-based worker index.
        worker: usize,
        /// Zero-based round index.
        round: usize,
        /// Zero-based registered block index.
        block: usize,
        /// Caller-provided failure cause.
        error: E,
    },
    /// A registered synchronous test block panicked.
    TestBlockPanicked {
        /// Zero-based worker index.
        worker: usize,
        /// Zero-based round index.
        round: usize,
        /// Zero-based registered block index.
        block: usize,
    },
    /// An operation failed at the given worker and iteration.
    OperationFailed {
        /// Zero-based worker index.
        worker: usize,
        /// Zero-based iteration index for the worker.
        iteration: usize,
        /// Caller-provided failure cause.
        error: E,
    },
    /// A worker task failed to join, usually because it panicked or was
    /// cancelled by the runtime.
    WorkerJoinFailed {
        /// Zero-based worker index when the runtime can report it.
        worker: Option<usize>,
        /// Tokio join error.
        source: JoinError,
    },
    /// A worker OS thread could not be spawned.
    WorkerThreadSpawnFailed {
        /// Zero-based worker index.
        worker: usize,
        /// Thread spawn error.
        source: std::io::Error,
    },
    /// A worker OS thread panicked outside a registered test block.
    WorkerThreadPanicked {
        /// Zero-based worker index.
        worker: usize,
    },
}

impl<E> std::fmt::Display for ConcurrentAssertError<E>
where
    E: std::fmt::Display,
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoBlocks => formatter.write_str("no test blocks were registered"),
            Self::InvalidWorkers { workers } => write!(
                formatter,
                "workers must be in {MIN_WORKERS}..={MAX_WORKERS}, got {workers}"
            ),
            Self::InvalidRounds { rounds } => write!(
                formatter,
                "rounds must be in {MIN_ROUNDS}..={MAX_ROUNDS}, got {rounds}"
            ),
            Self::ZeroWorkers => formatter.write_str("workers must be greater than zero"),
            Self::ZeroIterations => {
                formatter.write_str("iterations_per_worker must be greater than zero")
            }
            Self::TooFewWorkers { workers, blocks } => write!(
                formatter,
                "workers ({workers}) must be greater than or equal to registered blocks ({blocks})"
            ),
            Self::TestBlockFailed {
                worker,
                round,
                block,
                error,
            } => write!(
                formatter,
                "test block {block} failed at worker {worker}, round {round}: {error}"
            ),
            Self::TestBlockPanicked {
                worker,
                round,
                block,
            } => write!(
                formatter,
                "test block {block} panicked at worker {worker}, round {round}"
            ),
            Self::OperationFailed {
                worker,
                iteration,
                error,
            } => write!(
                formatter,
                "concurrent operation failed at worker {worker}, iteration {iteration}: {error}"
            ),
            Self::WorkerJoinFailed {
                worker: Some(worker),
                source,
            } => write!(formatter, "worker {worker} failed to join: {source}"),
            Self::WorkerJoinFailed {
                worker: None,
                source,
            } => write!(formatter, "worker failed to join: {source}"),
            Self::WorkerThreadSpawnFailed { worker, source } => {
                write!(
                    formatter,
                    "worker thread {worker} failed to spawn: {source}"
                )
            }
            Self::WorkerThreadPanicked { worker } => {
                write!(formatter, "worker thread {worker} panicked")
            }
        }
    }
}

impl<E> std::error::Error for ConcurrentAssertError<E>
where
    E: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::TestBlockFailed { error, .. } => Some(error),
            Self::OperationFailed { error, .. } => Some(error),
            Self::WorkerJoinFailed { source, .. } => Some(source),
            Self::WorkerThreadSpawnFailed { source, .. } => Some(source),
            Self::NoBlocks
            | Self::InvalidWorkers { .. }
            | Self::InvalidRounds { .. }
            | Self::ZeroWorkers
            | Self::ZeroIterations
            | Self::TooFewWorkers { .. }
            | Self::TestBlockPanicked { .. }
            | Self::WorkerThreadPanicked { .. } => None,
        }
    }
}

/// Runs registered synchronous blocks on bounded OS worker threads.
#[derive(Clone)]
pub struct MultithreadingTester<E> {
    workers: usize,
    rounds: usize,
    blocks: Vec<SyncTestBlock<E>>,
}

impl<E> Default for MultithreadingTester<E> {
    fn default() -> Self {
        Self {
            workers: DEFAULT_WORKERS,
            rounds: DEFAULT_ROUNDS,
            blocks: Vec::new(),
        }
    }
}

impl<E> MultithreadingTester<E> {
    /// Creates a tester with the default worker and round counts.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the number of OS worker threads.
    #[must_use]
    pub fn workers(mut self, workers: usize) -> Self {
        self.workers = workers;
        self
    }

    /// Sets the number of rounds per worker.
    #[must_use]
    pub fn rounds(mut self, rounds: usize) -> Self {
        self.rounds = rounds;
        self
    }

    /// Registers a synchronous test block.
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub fn add<F>(mut self, block: F) -> Self
    where
        F: Fn() -> Result<(), E> + Send + Sync + 'static,
    {
        self.blocks.push(Arc::new(block));
        self
    }

    /// Runs all registered blocks on bounded OS worker threads.
    pub fn run(self) -> Result<(), ConcurrentAssertError<E>>
    where
        E: Send + 'static,
    {
        validate_tester_config(self.workers, self.rounds, self.blocks.len(), true)?;

        let workers = self.workers;
        let rounds = self.rounds;
        let blocks = Arc::new(self.blocks);
        let total_runs = workers * rounds;
        let next = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let stop = Arc::new(AtomicBool::new(false));
        let mut handles = Vec::with_capacity(workers);
        let mut spawn_error = None;

        for worker in 0..workers {
            let blocks = Arc::clone(&blocks);
            let next = Arc::clone(&next);
            let stop = Arc::clone(&stop);
            let stop_on_spawn_error = Arc::clone(&stop);
            match thread::Builder::new().spawn(move || {
                loop {
                    if stop.load(Ordering::SeqCst) {
                        return Ok(());
                    }

                    let run_index = next.fetch_add(1, Ordering::SeqCst);
                    if run_index >= total_runs {
                        return Ok(());
                    }

                    let block = run_index % blocks.len();
                    let round = run_index / workers;
                    let result = catch_unwind(AssertUnwindSafe(|| blocks[block]()));
                    match result {
                        Ok(Ok(())) => {}
                        Ok(Err(error)) => {
                            stop.store(true, Ordering::SeqCst);
                            return Err(ConcurrentAssertError::TestBlockFailed {
                                worker,
                                round,
                                block,
                                error,
                            });
                        }
                        Err(_) => {
                            stop.store(true, Ordering::SeqCst);
                            return Err(ConcurrentAssertError::TestBlockPanicked {
                                worker,
                                round,
                                block,
                            });
                        }
                    }
                }
            }) {
                Ok(handle) => handles.push((worker, handle)),
                Err(source) => {
                    stop_on_spawn_error.store(true, Ordering::SeqCst);
                    spawn_error =
                        Some(ConcurrentAssertError::WorkerThreadSpawnFailed { worker, source });
                    break;
                }
            }
        }

        let mut first_error = spawn_error;
        for (worker, handle) in handles {
            match handle.join() {
                Ok(Ok(())) => {}
                Ok(Err(error)) => {
                    if first_error.is_none() {
                        first_error = Some(error);
                    }
                }
                Err(_) => {
                    if first_error.is_none() {
                        first_error = Some(ConcurrentAssertError::WorkerThreadPanicked { worker });
                    }
                }
            }
        }

        match first_error {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }
}

/// Runs registered async blocks on bounded Tokio worker tasks.
#[derive(Clone)]
pub struct SuspendedJobTester<E> {
    workers: usize,
    rounds: usize,
    blocks: Vec<AsyncTestBlock<E>>,
}

impl<E> Default for SuspendedJobTester<E> {
    fn default() -> Self {
        Self {
            workers: DEFAULT_WORKERS,
            rounds: DEFAULT_ROUNDS,
            blocks: Vec::new(),
        }
    }
}

impl<E> SuspendedJobTester<E> {
    /// Creates a tester with the default worker and round counts.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the number of Tokio worker tasks.
    #[must_use]
    pub fn workers(mut self, workers: usize) -> Self {
        self.workers = workers;
        self
    }

    /// Sets the number of rounds per worker.
    #[must_use]
    pub fn rounds(mut self, rounds: usize) -> Self {
        self.rounds = rounds;
        self
    }

    /// Registers an async test block.
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub fn add<F, Fut>(mut self, block: F) -> Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), E>> + Send + 'static,
    {
        self.blocks.push(Arc::new(move || Box::pin(block())));
        self
    }

    /// Runs all registered async blocks on bounded Tokio worker tasks.
    pub async fn run(self) -> Result<(), ConcurrentAssertError<E>>
    where
        E: Send + 'static,
    {
        validate_tester_config(self.workers, self.rounds, self.blocks.len(), false)?;

        let workers = self.workers;
        let rounds = self.rounds;
        let block_count = self.blocks.len();
        let total_runs = workers * rounds;
        let blocks = Arc::new(self.blocks);
        let next = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let mut tasks = JoinSet::new();

        for worker in 0..workers {
            let blocks = Arc::clone(&blocks);
            let next = Arc::clone(&next);
            tasks.spawn(async move {
                loop {
                    let run_index = next.fetch_add(1, Ordering::SeqCst);
                    if run_index >= total_runs {
                        return (worker, Ok(()));
                    }

                    let block = run_index % block_count;
                    let round = run_index / workers;
                    if let Err(error) = blocks[block]().await {
                        return (
                            worker,
                            Err(ConcurrentAssertError::TestBlockFailed {
                                worker,
                                round,
                                block,
                                error,
                            }),
                        );
                    }
                }
            });
        }

        while let Some(result) = tasks.join_next().await {
            match result {
                Ok((_worker, Ok(()))) => {}
                Ok((_worker, Err(error))) => {
                    abort_and_drain(&mut tasks).await;
                    return Err(error);
                }
                Err(source) => {
                    abort_and_drain(&mut tasks).await;
                    return Err(ConcurrentAssertError::WorkerJoinFailed {
                        worker: None,
                        source,
                    });
                }
            }
        }

        Ok(())
    }
}

async fn abort_and_drain<E>(tasks: &mut JoinSet<(usize, Result<(), ConcurrentAssertError<E>>)>)
where
    E: Send + 'static,
{
    tasks.abort_all();
    while tasks.join_next().await.is_some() {}
}

fn validate_concurrent_config<E>(config: ConcurrentConfig) -> Result<(), ConcurrentAssertError<E>> {
    if config.workers == 0 {
        return Err(ConcurrentAssertError::ZeroWorkers);
    }
    if config.iterations_per_worker == 0 {
        return Err(ConcurrentAssertError::ZeroIterations);
    }
    if config.workers > MAX_WORKERS {
        return Err(ConcurrentAssertError::InvalidWorkers {
            workers: config.workers,
        });
    }
    if config.iterations_per_worker > MAX_ROUNDS {
        return Err(ConcurrentAssertError::InvalidRounds {
            rounds: config.iterations_per_worker,
        });
    }
    Ok(())
}

fn validate_tester_config<E>(
    workers: usize,
    rounds: usize,
    blocks: usize,
    require_workers_at_least_blocks: bool,
) -> Result<(), ConcurrentAssertError<E>> {
    if !(MIN_WORKERS..=MAX_WORKERS).contains(&workers) {
        return Err(ConcurrentAssertError::InvalidWorkers { workers });
    }
    if !(MIN_ROUNDS..=MAX_ROUNDS).contains(&rounds) {
        return Err(ConcurrentAssertError::InvalidRounds { rounds });
    }
    if blocks == 0 {
        return Err(ConcurrentAssertError::NoBlocks);
    }
    if require_workers_at_least_blocks && workers < blocks {
        return Err(ConcurrentAssertError::TooFewWorkers { workers, blocks });
    }
    Ok(())
}

/// Retries `condition` until it succeeds or `timeout` expires.
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

/// Runs `operation` concurrently across bounded worker tasks.
///
/// `operation` receives `(worker_index, iteration_index)`. The helper returns
/// after all workers finish, or after the first operation/join failure observed
/// while collecting worker results.
pub async fn run_concurrently<F, Fut, E>(
    config: ConcurrentConfig,
    operation: F,
) -> Result<(), ConcurrentAssertError<E>>
where
    F: Fn(usize, usize) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), E>> + Send + 'static,
    E: Send + 'static,
{
    validate_concurrent_config(config)?;

    let operation = Arc::new(operation);
    let mut tasks = JoinSet::new();

    for worker in 0..config.workers {
        let operation = Arc::clone(&operation);
        tasks.spawn(async move {
            for iteration in 0..config.iterations_per_worker {
                if let Err(error) = operation(worker, iteration).await {
                    return (
                        worker,
                        Err(ConcurrentAssertError::OperationFailed {
                            worker,
                            iteration,
                            error,
                        }),
                    );
                }
            }
            (worker, Ok(()))
        });
    }

    while let Some(result) = tasks.join_next().await {
        match result {
            Ok((_worker, Ok(()))) => {}
            Ok((_worker, Err(error))) => {
                abort_and_drain(&mut tasks).await;
                return Err(error);
            }
            Err(source) => {
                abort_and_drain(&mut tasks).await;
                return Err(ConcurrentAssertError::WorkerJoinFailed {
                    worker: None,
                    source,
                });
            }
        }
    }

    Ok(())
}

/// Requires `condition` to stay successful for `duration`.
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

/// Temporary directory removed on drop.
#[derive(Debug)]
pub struct TempDir {
    path: PathBuf,
}

impl TempDir {
    /// Creates a temporary directory under the process temp directory.
    pub fn new(prefix: impl AsRef<str>) -> std::io::Result<Self> {
        let prefix = prefix.as_ref();
        validate_temp_prefix(prefix)?;
        let id = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let path =
            std::env::temp_dir().join(format!("{prefix}-{}-{nanos}-{id}", std::process::id(),));
        create_private_dir(&path)?;
        Ok(Self { path })
    }

    /// Returns the temporary directory path.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Removes the directory now and prevents duplicate cleanup in `Drop`.
    pub fn close(mut self) -> std::io::Result<()> {
        close_path(&mut self.path, |path| std::fs::remove_dir_all(path))
    }
}

fn close_path(
    path: &mut PathBuf,
    remove_dir_all: impl FnOnce(&Path) -> std::io::Result<()>,
) -> std::io::Result<()> {
    if path.as_os_str().is_empty() {
        return Ok(());
    }
    remove_dir_all(path.as_path())?;
    path.clear();
    Ok(())
}

#[cfg(unix)]
fn create_private_dir(path: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::DirBuilderExt;

    std::fs::DirBuilder::new().mode(0o700).create(path)
}

#[cfg(not(unix))]
fn create_private_dir(path: &Path) -> std::io::Result<()> {
    std::fs::create_dir(path)
}

fn validate_temp_prefix(prefix: &str) -> std::io::Result<()> {
    if prefix.is_empty()
        || Path::new(prefix).is_absolute()
        || prefix.contains('/')
        || prefix.contains('\\')
    {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "temporary directory prefix must be a single relative path segment",
        ));
    }
    Ok(())
}

impl Drop for TempDir {
    fn drop(&mut self) {
        if !self.path.as_os_str().is_empty() {
            let _ = std::fs::remove_dir_all(&self.path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::sync::atomic::{AtomicBool, AtomicUsize};

    #[tokio::test(start_paused = true)]
    async fn eventually_succeeds_after_retries() {
        let attempts = Arc::new(AtomicUsize::new(0));
        let attempts_for_assertion = Arc::clone(&attempts);

        eventually(
            Duration::from_secs(1),
            Duration::from_millis(10),
            move || {
                let attempts = Arc::clone(&attempts_for_assertion);
                async move {
                    if attempts.fetch_add(1, Ordering::SeqCst) >= 2 {
                        Ok(())
                    } else {
                        Err("not yet")
                    }
                }
            },
        )
        .await
        .expect("eventual success");

        assert_eq!(attempts.load(Ordering::SeqCst), 3);
    }

    #[tokio::test(start_paused = true)]
    async fn eventually_returns_last_error_on_timeout() {
        let error = eventually(
            Duration::from_millis(30),
            Duration::from_millis(10),
            || async { Err::<(), _>("still missing") },
        )
        .await
        .expect_err("timeout");

        assert_eq!(
            error,
            AsyncAssertError::Timeout {
                last_error: Some("still missing")
            }
        );
    }

    #[tokio::test(start_paused = true)]
    async fn eventually_times_out_pending_condition() {
        let error = eventually(Duration::from_millis(30), Duration::from_secs(60), || {
            std::future::pending::<Result<(), &'static str>>()
        })
        .await
        .expect_err("pending condition times out");

        assert_eq!(error, AsyncAssertError::Timeout { last_error: None });
    }

    #[tokio::test(start_paused = true)]
    async fn eventually_accepts_zero_interval_with_minimum_poll_interval() {
        let attempts = Arc::new(AtomicUsize::new(0));
        let attempts_for_assertion = Arc::clone(&attempts);

        eventually(Duration::from_secs(1), Duration::ZERO, move || {
            let attempts = Arc::clone(&attempts_for_assertion);
            async move {
                if attempts.fetch_add(1, Ordering::SeqCst) >= 1 {
                    Ok(())
                } else {
                    Err("not yet")
                }
            }
        })
        .await
        .expect("eventual success with zero interval");

        assert_eq!(attempts.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn async_assert_error_exposes_typed_source() {
        #[derive(Debug)]
        struct SourceError;

        impl std::fmt::Display for SourceError {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("source")
            }
        }

        impl Error for SourceError {}

        let error = AsyncAssertError::BecameUnstable { error: SourceError };
        assert!(error.source().is_some());
    }

    #[tokio::test(start_paused = true)]
    async fn consistently_detects_instability() {
        let stable = Arc::new(AtomicBool::new(true));
        let stable_for_task = Arc::clone(&stable);
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(15)).await;
            stable_for_task.store(false, Ordering::SeqCst);
        });

        let error = consistently(
            Duration::from_millis(50),
            Duration::from_millis(10),
            move || {
                let stable = Arc::clone(&stable);
                async move {
                    if stable.load(Ordering::SeqCst) {
                        Ok(())
                    } else {
                        Err("unstable")
                    }
                }
            },
        )
        .await
        .expect_err("instability");

        assert_eq!(
            error,
            AsyncAssertError::BecameUnstable { error: "unstable" }
        );
    }

    #[test]
    fn multithreading_tester_executes_registered_blocks() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_for_block = Arc::clone(&counter);

        MultithreadingTester::new()
            .workers(4)
            .rounds(3)
            .add(move || {
                counter_for_block.fetch_add(1, Ordering::SeqCst);
                Ok::<(), &'static str>(())
            })
            .run()
            .expect("multithreaded execution");

        assert_eq!(counter.load(Ordering::SeqCst), 12);
    }

    #[test]
    fn multithreading_tester_reports_block_failure_location() {
        let error = MultithreadingTester::new()
            .workers(2)
            .rounds(2)
            .add(|| Err::<(), _>("boom"))
            .run()
            .expect_err("block failure");

        match error {
            ConcurrentAssertError::TestBlockFailed {
                worker,
                round,
                block,
                error,
            } => {
                assert!(worker < 2);
                assert!(round < 2);
                assert_eq!(block, 0);
                assert_eq!(error, "boom");
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn multithreading_tester_reports_block_panic() {
        let error = MultithreadingTester::<&'static str>::new()
            .workers(2)
            .rounds(1)
            .add(|| {
                panic!("block panic");
            })
            .run()
            .expect_err("block panic");

        assert!(matches!(
            error,
            ConcurrentAssertError::TestBlockPanicked { block: 0, .. }
        ));
    }

    #[test]
    fn multithreading_tester_rejects_invalid_configuration() {
        let no_blocks = MultithreadingTester::<&'static str>::new()
            .workers(1)
            .rounds(1)
            .run()
            .expect_err("no blocks");
        assert!(matches!(no_blocks, ConcurrentAssertError::NoBlocks));

        let too_few_workers = MultithreadingTester::new()
            .workers(1)
            .rounds(1)
            .add(|| Ok::<(), &'static str>(()))
            .add(|| Ok::<(), &'static str>(()))
            .run()
            .expect_err("too few workers");
        assert!(matches!(
            too_few_workers,
            ConcurrentAssertError::TooFewWorkers {
                workers: 1,
                blocks: 2
            }
        ));
    }

    #[tokio::test]
    async fn suspended_job_tester_executes_registered_blocks() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_for_block = Arc::clone(&counter);

        SuspendedJobTester::new()
            .workers(4)
            .rounds(25)
            .add(move || {
                let counter = Arc::clone(&counter_for_block);
                async move {
                    counter.fetch_add(1, Ordering::SeqCst);
                    Ok::<(), &'static str>(())
                }
            })
            .run()
            .await
            .expect("suspended job execution");

        assert_eq!(counter.load(Ordering::SeqCst), 100);
    }

    #[tokio::test]
    async fn suspended_job_tester_reports_block_failure_location() {
        let error = SuspendedJobTester::new()
            .workers(3)
            .rounds(2)
            .add(|| async { Err::<(), _>("boom") })
            .run()
            .await
            .expect_err("block failure");

        match error {
            ConcurrentAssertError::TestBlockFailed {
                worker,
                round,
                block,
                error,
            } => {
                assert!(worker < 3);
                assert!(round < 2);
                assert_eq!(block, 0);
                assert_eq!(error, "boom");
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[tokio::test]
    async fn suspended_job_tester_reports_task_panic() {
        let error = SuspendedJobTester::<&'static str>::new()
            .workers(2)
            .rounds(1)
            .add(|| async {
                panic!("task panic");
            })
            .run()
            .await
            .expect_err("task panic");

        match error {
            ConcurrentAssertError::WorkerJoinFailed { worker, source } => {
                assert_eq!(worker, None);
                assert!(source.is_panic());
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[tokio::test]
    async fn suspended_job_tester_aborts_siblings_after_failure() {
        let completed_slow_block = Arc::new(AtomicBool::new(false));
        let completed_slow_block_for_assertion = Arc::clone(&completed_slow_block);

        let result = tokio::time::timeout(
            Duration::from_millis(100),
            SuspendedJobTester::new()
                .workers(2)
                .rounds(1)
                .add({
                    let completed_slow_block = Arc::clone(&completed_slow_block);
                    move || {
                        let completed_slow_block = Arc::clone(&completed_slow_block);
                        async move {
                            tokio::time::sleep(Duration::from_secs(60)).await;
                            completed_slow_block.store(true, Ordering::SeqCst);
                            Ok::<(), &'static str>(())
                        }
                    }
                })
                .add(|| async { Err::<(), _>("fail fast") })
                .run(),
        )
        .await
        .expect("tester returns without waiting for sleeping sibling");

        assert!(matches!(
            result,
            Err(ConcurrentAssertError::TestBlockFailed {
                error: "fail fast",
                ..
            })
        ));
        assert!(!completed_slow_block_for_assertion.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn run_concurrently_executes_each_worker_iteration() {
        let attempts = Arc::new(AtomicUsize::new(0));
        let attempts_for_operation = Arc::clone(&attempts);

        run_concurrently(ConcurrentConfig::new(4, 25), move |_worker, _iteration| {
            let attempts = Arc::clone(&attempts_for_operation);
            async move {
                attempts.fetch_add(1, Ordering::SeqCst);
                Ok::<(), &'static str>(())
            }
        })
        .await
        .expect("concurrent execution");

        assert_eq!(attempts.load(Ordering::SeqCst), 100);
    }

    #[tokio::test]
    async fn run_concurrently_returns_operation_failure_location() {
        let error = run_concurrently(
            ConcurrentConfig::new(3, 5),
            |worker, iteration| async move {
                if worker == 1 && iteration == 2 {
                    Err("boom")
                } else {
                    Ok(())
                }
            },
        )
        .await
        .expect_err("operation failure");

        match error {
            ConcurrentAssertError::OperationFailed {
                worker,
                iteration,
                error,
            } => {
                assert_eq!(worker, 1);
                assert_eq!(iteration, 2);
                assert_eq!(error, "boom");
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[tokio::test]
    async fn run_concurrently_rejects_empty_bounds() {
        let zero_workers =
            run_concurrently(ConcurrentConfig::new(0, 1), |_worker, _iteration| async {
                Ok::<(), &'static str>(())
            })
            .await
            .expect_err("zero workers");
        assert!(matches!(zero_workers, ConcurrentAssertError::ZeroWorkers));

        let zero_iterations =
            run_concurrently(ConcurrentConfig::new(1, 0), |_worker, _iteration| async {
                Ok::<(), &'static str>(())
            })
            .await
            .expect_err("zero iterations");
        assert!(matches!(
            zero_iterations,
            ConcurrentAssertError::ZeroIterations
        ));
    }

    #[tokio::test]
    async fn run_concurrently_rejects_excessive_bounds() {
        let too_many_workers = run_concurrently(
            ConcurrentConfig::new(MAX_WORKERS + 1, 1),
            |_worker, _iteration| async { Ok::<(), &'static str>(()) },
        )
        .await
        .expect_err("too many workers");
        assert!(matches!(
            too_many_workers,
            ConcurrentAssertError::InvalidWorkers { workers } if workers == MAX_WORKERS + 1
        ));

        let too_many_iterations = run_concurrently(
            ConcurrentConfig::new(1, MAX_ROUNDS + 1),
            |_worker, _iteration| async { Ok::<(), &'static str>(()) },
        )
        .await
        .expect_err("too many iterations");
        assert!(matches!(
            too_many_iterations,
            ConcurrentAssertError::InvalidRounds { rounds } if rounds == MAX_ROUNDS + 1
        ));
    }

    #[tokio::test]
    async fn run_concurrently_aborts_siblings_after_failure() {
        let completed_slow_worker = Arc::new(AtomicBool::new(false));
        let completed_slow_worker_for_assertion = Arc::clone(&completed_slow_worker);

        let result = tokio::time::timeout(
            Duration::from_millis(100),
            run_concurrently(ConcurrentConfig::new(2, 1), move |worker, _iteration| {
                let completed_slow_worker = Arc::clone(&completed_slow_worker);
                async move {
                    if worker == 0 {
                        tokio::time::sleep(Duration::from_secs(60)).await;
                        completed_slow_worker.store(true, Ordering::SeqCst);
                        Ok(())
                    } else {
                        Err("fail fast")
                    }
                }
            }),
        )
        .await
        .expect("concurrent runner returns without waiting for sleeping sibling");

        assert!(matches!(
            result,
            Err(ConcurrentAssertError::OperationFailed {
                error: "fail fast",
                ..
            })
        ));
        assert!(!completed_slow_worker_for_assertion.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn run_concurrently_reports_worker_panic() {
        let error = run_concurrently(
            ConcurrentConfig::new(2, 1),
            |worker, _iteration| async move {
                if worker == 1 {
                    panic!("worker panic");
                }
                Ok::<(), &'static str>(())
            },
        )
        .await
        .expect_err("worker panic");

        match error {
            ConcurrentAssertError::WorkerJoinFailed { worker, source } => {
                assert_eq!(worker, None);
                assert!(source.is_panic());
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn temp_dir_rejects_path_like_prefixes() {
        for prefix in [
            "",
            "../escape",
            "nested/path",
            "nested\\path",
            "/tmp/escape",
        ] {
            let error = TempDir::new(prefix).expect_err("path-like prefix is rejected");
            assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
        }
    }

    #[test]
    fn temp_dir_removes_directory_on_close() {
        let temp = TempDir::new("bluetape-rs-test").expect("create temp dir");
        let path = temp.path().to_path_buf();
        assert!(path.exists());
        temp.close().expect("close temp dir");
        assert!(!path.exists());
    }

    #[test]
    fn temp_dir_close_keeps_path_after_cleanup_failure() {
        let mut path = PathBuf::from("still-needs-cleanup");
        let error = close_path(&mut path, |_| {
            Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "simulated cleanup failure",
            ))
        })
        .expect_err("cleanup fails");

        assert_eq!(error.kind(), std::io::ErrorKind::PermissionDenied);
        assert_eq!(path, PathBuf::from("still-needs-cleanup"));
    }

    #[test]
    fn temp_dir_removes_directory_on_drop() {
        let path = {
            let temp = TempDir::new("bluetape-rs-test").expect("create temp dir");
            let path = temp.path().to_path_buf();
            assert!(path.exists());
            path
        };
        assert!(!path.exists());
    }

    #[cfg(unix)]
    #[test]
    fn temp_dir_uses_private_unix_permissions() {
        use std::os::unix::fs::PermissionsExt;

        let temp = TempDir::new("bluetape-rs-test").expect("create temp dir");
        let mode = std::fs::metadata(temp.path())
            .expect("metadata")
            .permissions()
            .mode()
            & 0o777;

        assert_eq!(mode, 0o700);
    }
}
