#![allow(unused)]
use adjust::{controller::Controller, response::{HttpResult, NonJsonHttpResult}};
use axum::{extract::{Path, Query, State}, routing::{delete, get, post}, Json, Router};
use serde_json::json;
use crate::{models::{user::UserIdQuery, userprofile::UserStatus}, service::{friend::FriendService, privacy::PrivacyService, status::StatusService, userprofile::UserProfileService}, AppState};

pub struct StatusController;

impl StatusController {
  async fn get(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Query(query): Query<UserIdQuery>
  ) -> HttpResult<UserStatus> {
    let mut db = state.postgres.get()?;
    let mut redis = state.redis.get()?;

    PrivacyService::check_profile_visibility(&mut db, query, user_id)?;

    Ok(Json(StatusService::get_full_status(&mut db, &mut redis, user_id, None)?))
  }

  // внутренний метод
  /// Пользователь заходит в Online
  async fn online(
    State(state): State<AppState>,
    Path(user_id): Path<i32>
  ) -> NonJsonHttpResult<()> {
    let mut db = state.postgres.get()?;

    FriendService::send_event(&mut db, user_id, "FRIEND_ONLINE", json!({
      "id": user_id
    }))
  }

  // внутренний метод
  /// Пользователь уходит в Offline
  async fn offline(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
  ) -> NonJsonHttpResult<()> {
    let mut db = state.postgres.get()?;

    // get_status внутри себя проводит проверку на кэш в редисе
    // если в редисе status: Offline то оно удалится
    StatusService::get_status(&mut db, &mut state.redis.get()?, user_id)?;

    FriendService::send_event(&mut db, user_id, "FRIEND_OFFLINE", json!({
      "id": user_id
    }))?;

    Ok(())
  }
}

impl Controller<AppState> for StatusController {
  fn new() -> anyhow::Result<Box<Self>> {
    Ok(Box::new(Self))
  }

  fn register(&self, router: Router<AppState>) -> Router<AppState> {
    router.nest("/status",
      Router::new()
        .route("/{id}", get(Self::get))
        .route("/{id}", post(Self::online))
        .route("/{id}", delete(Self::offline))
    )
  }
}