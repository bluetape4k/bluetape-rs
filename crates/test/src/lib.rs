//! Reusable test support helpers.
//!
//! ```
//! use bluetape_rs_test::TempDir;
//!
//! let temp = TempDir::new("bluetape-rs-test").expect("temp dir");
//! assert!(temp.path().exists());
//! temp.close().expect("cleanup");
//! ```

mod async_assert;
mod concurrent;
mod temp_dir;

pub use async_assert::{AsyncAssertError, consistently, eventually};
pub use concurrent::{
    ConcurrentAssertError, ConcurrentConfig, MultithreadingTester, SuspendedJobTester,
    run_concurrently,
};
pub use temp_dir::TempDir;

#[cfg(test)]
mod tests;
