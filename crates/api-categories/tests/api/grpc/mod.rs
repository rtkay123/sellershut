use fake::{locales::EN, Fake};
use sellershut_core::{
    categories::{
        mutate_categories_client::MutateCategoriesClient,
        query_categories_client::QueryCategoriesClient, Category, CategoryEvent,
        DeleteCategoryRequest, GetCategoryRequest, UpsertCategoryRequest,
    },
    common::pagination::{cursor::Index, Cursor},
};
use sqlx::PgPool;
use tokio::sync::oneshot;
use tonic::IntoRequest;

use crate::utils::TestApp;

#[sqlx::test(migrations = "./migrations")]
async fn check_grpc_categories(pg_pool: PgPool) -> sqlx::Result<()> {
    let (tx, rx) = oneshot::channel();
    let _app = TestApp::new(pg_pool, tx).await;

    let port = rx.await.unwrap();
    let address = format!("http://127.0.0.1:{port}");

    let mut client = QueryCategoriesClient::connect(address).await.unwrap();

    let cursor = Cursor {
        cursor_value: None,
        index: Some(Index::First(10)),
    };

    let response = client.categories(cursor.into_request()).await.unwrap();

    let connection = response.into_inner();
    assert!(connection.page_info.is_some());

    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn check_grpc_crud(pg_pool: PgPool) -> sqlx::Result<()> {
    let (tx, rx) = oneshot::channel();
    let _app = TestApp::new(pg_pool, tx).await;

    let port = rx.await.unwrap();
    let address = format!("http://127.0.0.1:{port}");

    let mut client_mut = MutateCategoriesClient::connect(address.clone())
        .await
        .unwrap();
    let mut client = QueryCategoriesClient::connect(address).await.unwrap();

    let name: String = fake::faker::name::raw::Name(EN).fake();
    let category = Category {
        name: name.clone(),
        ..Default::default()
    };

    let request = UpsertCategoryRequest {
        category: Some(category),
        event: CategoryEvent::Create.into(),
    }
    .into_request();

    let create_result = client_mut.create(request).await.unwrap().into_inner();

    assert_eq!(21, create_result.id.len());
    assert_eq!(name, create_result.name);

    let req_by_id = GetCategoryRequest {
        id: create_result.id.clone(),
    };

    let read_result = client
        .category_by_id(req_by_id.clone().into_request())
        .await
        .unwrap()
        .into_inner();

    assert_eq!(read_result, create_result);

    let new_name: String = fake::faker::name::raw::Name(EN).fake();
    let mut update_data = create_result.clone();
    update_data.name = new_name.clone();

    let update_req = UpsertCategoryRequest {
        category: Some(update_data),
        event: CategoryEvent::Update.into(),
    }
    .into_request();

    let update_result = client_mut.update(update_req).await.unwrap().into_inner();

    assert_eq!(update_result.id, create_result.id);
    assert_eq!(update_result.created_at, update_result.created_at);
    assert_eq!(update_result.name, new_name);
    assert_ne!(update_result.updated_at, create_result.updated_at);

    let read_result = client
        .category_by_id(req_by_id.clone().into_request())
        .await
        .unwrap()
        .into_inner();
    assert_eq!(read_result, update_result);

    let delete_req = DeleteCategoryRequest {
        id: update_result.id,
        event: CategoryEvent::Delete.into(),
    }
    .into_request();
    client_mut.delete(delete_req).await.unwrap().into_inner();

    let read_result = client
        .category_by_id(req_by_id.clone().into_request())
        .await;

    assert!(read_result.is_err());

    Ok(())
}
