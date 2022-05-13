use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::models::{NewPost, Post};
use crate::{DbPool, schema};
use crate::models;
use crate::SqliteConnection;
use diesel::prelude::*;
use serde_json::{to_string, to_vec};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn create_new_post(
    post: &NewPost,
    conn: &SqliteConnection,
) -> Result<models::Post, DbError> {
    use schema::posts::dsl::posts;
    let mut tags_str: String = "".to_string();
    for tag in post.tags.iter() {
        tags_str.push_str(tag);
        tags_str.push(',');
    }
    tags_str.pop();
    let new_post = models::Post {
        slug: post.slug.to_string(),
        title: post.title.to_string(),
        content: post.content.to_string(),
        rendered_content: None,
        published: post.published,
        created_at: None,
        updated_at: None,
        tags: tags_str,
    };
    diesel::insert_into(posts).values(&new_post).execute(conn)?;
    Ok(new_post)
}

pub fn delete_post(slug_input: &str, conn: &SqliteConnection) -> Result<usize, DbError> {
    use schema::posts::dsl::{posts, slug};
    let num_deleted = diesel::delete(posts.filter(slug.like(slug_input))).execute(conn)?;
    Ok(num_deleted)
}

pub fn check_user(email_input: &str, password_input: &str, conn: &SqliteConnection) -> Result<Option<models::User>, DbError> {
    use schema::users::dsl::{users, email, password};
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let user = users.filter(
        email.like(email_input)
    ).first::<models::User>(conn);

    match user {
        Ok(user) => {
            let hash = PasswordHash::new(&user.password).unwrap();
            match Argon2::default().verify_password(password_input.as_bytes(), &hash) {
                Ok(_) => Ok(Some(user)),
                Err(_) => Ok(None),
            }
        }
        Err(_) => Ok(None),
    }
}

pub fn count_users(conn: &SqliteConnection) -> Result<usize, DbError> {
    use schema::users::dsl::users;
    let count = users.count().get_result::<i64>(conn)?;
    Ok(count as usize)
}

pub fn count_posts(conn: &SqliteConnection) -> Result<usize, DbError> {
    use schema::posts::dsl::posts;
    let count = posts.count().get_result::<i64>(conn)?;
    Ok(count as usize)
}

pub fn setup(input_email: &str, input_password: &str, conn: &SqliteConnection) -> Result<(), DbError> {
    use schema::users::dsl::{users};
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2.hash_password(input_password.as_bytes(), &salt)?;
    let new_user = models::User {
        email: input_email.to_string(),
        password: password_hash.to_string(),
    };
    diesel::insert_into(users).values(&new_user).execute(conn)?;
    Ok(())
}