# prom-remote-api

[![Crates.io](https://img.shields.io/crates/v/prom-remote-api.svg)](https://crates.io/crates/prom-remote-api)
[![docs.rs](https://img.shields.io/docsrs/prom-remote-api/latest)](https://docs.rs/prom-remote-api)
[![](https://github.com/jiacai2050/prom-remote-api/actions/workflows/ci.yml/badge.svg)](https://github.com/jiacai2050/prom-remote-api/actions/workflows/ci.yml)


Prometheus [remote storage](https://prometheus.io/docs/prometheus/latest/storage/#remote-storage-integrations) API for Rust.

## Usage

There are two interfaces in Prometheus remote storage API: write/read.

Both interfaces use a snappy-compressed protocol buffer encoding over HTTP.

This crate provides:
- Rust-binding to [prometheus remote storage protocol buffer definitions](https://github.com/prometheus/prometheus/blob/main/prompb/remote.proto)
- Various web framework utils to serve the remote wire protocols, which are controlled by corresponding feature-gates. Available features:
  - `warp`
  - more web framework will be added

Any third-party storage can integrate with Prometheus by implementing this `RemoteStorage` trait.

```rust
#[async_trait]
pub trait RemoteStorage {
    type Err;
    type Context;

    /// Write samples to remote storage
    async fn write(
        &self,
        ctx: Self::Context,
        req: WriteRequest,
    ) -> std::result::Result<(), Self::Err>;

    /// Read samples from remote storage,
    /// [ReadRequest](crate::types::ReadRequest) may contain more than one sub queries.
    async fn read(
        &self,
        ctx: Self::Context,
        req: ReadRequest,
    ) -> std::result::Result<ReadResponse, Self::Err>;
}
```

See [simple.rs](examples/simple.rs) to learn how to build a remote storage with [warp](https://github.com/seanmonstar/warp) web framework.
