use sellershut_core::categories::{
    mutate_categories_server::MutateCategories, Category, DeleteCategoryRequest, Empty,
};

use crate::state::ApiState;

#[tonic::async_trait]
impl MutateCategories for ApiState {
    #[doc = " Create a category"]
    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn create(
        &self,
        request: tonic::Request<Category>,
    ) -> Result<tonic::Response<Category>, tonic::Status> {
        todo!()
    }

    #[doc = " Update a category"]
    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn update(
        &self,
        request: tonic::Request<Category>,
    ) -> Result<tonic::Response<Category>, tonic::Status> {
        todo!()
    }

    #[doc = " Delete a category"]
    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn delete(
        &self,
        request: tonic::Request<DeleteCategoryRequest>,
    ) -> Result<tonic::Response<Empty>, tonic::Status> {
        todo!()
    }
}
