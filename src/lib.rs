//! Prometheus remote storage API
//!
//! This crate provides data structures to implement [prometheus remote storage](https://prometheus.io/docs/prometheus/latest/storage/#remote-storage-integrations).
//!
//! The main trait is [RemoteStorage](crate::types::RemoteStorage), which encapsulates write/read capability.
//! A simple usage can be found [here]()

pub mod types;
mod util;
#[cfg(feature = "warp")]
pub mod warp_adapter;
