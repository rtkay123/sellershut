mod api;
mod routes;
mod state;

use api::ApiSchemaBuilder;
use routes::router;
use state::ApiState;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state = ApiState::initialise().await?;
    state.index_categories().await;

    let env = state.0.config.env;

    // derive clone ok
    let schema = ApiSchemaBuilder::build(state.clone());

    let listener = TcpListener::bind(&state.0.config.listen_address).await?;

    let router = router(schema, env);

    info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, router).await?;

    Ok(())
}

#[cfg(test)]
mod tests;
