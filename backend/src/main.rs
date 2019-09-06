#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

mod actions;
mod excel;
mod models;
mod schema;

use actions::{add_timestamp::add_timestamp, get_timestamps::get_timestamps};
use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use dotenv::dotenv;
use std::{env, error};

pub fn establish_connection() -> Result<Pool<ConnectionManager<PgConnection>>, Box<dyn error::Error>>
{
    let database_url = env::var("DATABASE_URL")?;
    let cm = ConnectionManager::new(database_url);
    let pool = Pool::new(cm)?;
    Ok(pool)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    dotenv().ok();
    pretty_env_logger::init();

    info!("Starting...");

    let pool = establish_connection()?;

    info!("DB connection established");

    let addr = "0.0.0.0:3001";

    info!("Started. Listening on http://{}", addr);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .wrap(
                Cors::new()
                    // Next line commented out defaults to `"All"` origins
                    // .allowed_origin("http://localhost:8080")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
                    .max_age(3600),
            )
            .service(
                web::scope("/api")
                    .route("/add-timestamp", web::post().to_async(add_timestamp))
                    .route("/get-timestamps", web::get().to_async(get_timestamps)),
            )
    })
    .bind(&addr)
    .expect("Can not bind to port 8000")
    .run()
    .unwrap();

    Ok(())
}
