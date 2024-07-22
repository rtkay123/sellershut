use futures_util::TryFutureExt;
use meilisearch_sdk::search::SearchResults;
use sellershut_core::{
    categories::{
        query_categories_server::QueryCategories, Category, CategorySearchResult, Connection, Node,
        SearchResult,
    },
    common::{
        pagination::{self, cursor::cursor_value::CursorType, Cursor, CursorBuilder, PageInfo},
        request::{search_query_optional::Pagination, SearchQuery, SearchQueryOptional},
    },
};
use tracing::instrument;

use crate::{
    api::entity::{self, CategoryWithParent},
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
        request: tonic::Request<Cursor>,
    ) -> Result<tonic::Response<Connection>, tonic::Status> {
        let pagination = request.into_inner();
        let max = self.config.query_limit;

        // get count
        let actual_count = pagination::query_count(
            max,
            &pagination.index.ok_or_else(|| {
                tonic::Status::new(tonic::Code::Internal, "missing pagination index")
            })?,
        );

        let get_count: i64 = actual_count as i64 + 1;

        let db_conn = &self.db_pool;

        let (count_on_other_end, categories) = if let Some(cursor_value) =
            pagination.cursor_value.as_ref()
        {
            // get cursor
            let cursor_value = cursor_value.cursor_type.as_ref().ok_or_else(|| {
                tonic::Status::new(tonic::Code::Internal, "Cursor type is not set")
            })?;
            let cursor = CursorBuilder::decode(cursor_value);

            let index = cursor.idx();

            match cursor_value {
                CursorType::After(_) => {
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
                }
                CursorType::Before(_) => {
                    let fut_count = sqlx::query_scalar!(
                        "select count (*) from category where idx >= $1",
                        index
                    )
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
            }
        } else {
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
        };

        let left_side = CursorBuilder::is_paginating_from_left(&pagination);
        let cursor_unavailable = CursorBuilder::is_cursor_unavailable(&pagination);

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
                    let cursor = CursorBuilder::new(&category.id, category.idx);
                    Node {
                        node: Some(category),
                        cursor: cursor.encode(),
                    }
                })
                .collect(),
            page_info: Some(PageInfo {
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
            .ok_or_else(|| tonic::Status::new(tonic::Code::Internal, "pagination is missing"))?;
        let max = self.config.query_limit;

        // get count
        if let Pagination::Cursor(pagination) = pagination {
            // get count
            let actual_count = pagination::query_count(
                max,
                &pagination.index.ok_or_else(|| {
                    tonic::Status::new(tonic::Code::Internal, "missing pagination index")
                })?,
            );

            let get_count: i64 = actual_count as i64 + 1;

            let db_conn = &self.db_pool;

            let left_side = CursorBuilder::is_paginating_from_left(pagination);
            let cursor_unavailable = CursorBuilder::is_cursor_unavailable(pagination);

            let (count_on_other_end, categories) = match request.query.as_ref() {
                Some(parent_id) => match pagination.cursor_value.as_ref() {
                    Some(cursor_value) => {
                        let cursor_value = cursor_value.cursor_type.as_ref().ok_or_else(|| {
                            tonic::Status::new(tonic::Code::Internal, "Cursor type is not set")
                        })?;
                        let cursor = CursorBuilder::decode(cursor_value);

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
                    None => {
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
                    }
                },
                None => match pagination.cursor_value.as_ref() {
                    Some(cursor_value) => {
                        let cursor_value = cursor_value.cursor_type.as_ref().ok_or_else(|| {
                            tonic::Status::new(tonic::Code::Internal, "Cursor type is not set")
                        })?;
                        let cursor = CursorBuilder::decode(cursor_value);

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
                    None => {
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
                    }
                },
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
                        let cursor = CursorBuilder::new(&category.id, category.idx);
                        Node {
                            node: Some(category),
                            cursor: cursor.encode(),
                        }
                    })
                    .collect(),
                page_info: Some(PageInfo {
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
        } else {
            Err(tonic::Status::new(
                tonic::Code::Internal,
                "invalid pagination",
            ))
        }
    }

    #[doc = " search categories"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    #[instrument(skip(self), err(Debug))]
    async fn search(
        &self,
        request: tonic::Request<SearchQuery>,
    ) -> Result<tonic::Response<SearchResult>, tonic::Status> {
        let request = request.into_inner();

        let pagination = request
            .pagination
            .as_ref()
            .ok_or_else(|| tonic::Status::new(tonic::Code::Internal, "missing pagination data"))?;
        if let sellershut_core::common::request::search_query::Pagination::Offset(data) = pagination
        {
            let max = self.config.query_limit;
            let query = &request.query;

            let query = meilisearch_sdk::search::SearchQuery::new(&self.meilisearch_index)
                .with_query(query.as_ref())
                .with_limit(if data.limit > max || data.limit == 0 {
                    max as usize
                } else {
                    data.limit as usize
                })
                .with_offset(data.offset as usize)
                .build();

            let results: SearchResults<entity::CategorySearchResult> = self
                .meilisearch_index
                .execute_query(&query)
                .await
                .map_err(map_err)?;

            let search_results: Vec<_> = results
                .hits
                .into_iter()
                .map(|hit| {
                    CategorySearchResult::from(entity::CategorySearchResult {
                        id: hit.result.id,
                        category: hit.result.category,
                        parent_name: hit.result.parent_name,
                    })
                })
                .collect();

            let result = SearchResult {
                results: search_results,
            };

            Ok(tonic::Response::new(result))
        } else {
            Err(tonic::Status::new(
                tonic::Code::Internal,
                "invalid pagination type",
            ))
        }
    }
}

impl ApiState {
    pub async fn update_index(&self, resource: &Category) {
        let index = self.meilisearch_index.clone();
        let pool = self.db_pool.clone();
        let parent_id = resource.parent_id.clone();
        tokio::spawn(async move {
            let results = sqlx::query_as!(
                    CategoryWithParent,
                    "select c.name as name, p.name as parent_name, c.id as id, c.sub_categories as sub_categories, c.created_at as created_at, c.updated_at as updated_at, c.idx as idx, c.parent_id as parent_id, c.image_url as image_url
                    from category c
                    left join category p on c.parent_id = p.id where p.id = $1;", parent_id
                ).fetch_one(&pool).await.unwrap();
            let resource = vec![results];
            index.add_or_update(&resource, Some("id")).await.unwrap();
        });
    }
    pub async fn index_categories(&self) {
        let results = sqlx::query_as!(
            CategoryWithParent,
            "select c.name as name, p.name as parent_name, c.id as id, c.sub_categories as sub_categories, c.created_at as created_at, c.updated_at as updated_at, c.idx as idx, c.parent_id as parent_id, c.image_url as image_url
            from category c
            left join category p on c.parent_id = p.id;"
        ).fetch_all(&self.db_pool).await.unwrap();

        let parsed: Vec<entity::CategorySearchResult> = results
            .into_iter()
            .map(|value| entity::CategorySearchResult {
                id: value.id.clone(),
                parent_name: value.parent_name,
                category: entity::Category {
                    id: value.id,
                    name: value.name,
                    sub_categories: value.sub_categories,
                    image_url: value.image_url,
                    parent_id: value.parent_id,
                    created_at: value.created_at,
                    updated_at: value.updated_at,
                    idx: value.idx,
                },
            })
            .collect();

        self.meilisearch_index
            .add_documents(&parsed, Some("id"))
            .await
            .unwrap();
    }
}
