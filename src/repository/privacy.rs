#![allow(unused)]

use adjust::{database::{postgres::Postgres, Database}, response::{HttpError, NonJsonHttpResult}};
use axum::http::StatusCode;
use diesel::{dsl::insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};
use crate::{models::privacy::{UsersPrivacy, Visibility}, schema::users_privacy};
use paste::paste;

macro_rules! add_privacy_getter {
  ($name:ident) => {
    paste! {
      pub fn [<get_ $name>](db: &mut Database<Postgres>, id: i32) -> NonJsonHttpResult<Visibility> {
        let result = users_privacy::table
          .filter(users_privacy::id.eq(id))
          .select(users_privacy::$name)
          .first(db)?;

        Ok(result)
      }
    }
  };
}

pub struct PrivacyRepository;

impl PrivacyRepository {
  pub fn add_user(
    db: &mut Database<Postgres>,
    id: i32
  ) -> NonJsonHttpResult<UsersPrivacy> {
    let result = insert_into(users_privacy::table)
      .values(users_privacy::id.eq(id))
      .get_result::<UsersPrivacy>(db)?;

    Ok(result)
  }

  pub fn get_settings(
    db: &mut Database<Postgres>,
    id: i32
  ) -> NonJsonHttpResult<UsersPrivacy> {
    users_privacy::table
      .filter(users_privacy::id.eq(id))
      .first::<UsersPrivacy>(db)
      .map_err(|_| HttpError::new("Пользователь не найден", Some(StatusCode::BAD_GATEWAY)))
  }

  // бля какие же макросы удобные я того рот ебал
  add_privacy_getter!(profile_visibility);
  add_privacy_getter!(friends_visibility);
  add_privacy_getter!(can_invite);
  add_privacy_getter!(server_visibility);
  add_privacy_getter!(online_visibility);
  add_privacy_getter!(hours_visibility);
}