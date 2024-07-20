use sellershut_core::{
    categories::{query_categories_server::QueryCategories, Category, Connection, Node},
    common::{paginate::Cursor, Paginate, SearchQuery, SearchQueryOptional},
};
use tracing::instrument;

use crate::state::{impls::map_err, ApiState};

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
        let max = self.config.query_limit;

        // get count
        let actual_count = {
            let user_param = pagination.first.unwrap_or(pagination.last());
            if user_param > max {
                max
            } else {
                user_param
            }
        };

        let get_count: i64 = actual_count as i64 + 1;

        let db_conn = &self.db_pool;

        let left_side = pagination.before.is_none();
        let cursor_unavailable = pagination.after.is_none() && pagination.before.is_none();

        let categories = if cursor_unavailable {
            sqlx::query_as!(
                Category,
                "select * FROM category order by created_at asc
                limit $1",
                get_count
            )
            .fetch_all(db_conn)
            .await
            .map_err(map_err)?
        } else {
            // get cursor
            let cursor = Cursor::decode(pagination);

            let index = cursor.idx();

            if left_side {
                sqlx::query_as!(
                    Category,
                    "select * FROM category
                        where idx > $1
                    order by 
                        created_at asc
                    limit $2",
                    index,
                    get_count
                )
                .fetch_all(db_conn)
                .await
                .map_err(map_err)?
            } else {
                sqlx::query_as!(
                    Category,
                    "select * FROM category
                        where idx < $1
                    order by 
                        created_at desc
                    limit $2",
                    index,
                    get_count
                )
                .fetch_all(db_conn)
                .await
                .map_err(map_err)?
            }
        };

        let len = categories.len();
        let user_count = actual_count as usize;

        let has_more = len > user_count;

        println!("{categories:#?}");

        let categories = if has_more {
            let iter = categories.into_iter();
            if !left_side {
                // from the right side
                iter.rev().take(user_count).rev().collect()
            } else {
                iter.take(user_count).collect()
            }
        } else {
            categories
        };

        let connection = Connection {
            edges: categories
                .into_iter()
                .map(|category| {
                    let cursor = Cursor::new(&category.id, category.idx);
                    Node {
                        node: Some(category),
                        cursor: cursor.encode(),
                    }
                })
                .collect(),
            page_info: None,
        };

        Ok(tonic::Response::new(connection))
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
