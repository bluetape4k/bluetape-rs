use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use bluetape_rs_async::{
    AsyncControlError, TaskGroupError, shutdown_signal, try_map_bounded, with_timeout,
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

    let actual = try_map_bounded(0..64, 4, {
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
                yield_now().await;
                current.fetch_sub(1, Ordering::SeqCst);
                completed.fetch_add(1, Ordering::SeqCst);
                Ok::<_, &'static str>(value)
            }
        }
    })
    .await
    .unwrap();

    assert_eq!(actual, (0..64).collect::<Vec<_>>());
    assert_eq!(completed.load(Ordering::SeqCst), 64);
    assert!(peak.load(Ordering::SeqCst) <= 4);
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
    assert!(started.load(Ordering::SeqCst) >= 4);
    assert!(dropped.load(Ordering::SeqCst) >= 3);
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
