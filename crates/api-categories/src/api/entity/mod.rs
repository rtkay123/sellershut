use async_graphql::{InputObject, SimpleObject};
use sellershut_core::google::protobuf::Timestamp;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::time::OffsetDateTime};

fn default_time() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

#[derive(
    SimpleObject, InputObject, FromRow, Debug, Serialize, Deserialize, PartialEq, Eq, Clone,
)]
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
    #[graphql(default_with = "default_time()")]
    pub created_at: OffsetDateTime,
    #[graphql(default_with = "default_time()")]
    pub updated_at: OffsetDateTime,
}

pub fn to_offset_datetime(timestamp: Option<Timestamp>) -> async_graphql::Result<OffsetDateTime> {
    let timestamp = timestamp.ok_or(tonic::Status::invalid_argument("timestamp is missing"))?;
    let seconds = timestamp.seconds;
    let nanos = timestamp.nanos as i64;
    // Ensure the nanoseconds are within the valid range
    let nanoseconds = nanos % 1_000_000_000;
    // Create OffsetDateTime from seconds and nanoseconds
    let d = OffsetDateTime::from_unix_timestamp(seconds).map_err(|e| e.to_string())?
        + time::Duration::nanoseconds(nanoseconds);
    Ok(d)
}

impl TryFrom<sellershut_core::categories::Category> for Category {
    type Error = async_graphql::Error;

    fn try_from(value: sellershut_core::categories::Category) -> async_graphql::Result<Self> {
        Ok(Self {
            id: value.id,
            name: value.name,
            sub_categories: value.sub_categories,
            image_url: value.image_url,
            parent_id: value.parent_id,
            created_at: to_offset_datetime(value.created_at)?,
            updated_at: to_offset_datetime(value.updated_at)?,
        })
    }
}

fn to_timestamp(dt: OffsetDateTime) -> Timestamp {
    Timestamp {
        seconds: dt.unix_timestamp(),
        nanos: dt.nanosecond() as i32,
    }
}

impl From<Category> for sellershut_core::categories::Category {
    fn from(value: Category) -> Self {
        Self {
            id: value.id,
            name: value.name,
            sub_categories: value.sub_categories,
            image_url: value.image_url,
            parent_id: value.parent_id,
            created_at: Some(to_timestamp(value.created_at)),
            updated_at: Some(to_timestamp(value.updated_at)),
        }
    }
}

#[cfg(test)]
mod tests {
    use fake::{Fake, Faker};
    use sellershut_core::google::protobuf::Timestamp;
    use time::OffsetDateTime;

    use crate::api::entity::{to_offset_datetime, Category};

    #[test]
    fn convert_timestamp() {
        let dt = OffsetDateTime::now_utc();

        let res = Timestamp {
            seconds: dt.unix_timestamp(),
            nanos: dt.nanosecond() as i32,
        };

        let dt_2 = to_offset_datetime(Some(res)).unwrap();
        assert_eq!(dt, dt_2);
    }

    #[test]
    fn convert_category() {
        let category: Category = Faker.fake();

        let other_category = sellershut_core::categories::Category::from(category.clone());
        let original = Category::try_from(other_category).unwrap();

        assert_eq!(original, category);
    }
}
