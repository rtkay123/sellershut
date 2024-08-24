mod state;

use axum::{routing::get, Router};
use sellershut_core::categories::{
    mutate_categories_server::MutateCategoriesServer,
    query_categories_server::QueryCategoriesServer,
};
use state::{multiplex::GrpcMultiplexLayer, ApiState};
use tonic::transport::Server;
use tower::ServiceExt;

#[tokio::main]
async fn main() {
    let web = Router::new()
        .route("/test", get(|| async { "Hello, World!" }))
        .into_service()
        .map_response(|r| r.map(tonic::body::boxed));

    let service = ApiState::initialise().await.unwrap();

    let grpc = Server::builder()
        .layer(GrpcMultiplexLayer::new(web))
        .add_service(QueryCategoriesServer::new(service.clone()))
        .add_service(MutateCategoriesServer::new(service));

    let addr = "[::1]:50051".parse().unwrap();

    grpc.serve(addr).await.unwrap();
}
