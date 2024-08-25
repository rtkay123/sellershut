use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NatsConfig {
    pub nats_url: String,
    pub stream: Vec<StreamConfig>,
}

#[derive(Debug, Deserialize)]
pub struct StreamConfig {
    pub name: String,
    pub subjects: Vec<String>,
    pub max_bytes: i64,
}
