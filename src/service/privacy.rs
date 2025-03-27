#![allow(unused)]

use paste::paste;
use adjust::{database::{postgres::Postgres, Database}, response::{HttpError, NonJsonHttpResult}};
use axum::http::StatusCode;
use crate::{models::user::UserIdQuery, models::privacy::Visibility, repository::{friends::FriendsRepository, privacy::PrivacyRepository}};

macro_rules! add_privacy_checker {
  ($name:ident, $message:expr) => {
    paste! {
      fn [<has_right_ $name>](
        db: &mut Database<Postgres>,
        query: UserIdQuery,
        user_id: i32
      ) -> NonJsonHttpResult<bool> {
        let privacy = PrivacyRepository::[<get_ $name>](db, user_id)?;

        let result = match query.user_id {
          Some(requester) if requester == user_id => true,
          Some(requester) => FriendsRepository::is_friends(db, user_id, requester)?,
          None => privacy == Visibility::Open,
        };

        Ok(result)
      }

      pub fn [<check_ $name>](
        db: &mut Database<Postgres>,
        query: UserIdQuery,
        user_id: i32
      ) -> NonJsonHttpResult<()> {
        if !Self::[<has_right_ $name>](db, query, user_id)? {
          return Err(HttpError::new($message, Some(StatusCode::FORBIDDEN)));
        }
        Ok(())
      }
    }
  };
}

pub struct PrivacyService;

impl PrivacyService {
  add_privacy_checker!(profile_visibility, "Пользователь скрыл профиль");
  add_privacy_checker!(friends_visibility, "Пользователь скрыл друзей");
  add_privacy_checker!(can_invite, "Вы не можете приглашать пользователя");
  add_privacy_checker!(server_visibility, "Пользователь скрыл статистику");
  add_privacy_checker!(online_visibility, "Пользователь скрыл статистику");
  add_privacy_checker!(hours_visibility, "Пользователь скрыл статистику");

  // fn has_right_see_friends(
  //   db: &mut Database<Postgres>,
  //   query: UserIdQuery,
  //   user_id: i32
  // ) -> NonJsonHttpResult<bool> {
  //   let privacy = PrivacyRepository::get_friends_visibility(db, user_id)?;

  //   let result = match query.user_id {
  //     Some(requester) if requester == user_id => true,
  //     Some(requester) => FriendsRepository::is_friends(db, user_id, requester)?,
  //     None => privacy == Visibility::Open,
  //   };

  //   Ok(result)
  // }

  // pub fn need_permission_see_friends(
  //   db: &mut Database<Postgres>,
  //   query: UserIdQuery,
  //   user_id: i32
  // ) -> NonJsonHttpResult<()> {
  //   Self::has_right_see_friends(db, query, user_id)
  //     .map_err(|_| HttpError::new("Пользователь скрыл список друзей", Some(StatusCode::FORBIDDEN)))?;

  //   Ok(())
  // }
}