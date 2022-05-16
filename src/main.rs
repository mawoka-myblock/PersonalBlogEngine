pub mod actions;
pub mod db;
pub mod models;
pub mod routes;
pub mod schema;
pub mod search;

extern crate chrono;

#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate diesel_migrations;



use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::{embed_migrations};
use actix_cors::Cors;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

embed_migrations!();
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    // let manager = ConnectionManager::<SqliteConnection>::new(conn_spec);
    /*    let pool = r2d2::Pool::builder()
    .build(manager)
    .expect("Failed to create pool.");*/
    // let private_key = actix_web::cookie::Key::generate();
    // let redis_uri = std::env::var("REDIS_URL").expect("REDIS_URL");
    let pool = db::get_pool();

    let conn = pool.get().unwrap();
    embedded_migrations::run(&conn).unwrap();

    HttpServer::new(move || {
        let policy = CookieIdentityPolicy::new(&[0; 32])
            .name("auth-cookie")
            .secure(false);
        let cors = Cors::default()
              .allow_any_origin();

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
            .wrap(cors)
            .service(
                web::scope("/api/v1")
                    .service(
                        web::scope("/manage")
                            .service(routes::manage::create_post) // POST create_post AUTH
                            .service(routes::manage::delete_post) // DELETE post?slug=slug AUTH
                            .service(routes::manage::setup) // POST setup
                            .service(routes::manage::update_post) // PUT update AUTH
                            .service(routes::manage::check_setup) // GET setup
                            .service(routes::manage::get_posts) // GET posts AUTH
                            .service(routes::manage::get_post), // GET post?slug=slug AUTH
                    )
                    .service(
                        web::scope("/account")
                            .service(routes::account::login) // POST login
                            .service(routes::account::logout) // POST logout AUTH
                            .service(routes::account::check_login_status), // GET check AUTH
                    )
                    .service(
                        web::scope("/public")
                            .service(routes::public::get_rendered_markdown) // GET rendered?slug=slug
                            .service(routes::public::get_raw_markdown) // GET raw?slug=slug
                            .service(routes::public::get_posts) // GET post?offset=0
                            .service(routes::public::get_posts_with_tag) // GET post/{tag}?offset=0
                            .service(routes::public::search_posts), // GET search?q=query
                    ),
            )
            .service(
                web::scope("/admin")
                    .service(routes::dashboard::index)
                    .service(routes::dashboard::dist),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
