table! {
    posts (slug) {
        slug -> Nullable<Text>,
        title -> Text,
        content -> Text,
        rendered_content -> Nullable<Text>,
        published -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        tags -> Nullable<Text>,
    }
}

table! {
    users (email) {
        email -> Text,
        password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
