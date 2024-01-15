use std::env;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{r2d2, SqliteConnection};
use dotenvy::dotenv;

pub mod models;
pub mod schema;
pub mod tasks_domain;
pub mod utils;

pub fn establish_connection() -> Pool<ConnectionManager<SqliteConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
