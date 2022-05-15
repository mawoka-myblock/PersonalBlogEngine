table! {
    posts (slug) {
        slug -> Varchar,
        title -> Varchar,
        content -> Text,
        rendered_content -> Nullable<Text>,
        published -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        tags -> Array<Text>,
        intro -> Text,
    }
}

table! {
    users (email) {
        email -> Varchar,
        password -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(posts, users,);
