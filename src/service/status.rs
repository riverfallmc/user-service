use adjust::{database::{postgres::Postgres, redis::Redis, Database}, redis::Commands, response::{HttpError, NonJsonHttpResult}};
use axum::http::StatusCode;
use chrono::NaiveDateTime;
use crate::{models::userprofile::{UserStatus, UserStatuses}, repository::user::UserRepository};

pub struct StatusService;

impl StatusService {
  fn get_record(
    db: &mut Database<Postgres>,
    redis: &mut Database<Redis>,
    user_id: i32
  ) -> NonJsonHttpResult<UserStatus> {
    let json = redis.get::<_, String>(format!("status:{user_id}"))
      .map_err(|_| HttpError::new("Запись статуса не была найдена", Some(StatusCode::NOT_FOUND)))?;

    let record: UserStatus = serde_json::from_str(&json)?;

    if let Some(last_seen) = record.last_seen_at {
      UserRepository::set_last_seen(db, user_id, last_seen)?;

      redis.del::<_, ()>(format!("status:{user_id}"))
        .map_err(|_| HttpError::new("Не получилось обновить статус", None))?;
    }

    Ok(record)
  }

  #[allow(unused)]
  pub fn get_status(
    db: &mut Database<Postgres>,
    redis: &mut Database<Redis>,
    user_id: i32
  ) -> NonJsonHttpResult<UserStatuses> {
    if let Ok(record) = Self::get_record(db, redis, user_id) {
      Ok(record.status)
    } else {
      Ok(UserStatuses::Offline)
    }
  }

  pub fn get_full_status(
    db: &mut Database<Postgres>,
    redis: &mut Database<Redis>,
    user_id: i32,
    last_seen_at: Option<NaiveDateTime>
  ) -> NonJsonHttpResult<UserStatus> {
    if let Ok(record) = Self::get_record(db, redis, user_id) {
      Ok(record)
    } else {
      Ok(UserStatus {
        status: UserStatuses::Offline,
        server: None,
        last_seen_at: Some(last_seen_at.unwrap_or(UserRepository::get_last_seen(db, user_id)?))
      })
    }
  }
}