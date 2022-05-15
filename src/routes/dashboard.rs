use actix_web::{get, Error, HttpResponse, web, Responder};
use serde::{Deserialize};
use crate::search::search;
use mime_guess::from_path;

use crate::{actions, DbPool};

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/public/"]
struct Asset;

fn handle_embedded_file(path: &str) -> HttpResponse {
    match Asset::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

#[actix_web::get("/")]
async fn index() -> impl Responder {
    handle_embedded_file("index.html")
}

#[actix_web::get("/dist/{_:.*}")]
async fn dist(path: web::Path<String>) -> impl Responder {
    handle_embedded_file(path.as_str())
}