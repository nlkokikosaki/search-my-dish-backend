// @generated automatically by Diesel CLI.

diesel::table! {
    dishes (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Nullable<Date>,
        image -> Nullable<Varchar>,
        content -> Nullable<Varchar>,
    }
}
