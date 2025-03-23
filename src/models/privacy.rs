use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use crate::schema::users_privacy;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::VisibilityEnum"]
pub enum Visibility {
  #[db_rename = "open"]
  Open,
  #[db_rename = "friendonly"]
  FriendOnly,
  #[db_rename = "hidden"]
  Hidden,
}

impl Default for Visibility {
  fn default() -> Self {
    Self::Open
  }
}

#[derive(Queryable, Default, Selectable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = users_privacy)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UsersPrivacy {
  #[diesel(sql_type = Integer)]
  pub id: i32,
  /// Кто может видеть профиль игрока?
  pub profile_visibility: Visibility,
  /// Кто видит список друзей?
  pub friends_visibility: Visibility,
  /// Кто может пригласить пользователя в игру?
  pub can_invite: Visibility,
  /// Кто может видеть на каком сервере играет игрок?
  pub server_visibility: Visibility,
  /// Кто может видеть статус игрока?
  pub online_visibility: Visibility,
  /// Кто может видеть сколько игрок наиграл?
  pub hours_visibility: Visibility,
}