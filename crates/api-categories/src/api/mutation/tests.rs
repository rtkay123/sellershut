use sellershut_core::common::id_gen::generate_id;
use sqlx::PgPool;

use crate::tests::TestApp;

#[sqlx::test(migrations = "./migrations")]
async fn graphql_create(pg_pool: PgPool) -> sqlx::Result<()> {
    let app = TestApp::new(pg_pool).await;

    let schema = &app.schema;

    let res = schema
        .execute(
            r#"
            mutation {
              create(input: { name: "name", subCategories: []}) {
                id
              }
            }
           "#,
        )
        .await;

    assert!(res.errors.is_empty());

    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn graphql_update(pg_pool: PgPool) -> sqlx::Result<()> {
    let app = TestApp::new(pg_pool).await;

    let schema = &app.schema;

    let res = schema
        .execute(
            r#"
            mutation {
              update(input: { name: "name", subCategories: []}) {
                id
              }
            }
           "#,
        )
        .await;

    assert!(res.errors.is_empty());

    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn graphql_delete(pg_pool: PgPool) -> sqlx::Result<()> {
    let app = TestApp::new(pg_pool).await;

    let schema = &app.schema;
    let _id = generate_id();

    let res = schema
        .execute(
            r#"
            mutation {
              delete(id: "abcdefghijklmnopqrstu" ) {
                id
              }
            }
           "#,
        )
        .await;

    dbg!(&res);
    assert!(res.errors.is_empty());

    Ok(())
}
