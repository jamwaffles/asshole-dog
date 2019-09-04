use crate::{models::Timestamp, schema::timestamps};
use actix_web::{web, Error as WebError, HttpResponse};
use diesel::{
    pg::PgConnection,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use futures::future::Future;

#[derive(serde_derive::Serialize)]
struct TimestampsResponse {
    timestamps: Vec<Timestamp>,
}

/// Get all timestamps, ordered ascending (oldest first)
pub fn get_timestamps(
    conn: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Future<Item = HttpResponse, Error = WebError> {
    web::block(move || {
        let c = conn
            .get()
            .map_err(|e| error!("Error connecting to DB: {}", e))?;

        timestamps::table
            .order(timestamps::dsl::created_at.asc())
            .load::<Timestamp>(&c)
            .map_err(|e| error!("Failed to get all timestamps: {}", e))
    })
    .map_err(|e| e.into())
    .and_then(|timestamps| HttpResponse::Ok().json(TimestampsResponse { timestamps }))
}
