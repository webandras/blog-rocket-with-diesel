// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        genre -> Varchar,
        published -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
