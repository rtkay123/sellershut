mod state;

use std::{collections::HashMap, path::Path, str::FromStr};

use anyhow::{anyhow, Result};
use async_nats::jetstream::{consumer, stream};
use core_services::{
    cache::{
        key::{CacheKey, CursorParams, Index},
        PoolLike, PooledConnectionLike,
    },
    state::{
        config::{env_var, Configuration},
        events::{Entity, Event},
        utils::NatsMetadataExtractor,
        ServiceState,
    },
    tracing::{
        config::{AppMetadata, LokiConfig},
        TelemetryBuilder,
    },
};
use futures_util::{
    future::{join_all, try_join_all},
    StreamExt, TryFutureExt,
};
use opentelemetry::global;
use prost::Message;
use sellershut_core::{
    categories::{CacheCategoriesConnectionRequest, Category},
    common::pagination::{cursor::cursor_value::CursorType, Cursor},
};
use state::ApiState;
use tracing::{debug, error, info, instrument, trace, warn, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

#[tokio::main]
async fn main() -> Result<()> {
    let man_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
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

    let state = ApiState::initialise(config).await?;

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
    info!("consumer is ready to receive messages");

    while let Some(Ok(message)) = messages.next().await {
        debug!("message received");
        let subject = message.subject.to_string();

        match Event::from_str(&subject) {
            Ok(event) => {
                let _ = process_event(event, &state.0, message).await;
                info!(event = ?event, "event processed");
            }
            Err(_) => {
                warn!(
                    subject = subject,
                    "received a message, subject cannot be mapped to event"
                );
            }
        }
    }

    Ok(())
}

#[instrument(err(Debug), skip(message))]
async fn process_event(
    event: Event,
    state: &ServiceState,
    message: async_nats::jetstream::Message,
) -> anyhow::Result<()> {
    let payload = message.payload.as_ref();

    let transaction = message.headers.as_ref().map(|headers| {
        let mut hash_map = HashMap::new();
        headers.iter().for_each(|(key, value)| {
            for a in value.iter() {
                hash_map.insert(key.to_string(), a.to_string());
            }
        });
        let map: HashMap<_, _> = hash_map
            .iter()
            .map(|(a, b)| (a.as_ref(), b.as_str()))
            .collect();

        let tx_ctx = sentry::TransactionContext::continue_from_headers(
            "cache-update",
            "nats.jetstream",
            map,
        );

        let parent_context = global::get_text_map_propagator(|propagator| {
            let extractor = NatsMetadataExtractor(headers);
            propagator.extract(&extractor)
        });

        Span::current().set_parent(parent_context);
        sentry::start_transaction(tx_ctx)
    });
    match event {
        Event::SetSingle(entity) => match entity {
            Entity::Categories => {
                trace!(entity = ?entity, "decoding payload");
                let category = Category::decode(payload)?;

                let cache_key = CacheKey::Category(&category.id);
                write_to_cache(cache_key, payload, state).await?;
            }
            _ => {}
        },
        Event::SetBatch(_) => {}
        Event::UpdateSingle(entity) => match entity {
            Entity::Categories => {
                trace!(entity = ?entity, "decoding payload");
                let category = Category::decode(payload)?;

                let cache_key = CacheKey::Category(&category.id);
                write_to_cache(cache_key, payload, state).await?;
            }
            _ => todo!(),
        },
        Event::UpdateBatch(entity) => match entity {
            Entity::Categories => {
                trace!(entity = ?entity, "decoding payload");
                let category = CacheCategoriesConnectionRequest::decode(payload)?;

                if let Some((cursor, index)) = get_cursor_params(category.pagination) {
                    let cache_key = CacheKey::Categories(CursorParams {
                        cursor: cursor.as_deref(),
                        index,
                    });
                    write_to_cache(cache_key, payload, state).await?;
                } else {
                    error!("pagination is missing from payload");
                }
            }
            _ => todo!(),
        },
        Event::DeleteSingle(entity) => {
            let mut cache = state.cache.get().await?;
            match entity {
                Entity::Categories => {
                    trace!(entity = ?entity, "decoding payload");
                    let category = Category::decode(payload)?;

                    let cache_key = CacheKey::Category(&category.id);
                    cache.del::<_, ()>(cache_key).await?;
                }
                _ => todo!(),
            }
        }
        Event::DeleteBatch(entity) => {
            let _cache = state.cache.get().await?;
            match entity {
                Entity::Categories => {
                    unimplemented!()
                }
                _ => todo!(),
            }
        }
        Event::CacheUpdateSingle(_) => {}
        Event::CacheUpdateBatch(_) => {}
        _ => {}
    }

    if let Err(e) = message.ack().await {
        error!("{e}");
    }

    if let Some(transaction) = transaction {
        transaction.finish();
        trace!("finishing sentry transaction");
    }

    Ok(())
}

fn get_cursor_params(pagination: Option<Cursor>) -> Option<(Option<String>, Index)> {
    if let Some(pagination) = pagination {
        if let Some(index) = pagination.index {
            let index = match index {
                sellershut_core::common::pagination::cursor::Index::First(value) => {
                    Index::First(value)
                }
                sellershut_core::common::pagination::cursor::Index::Last(value) => {
                    Index::Last(value)
                }
            };

            let cursor = pagination.cursor_value.and_then(|value| {
                value.cursor_type.map(|value| match value {
                    CursorType::After(value) => value,
                    CursorType::Before(value) => value,
                })
            });
            Some((cursor, index))
        } else {
            error!("index is missing from pagination params");
            None
        }
    } else {
        error!("pagination is missing from payload");
        None
    }
}

#[instrument(err(Debug), skip(state, payload))]
async fn write_to_cache(
    cache_key: CacheKey<'_>,
    payload: &[u8],
    state: &ServiceState,
) -> anyhow::Result<()> {
    let mut cache = state.cache.get().await?;
    trace!(key = ?cache_key, "writing to cache");
    Ok(cache.pset_ex::<_, _, ()>(cache_key, payload, 20000).await?)
}
