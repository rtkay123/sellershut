use async_nats::jetstream::stream;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = async_nats::connect("localhost:4222").await?;
    let jetstream = async_nats::jetstream::new(client);

    let stream = jetstream
        .get_or_create_stream(stream::Config {
            name: "categories".to_string(),
            max_messages: 10_000,
            ..Default::default()
        })
        .await?;

    let consumer = stream
        .get_or_create_consumer(
            "consumer",
            async_nats::jetstream::consumer::pull::OrderedConfig {
                name: Some("consumer".to_string()),
                ..Default::default()
            },
        )
        .await?;

    let mut messages = consumer.messages().await?;

    println!("listening");
    while let Some(Ok(message)) = messages.next().await {
        println!("got message {:?}", message);
        message.ack().await.unwrap();
    }

    Ok(())
}
