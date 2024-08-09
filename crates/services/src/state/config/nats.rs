use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// MeilisearchConfig
pub struct NatsConfig {
    /// Meilisearch URL
    pub nats_url: String,
    /// Meilisearch api-key
    pub jetstream_name: String,
    /// Meilisearch index
    pub jetstream_subjects: Vec<String>,
    /// Meilisearch index
    pub jetstream_max_bytes: i64,
}
