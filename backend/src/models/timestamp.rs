use crate::schema::timestamps;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, diesel::Queryable, diesel::Insertable)]
#[table_name = "timestamps"]
pub struct Timestamp {
    id: Uuid,
    created_at: DateTime<Utc>,
}
