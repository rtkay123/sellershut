use anyhow::Result;
use std::path::Path;
use tokio::task::JoinHandle;

use core_services::state::ServiceState;

pub struct ApiState {
    pub state: ServiceState,

    #[cfg(feature = "nlp")]
    pub classifier: crate::nlp::ZeroshotClassifier,
    #[cfg(feature = "nlp")]
    classifier_handle: JoinHandle<anyhow::Result<()>>,
}

impl ApiState {
    pub async fn initialise() -> Result<Self> {
        let man_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
        dotenvy::from_path(man_path).ok();

        let state = ServiceState::initialise(env!("CARGO_CRATE_NAME")).await?;

        #[cfg(feature = "nlp")]
        let (handle, classifier) = crate::nlp::ZeroshotClassifier::spawn();

        Ok(Self {
            state,
            #[cfg(feature = "nlp")]
            classifier_handle: handle,
            #[cfg(feature = "nlp")]
            classifier,
        })
    }
}
