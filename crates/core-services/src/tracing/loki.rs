use std::{collections::HashMap, process};

use crate::state::config::Configuration;

#[derive(Clone, Debug)]
/// Configuration for Loki
pub struct LokiConfig {
    /// Labels
    pub labels: HashMap<String, String>,
    /// Extra fields
    pub extra_fields: HashMap<String, String>,
    /// Host
    #[cfg(feature = "tracing-loki")]
    pub host: String,
}

impl From<&Configuration> for LokiConfig {
    fn from(config: &Configuration) -> Self {
        let mut labels = HashMap::new();
        labels.insert("environment".into(), config.env.to_string());
        labels.insert("application".into(), config.crate_name.to_string());
        labels.insert("version".into(), config.crate_version.to_string());

        let mut extra_fields = HashMap::new();
        extra_fields.insert("pid".into(), format!("{}", process::id()));
        LokiConfig {
            labels,
            extra_fields,
            #[cfg(feature = "tracing-loki")]
            host: config.loki_url.to_string(),
        }
    }
}
