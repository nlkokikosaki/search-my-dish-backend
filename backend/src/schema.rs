table! {
    dishes (id) {
        id -> Int4,
        name -> Text,
        created_at -> Nullable<Timestamp>,
        image -> Nullable<Text>,
        content -> Nullable<Text>,
    }
}
