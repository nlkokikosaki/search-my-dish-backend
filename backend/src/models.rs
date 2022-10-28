use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Dishes {
    id: i32,
    name: String,
    image: Option<String>,
    content: Option<String>,
}

#[derive(Queryable, Serialize)]
pub struct Id {
    id: i32,
}
