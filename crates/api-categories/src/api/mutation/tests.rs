use sellershut_core::common::id::generate_id;
use sqlx::PgPool;

use crate::{api::ApiSchemaBuilder, state::ApiState, tests::TestApp};

#[sqlx::test(migrations = "./migrations")]
async fn test_graphql_mutation_create(pg_pool: PgPool) -> sqlx::Result<()> {
    let app = TestApp::new(pg_pool).await;
    let res = app
        .schema
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

    let res = app
        .schema
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

    // category to update does not exist
    assert!(!res.errors.is_empty());

    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn graphql_delete(pg_pool: PgPool) -> sqlx::Result<()> {
    let app = TestApp::new(pg_pool).await;

    let query = format!(
        r#"
            mutation {{
              delete(id: "{}") {{
                id
              }}
            }}
        "#,
        generate_id()
    );

    let res = app.schema.execute(&query).await;

    dbg!(&res);
    assert!(res.errors.is_empty());

    Ok(())
}
