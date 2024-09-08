use sellershut_core::{
    categories::query_categories_client::QueryCategoriesClient,
    common::pagination::{cursor::Index, Cursor},
};
use sqlx::PgPool;
use tokio::sync::oneshot;
use tonic::IntoRequest;
use tracing::info;

use crate::utils::TestApp;

#[sqlx::test(migrations = "./migrations")]
async fn check_grpc_categories(pg_pool: PgPool) -> sqlx::Result<()> {
    let (tx, rx) = oneshot::channel();
    info!("creating app");
    let _app = TestApp::new(pg_pool, tx).await;

    let port = rx.await.unwrap();
    info!("received port");
    let address = format!("http://127.0.0.1:{port}");

    info!("creating client");
    let mut client = QueryCategoriesClient::connect(address).await.unwrap();
    info!("created client");

    let cursor = Cursor {
        cursor_value: None,
        index: Some(Index::First(10)),
    };

    let response = client.categories(cursor.into_request()).await.unwrap();
    info!("got tresponse");

    let connection = response.into_inner();
    assert!(connection.page_info.is_some());

    Ok(())
}
