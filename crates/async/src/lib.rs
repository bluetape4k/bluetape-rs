//! Tokio-first helpers for bounded async task execution.
//!
//! The helpers in this crate make task lifecycle policy explicit: callers
//! choose between first-error execution with sibling cancellation, collect-all
//! execution that records every operation result, and small cancellation or
//! timeout wrappers built directly on Tokio primitives.
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

mod control;
mod task_group;

pub use control::{
    AsyncControlError, CancellationSource, CancellationToken, ShutdownSignal, ShutdownTrigger,
    run_until_cancelled, shutdown_signal, with_deadline, with_timeout, with_timeout_or_cancel,
};
pub use task_group::{
    DEFAULT_MAX_CONCURRENCY, MAX_CONCURRENCY, TaskFailure, TaskGroupError, TaskGroupReport,
    TaskSuccess, map_bounded_collect, try_map_bounded,
};
