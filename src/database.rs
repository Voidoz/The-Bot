mod models;
mod schema;

use diesel::{
    prelude::*,
    r2d2::{ ConnectionManager, Pool },
    result::Error,
    sqlite::SqliteConnection
};
use std::env;
use dotenvy::dotenv;

pub fn get_connection_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");

    let manager = ConnectionManager::<SqliteConnection>::new(url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

pub fn get_config(con: &mut SqliteConnection) -> Result<models::Config, Error> {
    use schema::config::dsl::*;

    config.first::<models::Config>(con)
}