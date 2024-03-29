use crate::DbPool;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;
use r2d2::Pool;
use std::env;

pub fn get_pool() -> DbPool {
    // it from the environment within this function
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("no DB URL");
    let migr = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .build(migr)
        .expect("could not build connection pool")
}
