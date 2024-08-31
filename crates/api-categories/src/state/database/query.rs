use core_services::cache::{PoolLike, PooledConnectionLike};
use futures_util::FutureExt;
use prost::Message;
use sellershut_core::{
    categories::{
        query_categories_server::QueryCategories, Category, Connection, GetCategoryRequest,
        GetSubCategoriesRequest,
    },
    common::pagination::{self, cursor::cursor_value::CursorType, CursorBuilder},
};
use tracing::{error, warn};

use crate::state::ApiState;

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
            .then(|payload| async move {
                match payload {
                    Ok(payload) => {
                        if payload.is_empty() || payload.iter().any(|value| value.is_empty()) {
                            let err = "cache is corrupted, empty bytes";
                            Err(tonic::Status::internal(err))
                        } else {
                            let results:Result<Vec<_>,_> = payload
                                .iter()
                                .map(|value| {
                                    Category::decode(value.as_ref())
                                        .map_err(|e| tonic::Status::internal(e.to_string()))
                                })
                                .collect();
                            results
                        }
                    }
                    Err(e) => Err(tonic::Status::internal(e.to_string())),
                }
            })
            .await;

        let categories = match cache_result {
            Ok(result) => {
                result
            },
            Err(e) => {
                error!("cache read {e}");

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
        todo!()
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
