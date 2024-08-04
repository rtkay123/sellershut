use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// MeilisearchConfig
pub struct MeilisearchConfig {
    /// Meilisearch URL
    pub meilisearch_url: String,
    /// Meilisearch api-key
    pub meilisearch_api_key: String,
    /// Meilisearch index
    pub meilisearch_index: String,
}
