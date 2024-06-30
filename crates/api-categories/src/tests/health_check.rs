use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use sqlx::PgPool;

use crate::tests::TestApp;

#[sqlx::test(migrations = "./migrations")]
async fn test_health_check_ok(pg_pool: PgPool) -> sqlx::Result<()> {
    let app = TestApp::new(pg_pool).await;

    let req = Request::get("/health").body(Body::empty()).unwrap();
    let resp = app.request(req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    /*
    let headers = resp.headers().clone();
    assert!(headers.get("x-request-id").is_some());
    assert_eq!(headers.get("access-control-allow-origin").unwrap(), "*");
    assert!(headers.get("vary").is_some()); */

    Ok(())
}
