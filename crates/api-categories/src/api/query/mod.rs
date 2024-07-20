use async_graphql::{
    connection::{Connection, Edge, EmptyFields},
    Context, MergedObject, Object, Result,
};
use sellershut_core::{categories::query_categories_server::QueryCategories, common::Paginate};
use tonic::IntoRequest;
use tracing::instrument;

use crate::{api::entity::Category, state::ApiState};

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
        #[graphql(validator(min_length = 1))] after: Option<String>,
        #[graphql(validator(min_length = 1))] before: Option<String>,
        #[graphql(validator(minimum = 1, maximum = 100))] first: Option<i32>,
        #[graphql(validator(minimum = 1, maximum = 100))] last: Option<i32>,
    ) -> Result<Connection<String, Category, EmptyFields, EmptyFields>> {
        let pagination = Params::parse(after, before, first, last)?;

        let service = ctx.data::<ApiState>()?;

        let res = service
            .categories(pagination.into_request())
            .await?
            .into_inner();

        let mut conn = Connection::new(false, false);

        conn.edges = res
            .edges
            .into_iter()
            .map(|f| {
                Edge::new(
                    f.cursor,
                    Category::from(f.node.expect("category to be some")),
                )
            })
            .collect();

        Ok(conn)
    }
}

/// Relay-compliant connection parameters to page results by cursor/page size
pub struct Params;

impl Params {
    pub fn parse(
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> async_graphql::Result<Paginate> {
        if (last.is_some() && after.is_some()) || (before.is_some() && first.is_some()) {
            return Err("invalid pagination arguments. Backwards pagination needs 'last' and 'before'. Forward pagination uses 'first' and (optionally) 'after'".into());
        }
        if last.is_none() && first.is_none() {
            return Err("One of 'first' or 'last' should be provided".into());
        }

        if after.is_some() && before.is_some() {
            return Err("Only one or none of 'after' or 'before' should be provided".into());
        }

        Ok(Paginate {
            after,
            before,
            first,
            last,
        })
    }
}
