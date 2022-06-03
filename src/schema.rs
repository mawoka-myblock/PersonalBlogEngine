table! {
    feedback (id) {
        id -> Uuid,
        ip_hash -> Bytea,
        feedback_text -> Nullable<Text>,
        thumbs_up -> Bool,
        post_id -> Uuid,
        created_at -> Timestamp,
    }
}

table! {
    posts (slug) {
        slug -> Text,
        title -> Varchar,
        content -> Text,
        rendered_content -> Nullable<Text>,
        published -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        tags -> Array<Text>,
        intro -> Text,
        id -> Uuid,
        thumbs_up -> Int2,
        thumbs_down -> Int2,
    }
}

table! {
    users (email) {
        email -> Varchar,
        password -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    feedback,
    posts,
    users,
);
