// Внимание
//
// Перед просмотром следующего кода
// Предприймите следующие меры безопасности:
//  1. Подготовьте прохладную мокрую тряпку для лба
//  2. Присядьте
//  3. Выньте любую еду из полости рта

// почему сука в репозитории wss ивенты отправляются??

use adjust::{database::{postgres::Postgres, redis::Redis, Database}, response::{HttpError, NonJsonHttpResult}};
use anyhow::anyhow;
use axum::http::StatusCode;
use diesel::{delete, insert_into, update, BoolExpressionMethods, Connection, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use serde_json::{json, Value};
use crate::{models::{friends::{Friendship, Relationship}, user::UserIdQuery, userprofile::UserProfile}, schema::{friendships, users}, service::{userprofile::UserProfileService, wss::WssService}};

pub struct FriendsRepository;

impl FriendsRepository {
  /// Кидает запрос в друзья игроку
  pub fn add(
    db: &mut Database<Postgres>,
    redis: &mut Database<Redis>,
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

    #[allow(unused)]
    WssService::send(
      user_id,
      "FRIEND_REQUEST",
      UserProfileService::get_user_profile(db, redis, None, friend_id)?
    );

    #[allow(unused)]
    WssService::send(
      friend_id,
      "FRIEND_REQUEST",
      UserProfileService::get_user_profile(db, redis, None, user_id)?
    );

    Ok(result)
  }

  /// Принимает запрос в друзья
  pub fn accept(
    db: &mut Database<Postgres>,
    redis: &mut Database<Redis>,
    user_id: i32, // кто кинул запрос
    friend_id: i32, // кому идет запрос в друзья
  ) -> NonJsonHttpResult<Friendship> {
    let result = db.transaction(|db| {
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
    });

    #[allow(unused)]
    WssService::send(
      user_id,
      "FRIEND_ADD",
      UserProfileService::get_user_profile(db, redis, None, friend_id)?
    );

    #[allow(unused)]
    WssService::send(
      friend_id,
      "FRIEND_ADD",
      UserProfileService::get_user_profile(db, redis, None, user_id)?
    );

    result
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

  /// Возвращает список друзей в виде массива айдишников
  pub fn get_friend_list(
    db: &mut Database<Postgres>,
    redis: &mut Database<Redis>,
    user_id: i32
  ) -> NonJsonHttpResult<Vec<UserProfile>> {
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

    let mut result: Vec<UserProfile> = vec![];

    friends.into_iter().try_for_each(|id| {
      result.push(UserProfileService::get_user_profile(
        db,
        redis,
        Some(UserIdQuery { user_id: Some(user_id) }),
        id
      )?);
      Ok::<(), HttpError>(())
    })?;

    Ok(result)
  }

  #[allow(unused)]
  pub fn get_friend_ids(
    db: &mut Database<Postgres>,
    user_id: i32,
  ) -> NonJsonHttpResult<Vec<i32>> {
    let records = users::table
      .filter(users::id.eq(user_id))
      .select(users::friends)
      .get_result::<Value>(db)?;

    let mut result = vec![];

    if let Some(friends) = records.as_array() {
      friends.iter()
        .for_each(|id| result.push(id.as_str().unwrap_or("0").parse().unwrap_or_default()));
    }

    Ok(result)
  }

  /// Возвращает список друзей
  pub fn get_friendship_list(
    db: &mut Database<Postgres>,
    user_id: i32
  ) -> NonJsonHttpResult<Vec<Friendship>> {
    let results = friendships::table
      .filter(friendships::user_id.eq(user_id).or(friendships::friend_id.eq(user_id)))
      .get_results::<Friendship>(db)
      .map_err(|_| anyhow!("Друзья не были найдены"))?;

    Ok(results)
  }

  /// Возвращает список запросов в друзья
  pub fn get_friend_requests(
    db: &mut Database<Postgres>,
    user_id: i32
  ) -> NonJsonHttpResult<Vec<Friendship>> {
    let results = friendships::table
      .filter(friendships::user_id.eq(user_id).or(friendships::friend_id.eq(user_id)))
      .filter(friendships::status.eq(Relationship::Pending))
      .get_results::<Friendship>(db)
      .map_err(|_| anyhow!("Друзья не были найдены"))?;

    Ok(results)
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

    #[allow(unused)]
    WssService::send(
      user_id,
      "FRIEND_CANCEL",
      json!({
        "id": friend_id
      })
    );

    #[allow(unused)]
    WssService::send(
      friend_id,
      "FRIEND_CANCEL",
      json!({
        "id": user_id
      })
    );

    Ok(())
  }

  /// Удаляет игрока из друзей
  pub fn remove(
    db: &mut Database<Postgres>,
    user_id: i32,
    friend_id: i32
  ) -> NonJsonHttpResult<()> {
    let result = db.transaction(|db| {
      delete(friendships::table)
        .filter(
          friendships::user_id.eq(user_id).and(friendships::friend_id.eq(friend_id))
          .or(friendships::user_id.eq(friend_id).and(friendships::friend_id.eq(user_id)))
        )
        .execute(db)
        .map_err(|_| HttpError::new("Ошибка при удалении из друзей", None))?;

      for &id in &[user_id, friend_id] {
        let friend = if id == user_id { friend_id } else { user_id };
        update(users::table.filter(users::id.eq(id)))
          .set(users::friends.eq(diesel::dsl::sql::<diesel::sql_types::Jsonb>(&format!(
            "friends - '{}'",
            json!(friend)
          ))))
          .execute(db)?;

        // #[allow(unused)]
      }

      Ok(())
    });

    WssService::send(
      user_id,
      "FRIEND_REMOVE",
      json!({
        "id": friend_id
      })
    )?;

    WssService::send(
      friend_id,
      "FRIEND_REMOVE",
      json!({
        "id": user_id
      })
    )?;

    result
  }
}
