use crate::{models::{User, UserCreate, UserUpdate}, repository::user::UserRepository};
use dixxxie::{connection::DbPool, response::HttpResult};

pub struct UserService;

impl UserService {
  /// Добавляет игрока в базу данных
  pub async fn add_user(
    db: &DbPool,
    user: UserCreate
  ) -> HttpResult<usize> {
    let mut db = db.get()?;

    UserRepository::add_user(&mut db, user)
      .await
  }

  pub async fn get_user(
    db: &DbPool,
    id: i32
  ) -> HttpResult<User> {
    let mut db = db.get()?;

    UserRepository::get_user(&mut db, id)
      .await
  }

  pub async fn get_users(
    db: &DbPool,
    limit: u8,
    offset: u32
  ) -> HttpResult<Vec<User>> {
    let mut db = db.get()?;

    UserRepository::get_users(&mut db, limit.min(25), offset)
      .await
  }

  pub async fn patch_user(
    db: &DbPool,
    id: i32,
    user: UserUpdate
  ) -> HttpResult<usize> {
    let mut db = db.get()?;

    UserRepository::patch_user(&mut db, id, user)
      .await
  }

  pub async fn delete_user(
    db: &DbPool,
    id: i32,
  ) -> HttpResult<usize> {
    let mut db = db.get()?;

    UserRepository::delete_user(&mut db, id)
      .await
  }
}