use crate::{models::{User, UserCreate, UserUpdate}, service::user::UserService};
use axum::{extract::{Query, State}, routing::{delete, get, patch, post}};
use dixxxie::{
  axum::{self, extract::Path, Json}, connection::DbPool, controller::Controller, response::{HttpMessage, HttpResult}
};
use serde::Deserialize;

pub struct UserController;

#[derive(Deserialize)]
pub struct GetUsersQuery {
  limit: Option<u8>,
  offset: Option<u32>
}

impl UserController {
  pub async fn get_user(
    Path(id): Path<i32>,
    State(pool): State<DbPool>
  ) -> HttpResult<Json<User>> {
    Ok(Json(UserService::get_user(&pool, id)
      .await?))
  }

  pub async fn get_users(
    Query(query): Query<GetUsersQuery>,
    State(pool): State<DbPool>
  ) -> HttpResult<Json<Vec<User>>> {
    Ok(Json(UserService::get_users(&pool, query.limit.unwrap_or_default(), query.offset.unwrap_or_default())
      .await?))
  }

  pub async fn add_user(
    State(pool): State<DbPool>,
    Json(body): Json<UserCreate>,
  ) -> HttpResult<Json<dixxxie::response::HttpMessage>> {
    let id = UserService::add_user(&pool, body.clone())
      .await?;

    Ok(Json(HttpMessage::new(&format!("User {} has been created with Id {}", body.username, id))))

  }

  pub async fn patch_user(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(user): Json<UserUpdate>,
  ) -> HttpResult<Json<HttpMessage>> {
    UserService::patch_user(&pool, id, user)
      .await?;

    Ok(Json(HttpMessage::new(&format!("User with Id {id} has been updated"))))
  }

  pub async fn delete_user(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
  ) -> HttpResult<Json<HttpMessage>> {
    UserService::delete_user(&pool, id)
      .await?;

    Ok(Json(HttpMessage::new(&format!("User with Id {id} has been deleted"))))
  }
}

impl Controller<DbPool> for UserController {
  fn register(&self, router: axum::Router<DbPool>) -> axum::Router<DbPool> {
    router
      .route("/user/{id}", get(UserController::get_user))
      .route("/users", get(UserController::get_users))
      .route("/user", post(UserController::add_user))
      .route("/user/{id}", patch(UserController::patch_user))
      .route("/user/{id}", delete(UserController::delete_user))
  }
}