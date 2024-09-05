use core_services::cache::{
    key::{CacheKey, CursorParams},
    PoolLike, PooledConnection, PooledConnectionLike,
};
use futures_util::TryFutureExt;
use prost::Message;
use sellershut_core::{
    categories::{
        query_categories_server::QueryCategories, Category, CategoryEvent, Connection,
        GetCategoryRequest, GetSubCategoriesRequest, Node, UpsertCategoryRequest,
    },
    common::pagination::{self, cursor::cursor_value::CursorType, Cursor, CursorBuilder, PageInfo},
};
use time::{format_description::well_known::Rfc3339, OffsetDateTime, UtcOffset};
use tracing::{debug, error, info_span, trace, Instrument};

use crate::{
    api::entity::{self, to_offset_datetime},
    state::{database::map_err, ApiState},
};

#[tonic::async_trait]
impl QueryCategories for ApiState {
    #[doc = " gets all categories"]
    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn categories(
        &self,
        request: tonic::Request<pagination::Cursor>,
    ) -> Result<tonic::Response<Connection>, tonic::Status> {
        // get cache first
        trace!("getting cache state");
        let cache = self.state.cache.get().await.map_err(map_err)?;

        let pagination = request.into_inner();

        let max = self.state.config.query_limit;
        // get count
        let actual_count = pagination::query_count(
            max,
            &pagination.index.ok_or_else(|| {
                tonic::Status::new(tonic::Code::Internal, "missing pagination index")
            })?,
        );
        // get 1 more
        let get_count: i64 = actual_count as i64 + 1;

        // a cursor was specified
        let connection = if let Some(ref cursor) = pagination.cursor_value {
            // get cursor
            let cursor_value = cursor.cursor_type.as_ref().ok_or_else(|| {
                tonic::Status::new(tonic::Code::Internal, "Cursor type is not set")
            })?;

            let decode_cursor = |cursor_value: &CursorType| {
                CursorBuilder::decode(cursor_value)
                    .map_err(|e| tonic::Status::internal(e.to_string()))
            };

            let connection = match cursor_value {
                CursorType::After(cursor) => {
                    // try cache first
                    let cache_key = CacheKey::Categories(CursorParams {
                        cursor: Some(cursor),
                        index: core_services::cache::key::Index::After(actual_count),
                    });

                    let cache_result = read_cache(cache_key, cache).await;

                    let connection = match cache_result {
                        Ok(result) => result,
                        Err(e) => {
                            error!("cache read {e}");

                            let cursor = decode_cursor(cursor_value)?;
                            let id = cursor.id();
                            debug!("converting to date {:?}", cursor.dt());

                            let created_at =
                                OffsetDateTime::parse(cursor.dt(), &Rfc3339).map_err(map_err)?;

                            let fut_count = sqlx::query_scalar!(
                                "
                                    select count(*) from category
                                    where 
                                        (
                                            created_at <> $1
                                            or id <= $2
                                        )
                                        and created_at < $1
                                ",
                                created_at,
                                id,
                            )
                            .fetch_one(&self.state.db_pool)
                            .map_err(map_err);

                            let fut_categories = sqlx::query_as!(
                                entity::Category,
                                "
                                    select * from category
                                    where 
                                        (
                                            created_at = $1
                                            and id > $2
                                        )
                                        or created_at >= $1
                                    order by
                                        created_at asc,
                                        id asc
                                    limit
                                        $3
                                ",
                                created_at,
                                id,
                                get_count
                            )
                            .fetch_all(&self.state.db_pool)
                            .map_err(map_err);

                            let (count_on_other_end, categories) =
                                tokio::try_join!(fut_count, fut_categories)?;

                            let categories: Vec<_> =
                                categories.into_iter().map(Category::from).collect();

                            let connection = parse_categories(
                                count_on_other_end,
                                categories,
                                &pagination,
                                actual_count,
                            )?;

                            connection
                        }
                    };
                    connection
                }
                CursorType::Before(cursor) => {
                    // try cache first
                    let cache_key = CacheKey::Categories(CursorParams {
                        cursor: Some(cursor),
                        index: core_services::cache::key::Index::Before(actual_count),
                    });

                    let cache_result = read_cache(cache_key, cache).await;

                    let connection = match cache_result {
                        Ok(result) => result,
                        Err(e) => {
                            error!("cache read {e}");

                            let cursor = decode_cursor(cursor_value)?;
                            let id = cursor.id();
                            let created_at = OffsetDateTime::parse(cursor.dt(), &Rfc3339)
                                .map_err(|e| tonic::Status::internal(e.to_string()))?;

                            let fut_count = sqlx::query_scalar!(
                                "
                                    select count(*) from category
                                    where 
                                        (
                                            created_at <> $1
                                            or id > $2
                                        )
                                        and created_at >= $1
                                ",
                                created_at,
                                id,
                            )
                            .fetch_one(&self.state.db_pool)
                            .map_err(map_err);

                            let fut_categories = sqlx::query_as!(
                                entity::Category,
                                "
                                    select * from category
                                    where 
                                        (
                                            created_at = $1
                                            and id < $2
                                        )
                                        or created_at < $1
                                    order by
                                        created_at desc,
                                        id desc
                                    limit
                                        $3
                                ",
                                created_at,
                                id,
                                get_count
                            )
                            .fetch_all(&self.state.db_pool)
                            .map_err(map_err);

                            let (count, categories) = tokio::try_join!(fut_count, fut_categories)?;

                            let categories: Vec<_> =
                                categories.into_iter().map(Category::from).collect();

                            let connection =
                                parse_categories(count, categories, &pagination, actual_count)?;

                            connection
                        }
                    };
                    connection
                }
            };
            connection
        } else {
            let categories = sqlx::query_as!(
                entity::Category,
                "select * FROM category order by created_at asc
                limit $1",
                get_count
            )
            .fetch_all(&self.state.db_pool)
            .await
            .map_err(map_err)?;

            let connection = parse_categories(
                Some(get_count - categories.len() as i64),
                categories.into_iter().map(Category::from).collect(),
                &pagination,
                actual_count,
            )?;

            connection
        };

        let byte_data = connection.clone().encode_to_vec();

        let subject = format!("{}.update.set", self.subject);

        let _ = self
            .state
            .jetstream_context
            .publish(subject, byte_data.into())
            .await;
        debug!("message published");

        Ok(tonic::Response::new(connection))
    }

    #[doc = " get category by id"]
    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn category_by_id(
        &self,
        request: tonic::Request<GetCategoryRequest>,
    ) -> Result<tonic::Response<Category>, tonic::Status> {
        let state = &self.state;
        let id = request.into_inner().id;

        let cache_key = CacheKey::Category(&id);

        let s = info_span!("cache call");

        // get cache first
        let mut cache = self.state.cache.get().await.map_err(map_err)?;
        let cache_result = cache
            .get::<_, Vec<u8>>(&cache_key)
            .map_err(|e| tonic::Status::internal(e.to_string()))
            .and_then(|payload| async move {
                if !payload.is_empty() {
                    Category::decode(payload.as_ref())
                        .map_err(|e| tonic::Status::internal(e.to_string()))
                } else {
                    let msg = "no data available in cache";
                    debug!("{}", msg);
                    Err(tonic::Status::not_found(msg))
                }
            })
            .instrument(s)
            .await;

        let category = match cache_result {
            Ok(category) => category,
            Err(e) => {
                debug!("cache read error: {e}");
                let category =
                    sqlx::query_as!(entity::Category, "select * from category where id = $1", id)
                        .fetch_one(&state.db_pool)
                        .await
                        .map_err(map_err)?;

                // update cache
                let category = Category::from(category);

                let req = UpsertCategoryRequest {
                    category: Some(category.clone()),
                    event: CategoryEvent::Create.into(),
                };

                let mut buf = Vec::new();
                req.encode(&mut buf).map_err(map_err)?;

                let subject = format!("{}.update.set", self.subject);

                let _ = self
                    .state
                    .jetstream_context
                    .publish(subject, buf.into())
                    .await;

                category
            }
        };

        Ok(tonic::Response::new(category))
    }

    #[doc = " get subcategories"]
    #[must_use]
    #[tracing::instrument(skip(self), err(Debug))]
    async fn sub_categories(
        &self,
        request: tonic::Request<GetSubCategoriesRequest>,
    ) -> Result<tonic::Response<Connection>, tonic::Status> {
        todo!()
    }
}

async fn read_cache(
    cache_key: CacheKey<'_>,
    mut cache: PooledConnection<'_>,
) -> Result<Connection, tonic::Status> {
    cache
        .get::<_, Vec<u8>>(&cache_key)
        .map_err(|e| tonic::Status::internal(e.to_string()))
        .and_then(|payload| async move {
            if payload.is_empty() {
                let err = "cache is corrupted, empty bytes";
                Err(tonic::Status::internal(err))
            } else {
                Connection::decode(payload.as_ref())
                    .map_err(|e| tonic::Status::internal(e.to_string()))
            }
        })
        .await
}

fn parse_categories(
    count_on_other_end: Option<i64>,
    categories: Vec<Category>,
    pagination: &Cursor,
    actual_count: i32,
) -> Result<Connection, tonic::Status> {
    let user_count = actual_count as usize;

    let count_on_other_end = count_on_other_end
        .ok_or_else(|| tonic::Status::new(tonic::Code::Internal, "count returned no items"))?;
    let left_side = CursorBuilder::is_paginating_from_left(pagination);
    let cursor_unavailable = CursorBuilder::is_cursor_unavailable(pagination);

    let len = categories.len();

    let has_more = len > user_count;

    let categories = if has_more {
        categories.into_iter().take(user_count).collect()
    } else {
        categories
    };

    let connection = Connection {
        edges: {
            let categories: Result<Vec<_>, _> = categories
                .into_iter()
                .map(|category| {
                    to_offset_datetime(category.created_at)
                        .map_err(|_| tonic::Status::invalid_argument("timestamp is invalid"))
                        .and_then(|result| {
                            result
                                .to_offset(UtcOffset::UTC)
                                .format(&Rfc3339)
                                .map(|dt| {
                                    let cursor = CursorBuilder::new(&category.id, &dt);
                                    Node {
                                        node: Some(category),
                                        cursor: cursor.encode(),
                                    }
                                })
                                .map_err(map_err)
                        })
                })
                .collect();
            let categories = categories?;
            categories
        },

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

    Ok(connection)
}
