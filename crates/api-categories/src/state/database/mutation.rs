use sellershut_core::{
    categories::{
        mutate_categories_server::MutateCategories, Category, DeleteCategoryRequest,
        UpsertCategoryRequest,
    },
    google::protobuf::Empty,
};

use crate::state::ApiState;

#[tonic::async_trait]
impl MutateCategories for ApiState {
    #[doc = " Create a category"]
    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn create(
        &self,
        request: tonic::Request<UpsertCategoryRequest>,
    ) -> Result<tonic::Response<Category>, tonic::Status> {
        // send message to cache update and search index update
        let req = request.into_inner();
        let event = req.event();
        let category = req.category.expect("category to be defined");
        let subject = format!("{}.create.{}", self.subject.to_string(), "");

        let _ = self
            .state
            .jetstream_context
            .publish(subject, "data".into())
            .await;
        todo!()
    }

    #[doc = " Update a category"]
    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn update(
        &self,
        request: tonic::Request<UpsertCategoryRequest>,
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
