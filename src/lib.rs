//! Prometheus [remote storage](https://prometheus.io/docs/prometheus/latest/storage/#remote-storage-integrations) API
//!
//! This crate provides:
//! - Rust-binding to [prometheus remote storage protocol buffer definitions](https://github.com/prometheus/prometheus/blob/main/prompb/remote.proto), and
//! - Various web framework utils to serve the remote wire protocols, which are controlled by corresponding feature-gates. Available features:
//!   - `warp`
//!   - more web framework will be added
//!
//! Any third-party storage can integrate with Prometheus by implementing [RemoteStorage](crate::types::RemoteStorage) trait.
//!
//! See [simple.rs](https://github.com/jiacai2050/prom-remote-api/blob/main/examples/simple.rs) to learn how to build a remote storage with [warp](https://github.com/seanmonstar/warp) web framework.
//!
//! In future, more web framework will be supported.

pub mod types;
#[cfg(feature = "warp")]
mod util;
pub mod web;
