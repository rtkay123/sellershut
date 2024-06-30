tonic::include_proto!("categories");

/// Category file descriptor
pub const CATEGORY_FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("categories_descriptor");

#[allow(non_snake_case)]
#[allow(missing_docs)]
#[cfg_attr(feature = "async-graphql", async_graphql::Object)]
impl Category {
    /// A unique identifier, it should be an 21 character ID
    pub async fn id(&self) -> &str {
        &self.id
    }

    /// Human readable name
    pub async fn name(&self) -> &str {
        &self.name
    }

    /// An optional image denoting this category
    pub async fn imageUrl(&self) -> Option<&str> {
        self.image_url.as_deref()
    }

    /// A list of sub categories of this current category
    pub async fn sub_categories(&self) -> &[String] {
        &self.sub_categories
    }

    /// The direct parent of this category (if applicable)
    pub async fn parentId(&self) -> Option<&str> {
        self.parent_id.as_deref()
    }

    /// Timestamp (millisecond precision) indicating when this category was created
    pub async fn created_at(&self) -> i64 {
        self.created_at
    }

    /// Timestamp (millisecond precision) indicating when this category was last updated
    pub async fn updated_at(&self) -> i64 {
        self.updated_at
    }
}
