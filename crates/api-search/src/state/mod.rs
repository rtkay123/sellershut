use anyhow::Result;
use sellershut_core::categories::query_categories_client::QueryCategoriesClient;
use tokio::task::JoinHandle;

use meilisearch_sdk::client::Client;

use core_services::state::{
    config::{env_var, Configuration},
    ServiceState,
};

pub struct ApiState {
    pub state: ServiceState,

    #[cfg(feature = "nlp")]
    pub classifier: crate::nlp::ZeroshotClassifier,
    #[cfg(feature = "nlp")]
    classifier_handle: JoinHandle<anyhow::Result<()>>,
    pub categories_client: QueryCategoriesClient<tonic::transport::Channel>,
    meilisearch: Client,
}

impl ApiState {
    pub async fn initialise(config: Configuration) -> Result<Self> {
        let state = ServiceState::initialise(config).await?;

        #[cfg(feature = "nlp")]
        let (handle, classifier) = crate::nlp::ZeroshotClassifier::spawn();

        let categories_client = QueryCategoriesClient::connect("http://[::1]:1304").await?;

        let meilisearch_url = env_var("MEILISEARCH_URL");
        let meilisearch_key = env_var("MEILISEARCH_APIKEY");

        let meilisearch = Client::new(&meilisearch_url, Some(meilisearch_key))?;

        Ok(Self {
            state,
            #[cfg(feature = "nlp")]
            classifier_handle: handle,
            #[cfg(feature = "nlp")]
            classifier,
            categories_client,
            meilisearch,
        })
    }
}
