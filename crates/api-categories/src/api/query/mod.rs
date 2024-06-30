use async_graphql::{Context, MergedObject, Object};
use sellershut_core::categories::Category;
use tracing::instrument;

use crate::state::ApiState;

#[derive(Default, Debug, MergedObject)]
pub struct Query(GraphqlQuery);

#[derive(Default, Debug)]
pub struct GraphqlQuery;

#[Object]
impl GraphqlQuery {
    #[instrument(skip(self, ctx), err(Debug))]
    async fn categories(
        &self,
        ctx: &Context<'_>,
        val: Category,
    ) -> async_graphql::Result<Vec<Category>> {
        let service = ctx.data::<ApiState>()?;

        Ok(vec![])
    }
}
