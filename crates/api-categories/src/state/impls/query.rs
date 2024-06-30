use sellershut_core::{
    categories::{query_categories_server::QueryCategories, Category, Connection},
    common::{Paginate, SearchQuery, SearchQueryOptional},
};

use crate::state::ApiState;

#[tonic::async_trait]
impl QueryCategories for ApiState {
    #[doc = " gets all categories"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn categories(
        &self,
        request: tonic::Request<Paginate>,
    ) -> Result<tonic::Response<Connection>, tonic::Status> {
        todo!()
    }

    #[doc = " get category by a with id"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn category_by_id(
        &self,
        request: tonic::Request<SearchQuery>,
    ) -> Result<tonic::Response<Category>, tonic::Status> {
        todo!()
    }

    #[doc = " get subcategories"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn sub_categories(
        &self,
        request: tonic::Request<SearchQueryOptional>,
    ) -> Result<tonic::Response<Connection>, tonic::Status> {
        todo!()
    }

    #[doc = " search categories"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn search(
        &self,
        request: tonic::Request<SearchQuery>,
    ) -> Result<tonic::Response<Connection>, tonic::Status> {
        todo!()
    }
}
