use std::sync::Arc;

use api::{http, pub_sub};
use state::ApiState;

pub mod api;
pub mod state;

#[cfg(feature = "nlp")]
pub mod nlp;

pub async fn serve(config: core_services::state::config::Configuration) -> anyhow::Result<()> {
    let state = Arc::new(ApiState::initialise(config).await?);

    let pub_sub_task = pub_sub::serve(Arc::clone(&state));
    let http_task = http::serve(state);

    let (_, _) = tokio::join!(pub_sub_task, http_task);

    Ok(())
}
