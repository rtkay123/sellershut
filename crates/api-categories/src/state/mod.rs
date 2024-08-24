pub mod multiplex;

use std::path::Path;

use core_services::state::ServiceState;
use sellershut_core::{
    categories::{
        query_categories_server::QueryCategories, Category, Connection, GetCategoryRequest,
        GetSubCategoriesRequest,
    },
    common::pagination,
};

#[derive(Clone)]
pub struct ApiState(pub ServiceState);

impl ApiState {
    pub async fn initialise() -> anyhow::Result<Self> {
        let man_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
        dotenvy::from_path(man_path).ok();

        let state = ServiceState::initialise(env!("CARGO_CRATE_NAME")).await?;

        //        #[cfg(not(test))]
        //        sqlx::migrate!("./migrations").run(&state.db_pool).await?;

        Ok(Self(state))
    }
}

#[tonic::async_trait]
impl QueryCategories for ApiState {
    #[doc = " gets all categories"]
    #[must_use]
    async fn categories(
        &self,
        request: tonic::Request<pagination::Cursor>,
    ) -> Result<tonic::Response<Connection>, tonic::Status> {
        todo!()
    }

    #[doc = " get category by id"]
    #[must_use]
    async fn category_by_id(
        &self,
        request: tonic::Request<GetCategoryRequest>,
    ) -> Result<tonic::Response<Category>, tonic::Status> {
        todo!()
    }

    #[doc = " get subcategories"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn sub_categories(
        &self,
        request: tonic::Request<GetSubCategoriesRequest>,
    ) -> Result<tonic::Response<Connection>, tonic::Status> {
        todo!()
    }
}
