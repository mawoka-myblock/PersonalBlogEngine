use actix_web::{delete, put, Error, HttpResponse, post, web};
use serde::Deserialize;

use crate::{DbPool};
use crate::actions;
use crate::models::{NewPost};

#[post("/create_post")]
pub async fn create_post(pool: web::Data<DbPool>, data: web::Json<NewPost>) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let conn = pool.get()?;
        actions::create_new_post(&data, &conn)
    })
        .await?
        .map_err(actix_web::error::ErrorConflict)?;
    Ok(HttpResponse::Ok().json(user))
}

#[derive(Deserialize)]
pub struct QueryDelete {
    pub slug: String,
}

#[delete("/post")]
pub async fn delete_post(pool: web::Data<DbPool>, query: web::Query<QueryDelete>) -> Result<HttpResponse, Error> {
    let slug = query.slug.to_string();
    web::block(move || {
        let conn = pool.get()?;
        actions::delete_post(&slug, &conn)
    })
        .await?
        .map_err(actix_web::error::ErrorNotFound)?;
    Ok(HttpResponse::Ok().finish())
}

#[derive(Deserialize, Debug)]
pub struct SetupData {
    pub email: String,
    pub password: String,
}

#[post("/setup")]
pub async fn setup(pool: web::Data<DbPool>, data: web::Json<SetupData>) -> Result<HttpResponse, Error> {
    // Creating 2 connections, because of the move-thing
    let conn = pool.get().map_err(actix_web::error::ErrorInternalServerError)?;
    let conn2 = pool.get().map_err(actix_web::error::ErrorInternalServerError)?;
    // get number of users in db to determine if the setup has already been completed
    let user_count = web::block(move || {
        actions::count_users(&conn)
    })
        .await?
        .map_err(actix_web::error::ErrorConflict)?;
    // Check if setup is already completed
    if user_count != 0 {
        return Ok(HttpResponse::BadRequest().body("Setup already completed"));
    }
    web::block(move || {
        actions::setup(&data.email, &data.password, &conn2)
    })
        .await?
        .map_err(actix_web::error::ErrorConflict)?;
    Ok(HttpResponse::Ok().finish())
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
pub async fn update_post(pool: web::Data<DbPool>, data: web::Json<UpdatePost>) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let conn = pool.get()?;
        actions::update_post(&data.slug, &data.content, &data.title, &data.published, &data.tags, &data.intro, &conn)
    })
        .await?
        .map_err(actix_web::error::ErrorConflict)?;
    Ok(HttpResponse::Ok().json(user))
}

