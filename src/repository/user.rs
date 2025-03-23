#![allow(unused)]

use crate::{models::{privacy::UsersPrivacy, user::{User, UserCreate, UserUpdate}}, schema::users};
use adjust::{database::{postgres::Postgres, Database}, response::{HttpError, HttpResult, NonJsonHttpResult}};
use axum::{http::StatusCode, Json};
use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};
use super::privacy::PrivacyRepository;

pub struct UserRepository;

impl UserRepository {
  pub async fn get_user(
    db: &mut Database<Postgres>,
    id: i32
  ) -> NonJsonHttpResult<User> {
    Ok(users::table.filter(users::id.eq(id))
      .first::<User>(db)
      .map_err(|_| HttpError::new("Не получилось найти пользователя", Some(StatusCode::BAD_REQUEST)))?)
  }

  pub async fn find_by_email(
    db: &mut Database<Postgres>,
    email: String
  ) -> NonJsonHttpResult<User> {
    Ok(users::table.filter(users::email.eq(email))
      .first::<User>(db)
      .map_err(|_| HttpError::new("Не получилось найти пользователя", Some(StatusCode::BAD_REQUEST)))?)
  }

  pub async fn get_users(
    db: &mut Database<Postgres>,
    limit: u8,
    offset: u32
  ) -> NonJsonHttpResult<Vec<User>> {
    Ok(users::table
      .limit(limit.into())
      .offset(offset.into())
      .load::<User>(db)
      .map_err(|_| HttpError::new("Не получилось найти пользователей", Some(StatusCode::BAD_REQUEST)))?)
  }

  pub async fn add_user(
    db: &mut Database<Postgres>,
    user: UserCreate
  ) -> NonJsonHttpResult<User> {
    let user = insert_into(users::table)
      .values(&user)
      .get_result::<User>(db)
      .map_err(|_| HttpError::new("Не получилось добавить пользователя", Some(StatusCode::CONFLICT)))?;

    PrivacyRepository::add_user(db, user.id);

    Ok(user)
  }

  pub async fn patch_user(
    db: &mut Database<Postgres>,
    id: i32,
    user: UserUpdate
  ) -> NonJsonHttpResult<User> {
    Ok(diesel::update(users::table.filter(users::columns::id.eq(id)))
      .set(user)
      .get_result::<User>(db)
      .map_err(|_| HttpError::new("Не получилось обновить пользователя", Some(StatusCode::BAD_REQUEST)))?)
  }

  pub async fn delete_user(
    db: &mut Database<Postgres>,
    id: i32
  ) -> NonJsonHttpResult<()> {
    diesel::delete(users::table.filter(users::columns::id.eq(id)))
      .execute(db)
      .map_err(|_| HttpError::new("Не получилось удалить пользователя", Some(StatusCode::BAD_REQUEST)))?;

    Ok(())
  }
}