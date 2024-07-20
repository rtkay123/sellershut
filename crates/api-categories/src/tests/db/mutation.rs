use fake::{Fake, Faker};
use sellershut_core::{
    categories::{
        mutate_categories_server::MutateCategories, query_categories_server::QueryCategories,
    },
    common::{id_gen::ID_LENGTH, SearchQuery},
};
use sqlx::PgPool;
use tonic::IntoRequest;

use crate::{api::entity::Category, tests::TestApp};

#[sqlx::test(migrations = "./migrations")]
async fn db_create(pg_pool: PgPool) -> sqlx::Result<()> {
    let app = TestApp::new(pg_pool).await;
    let state = &app.state;

    let category: Category = Faker.fake();
    dbg!(&category);

    let mut category = sellershut_core::categories::Category::from(category);

    let create_res = state
        .create(category.clone().into_request())
        .await
        .expect("category to be created")
        .into_inner();

    assert_eq!(ID_LENGTH, create_res.id.len());

    category.id = create_res.id.clone();
    category.created_at = create_res.created_at;
    category.updated_at = create_res.updated_at;

    assert_eq!(category, create_res);

    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn db_update(pg_pool: PgPool) -> sqlx::Result<()> {
    let app = TestApp::new(pg_pool).await;
    let state = &app.state;

    let category: Category = Faker.fake();

    let category = sellershut_core::categories::Category::from(category);
    dbg!(&category);

    let mut create_res = state
        .create(category.clone().into_request())
        .await
        .expect("category to be created")
        .into_inner();
    dbg!(&create_res);

    create_res.name = "New name".into();

    let mut update_res = state
        .update(create_res.clone().into_request())
        .await
        .expect("category to be updated")
        .into_inner();

    assert_eq!(ID_LENGTH, create_res.id.len());
    assert_eq!(create_res.id, update_res.id);
    assert_eq!(create_res.created_at, create_res.updated_at);
    assert_eq!(create_res.created_at, update_res.created_at);
    assert_ne!(create_res.updated_at, update_res.updated_at);
    assert_eq!(create_res.idx, update_res.idx);

    let id = SearchQuery {
        query: create_res.id.to_string(),
        ..Default::default()
    };
    let fetch_val = state
        .category_by_id(id.into_request())
        .await
        .unwrap()
        .into_inner();

    update_res.idx = fetch_val.idx;

    assert_eq!(update_res, fetch_val);

    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn db_delete(pg_pool: PgPool) -> sqlx::Result<()> {
    let app = TestApp::new(pg_pool).await;
    let state = &app.state;

    let category: Category = Faker.fake();
    dbg!(&category);

    let category = sellershut_core::categories::Category::from(category);

    let mut create_res = state
        .create(category.clone().into_request())
        .await
        .expect("category to be created")
        .into_inner();

    create_res.name = "New name".into();

    let id = SearchQuery {
        query: create_res.id.to_string(),
        ..Default::default()
    };

    let _update_res = state
        .delete(id.clone().into_request())
        .await
        .expect("category to be deleted")
        .into_inner();

    let fetch_val = state.category_by_id(id.into_request()).await;

    assert!(fetch_val.is_err());

    Ok(())
}
