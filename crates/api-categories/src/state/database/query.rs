use core_services::cache::{PoolLike, PooledConnectionLike};
use futures_util::{FutureExt, TryFutureExt};
use prost::Message;
use sellershut_core::{
    categories::{
        query_categories_server::QueryCategories, Category, CategoryEvent, Connection,
        GetCategoryRequest, GetSubCategoriesRequest, UpsertCategoryRequest,
    },
    common::pagination::{self, cursor::cursor_value::CursorType, CursorBuilder},
};
use tracing::{debug, error, info_span, warn, Instrument};

use crate::{
    api::entity::{self},
    state::ApiState,
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
        let mut cache = self.state.cache.get().await.unwrap();

        let cache_result = cache
            .get::<_, Vec<Vec<u8>>>("categories:cursor:after")
            .map_err(|e| tonic::Status::internal(e.to_string()))
            .and_then(|payload| async move {
                if payload.is_empty() || payload.iter().any(|value| value.is_empty()) {
                    let err = "cache is corrupted, empty bytes";
                    Err(tonic::Status::internal(err))
                } else {
                    let results: Result<Vec<_>, _> = payload
                        .iter()
                        .map(|value| {
                            Category::decode(value.as_ref())
                                .map_err(|e| tonic::Status::internal(e.to_string()))
                        })
                        .collect();
                    results
                }
            })
            .await;

        let categories = match cache_result {
            Ok(result) => result,
            Err(e) => {
                error!("cache read {e}");

                let subject = format!("{}.update.set", self.subject);

                let _ = self
                    .state
                    .jetstream_context
                    .publish(subject, "data".into())
                    .await;
                debug!("message published");

                //proceed with db call
                todo!()
            }
        };
        todo!()

        /* let _ = self
        .state
        .jetstream_context
        .publish("categories.create", "data".into())
        .await; */
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

        let cache_key = format!("category:id:{id}");

        let s = info_span!("cache call");

        // get cache first
        let mut cache = self.state.cache.get().await.unwrap();
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
                        .unwrap();

                // update cache
                let category = Category::from(category);

                let req = UpsertCategoryRequest {
                    category: Some(category.clone()),
                    event: CategoryEvent::Create.into(),
                };

                let mut buf = Vec::new();
                req.encode(&mut buf).expect("Failed to encode message");

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
