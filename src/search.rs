use milli::documents::{DocumentBatchBuilder, DocumentBatchReader};
use milli::heed::EnvOpenOptions;
use milli::index::{Index};
use milli::update::{IndexerConfig, IndexDocumentsConfig, IndexDocuments};
use milli::{DocumentId, Search};
use std::io::Cursor;
use diesel::{ExpressionMethods, PgConnection, QueryDsl};
use diesel::prelude::*;
use crate::models;
use crate::schema::posts::dsl::{posts, created_at, slug};
use serde::{Serialize, Deserialize};
use crate::actions::DbError;
use markdown_to_text;
use milli::heed::types::Str;
use milli::tokenizer::Token;
use crate::models::ListPosts;
use crate::schema::posts::published;

extern crate milli;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeiliInsert {
    pub id: String,
    pub intro: String,
    pub body: String,
}

fn clean_html(input: &str) -> Result<String, String> {
    let frag = scraper::Html::parse_fragment(input);
    for node in frag.tree {
        if let scraper::node::Node::Text(text) = node {
            return Ok(text.text.to_string());
        }
    }
    Err("Could not find text".to_string())
}

pub fn search(term: &str, conn: &PgConnection) -> Result<Vec<ListPosts>, DbError> {
    let path = tempfile::tempdir().unwrap();
    let mut options = EnvOpenOptions::new();
    options.map_size(10 * 1024 * 1024); // 10 MB
    let index = Index::new(options, &path).unwrap();

    let mut wtxn = index.write_txn().unwrap();


    let config = IndexerConfig::default();
    let indexing_config = IndexDocumentsConfig::default();
    let mut builder =
        IndexDocuments::new(&mut wtxn, &index, &config, indexing_config, |_| ())
            .unwrap();
    let mut writer = Cursor::new(Vec::new());
    let mut index_builder = DocumentBatchBuilder::new(&mut writer).unwrap();
    let res = posts.filter(published.eq_all(true)).order_by(created_at.desc()).limit(100).offset(0).load::<models::Post>(conn).unwrap();
    let mut docs_list: Vec<String> = Vec::new();
    for post in res {
        docs_list.push(post.slug.to_string());
        let doc = MeiliInsert {
            id: post.slug,
            intro: post.intro,
            // body: match clean_html(&post.rendered_content.unwrap()) {
            //     Ok(x) => x,
            //     Err(e) => e.to_string(),
            // },
            body: markdown_to_text::convert(&post.content),
        };
        let json = serde_json::to_string(&doc).unwrap();
        index_builder.extend_from_json(&mut json.as_bytes()).unwrap();
    }

    index_builder.finish().unwrap();
    let writer2 = writer.clone();

    let reader = DocumentBatchReader::from_reader(Cursor::new(writer.into_inner())).unwrap();

    builder.add_documents(reader).unwrap();
    builder.execute().unwrap();
    wtxn.commit().unwrap();


// You can search in the index now!
    let rtxn = index.read_txn().unwrap();
    let mut search = Search::new(&rtxn, &index);
    let docs_count = index.number_of_documents(&rtxn).unwrap();
    search.query(term);
    search.limit(10);

    let result = search.execute().unwrap();
    let reader = DocumentBatchReader::from_reader(Cursor::new(writer2.into_inner())).unwrap();
    let index = reader.index();
    let mut docs: Vec<String> = Vec::new();
    for docId in result.documents_ids {
        docs.push(docs_list[docId as usize].to_string())
    }
    let res = posts.filter(slug.eq_any(docs)).order_by(created_at.desc()).load::<models::Post>(conn).unwrap();
    Ok(res
        .into_iter()
        .map(|kv| ListPosts {
            slug: kv.slug,
            title: kv.title,
            created_at: kv.created_at,
            updated_at: kv.updated_at,
            tags: kv.tags,
            intro: kv.intro,
            published: kv.published,
        })
        .collect())
}