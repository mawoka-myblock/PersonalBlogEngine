use actix_identity::Identity;
use actix_web::{get, post, web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{actions, DbPool};

#[derive(Deserialize)]
pub struct FormData {
    pub email: String,
    pub password: String,
}

#[post("/login")]
pub async fn login(
    id: Identity,
    pool: web::Data<DbPool>,
    data: web::Json<FormData>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let conn = pool.get()?;
        actions::check_user(&data.email, &data.password, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    match user {
        Some(user) => {
            id.remember(user.email);
            Ok(HttpResponse::Ok().finish())
        }
        None => Ok(HttpResponse::Unauthorized().finish()),
    }
}

#[get("/logout")]
pub async fn logout(id: Identity) -> HttpResponse {
    // remove identity
    id.forget();
    HttpResponse::Ok().finish()
}
#[derive(Serialize)]
pub struct CheckResponse {
    pub is_logged_in: bool,
    pub user: Option<String>,
}

#[get("/check")]
pub async fn check_login_status(id: Identity) -> Result<HttpResponse, Error> {
    if let Some(id) = id.identity() {
        let resp = CheckResponse {
            is_logged_in: true,
            user: Some(id),
        };
        Ok(HttpResponse::Ok().json(resp))
    } else {
        let resp = CheckResponse {
            is_logged_in: false,
            user: None,
        };
        Ok(HttpResponse::Unauthorized().json(resp))
    }
}
