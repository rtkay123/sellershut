use std::path::Path;

use core_services::state::ServiceState;

#[derive(Clone, Debug)]
pub struct ApiState(pub ServiceState);

impl ApiState {
    pub async fn initialise() -> anyhow::Result<Self> {
        let man_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
        dotenvy::from_path(man_path).ok();

        let state = ServiceState::initialise(env!("CARGO_CRATE_NAME")).await?;

        Ok(Self(state))
    }
}
