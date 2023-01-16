# prom-remote-api

[![Crates.io](https://img.shields.io/crates/v/prom-remote-api.svg)](https://crates.io/crates/prom-remote-api)
[![docs.rs](https://img.shields.io/docsrs/prom-remote-api/latest)](https://docs.rs/prom-remote-api)
[![](https://github.com/jiacai2050/prom-remote-api/actions/workflows/ci.yml/badge.svg)](https://github.com/jiacai2050/prom-remote-api/actions/workflows/ci.yml)


Prometheus [remote storage](https://prometheus.io/docs/prometheus/latest/storage/#remote-storage-integrations) API for Rust.

## Usage

There are two interfaces in Prometheus remote storage API:
1. write
2. read

Both interfaces use a snappy-compressed protocol buffer encoding over HTTP.

This crate use [prost-build](https://github.com/tokio-rs/prost/tree/master/prost-build) to convert [remote storage protocol buffer definitions](https://github.com/prometheus/prometheus/blob/main/prompb/remote.proto) to Rust code, and expose a `RemoteStorage` trait for any third-party storage to integrate with Prometheus.

```rust
#[async_trait]
pub trait RemoteStorage {
    /// Write samples to remote storage
    async fn write(&self, req: WriteRequest) -> Result<()>;

    /// Read samples from remote storage
    async fn read(&self, req: ReadRequest) -> Result<ReadResponse>;
}
```

See [simple.rs](examples/simple.rs) to learn how to build a remote storage with [warp](https://github.com/seanmonstar/warp) web framework.

In future, more web framework will be supported.
