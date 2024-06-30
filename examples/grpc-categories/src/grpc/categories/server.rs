use sellershut_core::{
    categories::{
        query_categories_server::{QueryCategories, QueryCategoriesServer},
        Category, Connection,
    },
    common::{Paginate, SearchQuery, SearchQueryOptional},
};
use tonic::transport::Server;

#[derive(Default)]
pub struct CategoryService;

#[tonic::async_trait]
impl QueryCategories for CategoryService {
    #[doc = " gets all categories"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn categories(
        &self,
        request: tonic::Request<Paginate>,
    ) -> Result<tonic::Response<Connection>, tonic::Status> {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn category_by_id(
        &self,
        request: tonic::Request<SearchQuery>,
    ) -> Result<tonic::Response<Category>, tonic::Status> {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn sub_categories(
        &self,
        request: tonic::Request<SearchQueryOptional>,
    ) -> Result<tonic::Response<Connection>, tonic::Status> {
        todo!()
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn search(
        &self,
        request: tonic::Request<SearchQuery>,
    ) -> Result<tonic::Response<Connection>, tonic::Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    Server::builder()
        .add_service(QueryCategoriesServer::new(CategoryService))
        .serve(addr)
        .await?;

    Ok(())
}
