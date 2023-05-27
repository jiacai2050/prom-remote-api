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
  - `actix`
  - more web framework will be added

Any third-party storage can integrate with Prometheus by implementing `RemoteStorage` trait.

```rust
pub trait RemoteStorage: Sync {
    type Err: Send;
    type Context: Send + Sync;

    /// Write samples to remote storage.
    async fn write(&self, ctx: Self::Context, req: WriteRequest) -> Result<(), Self::Err>;

    /// Process one query within [ReadRequest](crate::types::ReadRequest).
    ///
    /// Note: Prometheus remote protocol sends multiple queries by default,
    /// use [read](crate::types::RemoteStorage::read) to serve ReadRequest.
    async fn process_query(
        &self,
        ctx: &Self::Context,
        q: Query,
    ) -> Result<QueryResult, Self::Err>;

    /// Read samples from remote storage.
    ///
    /// [ReadRequest](crate::types::ReadRequest) may contain more than one sub [queries](crate::types::Query).
    async fn read(
        &self,
        ctx: Self::Context,
        req: ReadRequest,
    ) -> Result<ReadResponse, Self::Err> {
        let results = futures::future::join_all(
            req.queries
                .into_iter()
                .map(|q| async { self.process_query(&ctx, q).await }),
        )
        .await
        .into_iter()
        .collect::<Result<Vec<_>, Self::Err>>()?;

        Ok(ReadResponse { results })
    }
}
```

See [warp-demo.rs](examples/warp-demo.rs), [actix-demo.rs](examples/actix-demo.rs) to learn how to build a remote storage.
