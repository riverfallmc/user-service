use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

use crate::schema::friendships;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::Relationship"]
pub enum Relationship {
  #[db_rename = "pending"]
  Pending,
  #[db_rename = "accepted"]
  Accepted,
}

impl Default for Relationship {
  fn default() -> Self {
    Self::Pending
  }
}

#[derive(Queryable, Default, Selectable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = friendships)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Friendship {
  pub user_id: i32,
  pub friend_id: i32,
  pub status: Relationship,
  pub created_at: NaiveDateTime
}