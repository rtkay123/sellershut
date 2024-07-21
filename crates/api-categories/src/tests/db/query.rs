use fake::{Fake, Faker};
use sellershut_core::{
    categories::{
        mutate_categories_server::MutateCategories, query_categories_server::QueryCategories,
    },
    common::{pagination::Cursor, request::SearchQueryOptional},
};
use sqlx::PgPool;
use tonic::IntoRequest;

use crate::{api::entity::Category, tests::TestApp};

#[sqlx::test(migrations = "./migrations")]
async fn db_query_all_no_pagination(pg_pool: PgPool) -> sqlx::Result<()> {
    let app = TestApp::new(pg_pool).await;
    let state = &app.state;

    let cursor = Cursor::default();

    let query_err = state.categories(cursor.into_request()).await;
    assert!(query_err.is_err());

    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn db_query_sub_no_pagination(pg_pool: PgPool) -> sqlx::Result<()> {
    let app = TestApp::new(pg_pool).await;
    let state = &app.state;

    let params = SearchQueryOptional::default();

    let query_err = state.sub_categories(params.into_request()).await;
    assert!(query_err.is_err());

    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn db_query_all_no_cursor(pg_pool: PgPool) -> sqlx::Result<()> {
    let app = TestApp::new(pg_pool).await;
    let state = &app.state;

    let category: Category = Faker.fake();
    dbg!(&category);

    let category = sellershut_core::categories::Category::from(category);

    let _create_res = state
        .create(category.clone().into_request())
        .await
        .expect("category to be created")
        .into_inner();

    let cursor = Cursor {
        cursor_value: None,
        index: Some(sellershut_core::common::pagination::cursor::Index::First(2)),
    };

    let query_ok = state.categories(cursor.into_request()).await;

    assert!(query_ok.is_ok());

    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn db_query_sub_no_cursor(pg_pool: PgPool) -> sqlx::Result<()> {
    let app = TestApp::new(pg_pool).await;
    let state = &app.state;

    let category: Category = Faker.fake();
    dbg!(&category);

    let category = sellershut_core::categories::Category::from(category);

    let _create_res = state
        .create(category.clone().into_request())
        .await
        .expect("category to be created")
        .into_inner();

    let cursor = SearchQueryOptional {
        query: None,
        pagination: Some(
            sellershut_core::common::request::search_query_optional::Pagination::Cursor(Cursor {
                cursor_value: None,
                index: Some(sellershut_core::common::pagination::cursor::Index::First(2)),
            }),
        ),
    };

    let query_ok = state.sub_categories(cursor.into_request()).await;

    assert!(query_ok.is_ok());

    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn db_query_all_after_pagination(pg_pool: PgPool) -> sqlx::Result<()> {
    let app = TestApp::new(pg_pool).await;
    let state = &app.state;

    let category: Category = Faker.fake();
    dbg!(&category);

    let category = sellershut_core::categories::Category::from(category);

    let _create_res = state
        .create(category.clone().into_request())
        .await
        .expect("category to be created")
        .into_inner();

    let cursor = Cursor {
        cursor_value: None,
        index: Some(sellershut_core::common::pagination::cursor::Index::First(2)),
    };

    let query_ok = state
        .categories(cursor.into_request())
        .await
        .expect("query to be ok")
        .into_inner();

    let cursor_str = &query_ok.edges[0].cursor;

    let cursor = Cursor {
        cursor_value: Some(sellershut_core::common::pagination::cursor::CursorValue {
            cursor_type: Some(
                sellershut_core::common::pagination::cursor::cursor_value::CursorType::After(
                    cursor_str.to_owned(),
                ),
            ),
        }),
        index: Some(sellershut_core::common::pagination::cursor::Index::First(2)),
    };

    let query_ok = state.categories(cursor.into_request()).await;
    assert!(query_ok.is_ok());

    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn db_query_sub_after_pagination(pg_pool: PgPool) -> sqlx::Result<()> {
    let app = TestApp::new(pg_pool).await;
    let state = &app.state;

    let category: Category = Faker.fake();
    dbg!(&category);

    let category = sellershut_core::categories::Category::from(category);

    let _create_res = state
        .create(category.clone().into_request())
        .await
        .expect("category to be created")
        .into_inner();

    let cursor = Cursor {
        cursor_value: None,
        index: Some(sellershut_core::common::pagination::cursor::Index::First(2)),
    };

    let query_ok = state
        .categories(cursor.into_request())
        .await
        .expect("query to be ok")
        .into_inner();

    let cursor_str = &query_ok.edges[0].cursor;

    let cursor = SearchQueryOptional {
        query: None,
        pagination: Some(
            sellershut_core::common::request::search_query_optional::Pagination::Cursor(Cursor {
                cursor_value: Some(sellershut_core::common::pagination::cursor::CursorValue {
                    cursor_type: Some(sellershut_core::common::pagination::cursor::cursor_value::CursorType::After(cursor_str.to_owned())),
                }),
                index: Some(sellershut_core::common::pagination::cursor::Index::First(2)),
            }),
        ),
    };

    let query_ok = state.sub_categories(cursor.into_request()).await;
    assert!(query_ok.is_ok());

    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn db_query_all_before_pagination(pg_pool: PgPool) -> sqlx::Result<()> {
    let app = TestApp::new(pg_pool).await;
    let state = &app.state;

    let category: Category = Faker.fake();
    dbg!(&category);

    let category = sellershut_core::categories::Category::from(category);

    let _create_res = state
        .create(category.clone().into_request())
        .await
        .expect("category to be created")
        .into_inner();

    let cursor = Cursor {
        cursor_value: None,
        index: Some(sellershut_core::common::pagination::cursor::Index::First(2)),
    };

    let query_ok = state
        .categories(cursor.into_request())
        .await
        .expect("query to be ok")
        .into_inner();

    let cursor_str = &query_ok.edges[0].cursor;

    let cursor = Cursor {
        cursor_value: Some(sellershut_core::common::pagination::cursor::CursorValue {
            cursor_type: Some(
                sellershut_core::common::pagination::cursor::cursor_value::CursorType::Before(
                    cursor_str.to_owned(),
                ),
            ),
        }),
        index: Some(sellershut_core::common::pagination::cursor::Index::Last(2)),
    };

    let query_ok = state.categories(cursor.into_request()).await;
    assert!(query_ok.is_ok());

    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn db_query_sub_before_pagination(pg_pool: PgPool) -> sqlx::Result<()> {
    let app = TestApp::new(pg_pool).await;
    let state = &app.state;

    let category: Category = Faker.fake();
    dbg!(&category);

    let category = sellershut_core::categories::Category::from(category);

    let _create_res = state
        .create(category.clone().into_request())
        .await
        .expect("category to be created")
        .into_inner();

    let cursor = Cursor {
        cursor_value: None,
        index: Some(sellershut_core::common::pagination::cursor::Index::First(2)),
    };

    let query_ok = state
        .categories(cursor.into_request())
        .await
        .expect("query to be ok")
        .into_inner();

    let cursor_str = &query_ok.edges[0].cursor;

    let cursor = SearchQueryOptional {
        query: None,
        pagination: Some(
            sellershut_core::common::request::search_query_optional::Pagination::Cursor(Cursor {
                cursor_value: Some(sellershut_core::common::pagination::cursor::CursorValue {
                    cursor_type: Some(sellershut_core::common::pagination::cursor::cursor_value::CursorType::Before(cursor_str.to_owned())),
                }),
                index: Some(sellershut_core::common::pagination::cursor::Index::Last(2)),
            }),
        ),
    };

    let query_ok = state.sub_categories(cursor.into_request()).await;
    assert!(query_ok.is_ok());

    Ok(())
}
