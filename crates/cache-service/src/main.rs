mod state;

use async_nats::jetstream::{self, consumer::PullConsumer, stream, Context};
use futures_util::{future::join_all, StreamExt};
use state::ApiState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state = ApiState::initialise().await?;
    let client = async_nats::connect("localhost:4222").await?;
    let jetstream = async_nats::jetstream::new(client);

    // Define your streams and consumers
    let configurations = vec![("categories", "consumer1"), ("users", "consumer2")];

    // Spawn tasks for each stream and consumer configuration
    let mut handles = vec![];
    for (stream_name, consumer_name) in configurations {
        let context = jetstream.clone();
        let handle = tokio::spawn(async move {
            if let Err(e) = handle_messages(stream_name, consumer_name, context).await {
                eprintln!(
                    "Error handling messages for stream '{}' and consumer '{}': {:?}",
                    stream_name, consumer_name, e
                );
            }
        });
        handles.push(handle);
    }

    join_all(handles).await;

    Ok(())
}

async fn handle_messages(
    stream_name: &str,
    consumer_name: &str,
    jetstream: Context,
) -> anyhow::Result<()> {
    // Create or get the consumer
    let consumer: PullConsumer = jetstream
        .get_or_create_stream(stream::Config {
            name: stream_name.to_string(),
            subjects: vec![format!("{stream_name}.>")],
            max_messages: 10_000,
            ..Default::default()
        })
        .await?
        // Then, on that `Stream` use method to create Consumer and bind to it too.
        .create_consumer(jetstream::consumer::pull::Config {
            durable_name: Some(consumer_name.into()),
            name: Some(consumer_name.into()),
            ..Default::default()
        })
        .await?;

    // Get messages
    let mut messages = consumer.messages().await?;

    println!(
        "Listening on stream '{}' with consumer '{}'",
        stream_name, consumer_name
    );
    while let Some(Ok(message)) = messages.next().await {
        println!(
            "Got message from '{}' with consumer '{}': {:?}",
            stream_name, consumer_name, message
        );
        message.ack().await.unwrap();
    }

    Ok(())
}
