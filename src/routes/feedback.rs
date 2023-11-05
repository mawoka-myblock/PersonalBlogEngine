use crate::actions::{self, AppError};
use crate::DbPool;
use actix_identity::Identity;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse};
use blake3;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct PostFeedbackInput {
    thumbs_up: bool,
    feedback: Option<String>,
    post_slug: String,
}

#[post("/")]
pub async fn post_feedback(
    req: HttpRequest,
    data: web::Json<PostFeedbackInput>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let ip_addr = match req.connection_info().realip_remote_addr() {
        Some(i) => i.to_string(),
        None => return Ok(HttpResponse::BadRequest().body("No IP-address could be extracted.")),
    };

    let ip_hash = blake3::hash(ip_addr.as_ref()).as_bytes().to_vec();
    let res = web::block(move || {
        let mut conn = pool.get()?;
        let post = actions::get_single_post(&*data.0.post_slug, &mut conn)?;

        actions::submit_feedback(
            actions::SubmitFeedbackInput {
                post_id: post.id,
                feedback: data.0.feedback,
                ip_hash,
                thumbs_up: data.0.thumbs_up,
            },
            &mut conn,
        )
    })
    .await?;

    res.map(|_ok| HttpResponse::Ok().finish())
        .map_err(|err| match err {
            AppError::QueryError(query_err) => match &query_err {
                DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _error_info) => {
                    actix_web::error::ErrorConflict(query_err)
                }
                DieselError::DatabaseError(_kind, _error_info) => {
                    // TODO: map to more specific errors
                    // => see here: https://docs.rs/actix-web/latest/actix_web/error/index.html#functions
                    actix_web::error::ErrorInternalServerError(query_err)
                }
                DieselError::NotFound => actix_web::error::ErrorNotFound(query_err),
                // TODO: map to more specific errors
                // => see here: https://docs.rs/actix-web/latest/actix_web/error/index.html#functions
                _ => actix_web::error::ErrorInternalServerError(query_err),
            },
            // TODO: map to more specific errors
            // => see here: https://docs.rs/actix-web/latest/actix_web/error/index.html#functions
            _ => actix_web::error::ErrorInternalServerError(err),
        })
}

#[derive(Serialize, Deserialize)]
pub struct GetPostFeedbackQuery {
    pub limit: i64,
    pub post_id: Uuid,
}

#[get("/post")]
pub async fn get_feedback_from_post(
    query: web::Query<GetPostFeedbackQuery>,
    pool: web::Data<DbPool>,
    id: Option<Identity>,
) -> Result<HttpResponse, Error> {
    if id.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    };
    let res = web::block(move || {
        let mut conn = pool.get()?;
        actions::get_x_feedback_for_post(query.limit, query.post_id, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(res))
}

#[derive(Serialize, Deserialize)]
pub struct GetFeedbackQuery {
    pub limit: i64,
}

#[get("/")]
pub async fn get_feedback_list(
    query: web::Query<GetFeedbackQuery>,
    pool: web::Data<DbPool>,
    id: Option<Identity>,
) -> Result<HttpResponse, Error> {
    if id.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    };
    let res = web::block(move || {
        let mut conn = pool.get()?;
        actions::get_last_x_feedback(query.limit, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(res))
}
