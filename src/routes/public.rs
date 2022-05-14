use actix_web::{get, Error, HttpResponse, web};
use serde::{Deserialize};

use crate::{actions, DbPool};

#[derive(Deserialize)]
pub struct Query {
    pub slug: String,
}

#[get("/raw")]
pub async fn get_raw_markdown(pool: web::Data<DbPool>, query: web::Query<Query>) -> Result<HttpResponse, Error> {
    let slug = query.slug.to_string();
    let markdown = web::block(move || {
        let conn = pool.get()?;
        actions::get_raw_markdown(&slug, &conn)
    })
        .await?
        .map_err(actix_web::error::ErrorNotFound)?;
    match markdown {
        Some(markdown) => Ok(HttpResponse::Ok().content_type("text/markdown").body(markdown)),
        None => Err(actix_web::error::ErrorNotFound("Post not found")),
    }
}

#[get("/rendered")]
pub async fn get_rendered_markdown(pool: web::Data<DbPool>, query: web::Query<Query>) -> Result<HttpResponse, Error> {
    let slug = query.slug.to_string();
    let html = web::block(move || {
        let conn = pool.get()?;
        actions::get_rendered_markdown(&slug, &conn)
    })
        .await?
        .map_err(actix_web::error::ErrorNotFound)?;
    match html {
        Some(html) => Ok(HttpResponse::Ok().content_type("text/html").body(html)),
        None => Err(actix_web::error::ErrorNotFound("Post not found")),
    }
}
#[derive(Deserialize)]
pub struct GetPostsQuery {
    pub offset: i64,
}

#[get("/posts")]
pub async fn get_posts(pool: web::Data<DbPool>, query: web::Query<GetPostsQuery>) -> Result<HttpResponse, Error> {
    let posts = web::block(move || {
        let conn = pool.get()?;
        actions::get_all_posts(&query.offset, &conn)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(posts))
}