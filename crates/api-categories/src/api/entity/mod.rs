use async_graphql::{InputObject, SimpleObject};
use sqlx::prelude::FromRow;

#[derive(SimpleObject, InputObject, FromRow, Debug)]
#[graphql(input_name = "CategoryInput")]
#[cfg_attr(test, derive(fake::Dummy))]
pub struct Category {
    #[graphql(skip_input)]
    pub id: String,
    pub name: String,
    #[graphql(default)]
    pub sub_categories: Vec<String>,
    pub image_url: Option<String>,
    #[cfg_attr(test, dummy(default))]
    pub parent_id: Option<String>,
    #[graphql(skip_input)]
    pub created_at: i64,
    #[graphql(skip_input)]
    pub updated_at: i64,
    #[graphql(skip)]
    pub idx: i32,
}

impl From<sellershut_core::categories::Category> for Category {
    fn from(value: sellershut_core::categories::Category) -> Self {
        Self {
            id: value.id,
            idx: value.idx,
            name: value.name,
            sub_categories: value.sub_categories,
            image_url: value.image_url,
            parent_id: value.parent_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<Category> for sellershut_core::categories::Category {
    fn from(value: Category) -> Self {
        Self {
            id: value.id,
            idx: value.idx,
            name: value.name,
            sub_categories: value.sub_categories,
            image_url: value.image_url,
            parent_id: value.parent_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
