mod state;

use core_services::state::ServiceState;
use state::ApiState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    ApiState::initialise().await?;
    tracing::info!("Hello, world!");

    Ok(())
}
