use adjust::{database::{postgres::Postgres, Database}, response::HttpResult};
use crate::{models::{User, UserCreate, UserUpdate}, repository::user::UserRepository};

pub struct UserService;

impl UserService {
  /// Добавляет игрока в базу данных
  pub async fn add_user(
    db: &mut Database<Postgres>,
    user: UserCreate
  ) -> HttpResult<usize> {
    UserRepository::add_user(db, user)
      .await
  }

  pub async fn get_user(
    db: &mut Database<Postgres>,
    id: i32
  ) -> HttpResult<User> {
    UserRepository::get_user(db, id)
      .await
  }

  pub async fn get_by_email(
    db: &mut Database<Postgres>,
    email: String
  ) -> HttpResult<User> {
    UserRepository::find_by_email(db, email)
      .await
  }

  pub async fn get_users(
    db: &mut Database<Postgres>,
    limit: u8,
    offset: u32
  ) -> HttpResult<Vec<User>> {
    UserRepository::get_users(db, limit.min(25), offset)
      .await
  }

  pub async fn patch_user(
    db: &mut Database<Postgres>,
    id: i32,
    user: UserUpdate
  ) -> HttpResult<usize> {
    UserRepository::patch_user(db, id, user)
      .await
  }

  pub async fn delete_user(
    db: &mut Database<Postgres>,
    id: i32,
  ) -> HttpResult<usize> {
    UserRepository::delete_user(db, id)
      .await
  }
}