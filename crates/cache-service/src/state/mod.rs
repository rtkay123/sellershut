use std::path::Path;

use anyhow::anyhow;
use async_nats::jetstream::{self, stream};
use core_services::state::{config::env_var, ServiceState};
use futures_util::{
    future::{join_all, try_join_all},
    StreamExt, TryFutureExt,
};

#[derive(Clone)]
pub struct ApiState(pub ServiceState);

impl ApiState {
    pub async fn initialise() -> anyhow::Result<Self> {
        let man_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
        dotenvy::from_path(man_path).ok();

        let state = ServiceState::initialise(env!("CARGO_CRATE_NAME")).await?;

        Ok(Self(state))
    }
}

async fn handle_message(
    consumer: jetstream::consumer::Consumer<jetstream::consumer::pull::Config>,
) -> anyhow::Result<()> {
    // Get messages
    let mut messages = consumer.messages().await?;

    while let Some(Ok(message)) = messages.next().await {
        if let Err(e) = message.ack().await {
            eprintln!("{e}");
        }
    }

    Ok(())
}
