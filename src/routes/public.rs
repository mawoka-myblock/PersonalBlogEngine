// use crate::search::search;
use actix_web::{get, web, Error, HttpResponse};
use serde::Deserialize;
use tantivy::{Index, LeasedItem, Searcher};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::Schema;

use crate::{actions, DbPool};

#[derive(Deserialize)]
pub struct Query {
    pub slug: String,
}

#[get("/raw")]
pub async fn get_raw_markdown(
    pool: web::Data<DbPool>,
    query: web::Query<Query>,
) -> Result<HttpResponse, Error> {
    let slug = query.slug.to_string();
    let markdown = web::block(move || {
        let conn = pool.get()?;
        actions::get_raw_markdown(&slug, &conn)
    })
        .await?
        .map_err(actix_web::error::ErrorNotFound)?;
    match markdown {
        Some(markdown) => Ok(HttpResponse::Ok().json(markdown)),
        None => Err(actix_web::error::ErrorNotFound("Post not found")),
    }
}

#[get("/rendered")]
pub async fn get_rendered_markdown(
    pool: web::Data<DbPool>,
    query: web::Query<Query>,
) -> Result<HttpResponse, Error> {
    let slug = query.slug.to_string();
    let html = web::block(move || {
        let conn = pool.get()?;
        actions::get_rendered_markdown(&slug, &conn)
    })
        .await?
        .map_err(actix_web::error::ErrorNotFound)?;
    match html {
        Some(html) => Ok(HttpResponse::Ok().json(html)),
        None => Err(actix_web::error::ErrorNotFound("Post not found")),
    }
}

#[derive(Deserialize)]
pub struct GetPostsQuery {
    pub offset: i64,
}

#[get("/posts")]
pub async fn get_posts(
    pool: web::Data<DbPool>,
    query: web::Query<GetPostsQuery>,
) -> Result<HttpResponse, Error> {
    let posts = web::block(move || {
        let conn = pool.get()?;
        actions::get_all_posts(&query.offset, true, &conn)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(posts))
}

#[get("/tags/{tag}")]
pub async fn get_posts_with_tag(
    pool: web::Data<DbPool>,
    query: web::Path<String>,
    offset: web::Query<GetPostsQuery>,
) -> Result<HttpResponse, Error> {
    let posts = web::block(move || {
        let conn = pool.get()?;
        actions::get_posts_with_specific_tag(&query.into_inner(), &offset.offset, &conn)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(posts))
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub query: String,
}

#[derive(Deserialize, Serialize)]
pub struct SearchResult {
    pub slug: String,
    pub intro: String,
    pub title: String,
}

#[get("/search")]
pub async fn search_posts(
    query_input: web::Query<SearchQuery>,
    index: web::Data<Index>,
    searcher: web::Data<LeasedItem<Searcher>>,
    schema: web::Data<Schema>,
) -> Result<HttpResponse, Error> {
    let slug = schema.get_field("slug").unwrap();
    let intro = schema.get_field("intro").unwrap();
    let title = schema.get_field("title").unwrap();
    let body = schema.get_field("body").unwrap();

    let query_parser = QueryParser::for_index(&index, vec![title, body, slug, intro]);
    let query = query_parser.parse_query(&*query_input.query).map_err(actix_web::error::ErrorBadRequest)?;
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10)).map_err(actix_web::error::ErrorInternalServerError).unwrap();

    let mut res_vec: Vec<SearchResult> = Vec::new();
    for (_score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address).map_err(actix_web::error::ErrorInternalServerError)?;
        let vals = retrieved_doc.field_values();

        res_vec.push(SearchResult {
            slug: vals[0].value.as_text().unwrap().parse().unwrap(),
            intro: vals[1].value.as_text().unwrap().parse().unwrap(),
            title: vals[2].value.as_text().unwrap().parse().unwrap(),
        })
    }

    Ok(HttpResponse::Ok().json(res_vec))
}
