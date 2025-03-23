use adjust::{controller::Controller, response::HttpResult};
use axum::{extract::{Path, State}, routing::get, Json, Router};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use crate::{models::privacy::UsersPrivacy, repository::privacy::PrivacyRepository, AppState};

#[derive(Deserialize, Serialize)]
pub struct Settings {
  settings: Vec<String>
}

static PRIVACY_SETTINGS: Lazy<Vec<String>> = Lazy::new(|| {
  // тут мы вытягиваем ключи из структуры, ну ибо хули
  let settings = serde_json::to_value(UsersPrivacy::default())
    .unwrap(); // соси

  let mut result = vec![];

  settings.as_object()
    .unwrap()
    .into_iter()
    .for_each(|(key, _)| result.push(key.to_owned()));

  result
});

pub struct PrivacyController;

impl PrivacyController {
  async fn settings() -> HttpResult<Vec<String>> {
    Ok(Json(PRIVACY_SETTINGS.clone()))
  }

  async fn user_settings(
    State(state): State<AppState>,
    Path(user_id): Path<i32>
  ) -> HttpResult<UsersPrivacy> {
    let mut db = state.postgres.get()?;

    Ok(Json(PrivacyRepository::get_settings(&mut db, user_id)?))
  }
}

impl Controller<AppState> for PrivacyController {
  fn new() -> anyhow::Result<Box<Self>> {
    Ok(Box::new(Self))
  }

  fn register(&self, router: Router<AppState>) -> Router<AppState> {
    router.nest("/privacy",
      Router::new()
        .route("/settings", get(Self::settings))
        .route("/{id}", get(Self::user_settings))
    )
  }
}