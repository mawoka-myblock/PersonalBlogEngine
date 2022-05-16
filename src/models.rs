use super::chrono;
use super::schema::*;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
pub struct ListPosts {
    pub slug: String,
    pub title: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub tags: Vec<String>,
    pub intro: String,
    pub published: bool,
}

#[derive(Serialize, Deserialize)]
pub struct GetPost {
    pub content: String,
    pub metadata: ListPosts,
}
