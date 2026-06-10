use crate::concurrent::{MAX_ROUNDS, MAX_WORKERS};
use crate::temp_dir::close_path;
use crate::*;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Duration;

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
    let zero_workers = run_concurrently(ConcurrentConfig::new(0, 1), |_worker, _iteration| async {
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
