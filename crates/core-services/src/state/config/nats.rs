use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NatsConfig {
    pub nats_url: String,
}

#[derive(Debug, Deserialize)]
pub struct StreamConfig {
    pub name: String,
    pub subjects: Vec<String>,
    pub max_bytes: i64,
}
