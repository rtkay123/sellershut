#[cfg(test)]
mod tests;

use std::time::{SystemTime, UNIX_EPOCH};

use sellershut_core::{
    categories::{mutate_categories_server::MutateCategories, Category},
    common::{
        id_gen::generate_id,
        request::{Empty, SearchQuery},
    },
};

use crate::state::{impls::map_err, ApiState};

#[tonic::async_trait]
impl MutateCategories for ApiState {
    #[doc = " Create a category"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn create(
        &self,
        request: tonic::Request<Category>,
    ) -> Result<tonic::Response<Category>, tonic::Status> {
        let category = request.into_inner();
        let id = generate_id();

        let now = SystemTime::now();

        // Calculate the duration since the epoch
        let duration_since_epoch = now.duration_since(UNIX_EPOCH).map_err(map_err)?.as_millis();

        // Check if the value fits within the range of i64
        if duration_since_epoch <= i64::MAX as u128 {
            let duration_since_epoch = duration_since_epoch as i64;
            sqlx::query!(
                "insert into category (id, name, sub_categories, image_url, parent_id, created_at, updated_at)
                values ($1, $2, $3, $4, $5, $6, $7)",
                &id,
                &category.name,
                &category.sub_categories,
                category.image_url,
                category.parent_id,
                duration_since_epoch,
                duration_since_epoch,
            ).execute(&self.0.db_pool).await.map_err(map_err)?;

            let mut resp = category;
            resp.id = id;
            resp.created_at = duration_since_epoch;
            resp.updated_at = duration_since_epoch;

            self.update_index(&resp).await;

            Ok(tonic::Response::new(resp))
        } else {
            Err(tonic::Status::new(
                tonic::Code::Internal,
                "Cannot convert u128 to i64: value exceeds i64 range",
            ))
        }
    }

    #[doc = " Update a category"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn update(
        &self,
        request: tonic::Request<Category>,
    ) -> Result<tonic::Response<Category>, tonic::Status> {
        let category = request.into_inner();

        let now = SystemTime::now();

        // Calculate the duration since the epoch
        let duration_since_epoch = now.duration_since(UNIX_EPOCH).map_err(map_err)?.as_millis();

        // Check if the value fits within the range of i64
        if duration_since_epoch <= i64::MAX as u128 {
            let duration_since_epoch = duration_since_epoch as i64;
            sqlx::query!(
                "update category set name = $2, sub_categories = $3, image_url = $4, parent_id = $5, updated_at = $6
                where id = $1",
                category.id,
                category.name,
                &category.sub_categories,
                category.image_url,
                category.parent_id,
                duration_since_epoch,
            ).execute(&self.0.db_pool).await.map_err(map_err)?;

            let mut resp = category;
            resp.updated_at = duration_since_epoch;

            self.update_index(&resp).await;

            Ok(tonic::Response::new(resp))
        } else {
            Err(tonic::Status::new(
                tonic::Code::Internal,
                "Cannot convert u128 to i64: value exceeds i64 range",
            ))
        }
    }

    #[doc = " Delete a category"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn delete(
        &self,
        request: tonic::Request<SearchQuery>,
    ) -> Result<tonic::Response<Empty>, tonic::Status> {
        let id = request.into_inner().query;

        sqlx::query!("delete from category where id = $1", id)
            .execute(&self.0.db_pool)
            .await
            .map_err(map_err)?;

        self.0.meilisearch_index.delete_document(&id).await.unwrap();

        Ok(tonic::Response::new(Empty::default()))
    }
}
