// todo
#![allow(unused)]
use adjust::{controller::Controller, response::HttpResult};
use axum::{extract::Path, routing::get, Router};
use serde::{Deserialize, Serialize};
use crate::AppState;

#[derive(Serialize, Deserialize)]
pub enum UserStatus {
  Online,
  Offline,
  Hidden
}

#[derive(Serialize, Deserialize)]
pub struct UserStatusJson {
  status: UserStatus
}

pub struct StatusController;

impl StatusController {
  /// Во
  async fn get(
    Path(user_id): Path<u64>
  ) -> HttpResult<UserStatusJson> {
    todo!()
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
    )
  }
}