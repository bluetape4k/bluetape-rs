use std::future::Future;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

use tokio::task::{JoinError, JoinSet};

pub(crate) const DEFAULT_WORKERS: usize = 16;
pub(crate) const MIN_WORKERS: usize = 1;
pub(crate) const MAX_WORKERS: usize = 2_000;
pub(crate) const DEFAULT_ROUNDS: usize = 2;
pub(crate) const MIN_ROUNDS: usize = 1;
pub(crate) const MAX_ROUNDS: usize = 1_000_000;

type SyncTestBlock<E> = Arc<dyn Fn() -> Result<(), E> + Send + Sync + 'static>;
type AsyncTestBlock<E> =
    Arc<dyn Fn() -> Pin<Box<dyn Future<Output = Result<(), E>> + Send>> + Send + Sync + 'static>;
/// Configuration for bounded concurrent test execution.
///
/// The total number of operation attempts is `workers * iterations_per_worker`.
/// Use this for small deterministic stress checks in tests, not for production
/// scheduling.
///
/// # Examples
///
/// ```
/// use bluetape_rs_test::ConcurrentConfig;
///
/// let config = ConcurrentConfig::new(8, 25);
/// assert_eq!(config.workers, 8);
/// assert_eq!(config.iterations_per_worker, 25);
/// ```
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
    ///
    /// Validation is performed by [`run_concurrently`], so construction stays
    /// cheap and infallible.
    ///
    /// # Examples
    ///
    /// ```
    /// use bluetape_rs_test::ConcurrentConfig;
    ///
    /// let config = ConcurrentConfig::new(4, 10);
    /// assert_eq!(config.workers * config.iterations_per_worker, 40);
    /// ```
    #[must_use]
    pub const fn new(workers: usize, iterations_per_worker: usize) -> Self {
        Self {
            workers,
            iterations_per_worker,
        }
    }
}

/// Error returned by concurrent test execution.
///
/// Worker, round, block, and iteration indexes are zero-based so failed
/// concurrent checks can be reproduced with a smaller targeted test.
///
/// # Examples
///
/// ```
/// use bluetape_rs_test::ConcurrentAssertError;
///
/// let error: ConcurrentAssertError<&'static str> =
///     ConcurrentAssertError::InvalidWorkers { workers: 0 };
/// assert!(error.to_string().contains("workers"));
/// ```
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
///
/// The tester is useful for exercising thread-safe synchronous code paths. Each
/// registered block is selected repeatedly across worker threads until the
/// configured number of rounds has completed or the first failure is observed.
///
/// # Examples
///
/// ```
/// use std::sync::Arc;
/// use std::sync::atomic::{AtomicUsize, Ordering};
/// use bluetape_rs_test::MultithreadingTester;
///
/// let calls = Arc::new(AtomicUsize::new(0));
/// let observed = Arc::clone(&calls);
///
/// MultithreadingTester::new()
///     .workers(2)
///     .rounds(4)
///     .add(move || {
///         observed.fetch_add(1, Ordering::SeqCst);
///         Ok::<(), &'static str>(())
///     })
///     .run()?;
///
/// assert_eq!(calls.load(Ordering::SeqCst), 8);
/// # Ok::<(), bluetape_rs_test::ConcurrentAssertError<&'static str>>(())
/// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use bluetape_rs_test::MultithreadingTester;
    ///
    /// let tester = MultithreadingTester::<&'static str>::new();
    /// let _tester = tester.workers(2).rounds(2);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the number of OS worker threads.
    ///
    /// # Examples
    ///
    /// ```
    /// use bluetape_rs_test::MultithreadingTester;
    ///
    /// let _tester = MultithreadingTester::<&'static str>::new().workers(4);
    /// ```
    #[must_use]
    pub fn workers(mut self, workers: usize) -> Self {
        self.workers = workers;
        self
    }

    /// Sets the number of rounds per worker.
    ///
    /// # Examples
    ///
    /// ```
    /// use bluetape_rs_test::MultithreadingTester;
    ///
    /// let _tester = MultithreadingTester::<&'static str>::new().rounds(10);
    /// ```
    #[must_use]
    pub fn rounds(mut self, rounds: usize) -> Self {
        self.rounds = rounds;
        self
    }

    /// Registers a synchronous test block.
    ///
    /// # Examples
    ///
    /// ```
    /// use bluetape_rs_test::MultithreadingTester;
    ///
    /// let tester = MultithreadingTester::new().add(|| Ok::<(), &'static str>(()));
    /// tester.workers(1).rounds(1).run()?;
    /// # Ok::<(), bluetape_rs_test::ConcurrentAssertError<&'static str>>(())
    /// ```
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
    ///
    /// # Errors
    ///
    /// Returns [`ConcurrentAssertError::NoBlocks`] when no block is registered,
    /// [`ConcurrentAssertError::InvalidWorkers`] or
    /// [`ConcurrentAssertError::InvalidRounds`] when bounds are invalid,
    /// [`ConcurrentAssertError::TooFewWorkers`] when there are fewer workers
    /// than registered blocks, or the first block/thread failure observed.
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
///
/// This tester mirrors [`MultithreadingTester`] for async code. Blocks are run
/// on Tokio tasks and remaining tasks are aborted after the first observed
/// failure.
///
/// # Examples
///
/// ```
/// use std::sync::Arc;
/// use std::sync::atomic::{AtomicUsize, Ordering};
/// use bluetape_rs_test::SuspendedJobTester;
///
/// # async fn demo() -> Result<(), bluetape_rs_test::ConcurrentAssertError<&'static str>> {
/// let calls = Arc::new(AtomicUsize::new(0));
/// let observed = Arc::clone(&calls);
///
/// SuspendedJobTester::new()
///     .workers(2)
///     .rounds(3)
///     .add(move || {
///         let observed = Arc::clone(&observed);
///         async move {
///             observed.fetch_add(1, Ordering::SeqCst);
///             Ok::<(), &'static str>(())
///         }
///     })
///     .run()
///     .await?;
///
/// assert_eq!(calls.load(Ordering::SeqCst), 6);
/// # Ok(())
/// # }
/// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use bluetape_rs_test::SuspendedJobTester;
    ///
    /// let tester = SuspendedJobTester::<&'static str>::new();
    /// let _tester = tester.workers(2).rounds(2);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the number of Tokio worker tasks.
    ///
    /// # Examples
    ///
    /// ```
    /// use bluetape_rs_test::SuspendedJobTester;
    ///
    /// let _tester = SuspendedJobTester::<&'static str>::new().workers(4);
    /// ```
    #[must_use]
    pub fn workers(mut self, workers: usize) -> Self {
        self.workers = workers;
        self
    }

    /// Sets the number of rounds per worker.
    ///
    /// # Examples
    ///
    /// ```
    /// use bluetape_rs_test::SuspendedJobTester;
    ///
    /// let _tester = SuspendedJobTester::<&'static str>::new().rounds(10);
    /// ```
    #[must_use]
    pub fn rounds(mut self, rounds: usize) -> Self {
        self.rounds = rounds;
        self
    }

    /// Registers an async test block.
    ///
    /// # Examples
    ///
    /// ```
    /// use bluetape_rs_test::SuspendedJobTester;
    ///
    /// # async fn demo() -> Result<(), bluetape_rs_test::ConcurrentAssertError<&'static str>> {
    /// SuspendedJobTester::new()
    ///     .workers(1)
    ///     .rounds(1)
    ///     .add(|| async { Ok::<(), &'static str>(()) })
    ///     .run()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
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
    ///
    /// # Errors
    ///
    /// Returns [`ConcurrentAssertError::NoBlocks`] when no block is registered,
    /// [`ConcurrentAssertError::InvalidWorkers`] or
    /// [`ConcurrentAssertError::InvalidRounds`] when bounds are invalid, or the
    /// first async block/task failure observed.
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
/// Runs `operation` concurrently across bounded worker tasks.
///
/// `operation` receives `(worker_index, iteration_index)`. The helper returns
/// after all workers finish, or after the first operation/join failure observed
/// while collecting worker results.
///
/// # Examples
///
/// ```
/// use std::sync::Arc;
/// use std::sync::atomic::{AtomicUsize, Ordering};
/// use bluetape_rs_test::{ConcurrentConfig, run_concurrently};
///
/// # async fn demo() -> Result<(), bluetape_rs_test::ConcurrentAssertError<&'static str>> {
/// let calls = Arc::new(AtomicUsize::new(0));
/// let observed = Arc::clone(&calls);
///
/// run_concurrently(ConcurrentConfig::new(2, 5), move |_worker, _iteration| {
///     let observed = Arc::clone(&observed);
///     async move {
///         observed.fetch_add(1, Ordering::SeqCst);
///         Ok::<(), &'static str>(())
///     }
/// }).await?;
///
/// assert_eq!(calls.load(Ordering::SeqCst), 10);
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// Returns [`ConcurrentAssertError::ZeroWorkers`],
/// [`ConcurrentAssertError::ZeroIterations`],
/// [`ConcurrentAssertError::InvalidWorkers`], or
/// [`ConcurrentAssertError::InvalidRounds`] for invalid config values. Returns
/// [`ConcurrentAssertError::OperationFailed`] for the first operation error, or
/// [`ConcurrentAssertError::WorkerJoinFailed`] when a Tokio worker task cannot
/// be joined.
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
