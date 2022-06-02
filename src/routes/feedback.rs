use std::fmt::format;
use crate::actions;
use crate::DbPool;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse};
use blake3;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::DatabaseError;
use serde::{Deserialize, Serialize};
use crate::actions::DbError;
use regex::Regex;

#[derive(Serialize, Deserialize)]
pub struct PostFeedbackInput {
    thumbs_up: bool,
    feedback: Option<String>,
    post_slug: String,
}

lazy_static! {
        static ref RE: Regex = Regex::new(r#"(?m)DatabaseError\((.*), ".*"\)"#).unwrap();
    }

#[post("/")]
pub async fn post_feedback(
    req: HttpRequest,
    data: web::Json<PostFeedbackInput>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn_info = req.connection_info();
    let ip_addr = match conn_info.realip_remote_addr() {
        Some(i) => i.to_string(),
        None => return Ok(HttpResponse::BadRequest().body("No IP-address could be extracted.")),
    };

    let ip_hash = blake3::hash(ip_addr.as_ref()).as_bytes().to_vec();
    let res = web::block(move || {
        let conn = pool.get()?;
        let post = actions::get_single_post(&*data.0.post_slug, &conn)?;

        actions::submit_feedback(
            actions::SubmitFeedbackInput {
                post_id: post.id,
                feedback: data.0.feedback,
                ip_hash,
                thumbs_up: data.0.thumbs_up,
            },
            &conn,
        )
    })
        .await?;
    return match res {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => {
            let err_str = format!("{:?}", e);
            println!("{:?}", e);
            let re = &RE;
            match re.captures(&*err_str) {
                Some(l) => {
                    let res = l.get(1).unwrap().as_str();
                    return match res {
                        "UniqueViolation" => Ok(HttpResponse::Conflict().finish()),

                        _ => Ok(HttpResponse::Ok().finish())
                    }
                }
                None => match &*err_str {
                    "NotFound" => return Ok(HttpResponse::NotFound().finish()),
                    _ => panic!("{:?}", e)
                }
            };
        }
    };
}
