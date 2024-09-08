use sqlx::PgPool;
use tokio::sync::oneshot;

use crate::utils::TestApp;

#[sqlx::test(migrations = "./migrations")]
async fn test_crate(pg_pool: PgPool) -> sqlx::Result<()> {
    let (tx, rx) = oneshot::channel();
    let _app = TestApp::new(pg_pool, tx).await;

    let port = rx.await.unwrap();
    let address = format!("http://127.0.0.1:{port}");

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{address}/health"))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    Ok(())
}
