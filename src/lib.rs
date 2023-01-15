//! Prometheus remote storage API
//!
//! This crate provides data structures to implement [prometheus remote storage](https://prometheus.io/docs/prometheus/latest/storage/#remote-storage-integrations).
//!
//! The main trait is [RemoteStorage](crate::types::RemoteStorage), which encapsulates write/read capability.
//!
//! See [this file](https://github.com/jiacai2050/prom-remote-api/blob/main/examples/simple.rs) for basic usage.

pub mod types;
mod util;
#[cfg(feature = "warp")]
pub mod warp_adapter;
