//! This module provides Rust-binding to [prometheus remote storage protocol buffer definitions][proto].
//!
//! Go to [docs.rs][docs] to see those binding types.
//!
//! [proto]: https://github.com/prometheus/prometheus/blob/main/prompb/remote.proto
//! [docs]: https://docs.rs/prom-remote-api/latest/prom_remote_api/types/index.html

use async_trait::async_trait;
use std::{fmt::Display, result::Result as StdResult, sync::Arc};

mod prometheus {
    include!(concat!(env!("OUT_DIR"), "/prometheus.rs"));
}
pub use prometheus::*;

#[derive(Debug)]
pub enum Error {
    SnappyEncode(snap::Error),
    SnappyDecode(snap::Error),
    ReadRequest(std::io::Error),
    ProtoDecode(prost::DecodeError),
}

pub type Result<T> = StdResult<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SnappyEncode(_) => f.write_str("SnappyEncode"),
            Self::SnappyDecode(_) => f.write_str("SnappyDecode"),
            Self::ReadRequest(_) => f.write_str("ReadRequest"),
            Self::ProtoDecode(_) => f.write_str("ProtoDecode"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::SnappyEncode(e) => Some(e),
            Self::SnappyDecode(e) => Some(e),
            Self::ReadRequest(e) => Some(e),
            Self::ProtoDecode(e) => Some(e),
        }
    }
}

/// Remote storage is Prometheus's solution for long-term storage.
///
/// Third-party storage can be integrated with Prometheus by implement this trait.
/// <https://prometheus.io/docs/prometheus/latest/storage/#remote-storage-integrations>
#[async_trait]
pub trait RemoteStorage: Sync {
    /// The type of failures yielded when write and read.
    type Err: Send;

    /// The type of request-scoped values provided for write and read.
    type Context: Send + Sync;

    /// Write samples to remote storage.
    async fn write(&self, ctx: Self::Context, req: WriteRequest) -> StdResult<(), Self::Err>;

    /// Process one query within [ReadRequest](crate::types::ReadRequest).
    ///
    /// Note: Prometheus remote protocol sends multiple queries by default,
    /// use [read](crate::types::RemoteStorage::read) to serve ReadRequest.
    async fn process_query(
        &self,
        ctx: &Self::Context,
        q: Query,
    ) -> StdResult<QueryResult, Self::Err>;

    /// Read samples from remote storage.
    ///
    /// [ReadRequest](crate::types::ReadRequest) may contain more than one sub [queries](crate::types::Query).
    async fn read(
        &self,
        ctx: Self::Context,
        req: ReadRequest,
    ) -> StdResult<ReadResponse, Self::Err> {
        let results = futures::future::join_all(
            req.queries
                .into_iter()
                .map(|q| async { self.process_query(&ctx, q).await }),
        )
        .await
        .into_iter()
        .collect::<StdResult<Vec<_>, Self::Err>>()?;

        Ok(ReadResponse { results })
    }
}

pub type RemoteStorageRef<C, E> = Arc<dyn RemoteStorage<Err = E, Context = C> + Send + Sync>;
