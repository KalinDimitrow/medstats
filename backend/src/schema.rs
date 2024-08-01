// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        id -> Int4,
        name -> Varchar,
        password -> Varchar,
    }
}
