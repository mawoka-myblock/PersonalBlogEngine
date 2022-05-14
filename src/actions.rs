use crate::models::{NewPost, Post};
use crate::{schema};
use crate::models;
use comrak::{ComrakOptions, markdown_to_html};
use diesel::prelude::*;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};

type DbError = Box<dyn std::error::Error + Send + Sync>;

fn get_markdown_options() -> ComrakOptions {
    let mut options = ComrakOptions::default();
    options.extension.autolink = true;
    options.extension.strikethrough = true;
    options.extension.superscript = true;
    options.extension.table = true;
    options.extension.footnotes = true;
    options.extension.header_ids = Some("header-".to_string());
    options.extension.tasklist = true;
    options.extension.description_lists = true;
    return options;
}

pub fn create_new_post(
    post: &NewPost,
    conn: &PgConnection,
) -> Result<models::Post, DbError> {
    use schema::posts::table;

    let rendered_content = markdown_to_html(&post.content, &get_markdown_options());
    let new_post = Post {
        slug: post.slug.to_string(),
        title: post.title.to_string(),
        content: post.content.to_string(),
        rendered_content: Some(rendered_content),
        published: post.published,
        created_at: None,
        updated_at: None,
        tags: post.tags.clone(),
    };
    diesel::insert_into(table).values(&new_post).get_result(conn)?;
    Ok(new_post)
}

pub fn delete_post(slug_input: &str, conn: &PgConnection) -> Result<usize, DbError> {
    use schema::posts::dsl::{posts, slug};
    let num_deleted = diesel::delete(posts.filter(slug.like(slug_input))).execute(conn)?;
    Ok(num_deleted)
}

pub fn check_user(email_input: &str, password_input: &str, conn: &PgConnection) -> Result<Option<models::User>, DbError> {
    use schema::users::dsl::{users, email};
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

pub fn count_users(conn: &PgConnection) -> Result<usize, DbError> {
    use schema::users::dsl::users;
    let count = users.count().get_result::<i64>(conn)?;
    Ok(count as usize)
}

pub fn count_posts(conn: &PgConnection) -> Result<usize, DbError> {
    use schema::posts::dsl::posts;
    let count = posts.count().get_result::<i64>(conn)?;
    Ok(count as usize)
}

pub fn setup(input_email: &str, input_password: &str, conn: &PgConnection) -> Result<(), DbError> {
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

pub fn get_raw_markdown(slug_input: &str, conn: &PgConnection) -> Result<Option<String>, DbError> {
    use schema::posts::dsl::{posts, slug};
    let post_obj = posts.filter(
        slug.like(slug_input)
    ).first::<models::Post>(conn);


    return match post_obj {
        Ok(post) => Ok(Some(post.content)),
        Err(_) => Ok(None),
    };
}

pub fn get_rendered_markdown(slug_input: &str, conn: &PgConnection) -> Result<Option<String>, DbError> {
    use schema::posts::dsl::{posts, slug};
    let post = posts.filter(slug.like(slug_input)).first::<models::Post>(conn);
    return match post {
        Ok(post) => Ok(Some(post.rendered_content.unwrap())),
        Err(_) => Ok(None),
    };


    /*    return if post.len() == 0 {
            Ok(post) => Ok(Some(match post[0].rendered_content {
                Some(rendered_content) => rendered_content,
                None => {
                    let rendered_content = markdown_to_html(&post.content, &get_markdown_options());
                    rendered_content
                }
            })),
            Err(_) => Ok(None),
        };*/
}

pub fn update_post(slug_input: &str, content_input: &str, title_input: &str, published: &bool, tags: &Vec<String>, conn: &PgConnection) -> Result<models::Post, DbError> {
    use schema::posts::dsl::{posts};
    use schema::posts::table;
    let post = posts.find(slug_input).get_result::<Post>(conn)?;
    let rendered_content = markdown_to_html(&post.content, &get_markdown_options());
    let new_post = models::Post {
        slug: slug_input.to_string(),
        title: title_input.to_string(),
        content: content_input.to_string(),
        rendered_content: Some(rendered_content),
        published: published.clone(),
        created_at: None,
        updated_at: None,
        tags: tags.clone(),
    };
    // diesel::update(posts.find(slug_input)).set(&new_post).execute(&conn)?;
/*    diesel::update(posts.find(slug_input))
        .set(&new_post)
        .get_result::<Post>(&conn)?;*/
    diesel::update(table).set(&new_post).execute(conn)?;

    Ok(new_post)
}