use super::schema::*;
use serde::{Deserialize, Serialize};
use super::chrono;

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "posts"]
pub struct Post {
    pub slug: String,
    pub title: String,
    pub content: String,
    pub rendered_content: Option<String>,
    pub published: bool,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub tags: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPost {
    pub slug: String,
    pub title: String,
    pub content: String,
    pub published: bool,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub email: String,
    pub password: String,
}