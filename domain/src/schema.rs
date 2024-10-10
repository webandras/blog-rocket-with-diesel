// @generated automatically by Diesel CLI.

diesel::table! {
    authors (id) {
        id -> Int4,
        #[max_length = 255]
        firstname -> Varchar,
        #[max_length = 255]
        lastname -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        is_active -> Bool,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        body -> Text,
        #[max_length = 255]
        genre -> Varchar,
        published -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        author_id -> Nullable<Int4>,
    }
}

diesel::joinable!(posts -> authors (author_id));

diesel::allow_tables_to_appear_in_same_query!(
    authors,
    posts,
);
