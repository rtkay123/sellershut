use sellershut_core::{
    categories::{mutate_categories_server::MutateCategories, Category},
    common::SearchQuery,
};

use crate::state::ApiState;

#[tonic::async_trait]
impl MutateCategories for ApiState {
    #[doc = " Create a category"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn create(
        &self,
        request: tonic::Request<Category>,
    ) -> Result<tonic::Response<Category>, tonic::Status> {
        todo!()
    }

    #[doc = " Update a category"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn update(
        &self,
        request: tonic::Request<Category>,
    ) -> Result<tonic::Response<Category>, tonic::Status> {
        todo!()
    }

    #[doc = " Delete a category"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn delete(
        &self,
        request: tonic::Request<SearchQuery>,
    ) -> Result<tonic::Response<Category>, tonic::Status> {
        todo!()
    }
}
