use std::{collections::HashMap, process};

use crate::state::config::Environment;

#[derive(Clone, Debug)]
/// Configuration for Loki
#[cfg(feature = "tracing-loki")]
pub struct LokiConfig {
    /// Labels
    pub labels: HashMap<String, String>,
    /// Extra fields
    pub extra_fields: HashMap<String, String>,
    /// Host
    #[cfg(feature = "tracing-loki")]
    pub host: String,
}

#[cfg(feature = "tracing-loki")]
impl LokiConfig {
    pub fn new(config: AppMetadata, loki_url: &str) -> Self {
        let mut labels = HashMap::new();
        labels.insert("environment".into(), config.env.to_string());
        labels.insert("service_name".into(), config.name.to_string());
        labels.insert("service_version".into(), config.version.to_string());

        let mut extra_fields = HashMap::new();
        extra_fields.insert("pid".into(), format!("{}", process::id()));
        LokiConfig {
            labels,
            extra_fields,
            #[cfg(feature = "tracing-loki")]
            host: loki_url.to_string(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AppMetadata<'a> {
    pub name: &'a str,
    pub version: &'a str,
    pub env: Environment,
}
