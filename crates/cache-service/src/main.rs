mod state;

use anyhow::{anyhow, Result};
use async_nats::jetstream::{consumer, stream};
use core_services::state::config::env_var;
use futures_util::{
    future::{join_all, try_join_all},
    StreamExt, TryFutureExt,
};
use state::ApiState;

#[tokio::main]
async fn main() -> Result<()> {
    let state = ApiState::initialise().await?;

    let js = state.0.jetstream_context.clone();

    let services: Vec<_> = ["CATEGORIES"]
        .iter()
        .map(|service| {
            let create_var = |s: &str| format!("{service}_{s}");
            let stream = env_var(&create_var("STREAM_NAME"));
            let subject = env_var(&create_var("STREAM_SUBJECTS"));
            let stream_max_bytes = env_var(&create_var("STREAM_MAX_BYTES"));
            let consumer = format!("CONSUMER_{service}");

            js.get_or_create_stream(stream::Config {
                name: stream.to_string(),
                subjects: vec![format!("{subject}")],
                max_messages: 10_000,
                max_bytes: stream_max_bytes.parse().unwrap(),
                ..Default::default()
            })
            .map_err(|e| anyhow!(e.to_string()))
            .and_then(|stream| async move {
                stream
                    .create_consumer(consumer::pull::Config {
                        durable_name: Some(consumer.clone().into()),
                        name: Some(consumer.into()),
                        ..Default::default()
                    })
                    .await
                    .map_err(|e| anyhow!(e.to_string()))
            })
        })
        .collect();

    let consumers = try_join_all(services).await?.into_iter().map(|consumer| {
        let state = state.clone();
        tokio::spawn(handle_message(consumer, state))
    });

    tokio::spawn(join_all(consumers)).await;

    Ok(())
}

async fn handle_message(
    consumer: consumer::Consumer<consumer::pull::Config>,
    state: ApiState,
) -> anyhow::Result<()> {
    // Get messages
    let mut messages = consumer.messages().await?;

    while let Some(Ok(message)) = messages.next().await {
        println!("Got message {:?}", message);
        if let Err(e) = message.ack().await {
            eprintln!("{e}");
        }
    }

    Ok(())
}
