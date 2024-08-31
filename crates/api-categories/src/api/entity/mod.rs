use async_graphql::{InputObject, SimpleObject};
use sellershut_core::google::protobuf::Timestamp;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::time::OffsetDateTime};

fn default_time() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

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
    #[graphql(default_with = "default_time()")]
    pub created_at: OffsetDateTime,
    #[graphql(default_with = "default_time()")]
    pub updated_at: OffsetDateTime,
}

fn to_offset_datetime(timestamp: Option<Timestamp>) -> OffsetDateTime {
    if let Some(timestamp) = timestamp {
        let duration = time::Duration::seconds(timestamp.seconds)
            + time::Duration::nanoseconds(timestamp.nanos.into());

        // Use Unix epoch (1970-01-01T00:00:00Z) as a starting point
        let epoch = OffsetDateTime::UNIX_EPOCH;

        // Add the duration to the Unix epoch
        epoch + duration
    } else {
        OffsetDateTime::now_utc()
    }
}

impl From<sellershut_core::categories::Category> for Category {
    fn from(value: sellershut_core::categories::Category) -> Self {
        Self {
            id: value.id,
            name: value.name,
            sub_categories: value.sub_categories,
            image_url: value.image_url,
            parent_id: value.parent_id,
            created_at: to_offset_datetime(value.created_at),
            updated_at: to_offset_datetime(value.updated_at),
        }
    }
}

fn to_timestamp(dt: OffsetDateTime) -> Timestamp {
    let duration_since_epoch = dt - OffsetDateTime::UNIX_EPOCH;
    Timestamp {
        seconds: duration_since_epoch.whole_seconds(),
        nanos: duration_since_epoch.whole_nanoseconds() as i32,
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
            updated_at: Some(to_timestamp(value.created_at)),
        }
    }
}
