use std::convert::Infallible;

use prost::Message;
use warp::{
    body,
    http::HeaderValue,
    hyper::StatusCode,
    reject::{self, Reject},
    Buf, Filter, Rejection, Reply,
};

use crate::{
    types::{Error, ReadRequest, ReadResponse, RemoteStorageRef, WriteRequest},
    util,
};

pub async fn write(storage: RemoteStorageRef, req: WriteRequest) -> Result<impl Reply, Rejection> {
    storage
        .write(req)
        .map_err(|err| reject::custom(err))
        .map(|_| warp::reply::reply())
}

pub async fn read(storage: RemoteStorageRef, req: ReadRequest) -> Result<impl Reply, Rejection> {
    storage.read(req).map_err(|err| reject::custom(err))
}

pub fn with_remote_storage(
    storage: RemoteStorageRef,
) -> impl Filter<Extract = (RemoteStorageRef,), Error = Infallible> + Clone {
    warp::any().map(move || storage.clone())
}

impl Reject for Error {}

// https://github.com/ParkMyCar/warp-protobuf/blob/master/src/lib.rs#L102
pub fn protobuf_body<T: Message + Send + Default>(
) -> impl Filter<Extract = (T,), Error = Rejection> + Copy {
    async fn from_reader<T: Message + Send + Default>(buf: impl Buf) -> Result<T, Rejection> {
        util::decode_snappy(buf.chunk())
            .map_err(|err| reject::custom(err))
            .and_then(|decoded_buf| {
                T::decode(decoded_buf.as_slice())
                    .map_err(|err| reject::custom(Error::ProtoDecode(err.into())))
            })
    }

    body::aggregate().and_then(from_reader)
}

impl warp::Reply for ReadResponse {
    fn into_response(self) -> warp::reply::Response {
        let bytes = match util::encode_snappy(self.encode_to_vec().as_slice()) {
            Ok(v) => v,
            Err(e) => {
                return warp::reply::with_status(
                    e.to_string().into_response(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
                .into_response()
            }
        };

        let mut ret = warp::http::Response::new(bytes.into());
        let headers = ret.headers_mut();
        headers.insert(
            warp::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/x-protobuf"),
        );
        headers.insert(
            warp::http::header::CONTENT_ENCODING,
            HeaderValue::from_static("snappy"),
        );

        ret
    }
}
