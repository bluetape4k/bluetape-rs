//! Rust backend primitives for the bluetape ecosystem.
//!
//! This root crate is a small convenience facade over the foundation crates
//! shipped in the first release. Use the focused crates directly when a smaller
//! dependency surface is preferred.

#[cfg(feature = "core")]
pub use bluetape_rs_core as core;
#[cfg(feature = "logging")]
pub use bluetape_rs_logging as logging;
