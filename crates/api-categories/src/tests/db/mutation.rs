use fake::{Fake, Faker};
use sellershut_core::{
    categories::mutate_categories_server::MutateCategories, common::id_gen::ID_LENGTH,
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
