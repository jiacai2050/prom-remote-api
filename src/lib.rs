//! Prometheus [remote storage](https://prometheus.io/docs/prometheus/latest/storage/#remote-storage-integrations) API
//!
//! This crate provides:
//! - Rust-binding to [prometheus remote storage protocol buffer definitions](https://github.com/prometheus/prometheus/blob/main/prompb/remote.proto).
//!
//!   Any third-party storage can integrate with Prometheus by implementing [RemoteStorage](crate::types::RemoteStorage) trait.
//! - Various web framework utils to serve the remote wire protocols, which are controlled by corresponding feature-gates.
//!   - [Warp](https://github.com/seanmonstar/warp)
//!   - [Actix](https://actix.rs/)
//!   - More web framework will be added
//!
//!
//! See [warp-demo.rs](https://github.com/jiacai2050/prom-remote-api/blob/main/examples/warp-demo.rs), [actix-demo.rs](https://github.com/jiacai2050/prom-remote-api/blob/main/examples/actix-demo.rs) to learn how to build a remote storage.
//!

pub mod types;
mod util;
pub mod web;
