// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Varchar,
        user_tag -> Varchar,
        level -> Int4,
        email -> Varchar,
        password -> Varchar,
    }
}
