use sellershut_core::{
    categories::{
        query_categories_server::{QueryCategories, QueryCategoriesServer},
        Category, Connection, GetCategoryRequest, GetSubCategoriesRequest,
    },
    common::pagination::Cursor,
};
use tonic::transport::Server;

#[derive(Default)]
pub struct CategoryService;

#[tonic::async_trait]
impl QueryCategories for CategoryService {
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn categories(
        &self,
        request: tonic::Request<Cursor>,
    ) -> Result<tonic::Response<Connection>, tonic::Status> {
        println!("handling categories request {request:?}");

        Ok(tonic::Response::new(Connection::default()))
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn category_by_id(
        &self,
        request: tonic::Request<GetCategoryRequest>,
    ) -> Result<tonic::Response<Category>, tonic::Status> {
        println!("handling category_by_id request {request:?}");

        Ok(tonic::Response::new(Category::default()))
    }

    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn sub_categories(
        &self,
        request: tonic::Request<GetSubCategoriesRequest>,
    ) -> Result<tonic::Response<Connection>, tonic::Status> {
        println!("handling subcategories request {request:?}");

        Ok(tonic::Response::new(Connection::default()))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    println!("starting server on {addr:?}");
    Server::builder()
        .add_service(QueryCategoriesServer::new(CategoryService))
        .serve(addr)
        .await?;

    Ok(())
}
