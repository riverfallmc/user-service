// Внимание
//
// Перед просмотром следующего кода
// Предприймите следующие меры безопасности:
//  1. Подготовьте прохладную мокрую тряпку для лба
//  2. Присядьте
//  3. Выньте любую еду из полости рта

use adjust::{database::{postgres::Postgres, Database}, response::{HttpError, NonJsonHttpResult}};
use axum::http::StatusCode;
use diesel::{delete, insert_into, update, BoolExpressionMethods, Connection, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use serde_json::json;
use crate::{models::friends::{Friendship, Relationship}, schema::{friendships, users}};

pub struct FriendsRepository;

impl FriendsRepository {
  /// Кидает запрос в друзья игроку
  pub fn add(
    db: &mut Database<Postgres>,
    user_id: i32,
    friend_id: i32
  ) -> NonJsonHttpResult<Friendship> {
    let existing = friendships::table
      .filter(
        (friendships::user_id.eq(user_id).and(friendships::friend_id.eq(friend_id)))
          .or(friendships::user_id.eq(friend_id).and(friendships::friend_id.eq(user_id)))
      )
      .first::<Friendship>(db)
      .optional()
      .map_err(|_| HttpError::new("Ошибка при проверке существующей дружбы", None))?;

    if existing.is_some() {
      return Err(HttpError::new("Запрос в друзья уже существует или вы уже друзья", Some(StatusCode::BAD_REQUEST)));
    }

    let result = insert_into(friendships::table)
      .values((
        friendships::user_id.eq(user_id),
        friendships::friend_id.eq(friend_id)
      ))
      .get_result::<Friendship>(db)
      .map_err(|e| HttpError(anyhow::anyhow!("Не получилось добавить игрока в друзья: {e}"), None))?;

    Ok(result)
  }

  /// Принимает запрос в друзья
  pub fn accept(
    db: &mut Database<Postgres>,
    user_id: i32, // кто кинул запрос
    friend_id: i32, // кому идет запрос в друзья
  ) -> NonJsonHttpResult<Friendship> {
    db.transaction(|db| {
      let friendship = update(friendships::table)
        .filter(
          friendships::user_id.eq(user_id)
          .and(friendships::friend_id.eq(friend_id))
          .and(friendships::status.eq(Relationship::Pending))
        )
        .set(friendships::status.eq(Relationship::Accepted))
        .get_result::<Friendship>(db)
        .map_err(|_| HttpError::new("Заявки не существует, или уже принята", Some(StatusCode::BAD_REQUEST)))?;

      for &id in &[user_id, friend_id] {
        let friend_list = if id == user_id { friend_id } else { user_id };
        update(users::table.filter(users::id.eq(id)))
          .set(users::friends.eq(diesel::dsl::sql::<diesel::sql_types::Jsonb>(&format!(
            "jsonb_insert(friends, '{{}}', friends || '{}'::jsonb)",
            json!([friend_list])
          ))))
          .execute(db)?;
      }

      Ok(friendship)
    })
  }

  /// Проверяет, в друзьях ли друг у друга игроки
  pub fn is_friends(
    db: &mut Database<Postgres>,
    user_id: i32,
    other_user_id: i32
  ) -> NonJsonHttpResult<bool> {
    let exists = friendships::table
      .filter(
        friendships::user_id.eq(user_id)
          .and(friendships::friend_id.eq(other_user_id))
          .and(friendships::status.eq(Relationship::Accepted))
        .or(
          friendships::user_id.eq(other_user_id)
            .and(friendships::friend_id.eq(user_id))
            .and(friendships::status.eq(Relationship::Accepted))
        )
      )
      .first::<Friendship>(db)
      .optional()
      .map_err(|_| HttpError::new("Ошибка при проверке статуса дружбы", None))?
      .is_some();
    Ok(exists)
  }

  /// Получает список друзей
  pub fn get_friend_list(
    db: &mut Database<Postgres>,
    user_id: i32
  ) -> NonJsonHttpResult<Vec<i32>> {
    let friends_json: serde_json::Value = users::table
    .filter(users::id.eq(user_id))
    .select(users::friends)
    .first::<serde_json::Value>(db)
    .map_err(|_| HttpError::new("Не удалось получить список друзей", None))?;

    let friends: Vec<i32> = match friends_json {
      serde_json::Value::Array(arr) => arr
        .into_iter()
        .filter_map(|v| v.as_str()?.parse::<i32>().ok())
        .collect(),
      _ => vec![],
    };

  Ok(friends)
  }

  /// Отменяет запрос в друзья
  pub fn cancel(
    db: &mut Database<Postgres>,
    friend_id: i32,
    user_id: i32,
  ) -> NonJsonHttpResult<()> {
    delete(friendships::table)
      .filter(friendships::user_id.eq(user_id).and(friendships::friend_id.eq(friend_id)))
      .execute(db)
      .map_err(|_| HttpError::new("Ошибка при отмене запроса в друзья", None))?;
    Ok(())
  }

  /// Удаляет игрока из друзей
  pub fn remove(
    db: &mut Database<Postgres>,
    user_id: i32,
    friend_id: i32
  ) -> NonJsonHttpResult<()> {
    db.transaction(|db| {
      delete(friendships::table)
        .filter(
          friendships::user_id.eq(user_id).and(friendships::friend_id.eq(friend_id))
          .or(friendships::user_id.eq(friend_id).and(friendships::friend_id.eq(user_id)))
        )
        .execute(db)
        .map_err(|_| HttpError::new("Ошибка при удалении из друзей", None))?;

      for &id in &[user_id, friend_id] {
        let friend_list = if id == user_id { friend_id } else { user_id };
        update(users::table.filter(users::id.eq(id)))
          .set(users::friends.eq(diesel::dsl::sql::<diesel::sql_types::Jsonb>(&format!(
            "friends - '{}'",
            json!(friend_list)
          ))))
          .execute(db)?;
      }

      Ok(())
    })
  }
}
