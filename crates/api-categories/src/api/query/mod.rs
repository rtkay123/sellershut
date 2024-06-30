use async_graphql::{Context, Object};
use tracing::instrument;

#[derive(Default, Debug)]
pub struct CategoryQuery;

#[Object]
impl CategoryQuery {
    #[instrument(skip(self, ctx), err(Debug))]
    async fn categories(&self, ctx: &Context<'_>) -> String {
        String::default()
    }
}
