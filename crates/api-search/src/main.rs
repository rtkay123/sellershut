mod state;

#[cfg(feature = "nlp")]
mod nlp;

use std::sync::Arc;

use anyhow::{anyhow, Result};
use async_nats::jetstream::{consumer, stream};
use core_services::{
    state::config::{env_var, Configuration},
    tracing::{
        config::{AppMetadata, LokiConfig},
        TelemetryBuilder,
    },
};
use futures_util::{
    future::{join_all, try_join_all},
    StreamExt, TryFutureExt,
};
use state::ApiState;

use axum::{extract::State, response::Html, routing::get, Router};
use tracing::{debug, error, info};

#[tokio::main]
async fn main() -> Result<()> {
    let man_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
    dotenvy::from_path(man_path).ok();

    let crate_name = env!("CARGO_CRATE_NAME");
    let crate_version = env!("CARGO_PKG_VERSION");

    let config = Configuration::new(crate_name, crate_version);
    let metadata = AppMetadata {
        name: crate_name,
        version: crate_version,
        env: config.env,
    };

    let mut telemetry = TelemetryBuilder::new()
        .try_with_loki(LokiConfig::new(metadata, &config.loki_url))?
        .try_with_opentelemetry(metadata, &config.otel_collector)?
        .try_with_sentry(&config.sentry_dsn)?
        .build();

    if let Some(task) = std::mem::take(&mut telemetry.loki_task) {
        tokio::spawn(task);
    };

    let state = Arc::new(ApiState::initialise(config).await?);

    let addr = state.state.config.listen_address;

    let js = state.state.jetstream_context.clone();

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
            println!("{}",subjects.len());

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
            let consumer = format!("CONSUMER_{}", env!("CARGO_PKG_NAME"));
            debug!(stream = stream, subjects = ?subjects, filter_subjects = ?filter_subjects, "configuring subjects");

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
                        filter_subjects,
                        // NOTE: use this for maybe the search one filter_subjects
                        ..Default::default()
                    })
                    .await
                    .map_err(|e| anyhow!(e.to_string()))
            })
        })
        .collect();

    let consumers = try_join_all(services).await?.into_iter().map(|consumer| {
        let state = Arc::clone(&state);
        tokio::spawn(handle_message(consumer, state))
    });

    if let Err(e) = tokio::spawn(join_all(consumers)).await {
        error!("{e}");
    }

    let app = Router::new().route("/", get(handler)).with_state(state);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    info!(addr = ?addr, "listening");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn handler(State(state): State<Arc<ApiState>>) -> Html<&'static str> {
    #[cfg(feature = "nlp")]
    {
        let texts = vec![
        "Who are you voting for in 2020?".into(),
        "The prime minister has announced a stimulus package which was widely criticized by the opposition.".into()
    ];
        let res = state.classifier.predict(texts).await;
        println!("{res:?}");
    }
    Html("<h1>Hello, World!</h1>")
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
