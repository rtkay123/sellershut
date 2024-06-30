use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use query::CategoryQuery;
use sellershut_core::categories::{
    mutate_categories_server::MutateCategories, query_categories_server::QueryCategories,
};

pub mod query;

pub struct ApiSchemaBuilder {}

impl ApiSchemaBuilder {
    pub fn new<T>(data: T) -> Schema<CategoryQuery, EmptyMutation, EmptySubscription>
    where
        T: QueryCategories + MutateCategories,
    {
        Schema::build(CategoryQuery::default(), EmptyMutation, EmptySubscription)
            .data(data)
            .finish()
    }
}
