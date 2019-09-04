use super::OkResponse;
use actix_web::{Error as WebError, HttpResponse};
use chrono::{DateTime, Utc};
use futures::future::{self, Future};

#[derive(serde_derive::Deserialize, Debug)]
struct AddTimestampBody {
    timestamp: DateTime<Utc>,
}

/// Add a timestamp to the database
pub fn add_timestamp() -> impl Future<Item = HttpResponse, Error = WebError> {
    future::ok(HttpResponse::Ok().json(OkResponse::default()))
}
