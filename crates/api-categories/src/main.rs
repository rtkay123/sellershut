mod api;
mod routes;
mod state;

use api::ApiSchemaBuilder;
use routes::router;
use sellershut_core::categories::{
    mutate_categories_server::MutateCategoriesServer,
    query_categories_server::QueryCategoriesServer,
};
use state::{multiplex::GrpcMultiplexLayer, ApiState};
use tonic::transport::Server;
use tower::ServiceExt;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state = ApiState::initialise().await?;

    let schema = ApiSchemaBuilder::build(state.clone());

    let addr = state.0.config.listen_address;

    let web = router(schema, state.0.config.env)
        .into_service()
        .map_response(|r| r.map(tonic::body::boxed));

    let grpc = Server::builder()
        .layer(GrpcMultiplexLayer::new(web))
        .add_service(QueryCategoriesServer::new(state.clone()))
        .add_service(MutateCategoriesServer::new(state));

    info!(addr = ?addr, "listening on");
    grpc.serve(addr).await?;

    Ok(())
}
