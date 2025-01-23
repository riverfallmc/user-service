#![allow(unused)]

use crate::{models::{User, UserCreate, UserUpdate}, schema::users};
use axum::http::StatusCode;
use dixxxie::{connection::{DbPool, DbPooled}, response::{HttpError, HttpResult}};
use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};

pub struct UserRepository;

impl UserRepository {
  pub async fn get_user(
    db: &mut DbPooled,
    id: i32
  ) -> HttpResult<User> {
    users::table.filter(users::id.eq(id))
      .first::<User>(db)
      .map_err(|_| HttpError::new("Не получилось найти пользователя", Some(StatusCode::BAD_REQUEST)))
  }

  pub async fn get_users(
    db: &mut DbPooled,
    limit: u8,
    offset: u32
  ) -> HttpResult<Vec<User>> {
    users::table
      .limit(limit.into())
      .offset(offset.into())
      .load::<User>(db)
      .map_err(|_| HttpError::new("Не получилось найти пользователей", Some(StatusCode::BAD_REQUEST)))
  }

  pub async fn add_user(
    db: &mut DbPooled,
    user: UserCreate
  ) -> HttpResult<usize> {
    insert_into(users::table)
      .values(&user)
      .execute(db)
      .map_err(|_| HttpError::new("Не получилось добавить пользователя", Some(StatusCode::CONFLICT)))
  }

  pub async fn patch_user(
    db: &mut DbPooled,
    id: i32,
    user: UserUpdate
  ) -> HttpResult<usize> {
    diesel::update(users::table.filter(users::columns::id.eq(id)))
      .set(user)
      .execute(db)
      .map_err(|_| HttpError::new("Не получилось обновить пользователя", Some(StatusCode::BAD_REQUEST)))
  }

  pub async fn delete_user(
    db: &mut DbPooled,
    id: i32
  ) -> HttpResult<usize> {
    diesel::delete(users::table.filter(users::columns::id.eq(id)))
      .execute(db)
      .map_err(|_| HttpError::new("Не получилось удалить пользователя", Some(StatusCode::BAD_REQUEST)))
  }
}