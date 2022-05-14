use dotenv::dotenv;
use std::env;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, self};
use r2d2::Pool;
use crate::DbPool;

/*pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}*/

// pub struct AppState {
//     pub db: PostgresPool,
// }

pub fn get_pool() -> DbPool {
   // it from the environment within this function
   dotenv().ok();
   let url = env::var("DATABASE_URL").expect("no DB URL");
   let migr = ConnectionManager::<PgConnection>::new(url);
   Pool::builder()
       .build(migr)
       .expect("could not build connection pool")
}
