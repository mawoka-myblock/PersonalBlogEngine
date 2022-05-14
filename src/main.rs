pub mod schema;
pub mod models;
pub mod routes;
pub mod db;
pub mod actions;

extern crate chrono;


#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use actix_identity::{CookieIdentityPolicy, IdentityService};

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(conn_spec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let private_key = actix_web::cookie::Key::generate();
    let redis_uri = std::env::var("REDIS_URL").expect("REDIS_URL");

    HttpServer::new(move || {
        let policy = CookieIdentityPolicy::new(&[0; 32])
            .name("auth-cookie")
            .secure(false);
        App::new()
/*            .wrap(
                SessionMiddleware::new(
                    RedisActorSessionStore::new(redis_uri.clone()),
                    private_key.clone(),
                )
            )*/
            .wrap(IdentityService::new(policy))
            // .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("/manage")
                            .service(routes::manage::create_post) // POST create_post
                            .service(routes::manage::delete_post)// DELETE post?slug=slug
                            .service(routes::manage::setup) // POST setup
                    )
                    .service(web::scope("/account")
                        .service(routes::account::login) // POST login
                        .service(routes::account::logout) // POST logout
                        .service(routes::account::check_login_status) // GET check
                    )

            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
