use actix_web::{get, Error, HttpResponse, web};

use crate::{actions, DbPool};

#[get("/raw/{slug}")]
async fn get_raw_markdown(pool: web::Data<DbPool>, path: web::Path<(String, )>) -> Result<HttpResponse, Error> {
    let slug = path.into_inner().0;
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

#[get("/rendered/{slug}")]
async fn get_rendered_markdown(pool: web::Data<DbPool>, path: web::Path<(String, )>) -> Result<HttpResponse, Error> {
    let slug = path.into_inner().0;
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