// @generated automatically by Diesel CLI.

diesel::table! {
    feedback (id) {
        id -> Uuid,
        ip_hash -> Bytea,
        feedback_text -> Nullable<Text>,
        thumbs_up -> Bool,
        post_id -> Uuid,
        created_at -> Timestamp,
    }
}

diesel::table! {
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

diesel::table! {
    uploads (id) {
        id -> Uuid,
        data -> Bytea,
        date_added -> Timestamp,
        mime_type -> Nullable<Text>,
        file_name -> Nullable<Text>,
    }
}

diesel::table! {
    users (email) {
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(feedback, posts, uploads, users,);
