use serde::{Deserialize, Serialize};
use adjust::{controller::Controller, response::{HttpMessage, HttpResult, NonJsonHttpResult}};
use axum::{extract::{Path, Query, State}, routing::{delete, get, post}, Json, Router};
use serde_json::Value;
use crate::{models::{friends::Friendship, user::UserIdQuery, userprofile::UserProfile}, repository::friends::FriendsRepository, service::{friend::{FriendEvents, FriendService}, privacy::PrivacyService}, AppState};

#[derive(Serialize, Deserialize)]
struct UserIdBody {
  user_id: i32
}

#[derive(Deserialize)]
struct EventTypeQuery {
  r#type: FriendEvents
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
  ) -> HttpResult<Vec<UserProfile>> {
    let mut db = state.postgres.get()?;
    let mut redis = state.redis.get()?;

    PrivacyService::check_friends_visibility(&mut db, query, user_id)?;

    Ok(Json(FriendsRepository::get_friend_list(&mut db, &mut redis, user_id)?))
  }

  async fn get_friendship_list(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Query(query): Query<UserIdQuery>
  ) -> HttpResult<Vec<Friendship>> {
    let mut db = state.postgres.get()?;

    PrivacyService::check_friends_visibility(&mut db, query, user_id)?;

    Ok(Json(FriendsRepository::get_friendship_list(&mut db, user_id)?))
  }

  // внутренний метод
  /// Отправляем ивент всем друзьям
  async fn send_event(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Query(query): Query<EventTypeQuery>,
    Json(body): Json<Value>
  ) -> NonJsonHttpResult<()> {
    let mut db = state.postgres.get()?;

    FriendService::send_event(&mut db, user_id, query.r#type, body)
  }

  /// Отправляет пользователю запрос в друзья
  async fn add(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Json(body): Json<UserIdBody>
  ) -> HttpResult<Friendship> {
    let mut db = state.postgres.get()?;
    let mut redis = state.redis.get()?;

    Ok(Json(FriendsRepository::add(&mut db, &mut redis, user_id, body.user_id)?))
  }

  /// Пользователь принимает запрос в друзья
  async fn confirm(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Json(body): Json<UserIdBody> // owner
  ) -> HttpResult<Friendship> {
    let mut db = state.postgres.get()?;
    let mut redis = state.redis.get()?;

    Ok(Json(FriendsRepository::accept(&mut db, &mut redis, user_id, body.user_id)?))
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
        .route("/friends/{id}", get(Self::get_friendship_list))
        .route("/event/{id}", post(Self::send_event))
        .route("/{id}", get(Self::get_friend_list))
        .route("/{id}", post(Self::add))
        .route("/{id}", delete(Self::remove))
    )
  }
}