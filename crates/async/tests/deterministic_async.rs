use std::future::pending;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use bluetape_rs_async::{
    AsyncControlError, CancellationSource, TaskGroupError, map_bounded_collect,
    run_until_cancelled, shutdown_signal, try_map_bounded, with_timeout, with_timeout_or_cancel,
};
use bluetape_rs_test::{consistently, eventually};
use tokio::task::yield_now;
use tokio::time::sleep;

struct DropCounter {
    counter: Arc<AtomicUsize>,
}

impl Drop for DropCounter {
    fn drop(&mut self) {
        self.counter.fetch_add(1, Ordering::SeqCst);
    }
}

#[tokio::test]
async fn bounded_runner_stress_stays_under_limit_and_completes_all_items() {
    let current = Arc::new(AtomicUsize::new(0));
    let peak = Arc::new(AtomicUsize::new(0));
    let completed = Arc::new(AtomicUsize::new(0));

    let actual = with_timeout(
        Duration::from_secs(1),
        try_map_bounded(0..64, 4, {
            let current = Arc::clone(&current);
            let peak = Arc::clone(&peak);
            let completed = Arc::clone(&completed);
            move |value| {
                let current = Arc::clone(&current);
                let peak = Arc::clone(&peak);
                let completed = Arc::clone(&completed);
                async move {
                    let active = current.fetch_add(1, Ordering::SeqCst) + 1;
                    peak.fetch_max(active, Ordering::SeqCst);
                    while peak.load(Ordering::SeqCst) < 4 {
                        yield_now().await;
                    }
                    current.fetch_sub(1, Ordering::SeqCst);
                    completed.fetch_add(1, Ordering::SeqCst);
                    Ok::<_, &'static str>(value)
                }
            }
        }),
    )
    .await
    .unwrap()
    .unwrap();

    assert_eq!(actual, (0..64).collect::<Vec<_>>());
    assert_eq!(completed.load(Ordering::SeqCst), 64);
    assert_eq!(peak.load(Ordering::SeqCst), 4);
}

#[tokio::test]
async fn collect_runner_stress_records_every_result_under_limit() {
    let current = Arc::new(AtomicUsize::new(0));
    let peak = Arc::new(AtomicUsize::new(0));
    let completed = Arc::new(AtomicUsize::new(0));

    let report = with_timeout(
        Duration::from_secs(1),
        map_bounded_collect(0..96, 6, {
            let current = Arc::clone(&current);
            let peak = Arc::clone(&peak);
            let completed = Arc::clone(&completed);
            move |value| {
                let current = Arc::clone(&current);
                let peak = Arc::clone(&peak);
                let completed = Arc::clone(&completed);
                async move {
                    let active = current.fetch_add(1, Ordering::SeqCst) + 1;
                    peak.fetch_max(active, Ordering::SeqCst);
                    while peak.load(Ordering::SeqCst) < 6 {
                        yield_now().await;
                    }
                    current.fetch_sub(1, Ordering::SeqCst);
                    completed.fetch_add(1, Ordering::SeqCst);

                    if value % 5 == 0 {
                        Err(value)
                    } else {
                        Ok(value)
                    }
                }
            }
        }),
    )
    .await
    .unwrap()
    .unwrap();

    assert_eq!(report.len(), 96);
    assert_eq!(report.successes.len(), 76);
    assert_eq!(report.failures.len(), 20);
    assert_eq!(completed.load(Ordering::SeqCst), 96);
    assert_eq!(peak.load(Ordering::SeqCst), 6);
}

#[tokio::test]
async fn first_error_aborts_and_drops_started_sibling_futures() {
    let started = Arc::new(AtomicUsize::new(0));
    let dropped = Arc::new(AtomicUsize::new(0));

    let actual = try_map_bounded(0..8, 8, {
        let started = Arc::clone(&started);
        let dropped = Arc::clone(&dropped);
        move |value| {
            let started = Arc::clone(&started);
            let dropped = Arc::clone(&dropped);
            async move {
                started.fetch_add(1, Ordering::SeqCst);
                if value == 0 {
                    while started.load(Ordering::SeqCst) < 4 {
                        yield_now().await;
                    }
                    return Err("boom");
                }

                let _guard = DropCounter { counter: dropped };
                pending::<()>().await;
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
    assert!(started.load(Ordering::SeqCst) >= 4);
    assert!(dropped.load(Ordering::SeqCst) >= 3);
}

#[tokio::test]
async fn run_until_cancelled_stress_drops_started_futures_when_source_cancels() {
    let (source, token) = CancellationSource::new();
    let started = Arc::new(AtomicUsize::new(0));
    let dropped = Arc::new(AtomicUsize::new(0));

    let task = tokio::spawn({
        let started = Arc::clone(&started);
        let dropped = Arc::clone(&dropped);
        async move {
            run_until_cancelled(token, async move {
                let guards = (0..32)
                    .map(|_| {
                        started.fetch_add(1, Ordering::SeqCst);
                        DropCounter {
                            counter: Arc::clone(&dropped),
                        }
                    })
                    .collect::<Vec<_>>();
                assert_eq!(guards.len(), 32);
                pending::<()>().await;
            })
            .await
        }
    });

    while started.load(Ordering::SeqCst) < 32 {
        yield_now().await;
    }
    source.cancel();
    let actual = task.await.unwrap();

    assert_eq!(actual, Err(AsyncControlError::Cancelled));
    assert_eq!(dropped.load(Ordering::SeqCst), 32);
}

#[tokio::test]
async fn with_timeout_or_cancel_prefers_pre_cancelled_token_when_future_is_ready() {
    let (source, token) = CancellationSource::new();
    source.cancel();

    let actual = with_timeout_or_cancel(Duration::from_secs(1), token, async { 42 }).await;

    assert_eq!(actual, Err(AsyncControlError::Cancelled));
}

#[tokio::test]
async fn cancellation_token_completes_when_source_is_dropped() {
    let (source, mut token) = CancellationSource::new();

    drop(source);
    token.cancelled().await;

    assert!(!token.is_cancelled());
}

#[tokio::test(start_paused = true)]
async fn timeout_coverage_uses_paused_tokio_time() {
    let actual = with_timeout(Duration::from_millis(10), async {
        sleep(Duration::from_secs(1)).await;
        42
    })
    .await;

    assert_eq!(actual, Err(AsyncControlError::TimedOut));
}

#[tokio::test]
async fn shutdown_signal_notifies_and_stays_observed() {
    let (trigger, mut signal) = shutdown_signal();
    let observed = Arc::new(AtomicUsize::new(0));

    let listener = tokio::spawn({
        let observed = Arc::clone(&observed);
        async move {
            signal.wait().await;
            observed.fetch_add(1, Ordering::SeqCst);
        }
    });

    trigger.shutdown();

    eventually(Duration::from_secs(1), Duration::from_millis(5), {
        let observed = Arc::clone(&observed);
        move || {
            let observed = Arc::clone(&observed);
            async move {
                if observed.load(Ordering::SeqCst) == 1 {
                    Ok(())
                } else {
                    Err("shutdown was not observed")
                }
            }
        }
    })
    .await
    .unwrap();

    consistently(Duration::from_millis(20), Duration::from_millis(5), {
        let observed = Arc::clone(&observed);
        move || {
            let observed = Arc::clone(&observed);
            async move {
                if observed.load(Ordering::SeqCst) == 1 {
                    Ok(())
                } else {
                    Err("shutdown observation changed")
                }
            }
        }
    })
    .await
    .unwrap();

    listener.await.unwrap();
}
