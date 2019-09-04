use super::OkResponse;
use crate::schema::timestamps;
use actix_web::{web, Error as WebError, HttpResponse};
use chrono::{DateTime, Utc};
use diesel::{
    pg::PgConnection,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use futures::future::Future;

#[derive(serde_derive::Deserialize, Debug)]
pub struct AddTimestampBody {
    timestamp: DateTime<Utc>,
}

/// Add a timestamp to the database
pub fn add_timestamp(
    conn: web::Data<Pool<ConnectionManager<PgConnection>>>,
    body: web::Json<AddTimestampBody>,
) -> impl Future<Item = HttpResponse, Error = WebError> {
    web::block(move || {
        let c = conn
            .get()
            .map_err(|e| error!("Error connecting to DB: {}", e))?;

        diesel::insert_into(timestamps::table)
            .values(timestamps::dsl::created_at.eq(body.timestamp))
            .execute(&c)
            .map_err(|e| error!("Failed to create timestamp: {}", e))
    })
    .map_err(|e| e.into())
    .and_then(|_| HttpResponse::Created().json(OkResponse { ok: true }))
}
