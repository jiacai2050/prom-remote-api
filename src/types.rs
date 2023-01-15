mod prometheus {
    include!(concat!(env!("OUT_DIR"), "/prometheus.rs"));
}

use std::{fmt::Display, sync::Arc};

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

pub trait RemoteStorage {
    fn write(&self, req: WriteRequest) -> Result<()>;
    fn read(&self, req: ReadRequest) -> Result<ReadResponse>;
}

pub type RemoteStorageRef = Arc<dyn RemoteStorage + Send + Sync>;
