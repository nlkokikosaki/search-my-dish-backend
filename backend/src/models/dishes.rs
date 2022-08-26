use chrono::{DateTime, Utc};
use crate::schema::dishes;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[table_name = "dishes"]
pub struct Dish {
   pub name: String,
   pub image: String,
   pub created_at: DateTime<Utc>,
   pub content: String,
}