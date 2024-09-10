pub mod api;
pub mod routes;
pub mod state;

use api::ApiSchemaBuilder;
use axum::{extract::Request, http::header::CONTENT_TYPE};
use futures_util::TryFutureExt;
use routes::router;
use sellershut_core::categories::{
    mutate_categories_server::MutateCategoriesServer,
    query_categories_server::QueryCategoriesServer, CATEGORY_FILE_DESCRIPTOR_SET,
};
use sentry::integrations::tower::{NewSentryLayer, SentryHttpLayer};
use state::ApiState;
use tokio::sync::oneshot;
use tonic::service::Routes;
use tower::{make::Shared, steer::Steer};
use tracing::{error, info};

pub async fn run(state: ApiState, tx: oneshot::Sender<u16>) -> anyhow::Result<()> {
    let schema = ApiSchemaBuilder::build(state.clone());

    let addr = state.state.config.listen_address;

    let web = router(schema, state.state.config.env)
        .layer(NewSentryLayer::new_from_top())
        .layer(SentryHttpLayer::with_transaction());

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(CATEGORY_FILE_DESCRIPTOR_SET)
        .build_v1()?;

    let grpc = Routes::new(reflection_service)
        .add_service(QueryCategoriesServer::new(state.clone()))
        .add_service(MutateCategoriesServer::new(state.clone()));
    let grpc = grpc
        .into_axum_router()
        .layer(NewSentryLayer::new_from_top())
        .layer(SentryHttpLayer::with_transaction());

    let service = Steer::new(vec![web, grpc], |req: &Request, _services: &[_]| {
        if req
            .headers()
            .get(CONTENT_TYPE)
            .map(|content_type| content_type.as_bytes())
            .filter(|content_type| content_type.starts_with(b"application/grpc"))
            .is_some()
        {
            1
        } else {
            0
        }
    });

    let listener = tokio::net::TcpListener::bind(addr)
        .map_err(anyhow::Error::new)
        .await?;

    let socket_addr = listener
        .local_addr()
        .expect("should get socket_addr from listener");
    if let Err(e) = tx.send(socket_addr.port()) {
        error!("{e}");
    }
    info!(addr = ?socket_addr, "listening");

    axum::serve(listener, Shared::new(service)).await?;

    Ok(())
}
