use futures_util::TryFutureExt;
use sellershut_core::{
    categories::{query_categories_server::QueryCategories, Category, Connection, Node},
    common::{
        paginate::{self, Cursor},
        Paginate, SearchQuery, SearchQueryOptional,
    },
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
        let actual_count = paginate::query_count(max, &pagination);

        let get_count: i64 = actual_count as i64 + 1;

        let db_conn = &self.db_pool;

        let left_side = pagination.before.is_none();
        let cursor_unavailable = pagination.after.is_none() && pagination.before.is_none();

        let (count_on_other_end, categories) = if cursor_unavailable {
            let categories = sqlx::query_as!(
                Category,
                "select * FROM category order by created_at asc
                limit $1",
                get_count
            )
            .fetch_all(db_conn)
            .await
            .map_err(map_err)?;
            (get_count - categories.len() as i64, categories)
        } else {
            // get cursor
            let cursor = Cursor::decode(&pagination);

            let index = cursor.idx();

            if left_side {
                let fut_count =
                    sqlx::query_scalar!("select count (*) from category where idx <= $1", 2)
                        .fetch_one(db_conn)
                        .map_err(map_err);

                let fut_categories = sqlx::query_as!(
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
                .map_err(map_err);

                let (count, categories) = tokio::try_join!(fut_count, fut_categories)?;
                let count = count.ok_or_else(|| {
                    tonic::Status::new(tonic::Code::Internal, "count returned no items")
                })?;

                (count, categories)
            } else {
                let fut_count =
                    sqlx::query_scalar!("select count (*) from category where idx >= $1", index)
                        .fetch_one(db_conn)
                        .map_err(map_err);

                let fut_categories = sqlx::query_as!(
                    Category,
                    "select * FROM category
                        where idx < $1
                    order by 
                        created_at asc
                    limit $2",
                    index,
                    get_count
                )
                .fetch_all(db_conn)
                .map_err(map_err);

                let (count, categories) = tokio::try_join!(fut_count, fut_categories)?;
                let count = count.ok_or_else(|| {
                    tonic::Status::new(tonic::Code::Internal, "count returned no items")
                })?;
                (count, categories)
            }
        };

        let len = categories.len();
        let user_count = actual_count as usize;

        let has_more = len > user_count;

        let categories = if has_more {
            categories.into_iter().take(user_count).collect()
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
            page_info: Some(sellershut_core::common::PageInfo {
                has_next_page: {
                    if cursor_unavailable || left_side {
                        has_more
                    } else {
                        count_on_other_end > 0
                    }
                },
                has_previous_page: {
                    if left_side {
                        count_on_other_end > 0
                    } else {
                        has_more
                    }
                },
                ..Default::default() // other props calculated by async-graphql
            }),
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
        let request = request.into_inner();

        let pagination = request
            .pagination
            .as_ref()
            .expect("pagination to be available");
        let max = self.config.query_limit;

        // get count
        let actual_count = paginate::query_count(max, pagination);

        let get_count: i64 = actual_count as i64 + 1;

        let db_conn = &self.db_pool;

        let left_side = pagination.before.is_none();
        let cursor_unavailable = pagination.after.is_none() && pagination.before.is_none();

        let (count_on_other_end, categories) = match request.query {
            Some(parent_id) => {
                if cursor_unavailable {
                    let categories = sqlx::query_as!(
                        Category,
                        "select * FROM category
                            where parent_id = $1
                        order by
                            created_at asc
                        limit $2",
                        parent_id,
                        get_count
                    )
                    .fetch_all(db_conn)
                    .await
                    .map_err(map_err)?;
                    (get_count - categories.len() as i64, categories)
                } else {
                    // get cursor
                    let cursor = Cursor::decode(pagination);

                    let index = cursor.idx();

                    if left_side {
                        let fut_count = sqlx::query_scalar!(
                            "select count (*) from category
                                where idx <= $1
                                    and parent_id = $2",
                            index,
                            parent_id
                        )
                        .fetch_one(db_conn)
                        .map_err(map_err);

                        let fut_categories = sqlx::query_as!(
                            Category,
                            "select * FROM category
                                where idx > $1
                                    and parent_id = $2
                            order by 
                                created_at asc
                            limit $3",
                            index,
                            parent_id,
                            get_count
                        )
                        .fetch_all(db_conn)
                        .map_err(map_err);

                        let (count, categories) = tokio::try_join!(fut_count, fut_categories)?;
                        let count = count.ok_or_else(|| {
                            tonic::Status::new(tonic::Code::Internal, "count returned no items")
                        })?;

                        (count, categories)
                    } else {
                        let fut_count = sqlx::query_scalar!(
                            "select count (*) from category where idx >= $1 and parent_id = $2",
                            index,
                            parent_id
                        )
                        .fetch_one(db_conn)
                        .map_err(map_err);

                        let fut_categories = sqlx::query_as!(
                            Category,
                            "select * FROM category
                                where idx < $1
                                    and parent_id = $2
                            order by 
                                created_at asc
                            limit $3",
                            index,
                            parent_id,
                            get_count
                        )
                        .fetch_all(db_conn)
                        .map_err(map_err);

                        let (count, categories) = tokio::try_join!(fut_count, fut_categories)?;
                        let count = count.ok_or_else(|| {
                            tonic::Status::new(tonic::Code::Internal, "count returned no items")
                        })?;
                        (count, categories)
                    }
                }
            }
            None => {
                if cursor_unavailable {
                    let categories = sqlx::query_as!(
                        Category,
                        "select * FROM category
                            where parent_id is null
                        order by
                            created_at asc
                        limit $1",
                        get_count
                    )
                    .fetch_all(db_conn)
                    .await
                    .map_err(map_err)?;
                    (get_count - categories.len() as i64, categories)
                } else {
                    // get cursor
                    let cursor = Cursor::decode(pagination);

                    let index = cursor.idx();

                    if left_side {
                        let fut_count = sqlx::query_scalar!(
                            "select count (*) from category
                                where idx <= $1
                                    and parent_id is null",
                            index,
                        )
                        .fetch_one(db_conn)
                        .map_err(map_err);

                        let fut_categories = sqlx::query_as!(
                            Category,
                            "select * FROM category
                                where idx > $1
                                    and parent_id is null
                            order by 
                                created_at asc
                            limit $2",
                            index,
                            get_count
                        )
                        .fetch_all(db_conn)
                        .map_err(map_err);

                        let (count, categories) = tokio::try_join!(fut_count, fut_categories)?;
                        let count = count.ok_or_else(|| {
                            tonic::Status::new(tonic::Code::Internal, "count returned no items")
                        })?;

                        (count, categories)
                    } else {
                        let fut_count = sqlx::query_scalar!(
                            "select count (*) from category where idx >= $1 and parent_id is null",
                            index
                        )
                        .fetch_one(db_conn)
                        .map_err(map_err);

                        let fut_categories = sqlx::query_as!(
                            Category,
                            "select * FROM category
                                where idx < $1
                                    and parent_id is null
                            order by 
                                created_at asc
                            limit $2",
                            index,
                            get_count
                        )
                        .fetch_all(db_conn)
                        .map_err(map_err);

                        let (count, categories) = tokio::try_join!(fut_count, fut_categories)?;
                        let count = count.ok_or_else(|| {
                            tonic::Status::new(tonic::Code::Internal, "count returned no items")
                        })?;
                        (count, categories)
                    }
                }
            }
        };

        let len = categories.len();
        let user_count = actual_count as usize;

        let has_more = len > user_count;

        let categories = if has_more {
            categories.into_iter().take(user_count).collect()
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
            page_info: Some(sellershut_core::common::PageInfo {
                has_next_page: {
                    if cursor_unavailable || left_side {
                        has_more
                    } else {
                        count_on_other_end > 0
                    }
                },
                has_previous_page: {
                    if left_side {
                        count_on_other_end > 0
                    } else {
                        has_more
                    }
                },
                ..Default::default() // other props calculated by async-graphql
            }),
        };

        Ok(tonic::Response::new(connection))
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

impl ApiState {
    pub async fn index_categories(&self) {}
}
