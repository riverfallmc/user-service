use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::schema::users;

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
  #[diesel(sql_type = Integer)]
  pub id: i32,
  #[diesel(sql_type = Text)]
  pub username: String,
  #[diesel(sql_type = Text)]
  pub email: String,
  #[diesel(sql_type = Jsonb)]
  pub friends: Value,
  #[diesel(sql_type = Text)]
  pub rank: String,
  #[diesel(sql_type = Timestamp)]
  pub registered_at: NaiveDateTime
}

#[derive(Insertable, Deserialize, Serialize, Clone)]
#[diesel(table_name = users)]
pub struct UserCreate {
  pub username: String,
  pub email: String,
  pub rank: Option<String>,
}

#[derive(AsChangeset, Deserialize, Serialize)]
#[diesel(table_name = users)]
pub struct UserUpdate {
  pub username: Option<String>,
  pub email: Option<String>,
  pub rank: Option<String>,
}