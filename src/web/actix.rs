use actix_web::{
    http::header::HeaderValue,
    http::{
        header::{CONTENT_ENCODING, CONTENT_TYPE},
        StatusCode,
    },
    web, HttpRequest, HttpResponse, Responder, ResponseError,
};
use futures::StreamExt;
use prost::Message;

use crate::types::{Error, ReadRequest, ReadResponse, RemoteStorageRef, WriteRequest};

impl ResponseError for Error {}

/// Actix-web handler for remote read request
pub async fn read<C: Send + Sync, Err: ResponseError + Send + 'static>(
    storage: web::Data<RemoteStorageRef<C, Err>>,
    ctx: C,
    body: web::Payload,
) -> Result<ReadResponse, actix_web::Error> {
    let req = decode_request::<ReadRequest>(body).await?;
    storage
        .read(ctx, req)
        .await
        .map_err(|e| actix_web::Error::from(e))
}

/// Actix-web handler for remote write request
pub async fn write<C: Send + Sync, Err: ResponseError + Send + 'static>(
    storage: web::Data<RemoteStorageRef<C, Err>>,
    ctx: C,
    body: web::Payload,
) -> Result<WriteResponse, actix_web::Error> {
    let req = decode_request::<WriteRequest>(body).await?;
    storage
        .write(ctx, req)
        .await
        .map_err(|e| actix_web::Error::from(e))
        .map(|v| v.into())
}

pub struct WriteResponse;

impl From<()> for WriteResponse {
    fn from(_v: ()) -> Self {
        WriteResponse
    }
}

async fn decode_request<T: Message + Default>(mut body: web::Payload) -> Result<T, Error> {
    let mut bytes = web::BytesMut::with_capacity(4096);
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item.unwrap());
    }

    crate::util::decode_snappy(&bytes)
        .and_then(|b| T::decode(b.as_slice()).map_err(Error::ProtoDecode))
}

impl Responder for WriteResponse {
    type Body = ();

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::with_body(StatusCode::OK, ())
    }
}

impl Responder for ReadResponse {
    type Body = Vec<u8>;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let bytes = match crate::util::encode_snappy(self.encode_to_vec().as_slice()) {
            Ok(v) => v,
            Err(e) => {
                return HttpResponse::with_body(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    e.to_string().into_bytes(),
                );
            }
        };

        let mut resp = HttpResponse::with_body(StatusCode::OK, bytes);
        let headers = resp.headers_mut();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-protobuf"),
        );
        headers.insert(CONTENT_ENCODING, HeaderValue::from_static("snappy"));

        resp
    }
}
