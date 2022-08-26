use chrono::{DateTime, Utc};
use crate::schema::dishes;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize)]
pub struct Dish {
   pub name: String,
   pub image: String,
   pub created_at: DateTime<Utc>,
   pub content: String,
}