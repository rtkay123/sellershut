mod database;

use std::path::Path;

use async_nats::jetstream::stream;
use core_services::state::{config::env_var, ServiceState};

#[derive(Clone)]
pub struct ApiState {
    pub state: ServiceState,
    pub subject: std::sync::Arc<str>,
}

impl ApiState {
    pub async fn initialise() -> anyhow::Result<Self> {
        let man_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
        dotenvy::from_path(man_path).ok();

        let state = ServiceState::initialise(env!("CARGO_CRATE_NAME")).await?;

        let stream = env_var("JETSTREAM_NAME");
        let subject = env_var("JETSTREAM_SUBJECTS");
        let stream_max_bytes = env_var("JETSTREAM_MAX_BYTES");

        state
            .jetstream_context
            .get_or_create_stream(stream::Config {
                name: stream.to_string(),
                subjects: vec![format!("{subject}")],
                max_messages: 10_000,
                //                max_bytes: stream_max_bytes.parse().unwrap(),
                ..Default::default()
            })
            .await?;

        #[cfg(not(test))]
        sqlx::migrate!("./migrations").run(&state.db_pool).await?;

        Ok(Self {
            state,
            subject: subject.into(),
        })
    }
}
