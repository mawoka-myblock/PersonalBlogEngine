use crate::models::UploadFileInput;
use crate::{actions, DbPool};
use actix_identity::Identity;
use actix_web::{delete, get, post, web, Error, HttpResponse};
use actix_web_lab::extract::Json as LabJson;
use serde::Deserialize;
use uuid::Uuid;

const LIMIT_16_MB: usize = 16_554_432;

#[post("/")]
pub async fn upload_file(
    data: LabJson<UploadFileInput, LIMIT_16_MB>,
    pool: web::Data<DbPool>,
    id: Option<Identity>,
) -> Result<HttpResponse, Error> {
    if id.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    };
    let res = web::block(move || {
        let mut conn = pool.get()?;
        actions::add_file(&data.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorConflict)?;

    Ok(HttpResponse::Ok().json(res))
}

#[get("/{file_id}")]
pub async fn get_file(
    query: web::Path<Uuid>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let data = web::block(move || {
        let mut conn = pool.get()?;
        actions::get_file(query.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorNotFound)?;
    let content_type = match data.mime_type {
        Some(d) => d,
        None => "application/octet-stream".to_string(),
    };
    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", content_type))
        .insert_header(("Cache-Control", "max-age=31536000, immutable"))
        .body(data.data))
}

#[delete("/{file_id}")]
pub async fn delete_file(
    query: web::Path<Uuid>,
    pool: web::Data<DbPool>,
    id: Option<Identity>,
) -> Result<HttpResponse, Error> {
    if id.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    };
    web::block(move || {
        let mut conn = pool.get()?;
        actions::delete_file(query.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorNotFound)?;
    Ok(HttpResponse::Ok().finish())
}

#[derive(Deserialize)]
pub struct DeleteFilesQuery {
    pub offset: i64,
}

#[get("/list")]
pub async fn list_files(
    offset: web::Query<DeleteFilesQuery>,
    pool: web::Data<DbPool>,
    id: Option<Identity>,
) -> Result<HttpResponse, Error> {
    if id.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    };
    let files = web::block(move || {
        let mut conn = pool.get()?;
        actions::get_all_files(&offset.offset, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(files))
}
