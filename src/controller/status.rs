#![allow(unused)]
use adjust::{controller::Controller, response::{HttpResult, NonJsonHttpResult}};
use axum::{extract::{Path, Query, State}, routing::{delete, get, patch, post}, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::{models::{privacy::Visibility, user::UserIdQuery, userprofile::UserStatus}, repository::privacy::PrivacyRepository, service::{friend::{FriendEvents, FriendService}, privacy::PrivacyService, status::StatusService, userprofile::UserProfileService}, AppState};

#[derive(Serialize, Deserialize)]
struct ServerIdQuery {
  server_id: i32
}

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

    let privacy = PrivacyRepository::get_online_visibility(&mut db, user_id)?;

    if privacy == Visibility::Hidden {
      return Ok(())
    }

    FriendService::send_event(&mut db, user_id, FriendEvents::FriendOnline, json!({
      "id": user_id
    }))
  }

  async fn playing(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Query(query): Query<ServerIdQuery>
  ) -> NonJsonHttpResult<()> {
    let mut db = state.postgres.get()?;

    if query.server_id == 0 {
      FriendService::send_event(&mut db, user_id, FriendEvents::FriendDisconnect, json!({
        "id": user_id,
      }))
    } else {
      FriendService::send_event(&mut db, user_id, FriendEvents::FriendJoinServer, json!({
        "id": user_id,
        "server": query.server_id
      }))
    }
  }

  // внутренний метод
  /// Пользователь уходит в Offline
  async fn offline(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
  ) -> NonJsonHttpResult<()> {
    let mut db = state.postgres.get()?;

    let privacy = PrivacyRepository::get_online_visibility(&mut db, user_id)?;

    if privacy == Visibility::Hidden {
      return Ok(())
    }

    // get_status внутри себя проводит проверку на кэш в редисе
    // если в редисе status: Offline то оно удалится
    StatusService::get_status(&mut db, &mut state.redis.get()?, user_id)?;

    FriendService::send_event(&mut db, user_id, FriendEvents::FriendOffline, json!({
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
        .route("/{id}", patch(Self::playing))
        .route("/{id}", delete(Self::offline))
    )
  }
}