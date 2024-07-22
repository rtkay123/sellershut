use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(SimpleObject, InputObject, FromRow, Debug, Serialize, Deserialize)]
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

#[derive(SimpleObject, FromRow, Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(fake::Dummy))]
pub struct CategorySearchResult {
    pub id: String,
    pub parent_name: Option<String>,
    pub category: Category,
}

#[derive(Serialize, Deserialize)]
pub struct CategoryWithParent {
    pub id: String,
    pub name: String,
    pub parent_name: Option<String>,
    pub sub_categories: Vec<String>,
    pub image_url: Option<String>,
    pub parent_id: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub idx: i32,
}

impl From<sellershut_core::categories::CategorySearchResult> for CategorySearchResult {
    fn from(value: sellershut_core::categories::CategorySearchResult) -> Self {
        Self {
            id: value.id,
            parent_name: value.parent_name,
            category: value.category.expect("category to be populated").into(),
        }
    }
}

impl From<CategorySearchResult> for sellershut_core::categories::CategorySearchResult {
    fn from(value: CategorySearchResult) -> Self {
        Self {
            id: value.id,
            parent_name: value.parent_name,
            category: Some(value.category.into()),
        }
    }
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
