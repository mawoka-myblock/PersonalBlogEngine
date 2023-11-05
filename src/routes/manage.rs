use actix_identity::Identity;
use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use serde::Deserialize;
use std::sync::Mutex;

use crate::models::NewPost;
use crate::search::update_index;
use crate::DbPool;
use crate::{actions, SearchData};

#[derive(Deserialize)]
pub struct QueryMarkdown {
    pub markdown: bool,
}

#[post("/create_post")]
pub async fn create_post(
    pool: web::Data<DbPool>,
    pool2: web::Data<DbPool>,
    data: web::Json<NewPost>,
    query: web::Query<QueryMarkdown>,
    search_data: web::Data<Mutex<SearchData>>,
    id: Option<Identity>,
) -> Result<HttpResponse, Error> {
    if id.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    };
    let user = web::block(move || {
        let mut conn = pool.get()?;
        actions::create_new_post(&data, query.0.markdown, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorConflict)?;

    let mut conn = web::block(move || pool2.get())
        .await?
        .map_err(actix_web::error::ErrorConflict)?;

    update_index(&mut conn, search_data);
    Ok(HttpResponse::Ok().json(user))
}

#[derive(Deserialize)]
pub struct QueryDelete {
    pub slug: String,
}

#[delete("/post")]
pub async fn delete_post(
    pool: web::Data<DbPool>,
    query: web::Query<QueryDelete>,
    id: Option<Identity>,
) -> Result<HttpResponse, Error> {
    if id.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    };
    let slug = query.slug.to_string();
    web::block(move || {
        let mut conn = pool.get()?;
        actions::delete_post(&slug, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorNotFound)?;
    Ok(HttpResponse::Ok().finish())
}

#[get("/post")]
pub async fn get_post(
    pool: web::Data<DbPool>,
    query: web::Query<QueryDelete>,
    id: Option<Identity>,
) -> Result<HttpResponse, Error> {
    if id.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    };
    let slug = query.slug.to_string();
    let post = web::block(move || {
        let mut conn = pool.get()?;
        actions::get_single_post(&slug, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorNotFound)?;
    Ok(HttpResponse::Ok().json(post))
}

#[derive(Deserialize, Debug)]
pub struct SetupData {
    pub email: String,
    pub password: String,
}

#[post("/setup")]
pub async fn setup(
    pool: web::Data<DbPool>,
    data: web::Json<SetupData>,
) -> Result<HttpResponse, Error> {
    // Creating 2 connections, because of the move-thing
    let mut conn = pool
        .get()
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let mut conn2 = pool
        .get()
        .map_err(actix_web::error::ErrorInternalServerError)?;
    // get number of users in db to determine if the setup has already been completed
    let user_count = web::block(move || actions::count_users(&mut conn))
        .await?
        .map_err(actix_web::error::ErrorConflict)?;
    // Check if setup is already completed
    if user_count != 0 {
        return Ok(HttpResponse::BadRequest().body("Setup already completed"));
    }
    web::block(move || actions::setup(&data.email, &data.password, &mut conn2))
        .await?
        .map_err(actix_web::error::ErrorConflict)?;
    Ok(HttpResponse::Ok().finish())
}

#[get("/setup")]
pub async fn check_setup(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let mut conn = pool
        .get()
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let user_count = web::block(move || actions::count_users(&mut conn))
        .await?
        .map_err(actix_web::error::ErrorConflict)?;
    if user_count == 0 {
        return Ok(HttpResponse::BadRequest().body("Setup not completed"));
    }
    Ok(HttpResponse::Ok().body("Setup already completed"))
}

#[derive(Deserialize)]
pub struct UpdatePost {
    pub slug: String,
    pub title: Option<String>,
    pub content: Option<String>,
    pub published: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub intro: Option<String>,
}

#[put("/update")]
pub async fn update_post(
    pool: web::Data<DbPool>,
    pool2: web::Data<DbPool>,
    data: web::Json<UpdatePost>,
    query: web::Query<QueryMarkdown>,
    search_data: web::Data<Mutex<SearchData>>,
    id: Option<Identity>,
) -> Result<HttpResponse, Error> {
    if id.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    };
    let user = web::block(move || {
        let mut conn = pool.get()?;
        actions::update_post(
            &data.slug,
            &data.content,
            &data.title,
            &data.published,
            query.0.markdown,
            &data.tags,
            &data.intro,
            &mut conn,
        )
    })
    .await?
    .map_err(actix_web::error::ErrorConflict)?;

    let mut conn = web::block(move || pool2.get())
        .await?
        .map_err(actix_web::error::ErrorConflict)?;

    update_index(&mut conn, search_data);

    Ok(HttpResponse::Ok().json(user))
}

#[derive(Deserialize)]
pub struct GetPostsQuery {
    pub offset: i64,
}

#[get("/posts")]
pub async fn get_posts(
    pool: web::Data<DbPool>,
    query: web::Query<GetPostsQuery>,
    id: Option<Identity>,
) -> Result<HttpResponse, Error> {
    if id.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    };
    let posts = web::block(move || {
        let mut conn = pool.get()?;
        actions::get_all_posts(&query.offset, false, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(posts))
}
