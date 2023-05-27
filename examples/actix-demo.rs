//! To configure Prometheus to send samples to this binary, add the following to your prometheus.yml:
//!
//! ```yml
//! remote_write:
//!   - url: "http://localhost:9201/api/write"
//!
//! remote_read:
//!   - url: "http://localhost:9201/api/read"
//! ```

use async_trait::async_trait;
use std::{
    future::{ready, Ready},
    sync::Arc,
};

use actix_web::{web, App, FromRequest, HttpServer};
use prom_remote_api::{
    types::{
        Error, Label, Query, QueryResult, RemoteStorage, RemoteStorageRef, Result, Sample,
        TimeSeries, WriteRequest,
    },
    web::actix::{read, write},
};

#[derive(Clone, Copy)]
struct MockStorage;

fn generate_samples(start_ms: i64, end_ms: i64, step_ms: i64) -> Vec<Sample> {
    // instant query
    if step_ms == 0 {
        return vec![Sample {
            value: 1.0,
            timestamp: start_ms,
        }];
    }

    // range query
    (start_ms..end_ms)
        .step_by(step_ms as usize)
        .enumerate()
        .map(|(i, timestamp)| Sample {
            value: 1.0 + i as f64,
            timestamp,
        })
        .collect()
}

#[async_trait]
impl RemoteStorage for MockStorage {
    type Err = Error;
    type Context = RequestContext;

    async fn write(&self, ctx: Self::Context, req: WriteRequest) -> Result<()> {
        let user = ctx.user;
        println!("mock write, user:{user}, req:{req:?}");
        Ok(())
    }

    async fn process_query(&self, ctx: &Self::Context, query: Query) -> Result<QueryResult> {
        let user = &ctx.user;
        println!("mock read, user:{user}, req:{query:?}");

        Ok(QueryResult {
            timeseries: vec![TimeSeries {
                labels: vec![
                    Label {
                        name: "job".to_string(),
                        value: "mock-remote".to_string(),
                    },
                    Label {
                        name: "instance".to_string(),
                        value: "127.0.0.1:9201".to_string(),
                    },
                    Label {
                        name: "__name__".to_string(),
                        value: "up".to_string(),
                    },
                ],
                samples: generate_samples(
                    query.start_timestamp_ms,
                    query.end_timestamp_ms,
                    query
                        .hints
                        .as_ref()
                        .map(|hint| hint.step_ms)
                        .unwrap_or(1000),
                ),
                ..Default::default()
            }],
        })
    }
}

struct RequestContext {
    user: String,
}

impl FromRequest for RequestContext {
    type Error = Error;

    type Future = Ready<Result<Self>>;

    fn from_request(
        _req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        ready(Ok(RequestContext {
            user: "foo".to_string(),
        }))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 9201;
    println!("Listen on {port}...");

    let storage: RemoteStorageRef<_, _> = Arc::new(MockStorage);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(storage.clone()))
            .route("/api/read", web::post().to(read::<RequestContext, Error>))
            .route("/api/write", web::post().to(write::<RequestContext, Error>))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
