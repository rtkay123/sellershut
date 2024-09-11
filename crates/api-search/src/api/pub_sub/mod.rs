use std::sync::Arc;

use anyhow::{anyhow, Result};
use async_nats::jetstream::{consumer, stream, Context};
use core_services::state::config::env_var;
use futures_util::{StreamExt, TryFutureExt};
use tracing::{debug, error, info};

use crate::state::ApiState;

pub async fn serve(state: Arc<ApiState>) -> Result<()> {
    let js = state.state.jetstream_context.clone();

    let consumers_iter: Vec<_> = env_var("EVENT_PUBLISHING_SERVICES")
        .split(',')
        .map(|service| {
            let service = service.to_uppercase();
            let create_var = |s: &str| format!("{service}_{s}");
            let stream = env_var(&create_var("STREAM_NAME"));
            let subjects: Vec<_> = env_var(&create_var("STREAM_SUBJECTS"))
                .split(',')
                .map(String::from)
                .collect();

            let filter_subjects: Vec<String> = subjects
                .iter()
                .filter_map(|value| {
                    if value.contains("search") {
                        Some(value.to_string())
                    } else {
                        None
                    }
                })
                .collect();

            let filter_subjects = if filter_subjects.is_empty() {
                subjects.clone()
            } else {
                filter_subjects
            };

            let stream_max_bytes = env_var(&create_var("STREAM_MAX_BYTES"));
            (stream, subjects, stream_max_bytes, filter_subjects)
        })
        .collect();

    let mut consumers = Vec::with_capacity(consumers_iter.len());
    for (stream, subjects, max_bytes, filter_subjects) in consumers_iter {
        let stream = create_stream(&js, &stream, subjects, max_bytes.parse().unwrap()).await?;
        let consumer = create_consumer(stream, filter_subjects).await.unwrap();
        consumers.push(consumer);
    }

    futures_util::stream::iter(consumers)
        .for_each_concurrent(None, |consumer| {
            let state = Arc::clone(&state);
            async move {
                tokio::spawn(async move { handle_message(consumer, state).await });
            }
        })
        .await;

    Ok(())
}

async fn handle_message(
    consumer: consumer::Consumer<consumer::pull::Config>,
    state: Arc<ApiState>,
) -> anyhow::Result<()> {
    // Get messages
    let mut messages = consumer.messages().await?;

    while let Some(Ok(message)) = messages.next().await {
        info!("Got message {:?}", message);
        if let Err(e) = message.ack().await {
            error!("{e}");
        }
    }

    Ok(())
}

async fn create_stream(
    js: &Context,
    stream: &str,
    subjects: Vec<String>,
    max_bytes: i64,
) -> Result<stream::Stream> {
    debug!(stream = stream, subjects = ?subjects, "configuring subjects");
    let stream = js
        .get_or_create_stream(stream::Config {
            name: stream.to_string(),
            subjects,
            max_messages: 10_000,
            max_bytes: max_bytes,
            ..Default::default()
        })
        .map_err(|e| anyhow!(e.to_string()))
        .await?;

    Ok(stream)
}

async fn create_consumer(
    stream: stream::Stream,
    filter_subjects: Vec<String>,
) -> Result<consumer::Consumer<consumer::pull::Config>> {
    debug!(filter_subjects = ?filter_subjects, "configuring consumers");
    let consumer = format!("CONSUMER_{}", env!("CARGO_PKG_NAME"));
    stream
        .create_consumer(consumer::pull::Config {
            durable_name: Some(consumer.to_string()),
            name: Some(consumer.to_string()),
            filter_subjects,
            // NOTE: use this for maybe the search one filter_subjects
            ..Default::default()
        })
        .await
        .map_err(|e| anyhow!(e.to_string()))
}
