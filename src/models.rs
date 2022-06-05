use super::chrono;
use super::schema::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// #[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[derive(Debug, Clone, Queryable, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "posts"]
// #[primary_key(slug)]
pub struct Post {
    pub slug: String,
    pub title: String,
    pub content: String,
    pub rendered_content: Option<String>,
    pub published: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub tags: Vec<String>,
    pub intro: String,
    pub id: Uuid,
    pub thumbs_up: i16,
    pub thumbs_down: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPost {
    pub slug: String,
    pub title: String,
    pub content: String,
    pub published: bool,
    pub tags: Vec<String>,
    pub intro: String,
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
// #[primary_key(email)]
pub struct User {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListPosts {
    pub slug: String,
    pub title: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub tags: Vec<String>,
    pub intro: String,
    pub published: bool,
    pub thumbs_up: i16,
    pub thumbs_down: i16,
}

#[derive(Serialize, Deserialize)]
pub struct GetPost {
    pub content: String,
    pub metadata: ListPosts,
}

#[derive(
    Debug, Clone, Queryable, Serialize, Deserialize, AsChangeset, Insertable, Associations,
)]
// #[table_name = "posts"]
#[belongs_to(foreign_key = id)]
#[primary_key(id)]
#[table_name = "feedback"]
pub struct Feedback {
    pub id: Uuid,
    pub ip_hash: Vec<u8>,
    pub feedback_text: Option<String>,
    pub thumbs_up: bool,
    pub post_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, Queryable)]
pub struct PublicFeedback {
    pub id: Uuid,
    pub thumbs_up: bool,
    pub post: ListPosts,
    pub feedback_text: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}
