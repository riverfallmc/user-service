use adjust::{controller::Controller, response::{HttpError, HttpMessage, HttpResult}};
use axum::{extract::{Path, Query, State}, http::StatusCode, routing::{delete, get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use crate::{models::{friends::Friendship, privacy::Visibility}, repository::{friends::FriendsRepository, privacy::PrivacyRepository}, AppState};

#[derive(Serialize, Deserialize)]
struct UserIdBody {
  user_id: i32
}

#[derive(Serialize, Deserialize)]
struct UserIdQuery {
  user_id: Option<i32>
}

pub struct FriendsController;

impl FriendsController {
  /// Возвращает список друзей
  ///
  /// Принимает JWT если пользователь авторизирован
  ///
  /// Если user_id == query.user_id то возвращаем в любом случае
  /// Если user_id != query.user_id то проверяем то что query.user_id это друг user_id
  async fn get_friend_list(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Query(query): Query<UserIdQuery>
  ) -> HttpResult<Vec<i32>> {
    let mut db = state.postgres.get()?;
    let privacy = PrivacyRepository::get_friends_visibility(&mut db, user_id)?;

    let has_rights = match query.user_id {
      Some(requester) if requester == user_id => true,
      Some(requester) => FriendsRepository::is_friends(&mut db, user_id, requester)?,
      None => privacy == Visibility::Open,
    };

    if !has_rights {
      return Err(HttpError::new("Пользователь скрыл список друзей", Some(StatusCode::FORBIDDEN)));
    }

    Ok(Json(FriendsRepository::get_friend_list(&mut db, user_id)?))
  }

  /// Отправляет пользователю запрос в друзья
  async fn add(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Json(body): Json<UserIdBody>
  ) -> HttpResult<Friendship> {
    let mut db = state.postgres.get()?;

    Ok(Json(FriendsRepository::add(&mut db, user_id, body.user_id)?))
  }

  /// Пользователь принимает запрос в друзья
  async fn confirm(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Json(body): Json<UserIdBody> // owner
  ) -> HttpResult<Friendship> {
    let mut db = state.postgres.get()?;

    Ok(Json(FriendsRepository::accept(&mut db, user_id, body.user_id)?))
  }

  /// Пользователь отменяет/отклоняет запрос в друзья
  async fn cancel(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Json(body): Json<UserIdBody>
  ) -> HttpResult<HttpMessage> {
    let mut db = state.postgres.get()?;

    FriendsRepository::cancel(&mut db, user_id, body.user_id)?;

    Ok(Json(HttpMessage::new("Заявка была отменена")))
  }

  /// Пользователь удаляет другого пользователя из друзей
  async fn remove(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Json(body): Json<UserIdBody>
  ) -> HttpResult<HttpMessage> {
    let mut db = state.postgres.get()?;

    FriendsRepository::remove(&mut db, user_id, body.user_id)?;

    Ok(Json(HttpMessage::new("Пользователь был удален из друзей")))
  }
}

impl Controller<AppState> for FriendsController {
  fn new() -> anyhow::Result<Box<Self>> {
    Ok(Box::new(Self))
  }

  fn register(&self, router: axum::Router<AppState>) -> axum::Router<AppState> {
    router.nest("/friends",
      Router::new()
        .route("/confirm/{id}", post(Self::confirm))
        .route("/cancel/{id}", post(Self::cancel))
        .route("/{id}", get(Self::get_friend_list))
        .route("/{id}", post(Self::add))
        .route("/{id}", delete(Self::remove))
    )
  }
}