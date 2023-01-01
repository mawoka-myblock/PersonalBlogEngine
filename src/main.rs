#![allow(clippy::extra_unused_lifetimes)]
#![allow(non_snake_case)]

pub mod actions;
pub mod db;
pub mod models;
pub mod routes;
pub mod schema;
pub mod search;

extern crate chrono;

use std::sync::Mutex;

#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate tantivy;

use crate::search::{get_schema, initialize_index};
use actix_cors::Cors;
use actix_form_data::{Error, Field, Form};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use diesel::prelude::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::embed_migrations;
use tantivy::schema::Schema;
use tantivy::{Index, LeasedItem, ReloadPolicy, Searcher};
use tempfile::TempDir;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub struct SearchData {
    pub index: Index,
    pub searcher: LeasedItem<Searcher>,
    pub schema: Schema,
}

embed_migrations!();
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let pool = db::get_pool();

    let conn = pool.get().unwrap();
    embedded_migrations::run(&conn).unwrap();
    let index_path = TempDir::new().unwrap();
    let index = Index::create_in_dir(&index_path, get_schema()).unwrap();
    let schema = get_schema();
    initialize_index(&index, &pool.get().unwrap());

    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()
        .unwrap();

    let searcher = reader.searcher();

    let data = Data::new(Mutex::new(SearchData {
        schema,
        searcher,
        index,
    }));

    HttpServer::new(move || {
        let policy = CookieIdentityPolicy::new(&[0; 32])
            .name("auth-cookie")
            .secure(false);
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

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
                web::scope("")
                    .app_data(data.clone())
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
                            )
                            .service(
                                web::scope("/feedback")
                                    .service(routes::feedback::post_feedback) // POST /
                                    .service(routes::feedback::get_feedback_from_post) // GET /post?limit=Int&post_id=UUID
                                    .service(routes::feedback::get_feedback_list), // GET /?limit=Int
                            )
                            .service(
                                web::scope("/uploads")
                                    .service(routes::uploads::list_files) // GET /list?offset=int
                                    .service(routes::uploads::upload_file) // POST /
                                    .service(routes::uploads::get_file) // GET /{file_id}
                                    .service(routes::uploads::delete_file) // DELETE /{file_id}

                            ),
                    )
                    .service(
                        web::scope("admin")
                            .service(routes::dashboard::index)
                            .service(routes::dashboard::dist),
                    ),
            )

        // .service(routes::dashboard::admin_index)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
