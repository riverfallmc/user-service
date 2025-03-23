use crate::{models::user::{User, UserCreate, UserUpdate}, repository::user::UserRepository, AppState};
use axum::{extract::{Path, Query, State}, http::StatusCode, routing::{delete, get, patch, post}, Json};
use adjust::{controller::Controller, response::{HttpError, HttpMessage, HttpResult}};
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
    Path(id): Path<Option<i32>>,
    Query(query): Query<UsersQuery>,
    State(state): State<AppState>,
  ) -> HttpResult<User> {
    let mut db = state.postgres.get()?;

    if let Some(email) = query.email {
      return Ok(Json(UserRepository::find_by_email(&mut db, email).await?));
    }

    if let Some(id) = id {
      return Ok(Json(UserRepository::get_user(&mut db, id).await?))
    }

    Err(HttpError::new("Вы не указали Id", Some(StatusCode::BAD_REQUEST)))
  }

  pub async fn get_users(
    Query(query): Query<GetUsersQuery>,
    State(state): State<AppState>
  ) -> HttpResult<Vec<User>> {
    let mut db = state.postgres.get()?;
    let result = UserRepository::get_users(&mut db, query.limit.unwrap_or(5), query.offset.unwrap_or_default())
      .await?;

    Ok(Json(result))
  }

  pub async fn add_user(
    State(state): State<AppState>,
    Json(body): Json<UserCreate>,
  ) -> HttpResult<User> {
    let mut db = state.postgres.get()?;

    let user = UserRepository::add_user(&mut db, body.clone())
      .await?;

    Ok(Json(user))
  }

  pub async fn patch_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(user): Json<UserUpdate>,
  ) -> HttpResult<User> {
    let mut db = state.postgres.get()?;
    let result = UserRepository::patch_user(&mut db, id, user)
      .await?;

    Ok(Json(result))
  }

  pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
  ) -> HttpResult<HttpMessage> {
    let mut db = state.postgres.get()?;

    UserRepository::delete_user(&mut db, id)
      .await?;

    Ok(Json(HttpMessage::new(&format!("User has been deleted"))))
  }
}

impl Controller<AppState> for UserController {
  fn new() -> anyhow::Result<Box<Self>> {
    Ok(Box::new(Self))
  }

  fn register(&self, router: axum::Router<AppState>) -> axum::Router<AppState> {
    router
      .route("/user/{id}", get(UserController::get_user))
      .route("/users", get(UserController::get_users))
      .route("/user", post(UserController::add_user))
      .route("/user/{id}", patch(UserController::patch_user))
      .route("/user/{id}", delete(UserController::delete_user))
  }
}