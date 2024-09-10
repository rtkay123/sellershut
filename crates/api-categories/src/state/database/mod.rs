use std::error::Error;

use async_nats::{jetstream::Context, HeaderMap};
use core_services::state::{events::Event, utils::NatsMetadataInjector};
use opentelemetry::global;
use prost::Message;
use tracing::{debug, instrument, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

pub mod mutation;
pub mod query;

pub fn map_err(err: impl Error) -> tonic::Status {
    tonic::Status::new(tonic::Code::Internal, err.to_string())
}

#[instrument(err(Debug))]
async fn publish_event(
    value: impl Message,
    event: Event,
    jetstream: &Context,
) -> Result<(), tonic::Status> {
    let mut headers = HeaderMap::new();
    let buf = Message::encode_to_vec(&value);

    let event = event.to_string();

    global::get_text_map_propagator(|propagator| {
        let context = Span::current().context();
        propagator.inject_context(&context, &mut NatsMetadataInjector(&mut headers))
    });

    jetstream
        .publish_with_headers(event, headers, buf.into())
        .await
        .map_err(map_err)?;

    debug!("message published");

    Ok(())
}
