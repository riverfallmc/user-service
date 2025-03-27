use adjust::{controller::Controller, response::{HttpMessage, HttpResult}};
use axum::{extract::{Path, Query, State}, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{models::user::UserIdQuery, service::{invite::InviteService, privacy::PrivacyService, wss::WssService}, AppState};

#[derive(Serialize, Deserialize)]
struct InviteQuery {
  inviter_id: i32,
  server: u16
}

pub struct InviteController;

impl InviteController {
  async fn invite(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Query(query): Query<InviteQuery>
  ) -> HttpResult<HttpMessage> {
    let mut db = state.postgres.get()?;
    let mut redis = state.redis.get()?;

    let inviter = query.inviter_id;

    InviteService::check_send(&mut redis, inviter, id)?;

    // проверяем что чел в друзьях, и его можно инвайтить
    PrivacyService::check_can_invite(&mut db, UserIdQuery { user_id: Some(inviter) }, id)?;

    WssService::send(id, "INVITE", json!({
      "id": inviter,
      "server": query.server
    }))?;

    Ok(Json(
      HttpMessage::new("Приглашение было отправлено")
    ))
  }
}

impl Controller<AppState> for InviteController {
  fn new() -> anyhow::Result<Box<Self>> {
    Ok(Box::new(Self))
  }

  fn register(&self, router: Router<AppState>) -> Router<AppState> {
    router
      .nest("/invite",
        Router::new()
          .route("/{id}", post(Self::invite))
      )
  }
}