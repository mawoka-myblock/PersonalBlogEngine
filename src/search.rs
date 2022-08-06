use std::sync::Mutex;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::Index;
use tantivy::ReloadPolicy;
use tempfile::TempDir;

use markdown_to_text;

use crate::schema::posts::dsl::{created_at, posts};
use crate::schema::posts::published;
use crate::{models, SearchData};
use diesel::prelude::*;
use diesel::{ExpressionMethods, PgConnection, QueryDsl};

pub fn get_schema() -> Schema {
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("slug", TEXT | STORED);
    schema_builder.add_text_field("intro", TEXT | STORED);
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("tags", TEXT | STORED);
    schema_builder.add_text_field("body", TEXT);

    schema_builder.build()
}

pub fn initialize_index(index: &Index, conn: &PgConnection) {
    let mut index_writer = index.writer(100_000_000).unwrap();
    let res = posts
        .filter(published.eq_all(true))
        .order_by(created_at.desc())
        .limit(100)
        .offset(0)
        .load::<models::Post>(conn)
        .unwrap();

    let schema = get_schema();
    let slug = schema.get_field("slug").unwrap();
    let intro = schema.get_field("intro").unwrap();
    let title = schema.get_field("title").unwrap();
    let tags = schema.get_field("tags").unwrap();
    let body = schema.get_field("body").unwrap();

    for post in res {
        index_writer
            .add_document(doc!(
                slug => post.slug,
                intro => post.intro,
                title => post.title,
                tags => post.tags.join(","),
                body => &*markdown_to_text::convert(&post.content),

            ))
            .unwrap();
    }

    index_writer.commit().unwrap();
}

pub fn update_index(conn: &PgConnection, search_data: actix_web::web::Data<Mutex<SearchData>>) {
    //.map_err(actix_web::error::ErrorConflict)?;
    let mut s_data = search_data.lock().unwrap();
    let index_path = TempDir::new().unwrap();
    s_data.index = Index::create_in_dir(&index_path, get_schema()).unwrap();
    initialize_index(&s_data.index, &conn);
    let reader = s_data
        .index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()
        .unwrap();

    s_data.searcher = reader.searcher();
}
