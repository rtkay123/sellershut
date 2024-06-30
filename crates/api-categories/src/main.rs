mod routes;
mod state;

use routes::router;
use state::ApiState;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state = ApiState::initialise().await?;

    let listener = TcpListener::bind(&state.config.listen_address).await?;

    let router = router(state);

    info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, router).await?;

    Ok(())
}
