//! Remote storage adapter for warp web framework

use std::{convert::Infallible, io::Read};

use prost::Message;
use warp::{
    body,
    http::HeaderValue,
    hyper::{
        header::{CONTENT_ENCODING, CONTENT_TYPE},
        StatusCode,
    },
    reject::{self, Reject},
    reply, Buf, Filter, Rejection, Reply,
};

use crate::{
    types::{Error, ReadRequest, ReadResponse, RemoteStorageRef, WriteRequest},
    util,
};

/// Warp handler for remote write request
pub async fn write<C: Send + Sync, Err: Reject + Send>(
    storage: RemoteStorageRef<C, Err>,
    ctx: C,
    req: WriteRequest,
) -> Result<impl Reply, Rejection> {
    storage
        .write(ctx, req)
        .await
        .map(|_| reply::reply())
        .map_err(reject::custom)
}

/// Warp handler for remote read request
pub async fn read<C: Send + Sync, Err: Reject + Send>(
    storage: RemoteStorageRef<C, Err>,
    ctx: C,
    req: ReadRequest,
) -> Result<impl Reply, Rejection> {
    storage.read(ctx, req).await.map_err(reject::custom)
}

/// Create a `Filter` that matches any requests and return a `RemoteStorageRef`,
/// which can be used in `and_then`.
pub fn with_remote_storage<C, Err>(
    storage: RemoteStorageRef<C, Err>,
) -> impl Filter<Extract = (RemoteStorageRef<C, Err>,), Error = Infallible> + Clone {
    warp::any().map(move || storage.clone())
}

// Make our `Error` `Reject`able
impl Reject for Error {}

/// Returns a `Filter` that matches any request and extracts a `Future` of a
/// protobuf-decode body
///
/// # Warning
///
/// This does not have a default size limit, it would be wise to use one to
/// prevent a overly large request from using too much memory.

// https://github.com/ParkMyCar/warp-protobuf/blob/master/src/lib.rs#L102
pub fn protobuf_body<T: Message + Send + Default>(
) -> impl Filter<Extract = (T,), Error = Rejection> + Copy {
    async fn from_reader<T: Message + Send + Default>(buf: impl Buf) -> Result<T, Rejection> {
        let mut body = Vec::new();
        buf.reader()
            .read_to_end(&mut body)
            .map_err(Error::ReadRequest)?;

        util::decode_snappy(&body)
            .map_err(reject::custom)
            .and_then(|decoded_buf| {
                T::decode(decoded_buf.as_slice())
                    .map_err(|err| reject::custom(Error::ProtoDecode(err)))
            })
    }

    body::aggregate().and_then(from_reader)
}

impl warp::Reply for ReadResponse {
    fn into_response(self) -> reply::Response {
        let bytes = match util::encode_snappy(self.encode_to_vec().as_slice()) {
            Ok(v) => v,
            Err(e) => {
                return reply::with_status(
                    e.to_string().into_response(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
                .into_response()
            }
        };

        let mut ret = warp::http::Response::new(bytes.into());
        let headers = ret.headers_mut();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-protobuf"),
        );
        headers.insert(CONTENT_ENCODING, HeaderValue::from_static("snappy"));

        ret
    }
}
