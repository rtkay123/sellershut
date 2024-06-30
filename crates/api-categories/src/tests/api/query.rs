use sqlx::PgPool;

use crate::tests::TestApp;

#[sqlx::test(migrations = "./migrations")]
async fn graphql_categories(pg_pool: PgPool) -> sqlx::Result<()> {
    let app = TestApp::new(pg_pool).await;

    let schema = &app.schema;

    let res = schema
        .execute(
            r#"
             query {
               categories(first: 2) {
                 edges {
                   cursor,
                   node {
                     id,
                     name
                   }
                 },
                 pageInfo {
                   hasNextPage,
                   hasPreviousPage
                 }
               }
             }
           "#,
        )
        .await;

    assert!(res.errors.is_empty());

    Ok(())
}
