pub mod query;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use query::Query;
use sellershut_core::categories::{
    mutate_categories_server::MutateCategories, query_categories_server::QueryCategories,
};

pub struct ApiSchemaBuilder {}

pub type ApiSchema = Schema<Query, EmptyMutation, EmptySubscription>;

impl ApiSchemaBuilder {
    pub fn build<T>(data: T) -> ApiSchema
    where
        T: QueryCategories + MutateCategories,
    {
        Schema::build(Query::default(), EmptyMutation, EmptySubscription)
            .data(data)
            .finish()
    }
}
