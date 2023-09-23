use crate::models;
use crate::models::{
    Feedback, GetPost, ListPosts, NewPost, Post, PublicFeedback, Upload, UploadFileInput,
    UploadFileResponse,
};
use crate::schema;
use argon2::{
    password_hash::{
        self, rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};
use comrak::{markdown_to_html, ComrakOptions};
use diesel::prelude::*;
use diesel::r2d2;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Error executing query")]
    QueryError(#[from] diesel::result::Error),
    #[error("Pool error")]
    PoolError(#[from] r2d2::PoolError),
    #[error("Password could not be hashed")]
    PasswordHashError(#[from] password_hash::Error),
    #[error("Base64 error")]
    Base64Error(#[from] base64::DecodeError),
    #[error("Unknown error occured")]
    Unknown,
}

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
    options
}

pub fn post_to_listpost(post: Post) -> ListPosts {
    models::ListPosts {
        slug: post.slug,
        title: post.title,
        created_at: post.created_at,
        updated_at: post.updated_at,
        tags: post.tags,
        intro: post.intro,
        published: post.published,
        thumbs_down: post.thumbs_down,
        thumbs_up: post.thumbs_up,
    }
}

fn posts_to_listposts(posts: Vec<Post>) -> Vec<models::ListPosts> {
    let mut listposts = Vec::new();
    for post in posts {
        listposts.push(post_to_listpost(post));
    }
    listposts
}

pub fn create_new_post(post: &NewPost, markdown: bool, conn: &PgConnection) -> Result<models::Post, AppError> {
    use schema::posts::table;

    let rendered_content = if markdown {
        markdown_to_html(&post.content, &get_markdown_options())
    } else {
        post.content.to_string()
    };
    let new_post = Post {
        slug: post.slug.to_string(),
        title: post.title.to_string(),
        content: post.content.to_string(),
        rendered_content: Some(rendered_content),
        published: post.published,
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
        tags: post.tags.clone(),
        intro: post.intro.clone(),
        id: Uuid::new_v4(),
        thumbs_down: 0,
        thumbs_up: 0,
    };
    diesel::insert_into(table).values(&new_post).execute(conn)?;
    Ok(new_post)
}

pub fn delete_post(slug_input: &str, conn: &PgConnection) -> Result<usize, AppError> {
    use schema::posts::dsl::{posts, slug};
    let num_deleted = diesel::delete(posts.filter(slug.like(slug_input))).execute(conn)?;
    Ok(num_deleted)
}

pub fn check_user(
    email_input: &str,
    password_input: &str,
    conn: &PgConnection,
) -> Result<Option<models::User>, AppError> {
    use schema::users::dsl::{email, users};
    // let argon2 = Argon2::default();
    // let salt = SaltString::generate(&mut OsRng);
    let user = users
        .filter(email.like(email_input))
        .first::<models::User>(conn);

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

pub fn count_users(conn: &PgConnection) -> Result<usize, AppError> {
    use schema::users::dsl::users;
    let count = users.count().get_result::<i64>(conn)?;
    Ok(count as usize)
}

pub fn count_posts(conn: &PgConnection) -> Result<usize, AppError> {
    use schema::posts::dsl::posts;
    let count = posts.count().get_result::<i64>(conn)?;
    Ok(count as usize)
}

pub fn setup(input_email: &str, input_password: &str, conn: &PgConnection) -> Result<(), AppError> {
    use schema::users::dsl::users;
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

pub fn get_raw_markdown(
    slug_input: &str,
    conn: &PgConnection,
) -> Result<Option<GetPost>, AppError> {
    use schema::posts::dsl::{posts, published, slug};
    let post_obj = posts
        .filter(slug.like(slug_input).and(published.eq(true)))
        .first::<models::Post>(conn);

    match post_obj {
        Ok(post) => Ok(Some(GetPost {
            content: post.content.to_string(),
            metadata: post_to_listpost(post),
        })),
        Err(_) => Ok(None),
    }
}

pub fn get_rendered_markdown(
    slug_input: &str,
    conn: &PgConnection,
) -> Result<Option<GetPost>, AppError> {
    use schema::posts::dsl::{posts, published, slug};
    let post = posts
        .filter(slug.like(slug_input).and(published.eq(true)))
        .first::<models::Post>(conn);
    match post {
        Ok(post) => Ok(Some(GetPost {
            content: post.rendered_content.as_ref().unwrap().to_string(),
            metadata: post_to_listpost(post),
        })),
        Err(_) => Ok(None),
    }

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

pub fn update_post(
    slug_input: &str,
    content_input: &Option<String>,
    title_input: &Option<String>,
    published: &Option<bool>,
    markdown: bool,
    tags: &Option<Vec<String>>,
    intro: &Option<String>,
    conn: &PgConnection,
) -> Result<models::Post, AppError> {
    use schema::posts::dsl::posts;
    let post = posts.find(slug_input).get_result::<Post>(conn)?;

    let new_post = models::Post {
        slug: slug_input.to_string(),
        title: match title_input {
            Some(title) => title.to_string(),
            None => post.title,
        },
        content: match content_input {
            Some(content) => content.to_string(),
            None => post.content,
        },
        rendered_content: match content_input {
            Some(content) => if markdown {
                Some(markdown_to_html(content, &get_markdown_options()))
            } else {
                Some(content.to_string())
            },
            None => Some(post.rendered_content.unwrap()),
        },
        published: match published {
            Some(published) => *published,
            None => post.published,
        },
        created_at: post.created_at,
        updated_at: chrono::Utc::now().naive_utc(),
        tags: match tags {
            Some(tags) => tags.to_vec(),
            None => post.tags,
        },
        intro: match intro {
            Some(intro) => intro.to_string(),
            None => post.intro,
        },
        id: post.id,
        thumbs_down: post.thumbs_down,
        thumbs_up: post.thumbs_up,
    };
    // diesel::update(posts.find(slug_input)).set(&new_post).execute(&conn)?;
    /*    diesel::update(posts.find(slug_input))
    .set(&new_post)
    .get_result::<Post>(&conn)?;*/
    let target = posts.find(slug_input);
    diesel::update(target).set(&new_post).execute(conn)?;
    //     use diesel::debug_query;
    // println!("{:?}", debug_query(&diesel::update(target).set(&new_post)));

    Ok(new_post)
}

pub fn get_all_posts(
    cursor: &i64,
    public: bool,
    conn: &PgConnection,
) -> Result<Vec<models::ListPosts>, AppError> {
    use schema::posts::dsl::{created_at, posts, published};
    let res = if public {
        posts
            .filter(published.eq_all(true))
            .order_by(created_at.desc())
            .limit(100)
            .offset(*cursor)
            .load::<Post>(conn)?
    } else {
        posts
            .order_by(created_at.desc())
            .limit(100)
            .offset(*cursor)
            .load::<Post>(conn)?
    };
    Ok(posts_to_listposts(res))
}

pub fn get_posts_with_specific_tag(
    tag: &str,
    offset: &i64,
    conn: &PgConnection,
) -> Result<Vec<models::ListPosts>, AppError> {
    use schema::posts::dsl::{created_at, posts, tags};
    let res = posts
        .filter(tags.contains(vec![tag.to_string()]))
        .order_by(created_at.desc())
        .limit(10)
        .offset(*offset)
        .load::<Post>(conn)?;
    Ok(posts_to_listposts(res))
}

pub fn get_single_post(slug: &str, conn: &PgConnection) -> Result<models::Post, AppError> {
    use schema::posts::dsl::posts;
    let res = posts.find(slug).get_result::<Post>(conn)?;
    Ok(res)
}

#[derive(Serialize, Deserialize)]
pub struct SubmitFeedbackInput {
    pub thumbs_up: bool,
    pub feedback: Option<String>,
    pub ip_hash: Vec<u8>,
    pub post_id: Uuid,
}

pub fn submit_feedback(
    feedback_input: SubmitFeedbackInput,
    conn: &PgConnection,
) -> Result<bool, AppError> {
    use schema::feedback::table;

    let fb = Feedback {
        feedback_text: feedback_input.feedback,
        post_id: feedback_input.post_id,
        thumbs_up: feedback_input.thumbs_up,
        id: Uuid::new_v4(),
        ip_hash: feedback_input.ip_hash,
        created_at: chrono::Utc::now().naive_utc(),
    };
    diesel::insert_into(table).values(&fb).execute(conn)?;
    use schema::posts::dsl::posts;
    use schema::posts::dsl::{id, thumbs_down, thumbs_up};
    let target = posts.filter(id.eq(feedback_input.post_id));
    if feedback_input.thumbs_up {
        diesel::update(target)
            .set(thumbs_up.eq(thumbs_up + 1))
            .execute(conn)?;
    } else {
        diesel::update(target)
            .set(thumbs_down.eq(thumbs_down + 1))
            .execute(conn)?;
    };

    Ok(true)
}

pub fn post_feedback_to_publicfeedback(data: Vec<(Feedback, Option<Post>)>) -> Vec<PublicFeedback> {
    let mut result_vec: Vec<PublicFeedback> = Vec::new();
    for i in data {
        let post = i.1.unwrap();
        result_vec.push(PublicFeedback {
            id: i.0.id,
            thumbs_up: i.0.thumbs_up,
            created_at: i.0.created_at,
            feedback_text: i.0.feedback_text,
            post: ListPosts {
                created_at: post.created_at,
                updated_at: post.updated_at,
                thumbs_up: post.thumbs_up,
                thumbs_down: post.thumbs_down,
                published: post.published,
                tags: post.tags,
                title: post.title,
                slug: post.slug,
                intro: post.intro,
            },
        })
    }
    result_vec
}

pub fn get_last_x_feedback(
    limit: i64,
    conn: &PgConnection,
) -> Result<Vec<PublicFeedback>, AppError> {
    use schema::feedback::dsl::{created_at, post_id};
    use schema::feedback::table as fb_table;
    use schema::posts::dsl::id;
    use schema::posts::table;
    let res = fb_table
        .left_join(table.on(id.eq(post_id)))
        .order_by(created_at.desc())
        .limit(limit)
        .load::<(Feedback, Option<Post>)>(conn)?;
    Ok(post_feedback_to_publicfeedback(res))
}

pub fn get_x_feedback_for_post(
    limit: i64,
    input_post_id: Uuid,
    conn: &PgConnection,
) -> Result<Vec<PublicFeedback>, AppError> {
    use schema::feedback::dsl::{created_at, post_id};
    use schema::feedback::table as fb_table;
    use schema::posts::dsl::id;
    use schema::posts::table;
    let res: Vec<(Feedback, Option<Post>)> = fb_table // SELECT * from feedback left join posts p on p.id = feedback.post_id where feedback_text = 'hallo welt'
        .left_join(table.on(id.eq(post_id)))
        .filter(post_id.eq(input_post_id))
        .order_by(created_at.desc())
        .limit(limit)
        .load::<(Feedback, Option<Post>)>(conn)?;

    Ok(post_feedback_to_publicfeedback(res))
}

pub fn add_file(
    input_data: &UploadFileInput,
    conn: &PgConnection,
) -> Result<UploadFileResponse, AppError> {
    use schema::uploads::table;
    let base64_data = match base64::decode(&input_data.data) {
        Err(error) => return Err(AppError::Base64Error(error)),
        Ok(d) => d,
    };
    let file_name = &input_data.file_name;
    let upload = Upload {
        id: Uuid::new_v4(),
        data: base64_data,
        date_added: chrono::Utc::now().naive_utc(),
        mime_type: input_data.mime_type.clone(),
        file_name: Some(file_name.clone()),
    };
    diesel::insert_into(table).values(&upload).execute(conn)?;
    Ok(UploadFileResponse {
        file_name: Some(file_name.to_string()),
        id: upload.id,
        mime_type: upload.mime_type,
        date_added: upload.date_added,
    })
}

pub fn get_file(file_id: Uuid, conn: &PgConnection) -> Result<Upload, AppError> {
    use schema::uploads::dsl::uploads;
    let res = uploads.find(file_id).get_result::<Upload>(conn)?;
    Ok(res)
}

pub fn delete_file(file_id: Uuid, conn: &PgConnection) -> Result<(), AppError> {
    use schema::uploads::dsl::uploads;
    diesel::delete(uploads.find(file_id)).execute(conn)?;
    // println!("{:?}", res);
    Ok(())
}

pub fn get_all_files(
    cursor: &i64,
    conn: &PgConnection,
) -> Result<Vec<models::UploadFileResponse>, AppError> {
    use schema::uploads::dsl::{date_added, file_name, id, mime_type, uploads};
    let res = uploads
        .order_by(date_added.desc())
        .limit(10)
        .select((id, date_added, mime_type, file_name))
        .offset(*cursor)
        .load::<UploadFileResponse>(conn)?;
    Ok(res)
}
