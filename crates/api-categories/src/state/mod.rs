mod database;

use std::str::FromStr;

use async_nats::jetstream::stream;
use core_services::state::{
    config::{env_var, Configuration},
    events::Entity,
    ServiceState,
};
use tracing::error;

#[derive(Clone)]
pub struct ApiState {
    pub state: ServiceState,
}

impl ApiState {
    pub async fn initialise(config: Configuration) -> anyhow::Result<Self> {
        let state = ServiceState::initialise(config).await?;

        let stream = env_var("JETSTREAM_NAME");
        let subjects: Vec<_> = env_var("JETSTREAM_SUBJECTS")
            .split(',')
            .map(String::from)
            .collect();
        let stream_max_bytes = env_var("JETSTREAM_MAX_BYTES");

        let valid = subjects.iter().any(|value| {
            // Seems we're statically typing the subjects, they no longer need to be in env
            let mut tokens = value.split('.');

            let is_ok_base = tokens.next().map(|value| Entity::from_str(value).is_ok());

            match is_ok_base {
                Some(value) => {
                    let operation = tokens.next().map(|v| (v == "update") && value);
                    operation.unwrap_or_default()
                }
                None => false,
            }
        });

        if !valid {
            error!("none of your subjects could be parsed to entities. Event dispatch will fire blanks");
        }

        state
            .jetstream_context
            .get_or_create_stream(stream::Config {
                name: stream.to_string(),
                subjects,
                max_messages: 10_000,
                max_bytes: stream_max_bytes.parse()?,
                ..Default::default()
            })
            .await?;

        #[cfg(not(test))]
        sqlx::migrate!("./migrations").run(&state.db_pool).await?;

        Ok(Self { state })
    }
}
