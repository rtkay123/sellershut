mod state;

use std::str::FromStr;

use anyhow::{anyhow, Context, Result};
use async_nats::jetstream::{consumer, stream};
use core_services::state::{config::env_var, events::Event};
use futures_util::{
    future::{join_all, try_join_all},
    StreamExt, TryFutureExt,
};
use state::ApiState;
use tracing::{debug, error, info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    let state = ApiState::initialise().await?;

    let js = state.0.jetstream_context.clone();

    let services: Vec<_> = env_var("EVENT_PUBLISHING_SERVICES")
        .split(',')
        .map(|service| {
            let service = service.to_uppercase();
            let create_var = |s: &str| format!("{service}_{s}");
            let stream = env_var(&create_var("STREAM_NAME"));
            let subjects: Vec<_> = env_var(&create_var("STREAM_SUBJECTS"))
                .split(',')
                .map(String::from)
                .collect();
            assert!(subjects.len() > 0);
            let stream_max_bytes = env_var(&create_var("STREAM_MAX_BYTES"));
            let consumer = format!("CONSUMER_{}", env!("CARGO_PKG_NAME"));
            debug!(stream = stream, subjects = ?subjects, "configuring subjects");

            js.get_or_create_stream(stream::Config {
                name: stream.to_string(),
                subjects,
                max_messages: 10_000,
                max_bytes: stream_max_bytes.parse().unwrap(),
                ..Default::default()
            })
            .map_err(|e| anyhow!(e.to_string()))
            .and_then(|stream| async move {
                debug!(consumer = consumer, "creating consumer");
                stream
                    .create_consumer(consumer::pull::Config {
                        durable_name: Some(consumer.to_string()),
                        name: Some(consumer),
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

    if let Err(e) = tokio::spawn(join_all(consumers)).await {
        error!("{e}");
    }

    Ok(())
}

async fn handle_message(
    consumer: consumer::Consumer<consumer::pull::Config>,
    state: ApiState,
) -> anyhow::Result<()> {
    // Get messages
    let mut messages = consumer.messages().await?;

    while let Some(Ok(message)) = messages.next().await {
        let subject = message.subject.to_string();

        match Event::from_str(&subject) {
            Ok(event) => match event {
                Event::SetAll(entity) => {
                    info!("set all entity = {}", entity.to_string());
                }
                Event::UpdateAll(_) => todo!(),
                Event::DeleteAll(_) => todo!(),
                Event::UpdateCache(entity) => {
                    info!("update cache only entity = {}", entity.to_string());
                }
                _ => todo!(),
            },
            Err(_) => {
                warn!(
                    subject = subject,
                    "received a message, subject cannot be mapped to event"
                );
            }
        }

        println!("Got message {:?}", message);
        if let Err(e) = message.ack().await {
            eprintln!("{e}");
        }
    }

    Ok(())
}
