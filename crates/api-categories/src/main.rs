use api_categories::state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state = state::ApiState::initialise().await?;
    let (tx, _rx) = tokio::sync::oneshot::channel();

    api_categories::run(state, tx).await
}
