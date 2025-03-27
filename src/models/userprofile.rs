use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum UserStatuses {
  Online,
  Playing,
  Offline,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserStatus {
  pub status: UserStatuses,
  pub server: Option<i32>,
  pub last_seen_at: Option<NaiveDateTime>
}

#[derive(Serialize, Deserialize)]
pub struct UserProfile { // смесь User и UserStatus
  pub id: i32,
  pub username: String,
  pub rank: String,
  pub status: UserStatus,
  pub registered_at: NaiveDateTime
}