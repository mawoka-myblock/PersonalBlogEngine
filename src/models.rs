use super::chrono;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// #[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[derive(Debug, Clone, Queryable, Serialize, Deserialize, Insertable, AsChangeset, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
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
#[diesel(table_name = crate::schema::users)]
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
    Debug,
    Clone,
    Queryable,
    Serialize,
    Deserialize,
    AsChangeset,
    Insertable,
    Associations,
    Selectable,
)]
// #[table_name = "posts"]
// #[belongs_to(foreign_key = id)]
#[diesel(belongs_to(Post))]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::schema::feedback)]
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

#[derive(Debug, Clone, Queryable, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::uploads)]
pub struct Upload {
    pub id: Uuid,
    pub data: Vec<u8>,
    pub date_added: chrono::NaiveDateTime,
    pub mime_type: Option<String>,
    pub file_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFileInput {
    pub file_name: String,
    pub mime_type: Option<String>,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct UploadFileResponse {
    pub id: Uuid,
    pub date_added: chrono::NaiveDateTime,
    pub mime_type: Option<String>,
    pub file_name: Option<String>,
}
