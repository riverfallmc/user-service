use adjust::{controller::Controller, response::{HttpError, HttpResult, NonJsonHttpResult}};
use axum::{extract::{Path, Query, State}, routing::get, Json, Router};
use once_cell::sync::Lazy;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use crate::{models::privacy::UsersPrivacy, repository::privacy::PrivacyRepository, AppState};

#[derive(Deserialize, Serialize)]
pub struct Settings {
  settings: Vec<String>
}

#[derive(Deserialize)]
pub struct CheckPermissionQuery {
  actor: i32,
  r#type: String
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

  async fn check_has_permissions(
    State(state): State<AppState>,
    Path(subject_id): Path<i32>,
    Query(query): Query<CheckPermissionQuery>
  ) -> NonJsonHttpResult<()> {
    let mut db = state.postgres.get()?;

    let visibility = match query.r#type.to_lowercase().as_str() {
      "invite" => Ok(PrivacyRepository::get_can_invite(&mut db, subject_id)?),
      _ => Err(HttpError::new("Метод не найден", Some(StatusCode::BAD_REQUEST)))
    }?.can_interact(&mut db, subject_id, query.actor)?;

    if !visibility {
      return Err(HttpError::new("Вы не можете этого сделать!", Some(StatusCode::FORBIDDEN)))
    }

    Ok(())
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
        .route("/check/{subject}", get(Self::check_has_permissions))
    )
  }
}