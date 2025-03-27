use crate::{models::{user::{User, UserCreate, UserIdQuery, UserUpdate}, userprofile::UserProfile}, repository::user::UserRepository, service::userprofile::UserProfileService, AppState};
use axum::{extract::{Path, Query, State}, routing::{delete, get, patch, post}, Json};
use adjust::{controller::Controller, response::{HttpMessage, HttpResult}};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetUsersQuery {
  limit: Option<u8>,
  offset: Option<u32>
}

#[derive(Deserialize)]
pub struct UsersQuery {
  email: Option<String>
}

pub struct UserController;

impl UserController {
  pub async fn get_user(
    Path(id): Path<i32>,
    Query(query): Query<UsersQuery>,
    State(state): State<AppState>,
  ) -> HttpResult<User> {
    let mut db = state.postgres.get()?;

    if let Some(email) = query.email {
      return Ok(Json(UserRepository::find_by_email(&mut db, email)?));
    }

    Ok(Json(UserRepository::get_user(&mut db, id)?))
  }

  pub async fn get_user_profile(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Query(query): Query<UserIdQuery>
  ) -> HttpResult<UserProfile> {
    let mut db = state.postgres.get()?;
    let mut redis = state.redis.get()?;

    Ok(Json(UserProfileService::get_user_profile(&mut db, &mut redis, Some(query), id)?))
  }

  pub async fn get_users(
    Query(query): Query<GetUsersQuery>,
    State(state): State<AppState>
  ) -> HttpResult<Vec<User>> {
    let mut db = state.postgres.get()?;
    let result = UserRepository::get_users(&mut db, query.limit.unwrap_or(5), query.offset.unwrap_or_default())?;

    Ok(Json(result))
  }

  pub async fn add_user(
    State(state): State<AppState>,
    Json(body): Json<UserCreate>,
  ) -> HttpResult<User> {
    let mut db = state.postgres.get()?;

    let user = UserRepository::add_user(&mut db, body.clone())?;

    Ok(Json(user))
  }

  pub async fn patch_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(user): Json<UserUpdate>,
  ) -> HttpResult<User> {
    let mut db = state.postgres.get()?;
    let result = UserRepository::patch_user(&mut db, id, user)?;

    Ok(Json(result))
  }

  pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
  ) -> HttpResult<HttpMessage> {
    let mut db = state.postgres.get()?;

    UserRepository::delete_user(&mut db, id)?;

    Ok(Json(HttpMessage::new("Пользователь был удалён")))
  }
}

impl Controller<AppState> for UserController {
  fn new() -> anyhow::Result<Box<Self>> {
    Ok(Box::new(Self))
  }

  fn register(&self, router: axum::Router<AppState>) -> axum::Router<AppState> {
    router
      .route("/user/{id}", get(Self::get_user))
      .route("/profile/{id}", get(Self::get_user_profile))
      .route("/users", get(Self::get_users))
      .route("/user", post(Self::add_user))
      .route("/user/{id}", patch(Self::patch_user))
      .route("/user/{id}", delete(Self::delete_user))
  }
}