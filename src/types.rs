use async_trait::async_trait;
use std::{fmt::Display, sync::Arc};

mod prometheus {
    include!(concat!(env!("OUT_DIR"), "/prometheus.rs"));
}
pub use prometheus::*;

#[derive(Debug)]
pub enum Error {
    SnappyEncode(snap::Error),
    SnappyDecode(snap::Error),
    ProtoDecode(prost::DecodeError),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SnappyEncode(_) => f.write_str("SnappyEncode"),
            Self::SnappyDecode(_) => f.write_str("SnappyDecode"),
            Self::ProtoDecode(_) => f.write_str("ProtoDecode"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::SnappyEncode(e) => Some(e),
            Self::SnappyDecode(e) => Some(e),
            Self::ProtoDecode(e) => Some(e),
        }
    }
}

/// Remote storage is Prometheus's solution for long-term storage.
/// Third-party storage can be integrated with Prometheus by implement this trait.
/// <https://prometheus.io/docs/prometheus/latest/storage/#remote-storage-integrations>
#[async_trait]
pub trait RemoteStorage {
    /// Write samples to remote storage
    async fn write(&self, req: WriteRequest) -> Result<()>;

    /// Read samples from remote storage
    async fn read(&self, req: ReadRequest) -> Result<ReadResponse>;
}

pub type RemoteStorageRef = Arc<dyn RemoteStorage + Send + Sync>;
