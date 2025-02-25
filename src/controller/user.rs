use adjust::{controller::Controller, response::{HttpMessage, HttpResult}};
use serde::Deserialize;
use crate::{models::{User, UserCreate, UserUpdate}, service::user::UserService, AppState};
use axum::{extract::{Path, Query, State}, routing::{delete, get, patch, post}, Json};

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
      return UserService::get_by_email(&mut db, email).await;
    }

    UserService::get_user(&mut db, id)
      .await
  }

  pub async fn get_users(
    Query(query): Query<GetUsersQuery>,
    State(state): State<AppState>
  ) -> HttpResult<Vec<User>> {
    let mut db = state.postgres.get()?;

    UserService::get_users(&mut db, query.limit.unwrap_or_default(), query.offset.unwrap_or_default())
      .await
  }

  pub async fn add_user(
    State(state): State<AppState>,
    Json(body): Json<UserCreate>,
  ) -> HttpResult<HttpMessage> {
    let mut db = state.postgres.get()?;

    let id = UserService::add_user(&mut db, body.clone())
      .await?;

    Ok(Json(HttpMessage::new(&format!("User {} has been created with Id {}", body.username, id.0))))

  }

  pub async fn patch_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(user): Json<UserUpdate>,
  ) -> HttpResult<HttpMessage> {
    let mut db = state.postgres.get()?;

    #[allow(unused)]
    UserService::patch_user(&mut db, id, user)
      .await?;

    Ok(Json(HttpMessage::new(&format!("User with Id {id} has been updated"))))
  }

  pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
  ) -> HttpResult<HttpMessage> {
    let mut db = state.postgres.get()?;

    #[allow(unused)]
    UserService::delete_user(&mut db, id)
      .await?;

    Ok(Json(HttpMessage::new(&format!("User with Id {id} has been deleted"))))
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