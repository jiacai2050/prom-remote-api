//! Prometheus should contains following configs to test against this remote storage
//!
//! ```yml
//! remote_write:
//!   - url: "http://localhost:9201/api/write"
//!
//! remote_read:
//!   - url: "http://localhost:9201/api/read"
//! ```

use async_trait::async_trait;
use std::sync::Arc;

use prom_remote_api::{
    types::{
        Label, QueryResult, ReadRequest, ReadResponse, RemoteStorage, RemoteStorageRef, Result,
        Sample, TimeSeries, WriteRequest,
    },
    web,
};
use warp::Filter;

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
        .into_iter()
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
    async fn write(&self, req: WriteRequest) -> Result<()> {
        println!("mock write, req:{req:?}");
        Ok(())
    }

    async fn read(&self, req: ReadRequest) -> Result<ReadResponse> {
        println!("mock read, req:{req:?}");
        let query = &req.queries[0];

        Ok(ReadResponse {
            results: vec![QueryResult {
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
            }],
        })
    }
}

#[tokio::main]
async fn main() {
    let storage = Arc::new(MockStorage) as RemoteStorageRef;
    let write_api = warp::path!("write")
        .and(web::warp::with_remote_storage(storage.clone()))
        .and(web::warp::protobuf_body())
        .and_then(web::warp::write);
    let query_api = warp::path!("read")
        .and(web::warp::with_remote_storage(storage))
        .and(web::warp::protobuf_body())
        .and_then(web::warp::read);

    let routes = warp::path("api").and(write_api.or(query_api));

    let port = 9201;
    println!("Listen on {port}...");

    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}
