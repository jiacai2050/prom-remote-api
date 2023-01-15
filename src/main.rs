use std::sync::Arc;

mod types;
mod util;
mod warp_adapter;

use types::RemoteStorage;
use types::RemoteStorageRef;
use types::Result;
use warp::Filter;

#[derive(Clone, Copy)]
struct Mock;

impl RemoteStorage for Mock {
    fn write(&self, req: types::WriteRequest) -> Result<()> {
        println!("mock write, req:{req:?}");
        Ok(())
    }

    fn read(&self, req: types::ReadRequest) -> Result<types::ReadResponse> {
        println!("mock read, req:{req:?}");

        Ok(types::ReadResponse {
            results: vec![types::QueryResult::default()],
        })
    }
}

#[tokio::main]
async fn main() {
    let storage = Arc::new(Mock) as RemoteStorageRef;
    let write_api = warp::path!("write")
        .and(warp_adapter::with_remote_storage(storage.clone()))
        .and(warp_adapter::protobuf_body())
        .and_then(warp_adapter::write);
    let query_api = warp::path!("read")
        .and(warp_adapter::with_remote_storage(storage))
        .and(warp_adapter::protobuf_body())
        .and_then(warp_adapter::read);

    let routes = warp::path("api").and(write_api.or(query_api));

    let port = 9201;
    println!("Listen on {port}...");

    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}
