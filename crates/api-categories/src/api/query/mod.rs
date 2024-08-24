use async_graphql::{
    connection::{Connection, Edge, EmptyFields},
    Context, MergedObject, Object, Result,
};
use sellershut_core::{
    categories::{
        query_categories_server::QueryCategories, GetCategoryRequest, GetSubCategoriesRequest,
    },
    common::pagination::{
        cursor::{cursor_value::CursorType, CursorValue, Index},
        Cursor,
    },
};
use tonic::IntoRequest;
use tracing::instrument;

use crate::{api::entity::Category, state::ApiState};

#[derive(Default, Debug, MergedObject)]
pub struct Query(GraphqlQuery);

#[derive(Default, Debug)]
pub struct GraphqlQuery;

#[Object]
impl GraphqlQuery {
    #[instrument(skip(self, ctx), err(Debug))]
    async fn categories(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1))] after: Option<String>,
        #[graphql(validator(min_length = 1))] before: Option<String>,
        #[graphql(validator(minimum = 1, maximum = 100))] first: Option<i32>,
        #[graphql(validator(minimum = 1, maximum = 100))] last: Option<i32>,
    ) -> Result<Connection<String, Category, EmptyFields, EmptyFields>> {
        let pagination = Params::parse(after, before, first, last)?;

        let service = ctx.data::<ApiState>()?;

        let res = service
            .categories(pagination.into_request())
            .await?
            .into_inner();

        let page_info = res.page_info.as_ref().expect("page_info to be defined");

        let mut conn = Connection::new(page_info.has_previous_page, page_info.has_next_page);

        conn.edges = res
            .edges
            .into_iter()
            .map(|f| {
                Edge::new(
                    f.cursor,
                    Category::from(f.node.expect("category to be some")),
                )
            })
            .collect();

        Ok(conn)
    }

    #[instrument(skip(self, ctx), err(Debug))]
    async fn sub_categories(
        &self,
        ctx: &Context<'_>,
        parent_id: Option<String>,
        #[graphql(validator(min_length = 1, max_length = 100))] after: Option<String>,
        #[graphql(validator(min_length = 1, max_length = 100))] before: Option<String>,
        #[graphql(validator(minimum = 1, maximum = 100))] first: Option<i32>,
        #[graphql(validator(minimum = 1, maximum = 100))] last: Option<i32>,
    ) -> Result<Connection<String, Category, EmptyFields, EmptyFields>> {
        let pagination = Params::parse(after, before, first, last)?;

        let service = ctx.data::<ApiState>()?;

        let req = GetSubCategoriesRequest {
            id: parent_id,
            pagination: Some(pagination),
        };

        let res = service
            .sub_categories(req.into_request())
            .await?
            .into_inner();

        let page_info = res.page_info.as_ref().expect("page_info to be defined");

        let mut conn = Connection::new(page_info.has_previous_page, page_info.has_next_page);

        conn.edges = res
            .edges
            .into_iter()
            .map(|f| {
                Edge::new(
                    f.cursor,
                    Category::from(f.node.expect("category to be some")),
                )
            })
            .collect();

        Ok(conn)
    }

    #[instrument(skip(ctx), err(Debug))]
    async fn category_by_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 21, max_length = 21))] id: String,
    ) -> async_graphql::Result<Option<Category>> {
        let service = ctx.data::<ApiState>()?;
        let request = GetCategoryRequest { id };

        let res = service
            .category_by_id(request.into_request())
            .await?
            .into_inner();

        Ok(Some(Category::from(res)))
    }
}

/// Relay-compliant connection parameters to page results by cursor/page size
pub struct Params;

impl Params {
    pub fn parse(
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> async_graphql::Result<Cursor> {
        if (last.is_some() && after.is_some()) || (before.is_some() && first.is_some()) {
            return Err("invalid pagination arguments. Backwards pagination needs 'last' and 'before'. Forward pagination uses 'first' and (optionally) 'after'".into());
        }
        if last.is_none() && first.is_none() {
            return Err("One of 'first' or 'last' should be provided".into());
        }

        if after.is_some() && before.is_some() {
            return Err("Only one or none of 'after' or 'before' should be provided".into());
        }

        Ok(Cursor {
            cursor_value: if after.is_some() || before.is_some() {
                Some(CursorValue {
                    cursor_type: Some(match after {
                        Some(cursor) => CursorType::After(cursor),
                        None => CursorType::Before(before.expect("before should be defined")),
                    }),
                })
            } else {
                None
            },
            index: Some(if let Some(first) = first {
                Index::First(first)
            } else {
                Index::Last(last.expect("last to be some"))
            }),
        })
    }
}
