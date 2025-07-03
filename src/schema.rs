// @generated automatically by Diesel CLI.

diesel::table! {
    person (person_id) {
        person_id -> Int4,
        first_name -> Nullable<Text>,
        last_name -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    post (post_id) {
        post_id -> Uuid,
        title -> Text,
        slug -> Text,
        content -> Text,
        author_id -> Int4,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(post -> person (author_id));

diesel::allow_tables_to_appear_in_same_query!(
    person,
    post,
);
