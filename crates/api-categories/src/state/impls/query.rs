use sellershut_core::{
    categories::{query_categories_server::QueryCategories, Category, Connection, Node},
    common::{Paginate, SearchQuery, SearchQueryOptional},
};
use tracing::instrument;

use crate::{
    api::entity,
    state::{impls::map_err, ApiState},
};

#[tonic::async_trait]
impl QueryCategories for ApiState {
    #[doc = " gets all categories"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    #[instrument(skip(self), err(Debug))]
    async fn categories(
        &self,
        request: tonic::Request<Paginate>,
    ) -> Result<tonic::Response<Connection>, tonic::Status> {
        let pagination = request.into_inner();
        let db_conn = &self.db_pool;

        let res = sqlx::query_as!(Category, "select * FROM category")
            .fetch_all(db_conn)
            .await
            .map_err(map_err)?;

        let len = res.len();

        let nodes = res
            .into_iter()
            .map(|f| Node {
                node: Some(f),
                cursor: String::default(),
            })
            .collect();

        let conn = Connection {
            edges: nodes,
            page_info: None,
        };

        Ok(tonic::Response::new(conn))
    }

    #[doc = " get category by id"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    #[instrument(skip(self), err(Debug))]
    async fn category_by_id(
        &self,
        request: tonic::Request<SearchQuery>,
    ) -> Result<tonic::Response<Category>, tonic::Status> {
        let id = request.into_inner().query;

        let category = sqlx::query_as!(Category, "select * from category where id = $1", id)
            .fetch_one(&self.db_pool)
            .await
            .map_err(map_err)?;

        Ok(tonic::Response::new(category))
    }

    #[doc = " get subcategories"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    #[instrument(skip(self), err(Debug))]
    async fn sub_categories(
        &self,
        request: tonic::Request<SearchQueryOptional>,
    ) -> Result<tonic::Response<Connection>, tonic::Status> {
        todo!()
    }

    #[doc = " search categories"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    #[instrument(skip(self), err(Debug))]
    async fn search(
        &self,
        request: tonic::Request<SearchQuery>,
    ) -> Result<tonic::Response<Connection>, tonic::Status> {
        todo!()
    }
}
