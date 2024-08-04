use state::ApiState;

mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _state = ApiState::initialise().await?;

    Ok(())
}
