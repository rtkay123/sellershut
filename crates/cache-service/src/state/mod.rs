use core_services::state::{config::Configuration, ServiceState};

#[derive(Clone, Debug)]
pub struct ApiState(pub ServiceState);

impl ApiState {
    pub async fn initialise(config: Configuration) -> anyhow::Result<Self> {
        let state = ServiceState::initialise(config).await?;

        Ok(Self(state))
    }
}
