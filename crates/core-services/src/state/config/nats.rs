use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NatsConfig {
    pub nats_url: String,
    pub jetstream_name: String,
    pub jetstream_subjects: Vec<String>,
    pub jetstream_max_bytes: i64,
}
